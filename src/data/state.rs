use std::{
    collections::{BTreeMap, VecDeque},
    path::{Path, PathBuf},
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::utils::PATHS;

use super::parsers::ParserInfo;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize, clap::ValueEnum)]
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

#[derive(Debug, Serialize, Deserialize)]
pub struct RestorePoint {
    pub date: DateTime<Utc>,
    pub location: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct State {
    last_updated: DateTime<Utc>,
    restore_points: VecDeque<RestorePoint>,
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

    pub fn all_installed(&self, parsers: &Vec<String>) -> (Vec<String>, Vec<String>) {
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

    pub fn get_parser(&self, name: &str) -> Option<&ParserState> {
        self.parsers.get(name)
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

    pub fn append_restore_point(&mut self, restore_point: RestorePoint) {
        self.restore_points.push_back(restore_point);
    }

    pub fn create_backup(&self, path: impl AsRef<Path>) -> anyhow::Result<()> {
        let state_str = serde_json::to_string_pretty(&self.parsers)?;
        std::fs::write(path, state_str)?;
        Ok(())
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
    // use super::*;

    // #[test]
    // fn test_parser_add() {
    //     let new_lang = "new_lang".to_string();
    //     let new_lang_state = ParserState {
    //         revision: "revision".to_string(),
    //         tag: "tag".to_string(),
    //         url: "https://url.com".to_string(),
    //         install_method: ParserInstallMethod::Copmiled,
    //         ..ParserState::default()
    //     };

    //     let mut state = State::new().unwrap();
    //     state.add_parser(new_lang.clone(), new_lang_state).unwrap();
    //     assert!(state.check_parser(&new_lang));
    // }

    // #[test]
    // fn test_parser_remove() {
    //     let new_lang = "new_lang".to_string();
    //     let new_lang_state = ParserState {
    //         revision: "revision".to_string(),
    //         tag: "tag".to_string(),
    //         url: "https://url.com".to_string(),
    //         install_method: ParserInstallMethod::Copmiled,
    //         ..ParserState::default()
    //     };

    //     let mut state = State::new().unwrap();
    //     state.add_parser(new_lang.clone(), new_lang_state).unwrap();
    //     state.remove_parser(&new_lang).unwrap();
    //     assert!(!state.check_parser(&new_lang));
    // }

    // #[test]
    // fn test_parser_update() {
    //     let new_lang = "new_lang".to_string();
    //     let new_lang_state = ParserState {
    //         revision: "revision".to_string(),
    //         tag: "tag".to_string(),
    //         url: "https://url.com".to_string(),
    //         ..ParserState::default()
    //     };

    //     let mut state = State::new().unwrap();
    //     state.add_parser(new_lang.clone(), new_lang_state).unwrap();

    //     let current_lang_state = state.get_parser(&new_lang).unwrap().clone();
    //     let new_lang_state = ParserState {
    //         revision: "new_revision".to_string(),
    //         tag: "new_tag".to_string(),
    //         url: "https://new_url.com".to_string(),
    //         install_method: ParserInstallMethod::Copmiled,
    //         ..current_lang_state
    //     };
    //     state.update_parser(&new_lang, new_lang_state).unwrap();
    //     state.toggle_lock(&new_lang).unwrap();

    //     let current_lang_state = state.get_parser(&new_lang).unwrap().clone();

    //     assert_eq!(
    //         state.get_parser(&new_lang).unwrap().clone(),
    //         ParserState {
    //             revision: "new_revision".to_string(),
    //             tag: "new_tag".to_string(),
    //             url: "https://new_url.com".to_string(),
    //             install_method: ParserInstallMethod::Copmiled,
    //             locked: true,
    //             still_supported: true,
    //             last_modified: current_lang_state.last_modified,
    //         }
    //     );
    // }
}
