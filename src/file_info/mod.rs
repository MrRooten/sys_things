use std::path::{self, PathBuf};

use libc::stat;
pub mod file;
pub mod elf;
#[derive(Default)]
pub struct File {
    r_path      : String,
    r_stat      : Option<stat>,
    stat_is_set : bool
}

