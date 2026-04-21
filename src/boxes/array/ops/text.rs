use super::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArrayTextSlotSessionMode {
    ResidentOnly,
    Compatible,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ArrayTextSlotSessionOutcomeKind {
    ResidentText,
    PromotedTextLane,
    BoxedString,
    BoxedStringLike,
}

struct ArrayTextSlotSession<'a> {
    storage: &'a mut ArrayStorage,
    mode: ArrayTextSlotSessionMode,
}

impl<'a> ArrayTextSlotSession<'a> {
    #[inline(always)]
    fn new(storage: &'a mut ArrayStorage, mode: ArrayTextSlotSessionMode) -> Self {
        Self { storage, mode }
    }

    #[inline(always)]
    fn update<R>(
        &mut self,
        idx: usize,
        f: impl FnOnce(&mut String) -> R,
    ) -> Option<(R, ArrayTextSlotSessionOutcomeKind)> {
        if self.mode == ArrayTextSlotSessionMode::Compatible {
            let promoted = match &*self.storage {
                ArrayStorage::Boxed(items) => ArrayBox::try_text_values(items),
                ArrayStorage::Text(_)
                | ArrayStorage::InlineI64(_)
                | ArrayStorage::InlineBool(_)
                | ArrayStorage::InlineF64(_) => None,
            };
            if let Some(values) = promoted {
                *self.storage = ArrayStorage::Text(values);
                if let ArrayStorage::Text(values) = self.storage {
                    return values.get_mut(idx).map(|value| {
                        (f(value), ArrayTextSlotSessionOutcomeKind::PromotedTextLane)
                    });
                }
                unreachable!("boxed text lane promoted to text");
            }
        }

        match self.storage {
            ArrayStorage::Text(values) => values
                .get_mut(idx)
                .map(|value| (f(value), ArrayTextSlotSessionOutcomeKind::ResidentText)),
            ArrayStorage::Boxed(_) if self.mode == ArrayTextSlotSessionMode::ResidentOnly => None,
            ArrayStorage::Boxed(items) => {
                let item = items.get_mut(idx)?;
                if let Some(value) = item.as_any_mut().downcast_mut::<StringBox>() {
                    return Some((
                        f(&mut value.value),
                        ArrayTextSlotSessionOutcomeKind::BoxedString,
                    ));
                }
                let mut value = item.as_str_fast().map(str::to_owned)?;
                let out = f(&mut value);
                *item = Box::new(StringBox::new(value));
                Some((out, ArrayTextSlotSessionOutcomeKind::BoxedStringLike))
            }
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => None,
        }
    }
}

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
        ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::ResidentOnly)
            .update(idx, f)
            .map(|(out, _kind)| out)
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
        ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible)
            .update(idx, f)
            .map(|(out, _kind)| out)
    }

    /// Mutate a text slot through the compatible substrate and report whether
    /// the slot was already text-resident at session entry.
    #[inline(always)]
    pub fn slot_update_text_resident_first_raw<R>(
        &self,
        idx: i64,
        f: impl FnOnce(&mut String) -> R,
    ) -> Option<(R, bool)> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible)
            .update(idx, f)
            .map(|(out, kind)| {
                (
                    out,
                    matches!(kind, ArrayTextSlotSessionOutcomeKind::ResidentText),
                )
            })
    }

    /// Runtime-private repeated text-cell update for a MIR-proven loop region.
    /// The write guard stays inside this call; legality and loop shape stay MIR-owned.
    #[inline(always)]
    pub fn slot_text_region_update_sum_raw(
        &self,
        loop_bound: i64,
        row_modulus: i64,
        mut f: impl FnMut(&mut String) -> Option<i64>,
    ) -> Option<i64> {
        if loop_bound < 0 || row_modulus <= 0 {
            return None;
        }
        let mut items = self.items.write();
        let mut session =
            ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible);
        let mut total = 0_i64;
        for step in 0..loop_bound {
            let idx = (step % row_modulus) as usize;
            let (delta, _kind) = session.update(idx, |value| f(value))?;
            total = total.checked_add(delta?)?;
        }
        Some(total)
    }
}
