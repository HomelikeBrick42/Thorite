use crate::{
    file::{FileId, Location},
    macros::with_location_kind,
};

with_location_kind! {
    #[derive(Debug)]
    pub enum Token {
        Name(String),
        LetKeyword,
        MatchKeyword,
        Semicolon,
        Colon,
        Comma,
        OpenBracket,
        CloseBracket,
        OpenParenthesis,
        CloseParenthesis,
        Pipe,
    }

    #[derive(Debug)]
    pub enum LexerError {
        UnexpectedChar(char),
    }
}

#[derive(Debug)]
pub struct Lexer<'a> {
    location: Location,
    source: &'a str,
    chars: std::iter::Peekable<std::str::CharIndices<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(file: FileId, source: &'a str) -> Self {
        Self {
            location: Location {
                file,
                line: 1,
                span: 0..0,
            },
            source,
            chars: source.char_indices().peekable(),
        }
    }

    pub fn get_location(&self) -> &Location {
        &self.location
    }

    pub fn peek_char(&mut self) -> Option<char> {
        self.chars.peek().map(|&(_, c)| c)
    }

    pub fn next_char(&mut self) -> Option<char> {
        let (_, c) = self.chars.next()?;
        let i = self.chars.peek().map_or(self.source.len(), |&(i, _)| i);
        self.location.span = i..i;
        if c == '\n' {
            self.location.line += 1;
        }
        Some(c)
    }

    pub fn peek(&self) -> Option<<Self as Iterator>::Item> {
        Self {
            location: self.location.clone(),
            source: self.source,
            chars: self.chars.clone(),
        }
        .next()
    }
}

impl Iterator for Lexer<'_> {
    type Item = Result<Token, LexerError>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let start_location = self.location.clone();
            macro_rules! location {
                () => {
                    start_location.combine(&self.location)
                };
            }
            macro_rules! simple {
                ($kind:expr) => {
                    Token {
                        location: location!(),
                        kind: $kind,
                    }
                };
            }
            return Some(Ok(match self.next_char() {
                Some(';') => simple!(TokenKind::Semicolon),
                Some(':') => simple!(TokenKind::Colon),
                Some(',') => simple!(TokenKind::Comma),
                Some('{') => simple!(TokenKind::OpenBracket),
                Some('}') => simple!(TokenKind::CloseBracket),
                Some('(') => simple!(TokenKind::OpenParenthesis),
                Some(')') => simple!(TokenKind::CloseParenthesis),
                Some('|') => simple!(TokenKind::Pipe),

                Some(c) if c.is_ascii_alphabetic() || c == '_' => {
                    while self
                        .peek_char()
                        .map_or(false, |c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        self.next_char();
                    }
                    let location = location!();
                    let name = &self.source[location.span.clone()];
                    Token {
                        location,
                        kind: match name {
                            "let" => TokenKind::LetKeyword,
                            "match" => TokenKind::MatchKeyword,
                            _ => TokenKind::Name(name.to_string()),
                        },
                    }
                }

                Some(c) if c.is_whitespace() => continue,

                None => return None,
                Some(c) => {
                    return Some(Err(LexerError {
                        location: location!(),
                        kind: LexerErrorKind::UnexpectedChar(c),
                    }));
                }
            }));
        }
    }
}
