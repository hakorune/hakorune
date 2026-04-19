use super::super::super::{ArrayBox, ArrayStorage};
use crate::box_trait::{IntegerBox, NyashBox, StringBox};

impl ArrayBox {
    /// 文字列結合
    pub fn join(&self, delimiter: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(sep_box) = delimiter.as_any().downcast_ref::<StringBox>() {
            let items = self.items.read();
            let parts: Vec<String> = match &*items {
                ArrayStorage::Boxed(items) => items
                    .iter()
                    .map(|item| item.to_string_box().value)
                    .collect(),
                ArrayStorage::Text(values) => values.clone(),
                ArrayStorage::InlineI64(values) => {
                    values.iter().map(|value| value.to_string()).collect()
                }
                ArrayStorage::InlineBool(values) => values
                    .iter()
                    .map(|value| {
                        if *value {
                            "true".to_string()
                        } else {
                            "false".to_string()
                        }
                    })
                    .collect(),
                ArrayStorage::InlineF64(values) => {
                    values.iter().map(|value| value.to_string()).collect()
                }
            };
            Box::new(StringBox::new(&parts.join(&sep_box.value)))
        } else {
            Box::new(StringBox::new("Error: join() requires string separator"))
        }
    }

    /// 配列をソート（昇順）
    pub fn sort(&self) -> Box<dyn NyashBox> {
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Text(values) => values.sort_unstable(),
            ArrayStorage::InlineI64(values) => values.sort_unstable(),
            ArrayStorage::InlineBool(values) => values.sort_unstable(),
            ArrayStorage::InlineF64(values) => values.sort_by(|lhs, rhs| lhs.total_cmp(rhs)),
            ArrayStorage::Boxed(items) => {
                // Numeric values first, then string values
                items.sort_by(|a, b| {
                    use std::cmp::Ordering;

                    if let (Some(a_int), Some(b_int)) = (
                        a.as_any().downcast_ref::<IntegerBox>(),
                        b.as_any().downcast_ref::<IntegerBox>(),
                    ) {
                        return a_int.value.cmp(&b_int.value);
                    }

                    if let (Some(a_float), Some(b_float)) = (
                        a.as_any()
                            .downcast_ref::<crate::boxes::math_box::FloatBox>(),
                        b.as_any()
                            .downcast_ref::<crate::boxes::math_box::FloatBox>(),
                    ) {
                        return a_float
                            .value
                            .partial_cmp(&b_float.value)
                            .unwrap_or(Ordering::Equal);
                    }

                    if let (Some(a_int), Some(b_float)) = (
                        a.as_any().downcast_ref::<IntegerBox>(),
                        b.as_any()
                            .downcast_ref::<crate::boxes::math_box::FloatBox>(),
                    ) {
                        return (a_int.value as f64)
                            .partial_cmp(&b_float.value)
                            .unwrap_or(Ordering::Equal);
                    }

                    if let (Some(a_float), Some(b_int)) = (
                        a.as_any()
                            .downcast_ref::<crate::boxes::math_box::FloatBox>(),
                        b.as_any().downcast_ref::<IntegerBox>(),
                    ) {
                        return a_float
                            .value
                            .partial_cmp(&(b_int.value as f64))
                            .unwrap_or(Ordering::Equal);
                    }

                    let a_str = a.to_string_box().value;
                    let b_str = b.to_string_box().value;
                    a_str.cmp(&b_str)
                });
            }
        }

        Box::new(StringBox::new("ok"))
    }

    /// 配列を反転
    pub fn reverse(&self) -> Box<dyn NyashBox> {
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Boxed(items) => items.reverse(),
            ArrayStorage::Text(values) => values.reverse(),
            ArrayStorage::InlineI64(values) => values.reverse(),
            ArrayStorage::InlineBool(values) => values.reverse(),
            ArrayStorage::InlineF64(values) => values.reverse(),
        }
        Box::new(StringBox::new("ok"))
    }
}
