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
use std::collections::BTreeSet;

// ============================================================================
// Phase 32: 新しい ID 型（LoopRegion / LoopControlShape 用）
// ============================================================================

/// ループを一意に識別する ID
///
/// Phase 264: PartialOrd, Ord を追加（ExitKind の BTreeMap キーとして使用）
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct LoopId(pub u32);

/// 出口辺を一意に識別する ID
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ExitEdgeId(pub u32);

/// continue 辺を一意に識別する ID
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
pub struct ContinueEdgeId(pub u32);

/// ループラベル（将来の labeled break/continue 用）
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct LoopLabel(pub String);

// ============================================================================
// Phase 32: LoopRegion - ブロック集合 + ネスト構造
// ============================================================================

/// ループ領域を表す箱
///
/// LoopShape の「形だけ」から拡張して、ブロック集合とネスト関係を持つ。
#[derive(Debug, Clone)]
pub struct LoopRegion {
    /// このループの ID
    pub id: LoopId,
    /// ループ直前のブロック
    pub preheader: BasicBlockId,
    /// ループヘッダ（条件判定）
    pub header: BasicBlockId,
    /// ラッチブロック群（通常は1つだが、複数の場合もある）
    pub latches: Vec<BasicBlockId>,
    /// ループ内の全ブロック集合
    pub blocks: BTreeSet<BasicBlockId>,
    /// 親ループの ID（ネストの外側）
    pub parent: Option<LoopId>,
    /// 子ループの ID 群（ネストの内側）
    pub children: Vec<LoopId>,
}

// ============================================================================
// Phase 32: LoopControlShape - 制御フロー辺
// ============================================================================

/// ループの制御フロー辺を表す箱
#[derive(Debug, Clone)]
pub struct LoopControlShape {
    /// このループの ID
    pub loop_id: LoopId,
    /// continue 辺の ID 群
    pub continues: Vec<ContinueEdgeId>,
    /// 出口辺の ID 群
    pub exits: Vec<ExitEdgeId>,
}

/// 出口辺を表す構造体
#[derive(Debug, Clone)]
pub struct ExitEdge {
    /// この辺の ID
    pub id: ExitEdgeId,
    /// どのループからの出口か（冗長だけど便利）
    pub loop_id: LoopId,
    /// 出発ブロック
    pub from: BasicBlockId,
    /// 到着ブロック
    pub to: BasicBlockId,
    /// 出口の種類
    pub kind: ExitKind,
}

/// 出口辺の種類
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ExitKind {
    /// while(cond) の cond が false
    ConditionFalse,
    /// break 文
    Break {
        /// labeled break 用（将来拡張）
        label: Option<LoopLabel>,
    },
    /// ループ内 return
    Return,
    /// throw / panic 相当
    Throw,
}

// ============================================================================
// Phase 32 L-1.4: ExitGroup / ExitAnalysis（出口辺のグループ化）
// ============================================================================

/// 同じ target ブロックに向かう出口辺のグループ
///
/// 複数の ExitEdge が同じブロックに向かう場合、Case-A 判定では
/// これらを 1 つの「論理的な出口」として扱える。
#[derive(Debug, Clone)]
pub struct ExitGroup {
    /// グループの出口先ブロック
    pub target: BasicBlockId,
    /// このグループに含まれる ExitEdge の ID 群
    pub edges: Vec<ExitEdgeId>,
    /// Break を含むか（ConditionFalse のみのグループと区別）
    pub has_break: bool,
}

/// ループの出口辺を分析した結果
///
/// - `loop_exit_groups`: ループ外の同一ブロックへ向かう辺のグループ群
/// - `nonlocal_exits`: Return/Throw など、ループ外への非ローカル出口
#[derive(Debug, Clone)]
pub struct ExitAnalysis {
    /// ループ外への出口グループ（target ブロック単位）
    pub loop_exit_groups: Vec<ExitGroup>,
    /// Return/Throw など、関数全体を抜ける出口
    pub nonlocal_exits: Vec<ExitEdgeId>,
}

impl ExitAnalysis {
    /// Case-A 判定: ループ外出口が 1 グループのみで、非ローカル出口がない
    pub fn is_single_exit_group(&self) -> bool {
        self.loop_exit_groups.len() == 1 && self.nonlocal_exits.is_empty()
    }

    /// 唯一のループ外出口先ブロック（Case-A の場合のみ有効）
    pub fn single_exit_target(&self) -> Option<BasicBlockId> {
        if self.is_single_exit_group() {
            self.loop_exit_groups.first().map(|g| g.target)
        } else {
            None
        }
    }
}

/// 出口辺リストを分析して ExitAnalysis を生成
///
/// # Arguments
/// * `exits` - ExitEdge のリスト
///
/// # Returns
/// * `ExitAnalysis` - グループ化された出口情報
pub fn analyze_exits(exits: &[ExitEdge]) -> ExitAnalysis {
    use std::collections::BTreeMap;

    // target ブロック → (辺ID群, has_break)
    let mut groups: BTreeMap<BasicBlockId, (Vec<ExitEdgeId>, bool)> = BTreeMap::new();
    let mut nonlocal: Vec<ExitEdgeId> = Vec::new();

    for edge in exits {
        match &edge.kind {
            ExitKind::ConditionFalse | ExitKind::Break { .. } => {
                let entry = groups.entry(edge.to).or_insert_with(|| (Vec::new(), false));
                entry.0.push(edge.id);
                if matches!(&edge.kind, ExitKind::Break { .. }) {
                    entry.1 = true;
                }
            }
            ExitKind::Return | ExitKind::Throw => {
                nonlocal.push(edge.id);
            }
        }
    }

    let loop_exit_groups: Vec<ExitGroup> = groups
        .into_iter()
        .map(|(target, (edges, has_break))| ExitGroup {
            target,
            edges,
            has_break,
        })
        .collect();

    ExitAnalysis {
        loop_exit_groups,
        nonlocal_exits: nonlocal,
    }
}

/// continue 辺を表す構造体
#[derive(Debug, Clone)]
pub struct ContinueEdge {
    /// この辺の ID
    pub id: ContinueEdgeId,
    /// どのループの continue か
    pub loop_id: LoopId,
    /// 出発ブロック
    pub from: BasicBlockId,
    /// 到着ブロック（通常は latch または header）
    pub to: BasicBlockId,
    /// continue の種類
    pub kind: ContinueKind,
}

/// continue 辺の種類
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ContinueKind {
    /// 通常の continue
    Normal,
    // 将来: Labeled { label: LoopLabel },
}

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

impl ControlForm {
    /// ループ用 Shape から ControlForm を生成するよ。
    ///
    /// ループの entry は preheader、exit は exit ブロック 1 つとみなす。
    pub fn from_loop(shape: LoopShape) -> Self {
        ControlForm {
            entry: shape.preheader,
            exits: vec![shape.exit],
            kind: ControlKind::Loop(shape),
        }
    }

    /// If 用 Shape から ControlForm を生成するよ。
    ///
    /// If の entry は cond_block、exit は merge_block 1 つとみなす。
    pub fn from_if(shape: IfShape) -> Self {
        ControlForm {
            entry: shape.cond_block,
            exits: vec![shape.merge_block],
            kind: ControlKind::If(shape),
        }
    }

    /// これはループかな？という軽い判定だよ。
    pub fn is_loop(&self) -> bool {
        matches!(self.kind, ControlKind::Loop(_))
    }

    /// これは if 構造かな？という軽い判定だよ。
    pub fn is_if(&self) -> bool {
        matches!(self.kind, ControlKind::If(_))
    }

    /// デバッグ用に構造をダンプするよ。
    ///
    /// 呼び出し側で `NYASH_CONTROL_FORM_TRACE=1` を見る想定なので、
    /// ここでは単純に eprintln! するだけにしておく。
    pub fn debug_dump(&self) {
        match &self.kind {
            ControlKind::Loop(shape) => {
                get_global_ring0().log.debug(&format!(
                    "[ControlForm::Loop] entry={:?} preheader={:?} header={:?} body={:?} latch={:?} exit={:?} continue={:?} break={:?}",
                    self.entry,
                    shape.preheader,
                    shape.header,
                    shape.body,
                    shape.latch,
                    shape.exit,
                    shape.continue_targets,
                    shape.break_targets,
                ));
            }
            ControlKind::If(shape) => {
                get_global_ring0().log.debug(&format!(
                    "[ControlForm::If] entry={:?} cond={:?} then={:?} else={:?} merge={:?} exits={:?}",
                    self.entry,
                    shape.cond_block,
                    shape.then_block,
                    shape.else_block,
                    shape.merge_block,
                    self.exits,
                ));
            }
        }
    }
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

/// ControlForm トレース用の環境フラグを判定するヘルパーだよ。
///
/// - 未設定         → 既定で ON
/// - "0" / "false" → OFF
/// - それ以外      → ON
pub fn is_control_form_trace_on() -> bool {
    std::env::var("NYASH_CONTROL_FORM_TRACE")
        .map(|v| v != "0" && v.to_lowercase() != "false")
        .unwrap_or(true)
}

impl LoopShape {
    // ========================================================================
    // Phase 32: View メソッド（段階移行用）
    // ========================================================================

    /// LoopRegion ビューを生成するよ。
    ///
    /// 既存の LoopShape から LoopRegion 形式に変換する。
    /// blocks 集合は呼び出し側が後から設定するか、空のままでもOK。
    pub fn to_region_view(&self, loop_id: LoopId) -> LoopRegion {
        LoopRegion {
            id: loop_id,
            preheader: self.preheader,
            header: self.header,
            latches: vec![self.latch], // 既存 LoopShape は latch 1つ
            blocks: BTreeSet::new(),   // 呼び出し側で設定可能
            parent: None,              // ネスト情報は後から設定
            children: vec![],
        }
    }

    /// LoopControlShape ビューを生成するよ。
    ///
    /// 既存の LoopShape から LoopControlShape 形式に変換する。
    /// 辺の ID は呼び出し側で管理するため、ここでは ID リストのみ返す。
    pub fn to_control_view(&self, loop_id: LoopId) -> LoopControlShape {
        // 辺の数だけ ID を振る（0始まり、呼び出し側でオフセット可能）
        let continues: Vec<ContinueEdgeId> = self
            .continue_targets
            .iter()
            .enumerate()
            .map(|(i, _)| ContinueEdgeId(i as u32))
            .collect();

        let exits: Vec<ExitEdgeId> = self
            .break_targets
            .iter()
            .enumerate()
            .map(|(i, _)| ExitEdgeId(i as u32))
            .collect();

        LoopControlShape {
            loop_id,
            continues,
            exits,
        }
    }

    /// 出口辺を生成するよ。
    ///
    /// break_targets の情報から ExitEdge 群を生成する。
    pub fn to_exit_edges(&self, loop_id: LoopId) -> Vec<ExitEdge> {
        self.break_targets
            .iter()
            .enumerate()
            .map(|(i, &from)| ExitEdge {
                id: ExitEdgeId(i as u32),
                loop_id,
                from,
                to: self.exit,
                kind: ExitKind::Break { label: None },
            })
            .collect()
    }

    /// continue 辺を生成するよ。
    ///
    /// continue_targets の情報から ContinueEdge 群を生成する。
    pub fn to_continue_edges(&self, loop_id: LoopId) -> Vec<ContinueEdge> {
        self.continue_targets
            .iter()
            .enumerate()
            .map(|(i, &from)| ContinueEdge {
                id: ContinueEdgeId(i as u32),
                loop_id,
                from,
                to: self.header, // continue は header に戻る
                kind: ContinueKind::Normal,
            })
            .collect()
    }

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
