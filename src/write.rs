use {Entry, EntryExt};
use std;
use std::io::Write;

/// A writer that accepts entries of type `E` and writes the associated M3U format.
///
/// Entries are always written using in UTF-8.
pub struct Writer<'w, W, E>
where
    W: Write,
{
    /// The writer to which the `M3U` format is written.
    writer: &'w mut W,
    /// Used for buffering lines as bytes for writing.
    line_buffer: Vec<u8>,
    /// The type of entries that will be written.
    entry: std::marker::PhantomData<E>,
}

/// A `Writer` that specifically writes `Entry`s.
pub type EntryWriter<'w, W> = Writer<'w, W, Entry>;
/// A `Writer` that specifically writes `EntryExt`s.
pub type EntryExtWriter<'w, W> = Writer<'w, W, EntryExt>;

impl<'w, W, E> Writer<'w, W, E>
where
    W: Write,
{
    fn new_inner(writer: &'w mut W, line_buffer: Vec<u8>) -> Self {
        Writer {
            writer,
            line_buffer,
            entry: std::marker::PhantomData,
        }
    }

    /// `Flush` the `writer` output stream, ensuring that all intermediately buffered entries reach
    /// their destination.
    ///
    /// This should be called after all entries have been written.
    ///
    /// If it is not called, the destructor will finalize the file, but any errors that occur in
    /// the process cannot be handled.
    pub fn flush(self) -> Result<(), std::io::Error> {
        self.writer.flush()
    }
}

impl<'w, W> EntryWriter<'w, W>
where
    W: Write,
{
    /// Create a writer that writes the original, non_extended M3U `Entry` type.
    pub fn new(writer: &'w mut W) -> Self {
        Self::new_inner(writer, Vec::new())
    }

    /// Attempt to write the given `Entry` to the given `writer`.
    ///
    /// Writes the `Path` or `Url` in plain text, ending with a newline.
    pub fn write_entry(&mut self, entry: &Entry) -> Result<(), std::io::Error> {
        let Writer { ref mut writer, ref mut line_buffer, .. } = *self;
        line_buffer.clear();
        write_entry(line_buffer, entry)?;
        writer.write_all(line_buffer)
    }

}

impl<'w, W> EntryExtWriter<'w, W>
where
    W: Write,
{

    /// Create a writer that writes extended M3U `EntryExt`s.
    ///
    /// The `#EXTM3U` header line is written immediately.
    pub fn new_ext(writer: &'w mut W) -> Result<Self, std::io::Error> {
        let mut line_buffer = Vec::new();
        writeln!(&mut line_buffer, "#EXTM3U")?;
        writer.write_all(&line_buffer)?;
        Ok(Self::new_inner(writer, line_buffer))
    }

    /// Attempt to write the given `EntryExt` to the given `writer`.
    ///
    /// First writes the `#EXTINF:` line, then writes the entry line.
    /// # Example
    /// ```rust
    /// let mut buff=std::io::Cursor::new(vec![]);
    /// {
    /// let mut writer=m3u::EntryExtWriter::new_ext(&mut buff).expect("Unable to open writer");
    /// let entry = m3u::url_entry("http://server/stream.mp4").unwrap().extend(-1.0, "Channel 1");
    /// writer.write_entry(&entry);
    /// }
    /// let result=std::str::from_utf8(buff.get_ref()).unwrap();
    /// assert_eq!(result, "#EXTM3U\n#EXTINF:-1,Channel 1\nhttp://server/stream.mp4\n");
    /// ```
    pub fn write_entry(&mut self, entry_ext: &EntryExt) -> Result<(), std::io::Error> {
        let Writer { ref mut writer, ref mut line_buffer, .. } = *self;
        line_buffer.clear();
        let extinf = &entry_ext.extinf;
        writeln!(
            line_buffer,
            "#EXTINF:{},{}",
            extinf.duration_secs, &extinf.name
        )?;
        write_entry(line_buffer, &entry_ext.entry)?;
        writer.write_all(line_buffer)
    }

}

/// Write the given `Entry` into the given `line_buffer`.
///
/// Writes the `Path` or `Url` in plain text, ending with a newline.
fn write_entry(line_buffer: &mut Vec<u8>, entry: &Entry) -> Result<(), std::io::Error> {
    match *entry {
        Entry::Path(ref path) => writeln!(line_buffer, "{}", path.display()),
        Entry::Url(ref url) => writeln!(line_buffer, "{}", url),
    }
}

impl<'w, W, E> Drop for Writer<'w, W, E>
where
    W: Write,
{
    fn drop(&mut self) {
        self.writer.flush().ok();
    }
}

#[cfg(feature = "iptv")]
pub mod iptv;
