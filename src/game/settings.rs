use std::time::Duration;

use config::{Config, File};
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

use once_cell::sync::OnceCell;

use super::init::InitError;

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    move_delay_ms: Option<u64>,
}

// const DEFAULT_SETTINGS: Settings = Settings {
//     move_delay_ms: None,
// };

#[cfg(test)]
pub const DEFAULT_TEST_SETTINGS: Settings = Settings {
    move_delay_ms: Some(0),
};

impl Settings {
    pub fn move_delay(&self) -> Option<Duration> {
        self.move_delay_ms.map(Duration::from_millis)
    }
}

pub(super) static SETTINGS: OnceCell<Settings> = OnceCell::new();

pub fn setting() -> &'static Settings {
    SETTINGS.get().unwrap()
}

pub(super) fn init() -> Result<(), InitError> {
    let mut builder = Config::builder();

    if let Some(project_dir) = ProjectDirs::from("", "", "tilt-game") {
        let mut config_file = project_dir.config_dir().to_path_buf();
        config_file.push("settings.toml");
        if let Some(config_file) = config_file.to_str() {
            builder = builder.add_source(File::with_name(config_file));
        }
    }

    let settings = builder
        .add_source(config::Environment::with_prefix("TiltGame"))
        .build()?;

    SETTINGS
        .set(settings.try_deserialize::<Settings>()?)
        .map_err(|_| InitError::AlreadyInit)
}

#[cfg(test)]
pub fn init_test(setting: Option<Settings>) {
    SETTINGS
        .set(setting.unwrap_or(DEFAULT_TEST_SETTINGS))
        .unwrap();
}
