//! LoopScopeShape 本体と分類API
//!
//! 変数分類の唯一の情報源 (SSOT) として、pinned/carrier/body_local/exit_live を保持する。

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::BasicBlockId;

/// Variable classification for loop PHI generation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LoopVarClass {
    Pinned,
    Carrier,
    BodyLocalExit,
    BodyLocalInternal,
}

impl LoopVarClass {
    #[cfg(test)]
    pub fn needs_exit_phi(self) -> bool {
        matches!(
            self,
            LoopVarClass::Pinned | LoopVarClass::Carrier | LoopVarClass::BodyLocalExit
        )
    }

    #[cfg(test)]
    pub fn needs_header_phi(self) -> bool {
        matches!(self, LoopVarClass::Pinned | LoopVarClass::Carrier)
    }
}

/// ループ変数スコープの統合ビュー
///
/// ## 4分類の定義
///
/// 1. **Pinned**（ループ外パラメータ）
///    - ループ開始前に定義され、ループ内で変更されない
///    - header/exit で PHI 必須
///
/// 2. **Carrier**（ループ更新変数）
///    - 各イテレーションで更新される
///    - header/exit で PHI 必須
///
/// 3. **BodyLocalExit**（全 exit 経路で定義）
///    - ループ内で定義され、全 exit predecessor で利用可能
///    - exit で PHI 必須、header では不要
///
/// 4. **BodyLocalInternal**（一部 exit 経路のみ）
///    - 一部の exit predecessor でのみ定義
///    - PHI 不要（Option C）
///
/// # Fields
///
/// - Block IDs: `header`, `body`, `latch`, `exit`
/// - Variable classification: `pinned`, `carriers`, `body_locals`, `exit_live`
/// - `progress_carrier`: 進捗チェック用（将来の Verifier で使用予定）
/// - `variable_definitions`: definition blocks collected from LoopFormIntake snapshots
///
/// # Phase 183-3: Construction Path
///
/// LoopScopeShape construction is owned by the LoopForm-based builder:
///
/// - **LoopForm-based** (JoinIR lowering): `loop_scope_shape/builder.rs`
/// - analyzes LoopForm and LoopFormIntake
/// - keeps field initialization and variable classification in one owner
#[derive(Debug, Clone)]
pub(crate) struct LoopScopeShape {
    pub header: BasicBlockId,
    #[allow(dead_code)]
    pub body: BasicBlockId,
    pub latch: BasicBlockId,
    pub exit: BasicBlockId,
    pub pinned: BTreeSet<String>,
    pub carriers: BTreeSet<String>,
    pub body_locals: BTreeSet<String>,
    pub exit_live: BTreeSet<String>,
    pub progress_carrier: Option<String>,
    pub(crate) variable_definitions: BTreeMap<String, BTreeSet<BasicBlockId>>,
}

impl LoopScopeShape {
    /// header PHI が必要か判定
    #[cfg(test)]
    pub fn needs_header_phi(&self, var_name: &str) -> bool {
        self.pinned.contains(var_name) || self.carriers.contains(var_name)
    }

    /// exit PHI が必要か判定
    #[cfg(test)]
    pub fn needs_exit_phi(&self, var_name: &str) -> bool {
        self.exit_live.contains(var_name)
    }

    /// 順序付き pinned 変数一覧
    pub fn pinned_ordered(&self) -> Vec<String> {
        self.pinned.iter().cloned().collect()
    }

    /// 順序付き carrier 変数一覧
    pub fn carriers_ordered(&self) -> Vec<String> {
        self.carriers.iter().cloned().collect()
    }

    /// 変数を4分類
    #[cfg(test)]
    pub fn classify(&self, var_name: &str) -> LoopVarClass {
        if self.pinned.contains(var_name) {
            return LoopVarClass::Pinned;
        }

        if self.carriers.contains(var_name) {
            return LoopVarClass::Carrier;
        }

        if self.body_locals.contains(var_name) {
            if self.exit_live.contains(var_name) {
                LoopVarClass::BodyLocalExit
            } else {
                LoopVarClass::BodyLocalInternal
            }
        } else {
            LoopVarClass::BodyLocalInternal
        }
    }

    /// ループ終了時に live な変数集合を返す
    #[cfg(test)]
    pub fn get_exit_live(&self) -> &BTreeSet<String> {
        &self.exit_live
    }

    /// 変数が required_blocks すべてで利用可能か判定
    #[cfg(test)]
    pub fn is_available_in_all(&self, var_name: &str, required_blocks: &[BasicBlockId]) -> bool {
        if let Some(def_blocks) = self.variable_definitions.get(var_name) {
            required_blocks.iter().all(|bid| def_blocks.contains(bid))
        } else {
            false
        }
    }
}
