//! Function roles and result contracts of the issued physical program.
use super::*;

/// One exact selected function in the physical lifecycle program.
#[derive(Debug, Clone)]
pub(crate) enum PublishedLifecyclePhysicalFunctionRoleV1 {
    Root {
        result: CompiledEntryRootResultV1,
    },
    BirthUnit {
        abi: crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1,
    },
    OrdinaryUnit {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    OrdinaryI64 {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    /// Exact owned String pair result; source permission is checked separately.
    OrdinaryText {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    /// Owned String pair or the null sentinel pair (0, 0); the caller publishes it
    /// as one opaque Dynamic carrier. Source permission is checked separately.
    OrdinaryNullableText {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    /// Exact Bool result, independent of integer machine storage.
    OrdinaryBool {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    /// Caller-owned opaque Map storage result; never an i64 handle.
    OrdinaryMap {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
    },
    /// Caller-owned transferred object result (`return new` callee); the
    /// caller holds the handle and owes exactly one release.
    OrdinaryHandle {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
        returns: Box<[FinalizedObjectReturnOutcomeV1]>,
    },
    /// Caller-owned nullable object result (`NullableObject` claim): the
    /// wire value is a live handle or the null sentinel, and the caller
    /// owes a checked release, never an unconditional one.
    OrdinaryNullableHandle {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
        returns: Box<[FinalizedObjectReturnOutcomeV1]>,
    },
    /// Original checked Return rows, retained through the final consumer.
    OrdinaryMixedHandle {
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
        formal_ordinal: u32,
        returns: Box<[FinalizedObjectReturnOutcomeV1]>,
    },
    OrdinaryBorrowedNullableHandle {
        access: crate::mir::instruction::BorrowedResultAccessV1,
        read_result: Option<std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>>,
        key: hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
        receiver_object: Option<CanonicalObjectIdV1>,
        support: crate::mir::instruction::BorrowedResultSupportV1,
        returns: Box<[FinalizedObjectReturnOutcomeV1]>,
    },
}

impl PublishedLifecyclePhysicalFunctionRoleV1 {
    pub(crate) fn read_result(&self) -> Option<&std::sync::Arc<crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultDeclarationV1>> {
        match self { Self::OrdinaryBorrowedNullableHandle { read_result, .. } => read_result.as_ref(), _ => None }
    }

    pub(crate) fn read_result_matches_declaration_v1(&self) -> bool {
        use crate::mir::instruction::{BorrowedResultAccessV1 as Access, BorrowedResultSupportV1 as Support};
        match self {
            Self::OrdinaryBorrowedNullableHandle { read_result, support, access, .. } => match (access, read_result) {
                (Access::LegacyUnqualified, None) => true,
                (Access::ReadOnly, Some(read)) => match (support, read.support()) {
                    (Support::Formal(a), crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Formal { ordinal: b, .. }) => *a == b,
                    (Support::Receiver, crate::mir::callable_semantic_batch::ResolvedCallableBorrowedResultSupportV1::Receiver { .. }) => true,
                    _ => false,
                },
                _ => false,
            },
            _ => true,
        }
    }

    pub(crate) const fn wire_name(&self) -> &'static str {
        match self {
            Self::Root {
                result: CompiledEntryRootResultV1::I64,
                ..
            } => "root_i64",
            Self::Root {
                result: CompiledEntryRootResultV1::Unit,
                ..
            } => "root_unit",
            Self::BirthUnit { .. } => "birth_unit",
            Self::OrdinaryUnit { .. } => "ordinary_unit",
            Self::OrdinaryI64 { .. } => "ordinary_i64",
            Self::OrdinaryText { .. } => "ordinary_text",
            Self::OrdinaryNullableText { .. } => "ordinary_nullable_text",
            Self::OrdinaryBool { .. } => "ordinary_bool",
            Self::OrdinaryMap { .. } => "ordinary_map",
            Self::OrdinaryHandle { .. } => "ordinary_handle",
            Self::OrdinaryNullableHandle { .. } => "ordinary_nullable_handle",
            Self::OrdinaryMixedHandle { .. } => "ordinary_mixed_handle",
            Self::OrdinaryBorrowedNullableHandle { .. } => "ordinary_borrowed_nullable_handle",
        }
    }

    pub(crate) fn birth_target(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::BirthUnit { abi } => Some(abi.target()),
            Self::Root { .. }
            | Self::OrdinaryUnit { .. }
            | Self::OrdinaryText { .. }
            | Self::OrdinaryNullableText { .. }
            | Self::OrdinaryI64 { .. }
            | Self::OrdinaryBool { .. }
            | Self::OrdinaryMap { .. }
            | Self::OrdinaryHandle { .. }
            | Self::OrdinaryNullableHandle { .. }
            | Self::OrdinaryMixedHandle { .. }
            | Self::OrdinaryBorrowedNullableHandle { .. } => None,
        }
    }

    pub(crate) fn ordinary_target(
        &self,
    ) -> Option<&hakorune_mir_defs::CanonicalSameModuleCallableKeyV1> {
        match self {
            Self::OrdinaryUnit { key, .. }
            | Self::OrdinaryText { key, .. }
            | Self::OrdinaryNullableText { key, .. }
            | Self::OrdinaryI64 { key, .. }
            | Self::OrdinaryBool { key, .. }
            | Self::OrdinaryMap { key, .. }
            | Self::OrdinaryHandle { key, .. }
            | Self::OrdinaryNullableHandle { key, .. }
            | Self::OrdinaryMixedHandle { key, .. }
            | Self::OrdinaryBorrowedNullableHandle { key, .. } => Some(key),
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) const fn ordinary_result(&self) -> Option<InvokeCallResultKind> {
        match self {
            Self::OrdinaryUnit { .. } => Some(InvokeCallResultKind::Unit),
            Self::OrdinaryI64 { .. } => Some(InvokeCallResultKind::I64),
            Self::OrdinaryText { .. } => Some(InvokeCallResultKind::OwnedString),
            Self::OrdinaryNullableText { .. } => Some(InvokeCallResultKind::DynamicCarrier),
            Self::OrdinaryBool { .. } => Some(InvokeCallResultKind::Bool),
            Self::OrdinaryMap { .. } => Some(InvokeCallResultKind::Map),
            Self::OrdinaryHandle { .. } => Some(InvokeCallResultKind::Handle),
            Self::OrdinaryNullableHandle { .. } => Some(InvokeCallResultKind::NullableHandle),
            Self::OrdinaryMixedHandle { formal_ordinal, .. } => Some(InvokeCallResultKind::MixedHandle { formal_ordinal: *formal_ordinal }),
            Self::OrdinaryBorrowedNullableHandle { support, access, .. } => Some(InvokeCallResultKind::BorrowedNullableHandle { support: *support, access: *access }),
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) const fn receiver_object(&self) -> Option<CanonicalObjectIdV1> {
        match self {
            Self::OrdinaryUnit { receiver_object, .. }
            | Self::OrdinaryText { receiver_object, .. }
            | Self::OrdinaryNullableText { receiver_object, .. }
            | Self::OrdinaryI64 {
                receiver_object, ..
            }
            | Self::OrdinaryBool {
                receiver_object, ..
            }
            | Self::OrdinaryMap {
                receiver_object, ..
            }
            | Self::OrdinaryHandle {
                receiver_object, ..
            }
            | Self::OrdinaryNullableHandle {
                receiver_object, ..
            }
            | Self::OrdinaryMixedHandle { receiver_object, .. }
            | Self::OrdinaryBorrowedNullableHandle { receiver_object, .. } => *receiver_object,
            Self::Root { .. } | Self::BirthUnit { .. } => None,
        }
    }

    pub(crate) fn has_receiver(&self) -> bool {
        matches!(self, Self::BirthUnit { .. }) || self.ordinary_target().is_some_and(|key|
            key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod)
    }

    pub(crate) fn object_return_outcomes(&self) -> Option<&[FinalizedObjectReturnOutcomeV1]> {
        match self {
            Self::OrdinaryHandle { returns, .. }
            | Self::OrdinaryNullableHandle { returns, .. }
            | Self::OrdinaryMixedHandle { returns, .. }
            | Self::OrdinaryBorrowedNullableHandle { returns, .. } => Some(returns),
            _ => None,
        }
    }

    pub(crate) fn receiver_value(&self, params: &[ValueId]) -> Option<ValueId> {
        self.has_receiver()
            .then(|| params.first().copied())
            .flatten()
    }
}
