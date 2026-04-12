/*!
 * Semantic simplification bundle owner seam.
 *
 * This module is the first top-level MIR transform owner for the semantic
 * simplification lane. In this cut it owns the already-landed DCE and CSE
 * passes without changing their behavior. Future `SimplifyCFG`, `SCCP`, and
 * jump-threading slices should enter through this seam instead of extending the
 * optimizer pipeline with more direct pass wiring.
 */

use crate::mir::{optimizer_stats::OptimizationStats, MirModule};

pub fn apply(module: &mut MirModule) -> OptimizationStats {
    let mut stats = OptimizationStats::new();

    stats.dead_code_eliminated += crate::mir::passes::dce::eliminate_dead_code(module);
    stats.cse_eliminated += crate::mir::passes::cse::eliminate_common_subexpressions(module);

    stats
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

        let stats = apply(&mut module);
        assert_eq!(stats.cse_eliminated, 1);
    }
}
