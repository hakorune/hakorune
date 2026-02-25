use super::*;
use crate::box_trait::VoidBox;
use std::string::String as StdString;

impl MirInterpreter {
    const VM_FAST_REGFILE_LIMIT: usize = 8192;

    #[inline]
    fn tag_nullish(v: &VMValue) -> &'static str {
        match v {
            VMValue::Void => "void",
            VMValue::BoxRef(b) => {
                if b.as_any()
                    .downcast_ref::<crate::boxes::null_box::NullBox>()
                    .is_some()
                {
                    "null"
                } else if b
                    .as_any()
                    .downcast_ref::<crate::boxes::missing_box::MissingBox>()
                    .is_some()
                {
                    "missing"
                } else if b
                    .as_any()
                    .downcast_ref::<crate::box_trait::VoidBox>()
                    .is_some()
                {
                    "void"
                } else {
                    ""
                }
            }
            _ => "",
        }
    }

    #[inline(always)]
    pub(super) fn resolve_copy_alias(&self, id: ValueId) -> ValueId {
        if self.vm_fast_regfile_enabled {
            return id;
        }
        let mut cur = id;
        let mut hops = 0usize;
        while hops < 16 {
            match self.reg_copy_aliases.get(&cur).copied() {
                Some(next) if next != cur => {
                    cur = next;
                    hops += 1;
                }
                _ => break,
            }
        }
        cur
    }

    #[inline(always)]
    fn vm_fast_reg_slot_index(&self, id: ValueId) -> Option<usize> {
        if !self.vm_fast_regfile_enabled {
            return None;
        }
        let idx = id.as_u32() as usize;
        if idx < Self::VM_FAST_REGFILE_LIMIT {
            Some(idx)
        } else {
            None
        }
    }

    #[inline(always)]
    fn vm_fast_reg_slots_ensure_len(&mut self, idx: usize) {
        if self.reg_fast_slots.len() <= idx {
            self.reg_fast_slots.resize(idx + 1, None);
        }
    }

    #[inline(always)]
    pub(super) fn prepare_fast_regfile_slots(&mut self, next_value_id: u32) {
        if !self.vm_fast_regfile_enabled {
            return;
        }
        let want = (next_value_id as usize).min(Self::VM_FAST_REGFILE_LIMIT);
        if self.reg_fast_slots.len() < want {
            self.reg_fast_slots.resize(want, None);
        }
    }

    #[inline(always)]
    fn reg_peek_raw(&self, id: ValueId) -> Option<&VMValue> {
        if let Some(idx) = self.vm_fast_reg_slot_index(id) {
            if idx < self.reg_fast_slots.len() {
                if let Some(v) = self.reg_fast_slots[idx].as_ref() {
                    return Some(v);
                }
            }
            // Fast-regfile invariant: in-range IDs are slot-only.
            debug_assert!(self.regs.get(&id).is_none());
            return None;
        }
        self.regs.get(&id)
    }

    #[inline(always)]
    pub(super) fn reg_peek_resolved(&self, id: ValueId) -> Option<&VMValue> {
        let resolved = self.resolve_copy_alias(id);
        self.reg_peek_raw(resolved)
    }

    #[inline(always)]
    pub(super) fn write_reg(&mut self, id: ValueId, value: VMValue) {
        if !self.vm_fast_regfile_enabled {
            // Overwrite invalidates copy-alias destination in map mode.
            self.reg_copy_aliases.remove(&id);
        }
        if let Some(idx) = self.vm_fast_reg_slot_index(id) {
            if idx >= self.reg_fast_slots.len() {
                self.vm_fast_reg_slots_ensure_len(idx);
            }
            self.reg_fast_slots[idx] = Some(value);
            // Fast-regfile invariant: in-range IDs are slot-only.
            debug_assert!(self.regs.get(&id).is_none());
            return;
        }
        self.regs.insert(id, value);
    }

    #[inline(always)]
    pub(super) fn take_reg(&mut self, id: ValueId) -> Option<VMValue> {
        if !self.vm_fast_regfile_enabled {
            self.reg_copy_aliases.remove(&id);
        }
        self.vm_fast_cache_clear(id);
        if let Some(idx) = self.vm_fast_reg_slot_index(id) {
            if idx < self.reg_fast_slots.len() {
                if let Some(v) = self.reg_fast_slots[idx].take() {
                    return Some(v);
                }
            }
            // Fast-regfile invariant: in-range IDs are slot-only.
            debug_assert!(self.regs.get(&id).is_none());
            return None;
        }
        self.regs.remove(&id)
    }

    const VM_FAST_REG_CACHE_LIMIT: usize = 4096;

    #[inline(always)]
    fn vm_fast_cache_index(&self, id: ValueId) -> Option<usize> {
        if !self.vm_fast_enabled || self.vm_fast_regfile_enabled {
            return None;
        }
        let idx = id.as_u32() as usize;
        if idx < Self::VM_FAST_REG_CACHE_LIMIT {
            Some(idx)
        } else {
            None
        }
    }

    #[inline(always)]
    fn vm_fast_cache_ensure_len(&mut self, idx: usize) {
        if self.reg_i64_cache.len() <= idx {
            self.reg_i64_cache.resize(idx + 1, None);
        }
        if self.reg_bool_cache.len() <= idx {
            self.reg_bool_cache.resize(idx + 1, None);
        }
    }

    #[inline(always)]
    pub(super) fn vm_fast_cache_clear(&mut self, id: ValueId) {
        if self.vm_fast_regfile_enabled {
            return;
        }
        if let Some(idx) = self.vm_fast_cache_index(id) {
            self.vm_fast_cache_ensure_len(idx);
            self.reg_i64_cache[idx] = None;
            self.reg_bool_cache[idx] = None;
        }
    }

    #[inline(always)]
    pub(super) fn vm_fast_cache_set_i64(&mut self, id: ValueId, value: i64) {
        if self.vm_fast_regfile_enabled {
            return;
        }
        if let Some(idx) = self.vm_fast_cache_index(id) {
            self.vm_fast_cache_ensure_len(idx);
            self.reg_i64_cache[idx] = Some(value);
            self.reg_bool_cache[idx] = None;
        }
    }

    #[inline(always)]
    pub(super) fn vm_fast_cache_set_bool(&mut self, id: ValueId, value: bool) {
        if self.vm_fast_regfile_enabled {
            return;
        }
        if let Some(idx) = self.vm_fast_cache_index(id) {
            self.vm_fast_cache_ensure_len(idx);
            self.reg_i64_cache[idx] = None;
            self.reg_bool_cache[idx] = Some(value);
        }
    }

    #[inline(always)]
    pub(super) fn vm_fast_read_i64(&mut self, id: ValueId) -> Option<i64> {
        if self.vm_fast_regfile_enabled {
            if let Some(VMValue::Integer(v)) = self.reg_peek_raw(id) {
                return Some(*v);
            }
            return None;
        }
        let resolved = self.resolve_copy_alias(id);
        if let Some(idx) = self.vm_fast_cache_index(resolved) {
            self.vm_fast_cache_ensure_len(idx);
            if let Some(v) = self.reg_i64_cache[idx] {
                return Some(v);
            }
        }
        if let Some(VMValue::Integer(v)) = self.reg_peek_raw(resolved) {
            let out = *v;
            self.vm_fast_cache_set_i64(resolved, out);
            return Some(out);
        }
        None
    }

    #[inline(always)]
    pub(super) fn vm_fast_read_bool(&mut self, id: ValueId) -> Option<bool> {
        if self.vm_fast_regfile_enabled {
            if let Some(VMValue::Bool(v)) = self.reg_peek_raw(id) {
                return Some(*v);
            }
            return None;
        }
        let resolved = self.resolve_copy_alias(id);
        if let Some(idx) = self.vm_fast_cache_index(resolved) {
            self.vm_fast_cache_ensure_len(idx);
            if let Some(v) = self.reg_bool_cache[idx] {
                return Some(v);
            }
        }
        if let Some(VMValue::Bool(v)) = self.reg_peek_raw(resolved) {
            let out = *v;
            self.vm_fast_cache_set_bool(resolved, out);
            return Some(out);
        }
        None
    }

    pub(super) fn reg_load(&self, id: ValueId) -> Result<VMValue, VMError> {
        match self.reg_peek_resolved(id).cloned() {
            Some(v) => Ok(v),
            None => {
                if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1")
                    || std::env::var("NYASH_VM_TRACE_EXEC").ok().as_deref() == Some("1")
                {
                    let keys: Vec<String> = self.regs.keys().map(|k| format!("{:?}", k)).collect();
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[vm-trace] reg_load undefined id={:?} fn={} last_block={:?} last_inst={:?} regs={}",
                        id,
                        self.cur_fn.as_deref().unwrap_or("<unknown>"),
                        self.last_block,
                        self.last_inst,
                        keys.join(", ")
                    ));
                }
                // Dev-time safety valve: tolerate undefined registers as Void when enabled
                let tolerate = self.vm_tolerate_void_enabled;
                if tolerate {
                    return Ok(VMValue::Void);
                }
                let fn_name = self.cur_fn.as_deref().unwrap_or("<unknown>");
                Err(VMError::InvalidValue(format!(
                    "[rust-vm] use of undefined value {:?} (fn={}, last_block={:?}, last_inst={:?})",
                    id,
                    fn_name,
                    self.last_block,
                    self.last_inst,
                )))
            }
        }
    }

    /// Compute a stable key for an object receiver to store fields across functions.
    /// Prefer Arc ptr address for BoxRef; else fall back to ValueId number cast.
    pub(super) fn object_key_for(&self, id: crate::mir::ValueId) -> u64 {
        if let Ok(v) = self.reg_load(id) {
            if let crate::backend::vm::VMValue::BoxRef(arc) = v {
                let ptr = std::sync::Arc::as_ptr(&arc) as *const ();
                return ptr as usize as u64;
            }
        }
        id.as_u32() as u64
    }
    pub(super) fn eval_binop(
        &self,
        op: BinaryOp,
        a: VMValue,
        b: VMValue,
    ) -> Result<VMValue, VMError> {
        use BinaryOp::*;
        use VMValue::*;
        // Dev-time: normalize BoxRef(VoidBox) → VMValue::Void when tolerance is enabled or in --dev mode.
        let tolerate = self.vm_tolerate_void_enabled;
        let (a, b) = if tolerate {
            let norm = |v: VMValue| -> VMValue {
                if let VMValue::BoxRef(bx) = &v {
                    if bx.as_any().downcast_ref::<VoidBox>().is_some() {
                        return VMValue::Void;
                    }
                }
                v
            };
            (norm(a), norm(b))
        } else {
            (a, b)
        };
        // Dev: nullish trace for binop
        if self.vm_null_missing_box_enabled && self.vm_box_trace_enabled {
            let (ak, bk) = (
                crate::backend::abi_util::tag_of_vm(&a),
                crate::backend::abi_util::tag_of_vm(&b),
            );
            let (an, bn) = (Self::tag_nullish(&a), Self::tag_nullish(&b));
            let op_s = match op {
                BinaryOp::Add => "Add",
                BinaryOp::Sub => "Sub",
                BinaryOp::Mul => "Mul",
                BinaryOp::Div => "Div",
                BinaryOp::Mod => "Mod",
                BinaryOp::BitAnd => "BitAnd",
                BinaryOp::BitOr => "BitOr",
                BinaryOp::BitXor => "BitXor",
                BinaryOp::And => "And",
                BinaryOp::Or => "Or",
                BinaryOp::Shl => "Shl",
                BinaryOp::Shr => "Shr",
            };
            crate::runtime::get_global_ring0().log.debug(&format!("{{\"ev\":\"binop\",\"op\":\"{}\",\"a_k\":\"{}\",\"b_k\":\"{}\",\"a_n\":\"{}\",\"b_n\":\"{}\"}}", op_s, ak, bk, an, bn));
        }
        Ok(match (op, a, b) {
            // Dev-only safety valves for Add (guarded by tolerance or --dev):
            // - Treat Void as 0 for numeric +
            // - Treat Void as empty string for string +
            (Add, VMValue::Void, Integer(y)) | (Add, Integer(y), VMValue::Void) if tolerate => {
                Integer(y)
            }
            (Add, VMValue::Void, Float(y)) | (Add, Float(y), VMValue::Void) if tolerate => Float(y),
            (Add, String(s), VMValue::Void) | (Add, VMValue::Void, String(s)) if tolerate => {
                String(s)
            }
            // Dev-only safety valve for Sub (guarded): treat Void as 0
            (Sub, Integer(x), VMValue::Void) if tolerate => Integer(x),
            (Sub, VMValue::Void, Integer(y)) if tolerate => Integer(0 - y),
            // Phase 275 C2: Number-only promotion
            (Add, Integer(x), Integer(y)) => Integer(x + y),
            (Add, Integer(x), Float(y)) => Float(x as f64 + y),
            (Add, Float(x), Integer(y)) => Float(x + y as f64),
            (Add, Float(x), Float(y)) => Float(x + y),
            // String concat (String + any -> String via to_string)
            (Add, String(s), String(t)) => String(format!("{}{}", s, t)),
            (Add, String(_), VMValue::Void) | (Add, VMValue::Void, String(_)) => {
                return Err(VMError::TypeError(
                    "unsupported binop Add on String and Void".to_string(),
                ))
            }
            (Add, String(s), other) => String(format!("{}{}", s, other.to_string())),
            (Add, other, String(s)) => String(format!("{}{}", other.to_string(), s)),
            (Sub, Integer(x), Integer(y)) => Integer(x - y),
            (Mul, Integer(x), Integer(y)) => Integer(x * y),
            (Div, Integer(_), Integer(0)) => return Err(VMError::DivisionByZero),
            (Div, Integer(x), Integer(y)) => Integer(x / y),
            (Mod, Integer(_), Integer(0)) => return Err(VMError::DivisionByZero),
            (Mod, Integer(x), Integer(y)) => Integer(x % y),
            (Sub, Float(x), Float(y)) => Float(x - y),
            (Mul, Float(x), Float(y)) => Float(x * y),
            (Div, Float(_), Float(y)) if y == 0.0 => return Err(VMError::DivisionByZero),
            (Div, Float(x), Float(y)) => Float(x / y),
            (Mod, Float(x), Float(y)) => Float(x % y),
            (BitAnd, Integer(x), Integer(y)) => Integer(x & y),
            (BitOr, Integer(x), Integer(y)) => Integer(x | y),
            (BitXor, Integer(x), Integer(y)) => Integer(x ^ y),
            (And, VMValue::Bool(x), VMValue::Bool(y)) => VMValue::Bool(x && y),
            (Or, VMValue::Bool(x), VMValue::Bool(y)) => VMValue::Bool(x || y),
            (Shl, Integer(x), Integer(y)) => Integer(x.wrapping_shl(y as u32)),
            (Shr, Integer(x), Integer(y)) => Integer(x.wrapping_shr(y as u32)),
            (opk, va, vb) => {
                if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[vm-trace] binop error fn={:?} op={:?} a={:?} b={:?} last_block={:?} last_inst={:?}",
                        self.cur_fn, opk, va, vb, self.last_block, self.last_inst
                    ));
                }
                return Err(VMError::TypeError(format!(
                    "unsupported binop {:?} on {:?} and {:?}",
                    opk, va, vb
                )));
            }
        })
    }

    pub(super) fn eval_cmp(&self, op: CompareOp, a: VMValue, b: VMValue) -> Result<bool, VMError> {
        use CompareOp::*;
        use VMValue::*;
        // Dev-time: normalize BoxRef(VoidBox) → VMValue::Void when tolerance is enabled or in --dev.
        let tolerate = self.vm_tolerate_void_enabled;
        let (a, b) = if tolerate {
            let norm = |v: VMValue| -> VMValue {
                if let VMValue::BoxRef(bx) = &v {
                    if bx.as_any().downcast_ref::<VoidBox>().is_some() {
                        return VMValue::Void;
                    }
                }
                v
            };
            (norm(a), norm(b))
        } else {
            (a, b)
        };
        // Dev-only safety valve: tolerate Void in comparisons when enabled or in --dev
        // → treat Void as 0 for numeric, empty for string
        let (a2, b2) = if tolerate {
            match (&a, &b) {
                (VMValue::Void, VMValue::Integer(_)) => (Integer(0), b.clone()),
                (VMValue::Integer(_), VMValue::Void) => (a.clone(), Integer(0)),
                (VMValue::Void, VMValue::Float(_)) => (Float(0.0), b.clone()),
                (VMValue::Float(_), VMValue::Void) => (a.clone(), Float(0.0)),
                (VMValue::Void, VMValue::String(_)) => (String(StdString::new()), b.clone()),
                (VMValue::String(_), VMValue::Void) => (a.clone(), String(StdString::new())),
                (VMValue::Void, _) => (Integer(0), b.clone()),
                (_, VMValue::Void) => (a.clone(), Integer(0)),
                _ => (a.clone(), b.clone()),
            }
        } else {
            (a, b)
        };
        // Final safety (dev-only): if types still mismatch and any side is Void, coerce to numeric zeros
        // Enabled only when tolerance is active (NYASH_VM_TOLERATE_VOID=1 or --dev)
        let (a3, b3) = if tolerate {
            match (&a2, &b2) {
                (VMValue::Void, VMValue::Integer(_)) => (Integer(0), b2.clone()),
                (VMValue::Integer(_), VMValue::Void) => (a2.clone(), Integer(0)),
                (VMValue::Void, VMValue::Float(_)) => (Float(0.0), b2.clone()),
                (VMValue::Float(_), VMValue::Void) => (a2.clone(), Float(0.0)),
                _ => (a2.clone(), b2.clone()),
            }
        } else {
            (a2.clone(), b2.clone())
        };
        // Dev: nullish trace for compare
        if self.vm_null_missing_box_enabled && self.vm_box_trace_enabled {
            let (ak, bk) = (
                crate::backend::abi_util::tag_of_vm(&a2),
                crate::backend::abi_util::tag_of_vm(&b2),
            );
            let (an, bn) = (Self::tag_nullish(&a2), Self::tag_nullish(&b2));
            let op_s = match op {
                CompareOp::Eq => "Eq",
                CompareOp::Ne => "Ne",
                CompareOp::Lt => "Lt",
                CompareOp::Le => "Le",
                CompareOp::Gt => "Gt",
                CompareOp::Ge => "Ge",
            };
            crate::runtime::get_global_ring0().log.debug(&format!("{{\"ev\":\"cmp\",\"op\":\"{}\",\"a_k\":\"{}\",\"b_k\":\"{}\",\"a_n\":\"{}\",\"b_n\":\"{}\"}}", op_s, ak, bk, an, bn));
        }
        let result = match (op, &a3, &b3) {
            (Eq, _, _) => eq_vm(&a2, &b2),
            (Ne, _, _) => !eq_vm(&a2, &b2),
            (Lt, Integer(x), Integer(y)) => x < y,
            (Le, Integer(x), Integer(y)) => x <= y,
            (Gt, Integer(x), Integer(y)) => x > y,
            (Ge, Integer(x), Integer(y)) => x >= y,
            (Lt, Float(x), Float(y)) => x < y,
            (Le, Float(x), Float(y)) => x <= y,
            (Gt, Float(x), Float(y)) => x > y,
            (Ge, Float(x), Float(y)) => x >= y,
            (Lt, VMValue::String(ref s), VMValue::String(ref t)) => s < t,
            (Le, VMValue::String(ref s), VMValue::String(ref t)) => s <= t,
            (Gt, VMValue::String(ref s), VMValue::String(ref t)) => s > t,
            (Ge, VMValue::String(ref s), VMValue::String(ref t)) => s >= t,
            (opk, va, vb) => {
                if std::env::var("NYASH_VM_TRACE").ok().as_deref() == Some("1") {
                    crate::runtime::get_global_ring0().log.debug(&format!(
                        "[vm-trace] compare error fn={:?} op={:?} a={:?} b={:?} last_block={:?} last_inst={:?}",
                        self.cur_fn, opk, va, vb, self.last_block, self.last_inst
                    ));
                }
                return Err(VMError::TypeError(format!(
                    "unsupported compare {:?} on {:?} and {:?}",
                    opk, va, vb
                )));
            }
        };
        Ok(result)
    }
}

// ---- Box trace (dev-only observer) ----
impl MirInterpreter {
    #[inline]
    pub(super) fn box_trace_enabled() -> bool {
        std::env::var("NYASH_BOX_TRACE").ok().as_deref() == Some("1")
    }

    fn box_trace_filter_match(class_name: &str) -> bool {
        if let Ok(filt) = std::env::var("NYASH_BOX_TRACE_FILTER") {
            let want = filt.trim();
            if want.is_empty() {
                return true;
            }
            // comma/space separated tokens; match if any token is contained in class
            for tok in want.split(|c: char| c == ',' || c.is_whitespace()) {
                let t = tok.trim();
                if !t.is_empty() && class_name.contains(t) {
                    return true;
                }
            }
            false
        } else {
            true
        }
    }

    fn json_escape(s: &str) -> String {
        let mut out = String::with_capacity(s.len() + 8);
        for ch in s.chars() {
            match ch {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if c.is_control() => out.push(' '),
                c => out.push(c),
            }
        }
        out
    }

    pub(super) fn box_trace_emit_new(&self, class_name: &str, argc: usize) {
        if !Self::box_trace_enabled() || !Self::box_trace_filter_match(class_name) {
            return;
        }
        crate::runtime::get_global_ring0().log.debug(&format!(
            "{{\"ev\":\"new\",\"class\":\"{}\",\"argc\":{}}}",
            Self::json_escape(class_name),
            argc
        ));
    }

    pub(super) fn box_trace_emit_call(&self, class_name: &str, method: &str, argc: usize) {
        if !Self::box_trace_enabled() || !Self::box_trace_filter_match(class_name) {
            return;
        }
        crate::runtime::get_global_ring0().log.debug(&format!(
            "{{\"ev\":\"call\",\"class\":\"{}\",\"method\":\"{}\",\"argc\":{}}}",
            Self::json_escape(class_name),
            Self::json_escape(method),
            argc
        ));
    }

    pub(super) fn box_trace_emit_get(&self, class_name: &str, field: &str, val_kind: &str) {
        if !Self::box_trace_enabled() || !Self::box_trace_filter_match(class_name) {
            return;
        }
        crate::runtime::get_global_ring0().log.debug(&format!(
            "{{\"ev\":\"get\",\"class\":\"{}\",\"field\":\"{}\",\"val\":\"{}\"}}",
            Self::json_escape(class_name),
            Self::json_escape(field),
            Self::json_escape(val_kind)
        ));
    }

    pub(super) fn box_trace_emit_set(&self, class_name: &str, field: &str, val_kind: &str) {
        if !Self::box_trace_enabled() || !Self::box_trace_filter_match(class_name) {
            return;
        }
        crate::runtime::get_global_ring0().log.debug(&format!(
            "{{\"ev\":\"set\",\"class\":\"{}\",\"field\":\"{}\",\"val\":\"{}\"}}",
            Self::json_escape(class_name),
            Self::json_escape(field),
            Self::json_escape(val_kind)
        ));
    }
}

// ---- Print trace (dev-only) ----
impl MirInterpreter {
    #[inline]
    pub(super) fn print_trace_enabled() -> bool {
        std::env::var("NYASH_PRINT_TRACE").ok().as_deref() == Some("1")
    }

    pub(super) fn print_trace_emit(&self, val: &VMValue) {
        if !Self::print_trace_enabled() {
            return;
        }
        let (kind, class, nullish) = match val {
            VMValue::Integer(_) => ("Integer", "".to_string(), None),
            VMValue::Float(_) => ("Float", "".to_string(), None),
            VMValue::Bool(_) => ("Bool", "".to_string(), None),
            VMValue::String(_) => ("String", "".to_string(), None),
            VMValue::Void => ("Void", "".to_string(), None),
            VMValue::Future(_) => ("Future", "".to_string(), None),
            VMValue::BoxRef(b) => {
                // Prefer InstanceBox.class_name when available
                if let Some(inst) = b.as_any().downcast_ref::<crate::instance_v2::InstanceBox>() {
                    let tag = if crate::config::env::null_missing_box_enabled() {
                        if b.as_any()
                            .downcast_ref::<crate::boxes::null_box::NullBox>()
                            .is_some()
                        {
                            Some("null")
                        } else if b
                            .as_any()
                            .downcast_ref::<crate::boxes::missing_box::MissingBox>()
                            .is_some()
                        {
                            Some("missing")
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    ("BoxRef", inst.class_name.clone(), tag)
                } else {
                    let tag = if crate::config::env::null_missing_box_enabled() {
                        if b.as_any()
                            .downcast_ref::<crate::boxes::null_box::NullBox>()
                            .is_some()
                        {
                            Some("null")
                        } else if b
                            .as_any()
                            .downcast_ref::<crate::boxes::missing_box::MissingBox>()
                            .is_some()
                        {
                            Some("missing")
                        } else {
                            None
                        }
                    } else {
                        None
                    };
                    ("BoxRef", b.type_name().to_string(), tag)
                }
            }
            VMValue::WeakBox(_) => ("WeakRef", "".to_string(), None), // Phase 285A0
        };
        if let Some(tag) = nullish {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "{{\"ev\":\"print\",\"kind\":\"{}\",\"class\":\"{}\",\"nullish\":\"{}\"}}",
                kind,
                Self::json_escape(&class),
                tag
            ));
        } else {
            crate::runtime::get_global_ring0().log.debug(&format!(
                "{{\"ev\":\"print\",\"kind\":\"{}\",\"class\":\"{}\"}}",
                kind,
                Self::json_escape(&class)
            ));
        }
    }
}
