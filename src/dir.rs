use crate::file::EzFile;
use std::{io, fs, path::Path, fmt::Display};

macro_rules! io_err {
    ($err:tt) => {
        io::Error::new(io::ErrorKind::Other, $err)   
    };
}

///Representation of a directory.
///Directories are lazily evaluated, and will not be scanned until asked to.
#[derive(Debug)]
pub struct EzDir {
    path: String,
    entries: Option<Vec<EzEntry>>
}
impl EzDir {
    ///Constructs a new directory from a given path.
    ///Directories are lazy, so pass `cache` as `true` to fill the directory.
    ///This is so that subdirectories are not walked immediately.
    ///```
    ///use ez_fs::EzDir;
    ///
    ///let dir = EzDir::new(".", true).unwrap();
    ///
    ///assert!(dir.is_cached())
    ///```
    ///# Errors
    ///This function will error if path does not exist.
    pub fn new(path: &str, cache: bool) -> io::Result<Self> {
        let dir = Path::new(path);
        if dir.is_dir() {
            if cache {
                let dir = fs::read_dir(path)?
                    .filter_map(|e| e.and_then(EzEntry::try_from).ok())
                    .collect();
                Ok(Self {path: path.to_owned(), entries: Some(dir)}) 
            } else {
                Ok(Self {path: path.to_owned(), entries: None}) 
            }
        } else {
            Err(io_err!("Path is not a directory"))
        }
    }


    ///Returns the path of this directory.
    ///```
    ///use ez_fs::EzDir;
    ///
    ///let dir = EzDir::new(".", false).unwrap();
    ///
    ///assert_eq!(dir.path(), ".");
    ///```
    #[must_use] pub fn path(&self) -> &str {
        self.path.as_ref()
    }
    
    ///Returns an iterator yielding references.
    pub fn iter(&self) -> std::slice::Iter<'_, EzEntry> {
        self.into_iter()
    }
    ///Returns an iterator yielding mutable references.
    pub fn iter_mut(&mut self) -> std::slice::IterMut<'_, EzEntry> {
        self.into_iter()
    }
    
    ///Returns `true` if this directory has been checked.
    ///```
    ///use ez_fs::EzDir;
    ///
    ///let dir = EzDir::new(".", false).unwrap();
    ///
    ///assert!(!dir.is_cached());
    ///```
    #[must_use] pub fn is_cached(&self) -> bool {
        self.entries.is_some()
    }
    ///Scans current directory and saves results. This will override/update an already scanned
    ///directory.
    ///```
    ///use ez_fs::EzDir;
    ///
    ///let mut dir = EzDir::new(".", false).unwrap();
    ///dir.cache();
    ///assert!(dir.is_cached());
    ///```
    pub fn cache(&mut self) {
        self.entries = Some(fs::read_dir(&self.path).unwrap()
            .filter_map(|e| e.and_then(EzEntry::try_from).ok())
            .collect())
    }

    ///Recursively fills subdirectories up to the specified depth. For example a depth of 1 will
    ///walk at most 1 subdirectory down. A depth of 0 will walk ALL subdirectories. Automatically
    ///caches dir before walking.
    ///```
    ///use ez_fs::EzDir;
    ///
    ///let mut dir = EzDir::new(".", true).unwrap();
    /////recursively walks all subdirectories
    ///dir.walk(0);
    ///println!("{dir}");
    ///```
    pub fn walk(&mut self, depth: usize) {
        fn fill(dir: &mut EzDir, curr: usize, max: usize) {
            for entry in dir.iter_mut() {
                if let EzEntry::Dir(d) = entry {
                    d.cache();
                    if curr < max-1 {fill(d, curr + 1, max)}
                }
            } 
        }
        
        self.cache(); 

        if depth > 0 {
            fill(self, 0, depth);
        } else {
            fill(self, 0, usize::MAX);
        }
    }

    ///Returns entry reference if the given index exists. Returns [`None`] if the index is out of bounds or the
    ///directory hasn't been cached.
    #[must_use] pub fn get(&self, idx:usize) -> Option<&EzEntry> {
        self.entries.as_ref()?.get(idx)
    }
    ///Returns mutable entry reference if the given index exists. Returns [`None`] if the index is out of bounds or the
    ///directory hasn't been cached.
    pub fn get_mut(&mut self, idx:usize) -> Option<&mut EzEntry> {
        self.entries.as_mut()?.get_mut(idx)
    }
    
    ///Returns the length of the directory or [`None`] if it hasn't been cached.
    #[must_use] pub fn len(&self) -> Option<usize> {
        self.entries.as_ref().map(std::vec::Vec::len)
    }
    ///Returns a boolean of whether or not the dir is empty, or [`None`] if it hasn't been cached.
    pub fn is_empty(&self) -> Option<bool> {
        self.entries.as_ref().map(std::vec::Vec::is_empty)
    } 

    
    ///Flattens a directory of files and subdirectories to a single vector of files.
    ///Only collects files that have been walked.
    ///```
    ///use ez_fs::EzDir;
    ///
    ///let dir = EzDir::new(".", true).unwrap();
    ///let files = dir.flatten();
    ///for file in files {
    ///    println!("{file}")
    ///}
    ///```
    #[must_use] pub fn flatten(self) -> Vec<EzFile> {
        fn collect(dir: EzDir, vec:&mut Vec<EzFile>) {
            for entry in dir {
                match entry {
                    EzEntry::File(file) => vec.push(*file),
                    EzEntry::Dir(dir) => collect(dir, vec),
                }
            }
        }
        
        let mut vec = vec![];
        collect(self, &mut vec);
        vec

    }

    ///Flattens a directory of files and subdirectories to a single vector of files.
    ///Collects ALL files, including those that have not been scanned yet.
    ///```
    ///use ez_fs::EzDir;
    ///
    ///let dir = EzDir::new(".", false).unwrap();
    ///let files = dir.flatten_all();
    ///for file in files {
    ///    println!("{file}")
    ///}
    ///```
    #[must_use] pub fn flatten_all(self) -> Vec<EzFile> {
        let mut dir = self;
        dir.walk(0); 
        dir.flatten()
    }

}

//just so i can generate an empty iterator
const EMPTY_ITER: [EzEntry; 0] = [];

impl IntoIterator for EzDir {
    type Item = EzEntry;

    type IntoIter = std::vec::IntoIter<EzEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.map_or(vec![].into_iter(), std::iter::IntoIterator::into_iter)
    }
}
impl<'i> IntoIterator for &'i EzDir {
    type Item = &'i EzEntry;

    type IntoIter = std::slice::Iter<'i, EzEntry>;

    fn into_iter(self) -> Self::IntoIter {
        self.entries.as_ref().map_or(EMPTY_ITER.iter(), |v| v.iter())
    }
}
impl<'i> IntoIterator for &'i mut EzDir {
    type Item = &'i mut EzEntry;

    type IntoIter = std::slice::IterMut<'i, EzEntry>;
    
    //hey whatever works
    #[allow(const_item_mutation)]
    fn into_iter(self) -> Self::IntoIter {
        self.entries.as_deref_mut().map_or(EMPTY_ITER.iter_mut(), <[EzEntry]>::iter_mut)
    }
}




#[derive(Debug)]
///Represents an entry in a directory.
pub enum EzEntry {
    File(Box<EzFile>),
    Dir(EzDir),
}

impl EzEntry {
    /// Returns `true` if the entry is [`File`].
    ///
    /// [`File`]: EzEntry::File
    #[must_use]
    pub fn is_file(&self) -> bool {
        matches!(self, Self::File(..))
    }

    /// Returns `true` if the entry is [`Dir`].
    ///
    /// [`Dir`]: EzEntry::Dir
    #[must_use]
    pub fn is_dir(&self) -> bool {
        matches!(self, Self::Dir(..))
    }
}

impl TryFrom<fs::DirEntry> for EzEntry {
    type Error = io::Error;

    fn try_from(value: fs::DirEntry) -> Result<Self, Self::Error> {
        let file_type = value.file_type()?;
        //this looks like a mess
        if file_type.is_file() {return 
            Ok(
                Self::File(
                    Box::new(EzFile::open(
                        value.path().to_str().ok_or(io::Error::new(io::ErrorKind::Other, "Error converting path"))?
                        )?)
                )
            )
        }
        
        if file_type.is_dir() {
            return Ok(Self::Dir(EzDir::new(value.path().to_str().ok_or(io::Error::new(io::ErrorKind::Other, "Error converting path"))?, false)?))
        }

        Err(io::Error::new(io::ErrorKind::Other, "Invalid file type (likely simlink)"))
    }
}

impl Display for EzDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for entry in self {
            match entry {
                EzEntry::File(file) => writeln!(f, "{}", file.path())?,
                EzEntry::Dir(dir) => write!(f, "{}\n{}", dir.path(), dir)?,
            }
        }
        Ok(())
    }
}

impl Display for EzEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File(file) => write!(f, "{}", file.path()),
            Self::Dir(dir) => write!(f, "{}", dir.path()),
        }
    }
}

