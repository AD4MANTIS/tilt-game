use std::{
    fs,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use classes::Level;

use super::settings::PROJECT_DIR;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Save {
    pub next_level: Level,
}

impl Default for Save {
    fn default() -> Self {
        Self {
            next_level: Level::Lv1,
        }
    }
}

fn get_save_dir() -> Option<&'static Path> {
    PROJECT_DIR.as_ref().map(directories::ProjectDirs::data_dir)
}

fn get_save_path() -> Option<PathBuf> {
    let mut save_file_path = get_save_dir()?.to_path_buf();
    save_file_path.push("save.toml");
    Some(save_file_path)
}

pub fn get_save() -> Option<Save> {
    ron::from_str(&fs::read_to_string(get_save_path()?).ok()?).ok()
}

pub fn save(save: &Save) -> Option<()> {
    fs::create_dir_all(get_save_dir()?).ok()?;
    fs::write(get_save_path()?, ron::to_string(save).ok()?).unwrap();
    Some(())
}
