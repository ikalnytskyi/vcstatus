use std::error;
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};

use super::VCS;


pub struct Hg {
    /// a path to mercurial repo root
    pub root: PathBuf,
}


impl Hg {
    /// Creates a new `Hg` instance based on a given `path`. If `path` is
    /// not a Mercurial root, `None` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// match Hg::new(Path::new(".")) {
    ///     Some(hg) => println!("Yay! That's a mercurial root!"),
    ///     None => println!("Nah.. That's not a mercurial root."),
    /// }
    /// ```
    pub fn new(path: &Path) -> Option<Box<VCS>> {
        if path.join(".hg").exists() {
            Some(Box::new(Hg {root: path.to_path_buf()}))
        } else {
            None
        }
    }
}


/// VCS implementation for Mercurial.
///
/// # Example
///
/// ```rust
/// if let Some(hg) = Hg::new(Path::new(".")) {
///     println!("{}:{}", hg.name(), hg.branch().unwrap());
/// }
/// ```
impl VCS for Hg {

    /// Returns Mercurial's short name.
    fn name(&self) -> &str {
        "hg"
    }

    /// Returns Mercurial's active branch.
    ///
    /// # Errors
    ///
    /// The method might return an `Error` due to I/O errors.
    fn branch(&self) -> Result<String, Box<error::Error>> {
        let mut branch = String::new();

        try!(try!(File::open(&self.root.join(".hg").join("branch")))
            .read_to_string(&mut branch));

        Ok(branch.trim().to_string())
    }

    /// Returns `true` if uncommitted changes are detected.
    ///
    /// # Errors
    ///
    /// If `hg` binary is not installed in the system.
    fn modified(&self) -> Result<bool, Box<error::Error>> {
        use std::process::{Command, Stdio};

        let status = try!(Command::new("hg")
            .args(&[
                "status",
                "--quiet",
                "--modified",
                "--added",
                "--removed",
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status());

        Ok(!status.success())
    }
}
