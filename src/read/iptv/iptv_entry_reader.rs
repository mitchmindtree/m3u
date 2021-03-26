use super::next_iptv_entry::next_iptv_entry;
use super::EntryExtReaderConstructionError;
use super::{EntryExtReader, IptvEntries, ReadEntryExtError, Reader};
use crate::iptv::IptvEntry;
use std::io::BufRead;

/// A `Reader` that specifically reads `IptvEntry`es.
pub type IptvEntryReader<R> = Reader<R, IptvEntry>;

impl<R: BufRead> IptvEntryReader<R> {
    pub fn new(reader: R) -> Result<Self, EntryExtReaderConstructionError> {
        let entry_ext_reader = EntryExtReader::new_ext(reader)?;
        Ok(Self::new_inner(
            entry_ext_reader.reader,
            entry_ext_reader.line_buffer,
        ))
    }

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
    fn read_next_entry(&mut self) -> Result<Option<IptvEntry>, ReadEntryExtError> {
        let Reader {
            ref mut reader,
            ref mut line_buffer,
            ..
        } = *self;
        next_iptv_entry(reader, line_buffer)
    }

    /// Produce an iterator that yields `IptvEntry`es.
    ///
    /// All `IptvEntry`es are lazily read from the inner buffered reader.
    pub fn iptv_entries(&mut self) -> IptvEntries<R> {
        IptvEntries { reader: self }
    }
}

impl IptvEntryReader<std::io::BufReader<std::fs::File>> {
    /// Attempts to create a reader that reads `IptvEntry`es from the specified file.
    ///
    /// This is a convenience constructor that opens a `File`, wraps it in a `BufReader` and then
    /// constructs a `Reader` from it.
    pub fn open_iptv<P>(filename: P) -> Result<Self, EntryExtReaderConstructionError>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(filename)?;
        let buf_reader = std::io::BufReader::new(file);
        Self::new(buf_reader)
    }
}

impl<'r, R> Iterator for IptvEntries<'r, R>
where
    R: std::io::BufRead,
{
    type Item = Result<IptvEntry, ReadEntryExtError>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_next_entry() {
            Ok(Some(entry)) => Some(Ok(entry)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

mod test;
