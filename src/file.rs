use std::{io, time::SystemTime, fmt::Display};


///Representation of an open file.
///Wraps [`std::fs`] things such as metadata together to handle neatly.
#[derive(Debug)]
pub struct EzFile {
    path: String,
    handle: std::fs::File,
    metadata: std::fs::Metadata,
}
impl EzFile {
    ///Open a file in read-only mode.
    ///Refer to [`std::io::Read`] for more information.
    ///```
    ///use ez_fs::EzFile;
    ///use std::io::Read;
    ///
    ///let mut file = EzFile::open("foo.txt").unwrap();
    ///let mut buf = String::new();
    ///file.read_to_string(&mut buf).unwrap();
    ///```
    ///# Errors
    ///This function will error if `path` does not exist.
    pub fn open(path: &str) -> io::Result<Self> {
        let handle = std::fs::File::open(path)?;
        let metadata = handle.metadata()?;
        Ok(Self {
            path: path.to_string(),
            handle,
            metadata,
        })
    }
    ///Open a file in write-only mode.
    ///Refer to [`std::io::Write`] for more information.
    ///```
    ///use ez_fs::EzFile;
    ///use std::io::Write;
    ///
    ///let mut file = EzFile::create("foo.txt").unwrap();
    ///file.write_all(b"bar").unwrap();
    ///```
    ///# Errors
    ///This function will error if `path` does not exist.
    pub fn create(path: &str) -> io::Result<Self> {
        let handle = std::fs::File::create(path)?;
        let metadata = handle.metadata()?;
        Ok(Self {
            path: path.to_string(),
            handle,
            metadata,
        })
    }

    

    ///Deconstructs and returns the path, file handle, and metadata of a file.
    #[must_use] pub fn into_raw(self) -> (String, std::fs::File, std::fs::Metadata) {
        (self.path, self.handle, self.metadata)
    }


    //metadata delagates
    ///Returns the last access time of the file.
    ///Derived from [`std::fs::Metadata`].
    pub fn accessed(&self) -> io::Result<SystemTime> {
        self.metadata.accessed()
    }

    ///Returns the creation time of the file.
    ///Derived from [`std::fs::Metadata`].
    pub fn created(&self) -> io::Result<SystemTime> {
        self.metadata.created()
    }

    ///Returns the last modification time listed in the file.
    ///Derived from [`std::fs::Metadata`].
    pub fn modified(&self) -> io::Result<SystemTime> {
        self.metadata.modified()
    }

    ///Returns the permissions of the file.
    ///Derived from [`std::fs::Metadata`].
    #[must_use] pub fn permissions(&self) -> std::fs::Permissions {
        self.metadata.permissions()
    }

    ///Returns the path of the file. 
    #[must_use] pub fn path(&self) -> &str {
        self.path.as_ref()
    }
    
    ///Converts a read-only handle to a write-only handle. Calling this on an already WO handle
    ///will do nothing.
    pub fn to_write(&mut self) -> io::Result<()> {
        self.handle = std::fs::File::create(&self.path)?;
        Ok(())
    }

    ///Converts a write-only handle to a read-only handle. Calling this on an already RO handle
    ///will do nothing.
    pub fn to_read(&mut self) -> io::Result<()> {
        self.handle = std::fs::File::open(&self.path)?;
        Ok(())
    }

}

impl io::Read for EzFile {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        //self.to_read()?;
        self.handle.read(buf)
    }
} 
impl io::Write for EzFile {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        //self.to_write()?;
        self.handle.write(buf) 
    }

    fn flush(&mut self) -> io::Result<()> {
        self.handle.flush() 
    }
}

impl Display for EzFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.path())
    }
}


