#[cfg(unix)]
mod unix;
#[cfg(unix)]
pub use unix::FileType;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::FileType;
