//! Route-local generic Case-A probe for `Main.skip/1`.
//!
//! This remains local because skip-ws uses a minimal hand-assembled LoopForm
//! canary. Do not route it through `common/target_adapter.rs`.

use crate::mir::join_ir::lowering::loop_to_join::LoopToJoinLowerer;
use crate::mir::join_ir::JoinModule;
use crate::mir::loop_form::LoopForm;
use crate::mir::query::{MirQuery, MirQueryBox};

/// トグル ON 時にだけ試す generic Case A ロワー（minimal_ssa_skip_ws 限定）
///
/// Phase 31: LoopToJoinLowerer 統一箱経由に移行
pub(super) fn try_lower_skip_ws_generic_case_a(
    module: &crate::mir::MirModule,
) -> Option<JoinModule> {
    let target_func = module.functions.get("Main.skip/1")?;
    let query = MirQueryBox::new(target_func);

    // 最小限の LoopForm 形状推定（Case A/constant-true ループ想定）
    let preheader = target_func.entry_block;
    let header = query.succs(preheader).get(0).copied().unwrap_or(preheader);
    let succs_header = query.succs(header);
    let body = succs_header.get(0).copied().unwrap_or(header);
    let exit = succs_header.get(1).copied().unwrap_or(header);
    let latch = body;

    let loop_form = LoopForm {
        preheader,
        header,
        body,
        latch,
        exit,
        continue_targets: vec![body],
        break_targets: vec![exit],
    };

    // Phase 31: LoopToJoinLowerer 経由で JoinModule 生成
    let lowerer = LoopToJoinLowerer::new();
    lowerer.lower_case_a_for_skip_ws(target_func, &loop_form)
}
