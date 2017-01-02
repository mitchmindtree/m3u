extern crate m3u;

fn path_entry(s: &str) -> m3u::Entry {
    m3u::Entry::Path(s.into())
}

fn url_entry(s: &str) -> m3u::Entry {
    m3u::Entry::Url(m3u::url::Url::parse(s).unwrap())
}

#[test]
fn entry() {

    let entries = vec![
        path_entry(r"Alternative\Band - Song.mp3"),
        path_entry(r"Classical\Other Band - New Song.mp3"),
        path_entry(r"Stuff.mp3"),
        path_entry(r"D:\More Music\Foo.mp3"),
        path_entry(r"..\Other Music\Bar.mp3"),
        url_entry(r"http://emp.cx:8000/Listen.pls"),
        url_entry(r"http://www.example.com/~user/Mine.mp3"),
    ];

    const FILEPATH: &'static str = "tests/entries.m3u";

    if std::path::Path::new(FILEPATH).exists() {
        std::fs::remove_file(FILEPATH).unwrap();
    }

    // Write the entries to the file.
    {
        let mut file = std::fs::File::create(FILEPATH).unwrap();
        let mut writer = m3u::Writer::new(&mut file);
        for entry in &entries {
            writer.write_entry(entry).unwrap();
        }
        writer.flush().unwrap();
    }

    // Read the entries from the file.
    {
        let mut reader = m3u::Reader::open(FILEPATH).unwrap();
        let read_entries: Vec<_> = reader.entries().map(|entry| entry.unwrap()).collect();
        assert_eq!(&entries, &read_entries);
    }

    std::fs::remove_file(FILEPATH).unwrap();
}

#[test]
fn entry_ext() {

    fn entry_ext(duration_secs: u64, name: String, entry: m3u::Entry) -> m3u::EntryExt {
        m3u::EntryExt {
            extinf: m3u::ExtInf {
                duration_secs: duration_secs,
                name: name,
            },
            entry: entry,
        }
    }

    let entries = vec![
        entry_ext(123, "Sample artist - Sample title".into(),
                  path_entry(r"C:\Documents and Settings\I\My Music\Sample.mp3")),
        entry_ext(321, "Example Artist - Example title".into(),
                  path_entry(r"C:\Documents and Settings\I\My Music\Greatest Hits\Example.ogg")),
        entry_ext(123, "Sample artist - Sample title".into(),
                  path_entry(r"Sample.mp3")),
        entry_ext(321, "Example Artist - Example title".into(),
                  path_entry(r"Greatest Hits\Example.ogg")),
    ];

    const FILEPATH: &'static str = "tests/entries_ext.m3u";

    if std::path::Path::new(FILEPATH).exists() {
        std::fs::remove_file(FILEPATH).unwrap();
    }

    // Write the entries to the file.
    {
        let mut file = std::fs::File::create(FILEPATH).unwrap();
        let mut writer = m3u::Writer::new_ext(&mut file).unwrap();
        for entry in &entries {
            writer.write_entry(entry).unwrap();
        }
        writer.flush().unwrap();
    }

    // Read the entries from the file.
    {
        let mut reader = m3u::Reader::open_ext(FILEPATH).unwrap();
        let read_entries: Vec<_> = reader.entry_exts().map(|entry| entry.unwrap()).collect();
        assert_eq!(&entries, &read_entries);
    }

    std::fs::remove_file(FILEPATH).unwrap();
}
