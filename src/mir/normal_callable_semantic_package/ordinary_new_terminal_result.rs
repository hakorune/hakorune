//! Physical consumption of the source-issued terminal Pair result.
//!
//! The relation remains source-only. This module retains only the one-way
//! physical progress needed to prove it was emitted without raw AST descent.
use super::*;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;

#[derive(Debug)]
pub(super) enum Progress {
    Pending,
    Reserved {
        reads: [(OwnedExprSiteV1, ValueId, CanonicalFieldRefV1); 2],
    },
    AddEmitted {
        result: ValueId,
        block: BasicBlockId,
        instruction: MirInstruction,
    },
    Completed {
        result: ValueId,
        block: BasicBlockId,
        instruction: MirInstruction,
    },
}

pub(crate) struct PreparedTerminalI64AddReturnV1 {
    pub(crate) reads: [(OwnedExprSiteV1, ValueId, CanonicalFieldRefV1); 2],
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_terminal_i64_add_return(
        &self,
        owner: FunctionOwnerIdV1,
        return_site: &SourceNodeSiteV1,
        mut resolve_binding: impl FnMut(BindingRefV1, &SourceNodeSiteV1) -> Result<ValueId, String>,
    ) -> Result<Option<PreparedTerminalI64AddReturnV1>, String> {
        let Some(relation) = self.terminal_result.as_ref() else {
            return Ok(None);
        };
        if relation.owner() != owner || relation.return_site().node() != return_site {
            return Err(fault("return-site-mismatch"));
        }
        if !matches!(*self.terminal_result_progress.borrow(), Progress::Pending) {
            return Err(fault("duplicate-reservation"));
        }
        let reads = relation.field_reads();
        let receiver_sites = {
            let rows = self.field_reads.borrow();
            let receiver = |site: &OwnedExprSiteV1| {
                rows.get(site)
                    .map(|row| row.receiver_site.node().clone())
                    .ok_or_else(|| fault("field-read-missing"))
            };
            [receiver(&reads[0])?, receiver(&reads[1])?]
        };
        let first = self
            .take_terminal_field_read(&reads[0], |binding| {
                resolve_binding(binding, &receiver_sites[0])
            })?
            .ok_or_else(|| fault("first-field-read-missing"))?;
        let second = self
            .take_terminal_field_read(&reads[1], |binding| {
                resolve_binding(binding, &receiver_sites[1])
            })?
            .ok_or_else(|| fault("second-field-read-missing"))?;
        let reads = [
            (reads[0].clone(), first.0, first.1),
            (reads[1].clone(), second.0, second.1),
        ];
        *self.terminal_result_progress.borrow_mut() = Progress::Reserved {
            reads: reads.clone(),
        };
        Ok(Some(PreparedTerminalI64AddReturnV1 { reads }))
    }

    pub(crate) fn record_terminal_i64_add(
        &self,
        block: BasicBlockId,
        result: ValueId,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<(), String> {
        let expected = {
            let progress = self.terminal_result_progress.borrow();
            let Progress::Reserved { reads: reserved } = &*progress else {
                return Err(fault("add-without-reservation"));
            };
            let reads = self.field_reads.borrow();
            let mut values = Vec::with_capacity(2);
            for (site, _, _) in reserved {
                let Some(row) = reads.get(site) else {
                    unreachable!()
                };
                let field_reads::Progress::Emitted(_, MirInstruction::ObjectFieldGet { dst, .. }) =
                    row.progress
                else {
                    return Err(fault("field-read-not-emitted"));
                };
                values.push(dst);
            }
            [values[0], values[1]]
        };
        let progress = &mut *self.terminal_result_progress.borrow_mut();
        if !matches!(*progress, Progress::Reserved { .. }) {
            return Err(fault("add-without-reservation"));
        }
        if [lhs, rhs] != expected {
            return Err(fault("add-operand-order"));
        }
        *progress = Progress::AddEmitted {
            result,
            block,
            instruction: MirInstruction::BinOp {
                dst: result,
                op: crate::mir::BinaryOp::Add,
                lhs,
                rhs,
            },
        };
        Ok(())
    }

    pub(crate) fn complete_terminal_i64_add_return(&self, result: ValueId) -> Result<(), String> {
        let progress = &mut *self.terminal_result_progress.borrow_mut();
        let Progress::AddEmitted {
            result: expected,
            block,
            instruction,
        } = std::mem::replace(progress, Progress::Pending)
        else {
            return Err(fault("complete-without-add"));
        };
        if expected != result {
            return Err(fault("return-result-mismatch"));
        }
        *progress = Progress::Completed {
            result,
            block,
            instruction,
        };
        Ok(())
    }

    pub(crate) fn terminal_result_blocks_raw_field_read(&self, site: &OwnedExprSiteV1) -> bool {
        self.terminal_result.as_ref().is_some_and(|relation| {
            relation.field_reads().contains(site)
                && matches!(*self.terminal_result_progress.borrow(), Progress::Pending)
        })
    }

    pub(super) fn terminal_result_complete(&self) -> bool {
        self.terminal_result.is_none()
            || matches!(
                *self.terminal_result_progress.borrow(),
                Progress::Completed { .. }
            )
    }

    pub(super) fn validate_terminal_i64_add_return(
        &self,
        owner: FunctionOwnerIdV1,
        function: &MirFunction,
    ) -> Result<(), String> {
        let Some(relation) = self.terminal_result.as_ref() else {
            return Ok(());
        };
        if relation.owner() != owner {
            return Err(fault("foreign-owner"));
        }
        let Progress::Completed {
            result,
            block,
            ref instruction,
        } = *self.terminal_result_progress.borrow()
        else {
            return Err(fault("unconsumed"));
        };
        let MirInstruction::BinOp {
            dst,
            op: crate::mir::BinaryOp::Add,
            lhs,
            rhs,
        } = instruction
        else {
            return Err(fault("add-kind-drift"));
        };
        if *dst != result {
            return Err(fault("add-result-drift"));
        }
        let reads = self.field_reads.borrow();
        let mut operands = Vec::new();
        for site in relation.field_reads() {
            let Some(row) = reads.get(site) else {
                return Err(fault("field-read-missing"));
            };
            let field_reads::Progress::Emitted(_, MirInstruction::ObjectFieldGet { dst, .. }) =
                row.progress
            else {
                return Err(fault("field-read-not-emitted"));
            };
            operands.push(dst);
        }
        if [*lhs, *rhs] != [operands[0], operands[1]] {
            return Err(fault("add-operand-drift"));
        }
        if !function
            .blocks
            .get(&block)
            .is_some_and(|row| row.all_instructions().any(|actual| actual == instruction))
        {
            return Err(fault("add-binding-drift"));
        }
        if !function.blocks.values().any(|block| block.all_instructions().any(|instruction|
            matches!(instruction, MirInstruction::Return { value: Some(value) } if *value == result)))
        {
            return Err(fault("return-missing"));
        }
        Ok(())
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-terminal-result/{reason}]")
}
