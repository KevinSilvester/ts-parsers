mod backups;
pub mod command;
pub mod fs;
pub mod http;
mod paths;
pub mod renderer;

pub use self::backups::Backups;
pub use self::paths::PATHS;

#[macro_export]
macro_rules! c_println {
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
