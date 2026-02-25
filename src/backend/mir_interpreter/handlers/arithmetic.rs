use super::*;

impl MirInterpreter {
    #[inline]
    fn try_eval_int_binop(op: BinaryOp, x: i64, y: i64) -> Result<Option<VMValue>, VMError> {
        use BinaryOp::*;
        let v = match op {
            Add => VMValue::Integer(x + y),
            Sub => VMValue::Integer(x - y),
            Mul => VMValue::Integer(x * y),
            Div => {
                if y == 0 {
                    return Err(VMError::DivisionByZero);
                }
                VMValue::Integer(x / y)
            }
            Mod => {
                if y == 0 {
                    return Err(VMError::DivisionByZero);
                }
                VMValue::Integer(x % y)
            }
            BitAnd => VMValue::Integer(x & y),
            BitOr => VMValue::Integer(x | y),
            BitXor => VMValue::Integer(x ^ y),
            Shl => VMValue::Integer(x.wrapping_shl(y as u32)),
            Shr => VMValue::Integer(x.wrapping_shr(y as u32)),
            And | Or => return Ok(None),
        };
        Ok(Some(v))
    }

    #[inline]
    fn try_eval_int_compare(op: CompareOp, x: i64, y: i64) -> bool {
        match op {
            CompareOp::Eq => x == y,
            CompareOp::Ne => x != y,
            CompareOp::Lt => x < y,
            CompareOp::Le => x <= y,
            CompareOp::Gt => x > y,
            CompareOp::Ge => x >= y,
        }
    }

    pub(crate) fn handle_const(&mut self, dst: ValueId, value: &ConstValue) -> Result<(), VMError> {
        let v = match value {
            ConstValue::Integer(i) => VMValue::Integer(*i),
            ConstValue::Float(f) => VMValue::Float(*f),
            ConstValue::Bool(b) => VMValue::Bool(*b),
            ConstValue::String(s) => VMValue::String(s.clone()),
            ConstValue::Null | ConstValue::Void => VMValue::Void,
        };
        match &v {
            VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst, *i),
            VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst, *b),
            _ => self.vm_fast_cache_clear(dst),
        }
        self.write_reg(dst, v);
        Ok(())
    }

    pub(crate) fn handle_binop(
        &mut self,
        dst: ValueId,
        op: BinaryOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<(), VMError> {
        // Operator Box (Add) — observe always; adopt gated
        if matches!(op, BinaryOp::Add)
            && self.operator_box_caps.add_apply
            && self.operator_box_add_adopt_enabled
        {
            let in_guard = self
                .cur_fn
                .as_deref()
                .map(|n| n.starts_with("AddOperator.apply/"))
                .unwrap_or(false);
            if !in_guard {
                let a = self.reg_load(lhs)?;
                let b = self.reg_load(rhs)?;
                if let Some(op_fn) = self.functions.get("AddOperator.apply/2").cloned() {
                    let out = self.exec_function_inner(&op_fn, Some(&[a.clone(), b.clone()]))?;
                    self.write_reg(dst, out);
                    return Ok(());
                }
            }
        }

        if let (Some(x), Some(y)) = (self.vm_fast_read_i64(lhs), self.vm_fast_read_i64(rhs)) {
            if let Some(v) = Self::try_eval_int_binop(op, x, y)? {
                if let VMValue::Integer(out) = &v {
                    self.vm_fast_cache_set_i64(dst, *out);
                } else {
                    self.vm_fast_cache_clear(dst);
                }
                self.write_reg(dst, v);
                return Ok(());
            }
        }

        let a = self.reg_load(lhs)?;
        let b = self.reg_load(rhs)?;
        let v = self.eval_binop(op, a, b)?;
        match &v {
            VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst, *i),
            VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst, *b),
            _ => self.vm_fast_cache_clear(dst),
        }
        self.write_reg(dst, v);
        Ok(())
    }

    pub(crate) fn handle_unary_op(
        &mut self,
        dst: ValueId,
        op: crate::mir::UnaryOp,
        operand: ValueId,
    ) -> Result<(), VMError> {
        let x = self.reg_load(operand)?;
        let v = match op {
            crate::mir::UnaryOp::Neg => match x {
                VMValue::Integer(i) => VMValue::Integer(-i),
                VMValue::Float(f) => VMValue::Float(-f),
                _ => {
                    return Err(VMError::TypeError(format!(
                        "neg expects number, got {:?}",
                        x
                    )))
                }
            },
            crate::mir::UnaryOp::Not => VMValue::Bool(!to_bool_vm(&x).map_err(VMError::TypeError)?),
            crate::mir::UnaryOp::BitNot => match x {
                VMValue::Integer(i) => VMValue::Integer(!i),
                _ => {
                    return Err(VMError::TypeError(format!(
                        "bitnot expects integer, got {:?}",
                        x
                    )))
                }
            },
        };
        match &v {
            VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst, *i),
            VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst, *b),
            _ => self.vm_fast_cache_clear(dst),
        }
        self.write_reg(dst, v);
        Ok(())
    }

    pub(crate) fn handle_compare(
        &mut self,
        dst: ValueId,
        op: CompareOp,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<(), VMError> {
        // Operator Box (Compare) — observe always; adopt gated
        if self.operator_box_caps.compare_apply && self.operator_box_compare_adopt_enabled {
            let in_guard = self
                .cur_fn
                .as_deref()
                .map(|n| n.starts_with("CompareOperator.apply/"))
                .unwrap_or(false);
            if !in_guard {
                let opname = match op {
                    CompareOp::Eq => "Eq",
                    CompareOp::Ne => "Ne",
                    CompareOp::Lt => "Lt",
                    CompareOp::Le => "Le",
                    CompareOp::Gt => "Gt",
                    CompareOp::Ge => "Ge",
                };
                let a = self.reg_load(lhs)?;
                let b = self.reg_load(rhs)?;
                if let Some(op_fn) = self.functions.get("CompareOperator.apply/3").cloned() {
                    let out = self.exec_function_inner(
                        &op_fn,
                        Some(&[VMValue::String(opname.to_string()), a.clone(), b.clone()]),
                    )?;
                    let res = match out {
                        VMValue::Bool(b) => b,
                        _ => self.eval_cmp(op, a.clone(), b.clone())?,
                    };
                    self.write_reg(dst, VMValue::Bool(res));
                    return Ok(());
                }
            }
        }

        if let (Some(x), Some(y)) = (self.vm_fast_read_i64(lhs), self.vm_fast_read_i64(rhs)) {
            let res = Self::try_eval_int_compare(op, x, y);
            self.vm_fast_cache_set_bool(dst, res);
            self.write_reg(dst, VMValue::Bool(res));
            return Ok(());
        }

        let a = self.reg_load(lhs)?;
        let b = self.reg_load(rhs)?;
        let res = self.eval_cmp(op, a.clone(), b.clone())?;
        if self.vm_provider_trace_enabled {
            if let CompareOp::Eq = op {
                let watch = |s: &str| {
                    matches!(
                        s,
                        "-42"
                            | "0123456789"
                            | "-"
                            | "0"
                            | "1"
                            | "2"
                            | "3"
                            | "4"
                            | "5"
                            | "6"
                            | "7"
                            | "8"
                            | "9"
                    )
                };
                match (&a, &b) {
                    (VMValue::String(xs), VMValue::String(ys)) if watch(xs) || watch(ys) => {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[provider/trace][cmp] {:?} == {:?} -> {}",
                            xs, ys, res
                        ));
                    }
                    (VMValue::BoxRef(_), VMValue::String(ys)) if watch(ys) => {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[provider/trace][cmp] BoxRef == {:?} -> {}",
                            ys, res
                        ));
                    }
                    (VMValue::String(xs), VMValue::BoxRef(_)) if watch(xs) => {
                        crate::runtime::get_global_ring0().log.debug(&format!(
                            "[provider/trace][cmp] {:?} == BoxRef -> {}",
                            xs, res
                        ));
                    }
                    _ => {}
                }
            }
        }
        self.vm_fast_cache_set_bool(dst, res);
        self.write_reg(dst, VMValue::Bool(res));
        Ok(())
    }

    pub(crate) fn handle_copy(&mut self, dst: ValueId, src: ValueId) -> Result<(), VMError> {
        if self.vm_fast_regfile_enabled {
            if let Some(v) = self.reg_peek_resolved(src).cloned() {
                self.write_reg(dst, v);
                return Ok(());
            }
            let v = self.reg_load(src)?;
            self.write_reg(dst, v);
            return Ok(());
        }

        if self.vm_fast_enabled {
            let resolved_src = self.resolve_copy_alias(src);
            if matches!(
                self.reg_peek_resolved(resolved_src),
                Some(VMValue::Integer(_))
            ) {
                self.reg_copy_aliases.insert(dst, resolved_src);
                return Ok(());
            }
        }

        if let Some(v) = self.reg_peek_resolved(src).cloned() {
            match &v {
                VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst, *i),
                VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst, *b),
                _ => self.vm_fast_cache_clear(dst),
            }
            self.write_reg(dst, v);
            return Ok(());
        }
        let v = self.reg_load(src)?;
        match &v {
            VMValue::Integer(i) => self.vm_fast_cache_set_i64(dst, *i),
            VMValue::Bool(b) => self.vm_fast_cache_set_bool(dst, *b),
            _ => self.vm_fast_cache_clear(dst),
        }
        self.write_reg(dst, v);
        Ok(())
    }
}
