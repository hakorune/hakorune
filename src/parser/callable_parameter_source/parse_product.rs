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
    #[cfg(test)]
    pub(in crate::parser) fn from_neutral(
        neutral: Vec<ParamDecl>,
    ) -> Result<Self, CallableParameterListIssueV1> {
        let transfers = vec![false; neutral.len()];
        Self::from_parsed(neutral, transfers)
    }

    pub(in crate::parser) fn from_parsed(
        neutral: Vec<ParamDecl>, take: Vec<bool>,
    ) -> Result<Self, CallableParameterListIssueV1> {
        if neutral.len() != take.len() || neutral.iter().zip(&take)
            .any(|(declaration, take)| *take && declaration.declared_type_name.is_none()) {
            return Err(CallableParameterListIssueV1::TransferCoverageMismatch);
        }
        let mut rows = Vec::with_capacity(neutral.len());
        for (index, declaration) in neutral.iter().enumerate() {
            let ordinal = u32::try_from(index)
                .map_err(|_| CallableParameterListIssueV1::ParameterOrdinalOverflow)?;
            rows.push(ParserCallableParameterSourceRowV1::parsed(
                ordinal, declaration, take[index],
            ));
        }
        Ok(Self {
            neutral,
            rows: rows.into_boxed_slice(),
        })
    }

    pub(in crate::parser) fn has_take(&self) -> bool {
        self.rows.iter().any(|row| !row.transfer().is_ordinary())
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
    TransferCoverageMismatch,
}
