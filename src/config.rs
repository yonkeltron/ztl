use anyhow::{anyhow, Result};
use confy;
use serde::{Deserialize, Serialize};

use async_std::path::PathBuf;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub zettelkasten_root: String,
}

impl ::std::default::Default for Config {
    fn default() -> Self {
        Self {
            zettelkasten_root: String::from(""),
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        match confy::load("ztl") {
            Ok(config) => Ok(config),
            Err(e) => Err(anyhow!("Unable to open config file because {}", e)),
        }
    }

    pub fn init(root_path_buf: &PathBuf) -> Result<Self> {
        let root_path_string = format!("{}", root_path_buf.display());
        let config = Config {
            zettelkasten_root: root_path_string,
        };

        config.store()?;

        Ok(config)
    }

    pub fn store(&self) -> Result<()> {
        match confy::store("ztl", self) {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow!("Unable to save config file because {}", e)),
        }
    }
}
