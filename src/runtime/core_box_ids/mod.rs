//! Phase 87: Core Box ID 定義
//!
//! Nyash の core Box を型安全な enum で管理する。
//! ハードコード文字列からの脱却により、コンパイル時検証を実現。

mod box_id;
mod method_id;
pub(super) mod specs;

pub use box_id::{CoreBoxCategory, CoreBoxId};
pub use method_id::CoreMethodId;
