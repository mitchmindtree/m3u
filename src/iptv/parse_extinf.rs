use self::split::{next_closing_mark_index, split_key_val};
use crate::iptv::{IptvExtInf, IptvProps};

/// splits /EXTINF:-1( +[\w-]=".*")*,.*/
/// Example:
/// EXTINF:-1 tvg-id="" tvg-name="TF1",TF1
/// => IptvExtInf{
///         duration_secs:-1,
///         name:"TF1",
///         props:iptv!("tvg-id"="", "tvg-name"="TF1")
///     }
pub fn parse_extinf(line: &str) -> Option<IptvExtInf> {
    let line = drop_extinf_tag(line)?;
    let (sep_index, _) = line.match_indices(|c| c == ',' || c == ' ').next()?;
    let duration_str = &line[0..sep_index];
    let line = &line[sep_index..];

    let mut props_map = IptvProps::new();
    let mut offset = 0;
    while let Some(prop_end_index) = next_closing_mark_index(line, offset) {
        let prop_str = &line[offset..=prop_end_index];
        let (key, val) = split_key_val(&prop_str)?;
        props_map.insert(key.to_string(), val.to_string());
        offset = prop_end_index + 1;
    }
    let name = &line[offset + 1..];
    let duration_secs: f64 = duration_str.parse().ok()?;

    Some(IptvExtInf {
        duration_secs,
        name: name.to_string(),
        iptv_props: props_map,
    })
}

fn drop_extinf_tag(line: &str) -> Option<&str> {
    line.get(8..)
}

mod split;

#[cfg(test)]
mod test;
