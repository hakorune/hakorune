use super::super::super::{ArrayBox, ArrayStorage};
use crate::box_trait::{IntegerBox, NyashBox, StringBox};

impl ArrayBox {
    /// 部分配列を取得
    pub fn slice(&self, start: Box<dyn NyashBox>, end: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        let contract = self.items.element_contract();
        let items = self.items.read();

        let start_idx = if let Some(start_int) = start.as_any().downcast_ref::<IntegerBox>() {
            if start_int.value < 0 {
                0
            } else {
                start_int.value as usize
            }
        } else {
            return Box::new(StringBox::new(
                "Error: slice() start index must be an integer",
            ));
        };

        let end_idx = if let Some(end_int) = end.as_any().downcast_ref::<IntegerBox>() {
            if end_int.value < 0 {
                items.len()
            } else {
                (end_int.value as usize).min(items.len())
            }
        } else {
            return Box::new(StringBox::new(
                "Error: slice() end index must be an integer",
            ));
        };

        if start_idx > items.len() || start_idx > end_idx {
            return Box::new(ArrayBox::new_with_storage_and_contract(
                ArrayStorage::Boxed(Vec::new()),
                contract,
            ));
        }

        let storage = match &*items {
            ArrayStorage::Boxed(items) => {
                let slice_items: Vec<Box<dyn NyashBox>> = items[start_idx..end_idx]
                    .iter()
                    .map(|item| Self::clone_visible_item(item.as_ref()))
                    .collect();
                ArrayStorage::Boxed(slice_items)
            }
            ArrayStorage::Text(values) => ArrayStorage::Text(Self::text_cells_from_strings(
                Self::strings_from_text(&values[start_idx..end_idx]),
            )),
            ArrayStorage::InlineI64(values) => {
                ArrayStorage::InlineI64(values[start_idx..end_idx].to_vec())
            }
            ArrayStorage::InlineBool(values) => {
                ArrayStorage::InlineBool(values[start_idx..end_idx].to_vec())
            }
            ArrayStorage::InlineF64(values) => {
                ArrayStorage::InlineF64(values[start_idx..end_idx].to_vec())
            }
            ArrayStorage::InlineRecord(values) => {
                ArrayStorage::InlineRecord(values.slice_rows(start_idx, end_idx))
            }
        };
        Box::new(ArrayBox::new_with_storage_and_contract(storage, contract))
    }
}
