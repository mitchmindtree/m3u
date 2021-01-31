use super::Writer;
use crate::iptv::{IptvEntry, IptvProps};
use std::io::Write;

/// A `Writer` that specifically writes `IptvEntry`es.
pub type IptvEntryWriter<'w, W> = Writer<'w, W, IptvEntry>;

impl<'w, W> IptvEntryWriter<'w, W>
where
    W: Write,
{
    /// Create a writer that writes extended M3U `IptvEntry`es.
    ///
    /// The `#EXTM3U` header line is written immediately.
    pub fn new_iptv(writer: &'w mut W) -> Result<Self, std::io::Error> {
        let mut line_buffer = Vec::new();
        writeln!(&mut line_buffer, "#EXTM3U")?;
        writer.write_all(&line_buffer)?;
        Ok(Self::new_inner(writer, line_buffer))
    }

    /// Converts IPTV properties to string usable in M3U
    /// # Examples
    /// assert_eq!(iptv_props_to_string(HashMap::new()), "".to_string());
    /// assert_eq!(iptv_props_to_string(iptv!("tvg-group"="fr")), r#" tvg-group="fr""#);
    fn iptv_props_to_string(props: &IptvProps) -> String {
        props
            .iter()
            .map(|(key, value)| format!(r#" {}="{}""#, key, value))
            .fold(String::new(), |mut buffer, s| {
                buffer.push_str(&s);
                buffer
            })
    }

    /// Attempt to write the given `IptvEntry` to the given `writer`.
    ///
    /// First writes the `#EXTINF:` line, then writes the entry line.
    /// # Example
    /// ```rust
    /// use m3u::iptv;
    /// let mut buff=std::io::Cursor::new(vec![]);
    /// {
    /// let mut writer=m3u::iptv::IptvEntryWriter::new_iptv(&mut buff).expect("Unable to open writer");
    /// let entry = m3u::url_entry("http://server/stream.mp4")
    ///     .unwrap()
    ///     .extend(-1.0, "Channel 1")
    ///     .with_iptv(iptv!("tvg-name"="Channel 1"));
    /// writer.write_entry(&entry);
    /// }
    /// let result=std::str::from_utf8(buff.get_ref()).unwrap();
    /// assert_eq!(result, "#EXTM3U\n#EXTINF:-1 tvg-name=\"Channel 1\",Channel 1\nhttp://server/stream.mp4\n");
    /// ```
    pub fn write_entry(&mut self, entry_ext: &IptvEntry) -> Result<(), std::io::Error> {
        let Writer {
            ref mut writer,
            ref mut line_buffer,
            ..
        } = *self;
        line_buffer.clear();
        let extinf = &entry_ext.extinf;
        let iptv = Self::iptv_props_to_string(&extinf.iptv_props);
        writeln!(
            line_buffer,
            "#EXTINF:{}{},{}",
            extinf.duration_secs, iptv, &extinf.name
        )?;
        super::write_entry(line_buffer, &entry_ext.entry)?;
        writer.write_all(line_buffer)
    }
}
