use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::c_println;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Changes {
    pub added: Vec<String>,
    pub updated: Vec<String>,
    pub removed: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChangeLogEntry {
    pub tag: String,
    pub url: String,
    pub date: DateTime<Utc>,
    pub changes: Changes,
}

#[derive(Debug)]
pub struct ChangeLog {
    pub entries: Vec<ChangeLogEntry>,
}

impl ChangeLog {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub async fn fetch_changelog(&mut self) -> anyhow::Result<()> {
        let url =
            "https://raw.githubusercontent.com/KevinSilvester/nvim-treesitter-parsers/master/CHANGELOG.json";
        let res = reqwest::get(url).await?;
        self.entries = res.json().await?;
        Ok(())
    }

    pub fn get_latest(&self) -> Option<&ChangeLogEntry> {
        self.entries.first()
    }

    pub fn get_latest_tag(&self) -> Option<String> {
        self.get_latest().map(|entry| entry.tag.clone())
    }

    pub fn check_entry(&self, tag: &Option<String>) -> anyhow::Result<()> {
        let tag = match tag {
            Some(tag) => tag,
            None => return Ok(()),
        };

        match self.entries.iter().any(|entry| &entry.tag == tag) {
            true => Ok(()),
            false => Err(anyhow::anyhow!("Tag {} not found in changelog", tag)),
        }
    }

    pub fn get_entry(&self, tag: &str) -> Option<&ChangeLogEntry> {
        self.entries.iter().find(|entry| entry.tag == tag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_fetch_changelog() {
        let mut changelog = ChangeLog::new();
        changelog.fetch_changelog().await.unwrap();
        dbg!(&changelog.entries);
        assert!(!changelog.entries.is_empty());
    }
}
