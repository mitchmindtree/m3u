extern crate m3u;

#[cfg(feature = "iptv")]
#[test]
fn read() {
    use m3u::iptv;
    let expected = vec![
        m3u::url_entry(r"http://borg.hopto.org:8090/html/greetings.mp4")
            .unwrap()
            .extend(-1.0, "#### BEIN SPORT #####")
            .with_iptv(iptv!(
                "tvg-id" = "",
                "tvg-name" = "##### BEIN SPORT #####",
                "tvg-logo" = "http://212.8.253.112/logo/BEIN/BEINSPORTS.png",
                "group-title" = "AR | BEIN SPORT"
            )),
    ];

    let path = std::path::Path::new("tests/iptv.m3u");
    let mut reader = m3u::Reader::open_iptv(path).unwrap();
    let entry_exts = reader.iptv_entries();
    dbg!(entry_exts);
    let entry_exts = reader.iptv_entries();
    let entries: Vec<_> = entry_exts
        .inspect(|e| println!("entry:{:?}", e))
        .map(|e| e.unwrap())
        .collect();
    for entry in entries.iter().take(1) {
        for (key, value) in entry.extinf.iptv_props.iter() {
            assert_eq!(&expected[0].extinf.iptv_props[key], value);
        }
    }
    assert_eq!(&entries, &expected);
}

#[cfg(feature = "iptv")]
#[test]
fn write() {
    use m3u::iptv;
    use m3u::iptv::IptvEntryWriter;
    use std::io::Cursor;
    let mut buff = Cursor::new(vec![]);
    {
        let mut writer = IptvEntryWriter::new_iptv(&mut buff).unwrap();

        let entry = m3u::url_entry("http://server/stream.mp4")
            .unwrap()
            .extend(-1.0, "Channel 1")
            .with_iptv(m3u::iptv!(
                "tvg-id" = "id channel 1",
                "tvg-logo" = "http://server/logo.png"
            ));

        writer.write_entry(&entry).expect("Unable to write entry");
    }
    //let buff = writer.get_inner_writer();
    let result = std::str::from_utf8(buff.get_ref()).unwrap();
    assert_eq!(
        result,
        r#"#EXTM3U
#EXTINF:-1 tvg-id="id channel 1" tvg-logo="http://server/logo.png",Channel 1
http://server/stream.mp4
"#
    );
}
