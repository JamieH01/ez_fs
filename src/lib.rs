//#![warn(clippy::suspicious)]
//#![warn(clippy::style)]
//#![warn(clippy::complexity)]
//#![warn(clippy::perf)]
//#![warn(clippy::pedantic)]
//#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

//!# ez_fs
//!
//!`ez_fs` is a Rust library that provides convenient abstractions for working with files and directories. It aims to simplify common file and directory operations by providing easy-to-use interfaces and utilities.
//!
//!## Features
//!
//!- Wrapper for `std::fs` types, providing a simplified and more ergonomic API.
//!- Lazy-loading directory representation with the ability to cache and walk through subdirectories.
//!- Flattening directories into vectors of files for easy traversal and manipulation.
//!
//!## Getting Started
//!
//!### Reading/Writing to Files
//!```rust
//!//open file in write-only mode
//!let mut file = EzFile::create("foo.txt").unwrap();
//!file.write_all(b"bar").unwrap();
//!
//!//change to read-only
//!file.to_read().unwrap();
//!
//!let mut buf = String::new();
//!file.read_to_string(&mut buf).unwrap();
//!assert_eq!(buf, "bar");
//!```
//!
//!### Collecting Directories
//!```rust
//!//open an existing directory
//!let dir = EzDir::new(".", true).unwrap();
//!
//!//recursively can subdirectories and collect all files
//!let files = dir.flatten_all();
//!for file in files {
//!    println!("{file}")
//!}
//!```


mod file;
mod dir;
pub use crate::{dir::*, file::*};

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};

    use super::*; 

    #[test]
    fn rw_test() {
        //open file in write-only mode
        let mut file = EzFile::create("foo.txt").unwrap();
        file.write_all(b"bar").unwrap();

        //change file to read-only mode
        file.to_read().unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();
        assert_eq!(buf, "bar");
    }

    #[test]
    fn dir_test() {
        let dir = EzDir::new(".", true).unwrap();
        println!("{dir:?}");
    }

    #[test]
    fn iter_test() {
        let mut dir = EzDir::new(".", true).unwrap();
        for entry in dir.iter() {
            println!("{entry:?}") 
        }
        for entry in dir.iter_mut() {
            println!("{entry:?}") 
        }
        for entry in dir.into_iter() {
            println!("{entry:?}") 
        }
    }

    #[test]
    fn walk_dirs() {
        let mut dir = EzDir::new(".", true).unwrap();
        //recursively walks all subdirectories
        dir.walk(0);
        println!("{dir}");
    }

    #[test]
    fn display_test() {
        let dir = EzDir::new(".", true).unwrap();
        for elem in dir {
            println!("{elem}")
        }
    }

    #[test]
    fn flatten() {
        let dir = EzDir::new(".", true).unwrap();
        let files = dir.flatten();
        for file in files {
            println!("{file}")
        }
    }
}
