/*!
 * ControlForm – 共通制御構造ビュー（Loop / If の箱化レイヤ）
 *
 * 目的:
 * - LoopForm v2（ループ）と If 降下を、1段上の「制御構造の形」として統一的に眺めるための薄いレイヤだよ。
 * - Conservative PHI Box や将来の可視化/検証ロジックが、Loop 専用 / If 専用に分かれず、
 *   ControlForm という SSOT から情報を取れるようにするのがねらいだよ。
 *
 * このモジュール自体は構造定義とデバッグ用のユーティリティのみを提供し、
 * 既存の LoopBuilder / If 降下の挙動は変えないよ（Phase 25.1f では観測レイヤ専用）。
 */

use crate::mir::{BasicBlock, BasicBlockId, MirFunction};
use crate::runtime::get_global_ring0;
pub use hakorune_mir_core::LoopId;

// ============================================================================
// Phase 32: 新しい ID 型（LoopRegion / LoopControlShape 用）
// ============================================================================


// ============================================================================
// Phase 32: LoopRegion - ブロック集合 + ネスト構造
// ============================================================================


// ============================================================================
// Phase 32: LoopControlShape - 制御フロー辺
// ============================================================================




// ============================================================================
// Phase 32 L-1.4: ExitGroup / ExitAnalysis（出口辺のグループ化）
// ============================================================================







/// ループ構造の形だけを表す箱だよ。
///
/// - `preheader`        : ループ直前のブロック（キャリア/ピン変数のコピー元）
/// - `header`           : ループヘッダ（条件判定と header PHI が置かれる）
/// - `body`             : 代表的なループ本体ブロック（最初の body など）
/// - `latch`            : ヘッダへ戻るバックエッジを張るブロック
/// - `exit`             : ループを抜けた先のブロック
/// - `continue_targets` : continue 文を含み、`header` へ遷移するブロック群（エッジの「出発点」）
/// - `break_targets`    : break 文を含み、`exit` へ遷移するブロック群（エッジの「出発点」）
#[derive(Debug, Clone)]
pub struct LoopShape {
    pub preheader: BasicBlockId,
    pub header: BasicBlockId,
    pub body: BasicBlockId,
    pub latch: BasicBlockId,
    pub exit: BasicBlockId,
    pub continue_targets: Vec<BasicBlockId>,
    pub break_targets: Vec<BasicBlockId>,
}

/// if/else 構造の形だけを表す箱だよ。
///
/// - `cond_block` : 条件式を評価するブロック
/// - `then_block` : then ブランチの先頭ブロック
/// - `else_block` : else ブランチの先頭ブロック（無ければ None）
/// - `merge_block`: then/else の合流ブロック
#[derive(Debug, Clone)]
pub struct IfShape {
    pub cond_block: BasicBlockId,
    pub then_block: BasicBlockId,
    pub else_block: Option<BasicBlockId>,
    pub merge_block: BasicBlockId,
}

/// 制御構造の種別だよ。
#[derive(Debug, Clone)]
pub enum ControlKind {
    Loop(LoopShape),
    If(IfShape),
}

/// ループ / if / 将来の switch などを、共通のビューとして扱う箱だよ。
///
/// - `entry` : 構造に入る入口ブロック
/// - `exits` : 構造を抜けたあとのブロック群
/// - `kind`  : Loop / If などの種別ごとの Shape
#[derive(Debug, Clone)]
pub struct ControlForm {
    pub entry: BasicBlockId,
    pub exits: Vec<BasicBlockId>,
    pub kind: ControlKind,
}


/// ControlForm の invariant を軽く検査するための CFG 抽象だよ。
///
/// 実装は MirFunction などに持たせて、`debug_validate` から使う想定。
pub trait CfgLike {
    fn has_edge(&self, from: BasicBlockId, to: BasicBlockId) -> bool;
    fn predecessors_len(&self, block: BasicBlockId) -> usize;
}

impl CfgLike for MirFunction {
    fn has_edge(&self, from: BasicBlockId, to: BasicBlockId) -> bool {
        self.blocks
            .get(&from)
            .map(|bb: &BasicBlock| bb.successors.contains(&to))
            .unwrap_or(false)
    }

    fn predecessors_len(&self, block: BasicBlockId) -> usize {
        self.blocks
            .get(&block)
            .map(|bb: &BasicBlock| bb.predecessors.len())
            .unwrap_or(0)
    }
}


impl LoopShape {
    // ========================================================================
    // Phase 32: View メソッド（段階移行用）
    // ========================================================================





    /// Debug ビルドでだけ呼ぶ用の簡易 invariant チェックだよ。
    ///
    /// - preheader → header にエッジがあること
    /// - latch → header にバックエッジがあること
    /// - continue_targets の各ブロックから header へのエッジがあること
    /// - break_targets の各ブロックから exit へのエッジがあること
    #[cfg(debug_assertions)]
    pub fn debug_validate<C: CfgLike>(&self, cfg: &C) {
        debug_assert!(
            cfg.has_edge(self.preheader, self.header),
            "LoopShape invalid: preheader -> header edge missing: {:?} -> {:?}",
            self.preheader,
            self.header
        );
        debug_assert!(
            cfg.has_edge(self.latch, self.header),
            "LoopShape invalid: latch -> header backedge missing: {:?} -> {:?}",
            self.latch,
            self.header
        );

        for ct in &self.continue_targets {
            debug_assert!(
                cfg.has_edge(*ct, self.header),
                "LoopShape invalid: continue source block {:?} does not branch to header {:?}",
                ct,
                self.header
            );
        }

        for bt in &self.break_targets {
            debug_assert!(
                cfg.has_edge(*bt, self.exit),
                "LoopShape invalid: break source block {:?} does not branch to exit {:?}",
                bt,
                self.exit
            );
        }
    }
}

impl IfShape {
    /// Debug ビルドでだけ呼ぶ用の簡易 invariant チェックだよ。
    ///
    /// - cond → then / else にエッジがあること
    /// - merge については、predecessor 情報がまだ配線途中のケースもあるので
    ///   ここでは「0 ならログだけ出す（panic しない）」ことにするよ。
    #[cfg(debug_assertions)]
    pub fn debug_validate<C: CfgLike>(&self, cfg: &C) {
        debug_assert!(
            cfg.has_edge(self.cond_block, self.then_block),
            "IfShape invalid: cond -> then edge missing: {:?} -> {:?}",
            self.cond_block,
            self.then_block
        );
        if let Some(else_blk) = self.else_block {
            debug_assert!(
                cfg.has_edge(self.cond_block, else_blk),
                "IfShape invalid: cond -> else edge missing: {:?} -> {:?}",
                self.cond_block,
                else_blk
            );
        }
        let preds = cfg.predecessors_len(self.merge_block);
        if preds == 0 && std::env::var("NYASH_CONTROL_FORM_TRACE").ok().as_deref() == Some("1") {
            get_global_ring0().log.warn(&format!(
                "[ControlForm::IfShape] WARN: merge block {:?} has no predecessors yet",
                self.merge_block
            ));
        }
    }
}
