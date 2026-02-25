use super::LoopToJoinLowerer;
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
        self.lower(func, loop_form, Some("Main.skip/1"))
    }

    /// Case-A 汎用 lowerer の「FuncScannerBox.trim/1 用」薄いラッパー。
    pub fn lower_case_a_for_trim(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(func, loop_form, Some("FuncScannerBox.trim/1"))
    }

    /// Case-A 汎用 lowerer の「FuncScannerBox.append_defs/2 用」薄いラッパー。
    pub fn lower_case_a_for_append_defs(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(func, loop_form, Some("FuncScannerBox.append_defs/2"))
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
            Some("Stage1UsingResolverBox.resolve_for_source/5"),
        )
    }

    /// Case-A 汎用 lowerer の「StageBBodyExtractorBox.build_body_src/2 用」薄いラッパー。
    pub fn lower_case_a_for_stageb_body(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(
            func,
            loop_form,
            Some("StageBBodyExtractorBox.build_body_src/2"),
        )
    }

    /// Case-A 汎用 lowerer の「StageBFuncScannerBox.scan_all_boxes/1 用」薄いラッパー。
    pub fn lower_case_a_for_stageb_funcscanner(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
    ) -> Option<JoinModule> {
        self.lower(
            func,
            loop_form,
            Some("StageBFuncScannerBox.scan_all_boxes/1"),
        )
    }
}
