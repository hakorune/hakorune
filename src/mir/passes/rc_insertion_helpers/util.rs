#[cfg(feature = "rc-insertion-minimal")]
use crate::mir::ValueId;

/// P7: ReleaseStrong の values を決定的順序（ValueId 昇順）にする
#[cfg(feature = "rc-insertion-minimal")]
pub(super) fn sorted_release_values(mut values: Vec<ValueId>) -> Vec<ValueId> {
    values.sort_unstable();
    values.dedup();
    values
}
