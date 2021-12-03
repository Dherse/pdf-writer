use super::*;

/// Writer for a _file specification dictionary_.
///
/// This struct is created by [`Annotation::file`], [`Reference::file`], and
/// [`Action::file`].
pub struct FileSpec<'a> {
    dict: Dict<'a>,
}

impl<'a> FileSpec<'a> {
    /// Create a new file specification writer.
    pub fn new(obj: Obj<'a>) -> Self {
        let mut dict = obj.dict();
        dict.pair(Name(b"Type"), Name(b"Filespec"));
        Self { dict }
    }

    /// Write the `/FS` attribute to set the file system this entry relates to.
    /// If you set the `system` argument to `Name(b"URL")`, this becomes an URL
    /// specification.
    pub fn file_system(&mut self, system: Name) -> &mut Self {
        self.pair(Name(b"FS"), system);
        self
    }

    /// Write the `/F` attribute to set the file path. Directories are indicated
    /// by `/`, independent of the platform.
    pub fn path(&mut self, path: Str) -> &mut Self {
        self.pair(Name(b"F"), path);
        self
    }

    /// Write the `/UF` attribute to set a Unicode-compatible path. Directories
    /// are indicated by `/`, independent of the platform. PDF 1.7+.
    pub fn unic_file(&mut self, path: TextStr) -> &mut Self {
        self.pair(Name(b"UF"), path);
        self
    }

    /// Write the `/V` attribute to indicate whether to cache the file.
    pub fn volatile(&mut self, no_cache: bool) -> &mut Self {
        self.pair(Name(b"V"), no_cache);
        self
    }

    /// Write the `/Desc` attribute to set a file description. PDF 1.6+.
    pub fn description(&mut self, desc: TextStr) -> &mut Self {
        self.pair(Name(b"Desc"), desc);
        self
    }

    /// Write the `/EF` attribute to reference an [embedded file](EmbeddedFile).
    /// PDF 1.3+.
    pub fn embedded_file(&mut self, id: Ref) -> &mut Self {
        self.key(Name(b"EF")).dict().pair(Name(b"F"), id);
        self
    }
}

deref!('a, FileSpec<'a> => Dict<'a>, dict);

/// Writer for a _embedded file stream_.
///
/// This struct is created by [`PdfWriter::embedded_file`].
pub struct EmbeddedFile<'a> {
    stream: Stream<'a>,
}

impl<'a> EmbeddedFile<'a> {
    /// Create a new embedded file writer.
    pub fn new(mut stream: Stream<'a>) -> Self {
        stream.pair(Name(b"Type"), Name(b"EmbeddedFile"));
        Self { stream }
    }

    /// Write the `/Subtype` attribute to set the file type.
    ///
    /// This can either be a MIME type or a name prefixed by a first class PDF
    /// prefix. Note that special characters must be encoded as described in
    /// section 7.3.5 of the PDF 1.7 specification, e.g. `image/svg+xml` would
    /// become `Name(b"image#2Fsvg+xml")`.
    pub fn subtype(&mut self, subtype: Name) -> &mut Self {
        self.pair(Name(b"Subtype"), subtype);
        self
    }

    /// Start writing the `/Params` dictionary.
    pub fn params(&mut self) -> EmbedParams<'_> {
        EmbedParams::new(self.key(Name(b"Params")))
    }
}

deref!('a, EmbeddedFile<'a> => Stream<'a>, stream);

/// Writer for a _embedded file parameter dictionary_.
///
/// This struct is created by [`EmbeddedFile::params`].
pub struct EmbedParams<'a> {
    dict: Dict<'a>,
}

impl<'a> EmbedParams<'a> {
    /// Create a new embedded file parameter writer.
    pub fn new(obj: Obj<'a>) -> Self {
        Self { dict: obj.dict() }
    }

    /// Write the `/Size` attribute to set the uncompressed file size in bytes.
    pub fn size(&mut self, size: i32) -> &mut Self {
        self.pair(Name(b"Size"), size);
        self
    }

    /// Write the `/CreationDate` attribute to set the file creation date.
    pub fn creation_date(&mut self, date: Date) -> &mut Self {
        self.pair(Name(b"CreationDate"), date);
        self
    }

    /// Write the `/ModDate` attribute to set the file modification date.
    pub fn modification_date(&mut self, date: Date) -> &mut Self {
        self.pair(Name(b"ModDate"), date);
        self
    }

    /// Write the `/CheckSum` attribute to set the file checksum.
    ///
    /// The checksum shall be a 16-byte MD5 string.
    pub fn checksum(&mut self, checksum: Str) -> &mut Self {
        self.pair(Name(b"CheckSum"), checksum);
        self
    }
}

deref!('a, EmbedParams<'a> => Dict<'a>, dict);