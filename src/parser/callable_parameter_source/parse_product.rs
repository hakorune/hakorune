use crate::ast::ParamDecl;

use super::model::{ParserCallableParameterSourceRowV1, ParserParameterTransferSyntaxV1};

/// One parser-owned parameter-list result.
///
/// The neutral AST projection may be cloned for AST construction. The source
/// rows remain one-shot staging and only the callable source session may
/// consume them.
#[derive(Debug)]
pub(in crate::parser) struct ParsedCallableParameterListV1 {
    neutral: Vec<ParamDecl>,
    receiver_transfer: super::model::ParserReceiverTransferSyntaxV1,
    declared_result_type: Option<crate::ast::TypeReferenceV1>,
    rows: Box<[ParserCallableParameterSourceRowV1]>,
    borrowed_result: Option<super::model::ParserBorrowedResultSyntaxV1>,
}

impl ParsedCallableParameterListV1 {
    #[cfg(test)]
    pub(in crate::parser) fn from_neutral(
        neutral: Vec<ParamDecl>,
    ) -> Result<Self, CallableParameterListIssueV1> {
        let transfers = vec![ParserParameterTransferSyntaxV1::ordinary(); neutral.len()];
        Self::from_parsed(neutral, transfers)
    }

    pub(in crate::parser) fn from_parsed(
        neutral: Vec<ParamDecl>, transfers: Vec<ParserParameterTransferSyntaxV1>,
    ) -> Result<Self, CallableParameterListIssueV1> {
        if neutral.len() != transfers.len() || neutral.iter().zip(&transfers)
            .any(|(declaration, transfer)| !transfer.is_ordinary() && declaration.declared_type_name.is_none()) {
            return Err(CallableParameterListIssueV1::TransferCoverageMismatch);
        }
        let mut rows = Vec::with_capacity(neutral.len());
        for (index, declaration) in neutral.iter().enumerate() {
            let ordinal = u32::try_from(index)
                .map_err(|_| CallableParameterListIssueV1::ParameterOrdinalOverflow)?;
            rows.push(ParserCallableParameterSourceRowV1::parsed(
                ordinal, declaration, transfers[index],
            ).map_err(|_| CallableParameterListIssueV1::UnsupportedDeclaredType)?);
        }
        Ok(Self {
            neutral,
            receiver_transfer: Default::default(),
            declared_result_type: None,
            rows: rows.into_boxed_slice(),
            borrowed_result: None,
        })
    }

    pub(in crate::parser) fn with_declared_result_type(mut self, spelling: Option<&str>) -> Result<Self, String> {
        self.declared_result_type = spelling.map(crate::ast::TypeReferenceV1::parse).transpose()?;
        Ok(self)
    }

    pub(in crate::parser) fn with_receiver_transfer(mut self, transfer: super::model::ParserReceiverTransferSyntaxV1) -> Self {
        self.receiver_transfer = transfer;
        self
    }

    pub(in crate::parser) fn has_modified_parameters(&self) -> bool {
        self.rows.iter().any(|row| !row.transfer().is_ordinary())
    }

    pub(in crate::parser) fn neutral(&self) -> &[ParamDecl] {
        &self.neutral
    }

    pub(in crate::parser) fn with_borrowed_result(mut self, result: Option<super::model::ParserBorrowedResultSyntaxV1>) -> Self {
        self.borrowed_result = result;
        self
    }
    pub(in crate::parser) fn parameter_is_take(&self, ordinal: usize) -> bool {
        self.rows.get(ordinal).is_some_and(|row| row.transfer().is_take())
    }
    pub(in crate::parser) fn has_from(&self) -> bool { self.borrowed_result.is_some() }
    pub(in crate::parser) fn requires_retained_source(&self) -> bool { self.receiver_transfer != super::model::ParserReceiverTransferSyntaxV1::Ordinary || self.has_modified_parameters() || self.has_from() }

    pub(super) fn into_parts(self) -> (Vec<ParamDecl>, Box<[ParserCallableParameterSourceRowV1]>, Option<super::model::ParserBorrowedResultSyntaxV1>, super::model::ParserReceiverTransferSyntaxV1, Option<crate::ast::TypeReferenceV1>) {
        (self.neutral, self.rows, self.borrowed_result, self.receiver_transfer, self.declared_result_type)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::parser) enum CallableParameterListIssueV1 {
    ParameterOrdinalOverflow,
    TransferCoverageMismatch,
    UnsupportedDeclaredType,
}
