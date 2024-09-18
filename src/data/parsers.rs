use ahash::AHashMap;
use serde::{Deserialize, Serialize};

use crate::utils::PATHS;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct ParserInfo {
    pub url: String,
    pub files: Vec<String>,
    pub location: Option<String>,
    pub revision: String,
    pub generate_from_grammar: bool,
}

#[derive(Debug)]
pub struct Parsers {
    pub langs: Vec<String>,
    pub list: AHashMap<String, ParserInfo>,
    pub wanted: Option<Vec<String>>,
}

impl Parsers {
    pub fn new() -> anyhow::Result<Self> {
        let wanted_file = &PATHS.wanted_parsers;

        let wanted = match wanted_file.is_file() {
            true => Some(
                std::fs::read_to_string(wanted_file)?
                    .lines()
                    .map(str::trim)
                    .filter(|line| !line.is_empty())
                    .map(str::to_string)
                    .collect(),
            ),
            false => None,
        };

        Ok(Self {
            langs: Vec::new(),
            list: AHashMap::new(),
            wanted,
        })
    }

    pub async fn fetch_list(&mut self, tag: &Option<String>) -> anyhow::Result<()> {
        let url = match tag {
            Some(tag) => format!( "https://raw.githubusercontent.com/KevinSilvester/nvim-treesitter-parsers/{tag}/parsers.min.json"),
            None => "https://raw.githubusercontent.com/KevinSilvester/nvim-treesitter-parsers/master/parsers.min.json".to_string(),
        };

        let res = reqwest::get(&url).await?;
        self.list = res.json().await?;
        self.langs = self.list.keys().cloned().collect();
        self.langs.sort();
        Ok(())
    }

    pub fn get_parser(&self, parser: &str) -> Option<&ParserInfo> {
        self.list.get(parser)
    }

    pub fn validate_parsers(&self, parsers: &[String]) -> anyhow::Result<()> {
        let invalid_parsers: Vec<_> = parsers
            .iter()
            .filter(|p| !self.list.contains_key(*p))
            .collect();

        if !invalid_parsers.is_empty() {
            anyhow::bail!("Invalid parsers: {:?}", invalid_parsers);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    const TAG: &str = "v2024-09-18-3c6af36";

    #[tokio::test]
    async fn test_fetch_list() {
        let mut parsers = Parsers::new().unwrap();
        parsers.fetch_list(&Some(TAG.to_string())).await.unwrap();

        let json = std::fs::read_to_string(
            Path::new("tests")
                .join("fixtures")
                .join(format!("parsers-{TAG}.json")),
        )
        .unwrap();
        let test_parsers: AHashMap<String, ParserInfo> = serde_json::from_str(&json).unwrap();

        //  check if the size of the list is the same as the test list
        assert_eq!(parsers.list.len(), test_parsers.len());

        //  check if the list keys are the same as the test list keys
        let mut keys: Vec<_> = parsers.list.keys().collect();
        let mut test_keys: Vec<_> = test_parsers.keys().collect();

        keys.sort();
        test_keys.sort();
        assert_eq!(keys, test_keys);

        //  check if the list values are the same as the test list values
        for (key, value) in parsers.list.iter() {
            assert_eq!(value, test_parsers.get(key).unwrap());
        }
    }

    #[tokio::test]
    async fn test_fetch_list_default() {
        let mut parsers = Parsers::new().unwrap();
        parsers.fetch_list(&None).await.unwrap();
        assert!(!parsers.list.is_empty());
    }
}
