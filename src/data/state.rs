use std::{
    collections::{BTreeMap, VecDeque},
    path::{Path, PathBuf},
};

use byte_unit::{Byte, UnitType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::PATHS;

use super::parsers::ParserInfo;

#[derive(Debug, Clone, Copy, Default, PartialEq, Serialize, Deserialize, clap::ValueEnum)]
pub enum ParserInstallMethod {
    #[default]
    Compile,
    Download,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParserState {
    pub last_modified: DateTime<Utc>,
    pub revision: String,
    pub url: String,
    pub tag: String,
    pub locked: bool,
    pub install_method: ParserInstallMethod,
}

impl Default for ParserState {
    fn default() -> Self {
        Self {
            last_modified: Utc::now(),
            revision: String::new(),
            url: String::new(),
            tag: String::new(),
            locked: false,
            install_method: ParserInstallMethod::Compile,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RestorePoint {
    pub date: DateTime<Utc>,
    pub location: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    last_updated: DateTime<Utc>,
    pub restore_points: VecDeque<RestorePoint>,
    pub parsers: BTreeMap<String, ParserState>,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let state_file = PATHS.ts_parsers.join("state.json");

        if state_file.is_file() {
            let state_str = std::fs::read_to_string(&state_file)?;
            let state: State = serde_json::from_str(&state_str)?;
            return Ok(state);
        }

        Ok(Self {
            last_updated: Utc::now(),
            restore_points: VecDeque::new(),
            parsers: BTreeMap::new(),
        })
    }

    pub fn check_all_installed(&self, parsers: &Vec<String>) -> (Vec<String>, Vec<String>) {
        let mut not_installed = Vec::new();
        let mut is_installed = Vec::new();
        for parser in parsers {
            match self.parsers.contains_key(parser) {
                true => is_installed.push(parser.clone()),
                false => not_installed.push(parser.clone()),
            }
        }
        (is_installed, not_installed)
    }

    pub fn add_parser(
        &mut self,
        name: &str,
        tag: &str,
        install_method: ParserInstallMethod,
        parser_info: &ParserInfo,
    ) {
        let parser_state = ParserState {
            last_modified: Utc::now(),
            revision: parser_info.revision.clone(),
            tag: tag.to_owned(),
            url: parser_info.url.clone(),
            install_method,
            ..ParserState::default()
        };
        self.parsers.insert(name.to_owned(), parser_state);
    }

    pub fn is_tag_up_to_date(&self, name: &str, tag: &str) -> bool {
        let parser = self.parsers.get(name).unwrap();
        parser.tag == tag
    }

    pub fn update_parser(
        &mut self,
        name: &str,
        tag: &str,
        install_method: ParserInstallMethod,
        parser_info: &ParserInfo,
    ) {
        let parser = self.parsers.get_mut(name).unwrap();

        parser.last_modified = Utc::now();
        parser.revision = parser_info.revision.clone();
        parser.tag = tag.to_owned();
        parser.url = parser_info.url.clone();
        parser.install_method = install_method;
    }

    pub fn remove_parser(&mut self, name: &str) {
        self.parsers.remove(name);
    }

    pub fn is_locked(&self, name: &str) -> bool {
        self.parsers.get(name).unwrap().locked
    }

    pub fn lock_parser(&mut self, name: &str) {
        self.parsers.get_mut(name).unwrap().locked = true;
    }

    pub fn unlock_parser(&mut self, name: &str) {
        self.parsers.get_mut(name).unwrap().locked = false;
    }

    pub fn append_restore_point(&mut self, restore_point: RestorePoint) {
        self.restore_points.push_front(restore_point);
    }

    pub fn create_backup(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let state_str = serde_json::to_string_pretty(&self.parsers)?;
        std::fs::write(path, state_str)?;
        Ok(())
    }

    pub fn restore_backup(&mut self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let state_str = std::fs::read_to_string(path)?;
        self.parsers = serde_json::from_str(&state_str)?;
        Ok(())
    }

    pub fn get_restore_point(&self, id: usize) -> anyhow::Result<&RestorePoint> {
        self.restore_points
            .get(id)
            .ok_or_else(|| anyhow::anyhow!("Invalid restore point '{id}'"))
    }

    pub fn check_restore_exists(&self, id: usize) -> anyhow::Result<bool> {
        if self.restore_points.is_empty() {
            anyhow::bail!("No restore points found");
        }

        match self.restore_points.get(id) {
            Some(_) => Ok(true),
            None => Ok(false),
        }
    }

    pub fn delete_restore_point(&mut self, id: usize) {
        self.restore_points.remove(id);
    }

    pub fn list_restore_points(&self) -> anyhow::Result<Vec<(String, String, String)>> {
        let mut restore_points = vec![];
        for restore_point in self.restore_points.iter() {
            let date = restore_point.date.format("%F %H:%M:%S").to_string();
            let file_name = restore_point
                .location
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string();
            let file_size = Byte::from_u64(std::fs::metadata(&restore_point.location)?.len())
                .get_appropriate_unit(UnitType::Binary);
            restore_points.push((date, file_name, format!("{file_size:.3}")));
        }
        Ok(restore_points)
    }

    pub fn commit(&mut self) -> anyhow::Result<()> {
        self.last_updated = Utc::now();
        let state_file = PATHS.ts_parsers.join("state.json");
        std::fs::write(state_file, serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_parser_info() -> ParserInfo {
        ParserInfo {
            url: "https://test.com".to_owned(),
            files: vec!["src/test.c".to_owned()],
            location: None,
            revision: "test".to_owned(),
            generate_from_grammar: false,
        }
    }

    #[test]
    fn test_parser_add() {
        let mut state = State::new().unwrap();
        let parser_info = dummy_parser_info();
        state.add_parser("Test", "test", ParserInstallMethod::Compile, &parser_info);

        assert!(state.parsers.contains_key("Test"));
    }

    #[test]
    fn test_parser_update() {
        let mut state = State::new().unwrap();
        let parser_info = dummy_parser_info();
        state.add_parser("Test", "test", ParserInstallMethod::Compile, &parser_info);

        let new_parser_info = ParserInfo {
            url: "https://new.com".to_owned(),
            files: vec!["src/new.c".to_owned()],
            location: None,
            revision: "new".to_owned(),
            generate_from_grammar: false,
        };

        state.update_parser(
            "Test",
            "new",
            ParserInstallMethod::Download,
            &new_parser_info,
        );

        let current_lang_state = state.parsers.get("Test").unwrap().clone();

        assert_eq!(current_lang_state.revision, "new");
        assert_eq!(current_lang_state.tag, "new");
        assert_eq!(current_lang_state.url, "https://new.com");
        assert_eq!(
            current_lang_state.install_method,
            ParserInstallMethod::Download
        );
    }
}
