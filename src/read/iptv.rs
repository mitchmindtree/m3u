//#[macro_use]
use self::iptv_entry_reader::IptvEntryReader;
use super::EntryExtReaderConstructionError;
use super::{read_entry, read_next_entry, EntryExtReader, ReadEntryExtError, Reader};

/// An iterator that yields `IptvEntry`es.
///
/// All `IptvEntry`es are lazily read from the inner buffered reader.
#[derive(Debug)]
pub struct IptvEntries<'r, R>
where
    R: 'r + std::io::BufRead,
{
    reader: &'r mut IptvEntryReader<R>,
}

mod iptv_entry_reader;
mod next_iptv_entry;
