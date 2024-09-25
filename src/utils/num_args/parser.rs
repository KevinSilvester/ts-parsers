use std::{iter::Peekable, slice::Iter};

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

    pub fn parse(&mut self) -> Result<Vec<u32>, ParserError> {
        let mut nums = vec![];

        while let Some(token) = self.tokens.next() {
            match token.kind {
                TokenKind::Int(val) => {
                    nums.push(val);
                }
                TokenKind::Range {
                    start,
                    end,
                    inclusive,
                } => {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid() {
        let input_chars = "1 1..=5 20".chars().collect::<Vec<char>>();
        let tokens = vec![
            Token::new(TokenKind::Int(1), (0, 1)),
            Token::new(
                TokenKind::Range {
                    start: 1,
                    end: 5,
                    inclusive: true,
                },
                (3, 7),
            ),
            Token::new(TokenKind::Int(20), (9, 10)),
        ];
        let mut parser = Parser::new(&tokens, &input_chars);
        let nums = parser.parse().unwrap();
        assert_eq!(nums, vec![1, 1, 2, 3, 4, 5, 20]);
    }

    #[test]
    fn test_parse_invalid() {
        let input_chars = "1 5..=1 20".chars().collect::<Vec<char>>();
        let tokens = vec![
            Token::new(TokenKind::Int(1), (0, 1)),
            Token::new(
                TokenKind::Range {
                    start: 5,
                    end: 1,
                    inclusive: true,
                },
                (3, 7),
            ),
            Token::new(TokenKind::Int(20), (9, 10)),
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
}
