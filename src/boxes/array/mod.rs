//! ArrayBox 📦 - 配列・リスト操作
//! Nyashの箱システムによる配列・リスト操作を提供します。
//! RwLockパターンで内部可変性を実現（Phase 9.75-B Arc<Mutex>削除）

use crate::box_trait::{BoolBox, BoxBase, BoxCore, IntegerBox, NyashBox, StringBox};
use crate::boxes::FloatBox;
use crate::config::env;
use parking_lot::RwLock;
use std::any::Any;
use std::fmt::Display;
use std::sync::Arc;

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
use inline_record_probe::ArrayInlineRecordProbe;
#[cfg(test)]
use storage::ArrayInlineRecordColumn;
use storage::{ArrayInlineRecordStorage, ArrayStorage};
pub use surface_catalog::{
    ArrayExposureState, ArrayMethodId, ArrayMethodSpec, ArraySurfaceEffect,
    ArraySurfaceInvokeError, ArraySurfaceInvokeResult, ArraySurfaceReturn, ARRAY_SURFACE_METHODS,
};
use text_cell::ArrayTextCell;

pub struct ArrayBox {
    items: Arc<RwLock<ArrayStorage>>,
    base: BoxBase,
}
