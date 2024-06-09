mod paths;
mod renderer;

pub mod archives;
pub mod command;
pub mod fs;
pub mod http;

use anstyle::{RgbColor, Style};

pub use self::paths::PATHS;

pub const RED: Style = RgbColor(235, 66, 66).on_default();
const BLUE: Style = RgbColor(2, 149, 235).on_default();
const GREEN: Style = RgbColor(57, 219, 57).on_default();
const AMBER: Style = RgbColor(245, 181, 61).on_default();

/// A macro to print colored text to the terminal.
#[macro_export]
macro_rules! c_println {
    // (red, $($arg:tt)*) => {{
    //     let red = $crate::utils::RED;
    //     eprintln!("{red}{}{red:#}", $($arg)*)
    // }};
    (red, $($arg:tt)*) => {{
        eprintln!("{}", ::ansi_term::Colour::RGB(235, 66, 66).paint(&format!($($arg)*)))
    }};
    (blue, $($arg:tt)*) => {{
        println!("{}", ::ansi_term::Colour::RGB(2, 149, 235).paint(&format!($($arg)*)))
    }};
    (green, $($arg:tt)*) => {{
        println!("{}", ::ansi_term::Colour::RGB(57, 219, 57).paint(&format!($($arg)*)))
    }};
    (amber, $($arg:tt)*) => {{
        println!("{}", ::ansi_term::Colour::RGB(245, 181, 61).paint(&format!($($arg)*)))
    }};
}
