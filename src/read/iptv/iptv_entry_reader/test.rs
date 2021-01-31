#[test]
fn test_iptv_entry_reader() {
    use super::IptvEntryReader;
    use crate::iptv;
    let m3u = r#"#EXTM3U
    #EXTINF:-1 tvg-id="" tvg-name="ES - Si Fueras Tú, La Película" tvg-logo="https://image.tmdb.org/t/p/w500/6TBLHCbQtxfiqPzlj3PPrVrRkEe.jpg" group-title="VOD | SPANISH",ES - Si Fueras Tú, La Película
http://borg.hopto.org:8090/html/greetings.mp4"#;
    let reader = m3u.as_bytes();
    let ier = IptvEntryReader::new(reader);
    let entry = crate::url_entry("http://borg.hopto.org:8090/html/greetings.mp4")
        .unwrap()
        .extend(-1.0, "ES - Si Fueras Tú, La Película")
        .with_iptv(iptv!(
            "tvg-id" = "",
            "tvg-name" = "ES - Si Fueras Tú, La Película",
            "tvg-logo" = "https://image.tmdb.org/t/p/w500/6TBLHCbQtxfiqPzlj3PPrVrRkEe.jpg",
            "group-title" = "VOD | SPANISH"
        ));

    assert_eq!(ier.unwrap().read_next_entry().unwrap().unwrap(), entry);
}
