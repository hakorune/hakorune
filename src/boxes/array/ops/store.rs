use super::super::super::ArrayBox;
use crate::box_trait::{BoolBox, IntegerBox, NyashBox};
use crate::boxes::FloatBox;

impl ArrayBox {
    /// インデックス(i64)へ整数値を設定し、成功可否を返す（AOT integer route 用）
    #[inline(always)]
    pub fn try_set_index_i64_integer(&self, idx: i64, value: i64) -> bool {
        self.slot_store_i64_raw(idx, value)
    }

    /// インデックス(i64)で要素を設定し、成功可否を返す（FFI/Kernel hot path 用）
    pub fn try_set_index_i64(&self, idx: i64, value: Box<dyn NyashBox>) -> bool {
        self.slot_store_box_raw(idx, value)
    }

    /// Raw store helper for substrate/plugin routes.
    /// Keeps the current append-at-end and OOB policy while visible `set()` stays above this seam.
    pub fn slot_store_box_raw(&self, idx: i64, value: Box<dyn NyashBox>) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(bool_value) = value.as_bool_fast() {
            if let Some(values) = Self::ensure_inline_bool(&mut items) {
                if idx < values.len() {
                    values[idx] = bool_value;
                    return true;
                } else if idx == values.len() {
                    values.push(bool_value);
                    return true;
                } else {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                    }
                    return false;
                }
            }
        }
        if let Some(float_value) = value.as_f64_fast() {
            if let Some(values) = Self::ensure_inline_f64(&mut items) {
                if idx < values.len() {
                    values[idx] = float_value;
                    return true;
                } else if idx == values.len() {
                    values.push(float_value);
                    return true;
                } else {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                    }
                    return false;
                }
            }
        }
        let boxed = Self::ensure_boxed(&mut items);
        if idx < boxed.len() {
            boxed[idx] = value;
            true
        } else if idx == boxed.len() {
            // Pragmatic semantics: allow set at exact end to append
            boxed.push(value);
            true
        } else {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            false
        }
    }

    /// Raw integer store helper for substrate/plugin routes.
    /// Keeps the current append-at-end / rebox policy while visible `set()` stays above this seam.
    #[inline(always)]
    pub fn slot_store_i64_raw(&self, idx: i64, value: i64) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_inline_i64(&mut items) {
            if idx < values.len() {
                values[idx] = value;
                true
            } else if idx == values.len() {
                values.push(value);
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        } else {
            let boxed = Self::ensure_boxed(&mut items);
            if idx < boxed.len() {
                if let Some(slot) = boxed[idx].i64_slot_mut() {
                    *slot = value;
                } else {
                    boxed[idx] = Box::new(IntegerBox::new(value));
                }
                true
            } else if idx == boxed.len() {
                boxed.push(Box::new(IntegerBox::new(value)));
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        }
    }

    /// Raw boolean store helper for substrate/plugin routes.
    #[inline(always)]
    pub fn slot_store_bool_raw(&self, idx: i64, value: bool) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_inline_bool(&mut items) {
            if idx < values.len() {
                values[idx] = value;
                true
            } else if idx == values.len() {
                values.push(value);
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        } else {
            let boxed = Self::ensure_boxed(&mut items);
            if idx < boxed.len() {
                boxed[idx] = Box::new(BoolBox::new(value));
                true
            } else if idx == boxed.len() {
                boxed.push(Box::new(BoolBox::new(value)));
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        }
    }

    /// Raw float store helper for substrate/plugin routes.
    #[inline(always)]
    pub fn slot_store_f64_raw(&self, idx: i64, value: f64) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_inline_f64(&mut items) {
            if idx < values.len() {
                values[idx] = value;
                true
            } else if idx == values.len() {
                values.push(value);
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        } else {
            let boxed = Self::ensure_boxed(&mut items);
            if idx < boxed.len() {
                boxed[idx] = Box::new(FloatBox::new(value));
                true
            } else if idx == boxed.len() {
                boxed.push(Box::new(FloatBox::new(value)));
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        }
    }

    /// Raw integer read-modify-write helper for substrate/plugin routes.
    /// Returns the updated value when the slot exists and can be treated as `i64`.
    #[inline(always)]
    pub fn slot_rmw_add1_i64_raw(&self, idx: i64) -> Option<i64> {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_inline_i64(&mut items) {
            let slot = values.get_mut(idx)?;
            *slot = slot.checked_add(1)?;
            return Some(*slot);
        }
        let boxed = Self::ensure_boxed(&mut items);
        let item = boxed.get_mut(idx)?;
        if let Some(slot) = item.i64_slot_mut() {
            *slot += 1;
            return Some(*slot);
        }
        let next = item.as_i64_fast()?.checked_add(1)?;
        *item = Box::new(IntegerBox::new(next));
        Some(next)
    }
}
