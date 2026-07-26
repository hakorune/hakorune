//! Passive VM-reference projection over a published source-entry invocation.
//!
//! This layer selects no entry and executes no MIR. It converts the sealed
//! backend-neutral result contract into the existing VM decode vocabulary.

use super::source_entry_published_invocation::{
    PublishedSourceEntryInvocationV1, PublishedSourceEntryResultContractV1,
    PublishedUnitPhysicalContractV1,
};
use super::source_entry_vm_reference::VmSourceEntryDecodePlanV1;

#[derive(Debug)]
pub(in crate::mir) struct PreparedVmReferenceSourceEntryInvocationV1<O> {
    published: PublishedSourceEntryInvocationV1<O>,
    decode: VmSourceEntryDecodePlanV1,
    _seal: PreparedVmReferenceSourceEntryInvocationSealV1,
}

#[derive(Debug)]
struct PreparedVmReferenceSourceEntryInvocationSealV1;

impl<O> PublishedSourceEntryInvocationV1<O> {
    pub(in crate::mir) fn prepare_vm_reference(
        self,
    ) -> PreparedVmReferenceSourceEntryInvocationV1<O> {
        let decode = match self.result() {
            PublishedSourceEntryResultContractV1::Unit { origin, physical } => {
                VmSourceEntryDecodePlanV1::Unit {
                    origin,
                    requires_void: matches!(physical, PublishedUnitPhysicalContractV1::ExactVoid),
                }
            }
            PublishedSourceEntryResultContractV1::Integer => VmSourceEntryDecodePlanV1::Integer,
            PublishedSourceEntryResultContractV1::Bool => VmSourceEntryDecodePlanV1::Bool,
            PublishedSourceEntryResultContractV1::Float => VmSourceEntryDecodePlanV1::Float,
            PublishedSourceEntryResultContractV1::String => VmSourceEntryDecodePlanV1::String,
        };
        PreparedVmReferenceSourceEntryInvocationV1 {
            published: self,
            decode,
            _seal: PreparedVmReferenceSourceEntryInvocationSealV1,
        }
    }
}

impl<O> PreparedVmReferenceSourceEntryInvocationV1<O> {
    pub(in crate::mir) const fn decode_plan(&self) -> VmSourceEntryDecodePlanV1 {
        self.decode
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        PublishedSourceEntryInvocationV1<O>,
        VmSourceEntryDecodePlanV1,
    ) {
        (self.published, self.decode)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::source_entry_published_invocation::{
        PendingPublishedSourceEntryTargetV1, PublishedSourceEntryMembershipV1,
    };
    use crate::mir::compiler::source_entry_result::UnitOriginV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    fn prepare(
        result: PublishedSourceEntryResultContractV1,
    ) -> PreparedVmReferenceSourceEntryInvocationV1<&'static str> {
        let target = PendingPublishedSourceEntryTargetV1::new("main", 0)
            .seal()
            .expect("exact target");
        PublishedSourceEntryInvocationV1::from_verified_parts(
            "owner",
            target,
            result,
            PublishedSourceEntryMembershipV1::CanonicalMain {
                source_owner: FunctionOwnerIssuerV1::new_for_compilation()
                    .expect("test owner issuer")
                    .issue()
                    .expect("test owner"),
            },
        )
        .prepare_vm_reference()
    }

    #[test]
    fn all_source_result_contracts_project_without_execution() {
        for origin in [
            UnitOriginV1::EmptyBody,
            UnitOriginV1::ImplicitFallthrough,
            UnitOriginV1::BareReturn,
            UnitOriginV1::ExplicitVoid,
            UnitOriginV1::ExplicitNull,
        ] {
            let prepared = prepare(PublishedSourceEntryResultContractV1::Unit {
                origin,
                physical: PublishedUnitPhysicalContractV1::ExactVoid,
            });
            assert_eq!(
                prepared.decode_plan(),
                VmSourceEntryDecodePlanV1::Unit {
                    origin,
                    requires_void: true,
                }
            );
        }

        let cases = [
            (
                PublishedSourceEntryResultContractV1::Unit {
                    origin: UnitOriginV1::PrintStatement,
                    physical: PublishedUnitPhysicalContractV1::CompatiblePayload,
                },
                VmSourceEntryDecodePlanV1::Unit {
                    origin: UnitOriginV1::PrintStatement,
                    requires_void: false,
                },
            ),
            (
                PublishedSourceEntryResultContractV1::Integer,
                VmSourceEntryDecodePlanV1::Integer,
            ),
            (
                PublishedSourceEntryResultContractV1::Bool,
                VmSourceEntryDecodePlanV1::Bool,
            ),
            (
                PublishedSourceEntryResultContractV1::Float,
                VmSourceEntryDecodePlanV1::Float,
            ),
            (
                PublishedSourceEntryResultContractV1::String,
                VmSourceEntryDecodePlanV1::String,
            ),
        ];
        for (result, expected) in cases {
            let prepared = prepare(result);
            assert_eq!(prepared.decode_plan(), expected);
            let (published, decode) = prepared.into_parts();
            assert_eq!(published.result(), result);
            assert_eq!(decode, expected);
        }
    }
}
