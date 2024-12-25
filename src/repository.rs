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
    use crate::test_utils::test_repository;
    use tokio::runtime::Runtime;

    #[test]
    fn test_from_config() {
        let repo = test_repository();
        assert_eq!(repo.path(), &PathBuf::from("."));
    }

    #[test]
    fn test_path() {
        let repo = test_repository();
        assert_eq!(repo.path(), &PathBuf::from("."));
    }

    #[test]
    fn test_config() {
        let repo = test_repository();
        // Assuming Config implements PartialEq;
        assert_eq!(repo.config(), &repo.config);
    }

    #[test]
    fn test_into_repository() {
        let repo = test_repository();
        let _: Repository = (&repo).into(); // Check trait implementation
    }

    #[test]
    fn test_clear_cache() {
        let repo = test_repository();
        let rt = Runtime::new().unwrap();

        // Run `clear_cache` asynchronously within the test runtime
        rt.block_on(async {
            assert!(repo.clear_cache().await.is_ok());
        });
    }
}
