//! EdgeCFG Fragment API（Phase 264: 入口SSOT）
//!
//! # コア概念
//! - [`ExitKind`]: 脱出種別（一次概念）
//! - [`EdgeStub`]: 未配線エッジ
//! - [`Frag`]: CFG断片
//! - [`compose`]: 合成関数群（Phase 264: TODO実装、pub(crate)）
//!
//! # 設計原則
//! - ExitKind を一次概念にし、pattern番号は「形の認識」までに縮退
//! - 値の合流は EdgeCFG の block params + edge-args で表す（PHI/推測/メタに逃げない）
//! - Fail-Fast: verify で不変条件を早期検証
//!
//! # 関連文書
//! - 北極星設計: `docs/development/current/main/design/edgecfg-fragments.md`
//! - EdgeCFG 基盤: `docs/development/current/main/design/join-explicit-cfg-construction.md`

pub mod block_params;
pub mod branch_stub; // Phase 267 P0: 追加
pub mod compose;
pub mod edge_stub;
pub mod emit; // Phase 266: 追加
pub mod exit_kind;
pub mod frag;
pub mod frag_emit_session;
pub mod verify; // Phase 29bq+: sealing 層中立化

// 公開型（安定）
pub use block_params::BlockParams;
pub use branch_stub::BranchStub;
pub use edge_stub::EdgeStub;
pub use exit_kind::ExitKind;
pub use frag::Frag; // Phase 267 P0: 追加

// 合成関数（Phase 264: crate内のみ公開、Phase 265+でpub化）

// 検証関数
// Phase 266: strict 版追加

// Phase 29bq+: sealing 層中立化
pub use frag_emit_session::FragEmitSession;
