use std::{error::Error, fmt};

use anstyle::{Color, Effects, RgbColor};
use indoc::formatdoc;

use crate::utils::colors::{BLUE, TURQUOISE, WHITE};

trait FancyError {
    fn error_ctx(&self) -> (&Vec<char>, (usize, usize));
    fn error_msg(&self) -> String;
    fn error_hint(&self) -> String;

    fn construct_error(&self) -> String {
        let (input, span) = self.error_ctx();
        let msg = self.error_msg();
        let hint = self.error_hint();
        let white = RgbColor(255, 255, 255);
        let white_on_red = white.on(Color::from(RgbColor(235, 66, 66))) | Effects::BOLD;

        let before_err: String = input[0..(span.0 - 1)].iter().collect();
        let after_err: String = input[span.1..].iter().collect();
        let err: String = input[(span.0 - 1)..span.1].iter().collect();

        let error_msg = formatdoc! {"Syntax error{WHITE}
            ╭╴{msg}
            │ 
            │ {before_err}{white_on_red}{err}{white_on_red:#}{after_err}
            │
            ╰╴= {TURQUOISE}HINT{TURQUOISE:#}: {hint}{WHITE:#}
        "};
        error_msg
    }
}

#[derive(Debug)]
pub enum LexicalError {
    InvalidToken(Vec<char>, (usize, usize)),
    InvalidRange(Vec<char>, (usize, usize)),
    UnexpectedEqual(Vec<char>, (usize, usize)),
    NoRangeStart(Vec<char>, (usize, usize)),
    NoRangeEnd(Vec<char>, (usize, usize)),
}

impl Error for LexicalError {}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexicalError::InvalidToken(_, _)
            | LexicalError::InvalidRange(_, _)
            | LexicalError::UnexpectedEqual(_, _)
            | LexicalError::NoRangeStart(_, _)
            | LexicalError::NoRangeEnd(_, _) => write!(f, "{}", self.construct_error()),
        }
    }
}

impl FancyError for LexicalError {
    fn error_ctx(&self) -> (&Vec<char>, (usize, usize)) {
        match self {
            LexicalError::InvalidToken(input, span)
            | LexicalError::InvalidRange(input, span)
            | LexicalError::UnexpectedEqual(input, span)
            | LexicalError::NoRangeStart(input, span)
            | LexicalError::NoRangeEnd(input, span) => (input, *span),
        }
    }

    fn error_msg(&self) -> String {
        let blue = BLUE.bold();

        match self {
            LexicalError::InvalidToken(_, span) => {
                format!(
                    "{blue}@ position {}{blue:#} {WHITE}- Invalid token{WHITE:#}",
                    span.0
                )
            }
            LexicalError::InvalidRange(_, span) => {
                format!(
                    "{blue}@ position {}-{}{blue:#} {WHITE}- Invalid range syntax{WHITE:#}",
                    span.0, span.1
                )
            }
            LexicalError::UnexpectedEqual(_, span) => {
                format!(
                    "{blue}@ position {}-{}{blue:#} {WHITE}- Unexpected equals sign{WHITE:#}",
                    span.0, span.1
                )
            }
            LexicalError::NoRangeStart(_, span) => {
                format!(
                    "{blue}@ position {}{blue:#} {WHITE}- No range start specified{WHITE:#}",
                    span.0
                )
            }
            LexicalError::NoRangeEnd(_, span) => {
                format!(
                    "{blue}@ position {}-{}{blue:#} {WHITE}- No range end specified{WHITE:#}",
                    span.0, span.1
                )
            }
        }
    }

    fn error_hint(&self) -> String {
        match self {
            LexicalError::InvalidToken(_, _) => {
                "Only numbers, spaces, ranges and underscore for numbers allowed ;)".to_owned()
            }
            LexicalError::InvalidRange(_, _) => {
                "Range should be formed like this '1..10' or '1..=10' ;)".to_owned()
            }
            LexicalError::UnexpectedEqual(_, _) => {
                "Equal sign should come after the two dots '1..=10' ;)".to_owned()
            }
            LexicalError::NoRangeStart(_, _) => {
                "Place a number right before the '..' or '..=' with no spaces ;)".to_owned()
            }
            LexicalError::NoRangeEnd(_, _) => {
                "Place a number right after '..' or '..=' with no spaces".to_owned()
            }
        }
    }
}

#[derive(Debug)]
pub enum ParserError {
    MalformedNumber(Vec<char>, (usize, usize)),
    NumberTooLarge(Vec<char>, (usize, usize)),
    RangeStartGreaterThanEnd(Vec<char>, (usize, usize)),
}

impl Error for ParserError {}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::MalformedNumber(_, _)
            | ParserError::NumberTooLarge(_, _)
            | ParserError::RangeStartGreaterThanEnd(_, _) => {
                write!(f, "{}", self.construct_error())
            }
        }
    }
}

impl FancyError for ParserError {
    fn error_ctx(&self) -> (&Vec<char>, (usize, usize)) {
        match self {
            ParserError::NumberTooLarge(input, span)
            | ParserError::MalformedNumber(input, span)
            | ParserError::RangeStartGreaterThanEnd(input, span) => (input, *span),
        }
    }

    fn error_msg(&self) -> String {
        let blue = BLUE.bold();

        match self {
            ParserError::MalformedNumber(_, span) => {
                format!(
                    "{blue}@ position {}-{}{blue:#} {WHITE}- Malformed number{WHITE:#}",
                    span.0, span.1
                )
            }
            ParserError::NumberTooLarge(_, span) => {
                format!(
                    "{blue}@ position {}-{}{blue:#} {WHITE}- Number too large. Largest possible number is 4_294_967_295{WHITE:#}",
                    span.0, span.1
                )
            }
            ParserError::RangeStartGreaterThanEnd(_, span) => {
                format!(
                    "{blue}@ position {}-{}{blue:#} {WHITE}- Range start is greater than range end{WHITE:#}",
                    span.0, span.1
                )
            }
        }
    }

    fn error_hint(&self) -> String {
        match self {
            ParserError::MalformedNumber(_, _) => "Welp! ＼（〇_ｏ）／".to_owned(),
            ParserError::NumberTooLarge(_, _) => "Try a smaller number ;)".to_owned(),
            ParserError::RangeStartGreaterThanEnd(_, _) => {
                "Place the larger number in the left-hand side of the range (e.g. '10..=200') ;)"
                    .to_owned()
            }
        }
    }
}
