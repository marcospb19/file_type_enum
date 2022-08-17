# file_type_enum

[![Crates.io](https://img.shields.io/crates/v/file_type_enum.svg)](https://crates.io/crates/file_type_enum)
[![Docs.rs](https://docs.rs/file_type_enum/badge.svg)](https://docs.rs/file_type_enum)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/marcospb19/file_type_enum/blob/main/LICENSE)

An enum with a variant for each file type.

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

If you don't need an enum, check these methods from `std` instead:

- [`Path::is_file`].
- [`Path::is_dir`].
- [`Path::is_symlink`].

## Example

```rust
use file_type_enum::FileType;

fn main() -> io::Result<()> {
    let file_type = FileType::from_path("/tmp")?;

    println!("There's a {} at {}!", file_type, path);
    // Out:  "There's a directory at /tmp!"

    Ok(())
}
```

Note that the [`FileType::from_path`] follows symlinks and [`FileType::from_symlink_path`] does not.

## Conversions

- From [`AsRef<Path>`], [`fs::Metadata`] and [std's `FileType`].
- From and into [`libc::mode_t`] (via the feature `"mode-t-conversion"`).

## Contributing

Issues and PRs are welcome.

[`AsRef<Path>`]: std::path::Path
[`fs::Metadata`]: std::fs::Metadata
[std's `FileType`]: std::fs::FileType

License: MIT
