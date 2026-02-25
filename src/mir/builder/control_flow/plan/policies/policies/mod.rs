//! JoinIR Pattern Policies - パターン認識とルーティング決定
//!
//! ## 概要
//! このモジュールには、Pattern認識とルーティング（policy決定）を行う「箱」が格納されています。
//!
//! ## Policy箱の責務
//! - パターン認識: LoopSkeletonから特定のパターン（Trim, Escape, etc.）を検出
//! - ルーティング決定: 適用可能なLoweringパターンを決定
//! - Recipe生成: Pattern固有の情報（ConditionOnlyRecipe, BodyLocalDerivedRecipe, etc.）を生成
//!
//! ## 決定型のSSOT
//! - `PolicyDecision<T>` に統一（Use / Reject / None）
//! - BodyLocal, Trim, P5b escape などすべてここ経由で route することで Pattern2 側の分岐を簡潔に保つ
//!
//! ## 設計原則
//! - **単一判断の原則**: 各policy箱は1つのパターン判断のみ
//! - **非破壊的判断**: 入力を変更せず、Decision型で結果を返す
//! - **Fail-Fast**: パターンマッチング失敗は即座にReject/Noneを返す
//!
//! ## 将来の拡張
//! policies/ は「認識とルーティング決定（policy）」を分離する受け皿です。
//! Phase 94（P5b derived）から段階的に移設を開始しました。
//!
//! 詳細は [README.md](README.md) を参照してください。

pub use crate::mir::policies::PolicyDecision;

pub(in crate::mir::builder) mod p5b_escape_derived_policy;
pub(in crate::mir::builder) mod trim_policy;
pub(in crate::mir::builder) mod loop_true_read_digits_policy;
pub(in crate::mir::builder) mod balanced_depth_scan_policy;
pub(in crate::mir::builder) mod balanced_depth_scan_policy_box;
pub(in crate::mir::builder) mod post_loop_early_return_plan;
pub(in crate::mir::builder) mod normalized_shadow_suffix_router_box; // Phase 132 P0.5
