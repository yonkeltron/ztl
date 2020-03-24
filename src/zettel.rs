use anyhow::{anyhow, Result};
use askama::Template;
use chrono::{DateTime, Utc};
use slug::slugify;
use uuid::Uuid;

use async_std::fs;
use async_std::fs::OpenOptions;
use async_std::path::{Path, PathBuf};
use async_std::prelude::*;

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
            tags,
            uuid: Uuid::new_v4(),
        }
    }

    pub fn path_buf(&self, config: &Config) -> PathBuf {
        let fname = self.filename();
        Path::new(&config.zettelkasten_root).join(&fname)
    }

    pub fn filename(&self) -> String {
        format!("{}.adoc", self.slug)
    }

    fn tags(&self) -> String {
        self.tags.join(", ")
    }

    fn timestamp(&self) -> String {
        format!("{:?}", self.now)
    }

    fn to_index_line(&self) -> String {
        format!(". <<{}, {}>>\n", self.filename(), self.name)
    }

    pub async fn render_to_file(&self, config: &Config) -> Result<String> {
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

    pub async fn write_to_index(&self, config: &Config) -> Result<()> {
        let index_path = Path::new(&config.zettelkasten_root).join("index.adoc");
        let mut file = OpenOptions::new().append(true).open(index_path).await?;
        file.write_all(self.to_index_line().as_bytes()).await?;

        Ok(())
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

    #[test]
    fn test_filename() {
        let zettel = Zettel::new("panda", vec![]);

        assert_eq!(&zettel.filename(), "panda.adoc");
    }

    #[test]
    fn test_to_index_line() {
        let zettel = Zettel::new("Panda Bamboo", vec![]);
        let expected = ". <<panda-bamboo.adoc, Panda Bamboo>>\n";

        assert_eq!(&zettel.to_index_line(), expected);
    }
}
