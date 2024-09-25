mod paths;
mod renderer;

pub mod archives;
pub mod colors;
pub mod command;
pub mod fs;
pub mod http;
pub mod num_args;

pub use self::paths::PATHS;

/// A macro to print colored text to the terminal.
#[macro_export]
macro_rules! c_println {
    (red, $($arg:tt)*) =>   {{ eprintln!("{0}{1}{0:#}",$crate::utils::colors::RED, format_args!($($arg)*)) }};
    (blue, $($arg:tt)*) =>  {{ println!("{0}{1}{0:#}",$crate::utils::colors::BLUE, format_args!($($arg)*)) }};
    (green, $($arg:tt)*) => {{ println!("{0}{1}{0:#}",$crate::utils::colors::GREEN, format_args!($($arg)*)) }};
    (amber, $($arg:tt)*) => {{ println!("{0}{1}{0:#}",$crate::utils::colors::AMBER, format_args!($($arg)*)) }};
}
