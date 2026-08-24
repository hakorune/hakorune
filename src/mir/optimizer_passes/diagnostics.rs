use crate::mir::optimizer::MirOptimizer;
use crate::mir::optimizer_stats::OptimizationStats;
use crate::mir::{MirInstruction, MirModule};

/// Diagnostic: detect unlowered is/as/isType/asType after Builder
pub fn diagnose_unlowered_type_ops(
    opt: &mut MirOptimizer,
    module: &MirModule,
) -> OptimizationStats {
    let mut stats = OptimizationStats::new();
    let diag_on = opt.debug_enabled() || crate::config::env::opt_diag();
    for (fname, function) in &module.functions {
        let mut count = 0usize;
        for (_bb, block) in &function.blocks {
            for inst in &block.instructions {
                match inst {
                    MirInstruction::Call {
                        callee: Some(crate::mir::Callee::Method { method, .. }),
                        ..
                    } if method == "is"
                        || method == "as"
                        || method == "isType"
                        || method == "asType" =>
                    {
                        count += 1;
                    }
                    _ => {}
                }
            }
        }
        if count > 0 {
            stats.diagnostics_reported += count;
            if diag_on {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[OPT][DIAG] Function '{}' has {} unlowered type-op calls",
                    fname, count
                ));
            }
        }
    }
    stats
}

#[cfg(test)]
mod tests {
    use super::diagnose_unlowered_type_ops;
    use crate::mir::definitions::call_unified::{CalleeBoxKind, TypeCertainty};
    use crate::mir::{
        BasicBlockId, Callee, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, ValueId,
    };

    #[test]
    fn diagnostics_observe_typed_method_without_legacy_func_const_scan() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "diagnostic_call_test/0".to_string(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let block = function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry block");
        block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(9),
            value: ConstValue::String("isType".to_string()),
        });
        block.instructions.push(MirInstruction::Call {
            dst: None,
            func: ValueId::new(9),
            callee: None,
            args: vec![],
            effects: EffectMask::PURE,
        });
        block.instructions.push(MirInstruction::Call {
            dst: None,
            func: ValueId::new(99),
            callee: Some(Callee::Method {
                box_name: "Box".to_string(),
                method: "isType".to_string(),
                receiver: None,
                certainty: TypeCertainty::Known,
                box_kind: CalleeBoxKind::UserDefined,
            }),
            args: vec![],
            effects: EffectMask::PURE,
        });
        let mut module = MirModule::new("diagnostic_call_test".to_string());
        module.add_function(function);

        let mut optimizer = crate::mir::optimizer::MirOptimizer::new();
        let stats = diagnose_unlowered_type_ops(&mut optimizer, &module);
        assert_eq!(stats.diagnostics_reported, 1);
    }
}

/// Diagnostic: detect lowered-away instructions that must not survive normalize pass.
pub fn diagnose_legacy_instructions(
    opt: &mut MirOptimizer,
    module: &MirModule,
) -> OptimizationStats {
    let mut stats = OptimizationStats::new();
    let diag_on = opt.debug_enabled()
        || crate::config::env::opt_diag()
        || crate::config::env::opt_diag_forbid_legacy();
    for (fname, function) in &module.functions {
        let mut count = 0usize;
        for (_bb, block) in &function.blocks {
            for inst in &block.instructions {
                if crate::mir::contracts::backend_core_ops::lowered_away_tag(inst).is_some() {
                    count += 1;
                }
            }
            if let Some(term) = &block.terminator {
                if crate::mir::contracts::backend_core_ops::lowered_away_tag(term).is_some() {
                    count += 1;
                }
            }
        }
        if count > 0 {
            stats.diagnostics_reported += count;
            if diag_on {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.warn(&format!(
                    "[OPT][DIAG] Function '{}' has {} legacy MIR ops: unify to Core‑13 (TypeOp/WeakRef/Barrier/BoxCall)",
                    fname, count
                ));
                if crate::config::env::opt_diag_forbid_legacy() {
                    panic!(
                        "NYASH_OPT_DIAG_FORBID_LEGACY=1: legacy MIR ops detected in '{}': {}",
                        fname, count
                    );
                }
            }
        }
    }
    stats
}
