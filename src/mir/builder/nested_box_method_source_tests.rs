use super::*;

use crate::mir::builder::raw_invocation_source_transport::RawUnlocatedPortalV1;


#[test]
fn unlocated_parent_rejects_before_method_capture() {
    let (_, unlocated) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::unlocated((), RawUnlocatedPortalV1::CallObject),
    );
    assert!(
        PreparedNestedBoxMethodSourceV1::from_located_parent(Some(unlocated), "run".into())
            .is_err()
    );
}
