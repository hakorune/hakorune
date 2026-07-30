use super::LoopToJoinLowerer;
use crate::mir::join_ir::lowering::loop_scope_shape::{
    case_a_minimal_target_name, CaseAMinimalTargetKind,
};
use crate::mir::join_ir::JoinModule;
use crate::mir::loop_form::LoopForm;
use crate::mir::MirFunction;

impl LoopToJoinLowerer {
    /// Case-A 汎用 lowerer の「Main.skip/1 用」薄いラッパー。
    pub fn lower_case_a_for_skip_ws(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(
            func,
            loop_form,
            Some(case_a_minimal_target_name(
                CaseAMinimalTargetKind::SkipWhitespace,
            )),
        )
    }

    /// Case-A 汎用 lowerer の「FuncScannerBox.trim/1 用」薄いラッパー。
    pub fn lower_case_a_for_trim(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(
            func,
            loop_form,
            Some(case_a_minimal_target_name(CaseAMinimalTargetKind::Trim)),
        )
    }
}
