use super::*;

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

    /// 文字列結合
    pub fn join(&self, delimiter: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(sep_box) = delimiter.as_any().downcast_ref::<StringBox>() {
            let items = self.items.read();
            let parts: Vec<String> = match &*items {
                ArrayStorage::Boxed(items) => items
                    .iter()
                    .map(|item| item.to_string_box().value)
                    .collect(),
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
            ArrayStorage::InlineI64(values) => values.sort_unstable(),
            ArrayStorage::InlineBool(values) => values.sort_unstable(),
            ArrayStorage::InlineF64(values) => values.sort_by(|lhs, rhs| lhs.total_cmp(rhs)),
            ArrayStorage::Boxed(items) => {
                // Numeric values first, then string values
                items.sort_by(|a, b| {
                    use std::cmp::Ordering;

                    // Try to compare as numbers first
                    if let (Some(a_int), Some(b_int)) = (
                        a.as_any().downcast_ref::<IntegerBox>(),
                        b.as_any().downcast_ref::<IntegerBox>(),
                    ) {
                        return a_int.value.cmp(&b_int.value);
                    }

                    // Try FloatBox comparison
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

                    // Mixed numeric types
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

                    // Fall back to string comparison
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
            ArrayStorage::InlineI64(values) => values.reverse(),
            ArrayStorage::InlineBool(values) => values.reverse(),
            ArrayStorage::InlineF64(values) => values.reverse(),
        }
        Box::new(StringBox::new("ok"))
    }

    /// 部分配列を取得
    pub fn slice(&self, start: Box<dyn NyashBox>, end: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        let items = self.items.read();

        // Extract start and end indices
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

        // Validate indices
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
