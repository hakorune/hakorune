#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct RuneAttr {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct DeclarationAttrs {
    pub runes: Vec<RuneAttr>,
}

impl DeclarationAttrs {
    pub fn is_empty(&self) -> bool {
        self.runes.is_empty()
    }
}
