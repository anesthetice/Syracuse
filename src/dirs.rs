use directories::ProjectDirs;
use std::sync::OnceLock;

pub static DIRS: OnceLock<ProjectDirs> = OnceLock::new();

pub struct Dirs;

impl Dirs {
    pub fn get() -> &'static ProjectDirs {
        DIRS.get().unwrap()
    }
}
