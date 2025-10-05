use std::{iter::Peekable, num::IntErrorKind, slice::Iter};

use super::{
    errors::ParserError,
    lexer::{Token, TokenKind},
};

pub struct Parser<'a> {
    input_chars: &'a Vec<char>,
    tokens: Peekable<Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token], input_chars: &'a Vec<char>) -> Self {
        Self {
            input_chars,
            tokens: tokens.iter().peekable(),
        }
    }

    fn parse_num(&mut self, num: &String, span: &(usize, usize)) -> Result<u32, ParserError> {
        match num.parse::<u32>() {
            Ok(val) => Ok(val),
            Err(e) if e.kind() == &IntErrorKind::PosOverflow => Err(ParserError::NumberTooLarge(
                self.input_chars.clone(),
                span.to_owned(),
            )),
            Err(_) => Err(ParserError::MalformedNumber(
                self.input_chars.clone(),
                span.to_owned(),
            )),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<u32>, ParserError> {
        let mut nums = vec![];

        for token in self.tokens.by_ref() {
            match token.kind.clone() {
                TokenKind::Int(val) => {
                    nums.push(self.parse_num(&val, &token.span)?);
                }
                TokenKind::Range {
                    start: _,
                    end: _,
                    inclusive: _,
                } => {
                    let start = self.parse_num(&start, &token.span)?;
                    let end = self.parse_num(&end, &token.span)?;
                    if start >= end {
                        return Err(ParserError::RangeStartGreaterThanEnd(
                            self.input_chars.clone(),
                            token.span,
                        ));
                    }

                    if inclusive {
                        for num in start..=end {
                            nums.push(num);
                        }
                    } else {
                        for num in start..end {
                            nums.push(num);
                        }
                    }
                }
                _ => unreachable!(),
            }
        }

        Ok(nums)
    }

    fn parse_int(&mut self) -> Result<u32, ParserError> {
        let token = self.tokens.next().expect("Expected Int token");
        let num_str = match &token.kind {
            TokenKind::Int(val) => val,
            _ => unreachable!(),
        };
        self.str_to_u32(num_str, token.span)
    }

    fn str_to_u32(&self, num_str: &str, span: (usize, usize)) -> Result<u32, ParserError> {
        match num_str.parse::<u32>() {
            Ok(val) => Ok(val),
            Err(e) if e.kind() == &IntErrorKind::PosOverflow => {
                Err(ParserError::NumberTooLarge(self.input_chars.clone(), span))
            }
            Err(_) => Err(ParserError::MalformedNumber(self.input_chars.clone(), span)),
        }
    }

    fn parse_range(&mut self) -> Result<Vec<u32>, ParserError> {
        let token = self.tokens.next().expect("Expected Range token");
        let (start_str, end_str, inclusive) = match &token.kind {
            TokenKind::Range {
                start,
                end,
                inclusive,
            } => (start, end, inclusive),
            _ => unreachable!(),
        };

        let (start, end) = (
            self.str_to_u32(start_str, token.span)?,
            self.str_to_u32(end_str, token.span)?,
        );

        if start >= end {
            return Err(ParserError::RangeStartGreaterThanEnd(
                self.input_chars.clone(),
                token.span,
            ));
        }
        let mut nums = vec![];
        if *inclusive {
            for num in start..=end {
                nums.push(num);
            }
        } else {
            for num in start..end {
                nums.push(num);
            }
        }
        Ok(nums)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let input_chars = "1 1..=5 20".chars().collect::<Vec<char>>();
        let tokens = vec![
            Token::new(TokenKind::Int("1".into()), (0, 1)),
            Token::new(
                TokenKind::Range {
                    start: "1".into(),
                    end: "5".into(),
                    inclusive: true,
                },
                (3, 7),
            ),
            Token::new(TokenKind::Int("20".into()), (9, 10)),
        ];
        let mut parser = Parser::new(&tokens, &input_chars);
        let nums = parser.parse().unwrap();
        assert_eq!(nums, vec![1, 1, 2, 3, 4, 5, 20]);
    }

    #[test]
    fn test_parse_invalid() {
        let input_chars = "1 5..=1 20".chars().collect::<Vec<char>>();
        let tokens = vec![
            Token::new(TokenKind::Int("1".into()), (0, 1)),
            Token::new(
                TokenKind::Range {
                    start: "5".into(),
                    end: "1".into(),
                    inclusive: true,
                },
                (3, 7),
            ),
            Token::new(TokenKind::Int("20".into()), (9, 10)),
        ];
        let mut parser = Parser::new(&tokens, &input_chars);
        let err = parser.parse();
        if let Err(ParserError::RangeStartGreaterThanEnd(_, span)) = err {
            println!("{}", err.err().unwrap());
            assert_eq!(span, (3, 7));
        } else {
            panic!("Expected RangeStartGreaterThanEnd error");
        }
    }

    #[test]
    fn test_parse_invalid_number_too_large() {
        let input_chars = "1 100_000_000_000".chars().collect::<Vec<char>>();
        let tokens = vec![
            Token::new(TokenKind::Int("1".into()), (0, 1)),
            Token::new(TokenKind::Int("100000000000".into()), (3, 17)),
        ];
        let mut parser = Parser::new(&tokens, &input_chars);
        let err = parser.parse();
        if let Err(ParserError::NumberTooLarge(_, span)) = err {
            println!("{}", err.err().unwrap());
            assert_eq!(span, (3, 17));
        } else {
            panic!("Expected NumberTooLarge error");
        }
    }
}
