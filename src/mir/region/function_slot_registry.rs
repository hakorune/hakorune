/*!
 * FunctionSlotRegistry – 関数スコープのスロット情報を管理する箱だよ。
 *
 * 目的:
 * - 変数名ごとの「スロット」を 1 箇所に集約して管理すること。
 * - 各スロットに型情報や RefSlotKind をひも付けられる足場を用意すること。
 *
 * このフェーズでは観測専用:
 * - MIR/SSA の挙動は一切変えないよ。
 * - MirBuilder.variable_map や PHI 生成ロジックには影響を与えないよ。
 */

use super::RefSlotKind;
use crate::mir::MirType;
use std::collections::HashMap;

/// 1 関数内でのスロット ID だよ（添字ベースの薄いラッパー）。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SlotId(pub u32);

/// 1 スロットに対応するメタデータだよ。
#[derive(Debug, Clone)]
pub struct SlotInfo {
    /// スロット名（変数名）だよ。
    pub name: String,
    /// MIR 型情報（分かっていれば Some）だよ。
    pub ty: Option<MirType>,
    /// GC/寿命管理の観点から見た種別（まだ観測専用）だよ。
    pub ref_kind: Option<RefSlotKind>,
}

/// 関数スコープごとのスロットレジストリだよ。
///
/// - `slots`: SlotId → SlotInfo の順序付き配列
/// - `name_to_slot`: 変数名 → SlotId の逆引き
#[derive(Debug, Default, Clone)]
pub struct FunctionSlotRegistry {
    slots: Vec<SlotInfo>,
    name_to_slot: HashMap<String, SlotId>,
}

impl FunctionSlotRegistry {
    /// 空のレジストリを作るよ。
    pub fn new() -> Self {
        Self::default()
    }

    /// スロットを「なければ作る・あれば返す」で確保するよ。
    ///
    /// - name: スロット名（変数名）
    /// - ty: 初期の型情報（後から埋めても OK）
    pub fn ensure_slot(&mut self, name: &str, ty: Option<MirType>) -> SlotId {
        if let Some(slot) = self.name_to_slot.get(name).copied() {
            // 既存スロットに対しては、型がまだ None で新しい情報があれば埋める程度に留める
            if let (Some(new_ty), Some(info)) = (ty, self.slots.get_mut(slot.0 as usize)) {
                if info.ty.is_none() {
                    info.ty = Some(new_ty);
                }
            }
            return slot;
        }

        let id = SlotId(self.slots.len() as u32);
        self.slots.push(SlotInfo {
            name: name.to_string(),
            ty,
            ref_kind: None,
        });
        self.name_to_slot.insert(name.to_string(), id);
        id
    }

    /// RefSlotKind を後から埋めるためのヘルパーだよ。
    pub fn set_ref_kind(&mut self, slot: SlotId, kind: RefSlotKind) {
        if let Some(info) = self.slots.get_mut(slot.0 as usize) {
            info.ref_kind = Some(kind);
        }
    }

    /// 全スロットを列挙するイテレータだよ（観測専用）。
    pub fn iter_slots(&self) -> impl Iterator<Item = &SlotInfo> {
        self.slots.iter()
    }

    /// 名前から SlotId を引くよ。
    pub fn get_slot(&self, name: &str) -> Option<SlotId> {
        self.name_to_slot.get(name).copied()
    }

    /// SlotId から SlotInfo を引くよ。
    pub fn get_slot_info(&self, slot: SlotId) -> Option<&SlotInfo> {
        self.slots.get(slot.0 as usize)
    }
}
