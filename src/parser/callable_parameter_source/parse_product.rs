use crate::ast::ParamDecl;

use super::model::ParserCallableParameterSourceRowV1;

/// One parser-owned parameter-list result.
///
/// The neutral AST projection may be cloned for AST construction. The source
/// rows remain one-shot staging and only the callable source session may
/// consume them.
#[derive(Debug)]
pub(in crate::parser) struct ParsedCallableParameterListV1 {
    neutral: Vec<ParamDecl>,
    rows: Box<[ParserCallableParameterSourceRowV1]>,
}

impl ParsedCallableParameterListV1 {
    pub(in crate::parser) fn from_neutral(
        neutral: Vec<ParamDecl>,
    ) -> Result<Self, CallableParameterListIssueV1> {
        let mut rows = Vec::with_capacity(neutral.len());
        for (index, declaration) in neutral.iter().enumerate() {
            let ordinal = u32::try_from(index)
                .map_err(|_| CallableParameterListIssueV1::ParameterOrdinalOverflow)?;
            rows.push(ParserCallableParameterSourceRowV1::ordinary(
                ordinal,
                declaration,
            ));
        }
        Ok(Self {
            neutral,
            rows: rows.into_boxed_slice(),
        })
    }

    pub(in crate::parser) fn neutral(&self) -> &[ParamDecl] {
        &self.neutral
    }

    pub(super) fn into_parts(self) -> (Vec<ParamDecl>, Box<[ParserCallableParameterSourceRowV1]>) {
        (self.neutral, self.rows)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum CallableParameterListIssueV1 {
    ParameterOrdinalOverflow,
}
