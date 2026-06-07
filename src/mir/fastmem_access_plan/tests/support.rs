use super::super::*;
use crate::ast::Span;
use crate::mir::function::{
    FastMemRegionMetadata, FastMemRegionOrigin, FastMemTableLengthFact,
    FastMemTableLengthPolicyKind, RangeIndexFact, RangeIndexFactOriginKind,
};
use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

pub(super) fn make_function(instructions: Vec<MirInstruction>) -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "Main.fastmem/0".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let block = function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block");
    for instruction in instructions {
        block.add_instruction(instruction);
    }
    function
        .metadata
        .fastmem_regions
        .push(FastMemRegionMetadata {
            id: FastMemRegionId::new(0),
            contract: "PageMapV0".to_string(),
            source_span: Span::unknown(),
            origin: FastMemRegionOrigin::SourceFastMemBlock,
            body_statement_count: 1,
            emitted_memop_count: function
                .blocks
                .get(&BasicBlockId::new(0))
                .map(|block| {
                    block
                        .instructions
                        .iter()
                        .filter(|instruction| matches!(instruction, MirInstruction::MemOp { .. }))
                        .count()
                })
                .unwrap_or(0),
        });
    function
}

pub(super) fn memop(
    kind: MemOpKind,
    dst: Option<ValueId>,
    operands: Vec<ValueId>,
    access: Option<MemOpAccess>,
) -> MirInstruction {
    MirInstruction::MemOp {
        region: FastMemRegionId::new(0),
        kind,
        dst,
        operands,
        access,
        effects: kind.effect_mask(),
    }
}

pub(super) fn table_length_fact() -> FastMemTableLengthFact {
    FastMemTableLengthFact {
        fact_id: 0,
        region: FastMemRegionId::new(0),
        table_id: "page_table".to_string(),
        table_value: ValueId::new(1),
        length_value: ValueId::new(50),
        resolved_length: Some(64),
        policy: FastMemTableLengthPolicyKind::ExplicitConstLen,
    }
}

pub(super) fn range_index_fact(fact_id: u32, index_value: ValueId) -> RangeIndexFact {
    RangeIndexFact {
        fact_id,
        origin_kind: RangeIndexFactOriginKind::CountingLoop,
        index_value,
        lower_value: ValueId::new(40),
        upper_exclusive_value: ValueId::new(50),
        body_bb: BasicBlockId::new(0),
        step: 1,
        end_exclusive: true,
        index_body_read_only: true,
        loop_carried_writes_supported: false,
    }
}
