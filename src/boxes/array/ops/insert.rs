use super::super::super::{ArrayBox, ArrayStorage, ArrayTextCell};
use crate::box_trait::{IntegerBox, NyashBox, StringBox};

impl ArrayBox {
    /// Insert a value at a visible ArrayBox index.
    pub fn insert(&self, index: Box<dyn NyashBox>, value: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(idx_box) = index.as_any().downcast_ref::<IntegerBox>() {
            self.insert_index_i64(idx_box.value, value)
        } else {
            Box::new(StringBox::new("Error: insert() requires integer index"))
        }
    }

    /// Insert a value at an i64 index.
    pub fn insert_index_i64(&self, idx: i64, value: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if self.slot_insert_box_raw(idx, value) {
            Box::new(StringBox::new("ok"))
        } else if Self::oob_strict_enabled() {
            Box::new(StringBox::new("[oob/array/insert] index out of bounds"))
        } else {
            Box::new(StringBox::new("Error: index out of bounds"))
        }
    }

    /// Raw insert helper for substrate/plugin routes.
    /// Visible `insert()` semantics stay above this seam.
    pub fn slot_insert_box_raw(&self, idx: i64, value: Box<dyn NyashBox>) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if idx > items.len() {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }

        if let Some(text_value) = value.as_str_fast() {
            if let ArrayStorage::Text(values) = &mut *items {
                values.insert(idx, ArrayTextCell::from(text_value.to_owned()));
                return true;
            }
        }

        if let Some(int_value) = value.as_i64_fast() {
            if let Some(values) = Self::ensure_inline_i64(&mut items) {
                values.insert(idx, int_value);
                return true;
            }
        }

        if let Some(bool_value) = value.as_bool_fast() {
            if let Some(values) = Self::ensure_inline_bool(&mut items) {
                values.insert(idx, bool_value);
                return true;
            }
        }

        if let Some(float_value) = value.as_f64_fast() {
            if let Some(values) = Self::ensure_inline_f64(&mut items) {
                values.insert(idx, float_value);
                return true;
            }
        }

        let boxed = Self::ensure_boxed(&mut items);
        boxed.insert(idx, value);
        true
    }
}
