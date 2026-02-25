use crate::ast::Span;
use crate::mir::optimizer::MirOptimizer;
use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::ssot::extern_call::extern_call as build_extern_call;
use crate::mir::{EffectMask, MirModule, SpannedInstruction};

fn idemp_enabled() -> bool {
    std::env::var("NYASH_MIR_DEV_IDEMP").ok().as_deref() == Some("1")
}

fn idemp_key(pass: &str, func_name: &str) -> String {
    format!("{}:{}", pass, func_name)
}

fn idemp_already_done(module: &crate::mir::MirModule, key: &str) -> bool {
    module.metadata.dev_processed_markers.contains(key)
}

fn idemp_mark(module: &mut crate::mir::MirModule, key: String) {
    module.metadata.dev_processed_markers.insert(key);
}

pub fn normalize_python_helper_calls(
    _opt: &mut MirOptimizer,
    module: &mut MirModule,
) -> OptimizationStats {
    use crate::mir::MirInstruction as I;
    let mut stats = OptimizationStats::new();
    let pass_name = "normalize.python_helper_calls";
    let func_names: Vec<String> = module.functions.keys().cloned().collect();
    for fname in func_names {
        if idemp_enabled() {
            let key = idemp_key(pass_name, &fname);
            if idemp_already_done(module, &key) {
                continue;
            }
        }
        let function = match module.functions.get_mut(&fname) {
            Some(f) => f,
            None => continue,
        };
        for (_bb, block) in &mut function.blocks {
            let old_spanned = block.drain_spanned_instructions();
            let mut new_spanned = Vec::with_capacity(old_spanned.len());
            for SpannedInstruction { mut inst, span } in old_spanned {
                if let I::Call {
                    callee:
                        Some(crate::mir::Callee::Method {
                            receiver: Some(receiver),
                            method,
                            ..
                        }),
                    args,
                    ..
                } = &mut inst
                {
                    if method == "getattr" && args.len() >= 2 {
                        let new_recv = args[0];
                        args.remove(0);
                        *receiver = new_recv;
                        stats.intrinsic_optimizations += 1;
                    } else if method == "call" && !args.is_empty() {
                        let new_recv = args[0];
                        args.remove(0);
                        *receiver = new_recv;
                        stats.intrinsic_optimizations += 1;
                    }
                }
                new_spanned.push(SpannedInstruction { inst, span });
            }

            let (insts, spans): (Vec<_>, Vec<_>) =
                new_spanned.into_iter().map(|sp| (sp.inst, sp.span)).unzip();
            block.instructions = insts;
            block.instruction_spans = spans;
            block.effects = block
                .instructions
                .iter()
                .chain(block.terminator.iter())
                .fold(EffectMask::PURE, |mask, inst| mask | inst.effects());
        }
        if idemp_enabled() {
            let key = idemp_key(pass_name, &fname);
            idemp_mark(module, key);
        }
    }
    stats
}

pub fn normalize_legacy_instructions(
    _opt: &mut MirOptimizer,
    module: &mut MirModule,
) -> OptimizationStats {
    use crate::mir::MirInstruction as I;
    let stats = OptimizationStats::new();
    let rw_dbg = crate::config::env::rewrite_debug();
    let rw_sp = crate::config::env::rewrite_safepoint();
    let rw_future = crate::config::env::rewrite_future();
    let pass_name = "normalize.legacy_instructions";
    let func_names: Vec<String> = module.functions.keys().cloned().collect();
    for fname in func_names {
        if idemp_enabled() {
            let key = idemp_key(pass_name, &fname);
            if idemp_already_done(module, &key) {
                continue;
            }
        }
        let function = match module.functions.get_mut(&fname) {
            Some(f) => f,
            None => continue,
        };
        for (_bb, block) in &mut function.blocks {
            let old_spanned = block.drain_spanned_instructions();
            let mut new_spanned = Vec::with_capacity(old_spanned.len());
            for SpannedInstruction { inst, span } in old_spanned {
                let rewritten = match inst {
                    I::Debug { .. } if !rw_dbg => None,
                    I::Safepoint if !rw_sp => None,
                    I::FutureNew { dst, value } if rw_future => Some(build_extern_call(
                        Some(dst),
                        "env.future".to_string(),
                        "new".to_string(),
                        vec![value],
                        crate::mir::EffectMask::PURE.add(crate::mir::Effect::Io),
                    )),
                    I::FutureSet { future, value } if rw_future => Some(build_extern_call(
                        None,
                        "env.future".to_string(),
                        "set".to_string(),
                        vec![future, value],
                        crate::mir::EffectMask::PURE.add(crate::mir::Effect::Io),
                    )),
                    I::Await { dst, future } if rw_future => Some(build_extern_call(
                        Some(dst),
                        "env.future".to_string(),
                        "await".to_string(),
                        vec![future],
                        crate::mir::EffectMask::PURE.add(crate::mir::Effect::Io),
                    )),
                    other => Some(other),
                };
                if let Some(inst) = rewritten {
                    new_spanned.push(SpannedInstruction { inst, span });
                }
            }

            let terminator = block.terminator.take();
            let terminator_span = block.terminator_span.take();
            if let Some(term) = terminator {
                let span = terminator_span.unwrap_or_else(Span::unknown);
                block.set_terminator_with_span(term, span);
            }

            let (insts, spans): (Vec<_>, Vec<_>) =
                new_spanned.into_iter().map(|sp| (sp.inst, sp.span)).unzip();
            block.instructions = insts;
            block.instruction_spans = spans;
            block.effects = block
                .instructions
                .iter()
                .chain(block.terminator.iter())
                .fold(EffectMask::PURE, |mask, inst| mask | inst.effects());
        }
        if idemp_enabled() {
            let key = idemp_key(pass_name, &fname);
            idemp_mark(module, key);
        }
    }
    stats
}

pub fn normalize_ref_field_access(
    _opt: &mut MirOptimizer,
    module: &mut MirModule,
) -> OptimizationStats {
    let stats = OptimizationStats::new();
    let pass_name = "normalize.ref_field_access";
    let func_names: Vec<String> = module.functions.keys().cloned().collect();
    for fname in func_names {
        if idemp_enabled() {
            let key = idemp_key(pass_name, &fname);
            if idemp_already_done(module, &key) {
                continue;
            }
        }
        if idemp_enabled() {
            let key = idemp_key(pass_name, &fname);
            idemp_mark(module, key);
        }
    }
    stats
}
