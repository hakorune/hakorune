use super::*;

impl MirInterpreter {
    /// Release the explicit strong-reference slots named by `ReleaseStrong`.
    ///
    /// Do not sweep every register that happens to point at the same `Arc`:
    /// SSA copies may also represent other live locals/params, and deleting
    /// those aliases creates use-after-release in later PHI inputs.
    pub(super) fn release_strong_refs(&mut self, values: &[ValueId]) {
        // Only BoxRef values participate in "strong ref" release.
        // Do not remove immediate values (Integer/Bool/String/etc): they have no strong refs,
        // and removing them can create use-after-release crashes.
        for value_id in values {
            if matches!(self.reg_peek_resolved(*value_id), Some(VMValue::BoxRef(_))) {
                let _ = self.take_reg(*value_id);
            }
        }
    }
}
