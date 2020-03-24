use anyhow::Result;
use askama::Template;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use async_std::fs;
use async_std::path::PathBuf;

#[derive(Debug, Template)]
#[template(path = "index.adoc")]
pub struct Index {
    now: DateTime<Utc>,
    uuid: Uuid,
}

impl Index {
    pub async fn init(root_path: &PathBuf) -> Result<()> {
        let index = Index {
            now: Utc::now(),
            uuid: Uuid::new_v4(),
        };

        let index_content = format!("{}\n\n", index.render()?);
        let path = root_path.join("index.adoc");

        fs::write(path, &index_content).await?;

        Ok(())
    }

    fn timestamp(&self) -> String {
        format!("{:?}", self.now)
    }
}
