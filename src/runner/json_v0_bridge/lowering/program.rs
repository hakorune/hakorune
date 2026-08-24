use super::{lower_stmt_list_with_vars, BridgeEnv, FunctionDefBuilder, LoopContext};
use crate::ast::RuneAttr;
use crate::mir::{
    BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};
use std::collections::BTreeMap;

pub(super) fn rune_attrs_from_json_v0(attrs: &super::super::ast::FuncAttrsV0) -> Vec<RuneAttr> {
    attrs
        .runes
        .iter()
        .map(|rune| RuneAttr {
            name: rune.name.clone(),
            args: rune.args.clone(),
        })
        .collect()
}

pub(super) fn is_stageb_entry_def(func_def: &super::super::ast::FuncDefV0) -> bool {
    super::program_call_targets::is_stageb_entry_def(func_def)
}

pub(super) fn lower_defs_into_module(
    module: &mut MirModule,
    defs: Vec<super::super::ast::FuncDefV0>,
    env: &BridgeEnv,
) -> Result<(), String> {
    // Phase 21.6: Process function definitions (defs)
    // Phase 25.1p: FunctionDefBuilder による箱化・SSOT化
    for func_def in defs {
        if is_stageb_entry_def(&func_def) {
            continue;
        }
        // Phase 25.1p: FunctionDefBuilder で SSOT 化
        let builder = FunctionDefBuilder::new(func_def.clone());

        // Build signature and function
        let sig = builder.build_signature();
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);
        func.metadata.runes = rune_attrs_from_json_v0(&func_def.attrs);
        crate::mir::rune_plan_refresh::refresh_function_rune_plans(&mut func);

        // Build variable map from the function's reserved parameter ValueIds (SSOT)
        let param_ids = func.params.clone();
        let mut func_var_map = builder.build_var_map(&param_ids);

        // Lower function body
        let mut loop_stack: Vec<LoopContext> = Vec::new();
        let start_bb = func.entry_block;
        // Per-def env: bind `me`-receiver static dispatch to the owning box name.
        // mode-B compatibility JSON often uses `me.method(...)` inside box methods even when `me` is not a
        // runtime value; lowering treats this as a static-box method call on `box_name`.
        let mut env_for_def = env.clone();
        env_for_def.me_class = func_def.box_name.clone();
        let _end_bb = lower_stmt_list_with_vars(
            &mut func,
            start_bb,
            &func_def.body.body,
            &mut func_var_map,
            &mut loop_stack,
            &env_for_def,
        )?;

        func.signature.return_type = MirType::Unknown;
        module.add_function(func);
    }
    Ok(())
}

pub(super) fn lower_main_body(
    module: &mut MirModule,
    prog_body: &[super::super::ast::StmtV0],
    env: &BridgeEnv,
) -> Result<(), String> {
    // Treat CLI entry as taking a single parameter `args`.
    let sig = FunctionSignature {
        name: "main".into(),
        params: vec![MirType::Unknown],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let entry = BasicBlockId::new(0);
    let mut f = MirFunction::new(sig, entry);
    let mut var_map: BTreeMap<String, ValueId> = BTreeMap::new();
    // Stage-3 programs (launcher / CLI entry) implicitly reference `args`.
    // MirFunction::new() already reserves canonical params [0..N-1]; keep that
    // SSOT so emitted MIR stays executable by both v1 and v0 loaders.
    let args_param = f.params.first().copied().unwrap_or(ValueId::new(0));
    var_map.insert("args".into(), args_param);
    let mut loop_stack: Vec<LoopContext> = Vec::new();
    let start_bb = f.entry_block;
    let end_bb = lower_stmt_list_with_vars(
        &mut f,
        start_bb,
        prog_body,
        &mut var_map,
        &mut loop_stack,
        env,
    )?;
    let need_default_ret = f.blocks.iter().any(|(_k, b)| !b.is_terminated());
    if need_default_ret {
        let target_bb = end_bb;
        let dst_id = f.next_value_id();
        if let Some(bb) = f.get_block_mut(target_bb) {
            if !bb.is_terminated() {
                bb.add_instruction(MirInstruction::Const {
                    dst: dst_id,
                    value: ConstValue::Integer(0),
                });
                bb.set_terminator(MirInstruction::Return {
                    value: Some(dst_id),
                });
            }
        }
    }
    f.signature.return_type = MirType::Unknown;
    module.add_function(f);
    Ok(())
}
