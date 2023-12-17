use config::ConfigError;

use super::settings;

#[derive(thiserror::Error, Debug)]
pub enum InitError {
    #[error(transparent)]
    ConfigError(#[from] ConfigError),

    #[error("The global Data was already set")]
    AlreadyInit,
}

pub(super) fn init() -> Result<(), InitError> {
    settings::init()
}

#[cfg(test)]
pub fn init_test() {
    settings::init_test(None);
}
