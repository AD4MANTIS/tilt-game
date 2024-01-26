use std::time::Duration;

use config::{Config, File};
use directories::ProjectDirs;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {
    move_delay_ms: Option<u64>,
}

impl Default for Settings {
    #[cfg(not(test))]
    fn default() -> Self {
        Self {
            move_delay_ms: None,
        }
    }

    #[cfg(test)]
    fn default() -> Self {
        Self {
            move_delay_ms: Some(0),
        }
    }
}

impl Settings {
    pub fn move_delay(&self) -> Option<Duration> {
        self.move_delay_ms.map(Duration::from_millis)
    }
}

pub(super) static PROJECT_DIR: Lazy<Option<ProjectDirs>> =
    Lazy::new(|| ProjectDirs::from("", "", "tilt-game"));

pub(super) static SETTINGS: Lazy<Settings> = Lazy::new(|| {
    let mut builder = Config::builder();

    if let Some(ref project_dir) = *PROJECT_DIR {
        let mut config_file = project_dir.config_dir().to_path_buf();
        config_file.push("settings.toml");
        if let Some(config_file) = config_file.to_str() {
            builder = builder.add_source(File::with_name(config_file));
        }
    }

    builder
        .add_source(config::Environment::with_prefix("TiltGame"))
        .build()
        .and_then(config::Config::try_deserialize)
        .unwrap_or_default()
});

pub fn setting() -> &'static Settings {
    &SETTINGS
}
