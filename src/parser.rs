use crate::{file::Location, macros::with_location_kind};
use slotmap::{new_key_type, SlotMap};

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
        Enum {
            name: String,
            variants: Vec<AstEnumVariant>,
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

    #[derive(Debug)]
    pub enum AstEnumVariant {
        Unit { name: String },
    }
}

#[derive(Debug)]
pub struct AstMatchArm {
    pub location: Location,
    pub pattern: AstPattern,
    pub expression: AstId,
}
