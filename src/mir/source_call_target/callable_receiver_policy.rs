//! Catalog namespace to source receiver-observation policy.

use crate::mir::builder::SameModuleCallableNamespaceV1;
use crate::mir::resolved_semantics::ReceiverPolicyV1;

/// Sole production projection from one verified callable namespace into the
/// source-neutral shadow receiver vocabulary.
///
/// This policy does not resolve a target, declare a lexical receiver, or
/// install a Builder/runtime singleton.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SameModuleCallableSourceReceiverPolicyV1 {
    StaticCurrentOwner,
    DeclaredInstance,
}

impl SameModuleCallableSourceReceiverPolicyV1 {
    pub(crate) const fn from_namespace(namespace: SameModuleCallableNamespaceV1) -> Self {
        match namespace {
            SameModuleCallableNamespaceV1::StaticBoxMethod => Self::StaticCurrentOwner,
            SameModuleCallableNamespaceV1::InstanceBoxMethod => Self::DeclaredInstance,
        }
    }

    pub(crate) const fn into_shadow_policy(self) -> ReceiverPolicyV1 {
        match self {
            Self::StaticCurrentOwner => ReceiverPolicyV1::StaticCurrentOwner,
            Self::DeclaredInstance => ReceiverPolicyV1::DeclaredInstance,
        }
    }
}
