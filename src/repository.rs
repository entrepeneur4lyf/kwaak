use anyhow::Result;
use std::{path::PathBuf, str::FromStr as _};

use tokio::fs;

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

    pub async fn clear_cache(&self) -> Result<()> {
        fs::remove_dir_all(self.config.cache_dir()).await?;
        Ok(())
    }
}

#[allow(clippy::from_over_into)]
impl Into<Repository> for &Repository {
    fn into(self) -> Repository {
        self.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;
    use tempfile::tempdir;
    use crate::config::Config;

    #[tokio::test]
    async fn test_from_config() {
        let config = Config::default();
        let repo = Repository::from_config(config.clone());
        assert_eq!(repo.path(), &PathBuf::from("."));
        assert_eq!(repo.config(), &config);
    }

    #[tokio::test]
    async fn test_clear_cache() {
        let temp_dir = tempdir().unwrap();
        let cache_dir = temp_dir.path().join("cache");
        fs::create_dir_all(&cache_dir).await.unwrap();

        let mut config = Config::default();
        config.set_cache_dir(cache_dir.clone());
        let repo = Repository::from_config(config);

        assert!(cache_dir.exists());
        repo.clear_cache().await.unwrap();
        assert!(!cache_dir.exists());
    }
}
