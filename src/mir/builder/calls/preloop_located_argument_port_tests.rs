//! Capability-boundary tests for the disconnected pre-loop candidate Port.

use crate::mir::builder::me_call_header_observation::MethodCallLoweringPortV1;
use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;

use super::PreloopLocatedArgumentPortV1;

fn assert_method_call_lowering_port<Port: MethodCallLoweringPortV1>() {}

#[test]
fn candidate_port_preserves_the_existing_method_call_capability_bundle() {
    assert_method_call_lowering_port::<
        PreloopLocatedArgumentPortV1<'static, 'static, 'static, RawLegacyChildLoweringPortV1>,
    >();
}
