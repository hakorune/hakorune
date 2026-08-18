//! Generated CoreMethod target/Home capability for the bounded Loop cohort.
//!
//! The generated manifest row is the semantic source. This module only adds
//! the explicit StringBox/Text Home contract and a one-shot target issuer;
//! it does not inspect AST/MIR, source sites, Recipe keys, or physical IDs.

use std::collections::BTreeSet;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    CoreMethodEffectV1, CoreMethodManifestBrandV2, CoreMethodManifestRowRefV2,
    CoreMethodResultKindV1, CORE_METHOD_MANIFEST_BRAND_V2,
};

use super::home_relation::{HomeRelationBrandIssuerV1, HomeRelationBrandV1, HomeRelationRejectV1};

static NEXT_CORE_METHOD_TARGET_BRAND: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeSchemaV1 {
    StringBoxText,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeReceiverRelationV1 {
    StringBoxReceiver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeParameterRelationV1 {
    I64Parameter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeResultRelationV1 {
    I64ToCaller,
    TextToCaller,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeAbiProfileV1 {
    StringBoxTextV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeExecutionPolicyV1 {
    NonSuspendingNonControl,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct CoreMethodTargetBrandV1(u64);

impl CoreMethodTargetBrandV1 {
    pub(crate) const fn ordinal(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct CoreMethodTargetIdentityV1 {
    op: &'static str,
    arity: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CoreMethodInstanceTargetRejectV1 {
    ManifestBrandMismatch,
    DesignOnlyRow,
    UnsupportedHomeSchema,
    UnsupportedOperation { op: CoreMethodOp, arity: u32 },
    ReceiverMismatch,
    ResultMismatch,
    EffectMismatch,
    DuplicateTarget { op: CoreMethodOp, arity: u32 },
    RelationBrandIssue(HomeRelationRejectV1),
    TargetBrandExhausted,
}

#[derive(Debug)]
pub(crate) struct CoreMethodInstanceTargetIssuerV1 {
    manifest_brand: CoreMethodManifestBrandV2,
    schema: CoreMethodHomeSchemaV1,
    relation_brand: HomeRelationBrandV1,
    issued: BTreeSet<CoreMethodTargetIdentityV1>,
}

impl CoreMethodInstanceTargetIssuerV1 {
    pub(crate) fn string_box_text(
        manifest_brand: CoreMethodManifestBrandV2,
    ) -> Result<Self, CoreMethodInstanceTargetRejectV1> {
        if manifest_brand != CORE_METHOD_MANIFEST_BRAND_V2 {
            return Err(CoreMethodInstanceTargetRejectV1::ManifestBrandMismatch);
        }
        let relation_brand = HomeRelationBrandIssuerV1::issue()
            .map_err(CoreMethodInstanceTargetRejectV1::RelationBrandIssue)?
            .brand();
        Ok(Self {
            manifest_brand,
            schema: CoreMethodHomeSchemaV1::StringBoxText,
            relation_brand,
            issued: BTreeSet::new(),
        })
    }

    pub(crate) const fn manifest_brand(&self) -> CoreMethodManifestBrandV2 {
        self.manifest_brand
    }

    pub(crate) const fn schema(&self) -> CoreMethodHomeSchemaV1 {
        self.schema
    }

    pub(crate) const fn relation_brand(&self) -> HomeRelationBrandV1 {
        self.relation_brand
    }

    pub(crate) fn issue(
        &mut self,
        row: CoreMethodManifestRowRefV2,
    ) -> Result<VerifiedCoreMethodInstanceTargetV1, CoreMethodInstanceTargetRejectV1> {
        if row.brand() != self.manifest_brand {
            return Err(CoreMethodInstanceTargetRejectV1::ManifestBrandMismatch);
        }
        if row.lowering_tier().is_design_only() {
            return Err(CoreMethodInstanceTargetRejectV1::DesignOnlyRow);
        }
        if self.schema != CoreMethodHomeSchemaV1::StringBoxText {
            return Err(CoreMethodInstanceTargetRejectV1::UnsupportedHomeSchema);
        }

        let generated = row.row();
        if generated.receiver_box != "StringBox" {
            return Err(CoreMethodInstanceTargetRejectV1::ReceiverMismatch);
        }
        if generated.effect != CoreMethodEffectV1::PureRead {
            return Err(CoreMethodInstanceTargetRejectV1::EffectMismatch);
        }

        let (receiver, parameters, result) = match (generated.op, row.arity()) {
            (CoreMethodOp::StringLen, 0) => {
                if generated.result_kind != CoreMethodResultKindV1::I64Value {
                    return Err(CoreMethodInstanceTargetRejectV1::ResultMismatch);
                }
                (
                    CoreMethodHomeReceiverRelationV1::StringBoxReceiver,
                    Vec::new().into_boxed_slice(),
                    CoreMethodHomeResultRelationV1::I64ToCaller,
                )
            }
            (CoreMethodOp::StringSubstring, 2) => {
                if generated.result_kind != CoreMethodResultKindV1::StringValue {
                    return Err(CoreMethodInstanceTargetRejectV1::ResultMismatch);
                }
                (
                    CoreMethodHomeReceiverRelationV1::StringBoxReceiver,
                    vec![
                        CoreMethodHomeParameterRelationV1::I64Parameter,
                        CoreMethodHomeParameterRelationV1::I64Parameter,
                    ]
                    .into_boxed_slice(),
                    CoreMethodHomeResultRelationV1::TextToCaller,
                )
            }
            (op, arity) => {
                return Err(CoreMethodInstanceTargetRejectV1::UnsupportedOperation { op, arity });
            }
        };

        let identity = CoreMethodTargetIdentityV1 {
            op: generated.op.as_manifest_name(),
            arity: row.arity(),
        };
        if !self.issued.insert(identity) {
            return Err(CoreMethodInstanceTargetRejectV1::DuplicateTarget {
                op: generated.op,
                arity: identity.arity,
            });
        }

        let ordinal = NEXT_CORE_METHOD_TARGET_BRAND
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |current| {
                current.checked_add(1)
            })
            .map_err(|_| CoreMethodInstanceTargetRejectV1::TargetBrandExhausted)?;

        Ok(VerifiedCoreMethodInstanceTargetV1 {
            manifest_brand: self.manifest_brand,
            target_brand: CoreMethodTargetBrandV1(ordinal),
            relation_brand: self.relation_brand,
            schema: self.schema,
            row,
            receiver,
            parameters,
            result,
            abi_profile: CoreMethodHomeAbiProfileV1::StringBoxTextV1,
            execution_policy: CoreMethodHomeExecutionPolicyV1::NonSuspendingNonControl,
        })
    }
}

/// Move-only target capability consumed by the later source-bound relation.
#[derive(Debug)]
pub(crate) struct VerifiedCoreMethodInstanceTargetV1 {
    manifest_brand: CoreMethodManifestBrandV2,
    target_brand: CoreMethodTargetBrandV1,
    relation_brand: HomeRelationBrandV1,
    schema: CoreMethodHomeSchemaV1,
    row: CoreMethodManifestRowRefV2,
    receiver: CoreMethodHomeReceiverRelationV1,
    parameters: Box<[CoreMethodHomeParameterRelationV1]>,
    result: CoreMethodHomeResultRelationV1,
    abi_profile: CoreMethodHomeAbiProfileV1,
    execution_policy: CoreMethodHomeExecutionPolicyV1,
}

impl VerifiedCoreMethodInstanceTargetV1 {
    pub(crate) const fn manifest_brand(&self) -> CoreMethodManifestBrandV2 {
        self.manifest_brand
    }

    pub(crate) const fn target_brand(&self) -> CoreMethodTargetBrandV1 {
        self.target_brand
    }

    pub(crate) const fn relation_brand(&self) -> HomeRelationBrandV1 {
        self.relation_brand
    }

    pub(crate) const fn schema(&self) -> CoreMethodHomeSchemaV1 {
        self.schema
    }

    pub(crate) const fn row(&self) -> CoreMethodManifestRowRefV2 {
        self.row
    }

    pub(crate) const fn receiver(&self) -> CoreMethodHomeReceiverRelationV1 {
        self.receiver
    }

    pub(crate) fn parameters(&self) -> &[CoreMethodHomeParameterRelationV1] {
        &self.parameters
    }

    pub(crate) const fn result(&self) -> CoreMethodHomeResultRelationV1 {
        self.result
    }

    pub(crate) const fn abi_profile(&self) -> CoreMethodHomeAbiProfileV1 {
        self.abi_profile
    }

    pub(crate) const fn execution_policy(&self) -> CoreMethodHomeExecutionPolicyV1 {
        self.execution_policy
    }
}

#[cfg(test)]
#[path = "core_method_instance_target_tests.rs"]
mod tests;
