use std::{iter::Peekable, str::Chars};

use super::errors::LexicalError;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenKind {
    Space,
    Int(String),
    Range {
        start: String,
        end: String,
        inclusive: bool,
    },
}

#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub span: (usize, usize),
}

impl Token {
    pub fn new(kind: TokenKind, span: (usize, usize)) -> Self {
        Self { kind, span }
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    pub input_chars: &'a Vec<char>,
    input: Peekable<Chars<'a>>,
    position: usize,
    lexing_range: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str, input_chars: &'a Vec<char>) -> Self {
        Self {
            input_chars,
            input: input.chars().peekable(),
            position: 1,
            lexing_range: false,
        }
    }

    fn advance(&mut self) {
        self.input.next();
        self.position += 1;
    }

    pub fn lex(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens = vec![];

        while let Some(ch) = self.input.peek() {
            match *ch {
                ' ' if !self.lexing_range => {
                    tokens.push(Token::new(
                        TokenKind::Space,
                        (self.position - 1, self.position),
                    ));
                    self.advance();
                }
                ' ' if self.lexing_range => {
                    return Err(LexicalError::NoRangeEnd(
                        self.input_chars.clone(),
                        (tokens.last().unwrap().span.0, self.position - 1),
                    ));
                }
                '0'..='9' if !self.lexing_range => {
                    let number = self.tokenize_numbers();
                    tokens.push(number);
                }
                '0'..='9' if self.lexing_range => {
                    let number = self.tokenize_numbers();
                    let mut range = tokens.pop().unwrap();

                    let range_value = match range.kind {
                        TokenKind::Range {
                            start,
                            end: _,
                            inclusive,
                        } => (start, inclusive),
                        _ => unreachable!(),
                    };
                    let number_value = match number.kind {
                        TokenKind::Int(val) => val,
                        _ => unreachable!(),
                    };

                    range.span.1 = number.span.1;
                    range.kind = TokenKind::Range {
                        start: range_value.0,
                        end: number_value,
                        inclusive: range_value.1,
                    };

                    tokens.push(range);
                    self.lexing_range = false;
                }
                '.' => {
                    let range = self.tokenize_range(tokens.last())?;
                    tokens.pop();
                    tokens.push(range);
                }
                '\0' => break,
                _ => {
                    return Err(LexicalError::InvalidToken(
                        self.input_chars.clone(),
                        (self.position, self.position),
                    ));
                }
            }
        }

        if self.lexing_range {
            return Err(LexicalError::NoRangeEnd(
                self.input_chars.clone(),
                (tokens.last().unwrap().span.0, self.position - 1),
            ));
        }

        Ok(tokens
            .into_iter()
            .filter(|t| t.kind != TokenKind::Space)
            .collect::<Vec<Token>>())
    }

    fn tokenize_numbers(&mut self) -> Token {
        let mut number = String::new();
        let start_pos = self.position;

        while let Some(ch @ ('0'..='9' | '_')) = self.input.peek() {
            if *ch != '_' {
                number.push(*ch);
            }
            self.advance();
        }
        Token::new(TokenKind::Int(number), (start_pos, self.position - 1))
    }

    fn tokenize_range(&mut self, last_token: Option<&Token>) -> Result<Token, LexicalError> {
        #[allow(unused_assignments)]
        let mut start = String::new();
        #[allow(unused_assignments)]
        let mut start_pos = 0;

        if let Some(token) = last_token {
            match token.kind.clone() {
                TokenKind::Int(val) => {
                    start = val.clone();
                    start_pos = token.span.0
                }
                _ => {
                    return Err(LexicalError::NoRangeStart(
                        self.input_chars.clone(),
                        (self.position, self.position),
                    ));
                }
            }
        } else {
            return Err(LexicalError::NoRangeStart(
                self.input_chars.clone(),
                (self.position, self.position),
            ));
        }

        self.lexing_range = true;

        let mut dot_count = 0;
        let mut inclusive = false;
        let rng_start_pos = self.position;
        let mut prev_ch = '\0';

        while let Some(ch @ ('.' | '=')) = self.input.peek() {
            match *ch {
                '.' => {
                    if prev_ch == '=' {
                        return Err(LexicalError::UnexpectedEqual(
                            self.input_chars.clone(),
                            (rng_start_pos, self.position),
                        ));
                    }

                    dot_count += 1;

                    prev_ch = *ch;
                    self.advance();
                }
                '=' => {
                    inclusive = true;
                    prev_ch = *ch;
                    self.advance();
                }
                _ => {}
            }
        }

        if dot_count != 2 {
            return Err(LexicalError::InvalidRange(
                self.input_chars.clone(),
                (rng_start_pos, self.position - 1),
            ));
        }

        Ok(Token::new(
            TokenKind::Range {
                start,
                end: String::new(),
                inclusive,
            },
            (start_pos, 0),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lex() {
        let input = "1 2 3 4 5 6 7 8 9 10 11 12 13 14 15";
        let input_chars = input.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input, &input_chars);
        let tokens = lexer.lex().unwrap();
        assert_eq!(tokens.len(), 15);
    }

    #[test]
    fn test_lex_range() {
        let input = "1..10 5 20..=40";
        let input_chars = input.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input, &input_chars);
        let tokens = lexer.lex().unwrap();
        dbg!(&tokens);
        assert_eq!(tokens.len(), 3);
    }

    #[test]
    fn test_lex_invalid_range() {
        let input = "1..=10..20";
        let input_chars = input.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input, &input_chars);
        let tokens = lexer.lex();
        if let Err(LexicalError::NoRangeStart(_, span)) = tokens {
            println!("{}", tokens.err().unwrap());
            assert_eq!(span, (7, 7));
        } else {
            panic!("Expected NoRangeStart error");
        }

        let input = "..33";
        let input_chars = input.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input, &input_chars);
        let tokens = lexer.lex();
        if let Err(LexicalError::NoRangeStart(_, span)) = tokens {
            println!("{}", tokens.err().unwrap());
            assert_eq!(span, (1, 1));
        } else {
            panic!("Expected NoRangeStart error");
        }

        let input = "10 1.. 5";
        let input_chars = input.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input, &input_chars);
        let tokens = lexer.lex();
        if let Err(LexicalError::NoRangeEnd(_, span)) = tokens {
            println!("{}", tokens.err().unwrap());
            assert_eq!(span, (4, 6));
        } else {
            panic!("Expected NoRangeEnd error");
        }

        let input = "10..";
        let input_chars = input.chars().collect::<Vec<char>>();
        let mut lexer = Lexer::new(input, &input_chars);
        let tokens = lexer.lex();
        if let Err(LexicalError::NoRangeEnd(_, span)) = tokens {
            println!("{}", tokens.err().unwrap());
            assert_eq!(span, (1, 4));
        } else {
            panic!("Expected NoRangeEnd error");
        }
    }
}
