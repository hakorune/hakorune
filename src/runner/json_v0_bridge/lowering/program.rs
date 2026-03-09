use super::{lower_stmt_list_with_vars, BridgeEnv, FunctionDefBuilder, LoopContext};
use crate::mir::Callee;
use crate::mir::{
    BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction, MirModule,
    MirType, ValueId,
};
use std::collections::BTreeMap;

pub(super) fn lower_defs_into_module(
    module: &mut MirModule,
    defs: Vec<super::super::ast::FuncDefV0>,
    env: &BridgeEnv,
) -> Result<BTreeMap<String, String>, String> {
    // Phase 21.6: Process function definitions (defs)
    // Phase 25.1p: FunctionDefBuilder による箱化・SSOT化
    // Toggle: HAKO_STAGEB_FUNC_SCAN=1 + HAKO_MIR_BUILDER_FUNCS=1
    let mut func_map: BTreeMap<String, String> = BTreeMap::new();
    for func_def in defs {
        // Phase 25.1p: FunctionDefBuilder で SSOT 化
        let builder = FunctionDefBuilder::new(func_def.clone());

        let func_name = format!(
            "{}.{}/{}",
            func_def.box_name,
            func_def.name,
            func_def.params.len()
        );

        // Register function in map for Call resolution
        func_map.insert(func_def.name.clone(), func_name);

        // Build signature and function
        let sig = builder.build_signature();
        let entry = BasicBlockId::new(0);
        let mut func = MirFunction::new(sig, entry);

        // Build variable map from the function's reserved parameter ValueIds (SSOT)
        let param_ids = func.params.clone();
        let mut func_var_map = builder.build_var_map(&param_ids);

        // Lower function body
        let mut loop_stack: Vec<LoopContext> = Vec::new();
        let start_bb = func.entry_block;
        // Per-def env: bind `me`-receiver static dispatch to the owning box name.
        // Stage-B JSON often uses `me.method(...)` inside box methods even when `me` is not a
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
    Ok(func_map)
}

pub(super) fn maybe_resolve_calls(
    module: &mut MirModule,
    func_map: &BTreeMap<String, String>,
) {
    // Phase 21.6: Call resolution post-processing
    // Toggle: HAKO_MIR_BUILDER_CALL_RESOLVE=1 controls legacy unqualified function-name
    // resolution. Extern-like runtime names still need callee classification even when the
    // legacy resolver is off so Program(JSON v0) keeps the current bridge/runtime contract.
    let resolve_unqualified_calls = std::env::var("HAKO_MIR_BUILDER_CALL_RESOLVE")
        .ok()
        .as_deref()
        == Some("1");

    for (_func_idx, func) in module.functions.iter_mut() {
        for (_block_id, block) in func.blocks.iter_mut() {
            let mut const_replacements: Vec<(ValueId, String)> = Vec::new();

            // Find Call instructions and their associated Const values
            if resolve_unqualified_calls {
                for inst in &block.instructions {
                    if let MirInstruction::Call {
                        func: func_reg,
                        args,
                        ..
                    } = inst
                    {
                        // Look for the Const instruction that defines func_reg
                        for const_inst in &block.instructions {
                            if let MirInstruction::Const { dst, value } = const_inst {
                                if dst == func_reg {
                                    if let ConstValue::String(name) = value {
                                        // Try to resolve the name
                                        if let Some(resolved) = func_map.get(name) {
                                            let mut new_name = resolved.clone();
                                            // Avoid double suffix if already contains '/N'
                                            if !resolved
                                                .rsplit('/')
                                                .next()
                                                .unwrap_or("")
                                                .chars()
                                                .all(|c| c.is_ascii_digit())
                                                || !resolved.contains('/')
                                            {
                                                new_name = format!(
                                                    "{}{}",
                                                    resolved.clone(),
                                                    format!("/{}", args.len())
                                                );
                                            }
                                            const_replacements.push((*dst, new_name));
                                            if std::env::var("HAKO_MIR_BUILDER_DEBUG")
                                                .ok()
                                                .as_deref()
                                                == Some("1")
                                            {
                                                crate::runtime::get_global_ring0().log.debug(
                                                    &format!(
                                                        "[mirbuilder/call:resolve] {} => {}",
                                                        name, resolved
                                                    ),
                                                );
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Apply replacements
            for (dst, new_name) in const_replacements.iter() {
                for inst in &mut block.instructions {
                    if let MirInstruction::Const { dst: d, value } = inst {
                        if d == dst {
                            *value = ConstValue::String(new_name.clone());
                        }
                    }
                }
            }
            // Build a map reg -> name after replacements
            let mut reg_name: std::collections::BTreeMap<ValueId, String> =
                std::collections::BTreeMap::new();
            for inst in &block.instructions {
                if let MirInstruction::Const { dst, value } = inst {
                    if let ConstValue::String(s) = value {
                        reg_name.insert(*dst, s.clone());
                    }
                }
            }
            // Upgrade legacy calls to Global callee when name is known
            for inst in &mut block.instructions {
                if let MirInstruction::Call {
                    func: func_reg,
                    callee,
                    ..
                } = inst
                {
                    if let Some(name) = reg_name.get(func_reg).cloned() {
                        if should_classify_call_callee(&name, func_map, resolve_unqualified_calls) {
                            *callee = Some(resolve_legacy_call_callee(&name, func_map));
                        }
                    }
                }
            }
        }
    }
}

fn should_classify_call_callee(
    name: &str,
    func_map: &BTreeMap<String, String>,
    resolve_unqualified_calls: bool,
) -> bool {
    resolve_unqualified_calls
        || name.contains('/')
        || func_map.values().any(|resolved| resolved == name)
        || is_json_v0_extern_name(name)
}

fn resolve_legacy_call_callee(name: &str, func_map: &BTreeMap<String, String>) -> Callee {
    if func_map.values().any(|resolved| resolved == name) {
        return Callee::Global(name.to_string());
    }

    let base_name = normalize_legacy_call_name(name);
    if is_json_v0_extern_name(base_name) {
        return Callee::Extern(base_name.to_string());
    }

    Callee::Global(name.to_string())
}

fn normalize_legacy_call_name(name: &str) -> &str {
    match name.rsplit_once('/') {
        Some((base, arity)) if arity.chars().all(|c| c.is_ascii_digit()) => base,
        _ => name,
    }
}

fn is_json_v0_extern_name(name: &str) -> bool {
    name.starts_with("env.") || name.starts_with("nyash.")
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
    let args_param = ValueId::new(1);
    f.params = vec![args_param];
    if f.next_value_id < 2 {
        f.next_value_id = 2;
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use std::sync::{Mutex, OnceLock};

    fn env_guard() -> &'static Mutex<()> {
        static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
        GUARD.get_or_init(|| Mutex::new(()))
    }

    fn test_module_with_call_name(name: &str) -> MirModule {
        let mut module = MirModule::new("json_v0_program".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Unknown,
            effects: EffectMask::PURE,
        };
        let mut func = MirFunction::new(signature, BasicBlockId::new(0));
        let block = func
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry block exists");

        block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String(name.to_string()),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(2)),
            func: ValueId::new(1),
            callee: None,
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId::new(2)),
        });
        module.add_function(func);
        module
    }

    fn entry_call(module: &MirModule) -> &MirInstruction {
        &module
            .get_function("main")
            .expect("main function exists")
            .blocks
            .get(&BasicBlockId::new(0))
            .expect("entry block exists")
            .instructions[1]
    }

    #[test]
    fn maybe_resolve_calls_keeps_env_console_as_extern() {
        let _lock = env_guard().lock().expect("env lock");
        let mut module = test_module_with_call_name("env.console.log");
        maybe_resolve_calls(&mut module, &BTreeMap::new());
        let got = format!("{:?}", entry_call(&module));

        assert!(matches!(
            entry_call(&module),
            MirInstruction::Call {
                func,
                callee: Some(Callee::Extern(name)),
                ..
            } if *func == ValueId::new(1) && name == "env.console.log"
        ), "got={got}");
    }

    #[test]
    fn maybe_resolve_calls_keeps_qualified_defs_as_global() {
        let _lock = env_guard().lock().expect("env lock");
        std::env::set_var("HAKO_MIR_BUILDER_CALL_RESOLVE", "1");
        let mut module = test_module_with_call_name("Main.helper/0");
        let mut func_map = BTreeMap::new();
        func_map.insert("helper".to_string(), "Main.helper/0".to_string());
        maybe_resolve_calls(&mut module, &func_map);
        std::env::remove_var("HAKO_MIR_BUILDER_CALL_RESOLVE");

        assert!(matches!(
            entry_call(&module),
            MirInstruction::Call {
                callee: Some(Callee::Global(name)),
                ..
            } if name == "Main.helper/0"
        ));
    }

    #[test]
    fn maybe_resolve_calls_normalizes_arity_suffixed_env_console_as_extern() {
        let _lock = env_guard().lock().expect("env lock");
        std::env::remove_var("HAKO_MIR_BUILDER_CALL_RESOLVE");
        let mut module = test_module_with_call_name("env.console.log/1");
        maybe_resolve_calls(&mut module, &BTreeMap::new());

        assert!(matches!(
            entry_call(&module),
            MirInstruction::Call {
                callee: Some(Callee::Extern(name)),
                ..
            } if name == "env.console.log"
        ));
    }

    #[test]
    fn maybe_resolve_calls_normalizes_arity_suffixed_env_get_as_extern() {
        let _lock = env_guard().lock().expect("env lock");
        std::env::remove_var("HAKO_MIR_BUILDER_CALL_RESOLVE");
        let mut module = test_module_with_call_name("env.get/1");
        maybe_resolve_calls(&mut module, &BTreeMap::new());

        assert!(matches!(
            entry_call(&module),
            MirInstruction::Call {
                callee: Some(Callee::Extern(name)),
                ..
            } if name == "env.get"
        ));
    }

    #[test]
    fn maybe_resolve_calls_normalizes_arity_suffixed_nyash_console_as_extern() {
        let _lock = env_guard().lock().expect("env lock");
        std::env::remove_var("HAKO_MIR_BUILDER_CALL_RESOLVE");
        let mut module = test_module_with_call_name("nyash.console.log/1");
        maybe_resolve_calls(&mut module, &BTreeMap::new());

        assert!(matches!(
            entry_call(&module),
            MirInstruction::Call {
                callee: Some(Callee::Extern(name)),
                ..
            } if name == "nyash.console.log"
        ));
    }

    #[test]
    fn maybe_resolve_calls_keeps_unqualified_names_unresolved_when_toggle_is_off() {
        let _lock = env_guard().lock().expect("env lock");
        std::env::remove_var("HAKO_MIR_BUILDER_CALL_RESOLVE");
        let mut module = test_module_with_call_name("helper");
        maybe_resolve_calls(&mut module, &BTreeMap::new());

        assert!(matches!(
            entry_call(&module),
            MirInstruction::Call { callee: None, .. }
        ));
    }
}
