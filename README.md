m3u [![Build Status](https://travis-ci.org/mitchmindtree/m3u.svg?branch=master)](https://travis-ci.org/mitchmindtree/m3u) [![Crates.io](https://img.shields.io/crates/v/m3u.svg)](https://crates.io/crates/m3u) [![Crates.io](https://img.shields.io/crates/l/m3u.svg)](https://github.com/mitchmindtree/m3u/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/m3u/badge.svg)](https://docs.rs/m3u/)
===

A lib for reading and writing `.m3u` files - the de facto standard for multimedia playlists.

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

### IPTV M3U

This lib can also read M3U files for IPTV lists, like the one above. An unofficial specification can be found at https://wiki.tvip.ru/en/m3u. 

```m3u
#EXTM3U
#EXTINF:-1 tvg-id="1508" tvg-logo="http://server.com/logo/FRANCE/TF1HD.png" group-title="EUROPE | France FHD - OTT,null", FR - TF1 FHD
http://server.com/live/1508.ts
#EXTINF:-1 tvg-id="1507" tvg-logo="http://server.com/logo/FRANCE/FRANCE2HD.png" group-title="EUROPE | France FHD - OTT,null", FR - FRANCE 2 FHD
http://server.com/live/1507.ts
#EXTINF:-1 tvg-id="1506" tvg-logo="http://server.com/logo/FRANCE/FRANCE3HD.png" group-title="EUROPE | France FHD - OTT,null", FR - FRANCE 3 FHD
http://server.com/live/1506.ts
```

The playlist is read the same way any other extended M3U list, and properties specific to IPTV are stored in `iptv_properties` field of `IptvExtInf` struct. For exemple, to list all channel groups present in a M3U file :

```rust
extern crate m3u;
use m3u::Reader;

fn main() { 
   // Read the playlist from the file. 
    let filename = std::env::args().nth(1).unwrap();
    let mut groups: Vec<String>  = Reader::open_iptv(filename)
        .unwrap()
        .iptv_entries()
        .filter_map(|e|e.unwrap().extinf.iptv_props.remove("group-title"))
        .collect() ; 
    groups.sort();
    groups.dedup();
    for s in groups{
        println!("{}",s);
    }
}
```

To use IPTV features, you must enable "iptv" feature in `Cargo.toml`:

```toml
[dependencies]
m3u = {version = "*", features = "iptv"}
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
