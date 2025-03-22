mod errors;
mod lexer;
mod parser;

use lexer::Lexer;
use parser::Parser;

pub fn parse_args(input: &str) -> anyhow::Result<Vec<u32>> {
    let input_chars: Vec<char> = input.chars().collect();

    let mut lexer = Lexer::new(input, &input_chars);
    let tokens = lexer.lex()?;

    let mut parser = Parser::new(&tokens, &input_chars);
    Ok(parser.parse()?)
}
