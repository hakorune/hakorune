use super::super::super::{ArrayBox, ArrayStorage};
use crate::box_trait::{BoolBox, IntegerBox, NyashBox};

impl ArrayBox {
    /// 指定された値のインデックスを検索
    pub fn indexOf(&self, value: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => {
                for (i, item) in items.iter().enumerate() {
                    if item.equals(value.as_ref()).value {
                        return Box::new(IntegerBox::new(i as i64));
                    }
                }
            }
            ArrayStorage::Text(values) => {
                if let Some(needle) = value.as_str_fast() {
                    if let Some(idx) = values.iter().position(|item| item.equals_text(needle)) {
                        return Box::new(IntegerBox::new(idx as i64));
                    }
                }
            }
            ArrayStorage::InlineI64(values) => {
                if let Some(needle) = value.as_i64_fast() {
                    if let Some(idx) = values.iter().position(|item| *item == needle) {
                        return Box::new(IntegerBox::new(idx as i64));
                    }
                }
            }
            ArrayStorage::InlineBool(values) => {
                if let Some(needle) = value.as_bool_fast() {
                    if let Some(idx) = values.iter().position(|item| *item == needle) {
                        return Box::new(IntegerBox::new(idx as i64));
                    }
                }
            }
            ArrayStorage::InlineF64(values) => {
                if let Some(needle) = value.as_f64_fast() {
                    if let Some(idx) = values
                        .iter()
                        .position(|item| (*item - needle).abs() < f64::EPSILON)
                    {
                        return Box::new(IntegerBox::new(idx as i64));
                    }
                }
            }
        }
        Box::new(IntegerBox::new(-1))
    }

    /// 指定された値が含まれているか確認
    pub fn contains(&self, value: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => {
                for item in items.iter() {
                    if item.equals(value.as_ref()).value {
                        return Box::new(BoolBox::new(true));
                    }
                }
            }
            ArrayStorage::Text(values) => {
                if let Some(needle) = value.as_str_fast() {
                    return Box::new(BoolBox::new(
                        values.iter().any(|item| item.equals_text(needle)),
                    ));
                }
            }
            ArrayStorage::InlineI64(values) => {
                if let Some(needle) = value.as_i64_fast() {
                    return Box::new(BoolBox::new(values.iter().any(|item| *item == needle)));
                }
            }
            ArrayStorage::InlineBool(values) => {
                if let Some(needle) = value.as_bool_fast() {
                    return Box::new(BoolBox::new(values.iter().any(|item| *item == needle)));
                }
            }
            ArrayStorage::InlineF64(values) => {
                if let Some(needle) = value.as_f64_fast() {
                    return Box::new(BoolBox::new(
                        values
                            .iter()
                            .any(|item| (*item - needle).abs() < f64::EPSILON),
                    ));
                }
            }
        }
        Box::new(BoolBox::new(false))
    }
}
