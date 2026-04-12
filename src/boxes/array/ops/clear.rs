use super::super::super::{ArrayBox, ArrayStorage};
use crate::box_trait::{NyashBox, StringBox};

impl ArrayBox {
    /// 配列を空にする
    pub fn clear(&self) -> Box<dyn NyashBox> {
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Boxed(items) => items.clear(),
            ArrayStorage::InlineI64(values) => values.clear(),
            ArrayStorage::InlineBool(values) => values.clear(),
            ArrayStorage::InlineF64(values) => values.clear(),
        }
        Box::new(StringBox::new("ok"))
    }
}
