use super::LoopToJoinLowerer;
use crate::mir::join_ir::lowering::loop_scope_shape::{
    case_a_minimal_target_name, CaseAMinimalTargetKind,
};
use crate::mir::join_ir::JoinModule;
use crate::mir::loop_form::LoopForm;
use crate::mir::MirFunction;

const STAGEB_BODY_CONTEXT_LABEL: &str = "StageBBodyExtractorBox.build_body_src/2";
const STAGEB_FUNCSCANNER_CONTEXT_LABEL: &str = "StageBFuncScannerBox.scan_all_boxes/1";

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

    /// Case-A 汎用 lowerer の「FuncScannerBox.append_defs/2 用」薄いラッパー。
    pub fn lower_case_a_for_append_defs(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(
            func,
            loop_form,
            Some(case_a_minimal_target_name(
                CaseAMinimalTargetKind::AppendDefs,
            )),
        )
    }

    /// Case-A 汎用 lowerer の「Stage1UsingResolverBox.resolve_for_source/5 用」薄いラッパー。
    pub fn lower_case_a_for_stage1_resolver(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(
            func,
            loop_form,
            Some(case_a_minimal_target_name(
                CaseAMinimalTargetKind::Stage1UsingResolver,
            )),
        )
    }

    /// Case-A 汎用 lowerer の「StageBBodyExtractorBox.build_body_src/2 用」薄いラッパー。
    pub fn lower_case_a_for_stageb_body(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(func, loop_form, Some(STAGEB_BODY_CONTEXT_LABEL))
    }

    /// Case-A 汎用 lowerer の「StageBFuncScannerBox.scan_all_boxes/1 用」薄いラッパー。
    pub fn lower_case_a_for_stageb_funcscanner(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(func, loop_form, Some(STAGEB_FUNCSCANNER_CONTEXT_LABEL))
    }
}
