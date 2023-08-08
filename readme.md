# ez_fs

`ez_fs` is a Rust library that provides convenient abstractions for working with files and directories. It aims to simplify common file and directory operations by providing easy-to-use interfaces and utilities.

## Features

- Wrapper for `std::fs` types, providing a simplified and more ergonomic API.
- Lazy-loading directory representation with the ability to cache and walk through subdirectories.
- Flattening directories into vectors of files for easy traversal and manipulation.

`ez_fs` is meant to simplify a majority of use cases; if you need more specific features you should fall back to the `std::fs` module.
Everything is prefixed with "Ez" to be unambiguous when using both `ez_fs` and `std::fs`.

## Getting Started

### Reading/Writing to Files
```rust
//open file in write-only mode
let mut file = EzFile::create("foo.txt").unwrap();
file.write_all(b"bar").unwrap();

//change to read-only
file.to_read().unwrap();

let mut buf = String::new();
file.read_to_string(&mut buf).unwrap();
assert_eq!(buf, "bar");
```

### Collecting Directories
```rust
//open an existing directory
let dir = EzDir::new(".", true).unwrap();

//recursively can subdirectories and collect all files
let files = dir.flatten_all();
for file in files {
    println!("{file}")
}
```

## Contributing
`ez_fs` was created due to my own personal frustrations. There are certain aspects such as simlinks that ive chosen to ignore.
My code certainly isnt perfect, so feel welcome to send an issue or PR!
