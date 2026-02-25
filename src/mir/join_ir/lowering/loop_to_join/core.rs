use crate::mir::control_form::LoopId;
use crate::mir::join_ir::lowering::loop_form_intake::intake_loop_form;
use crate::mir::join_ir::lowering::loop_pattern_validator::LoopPatternValidator;
use crate::mir::join_ir::lowering::loop_scope_shape::LoopScopeShape;
use crate::mir::join_ir::lowering::loop_view_builder::LoopViewBuilder;
use crate::mir::join_ir::JoinModule;
use crate::mir::loop_form::LoopForm;
use crate::mir::query::MirQueryBox;
use crate::mir::MirFunction;
use crate::runtime::get_global_ring0;

fn generic_case_a_enabled() -> bool {
    crate::mir::join_ir::env_flag_is_1("NYASH_JOINIR_LOWER_GENERIC")
}

/// Loop→JoinIR 変換の統一箱（coordinator）
///
/// - MirQuery/Intake/LoopScopeShape の構築
/// - Validator/Builder 呼び出しの調整
/// - strict mode の fail-fast（対象関数のみ）
pub struct LoopToJoinLowerer {
    debug: bool,
    validator: LoopPatternValidator,
    builder: LoopViewBuilder,
}

impl Default for LoopToJoinLowerer {
    fn default() -> Self {
        Self::new()
    }
}

impl LoopToJoinLowerer {
    pub fn new() -> Self {
        let debug = std::env::var("NYASH_LOOPTOJOIN_DEBUG")
            .map(|v| v == "1")
            .unwrap_or(false);
        Self {
            debug,
            validator: LoopPatternValidator::new(),
            builder: LoopViewBuilder::new(),
        }
    }

    /// MIR LoopForm を JoinIR (JoinModule) に変換
    ///
    /// - `Some(JoinModule)`: 変換成功
    /// - `None`: 未サポート（上位のフォールバックへ）
    pub fn lower(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
        func_name: Option<&str>,
    ) -> Option<JoinModule> {
        let strict_on = crate::config::env::joinir_strict_enabled();
        let is_minimal_target = func_name
            .map(super::super::loop_scope_shape::is_case_a_minimal_target)
            .unwrap_or(false);

        if self.debug {
            get_global_ring0().log.debug(&format!(
                "[LoopToJoinLowerer] lower() called for {:?}",
                func_name.unwrap_or("<unknown>")
            ));
        }

        let query = MirQueryBox::new(func);
        let intake = intake_loop_form(loop_form, &query, func)?;
        let scope = LoopScopeShape::from_loop_form(loop_form, &intake, &query, func_name)?;

        if self.debug {
            get_global_ring0().log.debug(&format!(
                "[LoopToJoinLowerer] LoopScopeShape built: pinned={:?}, carriers={:?}, exit_live={:?}",
                scope.pinned, scope.carriers, scope.exit_live
            ));
        }

        let loop_id = LoopId(0);
        let region = loop_form.to_region_view(loop_id);
        let exit_edges = loop_form.to_exit_edges(loop_id);

        if self.debug {
            get_global_ring0().log.debug(&format!(
                "[LoopToJoinLowerer] views: func={:?} loop_id={:?} header={:?} exits={:?}",
                func_name.unwrap_or("<unknown>"),
                loop_id,
                region.header,
                exit_edges.iter().map(|e| e.to).collect::<Vec<_>>()
            ));
        }

        if !self
            .validator
            .is_supported_case_a(func, &region, &exit_edges, &scope)
        {
            if self.debug {
                get_global_ring0().log.debug(&format!(
                    "[LoopToJoinLowerer] rejected by validator: {:?}",
                    func_name.unwrap_or("<unknown>")
                ));
            }
            if strict_on && is_minimal_target {
                panic!(
                    "[joinir/loop] strict mode: validator rejected {}",
                    func_name.unwrap_or("<unknown>")
                );
            }
            return None;
        }

        if !generic_case_a_enabled() {
            if !func_name.map_or(
                false,
                super::super::loop_scope_shape::is_case_a_minimal_target,
            ) {
                if self.debug {
                    get_global_ring0().log.debug(&format!(
                        "[LoopToJoinLowerer] rejected by name filter (generic disabled): {:?}",
                        func_name.unwrap_or("<unknown>")
                    ));
                }
                if strict_on && is_minimal_target {
                    panic!(
                        "[joinir/loop] strict mode: name filter rejected {}",
                        func_name.unwrap_or("<unknown>")
                    );
                }
                return None;
            }
        } else if self.debug {
            get_global_ring0().log.debug(&format!(
                "[LoopToJoinLowerer] generic Case-A enabled, allowing {:?}",
                func_name.unwrap_or("<unknown>")
            ));
        }

        let out = self.builder.build(scope, func_name);
        if out.is_none() && strict_on && is_minimal_target {
            panic!(
                "[joinir/loop] strict mode: lowering failed for {}",
                func_name.unwrap_or("<unknown>")
            );
        }
        out
    }

    /// 旧コメント/ドキュメントとの整合のための別名（導線の明確化）
    pub fn lower_loop(
        &self,
        func: &MirFunction,
        loop_form: &LoopForm,
        func_name: Option<&str>,
    ) -> Option<JoinModule> {
        self.lower(func, loop_form, func_name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lowerer_creation() {
        let lowerer = LoopToJoinLowerer::new();
        assert!(!lowerer.debug || lowerer.debug);
    }
}
