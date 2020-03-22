use anyhow::{anyhow, Result};
use askama::Template;
use chrono::{DateTime, Utc};
use slug::slugify;
use uuid::Uuid;

use async_std::fs;
use async_std::path::{Path, PathBuf};

use crate::config::Config;

#[derive(Debug, Template)]
#[template(path = "zettel.adoc")]
pub struct Zettel {
    name: String,
    now: DateTime<Utc>,
    slug: String,
    tags: Vec<String>,
    uuid: Uuid,
}

impl Zettel {
    pub fn new(name: &str, tags: Vec<String>) -> Self {
        Self {
            name: String::from(name),
            now: Utc::now(),
            slug: slugify(name),
            tags: tags,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn path_buf(&self, config: Config) -> PathBuf {
        let fname = format!("{}.adoc", self.slug);
        Path::new(&config.zettelkasten_root).join(&fname)
    }

    fn tags(&self) -> String {
        self.tags.join(", ")
    }

    fn timestamp(&self) -> String {
        format!("{:?}", self.now)
    }

    pub async fn render_to_file(&self, config: Config) -> Result<String> {
        let adoc = self.render()?;
        let path = self.path_buf(config);

        if path.exists().await {
            Err(anyhow!(
                "Refusing to create {} because it already exists",
                path.display()
            ))
        } else {
            fs::write(path, &adoc).await?;

            Ok(adoc)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tags() {
        let tags = vec![String::from("panda"), String::from("bamboo")];
        let zettel = Zettel::new("panda", tags);

        assert_eq!("panda, bamboo", &zettel.tags());
    }
}
