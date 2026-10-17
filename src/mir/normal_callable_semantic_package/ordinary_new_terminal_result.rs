//! Physical consumption of the source-issued terminal Pair result.
//!
//! The relation remains source-only. This module retains only the one-way
//! physical progress needed to prove it was emitted without raw AST descent.
//! Progress is keyed by owner and exact exit site: one function or `return`
//! path can never satisfy another function or sibling exit.
use super::*;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use hakorune_mir_defs::CanonicalFieldRefV1;

#[derive(Debug)]
pub(super) enum Progress {
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
    pub(crate) owner: FunctionOwnerIdV1,
    pub(crate) return_site: SourceStmtSiteV1,
    pub(crate) reads: [(OwnedExprSiteV1, ValueId, CanonicalFieldRefV1); 2],
}

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn prepare_terminal_i64_add_return(
        &self,
        owner: FunctionOwnerIdV1,
        return_site: &SourceNodeSiteV1,
        mut resolve_binding: impl FnMut(BindingRefV1, &SourceNodeSiteV1) -> Result<ValueId, String>,
    ) -> Result<Option<PreparedTerminalI64AddReturnV1>, String> {
        let stmt_site = SourceStmtSiteV1::from_node(return_site.clone());
        let Some(relation) = self.terminal_i64_add_return_for_owner_at(owner, &stmt_site) else {
            return Ok(None);
        };
        let completion = self.completion_for_owner(owner)
            .ok_or_else(|| fault("completion-missing"))?;
        if relation.owner() != owner || completion.owner() != owner
            || !completion.explicit_sites().contains(&stmt_site) {
            return Err(fault("return-site-mismatch"));
        }
        if self
            .terminal_result_progress
            .borrow()
            .contains_key(&(owner, stmt_site.clone()))
        {
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
        self.terminal_result_progress.borrow_mut().insert(
            (owner, stmt_site.clone()),
            Progress::Reserved {
                reads: reads.clone(),
            },
        );
        Ok(Some(PreparedTerminalI64AddReturnV1 {
            owner,
            return_site: stmt_site,
            reads,
        }))
    }

    pub(crate) fn record_terminal_i64_add(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        block: BasicBlockId,
        result: ValueId,
        lhs: ValueId,
        rhs: ValueId,
    ) -> Result<(), String> {
        let expected = {
            let progress = self.terminal_result_progress.borrow();
            let Some(Progress::Reserved { reads: reserved }) = progress.get(&(owner, site.clone())) else {
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
        let mut progress = self.terminal_result_progress.borrow_mut();
        if !matches!(progress.get(&(owner, site.clone())), Some(Progress::Reserved { .. })) {
            return Err(fault("add-without-reservation"));
        }
        if [lhs, rhs] != expected {
            return Err(fault("add-operand-order"));
        }
        progress.insert(
            (owner, site.clone()),
            Progress::AddEmitted {
                result,
                block,
                instruction: MirInstruction::BinOp {
                    dst: result,
                    op: crate::mir::BinaryOp::Add,
                    lhs,
                    rhs,
                },
            },
        );
        Ok(())
    }

    pub(crate) fn complete_terminal_i64_add_return(
        &self,
        owner: FunctionOwnerIdV1,
        site: &SourceStmtSiteV1,
        result: ValueId,
    ) -> Result<(), String> {
        let mut progress = self.terminal_result_progress.borrow_mut();
        let Some(Progress::AddEmitted {
            result: expected,
            block,
            instruction,
        }) = progress.remove(&(owner, site.clone()))
        else {
            return Err(fault("complete-without-add"));
        };
        if expected != result {
            return Err(fault("return-result-mismatch"));
        }
        progress.insert(
            (owner, site.clone()),
            Progress::Completed {
                result,
                block,
                instruction,
            },
        );
        Ok(())
    }

    /// A raw field read must not reenter the receiver the terminal Add
    /// reserved at the same owner and exact exit, including child owners.
    pub(crate) fn terminal_result_blocks_raw_field_read(&self, site: &OwnedExprSiteV1) -> bool {
        self.terminal_relations_for_owner(site.owner()).into_iter().any(|relation| {
            matches!(relation, TerminalRelationV1::I64Add(row)
                if row.field_reads().contains(site)
                    && !self
                        .terminal_result_progress
                        .borrow()
                        .contains_key(&(row.owner(), row.return_site().clone())))
        })
    }

    /// Every retained I64Add exit must reach Completed — a multi-exit
    /// function cannot leave one `return a.f + b.f` unprepared.
    pub(super) fn terminal_result_complete(&self) -> bool {
        self.terminal_relation
            .values()
            .chain(self.terminal_relation_index.values().flat_map(|rows| rows.values()))
            .filter_map(|relation| match relation {
                TerminalRelationV1::I64Add(row) => Some((row.owner(), row.return_site().clone())),
                _ => None,
            })
            .all(|site| {
                matches!(
                    self.terminal_result_progress.borrow().get(&site),
                    Some(Progress::Completed { .. })
                )
            })
    }

    pub(super) fn terminal_i64_add_bindings_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let progress = self.terminal_result_progress.borrow();
        let reads = self.field_reads.borrow();
        let mut bindings = Vec::new();
        for relation in self.terminal_relations_for_owner(owner) {
            let TerminalRelationV1::I64Add(row) = relation else { continue };
            let Some(Progress::Completed { block, instruction, .. }) =
                progress.get(&(owner, row.return_site().clone()))
            else { return Err(fault("unconsumed")) };
            for site in row.field_reads() {
                let Some(field_reads::FieldRead { progress: field_reads::Progress::Emitted(block, instruction), .. }) = reads.get(site)
                else { return Err(fault("field-read-not-emitted")) };
                bindings.push((*block, instruction.clone()));
            }
            bindings.push((*block, instruction.clone()));
        }
        Ok(bindings)
    }

    pub(super) fn validate_terminal_i64_add_return(
        &self,
        owner: FunctionOwnerIdV1,
        _function: &MirFunction,
        mut check_binding: impl FnMut(BasicBlockId, &MirInstruction) -> Result<bool, String>,
    ) -> Result<(), String> {
        let reads = self.field_reads.borrow();
        let progress = self.terminal_result_progress.borrow();
        for relation in self.terminal_relations_for_owner(owner) {
            let TerminalRelationV1::I64Add(relation) = relation else {
                continue;
            };
            if relation.owner() != owner {
                return Err(fault("foreign-owner"));
            }
            let Some(Progress::Completed {
                result,
                block,
                instruction,
            }) = progress.get(&(owner, relation.return_site().clone()))
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
            if dst != result {
                return Err(fault("add-result-drift"));
            }
            let mut operands = Vec::new();
            for site in relation.field_reads() {
                let Some(row) = reads.get(site) else {
                    return Err(fault("field-read-missing"));
                };
                let field_reads::Progress::Emitted(read_block, read @ MirInstruction::ObjectFieldGet { dst, .. }) =
                    &row.progress
                else {
                    return Err(fault("field-read-not-emitted"));
                };
                if !check_binding(*read_block, read)? {
                    return Err(fault("field-read-binding-drift"));
                }
                operands.push(*dst);
            }
            if [*lhs, *rhs] != [operands[0], operands[1]] {
                return Err(fault("add-operand-drift"));
            }
            if !check_binding(*block, instruction)? {
                return Err(fault("add-binding-drift"));
            }
            let exits = self.root_exits.borrow();
            let Some(local_commit::RootHomeExitProgress::Emitted { bindings, .. }) =
                exits.get(&(owner, relation.return_site().clone().into()))
            else { return Err(fault("return-missing")) };
            let mut returns = bindings.iter().filter(|(_, instruction)|
                matches!(instruction, MirInstruction::Return { .. }));
            let Some((return_block, returned)) = returns.next() else {
                return Err(fault("return-missing"));
            };
            if returns.next().is_some()
                || !matches!(returned, MirInstruction::Return { value: Some(value) } if value == result)
                || !check_binding(*return_block, returned)?
            {
                return Err(fault("return-binding-drift"));
            }
        }
        Ok(())
    }
}

fn fault(reason: &str) -> String {
    format!("[freeze:contract][ordinary-terminal-result/{reason}]")
}
