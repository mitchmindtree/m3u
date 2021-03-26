#[test]
fn test_iptv_entry_reader() {
    use super::IptvEntryReader;
    use crate::iptv;
    let m3u = r#"#EXTM3U
    #EXTINF:-1 group-title="VOD | SPANISH" tvg-id="" tvg-logo="https://image.tmdb.org/t/p/w500/6TBLHCbQtxfiqPzlj3PPrVrRkEe.jpg" tvg-name="ES - Si Fueras Tú, La Película",ES - Si Fueras Tú, La Película
http://borg.hopto.org:8090/html/greetings.mp4"#;
    let reader = m3u.as_bytes();
    let ier = IptvEntryReader::new(reader);
    let expected = crate::url_entry("http://borg.hopto.org:8090/html/greetings.mp4")
        .unwrap()
        .extend(-1.0, "ES - Si Fueras Tú, La Película")
        .with_iptv(iptv!(
            "tvg-id" = "",
            "tvg-name" = "ES - Si Fueras Tú, La Película",
            "tvg-logo" = "https://image.tmdb.org/t/p/w500/6TBLHCbQtxfiqPzlj3PPrVrRkEe.jpg",
            "group-title" = "VOD | SPANISH"
        ));
    let actual = ier.unwrap().read_next_entry().unwrap().unwrap();
    assert_eq!(actual.raw_extinf, expected.raw_extinf);
    assert_eq!(actual, expected);
}

#[test]
fn raw_extinf_is_available_in_entry() {
    use super::IptvEntryReader;
    let extinf = r#"#EXTINF:-1 tvg-id="" tvg-name="ES - Si Fueras Tú, La Película" tvg-logo="https://image.tmdb.org/t/p/w500/6TBLHCbQtxfiqPzlj3PPrVrRkEe.jpg" group-title="VOD | SPANISH",ES - Si Fueras Tú, La Película"#;
    let m3u = format!(
        r#"#EXTM3U
    {}
http://borg.hopto.org:8090/html/greetings.mp4"#,
        extinf
    );
    let reader = m3u.as_bytes();
    let ier = IptvEntryReader::new(reader);
    assert_eq!(
        ier.unwrap()
            .read_next_entry()
            .unwrap()
            .unwrap()
            .raw_extinf
            .as_str(),
        extinf
    );
}
