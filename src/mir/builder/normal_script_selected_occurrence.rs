//! One selected-Script Program occurrence handed from the work plan to semantics.
//!
//! The work plan classifies each Program item once. This private receipt keeps
//! the original ordinal, that exact classifier result, and the one typed
//! top-level callable transfer fact together without carrying runtime work.

use crate::ast::ASTNode;

use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct SelectedScriptProgramOccurrenceV1 {
    source_statement_index: usize,
    admission: NormalScriptProgramItemAdmissionV1,
    transfer: SelectedScriptProgramTransferV1,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SelectedScriptProgramTransferV1 {
    None,
    TopLevelCallable,
}

impl SelectedScriptProgramOccurrenceV1 {
    pub(super) fn new(
        source_statement_index: usize,
        statement: &ASTNode,
        admission: NormalScriptProgramItemAdmissionV1,
    ) -> Self {
        let transfer = matches!(statement, ASTNode::FunctionDeclaration { .. })
            .then_some(SelectedScriptProgramTransferV1::TopLevelCallable)
            .unwrap_or(SelectedScriptProgramTransferV1::None);
        Self {
            source_statement_index,
            admission,
            transfer,
        }
    }

    pub(super) const fn source_statement_index(self) -> usize {
        self.source_statement_index
    }

    pub(super) const fn admission(self) -> NormalScriptProgramItemAdmissionV1 {
        self.admission
    }

    pub(super) const fn transfers_top_level_callable(self) -> bool {
        matches!(self.transfer, SelectedScriptProgramTransferV1::TopLevelCallable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};

    #[test]
    fn top_level_function_receipt_keeps_its_ordinal_and_transfer_fact() {
        let function = ASTNode::FunctionDeclaration {
            name: "helper".to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body: Vec::new(),
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: Default::default(),
            span: Span::unknown(),
        };
        let occurrence = SelectedScriptProgramOccurrenceV1::new(
            3,
            &function,
            NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
        );
        assert_eq!(occurrence.source_statement_index(), 3);
        assert!(occurrence.transfers_top_level_callable());
    }

    #[test]
    fn ordinary_runtime_receipt_is_not_a_callable_transfer() {
        let literal = ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        };
        let occurrence = SelectedScriptProgramOccurrenceV1::new(
            0,
            &literal,
            NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression,
        );
        assert!(!occurrence.transfers_top_level_callable());
    }
}
