m3u [![Build Status](https://travis-ci.org/mitchmindtree/m3u.svg?branch=master)](https://travis-ci.org/mitchmindtree/m3u) [![Crates.io](https://img.shields.io/crates/v/m3u.svg)](https://crates.io/crates/m3u) [![Crates.io](https://img.shields.io/crates/l/m3u.svg)](https://github.com/mitchmindtree/m3u/blob/master/LICENSE-MIT)
===

A lib for reading and writing `.m3u` files - the de facto standard for multimedia playlists.

Please read [the documentation](https://docs.rs/m3u/).

Example
-------

### Original M3U

```rust
extern crate m3u;

fn main() {

    // Create a multimedia media playlist.
    let playlist = vec![
        m3u::path_entry(r"Alternative\Band - Song.mp3"),
        m3u::path_entry(r"Classical\Other Band - New Song.mp3"),
        m3u::path_entry(r"Stuff.mp3"),
        m3u::path_entry(r"D:\More Music\Foo.mp3"),
        m3u::path_entry(r"..\Other Music\Bar.mp3"),
        m3u::url_entry(r"http://emp.cx:8000/Listen.pls").unwrap(),
        m3u::url_entry(r"http://www.example.com/~user/Mine.mp3").unwrap(),
    ];

    // Write the playlist to a file.
    {
        let mut file = std::fs::File::create("playlist.m3u").unwrap();
        let mut writer = m3u::Writer::new(&mut file);
        for entry in &playlist {
            writer.write_entry(entry).unwrap();
        }
    }

    // Read the playlist from the file.
    let mut reader = m3u::Reader::open("playlist.m3u").unwrap();
    let read_playlist: Vec<_> = reader.entries().map(|entry| entry.unwrap()).collect();
    assert_eq!(&playlist, &read_playlist);
}
```

Writes then reads a plain text UTF-8 file that looks like this:

```m3u
Alternative\Band - Song.mp3
Classical\Other Band - New Song.mp3
Stuff.mp3
D:\More Music\Foo.mp3
..\Other Music\Bar.mp3
http://emp.cx:8000/Listen.pls
http://www.example.com/~user/Mine.mp3
```

### Extended M3U

```rust
extern crate m3u;

fn main() {

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

    // Write the playlist to the file.
    {
        let mut file = std::fs::File::create("playlist_ext.m3u").unwrap();
        let mut writer = m3u::Writer::new_ext(&mut file).unwrap();
        for entry in &playlist {
            writer.write_entry(entry).unwrap();
        }
    }

    // Read the playlist from the file.
    let mut reader = m3u::Reader::open_ext("playlist_ext.m3u").unwrap();
    let read_playlist: Vec<_> = reader.entry_exts().map(|entry| entry.unwrap()).collect();
    assert_eq!(&playlist, &read_playlist);
}
```

Writes then reads a plain text UTF-8 file in the Extended M3U format that looks like this:

```m3u
#EXTM3U
#EXTINF:123,Sample artist - Sample title
C:\Documents and Settings\I\My Music\Sample.mp3
#EXTINF:321,Example Artist - Example title
C:\Documents and Settings\I\My Music\Greatest Hits\Example.ogg
#EXTINF:123,Sample artist - Sample title
Sample.mp3
#EXTINF:321,Example Artist - Example title
Greatest Hits\Example.ogg
```

License
-------

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.


**Contributions**

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
