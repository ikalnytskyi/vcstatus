use std::error;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::{Path, PathBuf};

use regex::Regex;

use super::VCS;


pub struct Git {
    /// a path to mercurial repo root
    pub root: PathBuf,
}


impl Git {
    /// Creates a new `Git` instance based on a given `path`. If `path` is
    /// not a Git root, `None` is returned.
    ///
    /// # Example
    ///
    /// ```rust
    /// match Git::new(Path::new(".")) {
    ///     Some(git) => println!("Yay! That's a git root!"),
    ///     None => println!("Nah.. That's not a git root."),
    /// }
    /// ```
    pub fn new(path: &Path) -> Option<Box<VCS>> {
        if path.join(".git").exists() {
            Some(Box::new(Git {root: path.to_path_buf()}))
        } else {
            None
        }
    }
}


/// VCS implementation for Git.
///
/// # Example
///
/// ```rust
/// if let Some(git) = Git::new(Path::new(".")) {
///     println!("{}:{}", git.name(), git.branch().unwrap());
/// }
/// ```
impl VCS for Git {

    /// Returns Git's short name.
    fn name(&self) -> &str {
        "git"
    }

    /// Returns Git's active branch.
    ///
    /// # Errors
    ///
    /// The method might return an `Error` mostly due to I/O errors.
    ///
    /// # Panics
    ///
    /// The method might panic due to unsafe unwrapping of regexps.
    /// Nevertheless, it's an extremely rare occasion since we deal with
    /// predefined regexps and they are tested.
    fn branch(&self) -> Result<String, Box<error::Error>> {
        lazy_static! {
            static ref RE_BRANCH: Regex = Regex::new(r"ref:\s*refs/heads/(.*)").unwrap();
            static ref RE_GITDIR: Regex = Regex::new(r"gitdir:\s*(.*)").unwrap();
        }

        let mut gitdir = self.root.join(".git");

        // In case of Git submodule, `.git` is a plain text file that
        // contains a path to effective Git directory. So we need
        // to handle that, and amend `gitdir` var accordingly.
        if gitdir.is_file() {
            let mut content = String::new();
            try!(try!(File::open(&gitdir)).read_to_string(&mut content));

            let captures = RE_GITDIR.captures(&content).unwrap();
            gitdir = self.root.join(captures[1].to_string());
        }

        // Current active branch is stored in `.git/HEAD`, but we can't
        // rely on it. `.git/HEAD` may also contain the commit hash sum.
        let mut head = String::new();
        try!(try!(File::open(&gitdir.join("HEAD"))).read_to_string(&mut head));

        match RE_BRANCH.captures(&head) {
            Some(captures) => Ok(captures[1].to_string()),

            None => Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidData,
                format!(
                    "Can't parse .git/HEAD: '{}' doesn't match '{}'.",
                    head.trim(), RE_BRANCH.as_str()
                )))),
        }
    }
}
