use super::super::super::{ArrayBox, ArrayStorage};
use crate::box_trait::{IntegerBox, NyashBox, StringBox};

impl ArrayBox {
    /// 部分配列を取得
    pub fn slice(&self, start: Box<dyn NyashBox>, end: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
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
            return Box::new(ArrayBox::new());
        }

        match &*items {
            ArrayStorage::Boxed(items) => {
                let slice_items: Vec<Box<dyn NyashBox>> = items[start_idx..end_idx]
                    .iter()
                    .map(|item| Self::clone_visible_item(item.as_ref()))
                    .collect();
                Box::new(ArrayBox::new_with_elements(slice_items))
            }
            ArrayStorage::InlineI64(values) => Box::new(ArrayBox::new_with_inline_i64_elements(
                values[start_idx..end_idx].to_vec(),
            )),
            ArrayStorage::InlineBool(values) => Box::new(ArrayBox::new_with_inline_bool_elements(
                values[start_idx..end_idx].to_vec(),
            )),
            ArrayStorage::InlineF64(values) => Box::new(ArrayBox::new_with_inline_f64_elements(
                values[start_idx..end_idx].to_vec(),
            )),
        }
    }
}
