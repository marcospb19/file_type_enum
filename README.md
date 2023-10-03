# file_type_enum

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

## Example:

```rust
use std::{fs, io, path::Path};

use file_type_enum::FileType;

fn move_file(from: &Path, to: &Path) -> io::Result<()> {
    let from_type = FileType::symlink_read_at(from)?;
    let to_type = FileType::symlink_read_at(to)?;

    use FileType::{Directory, Regular, Symlink};

    match (from_type, to_type) {
        (Directory, Directory) => {
            println!("Replacing directory {to:?} by directory {from:?}.");
        }
        (Regular, Regular) | (Symlink, Symlink) => {
            // Might print:
            //       "Overwriting regular file at PATH."
            //       "Overwriting symbolic link at PATH."
            println!("Overwriting {from_type} at {to:?} by {to:?}.");
        }
        (_, Directory) => {
            println!("Moving file at {from:?} into folder {to:?}.");
            fs::rename(from, to)?;
        }
        (_, _) => {
            // Might print:
            // -   "Cannot overwrite regular file  with a symbolic link."
            // -   "Cannot overwrite directory     with a symbolic link."
            // -   "Cannot overwrite symbolic link with a regular file."
            panic!("Cannot overwrite {to_type}     with a {from_type}.");
        }
    }

    Ok(())
}
```

As shown in the example `FileType` also implements `Display`.

## Warning

Note that, like `std` functions, [`FileType::read_at`] follows symlinks, therefore it is
impossible to get the [`FileType::Symlink`] variant. If you want symlink-awareness, use
[`FileType::symlink_read_at`] instead.

## Conversions

- From [`AsRef<Path>`], [`fs::Metadata`] and [std's `FileType`].
- From and into [`libc::mode_t`] (via the feature `"mode-t-conversion"`).

[`AsRef<Path>`]: https://doc.rust-lang.org/std/path/struct.Path.html
[`FileType::read_at`]: https://docs.rs/file_type_enum/latest/file_type_enum/enum.FileType.html#method.read_at
[`FileType::symlink_read_at`]: https://docs.rs/file_type_enum/latest/file_type_enum/enum.FileType.html#method.symlink_read_at
[`fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
[`libc::mode_t`]: https://docs.rs/libc/latest/libc/type.mode_t.html
[std's `FileType`]: https://doc.rust-lang.org/std/fs/struct.FileType.html
[`FileType::Symlink`]: https://docs.rs/file_type_enum/latest/file_type_enum/enum.FileType.html#variant.Symlink
