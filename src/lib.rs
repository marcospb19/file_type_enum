//! Simple and minimal enum with one variant for each file type.
//!
//! This crate is very, very small. But it may save you from writing 120
//! redundant lines in your projects.
//!
//! Size for each target family:
//! - `unix.rs` - ~120 SLOC (Source lines of code)
//! - `windows.rs` - ~70 SLOC (Source lines of code)
//!
//! Here are some examples in `unix`, but `file_type_enum` works on `windows`
//! too!
//!
//! # Examples:
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! fn main() {
//!     let path = "/tmp";
//!     let file_type = FileType::try_from_path(path).unwrap();
//!
//!     println!("There's a {} at {}!", file_type, path);
//!     // Outputs: "There's a directory at /tmp!"
//! }
//! ```
//!
//! Note: `FileType::try_from_path()` fails if:
//! * The user lacks permissions to read metadata on the path.
//! * Path does not exist.
//!
//! ---
//!
//! By default, `FileType::try_from_path()` will follow symlinks, so the
//! `Result` will never retrieve a `FileType::Symlink`, however, we have a
//! specific function if you don't want it to follow symlinks:
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! fn main() {
//!     let path = "/dev/stdout";
//!     let file_type = FileType::try_from_symlink_path(path).unwrap();
//!
//!     println!("There's a {} at {}!", file_type, path);
//!     // Outputs: "There's a symbolic link at /dev/stdout!"
//! }
//! ```
//!
//! ---
//!
//! # Enum FileType:
//! There are up to 7 file types in `unix` and 3 in `windows`:
//!
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
//! ---
//!
//! # Bool short hands for each type
//! Besides granting a variant for each file type, there is also a function for
//! each type to serve as a short hand to return a `bool`
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! fn main() {
//!     let path = ".git";
//!     let file_type = FileType::try_from_path(path).unwrap();
//!
//!     if !file_type.is_directory() {
//!         println!("We are not inside of a git repository.");
//!     }
//! }
//! ```
//!
//! ---
//!
//! # Last example, passing the `io::Error` around.
//! Show file type for every `env::args()`:
//!
//! ```rust
//! use file_type_enum::FileType;
//!
//! use std::{env, io};
//!
//! fn main() -> Result<(), io::Error> {
//!     for path in env::args().skip(1) {
//!         let file_type = FileType::try_from_path(&path)?;
//!         println!("'{}' is a {}.", path, file_type);
//!     }
//!     Ok(())
//! }
//! ```

use std::{fmt, fs, io, path::Path};

#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;

/// # Variants:
///
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
    /// FileType::Symlink.
    pub fn from_path(path: &dyn AsRef<Path>) -> Result<Self, io::Error> {
        let fs_file_type = fs::metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    /// Try to get `FileType` from a path.
    ///
    /// Don't follow symlinks, so the result can be the variant
    /// `FileType::Symlink` too.
    pub fn from_symlink_path(path: &dyn AsRef<Path>) -> Result<Self, io::Error> {
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
        // Check each type, except for symlink, because fs::metadata() follows symlinks
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
                panic!();
            }
        };

        #[cfg(windows)]
        let result = {
            if ft.is_file() {
                FileType::File
            } else if ft.is_dir() {
                FileType::Directory
            } else if ft.is_symlink() {
                FileType::Symlink
            } else {
                panic!();
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
