use crate::mir::builder::SameModuleCallableNamespaceV1;
use crate::mir::resolved_semantics::ReceiverPolicyV1;

use super::SameModuleCallableSourceReceiverPolicyV1;

#[test]
fn catalog_namespace_has_one_exact_source_receiver_projection() {
    assert_eq!(
        SameModuleCallableSourceReceiverPolicyV1::from_namespace(
            SameModuleCallableNamespaceV1::StaticBoxMethod,
        )
        .into_shadow_policy(),
        ReceiverPolicyV1::StaticCurrentOwner,
    );
    assert_eq!(
        SameModuleCallableSourceReceiverPolicyV1::from_namespace(
            SameModuleCallableNamespaceV1::InstanceBoxMethod,
        )
        .into_shadow_policy(),
        ReceiverPolicyV1::DeclaredInstance,
    );
}
