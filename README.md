# file_type_enum

[![Crates.io](https://img.shields.io/crates/v/file_type_enum.svg)](https://crates.io/crates/file_type_enum)
[![Docs.rs](https://docs.rs/file_type_enum/badge.svg)](https://docs.rs/file_type_enum)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/file_type_enum/blob/main/LICENSE)

A enum with one variant for each file type.

Cross-platform, this crate is made of a single small `lib.rs` with a very
simple [enum](FileType) implementation so that you don't have to rewrite
your own.

## Enum `FileType`:
```rust
pub enum FileType {
    Regular,
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

let path = "/tmp";
let file_type = FileType::from_path(path).unwrap();

println!("There's a {} at {}!", file_type, path);
// Outputs: "There's a directory at /tmp!"
```

### Errors:
- If path does not exist, or
- Current user don't have permissions to read `fs::Metadata` from `path`.

---

For each variant, there is also a short hand method:

```rust
let ft = FileType::from(path);
if ft.is_regular() { ... }
if ft.is_directory() { ... }
if ft.is_symlink() { ... }
if ft.is_block_device() { ... }
if ft.is_char_device() { ... }
if ft.is_fifo() { ... }
if ft.is_socket() { ... }
```

```rust
use file_type_enum::FileType;

let path = ".git";
let file_type = FileType::from_path(path).unwrap();

if file_type.is_directory() {
    println!("We are at the root of a git repository.");
}
```

---

If `path` points to a _symlink_, `from_path(path)` follows it, so the
returned type can never be a _symlink_.

To avoid this, use `FileType::from_symlink_path`, this don't follow, and can
return a _symlink_.

```rust
use file_type_enum::FileType;

let path = "/dev/stdout";
let file_type = FileType::from_symlink_path(path).unwrap();

println!("There's a {} at {}!", file_type, path);
// Outputs: "There's a symbolic link at /dev/stdout!"
```

---

## Conversions
- From `std::fs::FileType`.
- From and into `libc::mode_t` (enable `mode-t-conversion` optional
  feature).

## Future versions note:
Changes might occur on `std` _API_ for `Windows` (related to _symlinks_), I
personally don't consider this part very stable.

## Helping and contributing:
It's easy to contribute to this crate, here are some options:
- Share it to a friend.
- Help improve this README.md, even with little details.
- Open an issue or PR in the repository.
- Leave a star on GitHub.
- Use it!!!

#### TODO:
Add example on how to add the crate with the feature to the Cargo.toml.
