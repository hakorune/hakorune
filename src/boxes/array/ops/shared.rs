use super::*;

impl ArrayBox {
    pub fn new() -> Self {
        Self::new_with_storage(ArrayStorage::Boxed(Vec::new()))
    }

    /// 要素を持つArrayBoxを作成
    pub fn new_with_elements(elements: Vec<Box<dyn NyashBox>>) -> Self {
        Self::new_with_storage(ArrayStorage::Boxed(elements))
    }

    pub(super) fn new_with_inline_i64_elements(values: Vec<i64>) -> Self {
        Self::new_with_storage(ArrayStorage::InlineI64(values))
    }

    pub(super) fn new_with_inline_bool_elements(values: Vec<bool>) -> Self {
        Self::new_with_storage(ArrayStorage::InlineBool(values))
    }

    pub(super) fn new_with_inline_f64_elements(values: Vec<f64>) -> Self {
        Self::new_with_storage(ArrayStorage::InlineF64(values))
    }

    pub(super) fn new_with_text_elements(values: Vec<String>) -> Self {
        Self::new_with_storage(ArrayStorage::Text(Self::text_cells_from_strings(values)))
    }

    #[inline(always)]
    pub fn with_items_read<R>(&self, f: impl FnOnce(&Vec<Box<dyn NyashBox>>) -> R) -> R {
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => f(items),
            ArrayStorage::Text(values) => {
                let materialized = Self::boxed_from_text(values);
                f(&materialized)
            }
            ArrayStorage::InlineI64(values) => {
                let materialized = Self::boxed_from_inline(values);
                f(&materialized)
            }
            ArrayStorage::InlineBool(values) => {
                let materialized = Self::boxed_from_inline_bool(values);
                f(&materialized)
            }
            ArrayStorage::InlineF64(values) => {
                let materialized = Self::boxed_from_inline_f64(values);
                f(&materialized)
            }
        }
    }

    #[inline(always)]
    pub fn with_items_write<R>(&self, f: impl FnOnce(&mut Vec<Box<dyn NyashBox>>) -> R) -> R {
        let mut items = self.items.write();
        let boxed = Self::ensure_boxed(&mut items);
        f(boxed)
    }
}
