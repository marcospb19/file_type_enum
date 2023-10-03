#![warn(missing_docs)]

//! An enum with a variant for each file type.
//!
//! ```
//! pub enum FileType {
//!     Regular,
//!     Directory,
//!     Symlink,
//!     BlockDevice, // unix only
//!     CharDevice,  // unix only
//!     Fifo,        // unix only
//!     Socket,      // unix only
//! }
//! ```
//!
//! # Alternatives:
//!
//! 1. If you want a enum tree, check the crate [`fs-tree`](https://docs.rs/fs-tree).
//! 2. If you don't need an enum, check these methods from `std` instead:
//!     - [`Path::is_file`](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_file).
//!     - [`Path::is_dir`](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_dir).
//!     - [`Path::is_symlink`](https://doc.rust-lang.org/std/path/struct.Path.html#method.is_symlink).
//!
//! # Example:
//!
//! ```
//! use std::{fs, io, path::Path};
//!
//! use file_type_enum::FileType;
//!
//! fn move_file(from: &Path, to: &Path) -> io::Result<()> {
//!     let from_type = FileType::symlink_read_at(from)?;
//!     let to_type = FileType::symlink_read_at(to)?;
//!
//!     use FileType::{Directory, Regular, Symlink};
//!
//!     match (from_type, to_type) {
//!         (Directory, Directory) => {
//!             println!("Replacing directory {to:?} by directory {from:?}.");
//!         }
//!         (Regular, Regular) | (Symlink, Symlink) => {
//!             // Might print:
//!             //       "Overwriting regular file at PATH."
//!             //       "Overwriting symbolic link at PATH."
//!             println!("Overwriting {from_type} at {to:?} by {to:?}.");
//!         }
//!         (_, Directory) => {
//!             println!("Moving file at {from:?} into folder {to:?}.");
//!             fs::rename(from, to)?;
//!         }
//!         (_, _) => {
//!             // Might print:
//!             // -   "Cannot overwrite regular file  with a symbolic link."
//!             // -   "Cannot overwrite directory     with a symbolic link."
//!             // -   "Cannot overwrite symbolic link with a regular file."
//!             panic!("Cannot overwrite {to_type}     with a {from_type}.");
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! As shown in the example `FileType` also implements `Display`.
//!
//! # Warning
//!
//! Note that, like `std` functions, [`FileType::read_at`] follows symlinks, therefore it is
//! impossible to get the [`FileType::Symlink`] variant. If you want symlink-awareness, use
//! [`FileType::symlink_read_at`] instead.
//!
//! # Conversions
//!
//! - From [`AsRef<Path>`], [`fs::Metadata`] and [std's `FileType`].
//! - From and into [`libc::mode_t`] (via the feature `"mode-t-conversion"`).
//!
//! [`AsRef<Path>`]: https://doc.rust-lang.org/std/path/struct.Path.html
//! [`fs::Metadata`]: https://doc.rust-lang.org/std/fs/struct.Metadata.html
//! [std's `FileType`]: https://doc.rust-lang.org/std/fs/struct.FileType.html
//! [`libc::mode_t`]: https://docs.rs/libc/latest/libc/type.mode_t.html

#[cfg(feature = "mode-t-conversion")]
mod mode_t_conversion_feature;

#[cfg(unix)]
use std::os::unix::fs::FileTypeExt;
use std::{fmt, fs, io, path::Path};

#[cfg(feature = "mode-t-conversion")]
pub use mode_t_conversion_feature::*;

/// An enum with a variant for each file type.
///
/// ```
/// # use file_type_enum::FileType;
/// # let file_type = FileType::read_at("src/").unwrap();
/// match file_type {
///     FileType::Regular     => {},
///     FileType::Directory   => {},
///     FileType::Symlink     => {},
///     FileType::BlockDevice => {}, // unix only
///     FileType::CharDevice  => {}, // unix only
///     FileType::Fifo        => {}, // unix only
///     FileType::Socket      => {}, // unix only
/// }
/// ```
#[rustfmt::skip]
#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy, Ord, PartialOrd)]
pub enum FileType {
    /// A regular file.
    Regular,
    /// A directory, folder of files.
    Directory,
    /// A symbolic link, points to another path.
    Symlink,
    /// Unix block device.
    #[cfg(unix)] BlockDevice,
    /// Unix char device.
    #[cfg(unix)] CharDevice,
    /// Unix FIFO.
    #[cfg(unix)] Fifo,
    /// Unix socket.
    #[cfg(unix)] Socket,
}

impl FileType {
    /// Reads a `FileType` from a path.
    ///
    /// This function follows symlinks, so it can never return a `FileType::Symlink`.
    ///
    /// # Errors
    ///
    /// - Path does not exist, or
    /// - Current user lacks permissions to read `fs::Metadata` of `path`.
    pub fn read_at(path: impl AsRef<Path>) -> io::Result<Self> {
        let fs_file_type = fs::metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    /// Reads a `FileType` from a path, considers symlinks.
    ///
    /// This function does not follow symlinks, therefore, `FileType::Symlink` can be returned, if
    /// you don't want that, see [`FileType::read_at`].
    ///
    /// # Errors
    ///
    /// - Path does not exist, or
    /// - Current user lacks permissions to read `fs::Metadata` of `path`.
    pub fn symlink_read_at(path: impl AsRef<Path>) -> io::Result<Self> {
        let fs_file_type = fs::symlink_metadata(path.as_ref())?.file_type();
        let result = FileType::from(fs_file_type);
        Ok(result)
    }

    /// Returns true if is a [`FileType::Regular`].
    pub fn is_regular(&self) -> bool {
        matches!(self, FileType::Regular)
    }

    /// Returns true if is a [`FileType::Directory`].
    pub fn is_directory(&self) -> bool {
        matches!(self, FileType::Directory)
    }

    /// Returns true if is a [`FileType::Symlink`].
    pub fn is_symlink(&self) -> bool {
        matches!(self, FileType::Symlink)
    }

    /// Returns true if is a [`FileType::BlockDevice`].
    #[cfg(unix)]
    pub fn is_block_device(&self) -> bool {
        matches!(self, FileType::BlockDevice)
    }

    /// Returns true if is a [`FileType::CharDevice`].
    #[cfg(unix)]
    pub fn is_char_device(&self) -> bool {
        matches!(self, FileType::CharDevice)
    }

    /// Returns true if is a [`FileType::Fifo`].
    #[cfg(unix)]
    pub fn is_fifo(&self) -> bool {
        matches!(self, FileType::Fifo)
    }

    /// Returns true if is a [`FileType::Socket`].
    #[cfg(unix)]
    pub fn is_socket(&self) -> bool {
        matches!(self, FileType::Socket)
    }
}

impl From<fs::FileType> for FileType {
    fn from(ft: fs::FileType) -> Self {
        // Check each type
        #[cfg(unix)]
        let result = {
            if ft.is_file() {
                FileType::Regular
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
                unreachable!("file_type_enum: unexpected file type: {:?}.", ft)
            }
        };

        #[cfg(not(unix))]
        let result = {
            if ft.is_file() {
                FileType::Regular
            } else if ft.is_dir() {
                FileType::Directory
            } else if ft.is_symlink() {
                FileType::Symlink
            } else {
                unreachable!("file_type_enum: unexpected file type: {:?}.", ft)
            }
        };

        result
    }
}

impl From<fs::Metadata> for FileType {
    fn from(metadata: fs::Metadata) -> Self {
        metadata.file_type().into()
    }
}

impl fmt::Display for FileType {
    #[rustfmt::skip]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FileType::Regular => write!(f, "regular file"),
            FileType::Directory => write!(f, "directory"),
            FileType::Symlink => write!(f, "symbolic link"),
            #[cfg(unix)] FileType::BlockDevice => write!(f, "block device"),
            #[cfg(unix)] FileType::CharDevice => write!(f, "char device"),
            #[cfg(unix)] FileType::Fifo => write!(f, "FIFO"),
            #[cfg(unix)] FileType::Socket => write!(f, "socket"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FileType;

    #[test]
    fn test_with_this_repository_structured() {
        let this_file = FileType::read_at("src/lib.rs").unwrap();
        assert!(this_file.is_regular());
    }
}
