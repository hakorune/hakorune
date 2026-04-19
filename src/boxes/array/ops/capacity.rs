use super::*;

impl ArrayBox {
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.items.read().capacity()
    }

    /// 要素を追加
    pub fn push(&self, item: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        let _ = self.slot_append_box_raw(item);
        Box::new(StringBox::new("ok"))
    }

    /// Raw append helper for substrate/plugin routes.
    /// Visible `push()` semantics stay above this seam.
    pub fn slot_append_box_raw(&self, item: Box<dyn NyashBox>) -> i64 {
        let mut items = self.items.write();
        let boxed = Self::ensure_boxed(&mut items);
        boxed.push(item);
        boxed.len() as i64
    }

    /// Raw reserve helper for substrate/plugin routes.
    /// Keeps visible owner semantics above the capacity seam.
    pub fn slot_reserve_capacity_raw(&self, additional: usize) -> bool {
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Boxed(items) => items.reserve(additional),
            ArrayStorage::Text(values) => values.reserve(additional),
            ArrayStorage::InlineI64(values) => values.reserve(additional),
            ArrayStorage::InlineBool(values) => values.reserve(additional),
            ArrayStorage::InlineF64(values) => values.reserve(additional),
        }
        true
    }

    /// Raw grow helper for substrate/plugin routes.
    /// Keeps visible owner semantics above the capacity seam.
    pub fn slot_grow_capacity_raw(&self, target_capacity: usize) -> bool {
        let mut items = self.items.write();
        let current_capacity = items.capacity();
        if current_capacity < target_capacity {
            let needed = target_capacity.saturating_sub(items.len());
            if needed > 0 {
                match &mut *items {
                    ArrayStorage::Boxed(items) => items.reserve(needed),
                    ArrayStorage::Text(values) => values.reserve(needed),
                    ArrayStorage::InlineI64(values) => values.reserve(needed),
                    ArrayStorage::InlineBool(values) => values.reserve(needed),
                    ArrayStorage::InlineF64(values) => values.reserve(needed),
                }
            }
        }
        true
    }

    /// 最後の要素を取り出す
    pub fn pop(&self) -> Box<dyn NyashBox> {
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Boxed(items) => match items.pop() {
                Some(item) => item,
                None => {
                    if Self::oob_strict_enabled() {
                        Box::new(StringBox::new("[array/empty/pop] empty array"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::Text(values) => match values.pop() {
                Some(value) => Box::new(StringBox::new(value)),
                None => {
                    if Self::oob_strict_enabled() {
                        Box::new(StringBox::new("[array/empty/pop] empty array"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::InlineI64(values) => match values.pop() {
                Some(value) => Box::new(IntegerBox::new(value)),
                None => {
                    if Self::oob_strict_enabled() {
                        Box::new(StringBox::new("[array/empty/pop] empty array"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::InlineBool(values) => match values.pop() {
                Some(value) => Box::new(BoolBox::new(value)),
                None => {
                    if Self::oob_strict_enabled() {
                        Box::new(StringBox::new("[array/empty/pop] empty array"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::InlineF64(values) => match values.pop() {
                Some(value) => Box::new(FloatBox::new(value)),
                None => {
                    if Self::oob_strict_enabled() {
                        Box::new(StringBox::new("[array/empty/pop] empty array"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
        }
    }

    /// 要素数を取得
    pub fn length(&self) -> Box<dyn NyashBox> {
        Box::new(IntegerBox::new(self.items.read().len() as i64))
    }

    /// size() エイリアス（length と同義）
    pub fn size(&self) -> Box<dyn NyashBox> {
        self.length()
    }

    /// Rust向けヘルパー: 要素数をusizeで取得（テスト用）
    pub fn len(&self) -> usize {
        self.items.read().len()
    }
}
