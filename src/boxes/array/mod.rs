//! ArrayBox 📦 - 配列・リスト操作
// Nyashの箱システムによる配列・リスト操作を提供します。
// RwLockパターンで内部可変性を実現（Phase 9.75-B Arc<Mutex>削除）

use crate::box_trait::{BoolBox, BoxBase, BoxCore, IntegerBox, NyashBox, StringBox};
use crate::config::env;
use parking_lot::RwLock;
use std::any::Any;
use std::fmt::Display;
use std::sync::Arc;

enum ArrayStorage {
    Boxed(Vec<Box<dyn NyashBox>>),
    InlineI64(Vec<i64>),
}

impl ArrayStorage {
    fn len(&self) -> usize {
        match self {
            Self::Boxed(items) => items.len(),
            Self::InlineI64(values) => values.len(),
        }
    }

    fn capacity(&self) -> usize {
        match self {
            Self::Boxed(items) => items.capacity(),
            Self::InlineI64(values) => values.capacity(),
        }
    }
}

pub struct ArrayBox {
    items: Arc<RwLock<ArrayStorage>>,
    base: BoxBase,
}

impl ArrayBox {
    fn oob_strict_enabled() -> bool {
        env::env_bool("HAKO_OOB_STRICT") || env::env_bool("NYASH_OOB_STRICT")
    }

    fn new_with_storage(storage: ArrayStorage) -> Self {
        ArrayBox {
            items: Arc::new(RwLock::new(storage)),
            base: BoxBase::new(),
        }
    }

    fn boxed_from_inline(values: &[i64]) -> Vec<Box<dyn NyashBox>> {
        values
            .iter()
            .map(|value| Box::new(IntegerBox::new(*value)) as Box<dyn NyashBox>)
            .collect()
    }

    fn try_inline_i64_values(items: &[Box<dyn NyashBox>]) -> Option<Vec<i64>> {
        items.iter().map(|item| item.as_i64_fast()).collect()
    }

    fn ensure_boxed(storage: &mut ArrayStorage) -> &mut Vec<Box<dyn NyashBox>> {
        if let ArrayStorage::InlineI64(values) = storage {
            *storage = ArrayStorage::Boxed(Self::boxed_from_inline(values));
        }
        match storage {
            ArrayStorage::Boxed(items) => items,
            ArrayStorage::InlineI64(_) => unreachable!("inline storage promoted to boxed"),
        }
    }

    fn ensure_inline_i64(storage: &mut ArrayStorage) -> Option<&mut Vec<i64>> {
        if let ArrayStorage::Boxed(items) = storage {
            let values = Self::try_inline_i64_values(items)?;
            *storage = ArrayStorage::InlineI64(values);
        }
        match storage {
            ArrayStorage::InlineI64(values) => Some(values),
            ArrayStorage::Boxed(_) => None,
        }
    }

    fn clone_visible_item(item: &dyn NyashBox) -> Box<dyn NyashBox> {
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

    fn format_inline_values(values: &[i64]) -> String {
        values
            .iter()
            .map(|value| value.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    }

    /// 新しいArrayBoxを作成
    pub fn new() -> Self {
        Self::new_with_storage(ArrayStorage::Boxed(Vec::new()))
    }

    /// 要素を持つArrayBoxを作成
    pub fn new_with_elements(elements: Vec<Box<dyn NyashBox>>) -> Self {
        Self::new_with_storage(ArrayStorage::Boxed(elements))
    }

    fn new_with_inline_i64_elements(values: Vec<i64>) -> Self {
        Self::new_with_storage(ArrayStorage::InlineI64(values))
    }

    #[inline(always)]
    pub fn with_items_read<R>(&self, f: impl FnOnce(&Vec<Box<dyn NyashBox>>) -> R) -> R {
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => f(items),
            ArrayStorage::InlineI64(values) => {
                let materialized = Self::boxed_from_inline(values);
                f(&materialized)
            }
        }
    }

    #[inline(always)]
    pub fn with_items_write<R>(&self, f: impl FnOnce(&mut Vec<Box<dyn NyashBox>>) -> R) -> R {
        let mut items = self.items.write();
        let boxed = Self::ensure_boxed(&mut items);
        f(boxed)
    }

    #[inline(always)]
    pub fn capacity(&self) -> usize {
        self.items.read().capacity()
    }

    /// 要素を追加
    pub fn push(&self, item: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        let _ = self.slot_append_box_raw(item);
        Box::new(StringBox::new("ok"))
    }

    /// Raw append helper for substrate/plugin routes.
    /// Visible `push()` semantics stay above this seam.
    pub fn slot_append_box_raw(&self, item: Box<dyn NyashBox>) -> i64 {
        let mut items = self.items.write();
        let boxed = Self::ensure_boxed(&mut items);
        boxed.push(item);
        boxed.len() as i64
    }

    /// Raw reserve helper for substrate/plugin routes.
    /// Keeps visible owner semantics above the capacity seam.
    pub fn slot_reserve_capacity_raw(&self, additional: usize) -> bool {
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Boxed(items) => items.reserve(additional),
            ArrayStorage::InlineI64(values) => values.reserve(additional),
        }
        true
    }

    /// Raw grow helper for substrate/plugin routes.
    /// Keeps visible owner semantics above the capacity seam.
    pub fn slot_grow_capacity_raw(&self, target_capacity: usize) -> bool {
        let mut items = self.items.write();
        let current_capacity = items.capacity();
        if current_capacity < target_capacity {
            let needed = target_capacity.saturating_sub(items.len());
            if needed > 0 {
                match &mut *items {
                    ArrayStorage::Boxed(items) => items.reserve(needed),
                    ArrayStorage::InlineI64(values) => values.reserve(needed),
                }
            }
        }
        true
    }

    /// 最後の要素を取り出す
    pub fn pop(&self) -> Box<dyn NyashBox> {
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Boxed(items) => match items.pop() {
                Some(item) => item,
                None => {
                    if Self::oob_strict_enabled() {
                        Box::new(StringBox::new("[array/empty/pop] empty array"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::InlineI64(values) => match values.pop() {
                Some(value) => Box::new(IntegerBox::new(value)),
                None => {
                    if Self::oob_strict_enabled() {
                        Box::new(StringBox::new("[array/empty/pop] empty array"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
        }
    }

    /// 要素数を取得
    pub fn length(&self) -> Box<dyn NyashBox> {
        Box::new(IntegerBox::new(self.items.read().len() as i64))
    }

    /// size() エイリアス（length と同義）
    pub fn size(&self) -> Box<dyn NyashBox> {
        self.length()
    }

    /// Rust向けヘルパー: 要素数をusizeで取得（テスト用）
    pub fn len(&self) -> usize {
        self.items.read().len()
    }

    #[inline(always)]
    pub fn slot_load_i64_raw(&self, idx: i64) -> Option<i64> {
        if idx < 0 {
            return None;
        }
        let idx = idx as usize;
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => items.get(idx).and_then(|item| item.as_i64_fast()),
            ArrayStorage::InlineI64(values) => values.get(idx).copied(),
        }
    }

    /// インデックス(i64)で要素を取得（FFI/Kernel hot path 用）
    pub fn get_index_i64(&self, idx: i64) -> Box<dyn NyashBox> {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
                return Box::new(StringBox::new("[oob/array/get] index out of bounds"));
            }
            return Box::new(crate::boxes::null_box::NullBox::new());
        }
        let idx = idx as usize;
        let items = self.items.read();
        match &*items {
            ArrayStorage::Boxed(items) => match items.get(idx) {
                Some(item) => Self::clone_visible_item(item.as_ref()),
                None => {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                        Box::new(StringBox::new("[oob/array/get] index out of bounds"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
            ArrayStorage::InlineI64(values) => match values.get(idx) {
                Some(value) => Box::new(IntegerBox::new(*value)),
                None => {
                    if Self::oob_strict_enabled() {
                        crate::runtime::observe::mark_oob();
                        Box::new(StringBox::new("[oob/array/get] index out of bounds"))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            },
        }
    }

    /// インデックス(i64)で要素を設定し、成功可否を返す（FFI/Kernel hot path 用）
    pub fn try_set_index_i64(&self, idx: i64, value: Box<dyn NyashBox>) -> bool {
        self.slot_store_box_raw(idx, value)
    }

    /// Raw store helper for substrate/plugin routes.
    /// Keeps the current append-at-end and OOB policy while visible `set()` stays above this seam.
    pub fn slot_store_box_raw(&self, idx: i64, value: Box<dyn NyashBox>) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        let boxed = Self::ensure_boxed(&mut items);
        if idx < boxed.len() {
            boxed[idx] = value;
            true
        } else if idx == boxed.len() {
            // Pragmatic semantics: allow set at exact end to append
            boxed.push(value);
            true
        } else {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            false
        }
    }

    /// インデックス(i64)へ整数値を設定し、成功可否を返す（AOT integer route 用）
    #[inline(always)]
    pub fn try_set_index_i64_integer(&self, idx: i64, value: i64) -> bool {
        self.slot_store_i64_raw(idx, value)
    }

    /// Raw integer store helper for substrate/plugin routes.
    /// Keeps the current append-at-end / rebox policy while visible `set()` stays above this seam.
    #[inline(always)]
    pub fn slot_store_i64_raw(&self, idx: i64, value: i64) -> bool {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return false;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_inline_i64(&mut items) {
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
            if idx < boxed.len() {
                if let Some(slot) = boxed[idx].i64_slot_mut() {
                    *slot = value;
                } else {
                    boxed[idx] = Box::new(IntegerBox::new(value));
                }
                true
            } else if idx == boxed.len() {
                boxed.push(Box::new(IntegerBox::new(value)));
                true
            } else {
                if Self::oob_strict_enabled() {
                    crate::runtime::observe::mark_oob();
                }
                false
            }
        }
    }

    /// Raw integer read-modify-write helper for substrate/plugin routes.
    /// Returns the updated value when the slot exists and can be treated as `i64`.
    #[inline(always)]
    pub fn slot_rmw_add1_i64_raw(&self, idx: i64) -> Option<i64> {
        if idx < 0 {
            if Self::oob_strict_enabled() {
                crate::runtime::observe::mark_oob();
            }
            return None;
        }
        let idx = idx as usize;
        let mut items = self.items.write();
        if let Some(values) = Self::ensure_inline_i64(&mut items) {
            let slot = values.get_mut(idx)?;
            *slot = slot.checked_add(1)?;
            return Some(*slot);
        }
        let boxed = Self::ensure_boxed(&mut items);
        let item = boxed.get_mut(idx)?;
        if let Some(slot) = item.i64_slot_mut() {
            *slot += 1;
            return Some(*slot);
        }
        let next = item.as_i64_fast()?.checked_add(1)?;
        *item = Box::new(IntegerBox::new(next));
        Some(next)
    }

    /// インデックス(i64)が配列範囲内かを返す（副作用なし）
    pub fn has_index_i64(&self, idx: i64) -> bool {
        if idx < 0 {
            return false;
        }
        (idx as usize) < self.items.read().len()
    }

    /// インデックス(i64)で要素を設定
    pub fn set_index_i64(&self, idx: i64, value: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if self.try_set_index_i64(idx, value) {
            Box::new(StringBox::new("ok"))
        } else if Self::oob_strict_enabled() {
            Box::new(StringBox::new("[oob/array/set] index out of bounds"))
        } else {
            Box::new(StringBox::new("Error: index out of bounds"))
        }
    }

    /// インデックスで要素を取得
    pub fn get(&self, index: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(idx_box) = index.as_any().downcast_ref::<IntegerBox>() {
            self.get_index_i64(idx_box.value)
        } else {
            Box::new(StringBox::new("Error: get() requires integer index"))
        }
    }

    /// インデックスで要素を設定
    pub fn set(&self, index: Box<dyn NyashBox>, value: Box<dyn NyashBox>) -> Box<dyn NyashBox> {
        if let Some(idx_box) = index.as_any().downcast_ref::<IntegerBox>() {
            self.set_index_i64(idx_box.value, value)
        } else {
            Box::new(StringBox::new("Error: set() requires integer index"))
        }
    }

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
                ArrayStorage::InlineI64(values) => {
                    if idx < values.len() {
                        Box::new(IntegerBox::new(values.remove(idx)))
                    } else {
                        Box::new(crate::boxes::null_box::NullBox::new())
                    }
                }
            }
        } else {
            Box::new(StringBox::new("Error: remove() requires integer index"))
        }
    }

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
        }
        Box::new(BoolBox::new(false))
    }

    /// 配列を空にする
    pub fn clear(&self) -> Box<dyn NyashBox> {
        let mut items = self.items.write();
        match &mut *items {
            ArrayStorage::Boxed(items) => items.clear(),
            ArrayStorage::InlineI64(values) => values.clear(),
        }
        Box::new(StringBox::new("ok"))
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
        }
    }
}

// Clone implementation for ArrayBox (needed since RwLock doesn't auto-derive Clone)
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
            ArrayStorage::InlineI64(values) => write!(f, "[{}]", Self::format_inline_values(values)),
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
                (ArrayStorage::Boxed(lhs), ArrayStorage::InlineI64(rhs)) => {
                    for (a, b) in lhs.iter().zip(rhs.iter()) {
                        if a.as_i64_fast() != Some(*b) {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_store_i64_births_inline_lane() {
        let array = ArrayBox::new();
        assert!(array.slot_store_i64_raw(0, 10));
        assert!(array.uses_inline_i64_slots());
        assert_eq!(array.slot_load_i64_raw(0), Some(10));
        assert_eq!(array.get_index_i64(0).to_string_box().value, "10");
    }

    #[test]
    fn slot_store_box_promotes_inline_lane_to_boxed() {
        let array = ArrayBox::new();
        assert!(array.slot_store_i64_raw(0, 10));
        assert!(array.uses_inline_i64_slots());

        assert!(array.slot_store_box_raw(0, Box::new(StringBox::new("hello"))));
        assert!(!array.uses_inline_i64_slots());
        assert_eq!(array.get_index_i64(0).to_string_box().value, "hello");
        assert_eq!(array.slot_rmw_add1_i64_raw(0), None);
    }
}
