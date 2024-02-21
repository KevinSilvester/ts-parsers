use std::path::PathBuf;

use lazy_static::lazy_static;

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

        // variable is set with `assert_cmd` for subcommand integration tests
        match std::env::var("TS_PARSERS_TEST_DIR") {
            Ok(dir) => Self {
                nvim_config: PathBuf::from(&dir),
                ts_parsers: PathBuf::from(&dir),
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
            nvim_config: PathBuf::from("tests").join("outputs").join("unit-tests"),
            ts_parsers: PathBuf::from("tests").join("outputs").join("unit-tests"),
        }
    }
}

lazy_static! {
    pub static ref PATHS: Paths = Paths::new();
}
