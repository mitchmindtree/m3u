use std::collections::btree_map::{Iter, IterMut};
use std::collections::BTreeMap;
use std::ops::Index;
#[derive(Clone, Debug, PartialEq)]
pub struct IptvProps(BTreeMap<String, String>);

/// A macro to easily create a HashMap containing IPTV properties
/// # Examples
/// ```rust
/// use {m3u::iptv, m3u::iptv::IptvProps, std::collections::BTreeMap};
/// assert_eq!(iptv!(), IptvProps::new());
/// let actual=iptv!("tvg-id"="mychannel","group-title"="mygroup");
/// let expected=[
///     ("tvg-id".to_string(), "mychannel".to_string()),
///     ("group-title".to_string(), "mygroup".to_string())
/// ].iter().cloned().collect::<BTreeMap<_,_>>().into();
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

impl IptvProps {
    pub fn new() -> Self {
        IptvProps(BTreeMap::new())
    }
    pub fn insert(&mut self, key: String, val: String) {
        self.0.insert(key, val);
    }
    pub fn iter_mut(&mut self) -> IterMut<String, String> {
        self.0.iter_mut()
    }
    pub fn iter(&self) -> Iter<String, String> {
        self.0.iter()
    }
}

impl From<BTreeMap<String, String>> for IptvProps {
    fn from(map: BTreeMap<String, String>) -> Self {
        IptvProps(map)
    }
}

impl ToString for IptvProps {
    /// Converts IPTV properties to string usable in M3U
    /// # Examples
    /// assert_eq!(iptv_props_to_string(HashMap::new()), "".to_string());
    /// assert_eq!(iptv_props_to_string(iptv!("tvg-group"="fr")), r#" tvg-group="fr""#);
    fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|(key, value)| format!(r#" {}="{}""#, key, value))
            .fold(String::new(), |mut buffer, s| {
                buffer.push_str(&s);
                buffer
            })
    }
}

impl Index<&str> for IptvProps {
    type Output = String;
    fn index(&self, idx: &str) -> &String {
        self.0.index(idx)
    }
}
