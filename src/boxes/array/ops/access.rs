use super::*;

impl ArrayBox {
    #[inline(always)]
    pub fn slot_load_i64_raw(&self, idx: i64) -> Option<i64> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => items.get(idx).and_then(|item| item.as_i64_fast()),
            ArrayStorage::Text(_) => None,
            ArrayStorage::InlineI64(values) => values.get(idx).copied(),
            ArrayStorage::InlineBool(values) => {
                values.get(idx).map(|value| if *value { 1 } else { 0 })
            }
            ArrayStorage::InlineF64(_) => None,
        }
    }

    /// インデックス(i64)で要素を取得（FFI/Kernel hot path 用）
    pub fn get_index_i64(&self, idx: i64) -> Box<dyn NyashBox> {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
                return Box::new(StringBox::new("[oob/array/get] index out of bounds"));
            }
            return Box::new(crate::boxes::null_box::NullBox::new());
        }
        let idx = idx as usize;
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => match items.get(idx) {
                Some(item) => Self::clone_visible_item(item.as_ref()),
                None => {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                        Box::new(StringBox::new("[oob/array/get] index out of bounds"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::Text(values) => match values.get(idx) {
                Some(value) => Box::new(StringBox::new(value.clone())),
                None => {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                        Box::new(StringBox::new("[oob/array/get] index out of bounds"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::InlineI64(values) => match values.get(idx) {
                Some(value) => Box::new(IntegerBox::new(*value)),
                None => {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                        Box::new(StringBox::new("[oob/array/get] index out of bounds"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::InlineBool(values) => match values.get(idx) {
                Some(value) => Box::new(BoolBox::new(*value)),
                None => {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                        Box::new(StringBox::new("[oob/array/get] index out of bounds"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::InlineF64(values) => match values.get(idx) {
                Some(value) => Box::new(FloatBox::new(*value)),
                None => {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                        Box::new(StringBox::new("[oob/array/get] index out of bounds"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
        }
    }

    /// インデックス(i64)が配列範囲内かを返す（副作用なし）
    pub fn has_index_i64(&self, idx: i64) -> bool {
        if idx < 0 {
            return false;
        }
        (idx as usize) < self.items.read().len()
    }

    /// インデックス(i64)で要素を設定
    pub fn set_index_i64(&self, idx: i64, value: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if self.try_set_index_i64(idx, value) {
            Box::new(StringBox::new("ok"))
        } else if Self::oob_strict_enabled() {
            Box::new(StringBox::new("[oob/array/set] index out of bounds"))
        } else {
            Box::new(StringBox::new("Error: index out of bounds"))
        }
    }

    /// インデックスで要素を取得
    pub fn get(&self, index: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(idx_box) = index.as_any().downcast_ref::<IntegerBox>() {
            self.get_index_i64(idx_box.value)
        } else {
            Box::new(StringBox::new("Error: get() requires integer index"))
        }
    }

    /// インデックスで要素を設定
    pub fn set(&self, index: Box<dyn NyashBox>, value: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(idx_box) = index.as_any().downcast_ref::<IntegerBox>() {
            self.set_index_i64(idx_box.value, value)
        } else {
            Box::new(StringBox::new("Error: set() requires integer index"))
        }
    }
}
