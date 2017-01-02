extern crate m3u;

#[test]
fn mixed() {
    let path = std::path::Path::new("tests/mixed.m3u");
    let mut reader = m3u::Reader::open(path).unwrap();
    let mut entries = reader.entries();

    assert_eq!(entries.next().unwrap().unwrap(),
               m3u::path_entry(r"Alternative\Band - Song.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               m3u::path_entry(r"Classical\Other Band - New Song.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               m3u::path_entry(r"Stuff.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               m3u::path_entry(r"D:\More Music\Foo.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               m3u::path_entry(r"..\Other Music\Bar.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               m3u::url_entry(r"http://emp.cx:8000/Listen.pls").unwrap());
    assert_eq!(entries.next().unwrap().unwrap(),
               m3u::url_entry(r"http://www.example.com/~user/Mine.mp3").unwrap());
    assert!(entries.next().is_none());
}

#[test]
fn ext() {
    let expected = vec![
        m3u::path_entry(r"C:\Documents and Settings\I\My Music\Sample.mp3")
            .extend(123, "Sample artist - Sample title"),
        m3u::path_entry(r"C:\Documents and Settings\I\My Music\Greatest Hits\Example.ogg")
            .extend(321, "Example Artist - Example title"),
        m3u::path_entry(r"Sample.mp3")
            .extend(123, "Sample artist - Sample title"),
        m3u::path_entry(r"Greatest Hits\Example.ogg")
            .extend(321, "Example Artist - Example title"),
    ];

    let path = std::path::Path::new("tests/ext.m3u");
    let mut reader = m3u::Reader::open_ext(path).unwrap();
    let entries: Vec<_> = reader.entry_exts().map(|e| e.unwrap()).collect();

    assert_eq!(&entries, &expected);
}
