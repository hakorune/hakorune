//! Test-only reference for the future proof-backed suffix-view port.
//!
//! Production driver, router, and raw port policy remain untouched here. This
//! module lets callable-result tests carry the exact classification product to
//! the existing driver and compare it with both the raw port and an explicit
//! always-none non-reference.

use std::cell::RefCell;

use crate::ast::ASTNode;
use crate::mir::callable_result_representation::CallableResultBodySuffixDecisionV1;
use crate::mir::callable_result_representation::VerifiedCallableResultInactiveBodySuffixV1;
use crate::mir::{BasicBlockId, MirBuilder, MirInstruction, ValueId};

use super::block_driver::{drive_legacy_block_v1, LegacyBlockDescentPortV1};

pub(crate) enum BlockSuffixParityInputV1<'plan> {
    Raw,
    Classified(Vec<CallableResultBodySuffixDecisionV1<'plan>>),
    AlwaysNone,
    RejectAt { index: usize, message: &'static str },
}

#[derive(Clone, Copy)]
pub(crate) enum StatementDescentReferenceV1 {
    Actual,
    RecordOnly,
}

#[derive(Debug, PartialEq)]
struct BlockSnapshotV1 {
    id: BasicBlockId,
    instructions: Vec<MirInstruction>,
    terminator: Option<MirInstruction>,
    predecessors: Vec<BasicBlockId>,
    successors: Vec<BasicBlockId>,
    reachable: bool,
    sealed: bool,
}

#[derive(Debug)]
pub(crate) struct BlockDriverParityOutcomeV1 {
    pub(crate) output: Result<ValueId, String>,
    pub(crate) route_demand_indices: Vec<usize>,
    pub(crate) lowered_indices: Vec<usize>,
    pub(crate) instruction_count: usize,
    pub(crate) lexical_scope_depth: usize,
    current_block: Option<BasicBlockId>,
    entry_block: BasicBlockId,
    next_value_id: u32,
    blocks: Vec<BlockSnapshotV1>,
}

impl PartialEq for BlockDriverParityOutcomeV1 {
    fn eq(&self, other: &Self) -> bool {
        // Route-demand and statement-descent events are asserted separately.
        // Equality here is intentionally the normalized execution snapshot.
        self.output == other.output
            && self.current_block == other.current_block
            && self.entry_block == other.entry_block
            && self.next_value_id == other.next_value_id
            && self.instruction_count == other.instruction_count
            && self.lexical_scope_depth == other.lexical_scope_depth
            && self.blocks == other.blocks
    }
}

struct ClassifiedSuffixReferencePortV1<'plan, 'syntax> {
    statements: &'syntax [ASTNode],
    input: BlockSuffixParityInputV1<'plan>,
    descent: StatementDescentReferenceV1,
    route_demand_indices: RefCell<Vec<usize>>,
    lowered_indices: Vec<usize>,
}

impl<'plan> LegacyBlockDescentPortV1 for ClassifiedSuffixReferencePortV1<'plan, '_> {
    type SuffixInput<'a>
        = &'a VerifiedCallableResultInactiveBodySuffixV1<'plan>
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn suffix_route_input(&self, index: usize) -> Result<Option<Self::SuffixInput<'_>>, String> {
        self.route_demand_indices.borrow_mut().push(index);
        if let BlockSuffixParityInputV1::RejectAt {
            index: rejected,
            message,
        } = &self.input
        {
            if index == *rejected {
                return Err((*message).to_string());
            }
        }
        Ok(match &self.input {
            BlockSuffixParityInputV1::Classified(decisions) => match &decisions[index] {
                CallableResultBodySuffixDecisionV1::Inactive(proof) => Some(proof),
                CallableResultBodySuffixDecisionV1::Active { .. } => None,
            },
            BlockSuffixParityInputV1::AlwaysNone => None,
            BlockSuffixParityInputV1::RejectAt { .. } => None,
            BlockSuffixParityInputV1::Raw => unreachable!("raw input bypasses the reference port"),
        })
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        index: usize,
    ) -> Result<ValueId, String> {
        self.lowered_indices.push(index);
        match self.descent {
            StatementDescentReferenceV1::Actual => {
                super::block_stmt::build_statement(builder, self.statements[index].clone())
            }
            StatementDescentReferenceV1::RecordOnly => {
                crate::mir::builder::emission::constant::emit_integer(builder, index as i64)
            }
        }
    }
}

pub(crate) fn run_block_suffix_parity_reference_v1<'plan>(
    statements: &[ASTNode],
    input: BlockSuffixParityInputV1<'plan>,
    descent: StatementDescentReferenceV1,
    function_name: &str,
) -> BlockDriverParityOutcomeV1 {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(function_name.to_string());

    let (output, route_demand_indices, lowered_indices) = match input {
        BlockSuffixParityInputV1::Raw => (
            super::block_stmt::build_block(&mut builder, statements.to_vec()),
            Vec::new(),
            Vec::new(),
        ),
        input => {
            let mut port = ClassifiedSuffixReferencePortV1 {
                statements,
                input,
                descent,
                route_demand_indices: RefCell::new(Vec::new()),
                lowered_indices: Vec::new(),
            };
            let output = drive_legacy_block_v1(&mut builder, &mut port);
            (
                output,
                port.route_demand_indices.into_inner(),
                port.lowered_indices,
            )
        }
    };

    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .expect("parity reference current function");
    let mut blocks = function
        .blocks
        .values()
        .map(|block| BlockSnapshotV1 {
            id: block.id,
            instructions: block.instructions.clone(),
            terminator: block.terminator.clone(),
            predecessors: block.predecessors.iter().copied().collect(),
            successors: block.successors.iter().copied().collect(),
            reachable: block.reachable,
            sealed: block.sealed,
        })
        .collect::<Vec<_>>();
    blocks.sort_by_key(|block| block.id);
    let instruction_count = blocks
        .iter()
        .map(|block| block.instructions.len() + usize::from(block.terminator.is_some()))
        .sum();

    BlockDriverParityOutcomeV1 {
        output,
        route_demand_indices,
        lowered_indices,
        instruction_count,
        lexical_scope_depth: builder.scope_ctx.lexical_scope_stack.len(),
        current_block: builder.current_block,
        entry_block: function.entry_block,
        next_value_id: function.next_value_id,
        blocks,
    }
}
