use super::*;

pub(super) enum ArrayStorage {
    Boxed(Vec<Box<dyn NyashBox>>),
    InlineI64(Vec<i64>),
    InlineBool(Vec<bool>),
    InlineF64(Vec<f64>),
}

impl ArrayStorage {
    pub(super) fn len(&self) -> usize {
        match self {
            Self::Boxed(items) => items.len(),
            Self::InlineI64(values) => values.len(),
            Self::InlineBool(values) => values.len(),
            Self::InlineF64(values) => values.len(),
        }
    }

    pub(super) fn capacity(&self) -> usize {
        match self {
            Self::Boxed(items) => items.capacity(),
            Self::InlineI64(values) => values.capacity(),
            Self::InlineBool(values) => values.capacity(),
            Self::InlineF64(values) => values.capacity(),
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

    pub(super) fn try_inline_i64_values(items: &[Box<dyn NyashBox>]) -> Option<Vec<i64>> {
        items.iter().map(|item| item.as_i64_fast()).collect()
    }

    pub(super) fn try_inline_bool_values(items: &[Box<dyn NyashBox>]) -> Option<Vec<bool>> {
        items.iter().map(|item| item.as_bool_fast()).collect()
    }

    pub(super) fn try_inline_f64_values(items: &[Box<dyn NyashBox>]) -> Option<Vec<f64>> {
        items.iter().map(|item| item.as_f64_fast()).collect()
    }

    pub(super) fn ensure_boxed(storage: &mut ArrayStorage) -> &mut Vec<Box<dyn NyashBox>> {
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
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => {
                unreachable!("inline storage promoted to boxed")
            }
        }
    }

    pub(super) fn ensure_inline_i64(storage: &mut ArrayStorage) -> Option<&mut Vec<i64>> {
        if let ArrayStorage::Boxed(items) = storage {
            let values = Self::try_inline_i64_values(items)?;
            *storage = ArrayStorage::InlineI64(values);
        }
        match storage {
            ArrayStorage::InlineI64(values) => Some(values),
            ArrayStorage::Boxed(_) | ArrayStorage::InlineBool(_) | ArrayStorage::InlineF64(_) => {
                None
            }
        }
    }

    pub(super) fn ensure_inline_bool(storage: &mut ArrayStorage) -> Option<&mut Vec<bool>> {
        if let ArrayStorage::Boxed(items) = storage {
            let values = Self::try_inline_bool_values(items)?;
            *storage = ArrayStorage::InlineBool(values);
        }
        match storage {
            ArrayStorage::InlineBool(values) => Some(values),
            ArrayStorage::Boxed(_) | ArrayStorage::InlineI64(_) | ArrayStorage::InlineF64(_) => {
                None
            }
        }
    }

    pub(super) fn ensure_inline_f64(storage: &mut ArrayStorage) -> Option<&mut Vec<f64>> {
        if let ArrayStorage::Boxed(items) = storage {
            let values = Self::try_inline_f64_values(items)?;
            *storage = ArrayStorage::InlineF64(values);
        }
        match storage {
            ArrayStorage::InlineF64(values) => Some(values),
            ArrayStorage::Boxed(_) | ArrayStorage::InlineI64(_) | ArrayStorage::InlineBool(_) => {
                None
            }
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
}
