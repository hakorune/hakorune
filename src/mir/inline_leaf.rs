use crate::mir::contracts::backend_core_ops::instruction_tag;
use crate::mir::{BasicBlockId, Callee, MirFunction, MirInstruction};

pub const DEFAULT_LEAF_INLINE_MAX_INSTRUCTIONS: usize = 8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InlineLeafViolation {
    pub tag: &'static str,
    pub block: Option<BasicBlockId>,
    pub instruction_index: Option<usize>,
    pub reason: String,
}

impl InlineLeafViolation {
    fn function(tag: &'static str, function: &MirFunction, reason: impl Into<String>) -> Self {
        Self {
            tag,
            block: None,
            instruction_index: None,
            reason: format!(
                "[inline-plan/{tag}] fn={} reason={}",
                function.signature.name,
                reason.into()
            ),
        }
    }

    fn instruction(
        tag: &'static str,
        function: &MirFunction,
        block: BasicBlockId,
        instruction_index: usize,
        inst: &MirInstruction,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            tag,
            block: Some(block),
            instruction_index: Some(instruction_index),
            reason: format!(
                "[inline-plan/{tag}] fn={} bb={} inst={} op={} reason={}",
                function.signature.name,
                block,
                instruction_index,
                instruction_tag(inst),
                reason.into()
            ),
        }
    }
}

pub fn is_supported_leaf_instruction(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Const { .. }
            | MirInstruction::UnaryOp { .. }
            | MirInstruction::BinOp { .. }
            | MirInstruction::Compare { .. }
            | MirInstruction::StaticDataLoad { .. }
            | MirInstruction::Copy { .. }
            | MirInstruction::Select { .. }
            | MirInstruction::TypeOp { .. }
    ) && inst.effects().is_pure()
}

pub fn check_leaf_inline_shape(
    function: &MirFunction,
    max_ir: Option<u32>,
) -> Vec<InlineLeafViolation> {
    let mut violations = Vec::new();
    if function.blocks.len() != 1 {
        violations.push(InlineLeafViolation::function(
            "required-not-verified",
            function,
            format!("expected one block, got {}", function.blocks.len()),
        ));
        return violations;
    }

    let Some(block) = function.blocks.get(&function.entry_block) else {
        violations.push(InlineLeafViolation::function(
            "required-not-verified",
            function,
            format!("missing entry block {}", function.entry_block),
        ));
        return violations;
    };

    if block.return_env.is_some() || block.return_env_layout.is_some() {
        violations.push(InlineLeafViolation::function(
            "required-not-verified",
            function,
            "return environment metadata is unsupported for leaf inline",
        ));
    }

    let budget = max_ir
        .map(|v| v as usize)
        .unwrap_or(DEFAULT_LEAF_INLINE_MAX_INSTRUCTIONS);
    if block.instructions.len() > budget {
        violations.push(InlineLeafViolation::function(
            "body-too-large",
            function,
            format!(
                "instruction_count={} budget={}",
                block.instructions.len(),
                budget
            ),
        ));
    }

    match &block.terminator {
        Some(MirInstruction::Return { .. }) => {}
        Some(inst) => violations.push(InlineLeafViolation::instruction(
            "required-not-verified",
            function,
            block.id,
            block.instructions.len(),
            inst,
            "terminator must be Return",
        )),
        None => violations.push(InlineLeafViolation::function(
            "required-not-verified",
            function,
            "missing Return terminator",
        )),
    }

    for (idx, inst) in block.instructions.iter().enumerate() {
        if is_supported_leaf_instruction(inst) {
            continue;
        }
        violations.push(classify_unsupported_instruction(
            function, block.id, idx, inst,
        ));
    }

    violations
}

fn classify_unsupported_instruction(
    function: &MirFunction,
    block: BasicBlockId,
    instruction_index: usize,
    inst: &MirInstruction,
) -> InlineLeafViolation {
    match inst {
        MirInstruction::Call {
            callee: Some(Callee::Global(name)),
            ..
        } if name == &function.signature.name => InlineLeafViolation::instruction(
            "recursive-cycle",
            function,
            block,
            instruction_index,
            inst,
            "self-recursive call is unsupported for required inline",
        ),
        MirInstruction::Call { callee: None, .. }
        | MirInstruction::Call {
            callee: Some(Callee::Method { .. }),
            ..
        }
        | MirInstruction::Call {
            callee: Some(Callee::Closure { .. }),
            ..
        } => InlineLeafViolation::instruction(
            "dynamic-dispatch",
            function,
            block,
            instruction_index,
            inst,
            "dynamic call is unsupported for required inline",
        ),
        MirInstruction::Call { .. } => InlineLeafViolation::instruction(
            "unsupported-call",
            function,
            block,
            instruction_index,
            inst,
            "nested call is unsupported in the first required inline row",
        ),
        _ => InlineLeafViolation::instruction(
            "required-not-verified",
            function,
            block,
            instruction_index,
            inst,
            "instruction is outside the first leaf inline vocabulary or is not pure",
        ),
    }
}
