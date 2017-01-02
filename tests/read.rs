extern crate m3u;

fn path_entry(s: &str) -> m3u::Entry {
    m3u::Entry::Path(s.into())
}

fn url_entry(s: &str) -> m3u::Entry {
    m3u::Entry::Url(m3u::url::Url::parse(s).unwrap())
}

#[test]
fn mixed() {
    let path = std::path::Path::new("tests/mixed.m3u");
    let mut reader = m3u::Reader::open(path).unwrap();
    let mut entries = reader.entries();

    assert_eq!(entries.next().unwrap().unwrap(),
               path_entry(r"Alternative\Band - Song.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               path_entry(r"Classical\Other Band - New Song.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               path_entry(r"Stuff.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               path_entry(r"D:\More Music\Foo.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               path_entry(r"..\Other Music\Bar.mp3"));
    assert_eq!(entries.next().unwrap().unwrap(),
               url_entry(r"http://emp.cx:8000/Listen.pls"));
    assert_eq!(entries.next().unwrap().unwrap(),
               url_entry(r"http://www.example.com/~user/Mine.mp3"));
    assert!(entries.next().is_none());
}

#[test]
fn ext() {

    fn entry_ext(duration_secs: u64, name: String, entry: m3u::Entry) -> m3u::EntryExt {
        m3u::EntryExt {
            extinf: m3u::ExtInf {
                duration_secs: duration_secs,
                name: name,
            },
            entry: entry,
        }
    }

    let expected = vec![
        entry_ext(123, "Sample artist - Sample title".into(),
                  path_entry(r"C:\Documents and Settings\I\My Music\Sample.mp3")),
        entry_ext(321, "Example Artist - Example title".into(),
                  path_entry(r"C:\Documents and Settings\I\My Music\Greatest Hits\Example.ogg")),
        entry_ext(123, "Sample artist - Sample title".into(),
                  path_entry(r"Sample.mp3")),
        entry_ext(321, "Example Artist - Example title".into(),
                  path_entry(r"Greatest Hits\Example.ogg")),
    ];

    let path = std::path::Path::new("tests/ext.m3u");
    let mut reader = m3u::Reader::open_ext(path).unwrap();
    let entries: Vec<_> = reader.entry_exts().map(|e| e.unwrap()).collect();

    assert_eq!(&entries, &expected);
}
