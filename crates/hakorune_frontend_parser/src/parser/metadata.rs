use crate::ast::RuneAttr;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ParserMetadata {
    pub runes: Vec<RuneAttr>,
}
