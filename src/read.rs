use {Entry, EntryExt, ExtInf, EntryExtXStream, ExtXStreamInf};
use std;
use url;

/// A reader that reads the `M3U` format from the underlying reader.
///
/// A `Reader` is a streaming reader. It reads data from the underlying reader on demand and reads
/// no more than strictly necessary.
///
/// The inner `reader` `R` must be some buffered reader as the "#EXTM3U" header, "#EXTINF:" tags
/// and entries are each read from a single line of plain text.
///
/// A `Reader` will only attempt to read entries of type `E`.
pub struct Reader<R, E>
    where R: std::io::BufRead,
{
    /// The reader from which the `M3U` format is read.
    reader: R,
    /// String used for buffering read lines.
    line_buffer: String,
    /// The entry type that the `reader` will read.
    entry: std::marker::PhantomData<E>,
}

/// A `Reader` that specifically reads `Entry`s.
pub type EntryReader<R> = Reader<R, Entry>;
/// A `Reader` that specifically reads `EntryExt`s.
pub type EntryExtReader<R> = Reader<R, EntryExt>;
/// A `Reader` that specifically reads `EntryExtXStream`s.
pub type EntryExtXStreamReader<R> = Reader<R, EntryExtXStream>;

/// An iterator that yields `Entry`s.
///
/// All `Entry`s are lazily read from the inner buffered reader.
pub struct Entries<'r, R>
    where R: 'r + std::io::BufRead,
{
    reader: &'r mut EntryReader<R>,
}

/// An iterator that yields `EntryExt`s.
///
/// All `EntryExt`s are lazily read from the inner buffered reader.
pub struct EntryExts<'r, R>
    where R: 'r + std::io::BufRead,
{
    reader: &'r mut EntryExtReader<R>,
}

/// An iterator that yields `EntryExtXStream`s.
///
/// All `EntryExtXStream`s are lazily read from the inner buffered reader.
pub struct EntryExtXStreams<'r, R>
    where R: 'r + std::io::BufRead,
{
    reader: &'r mut EntryExtXStreamReader<R>,
}

/// Errors that may occur when constructing a new `Reader<R, EntryExt>`.
#[derive(Debug)]
pub enum EntryExtReaderConstructionError {
    /// The "#EXTM3U" header was not found in the first line when attempting to
    /// construct a `Reader<R, EntryExt>` from some given `Reader`.
    HeaderNotFound,
    /// Errors produced by the `BufRead::read_line` method.
    BufRead(std::io::Error),
}

/// Errors that may occur when attempting to read an `EntryExt` from a read line `str`.
#[derive(Debug)]
pub enum ReadEntryExtError {
    /// Either the "#EXTINF:" tag was not found for the `EntryExt` or the duration and name
    /// following the tag were not correctly formatted.
    ///
    /// Assuming that the tag was simply omitted, the line will instead be parsed as an `Entry`.
    ExtInfNotFound(Entry),
    /// Errors produced by the `BufRead::read_line` method.
    BufRead(std::io::Error),
}

/// Errors that may occur when attempting to read an `EntryExtXStream` from a read line `str`.
#[derive(Debug)]
pub enum ReadEntryExtXStreamError {
    /// Either the "#EXT-X-STREAM-INF:" tag was not found for the `EntryExtXStream` or the
    /// program_id, bandwidth, resolution or codecs were not correctly specified.
    ///
    /// Assuming that the tag was simply omitted, the line will instead be parsed as an `Entry`.
    ExtXStreamInfNotFound(Entry),
    /// Errors produced by the `BufRead::read_line` method.
    BufRead(std::io::Error),
}


impl<R, E> Reader<R, E>
    where R: std::io::BufRead,
{

    fn new_inner(reader: R, line_buffer: String) -> Self {
        Reader {
            reader: reader,
            line_buffer: line_buffer,
            entry: std::marker::PhantomData,
        }
    }

    /// Produce the inner `reader`.
    pub fn into_inner(self) -> R {
        self.reader
    }

}

impl<R> EntryReader<R>
    where R: std::io::BufRead,
{

    /// Create a reader that reads the original, non-extended M3U `Entry` type.
    pub fn new(reader: R) -> Self {
        Self::new_inner(reader, String::new())
    }

    /// Attempt to read the next `Entry` from the inner reader.
    ///
    /// Returns `Ok(None)` when there are no more lines.
    ///
    /// Returns an `Err(std::io::Error)` if an error occurs when calling the inner `reader`'s
    /// `BufRead::read_line` method.
    fn read_next_entry(&mut self) -> Result<Option<Entry>, std::io::Error> {
        let Reader { ref mut reader, ref mut line_buffer, .. } = *self;
        read_next_entry(reader, line_buffer)
    }

    /// Produce an iterator that yields `Entry`s.
    ///
    /// All `Entry`s are lazily read from the inner buffered reader.
    pub fn entries(&mut self) -> Entries<R> {
        Entries { reader: self }
    }

}

impl<R> EntryExtReader<R>
    where R: std::io::BufRead,
{

    /// Create a reader that reads the extended M3U `EntryExt` type.
    ///
    /// The `#EXTM3U` header is read immediately.
    ///
    /// Reading `EntryExt`s will be done on demand.
    pub fn new_ext(mut reader: R) -> Result<Self, EntryExtReaderConstructionError> {
        let mut line_buffer = String::new();

        loop {
            let num_read_bytes = try!(reader.read_line(&mut line_buffer));
            let line = line_buffer.trim_left();

            // The first line of the extended M3U format should always be the "#EXTM3U" header.
            const HEADER: &'static str = "#EXTM3U";
            if line.len() >= HEADER.len() && &line[..HEADER.len()] == HEADER {
                break;
            }

            // Skip any empty lines that might be present at the top of the file.
            if num_read_bytes != 0 && line.is_empty() {
                continue;
            }

            // If the first non-empty line was not the header, return an error.
            return Err(EntryExtReaderConstructionError::HeaderNotFound);
        }

        Ok(Self::new_inner(reader, line_buffer))
    }

    /// Attempt to read the next `EntryExt` from the inner reader.
    ///
    /// This method attempts to read two non-empty, non-comment lines.
    ///
    /// The first is checked for the `EXTINF` tag which is used to create an `ExtInf`. Upon failure
    /// an `ExtInfNotFound` error is returned and the line is instead parsed as an `Entry`.
    ///
    /// If an `#EXTINF:` tag was read, next line is parsed as an `Entry`.
    ///
    /// Returns `Ok(None)` when there are no more lines.
    fn read_next_entry(&mut self) -> Result<Option<EntryExt>, ReadEntryExtError> {
        let Reader { ref mut reader, ref mut line_buffer, .. } = *self;

        const TAG: &'static str = "#EXTINF:";

        // Read an `ExtInf` from the given line.
        //
        // This function assumes the the line begins with "#EXTINF:" and will panic otherwise.
        fn read_extinf(mut line: &str) -> Option<ExtInf> {
            line = &line[TAG.len()..];

            // The duration and track title should be delimited by the first comma.
            let mut parts = line.splitn(2, ',');

            // Get the duration, or return `None` if there isn't any.
            let duration_secs = match parts.next().and_then(|s| s.parse().ok()) {
                Some(secs) => secs,
                None => return None,
            };

            // Get the name or set it as an empty string.
            let name = parts.next().map(|s| s.trim().into()).unwrap_or_else(String::new);

            Some(ExtInf {
                duration_secs: duration_secs,
                name: name,
            })
        }

        // Skip empty lines and comments until we find the "#EXTINF:" tag.
        loop {
            // Read the next line or return `None` if we're done.
            line_buffer.clear();
            if try!(reader.read_line(line_buffer)) == 0 {
                return Ok(None);
            }

            let extinf = {
                let line = line_buffer.trim_left();

                match line.chars().next() {
                    // Skip empty lines.
                    None => continue,
                    // Distinguish between comments and the "#EXTINF:" tag.
                    Some('#') => match line.len() >= TAG.len() && &line[..TAG.len()] == TAG {
                        // Skip comments.
                        false => continue,
                        // We've found the "#EXTINF:" tag.
                        true => read_extinf(line),
                    },
                    // Assume the "#EXTINF:" tag was omitted and this was intended to be an `Entry`.
                    // Due to the lack of official specification, it is unclear whether a mixture
                    // of tagged and non-tagged entries should be supported for the EXTM3U format.
                    Some(_) => {
                        let entry = read_entry(line.trim_right());
                        return Err(ReadEntryExtError::ExtInfNotFound(entry));
                    },
                }
            };

            // Read the next non-empty, non-comment line as an entry.
            let entry = match try!(read_next_entry(reader, line_buffer)) {
                None => return Ok(None),
                Some(entry) => entry,
            };

            return match extinf {
                Some(extinf) => Ok(Some(EntryExt {
                    entry: entry,
                    extinf: extinf,
                })),
                None => Err(ReadEntryExtError::ExtInfNotFound(entry)),
            }
        }
    }

    /// Produce an iterator that yields `EntryExt`s.
    ///
    /// All `EntryExt`s are lazily read from the inner buffered reader.
    pub fn entry_exts(&mut self) -> EntryExts<R> {
        EntryExts { reader: self }
    }

}

impl<R> EntryExtXStreamReader<R>
    where R: std::io::BufRead,
{

    /// Create a reader that reads the extended M3U `EntryExtXStream` type.
    ///
    /// The `#EXTM3U` header is read immediately.
    ///
    /// Reading `EntryExtXStream`s will be done on demand.
    pub fn new_x_stream(mut reader: R) -> Result<Self, EntryExtReaderConstructionError> {
        let mut line_buffer = String::new();

        loop {
            let num_read_bytes = try!(reader.read_line(&mut line_buffer));
            let line = line_buffer.trim_left();

            // The first line of the extended M3U format should always be the "#EXTM3U" header.
            const HEADER: &'static str = "#EXTM3U";
            if line.len() >= HEADER.len() && &line[..HEADER.len()] == HEADER {
                break;
            }

            // Skip any empty lines that might be present at the top of the file.
            if num_read_bytes != 0 && line.is_empty() {
                continue;
            }

            // If the first non-empty line was not the header, return an error.
            return Err(EntryExtReaderConstructionError::HeaderNotFound);
        }

        Ok(Self::new_inner(reader, line_buffer))
    }

    /// Attempt to read the next `EntryExtXStream` from the inner reader.
    ///
    /// This method attempts to read two non-empty, non-comment lines.
    ///
    /// The first is checked for the `EXT-X-STREAM-INF` tag which is used to create
    /// an `EntryExtXStream`. Upon failure an `ExtXStreamInfNotFound` error is returned and
    /// the line is instead parsed as an `Entry`.
    ///
    /// If an `#EXT-X-STREAM-INF:` tag was read, next line is parsed as an `Entry`.
    ///
    /// Returns `Ok(None)` when there are no more lines.
    fn read_next_entry(&mut self) -> Result<Option<EntryExtXStream>, ReadEntryExtXStreamError> {
        let Reader { ref mut reader, ref mut line_buffer, .. } = *self;

        const TAG: &'static str = "#EXT-X-STREAM-INF:";

        // Read an `ExtXStreamInf` from the given line.
        //
        // This function assumes the line begins with "#EXT-X-STREAM-INF:" and will panic otherwise.
        fn read_x_stream(mut line: &str) -> Option<ExtXStreamInf> {
            line = &line[TAG.len()..];

            // The form of an attribute is `name=foo` or name="foo"`. Attributes are separated
            // by comma, so the quoted strings are required when the value itself has commas.
            fn read_attr(next: &str) -> Result<(&str, &str, &str),()> {
                let mut todo = next.splitn(2, '=');
                let name = todo.next().ok_or(())?;
                let rest = todo.next().ok_or(())?;
                const QUOTE: &'static str = "\"";
                let value_rest = match &rest[0..1] == QUOTE {
                    // unquoted value
                    false => {
                        let mut split = rest.splitn(2, ',');
                        let value = split.next().ok_or(())?;
                        let rest = split.next().unwrap_or("");
                        (value, rest)
                    }
                    true => {
                        // quoted value
                        let mut split = rest.splitn(3, '"');
                        let value = split.nth(1).ok_or(())?;
                        let rest = split.nth(2).unwrap_or(",");
                        (value, &rest[1..])
                    }
                };
                Ok((name, value_rest.0, value_rest.1))
            }

            // left to parse
            let mut todo = line;
            let mut attributes = std::collections::HashMap::new();

            loop {
                match read_attr(todo) {
                    Err(_) => break,
                    Ok((name, value, rest)) => {
                        todo = rest;
                        attributes.insert(name, value.to_string());
                    }
                }
            }

            Some(ExtXStreamInf {
                program_id: attributes.get("PROGRAM-ID").and_then(|s| s.parse().ok()),
                bandwidth: attributes.get("BANDWIDTH").and_then(|s| s.parse().ok()),
                resolution: attributes.get("RESOLUTION").cloned(),
                codecs: attributes.get("CODECS").cloned(),
            })
        }

        // Skip empty lines and comments until we find the "#EXT-X-STREAM-INF:" tag.
        loop {
            // Read the next line or return `None` if we're done.
            line_buffer.clear();
            if try!(reader.read_line(line_buffer)) == 0 {
                return Ok(None);
            }

            let extinf = {
                let line = line_buffer.trim_left();

                match line.chars().next() {
                    // Skip empty lines.
                    None => continue,
                    // Distinguish between comments and the "#EXT-X-STREAM-INF:" tag.
                    Some('#') => match line.len() >= TAG.len() && &line[..TAG.len()] == TAG {
                        // Skip comments.
                        false => continue,
                        // We've found the "#EXT-X-STREAM-INF:" tag.
                        true => read_x_stream(line),
                    },
                    // Assume the "#EXT-X-STREAM-INF:" tag was omitted and this was intended to
                    // be an `Entry`. Due to the lack of official specification, it is unclear
                    // whether a mixture of tagged and non-tagged entries should be supported
                    // for the EXTM3U format.
                    Some(_) => {
                        let entry = read_entry(line.trim_right());
                        return Err(ReadEntryExtXStreamError::ExtXStreamInfNotFound(entry));
                    },
                }
            };

            // Read the next non-empty, non-comment line as an entry.
            let entry = match try!(read_next_entry(reader, line_buffer)) {
                None => return Ok(None),
                Some(entry) => entry,
            };

            return match extinf {
                Some(extinf) => Ok(Some(EntryExtXStream {
                    entry: entry,
                    extinf: extinf,
                })),
                None => Err(ReadEntryExtXStreamError::ExtXStreamInfNotFound(entry)),
            }
        }
    }

    /// Produce an iterator that yields `EntryExtXStream`s.
    ///
    /// All `EntryExt`s are lazily read from the inner buffered reader.
    pub fn entry_exts(&mut self) -> EntryExtXStreams<R> {
        EntryExtXStreams { reader: self }
    }


}

impl EntryReader<std::io::BufReader<std::fs::File>> {

    /// Attempts to create a reader that reads `Entry`s from the specified file.
    ///
    /// This is a convenience constructor that opens a `File`, wraps it in a `BufReader` and then
    /// constructs a `Reader` from it.
    pub fn open<P>(filename: P) -> Result<Self, std::io::Error>
        where P: AsRef<std::path::Path>,
    {
        let file = try!(std::fs::File::open(filename));
        let buf_reader = std::io::BufReader::new(file);
        Ok(Self::new(buf_reader))
    }

}

impl EntryExtReader<std::io::BufReader<std::fs::File>> {

    /// Attempts to create a reader that reads `EntryExt`s from the specified file.
    ///
    /// This is a convenience constructor that opens a `File`, wraps it in a `BufReader` and then
    /// constructs a `Reader` from it.
    pub fn open_ext<P>(filename: P) -> Result<Self, EntryExtReaderConstructionError>
        where P: AsRef<std::path::Path>,
    {
        let file = try!(std::fs::File::open(filename));
        let buf_reader = std::io::BufReader::new(file);
        Self::new_ext(buf_reader)
    }

}


impl EntryExtXStreamReader<std::io::BufReader<std::fs::File>> {

    /// Attempts to create a reader that reads `EntryExtXStream`s from the specified file.
    ///
    /// This is a convenience constructor that opens a `File`, wraps it in a `BufReader` and then
    /// constructs a `Reader` from it.
    pub fn open_x_stream<P>(filename: P) -> Result<Self, EntryExtReaderConstructionError>
        where P: AsRef<std::path::Path>,
    {
        let file = try!(std::fs::File::open(filename));
        let buf_reader = std::io::BufReader::new(file);
        Self::new_x_stream(buf_reader)
    }

}


/// Attempt to read the next `Entry` from the inner reader.
fn read_next_entry<R>(reader: &mut R, line_buffer: &mut String) -> Result<Option<Entry>, std::io::Error>
    where R: std::io::BufRead,
{
    loop {
        // Read the next line or return `None` if we're done.
        line_buffer.clear();
        if try!(reader.read_line(line_buffer)) == 0 {
            return Ok(None);
        }

        let line = line_buffer.trim_left();
        match line.chars().next() {
            // Skip empty lines.
            None => continue,
            // Skip comments.
            Some('#') => continue,
            // Break when we have a non-empty, non-comment line.
            _ => return Ok(Some(read_entry(line.trim_right()))),
        }
    }
}

/// Read an `Entry` from the given line.
///
/// First attempts to read a URL entry. A URL is only returned if `Some` `host_str` is parsed.
///
/// If a URL cannot be parsed, we assume the entry is a `Path`.
fn read_entry(line: &str) -> Entry {
    if let Ok(url) = url::Url::parse(line) {
        if url.host_str().is_some() {
            return Entry::Url(url);
        }

    }
    Entry::Path(line.into())
}


impl<'r, R> Iterator for Entries<'r, R>
    where R: std::io::BufRead,
{
    type Item = Result<Entry, std::io::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_next_entry() {
            Ok(Some(entry)) => Some(Ok(entry)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'r, R> Iterator for EntryExts<'r, R>
    where R: std::io::BufRead,
{
    type Item = Result<EntryExt, ReadEntryExtError>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_next_entry() {
            Ok(Some(entry)) => Some(Ok(entry)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}

impl<'r, R> Iterator for EntryExtXStreams<'r, R>
    where R: std::io::BufRead,
{
    type Item = Result<EntryExtXStream, ReadEntryExtXStreamError>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.reader.read_next_entry() {
            Ok(Some(entry)) => Some(Ok(entry)),
            Ok(None) => None,
            Err(err) => Some(Err(err)),
        }
    }
}


impl From<std::io::Error> for EntryExtReaderConstructionError {
    fn from(err: std::io::Error) -> Self {
        EntryExtReaderConstructionError::BufRead(err)
    }
}

impl From<std::io::Error> for ReadEntryExtError {
    fn from(err: std::io::Error) -> Self {
        ReadEntryExtError::BufRead(err)
    }
}

impl From<std::io::Error> for ReadEntryExtXStreamError {
    fn from(err: std::io::Error) -> Self {
        ReadEntryExtXStreamError::BufRead(err)
    }
}

impl std::error::Error for EntryExtReaderConstructionError {
    fn description(&self) -> &str {
        match *self {
            EntryExtReaderConstructionError::HeaderNotFound =>
                "the \"#EXTM3U\" header was not found",
            EntryExtReaderConstructionError::BufRead(ref err) =>
                std::error::Error::description(err),
        }
    }
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            EntryExtReaderConstructionError::HeaderNotFound => None,
            EntryExtReaderConstructionError::BufRead(ref err) => Some(err),
        }
    }
}

impl std::error::Error for ReadEntryExtError {
    fn description(&self) -> &str {
        match *self {
            ReadEntryExtError::ExtInfNotFound(_) =>
                "the \"#EXTINF:\" tag was not found or was incorrectly formatted",
            ReadEntryExtError::BufRead(ref err) =>
                std::error::Error::description(err),
        }
    }
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ReadEntryExtError::ExtInfNotFound(_) => None,
            ReadEntryExtError::BufRead(ref err) => Some(err),
        }
    }
}


impl std::fmt::Display for EntryExtReaderConstructionError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            EntryExtReaderConstructionError::HeaderNotFound =>
                write!(f, "{}", std::error::Error::description(self)),
            EntryExtReaderConstructionError::BufRead(ref err) =>
                err.fmt(f),
        }
    }
}

impl std::fmt::Display for ReadEntryExtError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            ReadEntryExtError::ExtInfNotFound(_) =>
                write!(f, "{}", std::error::Error::description(self)),
            ReadEntryExtError::BufRead(ref err) =>
                err.fmt(f),
        }
    }
}



impl std::error::Error for ReadEntryExtXStreamError {
    fn description(&self) -> &str {
        match *self {
            ReadEntryExtXStreamError::ExtXStreamInfNotFound(_) =>
                "the \"#EXT-X-STREAM-INF:\" tag was not found or was incorrectly formatted",
            ReadEntryExtXStreamError::BufRead(ref err) =>
                std::error::Error::description(err),
        }
    }
    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ReadEntryExtXStreamError::ExtXStreamInfNotFound(_) => None,
            ReadEntryExtXStreamError::BufRead(ref err) => Some(err),
        }
    }
}


impl std::fmt::Display for ReadEntryExtXStreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            ReadEntryExtXStreamError::ExtXStreamInfNotFound(_) =>
                write!(f, "{}", std::error::Error::description(self)),
            ReadEntryExtXStreamError::BufRead(ref err) =>
                err.fmt(f),
        }
    }
}
