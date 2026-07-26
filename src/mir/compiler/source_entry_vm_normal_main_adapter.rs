//! Canonical Main publication adapter for the neutral VM-reference owner.
//!
//! The adapter consumes one completed, explicitly published Main candidate.
//! It transports sealed membership, target, and result evidence only.

use super::source_entry_published_invocation::{
    PendingPublishedSourceEntryTargetV1, PublishedSourceEntryInvocationV1,
    PublishedSourceEntryMembershipV1, PublishedSourceEntryResultContractV1,
    PublishedSourceEntryTargetErrorV1, PublishedUnitPhysicalContractV1,
};
use super::source_entry_result::UnitOriginV1;
use super::source_entry_vm_invocation::{
    PreparedVmReferenceSourceEntryInvocationV1, VmReferenceExecutablePublishedOwnerV1,
};
use super::source_entry_vm_reference::VmReferencePublishedOwnerV1;
use crate::mir::builder::PublishedNormalMainInvocationV1;
use crate::mir::compiler::normal_source_plan::VerifiedNormalMainThunkResultV1;
use crate::mir::resolved_control_flow::FunctionUnitOriginV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum NormalMainPublishedVmAdapterStageV1 {
    Membership,
    Target,
    ResultContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum NormalMainPublishedVmAdapterErrorV1 {
    MembershipMismatch,
    EntryRelationMismatch,
    Target(PublishedSourceEntryTargetErrorV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedNormalMainPublishedVmAdapterV1 {
    owner: PublishedNormalMainInvocationV1,
    stage: NormalMainPublishedVmAdapterStageV1,
    error: NormalMainPublishedVmAdapterErrorV1,
}

impl RejectedNormalMainPublishedVmAdapterV1 {
    pub(in crate::mir) const fn stage(&self) -> NormalMainPublishedVmAdapterStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &NormalMainPublishedVmAdapterErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {
        drop(self);
    }
}

impl VmReferenceExecutablePublishedOwnerV1 for PublishedNormalMainInvocationV1 {
    fn execute_exact_vm_entry(
        &self,
        symbol: &str,
    ) -> Result<crate::backend::vm_types::VMValue, crate::backend::vm_types::VMError> {
        PublishedNormalMainInvocationV1::execute_exact_vm_entry(self, symbol)
    }
}

impl From<PublishedNormalMainInvocationV1> for VmReferencePublishedOwnerV1 {
    fn from(owner: PublishedNormalMainInvocationV1) -> Self {
        Self::CanonicalMain(owner)
    }
}

impl PublishedNormalMainInvocationV1 {
    pub(in crate::mir) fn prepare_neutral_vm_reference(
        self,
    ) -> Result<
        PreparedVmReferenceSourceEntryInvocationV1<Self>,
        RejectedNormalMainPublishedVmAdapterV1,
    > {
        if !self.has_exact_membership() || self.verification_count() != 2 {
            return Err(reject(
                self,
                NormalMainPublishedVmAdapterStageV1::Membership,
                NormalMainPublishedVmAdapterErrorV1::MembershipMismatch,
            ));
        }
        if self.source_owner() != self.entry_source_owner() || self.physical_arity() != 0 {
            return Err(reject(
                self,
                NormalMainPublishedVmAdapterStageV1::Target,
                NormalMainPublishedVmAdapterErrorV1::EntryRelationMismatch,
            ));
        }
        let target = match PendingPublishedSourceEntryTargetV1::new(
            self.physical_symbol(),
            self.physical_arity(),
        )
        .seal()
        {
            Ok(target) => target,
            Err(rejected) => {
                let error = rejected.error().clone();
                rejected.discard();
                return Err(reject(
                    self,
                    NormalMainPublishedVmAdapterStageV1::Target,
                    NormalMainPublishedVmAdapterErrorV1::Target(error),
                ));
            }
        };
        let result = project_result(self.result());
        let membership = PublishedSourceEntryMembershipV1::CanonicalMain {
            source_owner: self.source_owner(),
        };
        Ok(
            PublishedSourceEntryInvocationV1::from_verified_parts(self, target, result, membership)
                .prepare_vm_reference(),
        )
    }
}

fn project_result(result: VerifiedNormalMainThunkResultV1) -> PublishedSourceEntryResultContractV1 {
    match result {
        VerifiedNormalMainThunkResultV1::Unit { origin } => {
            PublishedSourceEntryResultContractV1::Unit {
                origin: project_unit_origin(origin),
                physical: PublishedUnitPhysicalContractV1::ExactVoid,
            }
        }
        VerifiedNormalMainThunkResultV1::Integer => PublishedSourceEntryResultContractV1::Integer,
        VerifiedNormalMainThunkResultV1::Bool => PublishedSourceEntryResultContractV1::Bool,
        VerifiedNormalMainThunkResultV1::Float => PublishedSourceEntryResultContractV1::Float,
    }
}

fn project_unit_origin(origin: FunctionUnitOriginV1) -> UnitOriginV1 {
    match origin {
        FunctionUnitOriginV1::EmptyBody => UnitOriginV1::EmptyBody,
        FunctionUnitOriginV1::ImplicitFallthrough => UnitOriginV1::ImplicitFallthrough,
        FunctionUnitOriginV1::ExplicitVoid => UnitOriginV1::ExplicitVoid,
        FunctionUnitOriginV1::ExplicitNull => UnitOriginV1::ExplicitNull,
        FunctionUnitOriginV1::BareReturn => UnitOriginV1::BareReturn,
    }
}

fn reject(
    owner: PublishedNormalMainInvocationV1,
    stage: NormalMainPublishedVmAdapterStageV1,
    error: NormalMainPublishedVmAdapterErrorV1,
) -> RejectedNormalMainPublishedVmAdapterV1 {
    RejectedNormalMainPublishedVmAdapterV1 {
        owner,
        stage,
        error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
    use crate::mir::builder::MirBuilder;
    use crate::mir::compiler::normal_source_plan::with_main_thunk_for_test;
    use crate::mir::compiler::source_entry_vm_reference::VmSourceEntryDecodePlanV1;
    use std::collections::HashMap;

    fn literal(value: LiteralValue) -> ASTNode {
        ASTNode::Literal {
            value,
            span: Span::unknown(),
        }
    }

    fn return_(value: Option<LiteralValue>) -> ASTNode {
        ASTNode::Return {
            value: value.map(literal).map(Box::new),
            span: Span::unknown(),
        }
    }

    fn return_expr(value: ASTNode) -> ASTNode {
        ASTNode::Return {
            value: Some(Box::new(value)),
            span: Span::unknown(),
        }
    }

    fn divide_by_zero() -> ASTNode {
        return_expr(ASTNode::BinaryOp {
            operator: BinaryOperator::Divide,
            left: Box::new(literal(LiteralValue::Integer(1))),
            right: Box::new(literal(LiteralValue::Integer(0))),
            span: Span::unknown(),
        })
    }

    fn main_program(body: Vec<ASTNode>) -> ASTNode {
        let main = ASTNode::FunctionDeclaration {
            name: "main".to_owned(),
            params: Vec::new(),
            param_decls: Vec::new(),
            return_type_name: None,
            body,
            uses: Vec::new(),
            contracts: Vec::new(),
            is_static: true,
            is_override: false,
            attrs: DeclarationAttrs::default(),
            span: Span::unknown(),
        };
        let mut methods = HashMap::new();
        methods.insert("main".to_owned(), main);
        ASTNode::Program {
            statements: vec![ASTNode::BoxDeclaration {
                name: "Main".to_owned(),
                fields: Vec::new(),
                field_decls: Vec::new(),
                public_fields: Vec::new(),
                private_fields: Vec::new(),
                methods,
                constructors: HashMap::new(),
                init_fields: Vec::new(),
                weak_fields: Vec::new(),
                delegates: Vec::new(),
                invariants: Vec::new(),
                transitions: Vec::new(),
                is_interface: false,
                is_sync: false,
                is_record: false,
                type_parameters: Vec::new(),
                extends: Vec::new(),
                implements: Vec::new(),
                is_static: true,
                static_init: None,
                attrs: DeclarationAttrs::default(),
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    #[test]
    fn canonical_main_publication_projects_exact_result_and_executes_neutral_vm() {
        let cases = [
            (
                Vec::new(),
                VmSourceEntryDecodePlanV1::Unit {
                    origin: UnitOriginV1::EmptyBody,
                    requires_void: true,
                },
            ),
            (
                vec![literal(LiteralValue::Integer(3))],
                VmSourceEntryDecodePlanV1::Unit {
                    origin: UnitOriginV1::ImplicitFallthrough,
                    requires_void: true,
                },
            ),
            (
                vec![return_(None)],
                VmSourceEntryDecodePlanV1::Unit {
                    origin: UnitOriginV1::BareReturn,
                    requires_void: true,
                },
            ),
            (
                vec![return_(Some(LiteralValue::Void))],
                VmSourceEntryDecodePlanV1::Unit {
                    origin: UnitOriginV1::ExplicitVoid,
                    requires_void: true,
                },
            ),
            (
                vec![return_(Some(LiteralValue::Null))],
                VmSourceEntryDecodePlanV1::Unit {
                    origin: UnitOriginV1::ExplicitNull,
                    requires_void: true,
                },
            ),
            (
                vec![return_(Some(LiteralValue::Integer(7)))],
                VmSourceEntryDecodePlanV1::Integer,
            ),
            (
                vec![return_(Some(LiteralValue::Bool(true)))],
                VmSourceEntryDecodePlanV1::Bool,
            ),
            (
                vec![return_(Some(LiteralValue::Float(1.5)))],
                VmSourceEntryDecodePlanV1::Float,
            ),
        ];
        for (body, expected) in cases {
            with_main_thunk_for_test(main_program(body), |thunk| {
                let completed = MirBuilder::new().complete_normal_main_candidate_for_test(thunk);
                let prepared = completed
                    .publish()
                    .prepare_neutral_vm_reference()
                    .expect("canonical Main neutral adapter");
                assert_eq!(prepared.decode_plan(), expected);
                let outcome = prepared.execute().complete_canonical_source_entry();
                assert_eq!(
                    outcome.route_for_test(),
                    crate::mir::compiler::source_entry_selection::SelectedSourceEntryRouteV1::AppMain0
                );
            });
        }
    }

    fn run_main(
        builder: &mut MirBuilder,
        body: Vec<ASTNode>,
    ) -> super::super::source_entry_vm_reference::RawVmReferenceRunReportV1 {
        with_main_thunk_for_test(main_program(body), |thunk| {
            builder
                .complete_normal_main_candidate_for_test(thunk)
                .publish()
                .prepare_neutral_vm_reference()
                .expect("canonical Main neutral adapter")
                .execute()
                .complete_canonical_source_entry()
                .into_run_report()
        })
    }

    #[test]
    fn canonical_main_process_matrix_uses_shared_status_and_diagnostic_authorities() {
        let mut builder = MirBuilder::new();
        let cases = [
            (Vec::new(), 0, None),
            (vec![literal(LiteralValue::Integer(3))], 0, None),
            (vec![return_(None)], 0, None),
            (vec![return_(Some(LiteralValue::Void))], 0, None),
            (vec![return_(Some(LiteralValue::Null))], 0, None),
            (vec![return_(Some(LiteralValue::Integer(0)))], 0, None),
            (vec![return_(Some(LiteralValue::Integer(255)))], 255, None),
            (
                vec![return_(Some(LiteralValue::Integer(-1)))],
                70,
                Some("[process/exit-code-out-of-range]"),
            ),
            (
                vec![return_(Some(LiteralValue::Integer(256)))],
                70,
                Some("[process/exit-code-out-of-range]"),
            ),
            (
                vec![return_(Some(LiteralValue::Bool(true)))],
                70,
                Some("[process/unsupported-result]"),
            ),
            (
                vec![return_(Some(LiteralValue::Float(1.5)))],
                70,
                Some("[process/unsupported-result]"),
            ),
            (vec![divide_by_zero()], 70, Some("[process/source-fault]")),
        ];
        for (body, status, tag) in cases {
            let report = run_main(&mut builder, body);
            assert_eq!(report.status_code(), status);
            assert_eq!(report.diagnostic_tag(), tag);
        }
    }

    #[test]
    fn canonical_main_builder_reuses_after_process_and_vm_faults() {
        let mut builder = MirBuilder::new();
        let sequence = [
            (vec![return_(Some(LiteralValue::Integer(7)))], 7),
            (vec![return_(Some(LiteralValue::Integer(256)))], 70),
            (Vec::new(), 0),
            (vec![return_(Some(LiteralValue::Bool(false)))], 70),
            (vec![return_(Some(LiteralValue::Integer(1)))], 1),
            (vec![divide_by_zero()], 70),
            (vec![return_(Some(LiteralValue::Integer(2)))], 2),
        ];
        for (body, status) in sequence {
            assert_eq!(run_main(&mut builder, body).status_code(), status);
        }
    }
}
