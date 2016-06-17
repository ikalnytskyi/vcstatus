//! # vcstatus
//!
//! vcstatus is a command line tool that prints a short string with VCS
//! information about the current working directory. A main use case is
//! to make shell prompts consume that information, so you never forget
//! current VCS and active branch.
//!
//! vcstatus is designed keeping in mind the main use case, so it was
//! crucial to have a fast tool. That means it can't be written in
//! scripting language (like Python), even if I'd prefer to. So I choose
//! Rust just because I wanted to learn it for a while.
//!
//! ```bash
//! $ vcstatus -f "[%n %b]"
//! [git master]
//! ```
//!
//! ## VCS
//!
//! * Git
//! * Mercurial
//!
//! ## Formats
//!
//! * `%n` - prints VCS short name
//! * `%b` - prints VCS active branch

extern crate docopt;

#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::io::Write;

use vcs::get_vcs;
mod vcs;


fn main() {
    let version =
        format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let args = docopt::Docopt::new(include_str!("usage.txt"))
        .and_then(|dopt| dopt.version(Some(version)).parse())
        .unwrap_or_else(|e| e.exit());

    let cwd = std::env::current_dir().unwrap_or_else(|err| {
        if !args.get_bool("--quiet") {
            writeln!(&mut std::io::stderr(), "{}", err).ok();
        }
        std::process::exit(1);
    });

    if let Some(vcs) = get_vcs(&cwd) {
        // Print VCS information to standard output. It's not an optimal
        // solution because:
        //
        // (a) replace triggers a string reallocation and we call too
        //     often (for all supported format tokens)
        // (b) trait methods are called even if there are no format
        //     tokens in user's format input
        //
        // Even so I believe programs must be written for programmers
        // first, computers - second. I doubt we encounter any performance
        // issues here on modern computers, so let it go. :)
        print!("{}", args.get_str("--format").to_string()
            .replace("%n", vcs.name())
            .replace("%b",
                match vcs.branch() {
                    Ok(ref branch) => branch,
                    Err(ref err) => {
                        if !args.get_bool("--quiet") {
                            writeln!(&mut std::io::stderr(), "{}",
                                     err.description()).ok();
                        }

                        // vcstatus has been designed to be used in shell
                        // prompts, so it makes sense to return at least
                        // something rather than panic.
                        "(unknown)"
                    }
                })
        );
    }
}
