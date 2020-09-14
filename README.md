This crate grants a enum with one variant for each file type.

**Cross-platform and small**, this crate has a single file with around _150_
lines of source code. Simplest implementation, should be in `std`. If you
want to check file types, here's a _enum_ for you, don't rewrite it.

# Enum FileType:
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

# Examples:
```rust
use file_type_enum::FileType;

fn main() {
    let path = "/tmp";
    let file_type = FileType::from_path(path).unwrap();

    println!("There's a {} at {}!", file_type, path);
    // Outputs: "There's a directory at /tmp!"
}
```

Note: `FileType::from_path(path)` returns a `io::Error` if:
* Path does not exist.
* The user lacks permissions to read metadata from `path`.

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

By default, if `path` points to _symlink_ `FileType::from_path()` considers
the path at the symlink's target location (this implies that the returned
file type can't be `FileType::Symlink`).

If you don't wanna follow _symlinks_, use `FileType::from_symlink_path`
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

`FileType::from::<fs::FileType>(fs_ft)` is also available.

# Helping and contributing:
It's easy to contribute to this crate, here are some options:
- Share it to a friend.
- Help improve this README.md, even with little details.
- Open issues to the repository.
- Leave a star on GitHub.
- Use it!

# TODO:
Add optional feature to transform from and into `libc`'s `mode_t`
