# file_type_enum

[![Crates.io](https://img.shields.io/crates/v/file_type_enum.svg)](https://crates.io/crates/file_type_enum)
[![Rust](https://github.com/marcospb19/file_type_enum/workflows/Rust/badge.svg?branch=main)](https://github.com/marcospb19/file_type_enum/actions?query=workflow%3ARust)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/file_type_enum/blob/main/LICENSE)
[![Docs.rs](https://docs.rs/file_type_enum/badge.svg)](https://docs.rs/file_type_enum)

This crate grants a enum with one variant for each file type.

Cross-platform, this crate is made of a `lib.rs` with less than 200 lines of
source code with a very simple enum implementation so that you don't have to
rewrite your own.

## Enum [`FileType`]:
```rust
pub enum FileType {
    File,
    Directory,
    Symlink,
    BlockDevice, // unix only
    CharDevice,  // unix only
    Fifo,        // unix only
    Socket,      // unix only
}
```

## Examples:
```rust
use file_type_enum::FileType;

fn main() {
    let path = "/tmp";
    let file_type = FileType::from_path(path).unwrap();

    println!("There's a {} at {}!", file_type, path);
    // Outputs: "There's a directory at /tmp!"
}
```

### Errors:
`
* If path does not exist
* Or current user can't  permissions to read type information (metadata)
  from `path`.

---

For each variant, there's a short hand `.is_VARIANT()`:

`file_type.is_file()`      for `FileType::File`, \
`file_type.is_directory()` for `FileType::Directory`, \
`file_type.is_symlink()`   for `FileType::Symlink`, \
_And so on..._

```rust
use file_type_enum::FileType;

fn main() {
    let path = ".git";
    let file_type = FileType::from_path(path).unwrap();

    if file_type.is_directory() {
        println!("We are at the root a git repository.");
    }
}
```

---

By default, if `path` points to _symlink_, then `FileType::from_path()`
considers the path at the _symlink_'s target location (this implies that the
returned file type can't be `FileType::Symlink`).

If you don't want to follow _symlinks_, use `FileType::from_symlink_path`
instead, this function may return `Ok(FileType::Symlink)`.

```rust
use file_type_enum::FileType;

fn main() {
    let path = "/dev/stdout";
    let file_type = FileType::from_symlink_path(path).unwrap();

    println!("There's a {} at {}!", file_type, path);
    // Outputs: "There's a symbolic link at /dev/stdout!"
}
```

---

## Conversions

The `From` is implemented for the types:
- `std::fs::FileType`
- `libc::mode_t`

The conversion [`FileType::from::<fs::FileType>`](FileType) is also
available for convenience.

## Future versions note:
Changes might occur soon in future versions of the `std` _API_ for `Windows`
symlinks (there are two types of symlink in `Windows`) or any other file
types, when this happen, this crate will probably change it's _API_ too to
be up to date with it. If you spot it before than me, please open a issue in
the repository of this project.

## Helping and contributing:
It's easy to contribute to this crate, here are some options:
- Share it to a friend.
- Help improve this README.md, even with little details.
- Open an issue or PR in the repository.
- Leave a star on GitHub.
- Use it!
