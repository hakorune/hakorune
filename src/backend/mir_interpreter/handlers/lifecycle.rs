use super::*;

impl MirInterpreter {
    pub(super) fn copy_owned(&mut self, dst: ValueId, src: ValueId) -> Result<(), VMError> {
        if self.reg_peek_raw(dst).is_some() {
            return Err(VMError::InvalidInstruction(format!(
                "[freeze:contract][vm/ownership:dst_already_defined] dst={dst}"
            )));
        }
        let owned = match self.reg_peek_resolved(src) {
            Some(VMValue::BoxRef(value)) => VMValue::BoxRef(value.clone()),
            Some(other) => {
                return Err(VMError::TypeError(format!(
                    "CopyOwned expects BoxRef at {src}, got {other:?}"
                )))
            }
            None => {
                return Err(VMError::InvalidValue(format!(
                    "CopyOwned source is undefined: {src}"
                )))
            }
        };
        self.write_reg(dst, owned);
        Ok(())
    }

    pub(super) fn destroy_owned(&mut self, value: ValueId) -> Result<(), VMError> {
        match self.reg_peek_raw(value) {
            Some(VMValue::BoxRef(_)) => {
                let _ = self.take_reg(value);
                Ok(())
            }
            Some(other) => Err(VMError::TypeError(format!(
                "DestroyOwned expects BoxRef at {value}, got {other:?}"
            ))),
            None => Err(VMError::InvalidValue(format!(
                "DestroyOwned value is undefined: {value}"
            ))),
        }
    }

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
