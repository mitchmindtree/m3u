//! A crate for reading and writing the **M3U** format.
//!
//! The **M3U** format is considered the de facto standard for multimedia playlists.
//!
//! There is no formal specification for the **M3U** format. This crate is implemented based on the
//! rough description under the format's current wikipedia entry.

pub extern crate url;

mod read;
mod write;

pub use read::{Reader, EntryReader, EntryExtReader, Entries, EntryExts,
               EntryExtReaderConstructionError, ReadEntryExtError};
pub use write::{Writer, EntryWriter, EntryExtWriter};
pub use url::Url;

/// An entry in an **M3U** multimedia playlist.
///
/// Describes the source of the media.
///
/// In rare cases an `Entry` may point to another `.m3u` file. If a user wishes to support this in
/// their application, they must be sure to handle cycles within the **M3U** graph.
#[derive(Clone, Debug, Hash, PartialEq)]
pub enum Entry {
    /// The entry resides at the given `Path`.
    ///
    /// The `Path` may be either absolute or relative.
    ///
    /// Note that the `Path` may also point to a directory. After starting, the media player would
    /// play all contents of the directory.
    Path(std::path::PathBuf),
    /// The entry can be found at the given `Url`.
    Url(url::Url),
}

/// An entry with some associated extra information.
#[derive(Clone, Debug, PartialEq)]
pub struct EntryExt {
    /// The M3U entry. Can be either a `Path` or `Url`.
    pub entry: Entry,
    /// Extra information associated with the M3U entry.
    pub extinf: ExtInf,
}

/// Extra information associated with an M3U entry.
#[derive(Clone, Debug, PartialEq)]
pub struct ExtInf {
    /// The duration of the media's runtime in seconds.
    ///
    /// Note that some `m3u` extended formats specify streams with a `-1` duration.
    pub duration_secs: f64,
    /// The name of the media. E.g. "Aphex Twin - Windowlicker".
    pub name: String,
}


impl Entry {

    /// Whether or not the `Entry` is a `Path`.
    pub fn is_path(&self) -> bool {
        match *self {
            Entry::Path(_) => true,
            Entry::Url(_) => false,
        }
    }

    /// Whether or not the `Entry` is a `Url`.
    pub fn is_url(&self) -> bool {
        match *self {
            Entry::Url(_) => true,
            Entry::Path(_) => false,
        }
    }

    /// Extend the entry with extra information including the duration in seconds and a name.
    pub fn extend<N>(self, duration_secs: f64, name: N) -> EntryExt
        where N: Into<String>,
    {
        EntryExt {
            extinf: ExtInf {
                duration_secs,
                name: name.into(),
            },
            entry: self,
        }
    }

}

/// A helper function to simplify creation of the `Entry`'s `Path` variant.
pub fn path_entry<P>(path: P) -> Entry
    where P: Into<std::path::PathBuf>,
{
    Entry::Path(path.into())
}

/// A helper function to simplify creation of the `Entry`'s `Url` variant.
pub fn url_entry(url: &str) -> Result<Entry, url::ParseError> {
    Url::parse(url).map(Entry::Url)
}
