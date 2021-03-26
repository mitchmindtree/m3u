impl IptvEntryReader<std::io::BufReader<std::fs::File>> {
    /// Attempts to create a reader that reads `IptvEntry`es from the specified file.
    ///
    /// This is a convenience constructor that opens a `File`, wraps it in a `BufReader` and then
    /// constructs a `Reader` from it.
    pub fn open_iptv<P>(filename: P) -> Result<Self, EntryExtReaderConstructionError>
    where
        P: AsRef<std::path::Path>,
    {
        let file = std::fs::File::open(filename)?;
        let buf_reader = std::io::BufReader::new(file);
        Self::new(buf_reader)
    }
}
