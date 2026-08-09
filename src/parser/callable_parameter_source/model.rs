/// AST-free neutral parameter syntax carried by the parser handoff.
///
/// This is not yet a transfer-source row. In particular, absence of transfer
/// metadata must not be interpreted as `Ordinary`, Handle demand, or Home ABI.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolverMethodParameterSyntaxV1 {
    name: Box<str>,
    declared_type_name: Option<Box<str>>,
}

impl ResolverMethodParameterSyntaxV1 {
    pub(super) fn from_neutral_syntax(
        name: String,
        declared_type_name: Option<String>,
    ) -> Self {
        Self {
            name: name.into_boxed_str(),
            declared_type_name: declared_type_name.map(String::into_boxed_str),
        }
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn declared_type_name(&self) -> Option<&str> {
        self.declared_type_name.as_deref()
    }
}
