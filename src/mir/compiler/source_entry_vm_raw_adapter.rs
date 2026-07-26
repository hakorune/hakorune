//! Raw publication adapter for the source-family-neutral VM-reference owner.
//!
//! This module consumes existing Raw publication evidence once. It does not
//! execute MIR or own process-exit policy.

use super::raw_root_publication::RawPublishedInvocationV1;
use super::source_entry_published_invocation::{
    PendingPublishedSourceEntryTargetV1, PublishedSourceEntryInvocationV1,
    PublishedSourceEntryMembershipV1, PublishedSourceEntryResultContractV1,
    PublishedSourceEntryTargetErrorV1, PublishedUnitPhysicalContractV1,
};
use super::source_entry_selection::SelectedSourceEntryRouteV1;
use super::source_entry_vm_invocation::PreparedVmReferenceSourceEntryInvocationV1;
use super::source_entry_vm_invocation::VmReferenceExecutablePublishedOwnerV1;
use super::source_entry_vm_reference::{VmReferencePublishedOwnerV1, VmSourceEntryDecodePlanV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawPublishedVmAdapterStageV1 {
    Membership,
    Route,
    Target,
    ResultContract,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawPublishedVmAdapterErrorV1 {
    BrandMismatch,
    RouteMismatch,
    EntryTargetMismatch,
    Target(PublishedSourceEntryTargetErrorV1),
    DecodePlanUnavailable,
    DecodeRoundTripMismatch,
}

#[derive(Debug)]
pub(in crate::mir) struct RejectedRawPublishedVmAdapterV1 {
    owner: RawPublishedInvocationV1,
    stage: RawPublishedVmAdapterStageV1,
    error: RawPublishedVmAdapterErrorV1,
}

impl RejectedRawPublishedVmAdapterV1 {
    pub(in crate::mir) const fn stage(&self) -> RawPublishedVmAdapterStageV1 {
        self.stage
    }

    pub(in crate::mir) fn error(&self) -> &RawPublishedVmAdapterErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {
        drop(self);
    }

    pub(in crate::mir) fn into_public_string(self) -> String {
        let stage = match self.stage {
            RawPublishedVmAdapterStageV1::Membership => "membership",
            RawPublishedVmAdapterStageV1::Route => "route",
            RawPublishedVmAdapterStageV1::Target => "target",
            RawPublishedVmAdapterStageV1::ResultContract => "result-contract",
        };
        let detail = format!("{:?}", self.error);
        self.discard();
        format!("[raw-vm-reference/{stage}/rejected] {detail}")
    }
}

impl VmReferenceExecutablePublishedOwnerV1 for RawPublishedInvocationV1 {
    fn execute_exact_vm_entry(
        &self,
        symbol: &str,
    ) -> Result<crate::backend::vm_types::VMValue, crate::backend::vm_types::VMError> {
        RawPublishedInvocationV1::execute_exact_vm_entry(self, symbol)
    }
}

impl From<RawPublishedInvocationV1> for VmReferencePublishedOwnerV1 {
    fn from(owner: RawPublishedInvocationV1) -> Self {
        Self::Raw(owner)
    }
}

impl RawPublishedInvocationV1 {
    pub(in crate::mir) fn prepare_neutral_vm_reference(
        self,
    ) -> Result<PreparedVmReferenceSourceEntryInvocationV1<Self>, RejectedRawPublishedVmAdapterV1>
    {
        if self.invocation_brand() != self.selected_entry().brand() {
            return Err(reject(
                self,
                RawPublishedVmAdapterStageV1::Membership,
                RawPublishedVmAdapterErrorV1::BrandMismatch,
            ));
        }
        if !route_matches(&self) {
            return Err(reject(
                self,
                RawPublishedVmAdapterStageV1::Route,
                RawPublishedVmAdapterErrorV1::RouteMismatch,
            ));
        }
        if !self.main_entry_target_matches()
            || !self.selected_entry().is_main_target()
            || self.selected_entry().arity() != 0
        {
            return Err(reject(
                self,
                RawPublishedVmAdapterStageV1::Target,
                RawPublishedVmAdapterErrorV1::EntryTargetMismatch,
            ));
        }
        let target = match PendingPublishedSourceEntryTargetV1::new(
            self.selected_entry().symbol(),
            self.selected_entry().arity(),
        )
        .seal()
        {
            Ok(target) => target,
            Err(rejected) => {
                let error = rejected.error().clone();
                rejected.discard();
                return Err(reject(
                    self,
                    RawPublishedVmAdapterStageV1::Target,
                    RawPublishedVmAdapterErrorV1::Target(error),
                ));
            }
        };
        let decode = match self.vm_decode_plan() {
            Ok(decode) => decode,
            Err(()) => {
                return Err(reject(
                    self,
                    RawPublishedVmAdapterStageV1::ResultContract,
                    RawPublishedVmAdapterErrorV1::DecodePlanUnavailable,
                ))
            }
        };
        let result = result_from_decode(decode);
        let membership = PublishedSourceEntryMembershipV1::Raw {
            brand: self.invocation_brand(),
        };
        let prepared =
            PublishedSourceEntryInvocationV1::from_verified_parts(self, target, result, membership)
                .prepare_vm_reference();
        if prepared.decode_plan() != decode {
            let (published, _) = prepared.into_parts();
            let (owner, _, _, _) = published.into_parts();
            return Err(reject(
                owner,
                RawPublishedVmAdapterStageV1::ResultContract,
                RawPublishedVmAdapterErrorV1::DecodeRoundTripMismatch,
            ));
        }
        Ok(prepared)
    }
}

fn route_matches(owner: &RawPublishedInvocationV1) -> bool {
    matches!(
        (owner, owner.selected_entry().route()),
        (
            RawPublishedInvocationV1::Script(_),
            SelectedSourceEntryRouteV1::Script
        ) | (
            RawPublishedInvocationV1::App(_),
            SelectedSourceEntryRouteV1::AppMain0
        )
    )
}

fn result_from_decode(plan: VmSourceEntryDecodePlanV1) -> PublishedSourceEntryResultContractV1 {
    match plan {
        VmSourceEntryDecodePlanV1::Unit {
            origin,
            requires_void,
        } => PublishedSourceEntryResultContractV1::Unit {
            origin,
            physical: if requires_void {
                PublishedUnitPhysicalContractV1::ExactVoid
            } else {
                PublishedUnitPhysicalContractV1::CompatiblePayload
            },
        },
        VmSourceEntryDecodePlanV1::Integer => PublishedSourceEntryResultContractV1::Integer,
        VmSourceEntryDecodePlanV1::Bool => PublishedSourceEntryResultContractV1::Bool,
        VmSourceEntryDecodePlanV1::Float => PublishedSourceEntryResultContractV1::Float,
        VmSourceEntryDecodePlanV1::String => PublishedSourceEntryResultContractV1::String,
    }
}

fn reject(
    owner: RawPublishedInvocationV1,
    stage: RawPublishedVmAdapterStageV1,
    error: RawPublishedVmAdapterErrorV1,
) -> RejectedRawPublishedVmAdapterV1 {
    RejectedRawPublishedVmAdapterV1 {
        owner,
        stage,
        error,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::RawPublishedCompileRequestV1;

    fn script(value: LiteralValue) -> ASTNode {
        ASTNode::Program {
            statements: vec![ASTNode::Literal {
                value,
                span: Span::unknown(),
            }],
            span: Span::unknown(),
        }
    }

    #[test]
    fn raw_publication_projects_exact_target_and_decode_without_execution() {
        for (value, expected) in [
            (LiteralValue::Integer(7), VmSourceEntryDecodePlanV1::Integer),
            (LiteralValue::Bool(true), VmSourceEntryDecodePlanV1::Bool),
            (LiteralValue::Float(1.5), VmSourceEntryDecodePlanV1::Float),
            (
                LiteralValue::String("raw".to_owned()),
                VmSourceEntryDecodePlanV1::String,
            ),
        ] {
            let mut compiler = super::super::MirCompiler::new();
            let published = compiler
                .compile_raw_published_v1(RawPublishedCompileRequestV1::narrow_v1(
                    script(value),
                    Some("raw-neutral-adapter.hako"),
                ))
                .expect("Raw publication");
            let prepared = published
                .prepare_neutral_vm_reference()
                .expect("exact neutral projection");
            assert_eq!(prepared.decode_plan(), expected);
        }
    }
}
