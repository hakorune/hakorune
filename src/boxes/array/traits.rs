use super::*;

impl Clone for ArrayBox {
    fn clone(&self) -> Self {
        let items_guard = self.items.read();
        let cloned_items = match &*items_guard {
            ArrayStorage::Boxed(items) => ArrayStorage::Boxed(
                items
                    .iter()
                    .map(|item| Self::clone_visible_item(item.as_ref()))
                    .collect(),
            ),
            ArrayStorage::InlineI64(values) => ArrayStorage::InlineI64(values.clone()),
            ArrayStorage::InlineBool(values) => ArrayStorage::InlineBool(values.clone()),
            ArrayStorage::InlineF64(values) => ArrayStorage::InlineF64(values.clone()),
        };

        ArrayBox {
            items: Arc::new(RwLock::new(cloned_items)),
            base: BoxBase::new(),
        }
    }
}

impl BoxCore for ArrayBox {
    fn box_id(&self) -> u64 {
        self.base.id
    }

    fn parent_type_id(&self) -> Option<std::any::TypeId> {
        self.base.parent_type_id
    }

    fn fmt_box(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => {
                let strings: Vec<String> = items
                    .iter()
                    .map(|item| item.to_string_box().value)
                    .collect();
                write!(f, "[{}]", strings.join(", "))
            }
            ArrayStorage::InlineI64(values) => {
                write!(f, "[{}]", Self::format_inline_values(values))
            }
            ArrayStorage::InlineBool(values) => {
                write!(f, "[{}]", Self::format_inline_bool_values(values))
            }
            ArrayStorage::InlineF64(values) => {
                write!(f, "[{}]", Self::format_inline_f64_values(values))
            }
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for ArrayBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt_box(f)
    }
}

impl NyashBox for ArrayBox {
    fn is_identity(&self) -> bool {
        true
    }
    fn clone_box(&self) -> Box<dyn NyashBox> {
        Box::new(self.clone())
    }

    /// 🎯 状態共有の核心実装
    fn share_box(&self) -> Box<dyn NyashBox> {
        let new_instance = ArrayBox {
            items: Arc::clone(&self.items), // Arcクローンで状態共有
            base: BoxBase::new(),           // 新しいID
        };
        Box::new(new_instance)
    }

    fn to_string_box(&self) -> StringBox {
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => {
                let strings: Vec<String> = items
                    .iter()
                    .map(|item| item.to_string_box().value)
                    .collect();
                StringBox::new(format!("[{}]", strings.join(", ")))
            }
            ArrayStorage::InlineI64(values) => {
                StringBox::new(format!("[{}]", Self::format_inline_values(values)))
            }
            ArrayStorage::InlineBool(values) => {
                StringBox::new(format!("[{}]", Self::format_inline_bool_values(values)))
            }
            ArrayStorage::InlineF64(values) => {
                StringBox::new(format!("[{}]", Self::format_inline_f64_values(values)))
            }
        }
    }

    fn type_name(&self) -> &'static str {
        "ArrayBox"
    }

    fn equals(&self, other: &dyn NyashBox) -> BoolBox {
        if let Some(other_array) = other.as_any().downcast_ref::<ArrayBox>() {
            let self_items = self.items.read();
            let other_items = other_array.items.read();

            if self_items.len() != other_items.len() {
                return BoolBox::new(false);
            }

            match (&*self_items, &*other_items) {
                (ArrayStorage::InlineI64(lhs), ArrayStorage::InlineI64(rhs)) => {
                    return BoolBox::new(lhs == rhs);
                }
                (ArrayStorage::InlineI64(lhs), ArrayStorage::Boxed(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        if b.as_i64_fast() != Some(*a) {
                            return BoolBox::new(false);
                        }
                    }
                }
                (ArrayStorage::InlineBool(lhs), ArrayStorage::InlineBool(rhs)) => {
                    return BoolBox::new(lhs == rhs);
                }
                (ArrayStorage::InlineBool(lhs), ArrayStorage::Boxed(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        if b.as_bool_fast() != Some(*a) {
                            return BoolBox::new(false);
                        }
                    }
                }
                (ArrayStorage::InlineF64(lhs), ArrayStorage::InlineF64(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        if (*a - *b).abs() >= f64::EPSILON {
                            return BoolBox::new(false);
                        }
                    }
                }
                (ArrayStorage::InlineF64(lhs), ArrayStorage::Boxed(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        let Some(value) = b.as_f64_fast() else {
                            return BoolBox::new(false);
                        };
                        if (*a - value).abs() >= f64::EPSILON {
                            return BoolBox::new(false);
                        }
                    }
                }
                (ArrayStorage::Boxed(lhs), ArrayStorage::InlineI64(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        if a.as_i64_fast() != Some(*b) {
                            return BoolBox::new(false);
                        }
                    }
                }
                (ArrayStorage::Boxed(lhs), ArrayStorage::InlineBool(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        if a.as_bool_fast() != Some(*b) {
                            return BoolBox::new(false);
                        }
                    }
                }
                (ArrayStorage::Boxed(lhs), ArrayStorage::InlineF64(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        let Some(value) = a.as_f64_fast() else {
                            return BoolBox::new(false);
                        };
                        if (value - *b).abs() >= f64::EPSILON {
                            return BoolBox::new(false);
                        }
                    }
                }
                (ArrayStorage::Boxed(lhs), ArrayStorage::Boxed(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        if !a.equals(b.as_ref()).value {
                            return BoolBox::new(false);
                        }
                    }
                }
                (ArrayStorage::InlineI64(_), ArrayStorage::InlineBool(_))
                | (ArrayStorage::InlineBool(_), ArrayStorage::InlineI64(_))
                | (ArrayStorage::InlineI64(_), ArrayStorage::InlineF64(_))
                | (ArrayStorage::InlineF64(_), ArrayStorage::InlineI64(_))
                | (ArrayStorage::InlineBool(_), ArrayStorage::InlineF64(_))
                | (ArrayStorage::InlineF64(_), ArrayStorage::InlineBool(_)) => {
                    return BoolBox::new(false);
                }
            }

            BoolBox::new(true)
        } else {
            BoolBox::new(false)
        }
    }
}

// Debug implementation for ArrayBox
impl std::fmt::Debug for ArrayBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let items = self.items.read();
        let storage_kind = match &*items {
            ArrayStorage::Boxed(_) => "boxed",
            ArrayStorage::InlineI64(_) => "inline_i64",
            ArrayStorage::InlineBool(_) => "inline_bool",
            ArrayStorage::InlineF64(_) => "inline_f64",
        };
        f.debug_struct("ArrayBox")
            .field("id", &self.base.id)
            .field("length", &items.len())
            .field("storage", &storage_kind)
            .finish()
    }
}

#[cfg(test)]
impl ArrayBox {
    pub fn uses_inline_i64_slots(&self) -> bool {
        matches!(&*self.items.read(), ArrayStorage::InlineI64(_))
    }

    pub fn uses_inline_bool_slots(&self) -> bool {
        matches!(&*self.items.read(), ArrayStorage::InlineBool(_))
    }

    pub fn uses_inline_f64_slots(&self) -> bool {
        matches!(&*self.items.read(), ArrayStorage::InlineF64(_))
    }
}
