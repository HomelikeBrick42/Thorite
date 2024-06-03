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

#[derive(Debug, Clone)]
pub struct Location {
    pub file: FileId,
    pub line: usize,
    pub span: Range<usize>,
}

impl Location {
    pub fn combine(&self, other: &Self) -> Self {
        if self.file != other.file {
            return self.clone();
        }

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
