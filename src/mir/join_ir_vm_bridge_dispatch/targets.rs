use crate::mir::MirModule;

/// JoinIR ブリッジ対象の記述子
#[derive(Debug, Clone, Copy)]
pub struct JoinIrTargetDesc {
    /// 対象関数名（MirModule.functions のキー）
    pub func_name: &'static str,
    /// デフォルト有効化。
    ///
    /// Reserved for target policy. VM bridge rows currently require explicit
    /// activation and therefore set this to false.
    pub default_enabled: bool,
}

/// JoinIR VM bridgeの実行対象テーブル。
///
/// Loop/If/strict classification is the separate neutral
/// `join_ir::lowering::loop_target_policy`.
pub const JOINIR_VM_EXEC_TARGETS: &[JoinIrTargetDesc] = &[
    JoinIrTargetDesc {
        func_name: crate::mir::join_ir::lowering::loop_target_policy::MAIN_SKIP,
        default_enabled: false, // PHI canary のため env 必須
    },
    JoinIrTargetDesc {
        func_name: crate::mir::join_ir::lowering::loop_target_policy::FUNCSCANNER_TRIM,
        default_enabled: false, // VM bridge は env 必須
    },
];

/// VM実行テーブルから対象関数を探す。
pub(crate) fn find_joinir_target(module: &MirModule) -> Option<&'static JoinIrTargetDesc> {
    JOINIR_VM_EXEC_TARGETS
        .iter()
        .find(|target| module.functions.contains_key(target.func_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vm_exec_targets_are_exactly_skip_and_trim() {
        let names = JOINIR_VM_EXEC_TARGETS
            .iter()
            .map(|target| target.func_name)
            .collect::<Vec<_>>();
        assert_eq!(
            names,
            [
                crate::mir::join_ir::lowering::loop_target_policy::MAIN_SKIP,
                crate::mir::join_ir::lowering::loop_target_policy::FUNCSCANNER_TRIM,
            ]
        );
        assert!(JOINIR_VM_EXEC_TARGETS
            .iter()
            .all(|target| !target.default_enabled));
    }

    #[test]
    fn lower_only_observation_names_are_not_vm_targets() {
        for func_name in [
            crate::mir::join_ir::lowering::loop_target_policy::STAGE1_USING_RESOLVER,
            crate::mir::join_ir::lowering::loop_target_policy::STAGEB_BODY_EXTRACTOR,
            crate::mir::join_ir::lowering::loop_target_policy::STAGEB_FUNC_SCANNER,
            "FuncScannerBox.append_defs/2",
        ] {
            assert!(JOINIR_VM_EXEC_TARGETS
                .iter()
                .all(|target| target.func_name != func_name));
        }
    }
}
