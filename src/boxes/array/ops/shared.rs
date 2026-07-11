use super::*;

impl ArrayBox {
    pub fn new() -> Self {
        Self::new_with_storage(ArrayStorage::Boxed(Vec::new()))
    }

    /// 要素を持つArrayBoxを作成
    pub fn new_with_elements(elements: Vec<Box<dyn NyashBox>>) -> Self {
        Self::new_with_storage(ArrayStorage::Boxed(elements))
    }

    pub(super) fn new_with_storage_and_contract(
        storage: ArrayStorage,
        contract: Option<crate::typed_array_contract_spec::ArrayElementContractSpec>,
    ) -> Self {
        Self {
            items: Arc::new(ArrayStateCell::new_with_contract(storage, contract)),
            base: BoxBase::new(),
        }
    }

    pub(in crate::boxes::array) fn new_with_inline_record_storage(
        storage: ArrayInlineRecordStorage,
    ) -> Self {
        Self::new_with_storage(ArrayStorage::InlineRecord(storage))
    }

    #[allow(dead_code)] // C209 private pilot seam; C210 is the first planned consumer.
    pub(crate) fn new_with_inline_record_i64_columns_for_compiler_autouse(
        layout_id: u32,
        values_by_column: Vec<Vec<i64>>,
    ) -> Option<Self> {
        let storage = ArrayInlineRecordStorage::from_i64_columns(layout_id, values_by_column)?;
        Some(Self::new_with_inline_record_storage(storage))
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
            ArrayStorage::InlineRecord(_) => {
                panic!("[array/inline-record/unmaterialized] boxed read view is not enabled")
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
