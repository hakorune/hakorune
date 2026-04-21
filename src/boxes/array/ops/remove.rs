use super::super::super::{ArrayBox, ArrayStorage};
use crate::box_trait::{BoolBox, IntegerBox, NyashBox, StringBox};
use crate::FloatBox;

impl ArrayBox {
    /// 要素を削除
    pub fn remove(&self, index: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(idx_box) = index.as_any().downcast_ref::<IntegerBox>() {
            let idx = idx_box.value as usize;
            let mut items = self.items.write();
            match &mut *items {
                ArrayStorage::Boxed(items) => {
                    if idx < items.len() {
                        items.remove(idx)
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
                ArrayStorage::Text(values) => {
                    if idx < values.len() {
                        Box::new(StringBox::new(values.remove(idx).into_string()))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
                ArrayStorage::InlineI64(values) => {
                    if idx < values.len() {
                        Box::new(IntegerBox::new(values.remove(idx)))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
                ArrayStorage::InlineBool(values) => {
                    if idx < values.len() {
                        Box::new(BoolBox::new(values.remove(idx)))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
                ArrayStorage::InlineF64(values) => {
                    if idx < values.len() {
                        Box::new(FloatBox::new(values.remove(idx)))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            }
        } else {
            Box::new(StringBox::new("Error: remove() requires integer index"))
        }
    }
}
