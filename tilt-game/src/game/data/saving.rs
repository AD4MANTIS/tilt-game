use std::fs;

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

fn get_save_path() -> Option<std::path::PathBuf> {
    let Some(project_dir) = PROJECT_DIR.as_ref() else {
        return None;
    };

    let mut save_file_path = project_dir.data_dir().to_path_buf();
    save_file_path.push("save.toml");
    Some(save_file_path)
}

pub fn get_save() -> Option<Save> {
    ron::from_str(&fs::read_to_string(get_save_path()?).ok()?).ok()
}

pub fn save(save: &Save) -> Option<()> {
    fs::write(get_save_path()?, ron::to_string(save).ok()?).ok()?;
    Some(())
}
