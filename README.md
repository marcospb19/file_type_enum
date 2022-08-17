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

- [`Path::is_file`](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_file).
- [`Path::is_dir`](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_dir).
- [`Path::is_symlink`](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_symlink).

## Example

```rust
use file_type_enum::FileType;
use std::io;

fn main() -> io::Result<()> {
    let file_type = FileType::from_path("/tmp")?;

    println!("There's a {} at /tmp", file_type);
    // Out:  "There's a directory at /tmp"

    Ok(())
}
```

Note that the [`FileType::from_path`] follows symlinks and [`FileType::from_symlink_path`] does not.

[`FileType::from_path`]: https://docs.rs/file_type_enum/latest/file_type_enum/enum.FileType.html#method.from_path
[`FileType::from_symlink_path`]: https://docs.rs/file_type_enum/latest/file_type_enum/enum.FileType.html#method.from_symlink_path

## Conversions

- From [`AsRef<Path>`], [`fs::Metadata`] and [std's `FileType`].
- From and into [`libc::mode_t`] (via the feature `"mode-t-conversion"`).

[`AsRef<Path>`]: https://doc.rust-lang.org/std/path/struct.Path.html
[`fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
[std's `FileType`]: https://doc.rust-lang.org/std/fs/struct.FileType.html
[`libc::mode_t`]: https://docs.rs/libc/latest/libc/type.mode_t.html

## Contributing

Issues and PRs are welcome.

License: MIT
