//! Resolver-owned contract for one bounded Loop MethodCall.
//!
//! This is the first bridge between the resolver's exact source row, its
//! sealed Loop placement/frame, and an already-issued CoreMethod target. It
//! not select by spelling, issue a Recipe relation, or expose MIR identity.

use super::body_shape::{ResolvedMethodCallReceiverSourceV1, VerifiedResolvedMethodCallSourceV1};
use super::callable_source_ledger::{
    CallableSemanticSourceLedgerView, VerifiedCallableLoopMembershipV1,
};
use super::core_method_instance_target::{
    CoreMethodHomeAbiProfileV1, CoreMethodHomeExecutionPolicyV1, CoreMethodHomeReceiverRelationV1,
    CoreMethodHomeResultRelationV1, CoreMethodHomeSchemaV1, CoreMethodInstanceTargetRejectV1,
    VerifiedCoreMethodInstanceTargetV1,
};
use super::ids::FunctionOwnerIdV1;
use super::records::ResolvedLexicalRefV1;
use super::source_site::{SourceExprSiteV1, SourceStmtSiteV1};
use super::{
    LoopExecutionFrameKeyV1, ResolvedLoopPlacementV1, ResolvedLoopRegionLookupErrorV1,
    ResolvedMethodCallArgumentSourceV1,
};
use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{CoreMethodEffectV1, CORE_METHOD_MANIFEST_BRAND_V2};

/// Fail-closed boundary for the bounded resolver contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ResolverCoreMethodCallableContractRejectV1 {
    ForeignCallOwner,
    ForeignSourceRow,
    MissingSourceSite(SourceExprSiteV1),
    Loop(ResolvedLoopRegionLookupErrorV1),
    ForeignLoopMembership,
    PlacementMismatch {
        site: SourceExprSiteV1,
        expected: ResolvedLoopPlacementV1,
        actual: Option<ResolvedLoopPlacementV1>,
    },
    UnsupportedReceiver,
    ReceiverBindingMismatch(SourceExprSiteV1),
    ResultSiteMismatch,
    ArgumentCardinality {
        expected: u32,
        actual: usize,
    },
    ArgumentOrdinal {
        expected: u32,
        actual: u32,
    },
    Target(CoreMethodInstanceTargetRejectV1),
    TargetManifestBrandMismatch,
    TargetSchemaMismatch,
    TargetReceiverMismatch,
    TargetPolicyMismatch,
    TargetOperationMismatch {
        op: CoreMethodOp,
        arity: u32,
    },
    TargetPlacementMismatch {
        op: CoreMethodOp,
        expected: ResolvedLoopPlacementV1,
        actual: ResolvedLoopPlacementV1,
    },
    TargetArityMismatch {
        expected: u32,
        actual: u32,
    },
    TargetParameterMismatch,
    TargetEffectMismatch,
    TargetResultMismatch,
    SelectorMismatch(Box<str>),
}

/// One non-Clone source/frame/target contract for a selected resolver call.
#[derive(Debug)]
pub(crate) struct VerifiedResolverCoreMethodCallableContractV1 {
    owner: FunctionOwnerIdV1,
    call_site: SourceExprSiteV1,
    receiver_site: SourceExprSiteV1,
    receiver: ResolvedMethodCallReceiverSourceV1,
    arguments: Box<[ResolvedMethodCallArgumentSourceV1]>,
    result_site: SourceExprSiteV1,
    loop_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    placement: ResolvedLoopPlacementV1,
    target: VerifiedCoreMethodInstanceTargetV1,
}

impl VerifiedResolverCoreMethodCallableContractV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 {
        &self.call_site
    }

    pub(crate) fn receiver_site(&self) -> &SourceExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) const fn receiver(&self) -> ResolvedMethodCallReceiverSourceV1 {
        self.receiver
    }

    pub(crate) fn arguments(&self) -> &[ResolvedMethodCallArgumentSourceV1] {
        &self.arguments
    }

    pub(crate) fn result_site(&self) -> &SourceExprSiteV1 {
        &self.result_site
    }

    pub(crate) fn loop_site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) const fn placement(&self) -> ResolvedLoopPlacementV1 {
        self.placement
    }

    pub(crate) fn target(&self) -> &VerifiedCoreMethodInstanceTargetV1 {
        &self.target
    }
}

/// Issues one exact source/frame/target contract without reselecting meaning.
pub(crate) struct ResolverCoreMethodCallableContractIssuerV1;

impl ResolverCoreMethodCallableContractIssuerV1 {
    pub(crate) fn issue(
        ledger: &CallableSemanticSourceLedgerView<'_>,
        call: &VerifiedResolvedMethodCallSourceV1,
        membership: &VerifiedCallableLoopMembershipV1,
        placement: ResolvedLoopPlacementV1,
        target: VerifiedCoreMethodInstanceTargetV1,
    ) -> Result<
        VerifiedResolverCoreMethodCallableContractV1,
        ResolverCoreMethodCallableContractRejectV1,
    > {
        if call.owner() != ledger.owner() {
            return Err(ResolverCoreMethodCallableContractRejectV1::ForeignCallOwner);
        }
        if membership.source().function_origin() != ledger.function_origin()
            || membership.source().source_kind() != ledger.source_kind()
        {
            return Err(ResolverCoreMethodCallableContractRejectV1::ForeignLoopMembership);
        }
        let exact_row = ledger
            .method_calls()
            .any(|(site, row)| site == call.site() && std::ptr::eq(row, call));
        if !exact_row {
            return Err(ResolverCoreMethodCallableContractRejectV1::ForeignSourceRow);
        }

        let source_sites = ledger.source_site_inventory();
        for site in std::iter::once(call.site())
            .chain(std::iter::once(call.receiver_site()))
            .chain(call.arguments().iter().map(|argument| argument.site()))
            .chain(std::iter::once(call.result_site()))
        {
            if !source_sites.contains_expression(site) {
                return Err(
                    ResolverCoreMethodCallableContractRejectV1::MissingSourceSite(site.clone()),
                );
            }
        }
        if call.result_site() != call.site() {
            return Err(ResolverCoreMethodCallableContractRejectV1::ResultSiteMismatch);
        }

        let receiver = match call.receiver() {
            ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding))
                if binding.owner() == ledger.owner()
                    && ledger.variable_ref(call.receiver_site())
                        == Some(ResolvedLexicalRefV1::Local(binding)) =>
            {
                call.receiver()
            }
            ResolvedMethodCallReceiverSourceV1::Lexical(_) => {
                return Err(
                    ResolverCoreMethodCallableContractRejectV1::ReceiverBindingMismatch(
                        call.receiver_site().clone(),
                    ),
                )
            }
            ResolvedMethodCallReceiverSourceV1::QualifiedUnbound
            | ResolvedMethodCallReceiverSourceV1::CurrentOwner
            | ResolvedMethodCallReceiverSourceV1::Other => {
                return Err(ResolverCoreMethodCallableContractRejectV1::UnsupportedReceiver)
            }
        };

        let expected_arity = target.row().arity();
        if call.arity() != expected_arity {
            return Err(
                ResolverCoreMethodCallableContractRejectV1::TargetArityMismatch {
                    expected: expected_arity,
                    actual: call.arity(),
                },
            );
        }
        if call.arguments().len() != expected_arity as usize {
            return Err(
                ResolverCoreMethodCallableContractRejectV1::ArgumentCardinality {
                    expected: expected_arity,
                    actual: call.arguments().len(),
                },
            );
        }
        for (expected, argument) in call.arguments().iter().enumerate() {
            if argument.ordinal() != expected as u32 {
                return Err(
                    ResolverCoreMethodCallableContractRejectV1::ArgumentOrdinal {
                        expected: expected as u32,
                        actual: argument.ordinal(),
                    },
                );
            }
        }

        let expected_placement = required_target_placement(&target)?;
        if placement != expected_placement {
            return Err(
                ResolverCoreMethodCallableContractRejectV1::TargetPlacementMismatch {
                    op: target.row().row().op,
                    expected: expected_placement,
                    actual: placement,
                },
            );
        }
        for site in std::iter::once(call.site())
            .chain(std::iter::once(call.receiver_site()))
            .chain(call.arguments().iter().map(|argument| argument.site()))
            .chain(std::iter::once(call.result_site()))
        {
            let actual = ledger
                .resolved_loop_placement(membership.source().site(), site)
                .map_err(ResolverCoreMethodCallableContractRejectV1::Loop)?;
            if actual != Some(placement) {
                return Err(
                    ResolverCoreMethodCallableContractRejectV1::PlacementMismatch {
                        site: site.clone(),
                        expected: placement,
                        actual,
                    },
                );
            }
        }

        verify_target(&target, call)?;

        Ok(VerifiedResolverCoreMethodCallableContractV1 {
            owner: ledger.owner(),
            call_site: call.site().clone(),
            receiver_site: call.receiver_site().clone(),
            receiver,
            arguments: call.arguments().to_vec().into_boxed_slice(),
            result_site: call.result_site().clone(),
            loop_site: membership.source().site().clone(),
            frame: membership.frame().clone(),
            placement,
            target,
        })
    }
}

fn required_target_placement(
    target: &VerifiedCoreMethodInstanceTargetV1,
) -> Result<ResolvedLoopPlacementV1, ResolverCoreMethodCallableContractRejectV1> {
    match (target.row().row().op, target.row().arity()) {
        (CoreMethodOp::StringLen, 0) => Ok(ResolvedLoopPlacementV1::Condition),
        (CoreMethodOp::StringSubstring, 2) => Ok(ResolvedLoopPlacementV1::Body),
        (op, arity) => {
            Err(ResolverCoreMethodCallableContractRejectV1::TargetOperationMismatch { op, arity })
        }
    }
}

fn verify_target(
    target: &VerifiedCoreMethodInstanceTargetV1,
    call: &VerifiedResolvedMethodCallSourceV1,
) -> Result<(), ResolverCoreMethodCallableContractRejectV1> {
    if target.manifest_brand() != CORE_METHOD_MANIFEST_BRAND_V2 {
        return Err(ResolverCoreMethodCallableContractRejectV1::TargetManifestBrandMismatch);
    }
    if target.schema() != CoreMethodHomeSchemaV1::StringBoxText {
        return Err(ResolverCoreMethodCallableContractRejectV1::TargetSchemaMismatch);
    }
    if target.receiver() != CoreMethodHomeReceiverRelationV1::StringBoxReceiver {
        return Err(ResolverCoreMethodCallableContractRejectV1::TargetReceiverMismatch);
    }
    if target.execution_policy() != CoreMethodHomeExecutionPolicyV1::NonSuspendingNonControl {
        return Err(ResolverCoreMethodCallableContractRejectV1::TargetPolicyMismatch);
    }
    let row = target.row().row();
    if row.effect != CoreMethodEffectV1::PureRead {
        return Err(ResolverCoreMethodCallableContractRejectV1::TargetEffectMismatch);
    }
    if row.receiver_box != "StringBox" {
        return Err(ResolverCoreMethodCallableContractRejectV1::TargetReceiverMismatch);
    }
    if row.canonical != call.selector() && !row.aliases.contains(&call.selector()) {
        return Err(
            ResolverCoreMethodCallableContractRejectV1::SelectorMismatch(call.selector().into()),
        );
    }
    match (row.op, target.row().arity(), target.result()) {
        (CoreMethodOp::StringLen, 0, CoreMethodHomeResultRelationV1::I64ToCaller)
        | (CoreMethodOp::StringSubstring, 2, CoreMethodHomeResultRelationV1::TextToCaller) => {}
        (op, arity, _) => {
            return Err(
                ResolverCoreMethodCallableContractRejectV1::TargetOperationMismatch { op, arity },
            )
        }
    }
    let expected_parameters = match row.op {
        CoreMethodOp::StringLen => 0,
        CoreMethodOp::StringSubstring => 2,
        op => {
            return Err(
                ResolverCoreMethodCallableContractRejectV1::TargetOperationMismatch {
                    op,
                    arity: target.row().arity(),
                },
            )
        }
    };
    if target.parameters().len() != expected_parameters {
        return Err(ResolverCoreMethodCallableContractRejectV1::TargetParameterMismatch);
    }
    if target.abi_profile() != CoreMethodHomeAbiProfileV1::StringBoxTextV1
        || target.target_brand().ordinal() == 0
    {
        return Err(ResolverCoreMethodCallableContractRejectV1::TargetSchemaMismatch);
    }
    Ok(())
}
