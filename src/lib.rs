use paste::paste;
use slotmap::{new_key_type, SlotMap};
use std::{ops::Range, path::PathBuf};

new_key_type! {
    pub struct FileId;
}

#[derive(Debug)]
pub struct File {
    pub file_path: PathBuf,
    pub source: String,
}

pub type Files = SlotMap<FileId, File>;

#[derive(Debug)]
pub struct Location {
    pub file: FileId,
    pub line: usize,
    pub span: Range<usize>,
}

impl Location {
    pub fn combine(&self, other: &Self) -> Self {
        assert_eq!(self.file, other.file);
        assert!(self.line <= other.line);
        assert!(self.span.start <= other.span.start);
        assert!(self.span.end <= other.span.end);
        Location {
            file: self.file,
            line: self.line,
            span: self.span.start..other.span.end,
        }
    }
}

macro_rules! with_location_kind {
    {$(
        $(#[$($attrs:tt)+])*
        $vis:vis enum $name:ident {
            $($body:tt)*
        }
    )*} => {
        paste! {
            $(
                $(#[$($attrs)+])*
                $vis struct $name {
                    pub location: crate::Location,
                    pub kind: [<$name Kind>],
                }

                $(#[$($attrs)+])*
                $vis enum [<$name Kind>] {
                    $($body)*
                }
            )*
        }
    };
}

with_location_kind! {
    #[derive(Debug)]
    pub enum Token {
        Name(String),
        EnumKeyword,
        Colon,
        Comma,
    }
}

new_key_type! {
    pub struct AstId;
}

pub type AstNodes = SlotMap<AstId, Ast>;

with_location_kind! {
    #[derive(Debug)]
    pub enum Ast {
        Name {
            name: String,
        },
        Let {
            pattern: AstPattern,
            value: Option<AstId>,
        },
        Match {
            scrutinee: AstId,
            arms: Vec<AstMatchArm>,
        },
    }

    #[derive(Debug)]
    pub enum AstPattern {
        Name { name: String, typ: Option<AstId> },
    }
}

#[derive(Debug)]
pub struct AstMatchArm {
    pub pattern: AstPattern,
    pub expression: AstId,
}
