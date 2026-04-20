use super::*;

impl ArrayBox {
    /// Raw text store helper for runtime-private array string lanes.
    /// Public Array semantics stay object-based; mixed arrays degrade to Boxed.
    #[inline(always)]
    pub fn slot_store_text_raw(&self, idx: i64, value: String) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_text(&mut items) {
            if idx < values.len() {
                values[idx] = value;
                true
            } else if idx == values.len() {
                values.push(value);
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        } else {
            let boxed = Self::ensure_boxed(&mut items);
            let value = Box::new(StringBox::new(value)) as Box<dyn NyashBox>;
            if idx < boxed.len() {
                boxed[idx] = value;
                true
            } else if idx == boxed.len() {
                boxed.push(value);
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        }
    }

    /// Read a text slot without materializing a public StringBox.
    #[inline(always)]
    pub fn slot_with_text_raw<R>(&self, idx: i64, f: impl FnOnce(&str) -> R) -> Option<R> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let items = self.items.read();
        match &*items {
            ArrayStorage::Text(values) => values.get(idx).map(|value| f(value.as_str())),
            ArrayStorage::Boxed(items) => items.get(idx).and_then(|item| item.as_str_fast().map(f)),
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => None,
        }
    }

    /// Read a text slot length without materializing a public StringBox.
    #[inline(always)]
    pub fn slot_text_len_raw(&self, idx: i64) -> Option<i64> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let items = self.items.read();
        match &*items {
            ArrayStorage::Text(values) => values.get(idx).map(|value| value.len() as i64),
            ArrayStorage::Boxed(items) => items
                .get(idx)
                .and_then(|item| item.as_str_fast().map(|value| value.len() as i64)),
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => None,
        }
    }

    /// Mutate a text slot only when storage is already text-resident.
    /// This does not promote boxed arrays; callers that need full ArrayBox compatibility
    /// must fall back to `slot_update_text_raw`.
    #[inline(always)]
    pub fn slot_update_text_resident_raw<R>(
        &self,
        idx: i64,
        f: impl FnOnce(&mut String) -> R,
    ) -> Option<R> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Text(values) => values.get_mut(idx).map(f),
            ArrayStorage::Boxed(_)
            | ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => None,
        }
    }

    /// Mutate a text slot in-place when the array is text-resident.
    /// If the array is mixed but the target slot is string-like, only that slot is materialized.
    #[inline(always)]
    pub fn slot_update_text_raw<R>(&self, idx: i64, f: impl FnOnce(&mut String) -> R) -> Option<R> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_text(&mut items) {
            return values.get_mut(idx).map(f);
        }
        match &mut *items {
            ArrayStorage::Boxed(items) => {
                let item = items.get_mut(idx)?;
                if let Some(value) = item.as_any_mut().downcast_mut::<StringBox>() {
                    return Some(f(&mut value.value));
                }
                let mut value = item.as_str_fast().map(str::to_owned)?;
                let out = f(&mut value);
                *item = Box::new(StringBox::new(value));
                Some(out)
            }
            ArrayStorage::Text(_)
            | ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => None,
        }
    }
}
