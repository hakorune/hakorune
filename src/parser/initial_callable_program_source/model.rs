use crate::ast::{ASTNode, BoxMethodInventoryOrdinalV1};

use super::super::callable_source_anchor::PreparedCallableSourceV1;
use super::syntax_loan::{
    InitialCallableProgramSyntaxLoanV1, InitialCallableProgramSyntaxRowRefV1,
};

/// Final placement of one parser-issued callable anchor.
///
/// Placement is private cache data, never callable identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum InitialCallableFinalSlotV1 {
    TopLevel {
        statement: u32,
    },
    BoxMethod {
        statement: u32,
        method: BoxMethodInventoryOrdinalV1,
    },
}

#[derive(Debug)]
pub(super) struct VerifiedInitialCallableSourceRowV1 {
    source: PreparedCallableSourceV1,
    final_slot: InitialCallableFinalSlotV1,
}

impl VerifiedInitialCallableSourceRowV1 {
    pub(super) fn new(
        source: PreparedCallableSourceV1,
        final_slot: InitialCallableFinalSlotV1,
    ) -> Self {
        Self { source, final_slot }
    }

    pub(super) fn source(&self) -> &PreparedCallableSourceV1 {
        &self.source
    }

    pub(super) fn final_slot(&self) -> InitialCallableFinalSlotV1 {
        self.final_slot
    }
}

/// Non-splittable initial parser Program plus its complete callable source set.
///
/// The sole issuer is the parser finalizer.  There is intentionally no Clone,
/// arbitrary constructor, or consuming parts projection.
#[derive(Debug)]
pub(crate) struct VerifiedInitialCallableProgramSourceV1 {
    ast: ASTNode,
    sources: Box<[PreparedCallableSourceV1]>,
    slots: Box<[InitialCallableFinalSlotV1]>,
}

impl VerifiedInitialCallableProgramSourceV1 {
    pub(super) fn issue(ast: ASTNode, rows: Vec<VerifiedInitialCallableSourceRowV1>) -> Self {
        let mut sources = Vec::with_capacity(rows.len());
        let mut slots = Vec::with_capacity(rows.len());
        for row in rows {
            sources.push(row.source);
            slots.push(row.final_slot);
        }
        Self {
            ast,
            sources: sources.into_boxed_slice(),
            slots: slots.into_boxed_slice(),
        }
    }

    pub(crate) fn ast(&self) -> &ASTNode {
        &self.ast
    }

    pub(in crate::parser) fn callable_rows(&self) -> &[PreparedCallableSourceV1] {
        &self.sources
    }

    pub(crate) fn into_ast(self) -> ASTNode {
        self.ast
    }

    pub(in crate::parser) fn into_transform_parts(
        self,
    ) -> (
        ASTNode,
        Box<[PreparedCallableSourceV1]>,
        Box<[InitialCallableFinalSlotV1]>,
    ) {
        (self.ast, self.sources, self.slots)
    }

    /// Lend exact callable syntax without allowing an AST reference to escape.
    pub(in crate::parser) fn with_callable_syntax<R>(
        &self,
        callback: impl for<'syntax> FnOnce(InitialCallableProgramSyntaxLoanV1<'syntax>) -> R,
    ) -> R {
        let rows = self
            .sources
            .iter()
            .zip(self.slots.iter().copied())
            .map(|(source, slot)| {
                InitialCallableProgramSyntaxRowRefV1::from_verified(&self.ast, source, slot)
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        callback(InitialCallableProgramSyntaxLoanV1::new(rows))
    }
}
