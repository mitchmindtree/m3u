extern crate m3u;

#[test]
fn entry() {

    // Create a multimedia playlist.
    let playlist = vec![
        m3u::path_entry(r"Alternative\Band - Song.mp3"),
        m3u::path_entry(r"Classical\Other Band - New Song.mp3"),
        m3u::path_entry(r"Stuff.mp3"),
        m3u::path_entry(r"D:\More Music\Foo.mp3"),
        m3u::path_entry(r"..\Other Music\Bar.mp3"),
        m3u::url_entry(r"http://emp.cx:8000/Listen.pls").unwrap(),
        m3u::url_entry(r"http://www.example.com/~user/Mine.mp3").unwrap(),
    ];

    const FILEPATH: &'static str = "tests/playlist.m3u";

    if std::path::Path::new(FILEPATH).exists() {
        std::fs::remove_file(FILEPATH).unwrap();
    }

    // Write the playlist to the file.
    {
        let mut file = std::fs::File::create(FILEPATH).unwrap();
        let mut writer = m3u::Writer::new(&mut file);
        for entry in &playlist {
            writer.write_entry(entry).unwrap();
        }
        writer.flush().unwrap();
    }

    // Read the playlist from the file.
    {
        let mut reader = m3u::Reader::open(FILEPATH).unwrap();
        let read_playlist: Vec<_> = reader.entries().map(|entry| entry.unwrap()).collect();
        assert_eq!(&playlist, &read_playlist);
    }

    std::fs::remove_file(FILEPATH).unwrap();
}

#[test]
fn entry_ext() {

    // Create a multimedia playlist, including the duration in seconds and name for each entry.
    let playlist = vec![
        m3u::path_entry(r"C:\Documents and Settings\I\My Music\Sample.mp3")
            .extend(123.0, "Sample artist - Sample title"),
        m3u::path_entry(r"C:\Documents and Settings\I\My Music\Greatest Hits\Example.ogg")
            .extend(321.0, "Example Artist - Example title"),
        m3u::path_entry(r"Sample.mp3")
            .extend(123.0, "Sample artist - Sample title"),
        m3u::path_entry(r"Greatest Hits\Example.ogg")
            .extend(321.0, "Example Artist - Example title"),
    ];

    const FILEPATH: &'static str = "tests/playlist_ext.m3u";

    if std::path::Path::new(FILEPATH).exists() {
        std::fs::remove_file(FILEPATH).unwrap();
    }

    // Write the playlist to the file.
    {
        let mut file = std::fs::File::create(FILEPATH).unwrap();
        let mut writer = m3u::Writer::new_ext(&mut file).unwrap();
        for entry in &playlist {
            writer.write_entry(entry).unwrap();
        }
        writer.flush().unwrap();
    }

    // Read the playlist from the file.
    {
        let mut reader = m3u::Reader::open_ext(FILEPATH).unwrap();
        let read_playlist: Vec<_> = reader.entry_exts().map(|entry| entry.unwrap()).collect();
        assert_eq!(&playlist, &read_playlist);
    }

    std::fs::remove_file(FILEPATH).unwrap();
}


#[test]
fn entry_x_stream() {

    // Create a variant playlist.
    let playlist = vec![
        m3u::url_entry(r"http://example.com/low/index.m3u8").unwrap()
            .x_stream(1, 150000, "416x234", "avc1.42e00a,mp4a.40.2"),
        m3u::url_entry(r"http://example.com/mid/index.m3u8").unwrap()
            .x_stream(1, 240000, "416x234", "avc1.42e00a,mp4a.40.2"),
    ];

    const FILEPATH: &'static str = "tests/playlist_x_stream.m3u";

    if std::path::Path::new(FILEPATH).exists() {
        std::fs::remove_file(FILEPATH).unwrap();
    }

    // Write the playlist to the file.
    {
        let mut file = std::fs::File::create(FILEPATH).unwrap();
        let mut writer = m3u::Writer::new_x_stream(&mut file).unwrap();
        for entry in &playlist {
            writer.write_x_stream(entry).unwrap();
        }
        writer.flush().unwrap();
    }

    // Read the playlist from the file.
    {
        let mut reader = m3u::Reader::open_x_stream(FILEPATH).unwrap();
        let read_playlist: Vec<_> = reader.entry_exts().map(|entry| entry.unwrap()).collect();
        assert_eq!(&playlist, &read_playlist);
    }

    std::fs::remove_file(FILEPATH).unwrap();
}
