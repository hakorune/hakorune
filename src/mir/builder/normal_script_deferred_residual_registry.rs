//! Named residual Script responsibilities retained by the existing root lowerer.
//!
//! This sidecar records only source shapes intentionally outside the current
//! Script Complete closure.  It does not select lowering, alter admission, or
//! inspect child syntax.  The root-admission decision remains the sole proof
//! that the operational admission and source node belong together.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::ScriptRootSemanticDispositionV1;

use super::normal_script_program_item_admission::NormalScriptProgramItemAdmissionV1;
use super::normal_script_root_admission_witness::ScriptRootSemanticDecisionV1;

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
        Self {
            entries: Vec::new(),
        }
    }

    pub(super) fn record(
        &mut self,
        source_statement_index: usize,
        statement: &ASTNode,
        decision: ScriptRootSemanticDecisionV1,
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
                    decision.semantic(),
                    ScriptRootSemanticDispositionV1::Deferred(_)
                ) =>
            {
                Some(ScriptDeferredResidualKindV1::NonfinalReturn)
            }
            ASTNode::BoxDeclaration { .. }
                if matches!(
                    decision.semantic(),
                    ScriptRootSemanticDispositionV1::Deferred(_)
                ) =>
            {
                Some(ScriptDeferredResidualKindV1::Box(decision.admission()))
            }
            _ => None,
        };
        if let Some(kind) = kind {
            self.entries.push(ScriptDeferredResidualEntryV1 {
                source_statement_index,
                admission: decision.admission(),
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
    use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
    use std::collections::HashMap;

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    #[test]
    fn registry_records_only_named_residual_families() {
        let mut registry = ScriptDeferredResidualRegistryBuilderV1::new();
        let call = ASTNode::FunctionCall {
            name: "helper".to_owned(),
            arguments: Vec::new(),
            span: Span::unknown(),
        };
        let decision = ScriptRootSemanticDecisionV1::decide(
            3,
            4,
            &call,
            Some(NormalScriptProgramItemAdmissionV1::DirectPortAwareExpression),
            false,
        )
        .expect("call decision");
        registry.record(3, &call, decision);
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

    #[test]
    fn registry_names_every_current_root_only_residual_family() {
        use NormalScriptProgramItemAdmissionV1 as Admission;
        use ScriptDeferredResidualKindV1 as Kind;

        let cases = vec![
            (
                ASTNode::Call {
                    callee: Box::new(integer(1)),
                    arguments: Vec::new(),
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::Call,
            ),
            (
                ASTNode::MethodCall {
                    object: Box::new(integer(1)),
                    method: "call".to_owned(),
                    arguments: Vec::new(),
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::MethodCall,
            ),
            (
                ASTNode::Loop {
                    condition: Box::new(integer(1)),
                    body: Vec::new(),
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::Loop,
            ),
            (
                ASTNode::FieldAccess {
                    object: Box::new(integer(1)),
                    field: "value".to_owned(),
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::FieldAccess,
            ),
            (
                ASTNode::Index {
                    target: Box::new(integer(1)),
                    index: Box::new(integer(0)),
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::Index,
            ),
            (
                ASTNode::New {
                    class: "Box".to_owned(),
                    arguments: Vec::new(),
                    field_initializers: Vec::new(),
                    type_arguments: Vec::new(),
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::New,
            ),
            (
                ASTNode::RecordUpdate {
                    base: Box::new(integer(1)),
                    updates: Vec::new(),
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::RecordUpdate,
            ),
            (
                ASTNode::BoxDeclaration {
                    name: "Box".to_owned(),
                    fields: Vec::new(),
                    field_decls: Vec::new(),
                    public_fields: Vec::new(),
                    private_fields: Vec::new(),
                    methods: HashMap::new(),
                    constructors: HashMap::new(),
                    init_fields: Vec::new(),
                    weak_fields: Vec::new(),
                    delegates: Vec::new(),
                    invariants: Vec::new(),
                    transitions: Vec::new(),
                    is_interface: false,
                    is_record: false,
                    extends: Vec::new(),
                    implements: Vec::new(),
                    type_parameters: Vec::new(),
                    is_sync: false,
                    is_static: false,
                    static_init: None,
                    attrs: DeclarationAttrs::default(),
                    span: Span::unknown(),
                },
                Admission::NonPlainInstanceFullLifecycle,
                Kind::Box(Admission::NonPlainInstanceFullLifecycle),
            ),
            (
                ASTNode::TryCatch {
                    try_body: Vec::new(),
                    catch_clauses: Vec::new(),
                    finally_body: None,
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::TryCatch,
            ),
            (
                ASTNode::Throw {
                    expression: Box::new(integer(1)),
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::Throw,
            ),
            (
                ASTNode::Return {
                    value: None,
                    span: Span::unknown(),
                },
                Admission::DirectPortAwareExpression,
                Kind::NonfinalReturn,
            ),
        ];
        let mut registry = ScriptDeferredResidualRegistryBuilderV1::new();
        let statement_count = cases.len() + 1;

        for (index, (statement, admission, _)) in cases.iter().enumerate() {
            let decision = ScriptRootSemanticDecisionV1::decide(
                index,
                statement_count,
                statement,
                Some(*admission),
                false,
            )
            .expect("residual decision");
            registry.record(index, statement, decision);
        }

        let entries = registry.seal();
        assert_eq!(
            entries
                .entries()
                .iter()
                .map(|entry| entry.kind())
                .collect::<Vec<_>>(),
            cases
                .into_iter()
                .map(|(_, _, kind)| kind)
                .collect::<Vec<_>>(),
        );
    }
}
