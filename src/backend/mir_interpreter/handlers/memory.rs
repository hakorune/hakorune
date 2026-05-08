use super::*;

impl MirInterpreter {
    pub(crate) fn handle_load(&mut self, dst: ValueId, ptr: ValueId) -> Result<(), VMError> {
        let v = self.mem.get(&ptr).cloned().unwrap_or(VMValue::Void);
        self.write_reg(dst, v);
        Ok(())
    }

    pub(crate) fn handle_static_data_load(
        &mut self,
        dst: ValueId,
        source_name: &str,
        symbol: &str,
        element: &str,
        len: u32,
        index: ValueId,
    ) -> Result<(), VMError> {
        if element != "u16" {
            return Err(self.err_invalid(format!(
                "[static-const/load-unsupported-element] {} element={}",
                source_name, element
            )));
        }
        let index_value = self.reg_load(index)?.as_integer()?;
        if index_value < 0 {
            return Err(self.err_invalid(format!(
                "[static-const/load-index-out-of-range] {} index={} len={}",
                source_name, index_value, len
            )));
        }
        let index_usize = index_value as usize;
        let plan = self
            .static_data_plans
            .iter()
            .find(|plan| plan.source_name == source_name && plan.symbol == symbol)
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[static-const/load-missing-plan] {} symbol={}",
                    source_name, symbol
                ))
            })?;
        if plan.element != element {
            return Err(self.err_invalid(format!(
                "[static-const/load-plan-mismatch] {} element={} plan_element={}",
                source_name, element, plan.element
            )));
        }
        if plan.values.len() != len as usize {
            return Err(self.err_invalid(format!(
                "[static-const/load-plan-mismatch] {} len={} plan_len={}",
                source_name,
                len,
                plan.values.len()
            )));
        }
        let value = plan.values.get(index_usize).ok_or_else(|| {
            self.err_invalid(format!(
                "[static-const/load-index-out-of-range] {} index={} len={}",
                source_name, index_value, len
            ))
        })?;
        self.write_reg(dst, VMValue::Integer(*value as i64));
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
