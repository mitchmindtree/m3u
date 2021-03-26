use super::parse_extinf;
use super::IptvExtInf;
use crate::iptv;

#[test]
fn example() {
    let s = r#"#EXTINF:-1 tvg-id="" tvg-name="TF1",TF1"#;
    assert_eq!(
        parse_extinf(s),
        Some(IptvExtInf {
            duration_secs: -1.0,
            name: "TF1".into(),
            iptv_props: iptv!("tvg-id" = "", "tvg-name" = "TF1")
        })
    );
}

#[test]
fn empty_extinf_cannot_be_parsed() {
    assert_eq!(parse_extinf(""), None);
}

#[test]
fn extinf_may_contain_1_prop() {
    assert_eq!(
        parse_extinf("#EXTINF:10 a=\"\",Toto"),
        Some(IptvExtInf {
            name: "Toto".to_string(),
            duration_secs: 10.0,
            iptv_props: iptv!("a" = "")
        })
    );
}

#[test]
fn property_value_may_contain_quotes() {
    use crate::iptv;
    assert_eq!(
        parse_extinf(r#"#EXTINF:10 a="var "a"",Toto"#),
        Some(IptvExtInf {
            name: "Toto".to_string(),
            duration_secs: 10.0,
            iptv_props: iptv!("a" = "var \"a\"")
        })
    );
}

#[test]
fn property_value_may_contain_commas() {
    use crate::iptv;
    assert_eq!(
        parse_extinf(r#"#EXTINF:10 a="b,c",Toto"#),
        Some(IptvExtInf {
            name: "Toto".to_string(),
            duration_secs: 10.0,
            iptv_props: iptv!("a" = "b,c")
        })
    );
}

#[test]
fn simple_extinf_is_allowed() {
    use iptv::IptvProps;
    assert_eq!(
        parse_extinf("#EXTINF:10,Toto"),
        Some(IptvExtInf {
            name: "Toto".to_string(),
            duration_secs: 10.0,
            iptv_props: IptvProps::new()
        })
    );
}

#[test]
fn value_may_contain_commas_and_accents() {
    let s = r#"#EXTINF:-1 tvg-id="" tvg-name="ES - Si Fueras Tú, La Película" tvg-logo="https://image.tmdb.org/t/p/w500/6TBLHCbQtxfiqPzlj3PPrVrRkEe.jpg" group-title="VOD | SPANISH",ES - Si Fueras Tú, La Película"#;
    assert_eq!(
        parse_extinf(s),
        Some(IptvExtInf {
            duration_secs: -1.0,
            name: "ES - Si Fueras Tú, La Película".to_string(),
            iptv_props: iptv!(
                "tvg-id" = "",
                "tvg-name" = "ES - Si Fueras Tú, La Película",
                "tvg-logo" = "https://image.tmdb.org/t/p/w500/6TBLHCbQtxfiqPzlj3PPrVrRkEe.jpg",
                "group-title" = "VOD | SPANISH"
            )
        })
    );
}
