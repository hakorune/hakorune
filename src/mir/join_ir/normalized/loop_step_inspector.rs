//! Loop step 関数の構造を検査する純関数集合
//!
//! Phase 89 リファクタリング:
//! - 散在していた検出ロジックを共通化
//! - Select/Compare/Jump 検出を1箇所で管理
//! - detector 間で重複コードを削減

#![cfg(feature = "normalized_dev")]

use crate::mir::join_ir::{JoinFunction, JoinInst, MirLikeInst};

/// Loop step 関数の構造を検査する純関数集合
pub struct LoopStepInspector;

impl LoopStepInspector {
    /// Select 命令が含まれるか
    ///
    /// Continue パターンの核心となる命令。
    /// キャリア値を条件的に切り替える。
    pub fn has_select_instruction(loop_step_func: &JoinFunction) -> bool {
        loop_step_func.body.iter().any(|inst| match inst {
            JoinInst::Select { .. } => true,
            JoinInst::Compute(mir_inst) => matches!(mir_inst, MirLikeInst::Select { .. }),
            _ => false,
        })
    }

    /// Compare 命令が含まれるか
    ///
    /// ループ条件や continue/break 判定で使用される。
    pub fn has_compare_instruction(loop_step_func: &JoinFunction) -> bool {
        loop_step_func
            .body
            .iter()
            .any(|inst| matches!(inst, JoinInst::Compute(MirLikeInst::Compare { .. })))
    }

    /// 条件付き Jump の個数をカウント
    ///
    /// k_exit への条件付き Jump の数で、break/return の数を推定できる。
    /// - 1個: 通常の loop break のみ（Pattern 4）
    /// - 2個以上: loop break + early return（Continue+Return pattern）
    pub fn count_conditional_jumps(loop_step_func: &JoinFunction) -> usize {
        loop_step_func
            .body
            .iter()
            .filter(|inst| matches!(inst, JoinInst::Jump { cond: Some(_), .. }))
            .count()
    }

    /// tail call があるか（ループバック）
    ///
    /// k_next が None の Call 命令は、loop_step 自身への tail call（再帰呼び出し）を表す。
    /// これがあることでループが継続する。
    pub fn has_tail_call(loop_step_func: &JoinFunction) -> bool {
        loop_step_func
            .body
            .iter()
            .any(|inst| matches!(inst, JoinInst::Call { k_next: None, .. }))
    }

    /// パラメータ数が妥当な範囲にあるか
    ///
    /// 一般的なループパターンでは 2-4 個のパラメータを持つ:
    /// - 最小: (i, acc)
    /// - 典型: (i, acc, n)
    /// - 拡張: (i, acc, n, extra)
    pub fn has_reasonable_param_count(loop_step_func: &JoinFunction) -> bool {
        (2..=4).contains(&loop_step_func.params.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir::{JoinFuncId, VarId};

    fn make_dummy_loop_step(body: Vec<JoinInst>, param_count: usize) -> JoinFunction {
        let params: Vec<_> = (0..param_count).map(|i| VarId(i as u32)).collect();
        JoinFunction {
            id: JoinFuncId(1),
            name: "loop_step".to_string(),
            params,
            body,
            ..Default::default()
        }
    }

    #[test]
    fn test_has_select_instruction() {
        let func_with_select = make_dummy_loop_step(
            vec![JoinInst::Select {
                dst: VarId(10),
                cond: VarId(5),
                on_true: VarId(6),
                on_false: VarId(7),
            }],
            2,
        );
        assert!(LoopStepInspector::has_select_instruction(&func_with_select));

        let func_without_select = make_dummy_loop_step(vec![], 2);
        assert!(!LoopStepInspector::has_select_instruction(
            &func_without_select
        ));
    }

    #[test]
    fn test_count_conditional_jumps() {
        let func = make_dummy_loop_step(
            vec![
                JoinInst::Jump {
                    target: JoinFuncId(2),
                    cond: Some(VarId(1)),
                    args: vec![],
                },
                JoinInst::Jump {
                    target: JoinFuncId(2),
                    cond: Some(VarId(2)),
                    args: vec![],
                },
                JoinInst::Jump {
                    target: JoinFuncId(2),
                    cond: None, // 無条件は含まれない
                    args: vec![],
                },
            ],
            2,
        );
        assert_eq!(LoopStepInspector::count_conditional_jumps(&func), 2);
    }

    #[test]
    fn test_has_tail_call() {
        let func_with_tail_call = make_dummy_loop_step(
            vec![JoinInst::Call {
                target: JoinFuncId(1),
                args: vec![],
                k_next: None, // tail call
                results: vec![],
            }],
            2,
        );
        assert!(LoopStepInspector::has_tail_call(&func_with_tail_call));

        let func_without_tail_call = make_dummy_loop_step(
            vec![JoinInst::Call {
                target: JoinFuncId(1),
                args: vec![],
                k_next: Some(JoinFuncId(3)), // not tail call
                results: vec![],
            }],
            2,
        );
        assert!(!LoopStepInspector::has_tail_call(&func_without_tail_call));
    }

    #[test]
    fn test_has_reasonable_param_count() {
        assert!(!LoopStepInspector::has_reasonable_param_count(
            &make_dummy_loop_step(vec![], 1)
        ));
        assert!(LoopStepInspector::has_reasonable_param_count(
            &make_dummy_loop_step(vec![], 2)
        ));
        assert!(LoopStepInspector::has_reasonable_param_count(
            &make_dummy_loop_step(vec![], 3)
        ));
        assert!(LoopStepInspector::has_reasonable_param_count(
            &make_dummy_loop_step(vec![], 4)
        ));
        assert!(!LoopStepInspector::has_reasonable_param_count(
            &make_dummy_loop_step(vec![], 5)
        ));
    }
}
