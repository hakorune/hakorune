use crate::mir::builder::MirBuilder;
use crate::mir::verification::utils::compute_def_blocks;
use crate::mir::{BasicBlockId, ValueId};
use std::collections::HashMap;

pub(super) struct DebugCtx {
    pub(super) fn_name: String,
    pub(super) def_blocks: HashMap<ValueId, BasicBlockId>,
}

pub(super) fn build(builder: &MirBuilder) -> Option<DebugCtx> {
    if !crate::config::env::joinir_dev::strict_planner_required_debug_enabled() {
        return None;
    }
    let func = builder.scope_ctx.current_function.as_ref()?;
    Some(DebugCtx {
        fn_name: func.signature.name.clone(),
        def_blocks: compute_def_blocks(func),
    })
}

pub(super) fn current_fn_name(builder: &MirBuilder) -> String {
    builder
        .scope_ctx
        .current_function
        .as_ref()
        .map(|f| f.signature.name.clone())
        .unwrap_or_else(|| "<none>".to_string())
}
