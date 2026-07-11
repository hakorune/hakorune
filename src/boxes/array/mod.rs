//! ArrayBox 📦 - 配列・リスト操作
//! Nyashの箱システムによる配列・リスト操作を提供します。
//! RwLockパターンで内部可変性を実現（Phase 9.75-B Arc<Mutex>削除）

use crate::box_trait::{BoolBox, BoxBase, BoxCore, IntegerBox, NyashBox, StringBox};
use crate::boxes::FloatBox;
use crate::config::env;
use parking_lot::RwLock;
use std::any::Any;
use std::fmt::Display;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;

#[cfg(test)]
mod inline_record_plan_probe;
#[cfg(test)]
mod inline_record_probe;
mod ops;
mod storage;
mod surface_catalog;
#[cfg(test)]
mod tests;
mod text_cell;
mod traits;

#[cfg(test)]
use inline_record_plan_probe::ArrayInlineRecordPlanProbe;
#[cfg(test)]
use inline_record_probe::ArrayInlineRecordProbe;
#[cfg(test)]
use storage::ArrayInlineRecordColumn;
use storage::{ArrayInlineRecordStorage, ArrayStorage};
pub use surface_catalog::{
    ArrayExposureState, ArrayMethodId, ArrayMethodSpec, ArraySurfaceEffect,
    ArraySurfaceInvokeError, ArraySurfaceInvokeResult, ArraySurfaceReturn, ARRAY_SURFACE_METHODS,
};
use text_cell::ArrayTextCell;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ArrayStateIdentity(u64);

struct ArrayStateCell {
    identity: ArrayStateIdentity,
    storage: RwLock<ArrayStorage>,
}

impl ArrayStateCell {
    fn new(storage: ArrayStorage) -> Self {
        static NEXT_ID: AtomicU64 = AtomicU64::new(1);
        Self {
            identity: ArrayStateIdentity(NEXT_ID.fetch_add(1, Ordering::Relaxed)),
            storage: RwLock::new(storage),
        }
    }
}

impl std::ops::Deref for ArrayStateCell {
    type Target = RwLock<ArrayStorage>;

    fn deref(&self) -> &Self::Target {
        &self.storage
    }
}

pub struct ArrayBox {
    items: Arc<ArrayStateCell>,
    base: BoxBase,
}

impl ArrayBox {
    pub(crate) fn state_identity(&self) -> ArrayStateIdentity {
        self.items.identity
    }
}
