pub use self::iptv_props::IptvProps;
use self::parse_extinf::parse_extinf;
use super::{Entry, EntryExt};
pub use read::iptv::IptvEntries;
pub use write::iptv::IptvEntryWriter;

/// An entry with some associated extra information.
#[derive(Clone, Debug)]
pub struct IptvEntry {
    /// The M3U entry. Can be either a `Path` or `Url`.
    pub entry: Entry,
    /// Extra information associated with the M3U entry.
    pub parsed_extinf: Option<Result<IptvExtInf, ParseExtInfError>>,
    pub raw_extinf: String,
}

impl PartialEq for IptvEntry {
    fn eq(&self, other: &IptvEntry) -> bool {
        self.entry == other.entry
            && (self.raw_extinf == other.raw_extinf || self.parsed_extinf == other.parsed_extinf)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct ParseExtInfError(String);

impl IptvEntry {
    pub fn parsed_extinf(&mut self) -> &Result<IptvExtInf, ParseExtInfError> {
        if self.parsed_extinf.is_none() {
            self.parsed_extinf = match parse_extinf(&self.raw_extinf) {
                None => Some(Err(ParseExtInfError(self.raw_extinf.clone()))),
                Some(extinf) => Some(Ok(extinf)),
            };
        }
        &self.parsed_extinf.as_ref().unwrap()
    }
}
/// Extra information associated with an M3U entry.
#[derive(Clone, Debug, PartialEq)]
pub struct IptvExtInf {
    /// The duration of the media's runtime in seconds.
    ///
    /// Note that some `m3u` extended formats specify streams with a `-1` duration.
    duration_secs: f64,
    /// The name of the media. E.g. "Aphex Twin - Windowlicker".
    name: String,
    /// The properties for IPTV compatible players
    pub iptv_props: IptvProps,
}

impl IptvExtInf {
    pub fn new(duration_secs: f64, name: String, iptv_props: IptvProps) -> Self {
        IptvExtInf {
            duration_secs,
            name,
            iptv_props,
        }
    }
    fn to_string(&self) -> String {
        let iptv = self.iptv_props.to_string();
        format!("#EXTINF:{}{},{}", self.duration_secs, iptv, self.name)
    }
}

impl EntryExt {
    /// Add IPTV properties to this EXTINF
    /// # Example
    /// ```rust
    /// #[macro_use]
    /// use m3u::iptv;
    /// let mut entry = m3u::url_entry("http://server/stream.mp4")
    ///  .unwrap()
    ///  .extend(-1.0, "Channel 1")
    ///  .with_iptv(iptv!("tvg-id"="id channel 1","tvg-logo"="http://server/logo.png"));
    ///  assert_eq!(entry.parsed_extinf().as_ref().unwrap().iptv_props["tvg-id"], "id channel 1");
    ///  ```
    pub fn with_iptv(self, props: IptvProps) -> IptvEntry {
        let parsed_extinf = IptvExtInf {
            duration_secs: self.extinf.duration_secs,
            name: self.extinf.name,
            iptv_props: props,
        };
        let raw_extinf = parsed_extinf.to_string();

        IptvEntry {
            entry: self.entry,
            parsed_extinf: Some(Ok(parsed_extinf)),
            raw_extinf,
        }
    }
}

mod iptv_props;
mod parse_extinf;
