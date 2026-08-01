//! Named residual Script responsibilities retained by the existing root lowerer.
//!
//! This sidecar records only source shapes intentionally outside the current
//! Script Complete closure.  It does not select lowering, alter admission, or
//! inspect child syntax.  The root-admission witness remains the sole proof
//! that the operational admission and source node belong together.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::ScriptRootSemanticDispositionV1;

use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;
use super::normal_script_root_admission_witness::ScriptRootAdmissionWitnessV1;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum ScriptDeferredResidualKindV1 {
    FunctionCall,
    Call,
    MethodCall,
    Loop,
    FieldAccess,
    Index,
    New,
    RecordUpdate,
    Box(NormalScriptProgramItemAdmissionV1),
    TryCatch,
    Throw,
    NonfinalReturn,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct ScriptDeferredResidualEntryV1 {
    source_statement_index: usize,
    admission: NormalScriptProgramItemAdmissionV1,
    kind: ScriptDeferredResidualKindV1,
}

impl ScriptDeferredResidualEntryV1 {
    pub(super) fn source_statement_index(self) -> usize {
        self.source_statement_index
    }

    pub(super) fn kind(self) -> ScriptDeferredResidualKindV1 {
        self.kind
    }

    pub(super) fn admission(self) -> NormalScriptProgramItemAdmissionV1 {
        self.admission
    }
}

#[derive(Debug)]
pub(super) struct PreparedScriptDeferredResidualRegistryV1 {
    entries: Box<[ScriptDeferredResidualEntryV1]>,
}

impl PreparedScriptDeferredResidualRegistryV1 {
    pub(super) fn entries(&self) -> &[ScriptDeferredResidualEntryV1] {
        &self.entries
    }
}

#[derive(Debug)]
pub(super) struct ScriptDeferredResidualRegistryBuilderV1 {
    entries: Vec<ScriptDeferredResidualEntryV1>,
}

impl ScriptDeferredResidualRegistryBuilderV1 {
    pub(super) fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub(super) fn record(
        &mut self,
        source_statement_index: usize,
        statement: &ASTNode,
        witness: ScriptRootAdmissionWitnessV1,
    ) {
        let kind = match statement {
            ASTNode::FunctionCall { .. } => Some(ScriptDeferredResidualKindV1::FunctionCall),
            ASTNode::Call { .. } => Some(ScriptDeferredResidualKindV1::Call),
            ASTNode::MethodCall { .. } => Some(ScriptDeferredResidualKindV1::MethodCall),
            ASTNode::Loop { .. } => Some(ScriptDeferredResidualKindV1::Loop),
            ASTNode::FieldAccess { .. } => Some(ScriptDeferredResidualKindV1::FieldAccess),
            ASTNode::Index { .. } => Some(ScriptDeferredResidualKindV1::Index),
            ASTNode::New { .. } => Some(ScriptDeferredResidualKindV1::New),
            ASTNode::RecordUpdate { .. } => Some(ScriptDeferredResidualKindV1::RecordUpdate),
            ASTNode::TryCatch { .. } => Some(ScriptDeferredResidualKindV1::TryCatch),
            ASTNode::Throw { .. } => Some(ScriptDeferredResidualKindV1::Throw),
            ASTNode::Return { .. }
                if matches!(
                    witness.semantic(),
                    ScriptRootSemanticDispositionV1::Deferred(_)
                ) =>
            {
                Some(ScriptDeferredResidualKindV1::NonfinalReturn)
            }
            ASTNode::BoxDeclaration { .. }
                if matches!(
                    witness.semantic(),
                    ScriptRootSemanticDispositionV1::Deferred(_)
                ) =>
            {
                Some(ScriptDeferredResidualKindV1::Box(witness.admission()))
            }
            _ => None,
        };
        if let Some(kind) = kind {
            self.entries.push(ScriptDeferredResidualEntryV1 {
                source_statement_index,
                admission: witness.admission(),
                kind,
            });
        }
    }

    pub(super) fn seal(self) -> PreparedScriptDeferredResidualRegistryV1 {
        PreparedScriptDeferredResidualRegistryV1 {
            entries: self.entries.into_boxed_slice(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, Span};

    #[test]
    fn registry_records_only_named_residual_families() {
        let mut registry = ScriptDeferredResidualRegistryBuilderV1::new();
        let call = ASTNode::FunctionCall {
            name: "helper".to_owned(),
            arguments: Vec::new(),
            span: Span::unknown(),
        };
        let witness = ScriptRootAdmissionWitnessV1::issue(
            3,
            4,
            &call,
            Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
            false,
        )
        .expect("call witness");
        registry.record(
            3,
            &call,
            witness,
        );
        let entries = registry.seal();
        assert_eq!(entries.entries().len(), 1);
        assert_eq!(entries.entries()[0].source_statement_index(), 3);
        assert_eq!(
            entries.entries()[0].kind(),
            ScriptDeferredResidualKindV1::FunctionCall
        );
        assert_eq!(
            entries.entries()[0].admission(),
            NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression
        );
    }
}
