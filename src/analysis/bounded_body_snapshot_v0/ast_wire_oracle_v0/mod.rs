//! Test-only independent AST to ProgramV0 wire-observation oracle.
//!
//! This module may use the canonical AST and snapshot algebra. It must never
//! call or import the ProgramV0 serializer, JSON, MIR, planner, or runtime.

mod expr;
mod stmt;
#[cfg(test)]
mod tests;

use crate::ast::ASTNode;

use super::{
    AtomKeyV0, AtomValueV0, BoundedBodyAnalysisSnapshotV0, ChildRoleV0, PathV0,
    SnapshotBuildErrorV0, SnapshotBuilderV0, SnapshotNodeIndexV0, WireExprKindV0, WireNodeKindV0,
    WireStmtKindV0,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum AstWireOracleErrorV0 {
    Unsupported {
        path: String,
        source_kind: &'static str,
        reason: &'static str,
    },
    Snapshot(SnapshotBuildErrorV0),
}

pub(crate) fn observe_ast_body_v0(
    body: &[ASTNode],
) -> Result<BoundedBodyAnalysisSnapshotV0, AstWireOracleErrorV0> {
    let mut oracle = AstWireOracleV0 {
        builder: SnapshotBuilderV0::new(0),
    };
    oracle.emit_body(body, PathV0::root_body(), 1, true)?;
    oracle
        .builder
        .finish()
        .map_err(AstWireOracleErrorV0::Snapshot)
}

struct AstWireOracleV0 {
    builder: SnapshotBuilderV0,
}

impl AstWireOracleV0 {
    fn reserve(
        &mut self,
        path: &PathV0,
        kind: WireNodeKindV0,
        depth: usize,
    ) -> Result<SnapshotNodeIndexV0, AstWireOracleErrorV0> {
        self.builder
            .reserve_node(path.clone(), kind, depth)
            .map_err(AstWireOracleErrorV0::Snapshot)
    }

    fn seal(
        &mut self,
        index: SnapshotNodeIndexV0,
        atoms: Vec<(AtomKeyV0, AtomValueV0)>,
        children: Vec<(ChildRoleV0, SnapshotNodeIndexV0)>,
    ) -> Result<(), AstWireOracleErrorV0> {
        self.builder
            .seal_node(index, atoms, children)
            .map_err(AstWireOracleErrorV0::Snapshot)
    }

    fn unsupported<T>(
        path: &PathV0,
        source_kind: &'static str,
        reason: &'static str,
    ) -> Result<T, AstWireOracleErrorV0> {
        Err(AstWireOracleErrorV0::Unsupported {
            path: path.to_string(),
            source_kind,
            reason,
        })
    }
}
