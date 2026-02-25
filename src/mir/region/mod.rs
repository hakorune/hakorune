/*!
 * MIR Region/GC Observation Layer (Phase 25.1l)
 *
 * 目的:
 * - LoopForm v2 / ControlForm を「Region Box（寿命管理の箱）」として眺めるための
 *   型定義と観測用ヘルパーを提供するよ。
 * - このフェーズでは GC の retain/release などは一切挿入せず、あくまで
 *   「どの制御構造でどのスロットが生きているか」をログで観測するだけだよ。
 *
 * 注意:
 * - 既存の SSA/PHI 挙動には影響を与えない（NYASH_REGION_TRACE=1 のときだけ動く）。
 * - .hako 側やランタイム側の GC 実装には触れない。
 */

use crate::mir::{BasicBlockId, MirType};

/// GC/寿命管理の観点から見たスロット種別だよ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefSlotKind {
    /// GC root 候補となる強参照スロット（Box 系など）
    StrongRoot,
    /// 弱参照（将来の WeakRefBox 等）。Phase 25.1l ではまだ未使用。
    WeakRoot,
    /// 借用スロット（寿命は SSA で管理、GC root ではない想定）
    Borrowed,
    /// 非参照（プリミティブ値など、GC 対象外）
    NonRef,
}

/// 1 つの変数スロットに関するメタデータだよ。
#[derive(Debug, Clone)]
pub struct SlotMetadata {
    pub name: String,
    pub ref_kind: RefSlotKind,
}

/// Region ID の薄い newtype だよ（デバッグ用途）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RegionId(pub u32);

/// Region の種別（Function / Loop / If など）だよ。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionKind {
    Function,
    Loop,
    If,
}

/// ControlForm から派生した Region 情報だよ（観測専用）。
#[derive(Debug, Clone)]
pub struct Region {
    pub id: RegionId,
    pub kind: RegionKind,
    pub parent: Option<RegionId>,
    pub entry_block: BasicBlockId,
    pub exit_blocks: Vec<BasicBlockId>,
    pub slots: Vec<SlotMetadata>,
}

impl Region {
    /// MirType から簡易的に RefSlotKind を推定するよ（観測専用）。
    pub fn classify_ref_kind(ty: &MirType) -> RefSlotKind {
        match ty {
            MirType::Box(_) | MirType::Array(_) | MirType::Future(_) => RefSlotKind::StrongRoot,
            MirType::WeakRef => RefSlotKind::WeakRoot, // Phase 285A1
            MirType::Integer | MirType::Float | MirType::Bool | MirType::String => {
                RefSlotKind::NonRef
            }
            MirType::Void | MirType::Unknown => RefSlotKind::NonRef,
        }
    }
}

pub mod function_slot_registry;
pub mod observer;
