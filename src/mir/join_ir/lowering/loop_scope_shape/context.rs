//! CaseAContext - generic_case_a 共通ロジックの集約

use std::collections::BTreeMap;

use crate::mir::join_ir::lowering::exit_args_resolver::resolve_exit_args;
use crate::mir::ValueId;
use crate::runtime::get_global_ring0;

use super::LoopScopeShape;

/// Case A lowering の共通コンテキスト
///
/// generic_case_a.rs の4関数に共通するロジックを集約し、
/// 重複コードを削減する。
#[derive(Debug, Clone)]
pub(crate) struct CaseAContext {
    pub ordered_pinned: Vec<String>,
    pub ordered_carriers: Vec<String>,
    pub name_to_loop_id: BTreeMap<String, ValueId>,
    pub pinned_ids: Vec<ValueId>,
    pub carrier_ids: Vec<ValueId>,
    pub exit_args: Vec<ValueId>,
}

impl CaseAContext {
    /// LoopScopeShape を直接受け取るコンストラクタ
    ///
    /// # Arguments
    ///
    /// - `scope`: LoopScopeShape（変数スコープ情報）
    /// - `log_tag`: ログ出力用タグ（例: "skip_ws", "trim"）
    /// - `loop_step_id_fn`: offset から ValueId を生成する関数
    ///
    /// # Returns
    ///
    /// Some(CaseAContext) if successful, None if validation fails.
    pub(crate) fn from_scope<F>(
        scope: LoopScopeShape,
        log_tag: &str,
        loop_step_id_fn: F,
    ) -> Option<Self>
    where
        F: Fn(u32) -> ValueId,
    {
        if scope.header == scope.exit {
            if crate::config::env::joinir_dev::debug_enabled() {
                get_global_ring0().log.debug(&format!(
                    "[joinir/generic_case_a/{}] loop_form malformed (header == exit), fallback",
                    log_tag
                ));
            }
            return None;
        }

        let ordered_pinned = scope.pinned_ordered();
        let ordered_carriers = scope.carriers_ordered();

        let mut name_to_loop_id: BTreeMap<String, ValueId> = BTreeMap::new();
        let mut offset: u32 = 0;
        for name in &ordered_pinned {
            name_to_loop_id.insert(name.clone(), loop_step_id_fn(offset));
            offset += 1;
        }
        for name in &ordered_carriers {
            name_to_loop_id.insert(name.clone(), loop_step_id_fn(offset));
            offset += 1;
        }

        let pinned_ids: Vec<ValueId> = ordered_pinned
            .iter()
            .filter_map(|k| name_to_loop_id.get(k).copied())
            .collect();
        let carrier_ids: Vec<ValueId> = ordered_carriers
            .iter()
            .filter_map(|k| name_to_loop_id.get(k).copied())
            .collect();

        let exit_args = resolve_exit_args(&scope.exit_live, &name_to_loop_id, &ordered_carriers)?;

        Some(Self {
            ordered_pinned,
            ordered_carriers,
            name_to_loop_id,
            pinned_ids,
            carrier_ids,
            exit_args,
        })
    }

    /// 変数名から loop 関数内の ValueId を取得
    pub fn get_loop_id(&self, name: &str) -> Option<ValueId> {
        self.name_to_loop_id.get(name).copied()
    }

    /// pinned 変数の n 番目の名前を取得（なければ 0 番目を使う）
    pub fn pinned_name_or_first(&self, index: usize) -> Option<String> {
        self.ordered_pinned
            .get(index)
            .cloned()
            .or_else(|| self.ordered_pinned.first().cloned())
    }

    /// carrier 変数の n 番目の名前を取得（なければ 0 番目を使う）
    pub fn carrier_name_or_first(&self, index: usize) -> Option<String> {
        self.ordered_carriers
            .get(index)
            .cloned()
            .or_else(|| self.ordered_carriers.first().cloned())
    }
}
