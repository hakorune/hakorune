use super::*;

impl MirInterpreter {
    pub(crate) fn handle_load(&mut self, dst: ValueId, ptr: ValueId) -> Result<(), VMError> {
        let v = self.mem.get(&ptr).cloned().unwrap_or(VMValue::Void);
        self.write_reg(dst, v);
        Ok(())
    }

    pub(crate) fn handle_store(&mut self, ptr: ValueId, value: ValueId) -> Result<(), VMError> {
        if let Some(v) = self.reg_peek_resolved(value) {
            self.mem.insert(ptr, v.clone());
            return Ok(());
        }
        let v = self.reg_load(value)?;
        self.mem.insert(ptr, v);
        Ok(())
    }

    pub(crate) fn handle_ref_new(&mut self, dst: ValueId, box_val: ValueId) -> Result<(), VMError> {
        if let Some(v) = self.reg_peek_resolved(box_val) {
            return match v {
                VMValue::BoxRef(_) => {
                    self.write_reg(dst, v.clone());
                    Ok(())
                }
                _ => Err(self.err_invalid("RefNew: target is not a Box".to_string())),
            };
        }
        let v = self.reg_load(box_val)?;
        match v {
            VMValue::BoxRef(_) => {
                self.write_reg(dst, v);
                Ok(())
            }
            _ => Err(self.err_invalid("RefNew: target is not a Box".to_string())),
        }
    }
}
