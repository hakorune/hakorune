//! Generated CoreMethod target/Home capability for the bounded Loop cohort.
//!
//! The generated manifest row is the semantic source. This module only adds
//! explicit StringBox/Text and conditional Named Array/Text Home contracts
//! through one target issuer;
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
    ArrayTextAppend,
    ArrayIntegerAppend,
    ArrayDynamicRead,
    MapTextPresence,
    MapTextKeys,
    ArrayCount,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeReceiverRelationV1 {
    StringBoxReceiver,
    NamedArrayReceiver,
    MapReceiver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeParameterRelationV1 {
    I64Parameter,
    /// Borrowed text argument passed to the callee — the receiver does
    /// not retain it (distinct from `TextRetainedByReceiver`).
    TextParameter,
    TextRetainedByReceiver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeResultRelationV1 {
    I64ToCaller,
    BoolToCaller,
    TextToCaller,
    DynamicToCaller,
    /// A fresh plain Text Array. This is not a nominal object's identity.
    TextArrayToCaller,
    NoValue,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CoreMethodHomeAbiProfileV1 {
    StringBoxTextV1,
    NamedArrayTextV1,
    NamedArrayIntegerV1,
    NamedArrayReadV1,
    MapTextPresenceV1,
    MapTextKeysV1,
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
        Self::with_schema(manifest_brand, CoreMethodHomeSchemaV1::StringBoxText)
    }

    pub(crate) fn array_text_append(
        manifest_brand: CoreMethodManifestBrandV2,
    ) -> Result<Self, CoreMethodInstanceTargetRejectV1> {
        Self::with_schema(manifest_brand, CoreMethodHomeSchemaV1::ArrayTextAppend)
    }

    pub(crate) fn array_integer_append(
        manifest_brand: CoreMethodManifestBrandV2,
    ) -> Result<Self, CoreMethodInstanceTargetRejectV1> {
        Self::with_schema(manifest_brand, CoreMethodHomeSchemaV1::ArrayIntegerAppend)
    }

    pub(crate) fn array_dynamic_read(
        manifest_brand: CoreMethodManifestBrandV2,
    ) -> Result<Self, CoreMethodInstanceTargetRejectV1> {
        Self::with_schema(manifest_brand, CoreMethodHomeSchemaV1::ArrayDynamicRead)
    }

    pub(crate) fn map_text_presence(manifest_brand: CoreMethodManifestBrandV2)
        -> Result<Self, CoreMethodInstanceTargetRejectV1> {
        Self::with_schema(manifest_brand, CoreMethodHomeSchemaV1::MapTextPresence)
    }

    pub(crate) fn map_text_keys(manifest_brand: CoreMethodManifestBrandV2)
        -> Result<Self, CoreMethodInstanceTargetRejectV1> {
        Self::with_schema(manifest_brand, CoreMethodHomeSchemaV1::MapTextKeys)
    }
    pub(crate) fn array_count(manifest_brand: CoreMethodManifestBrandV2)
        -> Result<Self, CoreMethodInstanceTargetRejectV1> {
        Self::with_schema(manifest_brand, CoreMethodHomeSchemaV1::ArrayCount)
    }

    fn with_schema(
        manifest_brand: CoreMethodManifestBrandV2,
        schema: CoreMethodHomeSchemaV1,
    ) -> Result<Self, CoreMethodInstanceTargetRejectV1> {
        if manifest_brand != CORE_METHOD_MANIFEST_BRAND_V2 {
            return Err(CoreMethodInstanceTargetRejectV1::ManifestBrandMismatch);
        }
        let relation_brand = HomeRelationBrandIssuerV1::issue()
            .map_err(CoreMethodInstanceTargetRejectV1::RelationBrandIssue)?
            .brand();
        Ok(Self {
            manifest_brand,
            schema,
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
        let generated = row.row();
        let (receiver_name, effect, abi_profile) = match self.schema {
            CoreMethodHomeSchemaV1::MapTextKeys => (
                "MapBox", CoreMethodEffectV1::PureRead, CoreMethodHomeAbiProfileV1::MapTextKeysV1,
            ),
            CoreMethodHomeSchemaV1::ArrayCount => (
                "ArrayBox", CoreMethodEffectV1::PureRead, CoreMethodHomeAbiProfileV1::NamedArrayReadV1,
            ),
            CoreMethodHomeSchemaV1::MapTextPresence => (
                "MapBox", CoreMethodEffectV1::PureRead, CoreMethodHomeAbiProfileV1::MapTextPresenceV1,
            ),
            CoreMethodHomeSchemaV1::StringBoxText => (
                "StringBox",
                CoreMethodEffectV1::PureRead,
                CoreMethodHomeAbiProfileV1::StringBoxTextV1,
            ),
            CoreMethodHomeSchemaV1::ArrayTextAppend => (
                "ArrayBox",
                CoreMethodEffectV1::MutatesShape,
                CoreMethodHomeAbiProfileV1::NamedArrayTextV1,
            ),
            CoreMethodHomeSchemaV1::ArrayIntegerAppend => (
                "ArrayBox",
                CoreMethodEffectV1::MutatesShape,
                CoreMethodHomeAbiProfileV1::NamedArrayIntegerV1,
            ),
            CoreMethodHomeSchemaV1::ArrayDynamicRead => (
                "ArrayBox",
                CoreMethodEffectV1::PureRead,
                CoreMethodHomeAbiProfileV1::NamedArrayReadV1,
            ),
        };
        if generated.receiver_box != receiver_name {
            return Err(CoreMethodInstanceTargetRejectV1::ReceiverMismatch);
        }
        if generated.effect != effect {
            return Err(CoreMethodInstanceTargetRejectV1::EffectMismatch);
        }

        let (receiver, parameters, result) = match (generated.op, row.arity()) {
            (CoreMethodOp::MapKeys, 0) if self.schema == CoreMethodHomeSchemaV1::MapTextKeys => {
                if generated.result_kind != CoreMethodResultKindV1::Dynamic
                    || row.semantic_law_for_arity() != Some(crate::mir::core_method_result_kind::CoreMethodSemanticLawV2::MapTextKeysSnapshot) {
                    return Err(CoreMethodInstanceTargetRejectV1::ResultMismatch);
                }
                (CoreMethodHomeReceiverRelationV1::MapReceiver, Box::new([]) as Box<[_]>,
                    CoreMethodHomeResultRelationV1::TextArrayToCaller)
            },
            (CoreMethodOp::ArrayLen, 0) if self.schema == CoreMethodHomeSchemaV1::ArrayCount => {
                if generated.result_kind != CoreMethodResultKindV1::I64Value
                    || row.semantic_law_for_arity() != Some(crate::mir::core_method_result_kind::CoreMethodSemanticLawV2::ArrayCount) {
                    return Err(CoreMethodInstanceTargetRejectV1::ResultMismatch);
                }
                (CoreMethodHomeReceiverRelationV1::NamedArrayReceiver, Box::new([]) as Box<[_]>,
                    CoreMethodHomeResultRelationV1::I64ToCaller)
            },
            (CoreMethodOp::MapHas, 1) if self.schema == CoreMethodHomeSchemaV1::MapTextPresence => {
                if generated.result_kind != CoreMethodResultKindV1::BoolValue
                    || row.semantic_law_for_arity() != Some(crate::mir::core_method_result_kind::CoreMethodSemanticLawV2::MapKeyPresence) {
                    return Err(CoreMethodInstanceTargetRejectV1::ResultMismatch);
                }
                (CoreMethodHomeReceiverRelationV1::MapReceiver,
                    vec![CoreMethodHomeParameterRelationV1::TextParameter].into_boxed_slice(),
                    CoreMethodHomeResultRelationV1::BoolToCaller)
            },
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
            (CoreMethodOp::ArrayPush, 1) => {
                if generated.result_kind != CoreMethodResultKindV1::NoValue {
                    return Err(CoreMethodInstanceTargetRejectV1::ResultMismatch);
                }
                let parameter = match self.schema {
                    CoreMethodHomeSchemaV1::ArrayTextAppend => {
                        CoreMethodHomeParameterRelationV1::TextRetainedByReceiver
                    }
                    CoreMethodHomeSchemaV1::ArrayIntegerAppend => {
                        CoreMethodHomeParameterRelationV1::I64Parameter
                    }
                    CoreMethodHomeSchemaV1::MapTextPresence
                    | CoreMethodHomeSchemaV1::MapTextKeys
                    | CoreMethodHomeSchemaV1::ArrayCount
                    | CoreMethodHomeSchemaV1::ArrayDynamicRead
                    | CoreMethodHomeSchemaV1::StringBoxText => {
                        return Err(CoreMethodInstanceTargetRejectV1::UnsupportedOperation {
                            op: generated.op,
                            arity: row.arity(),
                        });
                    }
                };
                (
                    CoreMethodHomeReceiverRelationV1::NamedArrayReceiver,
                    vec![parameter].into_boxed_slice(),
                    CoreMethodHomeResultRelationV1::NoValue,
                )
            }
            (CoreMethodOp::ArrayGet, 1) => {
                if generated.result_kind != CoreMethodResultKindV1::Dynamic {
                    return Err(CoreMethodInstanceTargetRejectV1::ResultMismatch);
                }
                (
                    CoreMethodHomeReceiverRelationV1::NamedArrayReceiver,
                    vec![CoreMethodHomeParameterRelationV1::I64Parameter].into_boxed_slice(),
                    CoreMethodHomeResultRelationV1::DynamicToCaller,
                )
            }
            (CoreMethodOp::StringIndexOf, 1) => {
                if generated.result_kind != CoreMethodResultKindV1::I64Value {
                    return Err(CoreMethodInstanceTargetRejectV1::ResultMismatch);
                }
                (
                    CoreMethodHomeReceiverRelationV1::StringBoxReceiver,
                    vec![CoreMethodHomeParameterRelationV1::TextParameter].into_boxed_slice(),
                    CoreMethodHomeResultRelationV1::I64ToCaller,
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
            abi_profile,
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

    /// One primitive result projection; ownership is a separate contract.
    pub(crate) fn expression_result_class_v1(&self)
        -> Option<crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1> {
        use crate::mir::resolved_value_profile::expression_class::ExpressionValueClassV1 as Class;
        match self.result {
            CoreMethodHomeResultRelationV1::I64ToCaller => Some(Class::I64),
            CoreMethodHomeResultRelationV1::TextToCaller => Some(Class::String),
            CoreMethodHomeResultRelationV1::DynamicToCaller
                if self.schema == CoreMethodHomeSchemaV1::ArrayDynamicRead
                    && self.row.row().op == crate::mir::core_method_op::CoreMethodOp::ArrayGet => Some(Class::Dynamic),
            CoreMethodHomeResultRelationV1::BoolToCaller => Some(Class::Bool),
            _ => None,
        }
    }
    pub(crate) fn checked_text_keys_law_v1(&self) -> bool {
        self.schema == CoreMethodHomeSchemaV1::MapTextKeys
            && self.row.row().op == CoreMethodOp::MapKeys && self.row.arity() == 0
            && self.row.semantic_law_for_arity() == Some(crate::mir::core_method_result_kind::CoreMethodSemanticLawV2::MapTextKeysSnapshot)
            && self.result == CoreMethodHomeResultRelationV1::TextArrayToCaller
            && self.parameters.is_empty()
    }
    pub(crate) fn checked_array_count_law_v1(&self) -> bool {
        self.schema == CoreMethodHomeSchemaV1::ArrayCount
            && self.row.row().op == CoreMethodOp::ArrayLen && self.row.arity() == 0
            && self.row.semantic_law_for_arity() == Some(crate::mir::core_method_result_kind::CoreMethodSemanticLawV2::ArrayCount)
            && self.result == CoreMethodHomeResultRelationV1::I64ToCaller
            && self.parameters.is_empty()
    }
    pub(crate) fn checked_map_presence_law_v1(&self) -> bool {
        self.schema == CoreMethodHomeSchemaV1::MapTextPresence
            && self.receiver == CoreMethodHomeReceiverRelationV1::MapReceiver
            && self.abi_profile == CoreMethodHomeAbiProfileV1::MapTextPresenceV1
            && self.row.row().op == CoreMethodOp::MapHas && self.row.arity() == 1
            && self.row.row().effect == CoreMethodEffectV1::PureRead
            && self.row.row().result_kind == CoreMethodResultKindV1::BoolValue
            && self.result == CoreMethodHomeResultRelationV1::BoolToCaller
            && self.parameters() == [CoreMethodHomeParameterRelationV1::TextParameter]
            && self.row.semantic_law_for_arity() == Some(crate::mir::core_method_result_kind::CoreMethodSemanticLawV2::MapKeyPresence)
    }

    /// Exact generated semantics for the selected checked String operations.
    /// ABI family and non-suspension alone cannot establish an indexing law.
    pub(crate) fn checked_code_point_law_v1(&self) -> Option<crate::mir::core_method_result_kind::CoreMethodSemanticLawV2> {
        use crate::mir::core_method_result_kind::CoreMethodSemanticLawV2 as Law;
        if self.schema != CoreMethodHomeSchemaV1::StringBoxText
            || self.receiver != CoreMethodHomeReceiverRelationV1::StringBoxReceiver
            || self.abi_profile != CoreMethodHomeAbiProfileV1::StringBoxTextV1
            || self.execution_policy != CoreMethodHomeExecutionPolicyV1::NonSuspendingNonControl {
            return None;
        }
        let expected = match (self.row.row().op, self.row.arity(), self.parameters(), self.result) {
            (CoreMethodOp::StringLen, 0, [], CoreMethodHomeResultRelationV1::I64ToCaller) => Law::CodePointCount,
            (CoreMethodOp::StringSubstring, 2,
                [CoreMethodHomeParameterRelationV1::I64Parameter, CoreMethodHomeParameterRelationV1::I64Parameter],
                CoreMethodHomeResultRelationV1::TextToCaller) => Law::CodePointHalfOpenClamped,
            (CoreMethodOp::StringIndexOf, 1, [CoreMethodHomeParameterRelationV1::TextParameter],
                CoreMethodHomeResultRelationV1::I64ToCaller) => Law::CodePointSearchFromZero,
            _ => return None,
        };
        (self.row.semantic_law_for_arity() == Some(expected)).then_some(expected)
    }
}

#[cfg(test)]
#[path = "core_method_instance_target_tests.rs"]
mod tests;
