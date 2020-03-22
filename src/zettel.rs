use anyhow::{anyhow, Result};
use askama::Template;
use chrono::{DateTime, Utc};
use slug::slugify;
use uuid::Uuid;

use async_std::fs;
use async_std::path::{Path, PathBuf};

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

    pub fn path_buf(&self) -> PathBuf {
        let raw_path = format!("{}.adoc", self.slug);
        Path::new(&raw_path).to_path_buf()
    }

    fn tags(&self) -> String {
        self.tags.join(", ")
    }

    fn timestamp(&self) -> String {
        format!("{:?}", self.now)
    }

    pub async fn render_to_file(&self) -> Result<String> {
        let adoc = self.render()?;
        let path = self.path_buf();

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
