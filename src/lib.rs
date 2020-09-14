//! This crate grants a enum with one variant for each file type.
//!
//! **Cross-platform and small**, this crate has a single file with around _150_
//! lines of source code. Simplest implementation. if you want to check file
//! types, here's a _enum_ for you, don't rewrite it.
//!
//! # Enum FileType:
//! ```rust
//! pub enum FileType {
//!     File,
//!     Directory,
//!     Symlink,
//!     BlockDevice, // unix only
//!     CharDevice,  // unix only
//!     Fifo,        // unix only
//!     Socket,      // unix only
//! }
//! ```
//!
//! # Examples:
//! ```rust
//! use file_type_enum::FileType;
//!
//! fn main() {
//!     let path = "/tmp";
//!     let file_type = FileType::from_path(path).unwrap();
//!
//!     println!("There's a {} at {}!", file_type, path);
//!     // Outputs: "There's a directory at /tmp!"
//! }
//! ```
//!
//! Note: `FileType::from_path(path)` returns a `io::Error` if:
//! * Path does not exist.
//! * The user lacks permissions to read metadata from `path`.
//!
//! ---
//!
//! For each variant, there's a short hand `.is_VARIANT()`:
//!
//! `file_type.is_file()`      for `FileType::File`, \
//! `file_type.is_directory()` for `FileType::Directory`, \
//! `file_type.is_symlink()`   for `FileType::Symlink`, \
//! _And so on..._
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! fn main() {
//!     let path = ".git";
//!     let file_type = FileType::from_path(path).unwrap();
//!
//!     if file_type.is_directory() {
//!         println!("We are at the root a git repository.");
//!     }
//! }
//! ```
//!
//! ---
//!
//! By default, if `path` points to _symlink_, then `FileType::from_path()`
//! considers the path at the _symlink_'s target location (this implies that the
//! returned file type can't be `FileType::Symlink`).
//!
//! If you don't want to follow _symlinks_, use `FileType::from_symlink_path`
//! instead, this function may return `Ok(FileType::Symlink)`.
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! fn main() {
//!     let path = "/dev/stdout";
//!     let file_type = FileType::from_symlink_path(path).unwrap();
//!
//!     println!("There's a {} at {}!", file_type, path);
//!     // Outputs: "There's a symbolic link at /dev/stdout!"
//! }
//! ```
//!
//! ---
//!
//! The conversion `FileType::from::<fs::FileType>` is also available for
//! convenience.
//!
//! # Helping and contributing:
//! It's easy to contribute to this crate, here are some options:
//! - Share it to a friend.
//! - Help improve this README.md, even with little details.
//! - Open issues to the repository, or send a PR.
//! - Leave a star on GitHub.
//! - Use it!
//!
//! # TODO:
//! Add optional feature to transform from and into `libc`'s `mode_t`

use std::{fmt, fs, io, path::Path};

#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

/// Enum with a variant for each file type.
/// ```ignore
/// match file_type {
///     FileType::File        => { /* ... */ },
///     FileType::Directory   => { /* ... */ },
///     FileType::Symlink     => { /* ... */ },
///     FileType::BlockDevice => { /* ... */ },
///     FileType::CharDevice  => { /* ... */ },
///     FileType::Fifo        => { /* ... */ },
///     FileType::Socket      => { /* ... */ },
/// }
/// ```
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    #[cfg(unix)]
    BlockDevice,
    #[cfg(unix)]
    CharDevice,
    #[cfg(unix)]
    Fifo,
    #[cfg(unix)]
    Socket,
}

impl FileType {
    /// Try to get `FileType` from a path.
    ///
    /// This function follows symlinks, so it can never return a
    /// `FileType::Symlink`.
    ///
    /// # Example:
    /// ```rust
    /// use file_type_enum::FileType;
    ///
    /// fn main() {
    ///     let path = "/dev/tty";
    ///     let file_type = FileType::from_path(path).unwrap();
    ///
    ///     println!("There's a {} at {}!", file_type, path);
    ///     // Outputs: "There's a char device at /dev/tty!"
    /// }
    /// ```
    ///
    /// # Errors:
    /// - Path does not exist.
    /// - The user lacks permissions to run `fs::metadata(path)`.
    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let fs_file_type = fs::metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    /// Try to get `FileType` from a path.
    ///
    /// Don't follow symlinks, so the result can be the variant
    /// `FileType::Symlink` too.
    ///
    /// # Example:
    /// ```rust
    /// use file_type_enum::FileType;
    ///
    /// fn main() {
    ///     let path = "/dev/stdout";
    ///     let file_type = FileType::from_symlink_path(path).unwrap();
    ///
    ///     println!("There's a {} at {}!", file_type, path);
    ///     // Outputs: "There's a symlink at /dev/stdout!"
    /// }
    /// ```
    ///
    /// # Errors:
    /// - Path does not exist.
    /// - The user lacks permissions to run `fs::symlink_metadata(path)`.
    pub fn from_symlink_path(path: impl AsRef<Path>) -> Result<Self, io::Error> {
        let fs_file_type = fs::symlink_metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    pub fn is_file(&self) -> bool {
        match self {
            FileType::File => true,
            _ => false,
        }
    }

    pub fn is_directory(&self) -> bool {
        match self {
            FileType::Directory => true,
            _ => false,
        }
    }

    pub fn is_symlink(&self) -> bool {
        match self {
            FileType::Symlink => true,
            _ => false,
        }
    }

    #[cfg(unix)]
    pub fn is_block_device(&self) -> bool {
        match self {
            FileType::BlockDevice => true,
            _ => false,
        }
    }

    #[cfg(unix)]
    pub fn is_char_device(&self) -> bool {
        match self {
            FileType::CharDevice => true,
            _ => false,
        }
    }

    #[cfg(unix)]
    pub fn is_fifo(&self) -> bool {
        match self {
            FileType::Fifo => true,
            _ => false,
        }
    }

    #[cfg(unix)]
    pub fn is_socket(&self) -> bool {
        match self {
            FileType::Socket => true,
            _ => false,
        }
    }
}

impl From<fs::FileType> for FileType {
    fn from(ft: fs::FileType) -> Self {
        // Check each type
        #[cfg(unix)]
        let result = {
            if ft.is_file() {
                FileType::File
            } else if ft.is_dir() {
                FileType::Directory
            } else if ft.is_symlink() {
                FileType::Symlink
            } else if ft.is_block_device() {
                FileType::BlockDevice
            } else if ft.is_char_device() {
                FileType::CharDevice
            } else if ft.is_fifo() {
                FileType::Fifo
            } else if ft.is_socket() {
                FileType::Socket
            } else {
                unreachable!("unknown file type {:?} encountered", ft)
            }
        };

        #[cfg(not(unix))]
        let result = {
            if ft.is_file() {
                FileType::File
            } else if ft.is_dir() {
                FileType::Directory
            } else if ft.is_symlink() {
                FileType::Symlink
            } else {
                unreachable!("unknown file type {:?} encountered", ft)
            }
        };

        result
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::File => write!(f, "regular file"),
            FileType::Directory => write!(f, "directory"),
            FileType::Symlink => write!(f, "symbolic link"),
            #[cfg(unix)]
            FileType::BlockDevice => write!(f, "block device"),
            #[cfg(unix)]
            FileType::CharDevice => write!(f, "char device"),
            #[cfg(unix)]
            FileType::Fifo => write!(f, "FIFO"),
            #[cfg(unix)]
            FileType::Socket => write!(f, "socket"),
        }
    }
}
