use std::path::PathBuf;

use lazy_static::lazy_static;

#[derive(Debug, Clone)]
pub struct Paths {
    pub wanted_parsers: PathBuf,
    pub ts_parsers: PathBuf,
}

#[allow(unused)]
#[cfg(windows)]
const NVIM_DATA_DIR: &str = "nvim-data";

#[allow(unused)]
#[cfg(unix)]
const NVIM_DATA_DIR: &str = "nvim";

impl Paths {
    #[cfg(not(test))]
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

        let wanted_parsers = match std::env::var("TS_PARSERS_WANTED_PARSERS") {
            Ok(dir) => PathBuf::from(&dir),
            Err(_) => local_config_dir.join("nvim").join("wanted-parsers.txt"),
        };

        let ts_parsers = match std::env::var("TS_PARSERS_DATA") {
            Ok(dir) => PathBuf::from(&dir),
            Err(_) => local_data_dir.join(NVIM_DATA_DIR).join("ts-parsers"),
        };

        Self {
            wanted_parsers,
            ts_parsers,
        }
    }

    #[cfg(test)]
    pub fn new() -> Self {
        Self {
            wanted_parsers: PathBuf::from("tests").join("fixtures").join("wanted-parsers.txt"),
            ts_parsers: PathBuf::from("tests").join("outputs").join("unit-tests"),
        }
    }
}

lazy_static! {
    pub static ref PATHS: Paths = Paths::new();
}
