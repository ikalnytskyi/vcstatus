mod git;
mod hg;

use std::error::Error;
use std::path::Path;

pub use self::git::Git;
pub use self::hg::Hg;


/// List of supported VCS.
///
/// Each VCS must implement the `VCS` trait as well as `fn new(&Path)`
/// static method that returns `Option<Box<VCS>>`. By conventation,
/// if a given path is a root of VCS then pointer to VCS is returned.
/// Otherwise - `None`.
static SUPPORTED_VCS: [fn (&Path) -> Option<Box<VCS>>; 2] = [
    Git::new,
    Hg::new,
];


/// VCS interface definition.
pub trait VCS {
    /// Returns a human readable VCS name.
    fn name(&self) -> &str;

    /// Returns an active VCS branch.
    ///
    /// # Errors
    ///
    /// The error is returned if by any reason we failed to retrieve
    /// a current active branch. The most possible reason is various
    /// I/O errors.
    fn branch(&self) -> Result<String, Box<Error>>;

    /// Returns `true` if uncommitted changes are detected.
    fn modified(&self) -> Result<bool, Box<Error>>;
}


/// Deduce and return a VCS implementation of the given path.
///
/// # Arguments
///
/// * `path` - a path that should be checked for VCS
///
/// # Return
///
/// A boxed struct that implements the VCS trait if a given path is
/// under supported VCS; otherwise - `None`.
pub fn get_vcs(path: &Path) -> Option<Box<VCS>> {
    let mut pathbuf = path.to_path_buf();

    loop {
        for vcs_factory in &SUPPORTED_VCS {
            let vcs = vcs_factory(&pathbuf);
            if vcs.is_some() {
                return vcs;
            }
        }

        if !pathbuf.pop() { break }
    }

    None
}
