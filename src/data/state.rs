use std::{
    collections::{BTreeMap, VecDeque},
    path::PathBuf,
};

use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

use crate::utils::Paths;

lazy_static! {
    static ref PATHS: Paths = Paths::new();
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ParserInstallMethod {
    Downloaded,
    Copmiled,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParserState {
    last_modified: DateTime<Utc>,
    revision: String,
    url: String,
    tag: String,
    locked: bool,
    still_supported: bool,
    install_method: ParserInstallMethod,
}

impl Default for ParserState {
    fn default() -> Self {
        Self {
            last_modified: Utc::now(),
            revision: String::new(),
            url: String::new(),
            tag: String::new(),
            locked: false,
            still_supported: true,
            install_method: ParserInstallMethod::Downloaded,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RestorePoint {
    date: DateTime<Utc>,
    location: PathBuf,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    last_updated: DateTime<Utc>,
    current_tag: String,
    #[serde(skip_serializing, skip_deserializing)]
    wanted: Option<Vec<String>>,
    restore_points: VecDeque<RestorePoint>,
    parsers: BTreeMap<String, ParserState>,
}

// custom debug implementation to hide warning on `wanted` field
impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.debug_struct("State")
            .field("last_updated", &self.last_updated)
            .field("current_tag", &self.current_tag)
            .field("wanted", &self.wanted)
            .field("restore_points", &self.restore_points)
            .field("parsers", &self.parsers)
            .finish()
    }
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let state_file = PATHS.nvim_data.join("state.json");
        let wanted_file = PATHS.nvim_config.join("wanted-parsers.txt");

        let wanted = match wanted_file.is_file() {
            true => Some(
                std::fs::read_to_string(&wanted_file)?
                    .lines()
                    .map(str::to_string)
                    .collect(),
            ),
            false => None,
        };

        if state_file.is_file() {
            let mut state: State = serde_json::from_str(&std::fs::read_to_string(&state_file)?)?;
            state.wanted = wanted;
            return Ok(state);
        }

        Ok(Self {
            last_updated: Utc::now(),
            current_tag: String::new(),
            wanted,
            restore_points: VecDeque::new(),
            parsers: BTreeMap::new(),
        })
    }

    pub fn list_wanted(&self) -> Option<&Vec<String>> {
        self.wanted.as_ref()
    }

    pub fn check_parser(&self, name: &str) -> bool {
        self.parsers.contains_key(name)
    }

    pub fn get_parser(&self, name: &str) -> Option<&ParserState> {
        self.parsers.get(name)
    }

    pub fn add_parser(&mut self, name: String, parser: ParserState) -> anyhow::Result<()> {
        self.parsers.insert(name, parser);
        Ok(())
    }

    pub fn remove_parser(&mut self, name: &str) -> anyhow::Result<()> {
        self.parsers.remove(name);
        Ok(())
    }

    pub fn update_parser(&mut self, name: &str, mut parser: ParserState) -> anyhow::Result<()> {
        parser.last_modified = Utc::now();
        *self.parsers.get_mut(name).unwrap() = parser;
        Ok(())
    }

    pub fn toggle_lock(&mut self, name: &str) -> anyhow::Result<()> {
        let parser = self.parsers.get_mut(name).unwrap();
        self.parsers.get_mut(name).unwrap().locked = !parser.locked;
        Ok(())
    }

    pub fn toggle_support(&mut self, name: &str) -> anyhow::Result<()> {
        let parser = self.parsers.get_mut(name).unwrap();
        self.parsers.get_mut(name).unwrap().still_supported = !parser.still_supported;
        Ok(())
    }

    pub fn append_restore_point(&mut self, restore_point: RestorePoint) -> anyhow::Result<()> {
        self.restore_points.push_back(restore_point);
        Ok(())
    }

    pub fn commit(&self) -> anyhow::Result<()> {
        let state_file = PATHS.nvim_data.join("state.json");
        std::fs::write(state_file, serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }

    pub fn create_backup(&self) -> anyhow::Result<()> {
        let backup_file = PATHS.nvim_data.join("state.json.bak");
        std::fs::write(backup_file, serde_json::to_string_pretty(&self)?)?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_add() {
        let new_lang = "new_lang".to_string();
        let new_lang_state = ParserState {
            revision: "revision".to_string(),
            tag: "tag".to_string(),
            url: "https://url.com".to_string(),
            install_method: ParserInstallMethod::Copmiled,
            ..ParserState::default()
        };

        let mut state = State::new().unwrap();
        state.add_parser(new_lang.clone(), new_lang_state).unwrap();
        assert!(state.check_parser(&new_lang));
    }

    #[test]
    fn test_parser_remove() {
        let new_lang = "new_lang".to_string();
        let new_lang_state = ParserState {
            revision: "revision".to_string(),
            tag: "tag".to_string(),
            url: "https://url.com".to_string(),
            install_method: ParserInstallMethod::Copmiled,
            ..ParserState::default()
        };

        let mut state = State::new().unwrap();
        state.add_parser(new_lang.clone(), new_lang_state).unwrap();
        state.remove_parser(&new_lang).unwrap();
        assert!(!state.check_parser(&new_lang));
    }

    #[test]
    fn test_parser_update() {
        let new_lang = "new_lang".to_string();
        let new_lang_state = ParserState {
            revision: "revision".to_string(),
            tag: "tag".to_string(),
            url: "https://url.com".to_string(),
            ..ParserState::default()
        };

        let mut state = State::new().unwrap();
        state.add_parser(new_lang.clone(), new_lang_state).unwrap();

        let current_lang_state = state.get_parser(&new_lang).unwrap().clone();
        let new_lang_state = ParserState {
            revision: "new_revision".to_string(),
            tag: "new_tag".to_string(),
            url: "https://new_url.com".to_string(),
            install_method: ParserInstallMethod::Copmiled,
            ..current_lang_state
        };
        state.update_parser(&new_lang, new_lang_state).unwrap();
        state.toggle_lock(&new_lang).unwrap();

        let current_lang_state = state.get_parser(&new_lang).unwrap().clone();

        assert_eq!(
            state.get_parser(&new_lang).unwrap().clone(),
            ParserState {
                revision: "new_revision".to_string(),
                tag: "new_tag".to_string(),
                url: "https://new_url.com".to_string(),
                install_method: ParserInstallMethod::Copmiled,
                locked: true,
                still_supported: true,
                last_modified: current_lang_state.last_modified,
            }
        );
    }
}
