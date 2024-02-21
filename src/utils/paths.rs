use std::path::PathBuf;

use lazy_static::lazy_static;
use tempfile::{TempDir, tempdir_in};

#[derive(Debug, Clone)]
pub struct Paths {
    pub nvim_config: PathBuf,
    pub ts_parsers: PathBuf,
}

#[allow(unused)]
#[cfg(windows)]
const NVIM_DATA_DIR: &str = "nvim-data";

#[allow(unused)]
#[cfg(unix)]
const NVIM_DATA_DIR: &str = "nvim";

#[cfg(not(test))]
impl Paths {
    pub fn new() -> Self {
        use cfg_if::cfg_if;

        cfg_if! {
            if #[cfg(unix)] {
                let local_config_dir = dirs::home_dir().unwrap().join(".config");
                let local_data_dir = dirs::home_dir() .unwrap().join(".local").join("share");
            } else {
                let local_config_dir = dirs::config_local_dir().unwrap();
                let local_data_dir = dirs::data_local_dir().unwrap();
            }
        }

        // variable is set with `assert_cmd` for subcommand testing
        match std::env::var("TS_PARSERS_TEST") {
            Ok(_) => Self {
                nvim_config: std::env::current_dir().unwrap().join("test-assets"),
                ts_parsers: std::env::current_dir().unwrap().join("test-assets"),
            },
            Err(_) => Self {
                nvim_config: local_config_dir.join("nvim"),
                ts_parsers: local_data_dir.join(NVIM_DATA_DIR).join("ts-parsers"),
            },
        }
    }
}

#[cfg(test)]
impl Paths {
    pub fn new() -> Self {
        Self {
            nvim_config: std::env::current_dir().unwrap().join("test-assets"),
            ts_parsers: std::env::current_dir().unwrap().join("test-assets"),
        }
    }
}

lazy_static! {
    pub static ref PATHS: Paths = Paths::new();
}
