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
//!     if !file_type.is_dir() {
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

#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::FileType;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::FileType;
