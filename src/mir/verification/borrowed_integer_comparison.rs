//! Match the Resolve-owned comparison proof against the final CFG and reads.
use super::VerificationError;
use hakorune_mir_defs::PhysicalCallableLaneCarrierV1;
use crate::mir::instruction::InvokeOperation;
use crate::mir::resolved_semantics::borrowed_integer_comparison::BorrowedIntegerComparisonSourceV1;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeSet;

#[derive(Debug, Clone)]
pub(crate) struct ComparisonBinding {
    pub(crate) source: BorrowedIntegerComparisonSourceV1,
    pub(crate) tagged: ValueId,
    pub(crate) observed: ValueId,
    pub(crate) extracted: ValueId,
    pub(crate) other: ValueId,
    pub(crate) result: ValueId,
    pub(crate) support: Vec<(BasicBlockId, MirInstruction)>,
    pub(crate) unwind: Vec<InvokeOperation>,
}

impl ComparisonBinding {
    pub(crate) fn for_result(function: &MirFunction, result: ValueId) -> Option<&Self> {
        function
            .metadata
            .borrowed_integer_comparisons
            .iter()
            .find(|proof| proof.result == result)
    }

    pub(crate) fn original_operands(&self) -> (ValueId, ValueId) {
        if self.source.tagged_on_left() {
            (self.observed, self.other)
        } else {
            (self.other, self.observed)
        }
    }
}

pub(in crate::mir) fn check(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    verified_sites(function).map(|_| ())
}

pub(in crate::mir) fn verified_sites(
    function: &MirFunction,
) -> Result<Vec<((BasicBlockId, usize), &ComparisonBinding)>, Vec<VerificationError>> {
    let mut sites = Vec::new();
    let mut seen = BTreeSet::new();
    for (&block, body) in &function.blocks {
        let Some(MirInstruction::Invoke {
            operation: InvokeOperation::DynamicBorrowedInteger { tagged },
            normal_landing,
            fault_landing,
            fault_frame,
        }) = body.terminator.as_ref()
        else {
            continue;
        };
        let check = (|| {
            let span = body.terminator_span?;
            let normal = function.blocks.get(normal_landing)?;
            if normal_landing == fault_landing || !function.blocks.contains_key(fault_landing) {
                return None;
            }
            let MirInstruction::InvokeNormalResult { invoke_block, dst } =
                normal.instructions.first()?
            else {
                return None;
            };
            if *invoke_block != block {
                return None;
            }
            let mut sources = function
                .metadata
                .borrowed_integer_comparisons
                .iter()
                .filter(|row| row.extracted == *dst);
            let source = sources.next()?;
            if sources.next().is_some()
                || source.source.span() != span
                || source.tagged != *tagged
                || !seen.insert(source.result)
                || !is_original_parameter(function, source, *tagged)
                || !is_original_read(function, *tagged, source.observed)
                || !check_unwind(function, *fault_frame, *fault_landing, &source.unwind)
            {
                return None;
            }
            let (lhs, rhs) = if source.source.tagged_on_left() {
                (*dst, source.other)
            } else {
                (source.other, *dst)
            };
            normal.instructions.iter().find(|row| {
                matches!(row,
                MirInstruction::Compare { dst, op, lhs: actual_lhs, rhs: actual_rhs }
                    if *dst == source.result && *op == source.source.operator()
                    && (*actual_lhs, *actual_rhs) == (lhs, rhs))
            })?;
            Some(source)
        })();
        let Some(source) = check else {
            return Err(vec![failure(block)]);
        };
        sites.push(((block, body.instructions.len()), source));
    }
    if seen.len() != function.metadata.borrowed_integer_comparisons.len() {
        return Err(vec![failure(function.entry_block)]);
    }
    Ok(sites)
}

pub(crate) fn is_original_read(
    function: &crate::mir::MirFunction,
    formal: ValueId,
    mut value: ValueId,
) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    while value != formal {
        if !seen.insert(value) {
            return false;
        }
        let mut copies = function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter_map(|row| match row {
                crate::mir::MirInstruction::Copy { dst, src } if *dst == value => Some(*src),
                _ => None,
            });
        let (Some(src), None) = (copies.next(), copies.next()) else {
            return false;
        };
        value = src;
    }
    true
}

fn check_unwind(
    function: &MirFunction,
    frame: ValueId,
    mut start: BasicBlockId,
    expected: &[InvokeOperation],
) -> bool {
    let tail = |mut block: BasicBlockId| {
        let mut seen = BTreeSet::new();
        loop {
            if !seen.insert(block) {
                return None;
            }
            let body = function.blocks.get(&block)?;
            if !body.instructions.is_empty() {
                return None;
            }
            match body.terminator.as_ref()? {
                MirInstruction::Jump {
                    target,
                    edge_args: None,
                } => block = *target,
                end => return Some((block, end)),
            }
        }
    };
    for next in expected {
        let Some((
            _,
            MirInstruction::Invoke {
                operation,
                fault_frame,
                normal_landing,
                fault_landing,
            },
        )) = tail(start)
        else {
            return false;
        };
        if operation != next
            || *fault_frame != frame
            || tail(*normal_landing).map(|row| row.0) != tail(*fault_landing).map(|row| row.0)
        {
            return false;
        }
        start = *normal_landing;
    }
    matches!(tail(start), Some((_, MirInstruction::ReturnFault { fault_frame })) if *fault_frame == frame)
}

fn is_original_parameter(function: &MirFunction, row: &ComparisonBinding, tagged: ValueId) -> bool {
    let mut matches = function
        .params
        .iter()
        .enumerate()
        .filter(|(_, value)| **value == tagged);
    let (Some((index, _)), None) = (matches.next(), matches.next()) else {
        return false;
    };
    let declarations = &function.metadata.declared_param_decls;
    let explicit_ordinal = declarations
        .iter()
        .take(index)
        .filter(|row| !row.implicit_receiver)
        .count();
    explicit_ordinal == row.source.ordinal() as usize
        && declarations
            .get(index)
            .is_some_and(|row| !row.implicit_receiver && row.declared_type_name.is_none())
        && function
            .metadata
            .physical_param_carriers
            .as_deref()
            .and_then(|carriers| carriers.get(index))
            == Some(&PhysicalCallableLaneCarrierV1::BorrowedTaggedValue)
}

fn failure(block: BasicBlockId) -> VerificationError {
    VerificationError::ControlFlowError {
        block,
        reason: "[freeze:contract][borrowed-integer/compare-correspondence]".into(),
    }
}
