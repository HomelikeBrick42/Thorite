use crate::{
    file::Location,
    lexer::{Lexer, LexerError, Token, TokenKind},
    macros::with_location_kind,
};

with_location_kind! {
    #[derive(Debug)]
    pub enum Ast {
        Name {
            name: String,
        },
        Let {
            pattern: AstPattern,
            value: Option<Box<Ast>>,
        },
        Match {
            scrutinee: Box<Ast>,
            arms: Vec<AstMatchArm>,
        },
        BinaryOperator {
            left: Box<Ast>,
            operator: BinaryOperator,
            right: Box<Ast>,
        },
    }

    #[derive(Debug)]
    pub enum AstPattern {
        Name { name: String, typ: Option<Box<Ast>> },
    }

    #[derive(Debug)]
    pub enum BinaryOperator {
        Pipe,
    }

    #[derive(Debug)]
    pub enum ParserError {
        LexerError(LexerError),
        UnexpectedEOF,
        UnexpectedToken(Token),
    }
}

impl From<LexerError> for ParserError {
    fn from(error: LexerError) -> Self {
        ParserError {
            location: error.location.clone(),
            kind: ParserErrorKind::LexerError(error),
        }
    }
}

#[derive(Debug)]
pub struct AstMatchArm {
    pub location: Location,
    pub pattern: AstPattern,
    pub expression: Ast,
}

macro_rules! match_token {
    ($lexer:expr, $kind:pat) => {
        match $lexer.peek() {
            Some(Ok(token @ Token { kind: $kind, .. })) => Ok(Some(token)),
            Some(Ok(_)) | None => Ok(None),
            Some(Err(error)) => Err(error),
        }
    };
}

macro_rules! expect_token {
    ($lexer:ident, $kind:pat) => {
        match $lexer.next() {
            Some(Ok(token @ Token { kind: $kind, .. })) => Ok(token),
            #[allow(unreachable_patterns)]
            Some(Ok(token)) => Err(ParserError {
                location: token.location.clone(),
                kind: ParserErrorKind::UnexpectedToken(token),
            }),
            Some(Err(error)) => Err(error.into()),
            None => Err(ParserError {
                location: $lexer.get_location().clone(),
                kind: ParserErrorKind::UnexpectedEOF,
            }),
        }
    };
}

pub fn parse_statement(lexer: &mut Lexer<'_>) -> Result<Ast, ParserError> {
    let Token {
        location: start_location,
        kind,
    } = lexer.peek().transpose()?.ok_or_else(|| ParserError {
        location: lexer.get_location().clone(),
        kind: ParserErrorKind::UnexpectedEOF,
    })?;
    Ok(Ast {
        kind: match kind {
            TokenKind::LetKeyword => {
                lexer.next();
                AstKind::Let {
                    pattern: parse_pattern(lexer)?,
                    value: if match_token!(lexer, TokenKind::Semicolon)?.is_none() {
                        let value = parse_expression(lexer)?;
                        expect_token!(lexer, TokenKind::Semicolon)?;
                        Some(Box::new(value))
                    } else {
                        None
                    },
                }
            }

            TokenKind::EnumKeyword => {
                lexer.next();

                todo!()
            }

            TokenKind::MatchKeyword => {
                lexer.next();

                todo!()
            }

            _ => {
                let expression = parse_expression(lexer)?;
                expect_token!(lexer, TokenKind::Semicolon)?;
                return Ok(expression);
            }
        },
        location: start_location.combine(lexer.get_location()),
    })
}

pub fn parse_expression(lexer: &mut Lexer<'_>) -> Result<Ast, ParserError> {
    parse_binary_expression(lexer, 0)
}

pub fn parse_binary_expression(
    lexer: &mut Lexer<'_>,
    parent_precedence: u8,
) -> Result<Ast, ParserError> {
    let mut left = parse_primary_expression(lexer)?;

    while let Some(operator_token) = lexer.peek().transpose()? {
        let (operator_precedence, binary_operator_kind) = match operator_token.kind {
            TokenKind::Pipe => (1, BinaryOperatorKind::Pipe),
            _ => break,
        };
        if operator_precedence <= parent_precedence {
            break;
        }

        lexer.next();
        let right = parse_binary_expression(lexer, operator_precedence)?;

        left = Ast {
            location: left.location.combine(&right.location),
            kind: AstKind::BinaryOperator {
                left: Box::new(left),
                operator: BinaryOperator {
                    location: operator_token.location,
                    kind: binary_operator_kind,
                },
                right: Box::new(right),
            },
        };
    }

    Ok(left)
}

pub fn parse_primary_expression(lexer: &mut Lexer<'_>) -> Result<Ast, ParserError> {
    Ok(match expect_token!(lexer, _)? {
        Token {
            location,
            kind: TokenKind::Name(name),
        } => Ast {
            location,
            kind: AstKind::Name { name },
        },

        Token {
            kind: TokenKind::OpenParenthesis,
            ..
        } => {
            let expression = parse_expression(lexer)?;
            expect_token!(lexer, TokenKind::CloseParenthesis)?;
            expression
        }

        token => {
            return Err(ParserError {
                location: token.location.clone(),
                kind: ParserErrorKind::UnexpectedToken(token),
            });
        }
    })
}

pub fn parse_pattern(lexer: &mut Lexer<'_>) -> Result<AstPattern, ParserError> {
    Ok(match expect_token!(lexer, _)? {
        Token {
            location,
            kind: TokenKind::Name(name),
        } => AstPattern {
            location,
            kind: AstPatternKind::Name {
                name,
                typ: if match_token!(lexer, TokenKind::Colon)?.is_some() {
                    let typ = parse_expression(lexer)?;
                    Some(Box::new(typ))
                } else {
                    None
                },
            },
        },

        Token {
            kind: TokenKind::OpenParenthesis,
            ..
        } => {
            let pattern = parse_pattern(lexer)?;
            expect_token!(lexer, TokenKind::CloseParenthesis)?;
            pattern
        }

        token => {
            return Err(ParserError {
                location: token.location.clone(),
                kind: ParserErrorKind::UnexpectedToken(token),
            });
        }
    })
}
