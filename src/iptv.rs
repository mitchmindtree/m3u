use super::{Entry, EntryExt};
pub use read::iptv::IptvEntries;
use std::collections::*;
pub use write::iptv::IptvEntryWriter;
/// An entry with some associated extra information.
#[derive(Clone, Debug, PartialEq)]
pub struct IptvEntry {
    /// The M3U entry. Can be either a `Path` or `Url`.
    pub entry: Entry,
    /// Extra information associated with the M3U entry.
    pub extinf: IptvExtInf,
}

/// Extra information associated with an M3U entry.
#[derive(Clone, Debug, PartialEq)]
pub struct IptvExtInf {
    /// The duration of the media's runtime in seconds.
    ///
    /// Note that some `m3u` extended formats specify streams with a `-1` duration.
    pub duration_secs: f64,
    /// The name of the media. E.g. "Aphex Twin - Windowlicker".
    pub name: String,
    /// The properties for IPTV compatible players
    pub iptv_props: IptvProps,
}

impl EntryExt {
    /// Add IPTV properties to this EXTINF
    /// # Example
    /// ```rust
    /// #[macro_use]
    /// use m3u::iptv;
    /// let entry = m3u::url_entry("http://server/stream.mp4")
    ///  .unwrap()
    ///  .extend(-1.0, "Channel 1")
    ///  .with_iptv(iptv!("tvg-id"="id channel 1","tvg-logo"="http://server/logo.png"));
    ///  assert_eq!(entry.extinf.iptv_props["tvg-id"], "id channel 1");
    ///  ```
    pub fn with_iptv(self, props: IptvProps) -> IptvEntry {
        IptvEntry {
            entry: self.entry,
            extinf: IptvExtInf {
                duration_secs: self.extinf.duration_secs,
                name: self.extinf.name,
                iptv_props: props,
            },
        }
    }
}

pub type IptvProps = BTreeMap<String, String>;

/// A macro to easily create a HashMap containing IPTV properties
/// # Examples
/// ```rust
/// use {m3u::iptv, std::collections::BTreeMap};
/// assert_eq!(iptv!(), BTreeMap::new());
/// let actual=iptv!("tvg-id"="mychannel","group-title"="mygroup");
/// let expected=[
///     ("tvg-id".to_string(), "mychannel".to_string()),
///     ("group-title".to_string(), "mygroup".to_string())
/// ].iter().cloned().collect();
/// assert_eq!(actual, expected);
/// ```
#[macro_export]
macro_rules! iptv(
    { $($key:tt = $value:expr),* } => {
        {
            let mut m= iptv::IptvProps::new();
            $(
                m.insert($key.to_string(), $value.to_string());
            )*
            m
        }
     };
);
