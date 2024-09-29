use std::{path::PathBuf, str::FromStr as _};

use crate::config::Config;

#[derive(Debug, Clone)]
pub struct Repository {
    config: Config,
    path: PathBuf,
}

impl Repository {
    pub fn from_config(config: impl Into<Config>) -> Repository {
        Self {
            config: config.into(),
            path: PathBuf::from_str(".").expect("Failed to create path from current directory"),
        }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn config(&self) -> &Config {
        &self.config
    }
}
