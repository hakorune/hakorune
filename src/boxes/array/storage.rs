use super::*;

pub(super) enum ArrayStorage {
    Boxed(Vec<Box<dyn NyashBox>>),
    Text(Vec<ArrayTextCell>),
    InlineI64(Vec<i64>),
    InlineBool(Vec<bool>),
    InlineF64(Vec<f64>),
    InlineRecord(ArrayInlineRecordStorage),
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ArrayInlineRecordStorage {
    layout_id: u32,
    len: usize,
    columns: Vec<ArrayInlineRecordColumn>,
}

#[derive(Clone, Debug, PartialEq)]
pub(super) enum ArrayInlineRecordColumn {
    I64(Vec<i64>),
    Bool(Vec<bool>),
    F64(Vec<f64>),
}

#[allow(dead_code)] // C209 private pilot seam; C210 consumes it from metadata-store migration.
impl ArrayInlineRecordStorage {
    pub(super) fn new(layout_id: u32, columns: Vec<ArrayInlineRecordColumn>) -> Option<Self> {
        let len = columns.first().map_or(0, ArrayInlineRecordColumn::len);
        if columns.iter().all(|column| column.len() == len) {
            Some(Self {
                layout_id,
                len,
                columns,
            })
        } else {
            None
        }
    }

    pub(super) fn from_i64_columns(
        layout_id: u32,
        values_by_column: Vec<Vec<i64>>,
    ) -> Option<Self> {
        let columns = values_by_column
            .into_iter()
            .map(ArrayInlineRecordColumn::i64)
            .collect();
        Self::new(layout_id, columns)
    }

    pub(super) fn layout_id(&self) -> u32 {
        self.layout_id
    }

    pub(super) fn len(&self) -> usize {
        self.len
    }

    pub(super) fn capacity(&self) -> usize {
        self.columns
            .iter()
            .map(ArrayInlineRecordColumn::capacity)
            .min()
            .unwrap_or(0)
    }

    fn column_count(&self) -> usize {
        self.columns.len()
    }

    pub(super) fn reserve(&mut self, additional: usize) {
        for column in &mut self.columns {
            column.reserve(additional);
        }
    }

    pub(super) fn clear(&mut self) {
        for column in &mut self.columns {
            column.clear();
        }
        self.len = 0;
    }

    pub(super) fn slice_rows(&self, start: usize, end: usize) -> Self {
        let end = end.min(self.len);
        let start = start.min(end);
        let columns = self
            .columns
            .iter()
            .map(|column| column.slice_rows(start, end))
            .collect();
        Self {
            layout_id: self.layout_id,
            len: end.saturating_sub(start),
            columns,
        }
    }

    pub(super) fn summary(&self) -> String {
        format!(
            "[array/inline-record layout={} len={} columns={}]",
            self.layout_id,
            self.len,
            self.column_count()
        )
    }

    pub(super) fn load_i64_column(&self, row: usize, column: usize) -> Option<i64> {
        match self.columns.get(column)? {
            ArrayInlineRecordColumn::I64(values) => values.get(row).copied(),
            ArrayInlineRecordColumn::Bool(_) | ArrayInlineRecordColumn::F64(_) => None,
        }
    }
}

#[allow(dead_code)] // C209 private pilot seam; C210 consumes it from metadata-store migration.
impl ArrayInlineRecordColumn {
    pub(super) fn i64(values: Vec<i64>) -> Self {
        Self::I64(values)
    }

    #[cfg(test)]
    pub(super) fn bool_values(values: Vec<bool>) -> Self {
        Self::Bool(values)
    }

    #[cfg(test)]
    pub(super) fn f64(values: Vec<f64>) -> Self {
        Self::F64(values)
    }

    fn len(&self) -> usize {
        match self {
            Self::I64(values) => values.len(),
            Self::Bool(values) => values.len(),
            Self::F64(values) => values.len(),
        }
    }

    fn capacity(&self) -> usize {
        match self {
            Self::I64(values) => values.capacity(),
            Self::Bool(values) => values.capacity(),
            Self::F64(values) => values.capacity(),
        }
    }

    fn reserve(&mut self, additional: usize) {
        match self {
            Self::I64(values) => values.reserve(additional),
            Self::Bool(values) => values.reserve(additional),
            Self::F64(values) => values.reserve(additional),
        }
    }

    fn clear(&mut self) {
        match self {
            Self::I64(values) => values.clear(),
            Self::Bool(values) => values.clear(),
            Self::F64(values) => values.clear(),
        }
    }

    fn slice_rows(&self, start: usize, end: usize) -> Self {
        match self {
            Self::I64(values) => Self::I64(values[start..end].to_vec()),
            Self::Bool(values) => Self::Bool(values[start..end].to_vec()),
            Self::F64(values) => Self::F64(values[start..end].to_vec()),
        }
    }
}

impl ArrayStorage {
    pub(super) fn len(&self) -> usize {
        match self {
            Self::Boxed(items) => items.len(),
            Self::Text(values) => values.len(),
            Self::InlineI64(values) => values.len(),
            Self::InlineBool(values) => values.len(),
            Self::InlineF64(values) => values.len(),
            Self::InlineRecord(values) => values.len(),
        }
    }

    pub(super) fn capacity(&self) -> usize {
        match self {
            Self::Boxed(items) => items.capacity(),
            Self::Text(values) => values.capacity(),
            Self::InlineI64(values) => values.capacity(),
            Self::InlineBool(values) => values.capacity(),
            Self::InlineF64(values) => values.capacity(),
            Self::InlineRecord(values) => values.capacity(),
        }
    }
}

impl ArrayBox {
    pub(super) fn oob_strict_enabled() -> bool {
        env::env_bool("HAKO_OOB_STRICT") || env::env_bool("NYASH_OOB_STRICT")
    }

    pub(super) fn new_with_storage(storage: ArrayStorage) -> Self {
        ArrayBox {
            items: Arc::new(RwLock::new(storage)),
            base: BoxBase::new(),
        }
    }

    pub(super) fn boxed_from_inline(values: &[i64]) -> Vec<Box<dyn NyashBox>> {
        values
            .iter()
            .map(|value| Box::new(IntegerBox::new(*value)) as Box<dyn NyashBox>)
            .collect()
    }

    pub(super) fn boxed_from_inline_bool(values: &[bool]) -> Vec<Box<dyn NyashBox>> {
        values
            .iter()
            .map(|value| Box::new(BoolBox::new(*value)) as Box<dyn NyashBox>)
            .collect()
    }

    pub(super) fn boxed_from_inline_f64(values: &[f64]) -> Vec<Box<dyn NyashBox>> {
        values
            .iter()
            .map(|value| Box::new(FloatBox::new(*value)) as Box<dyn NyashBox>)
            .collect()
    }

    pub(super) fn boxed_from_text(values: &[ArrayTextCell]) -> Vec<Box<dyn NyashBox>> {
        values
            .iter()
            .map(|value| Box::new(StringBox::new(value.to_visible_string())) as Box<dyn NyashBox>)
            .collect()
    }

    pub(super) fn try_inline_i64_values(items: &[Box<dyn NyashBox>]) -> Option<Vec<i64>> {
        items.iter().map(|item| item.as_i64_fast()).collect()
    }

    pub(super) fn try_inline_bool_values(items: &[Box<dyn NyashBox>]) -> Option<Vec<bool>> {
        items.iter().map(|item| item.as_bool_fast()).collect()
    }

    pub(super) fn try_inline_f64_values(items: &[Box<dyn NyashBox>]) -> Option<Vec<f64>> {
        items.iter().map(|item| item.as_f64_fast()).collect()
    }

    pub(super) fn try_text_values(items: &[Box<dyn NyashBox>]) -> Option<Vec<ArrayTextCell>> {
        items
            .iter()
            .map(|item| {
                item.as_str_fast()
                    .map(|value| ArrayTextCell::flat(value.to_owned()))
            })
            .collect()
    }

    pub(super) fn ensure_boxed(storage: &mut ArrayStorage) -> &mut Vec<Box<dyn NyashBox>> {
        if let ArrayStorage::Text(values) = storage {
            *storage = ArrayStorage::Boxed(Self::boxed_from_text(values));
        }
        if let ArrayStorage::InlineI64(values) = storage {
            *storage = ArrayStorage::Boxed(Self::boxed_from_inline(values));
        }
        if let ArrayStorage::InlineBool(values) = storage {
            *storage = ArrayStorage::Boxed(Self::boxed_from_inline_bool(values));
        }
        if let ArrayStorage::InlineF64(values) = storage {
            *storage = ArrayStorage::Boxed(Self::boxed_from_inline_f64(values));
        }
        match storage {
            ArrayStorage::Boxed(items) => items,
            ArrayStorage::Text(_)
            | ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => {
                unreachable!("inline storage promoted to boxed")
            }
            ArrayStorage::InlineRecord(_) => {
                panic!("[array/inline-record/unmaterialized] boxed materialization is not enabled")
            }
        }
    }

    pub(super) fn ensure_text(storage: &mut ArrayStorage) -> Option<&mut Vec<ArrayTextCell>> {
        if let ArrayStorage::Boxed(items) = storage {
            let values = Self::try_text_values(items)?;
            *storage = ArrayStorage::Text(values);
        }
        match storage {
            ArrayStorage::Text(values) => Some(values),
            ArrayStorage::Boxed(_)
            | ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_)
            | ArrayStorage::InlineRecord(_) => None,
        }
    }

    pub(super) fn ensure_inline_i64(storage: &mut ArrayStorage) -> Option<&mut Vec<i64>> {
        if let ArrayStorage::Boxed(items) = storage {
            let values = Self::try_inline_i64_values(items)?;
            *storage = ArrayStorage::InlineI64(values);
        }
        match storage {
            ArrayStorage::InlineI64(values) => Some(values),
            ArrayStorage::Boxed(_)
            | ArrayStorage::Text(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_)
            | ArrayStorage::InlineRecord(_) => None,
        }
    }

    pub(super) fn ensure_inline_bool(storage: &mut ArrayStorage) -> Option<&mut Vec<bool>> {
        if let ArrayStorage::Boxed(items) = storage {
            let values = Self::try_inline_bool_values(items)?;
            *storage = ArrayStorage::InlineBool(values);
        }
        match storage {
            ArrayStorage::InlineBool(values) => Some(values),
            ArrayStorage::Boxed(_)
            | ArrayStorage::Text(_)
            | ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineF64(_)
            | ArrayStorage::InlineRecord(_) => None,
        }
    }

    pub(super) fn ensure_inline_f64(storage: &mut ArrayStorage) -> Option<&mut Vec<f64>> {
        if let ArrayStorage::Boxed(items) = storage {
            let values = Self::try_inline_f64_values(items)?;
            *storage = ArrayStorage::InlineF64(values);
        }
        match storage {
            ArrayStorage::InlineF64(values) => Some(values),
            ArrayStorage::Boxed(_)
            | ArrayStorage::Text(_)
            | ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineRecord(_) => None,
        }
    }

    pub(super) fn clone_visible_item(item: &dyn NyashBox) -> Box<dyn NyashBox> {
        #[cfg(all(feature = "plugins", not(target_arch = "wasm32")))]
        if item
            .as_any()
            .downcast_ref::<crate::runtime::plugin_loader_v2::PluginBoxV2>()
            .is_some()
        {
            return item.share_box();
        }
        if item
            .as_any()
            .downcast_ref::<crate::instance_v2::InstanceBox>()
            .is_some()
        {
            return item.share_box();
        }
        if item.as_any().downcast_ref::<ArrayBox>().is_some() {
            return item.share_box();
        }
        if item
            .as_any()
            .downcast_ref::<crate::boxes::MapBox>()
            .is_some()
        {
            return item.share_box();
        }
        if item.borrowed_handle_source_fast().is_some() {
            return item.share_box();
        }
        item.clone_box()
    }

    pub(super) fn format_inline_values(values: &[i64]) -> String {
        values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(super) fn format_inline_bool_values(values: &[bool]) -> String {
        values
            .iter()
            .map(|value| {
                if *value {
                    "true".to_string()
                } else {
                    "false".to_string()
                }
            })
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(super) fn format_inline_f64_values(values: &[f64]) -> String {
        values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    pub(super) fn text_cells_from_strings(values: Vec<String>) -> Vec<ArrayTextCell> {
        values.into_iter().map(ArrayTextCell::from).collect()
    }

    pub(super) fn strings_from_text(values: &[ArrayTextCell]) -> Vec<String> {
        values
            .iter()
            .map(ArrayTextCell::to_visible_string)
            .collect()
    }

    pub(super) fn format_text_values(values: &[ArrayTextCell]) -> String {
        Self::strings_from_text(values).join(", ")
    }
}
