//! Owner-private corruption seam for independent retained-binding tests.
use super::*;
impl FinishedBindings {
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn remove_recorded_binding_for_test(
        &mut self,
        row: &(BasicBlockId, MirInstruction),
    ) {
        self.recorded.retain(|actual| actual != row);
    }
}
