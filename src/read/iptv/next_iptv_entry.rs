use super::read_extinf::read_extinf;
use super::{read_entry, read_next_entry, ReadEntryExtError};
use crate::iptv::IptvEntry;
use std::io::BufRead;

/// Attempt to read the next `IptvEntry` from the inner reader.
///
/// This method attempts to read two non-empty, non-comment lines.
///
/// The first is checked for the `EXTINF` tag which is used to create an `IptvExtInf`. Upon failure
/// an `ExtInfNotFound` error is returned and the line is instead parsed as an `Entry`.
///
/// If an `#EXTINF:` tag was read, next line is parsed as an `Entry`.
///
/// Returns `Ok(None)` when there are no more lines.
pub fn next_iptv_entry<R: BufRead>(
    reader: &mut R,
    line_buffer: &mut String,
) -> Result<Option<IptvEntry>, ReadEntryExtError> {
    // Skip empty lines and comments until we find the "#EXTINF:" tag.
    loop {
        // Read the next line or return `None` if we're done.
        line_buffer.clear();
        if reader.read_line(line_buffer)? == 0 {
            return Ok(None);
        }

        let extinf = {
            //let line = line_buffer.trim_start();
            let line = line_buffer.trim();

            match line.chars().next() {
                // Skip empty lines.
                None => continue,
                // Distinguish between comments and the "#EXTINF:" tag.
                Some('#') => {
                    const TAG: &str = "#EXTINF:";
                    if line.len() >= TAG.len() && &line[..TAG.len()] == TAG {
                        // We've found the "#EXTINF:" tag.
                        read_extinf(line)
                    } else {
                        // Skip comments.
                        continue;
                    }
                }
                // Assume the "#EXTINF:" tag was omitted and this was intended to be an `Entry`.
                // Due to the lack of official specification, it is unclear whether a mixture
                // of tagged and non-tagged entries should be supported for the EXTM3U format.
                Some(_) => {
                    let entry = read_entry(line.trim_end());
                    return Err(ReadEntryExtError::ExtInfNotFound(entry));
                }
            }
        };

        // Read the next non-empty, non-comment line as an entry.
        let entry = match read_next_entry(reader, line_buffer)? {
            None => return Ok(None),
            Some(entry) => entry,
        };

        return match extinf {
            Some(extinf) => Ok(Some(IptvEntry { entry, extinf })),
            None => Err(ReadEntryExtError::ExtInfNotFound(entry)),
        };
    }
}

#[test]
fn test() {
    use iptv::{IptvExtInf, IptvProps};
    use url_entry;
    let m3u = r#"#EXTINF:-1,Titre 1
    http://toto"#;
    let mut reader = m3u.as_bytes();
    let mut buffer = String::new();
    let actual = next_iptv_entry(&mut reader, &mut buffer).unwrap().unwrap();
    let expected = IptvEntry {
        entry: url_entry("http://toto").unwrap(),
        extinf: IptvExtInf {
            duration_secs: -1.0,
            name: "Titre 1".into(),
            iptv_props: IptvProps::new(),
        },
    };
    assert_eq!(actual, expected);
}
