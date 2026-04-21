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
                        (
                            f(value.as_mut_string()),
                            ArrayTextSlotSessionOutcomeKind::PromotedTextLane,
                        )
                    });
                }
                unreachable!("boxed text lane promoted to text");
            }
        }

        match self.storage {
            ArrayStorage::Text(values) => values.get_mut(idx).map(|value| {
                (
                    f(value.as_mut_string()),
                    ArrayTextSlotSessionOutcomeKind::ResidentText,
                )
            }),
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
                values[idx] = ArrayTextCell::from(value);
                true
            } else if idx == values.len() {
                values.push(ArrayTextCell::from(value));
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
            ArrayStorage::Text(values) => values.get(idx).map(|value| value.with_text(f)),
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

    /// Execute the MIR-owned len-half insert-mid edit through the text-cell
    /// boundary. This keeps the hot operation from treating flat `String` as
    /// the long-term array text representation.
    #[inline(always)]
    pub fn slot_insert_const_mid_lenhalf_raw(&self, idx: i64, middle: &str) -> Option<i64> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();

        if let ArrayStorage::Boxed(boxed) = &*items {
            if let Some(values) = Self::try_text_values(boxed) {
                *items = ArrayStorage::Text(values);
            }
        }

        if let ArrayStorage::Text(values) = &mut *items {
            return values
                .get_mut(idx)
                .map(|value| value.insert_const_mid_lenhalf(middle));
        }

        let mut session =
            ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible);
        session
            .update(idx, |value| {
                ArrayTextCell::insert_const_mid_lenhalf_string(value, middle)
            })
            .map(|(out, _kind)| out)
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
        if loop_bound == 0 {
            return Some(0);
        }
        let mut items = self.items.write();

        if let ArrayStorage::Boxed(boxed) = &*items {
            if let Some(values) = Self::try_text_values(boxed) {
                *items = ArrayStorage::Text(values);
            }
        }

        if let ArrayStorage::Text(values) = &mut *items {
            let mut total = 0_i64;
            for step in 0..loop_bound {
                let idx = (step % row_modulus) as usize;
                let value = values.get_mut(idx)?;
                total = total.checked_add(f(value.as_mut_string())?)?;
            }
            return Some(total);
        }

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

    /// Runtime-private observer/store executor for a MIR-proven text-cell region.
    /// The write guard stays inside this call; legality and publication stay MIR-owned.
    #[inline(always)]
    pub fn slot_text_indexof_suffix_store_region_raw(
        &self,
        loop_bound: i64,
        needle: &str,
        suffix: &str,
    ) -> Option<i64> {
        if loop_bound < 0 {
            return None;
        }
        let loop_bound = usize::try_from(loop_bound).ok()?;
        if loop_bound == 0 {
            return Some(0);
        }
        let mut items = self.items.write();

        if let ArrayStorage::Boxed(boxed) = &*items {
            if let Some(values) = Self::try_text_values(boxed) {
                *items = ArrayStorage::Text(values);
            }
        }

        if let ArrayStorage::Text(values) = &mut *items {
            if loop_bound > values.len() {
                return None;
            }
            let mut stores = 0_i64;
            for value in values.iter_mut().take(loop_bound) {
                if value.contains_literal(needle) {
                    value.append_suffix(suffix);
                    stores += 1;
                }
            }
            return Some(stores);
        }

        let boxed_len = match &*items {
            ArrayStorage::Boxed(boxed) => boxed.len(),
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => return None,
            ArrayStorage::Text(_) => unreachable!("text storage returned above"),
        };
        if loop_bound > boxed_len {
            return None;
        }

        let mut session =
            ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible);
        let mut stores = 0_i64;
        for idx in 0..loop_bound {
            let (hit, _kind) = session.update(idx, |value| {
                if ArrayTextCell::string_contains_literal(value, needle) {
                    ArrayTextCell::append_suffix_to_string(value, suffix);
                    1_i64
                } else {
                    0_i64
                }
            })?;
            stores += hit;
        }
        Some(stores)
    }

    /// Runtime-private combined executor for a MIR-proven outer edit region.
    /// The write guard stays inside this call; MIR owns ordering and legality.
    #[inline(always)]
    pub fn slot_text_lenhalf_insert_mid_periodic_indexof_suffix_region_raw(
        &self,
        loop_bound: i64,
        row_modulus: i64,
        middle: &str,
        observer_period: i64,
        observer_bound: i64,
        needle: &str,
        suffix: &str,
    ) -> Option<i64> {
        if loop_bound < 0 || row_modulus <= 0 || observer_period <= 0 || observer_bound < 0 {
            return None;
        }
        if loop_bound == 0 {
            return Some(0);
        }
        let loop_bound = usize::try_from(loop_bound).ok()?;
        let row_modulus = usize::try_from(row_modulus).ok()?;
        let observer_period = usize::try_from(observer_period).ok()?;
        let observer_bound = usize::try_from(observer_bound).ok()?;
        let row_modulus_mask = row_modulus.is_power_of_two().then_some(row_modulus - 1);
        let observer_period_mask = observer_period
            .is_power_of_two()
            .then_some(observer_period - 1);
        let mut items = self.items.write();

        if let ArrayStorage::Boxed(boxed) = &*items {
            if let Some(values) = Self::try_text_values(boxed) {
                *items = ArrayStorage::Text(values);
            }
        }

        if let ArrayStorage::Text(values) = &mut *items {
            if row_modulus > values.len() || observer_bound > values.len() {
                return None;
            }
            for step in 0..loop_bound {
                let idx = match row_modulus_mask {
                    Some(mask) => step & mask,
                    None => step % row_modulus,
                };
                values.get_mut(idx)?.insert_const_mid_lenhalf(middle);
                let should_observe = match observer_period_mask {
                    Some(mask) => step & mask == 0,
                    None => step % observer_period == 0,
                };
                if should_observe {
                    for value in values.iter_mut().take(observer_bound) {
                        if value.contains_literal(needle) {
                            value.append_suffix(suffix);
                        }
                    }
                }
            }
            return i64::try_from(loop_bound).ok();
        }

        let boxed_len = match &*items {
            ArrayStorage::Boxed(boxed) => boxed.len(),
            ArrayStorage::InlineI64(_)
            | ArrayStorage::InlineBool(_)
            | ArrayStorage::InlineF64(_) => return None,
            ArrayStorage::Text(_) => unreachable!("text storage returned above"),
        };
        if row_modulus > boxed_len || observer_bound > boxed_len {
            return None;
        }

        let mut session =
            ArrayTextSlotSession::new(&mut items, ArrayTextSlotSessionMode::Compatible);
        for step in 0..loop_bound {
            let idx = match row_modulus_mask {
                Some(mask) => step & mask,
                None => step % row_modulus,
            };
            session.update(idx, |value| {
                ArrayTextCell::insert_const_mid_lenhalf_string(value, middle)
            })?;
            let should_observe = match observer_period_mask {
                Some(mask) => step & mask == 0,
                None => step % observer_period == 0,
            };
            if should_observe {
                for idx in 0..observer_bound {
                    session.update(idx, |value| {
                        if ArrayTextCell::string_contains_literal(value, needle) {
                            ArrayTextCell::append_suffix_to_string(value, suffix);
                        }
                        0_i64
                    })?;
                }
            }
        }
        i64::try_from(loop_bound).ok()
    }
}
