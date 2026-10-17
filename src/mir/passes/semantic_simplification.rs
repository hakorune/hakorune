/*!
 * Semantic simplification bundle owner seam.
 *
 * This module is the top-level MIR transform owner for the semantic
 * simplification lane. The first cut only bundled landed DCE/CSE behavior.
 * The current cut adds the first narrow structural `SimplifyCFG` slice,
 * including copied-constant branch folding, constant compare folding, and
 * empty trampoline jump-threading, including the trivial-PHI target rewrite
 * cut, before CFG merge.
 */

use crate::mir::{optimizer_stats::OptimizationStats, MirModule};

pub fn apply(module: &mut MirModule) -> OptimizationStats {
    apply_with_required_roots(module, &Default::default())
}

pub(in crate::mir) fn apply_with_required_roots(
    module: &mut MirModule,
    roots: &crate::mir::passes::dce::RequiredLivenessRoots,
) -> OptimizationStats {
    let mut stats = OptimizationStats::new();

    // The same source-consumed roots also constrain producer replacement.
    // Borrow their physical dependency closure before any CFG/CSE rewrite;
    // this grants no source permission and is not a second source census.
    let preserved = producer_dependency_closure(module, roots);
    stats.cfg_simplified += crate::mir::passes::simplify_cfg::simplify_with_preserved_producers(module, &preserved);
    stats.dead_code_eliminated +=
        crate::mir::passes::dce::eliminate_dead_code_with_required_roots(module, roots);
    stats.cse_eliminated += crate::mir::passes::cse::eliminate_with_preserved_producers(module, &preserved);

    stats
}

/// Physical liveness dependencies of already-validated source producers.
/// Following operands preserves their identities; it never proves semantics.
fn producer_dependency_closure(
    module: &MirModule,
    roots: &crate::mir::passes::dce::RequiredLivenessRoots,
) -> crate::mir::passes::dce::RequiredLivenessRoots {
    let mut preserved = roots.clone();
    for (name, required) in &mut preserved {
        let Some(function) = module.functions.get(name) else { continue };
        let mut operands = std::collections::BTreeMap::new();
        for row in function.blocks.values().flat_map(|block| block.all_instructions()) {
            if let Some(dst) = row.dst_value() {
                operands.entry(dst).or_insert_with(Vec::new).extend(row.used_values());
            }
        }
        let mut pending: Vec<_> = required.iter().copied().collect();
        while let Some(value) = pending.pop() {
            for operand in operands.get(&value).into_iter().flatten() {
                if required.insert(*operand) { pending.push(*operand); }
            }
        }
    }
    preserved
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::Span;
    use crate::mir::{
        BasicBlockId, BinaryOp, ConstValue, EffectMask, FunctionSignature, MirFunction,
        MirInstruction, MirModule, MirType, ValueId,
    };

    #[test]
    fn bundle_runs_landed_dce() {
        let mut module = MirModule::new("semantic_simplification_dce".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(1),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(7),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(2)),
        });
        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::Integer);
        module.add_function(function);

        let stats = apply(&mut module);
        assert_eq!(stats.dead_code_eliminated, 1);
    }

    #[test]
    fn bundle_runs_landed_cse() {
        let mut module = MirModule::new("semantic_simplification_cse".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Bool,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Const {
            dst: ValueId(2),
            value: ConstValue::Integer(3),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(3),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::BinOp {
            dst: ValueId(4),
            op: BinaryOp::Add,
            lhs: ValueId(1),
            rhs: ValueId(2),
        });
        block.instruction_spans.push(Span::unknown());
        block.instructions.push(MirInstruction::Compare {
            dst: ValueId(5),
            op: crate::mir::CompareOp::Eq,
            lhs: ValueId(3),
            rhs: ValueId(4),
        });
        block.instruction_spans.push(Span::unknown());
        block.set_terminator(MirInstruction::Return {
            value: Some(ValueId(5)),
        });
        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(2), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(3), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(4), MirType::Integer);
        function
            .metadata
            .value_types
            .insert(ValueId(5), MirType::Bool);
        module.add_function(function);

        let mut preserved = module.clone();
        let roots = std::collections::BTreeMap::from([("main".to_owned(),
            std::collections::BTreeSet::from([ValueId(4)]))]);
        let protected_stats = crate::test_support::with_env_var("NYASH_LLVM_FAST_INT", "1", ||
            apply_with_required_roots(&mut preserved, &roots));
        assert_eq!(protected_stats.cse_eliminated, 0);
        assert!(preserved.functions["main"].blocks.values().flat_map(|b| b.all_instructions())
            .any(|row| matches!(row, MirInstruction::BinOp { dst: ValueId(4), .. })));

        let stats =
            crate::test_support::with_env_var("NYASH_LLVM_FAST_INT", "1", || apply(&mut module));
        assert_eq!(stats.cse_eliminated, 1);
    }

    #[test]
    fn bundle_runs_first_simplify_cfg_cut() {
        let mut module = MirModule::new("semantic_simplification_cfg".to_string());
        let signature = FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId(0));
        {
            let block = function.blocks.get_mut(&BasicBlockId(0)).expect("entry");
            block.set_terminator(MirInstruction::Jump {
                target: BasicBlockId(1),
                edge_args: None,
            });
        }

        let mut middle = crate::mir::BasicBlock::new(BasicBlockId(1));
        middle.instructions.push(MirInstruction::Const {
            dst: ValueId(1),
            value: ConstValue::Integer(9),
        });
        middle.instruction_spans.push(Span::unknown());
        middle.set_terminator(MirInstruction::Return {
            value: Some(ValueId(1)),
        });
        function.add_block(middle);
        function
            .metadata
            .value_types
            .insert(ValueId(1), MirType::Integer);
        function.update_cfg();
        module.add_function(function);

        let stats = apply(&mut module);
        assert_eq!(stats.cfg_simplified, 1);

        let function = module.functions.get("main").expect("main");
        assert_eq!(function.blocks.len(), 1);
        let entry = function.blocks.get(&BasicBlockId(0)).expect("entry");
        assert!(matches!(
            entry.terminator,
            Some(MirInstruction::Return {
                value: Some(value)
            }) if value == ValueId(1)
        ));
    }
}
