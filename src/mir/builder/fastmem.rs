//! FastMemory source-region lowering.
//!
//! This module is the narrow MIRBuilder owner for `fastmem Contract { ... }`.
//! It records side-table region metadata and emits `MemOp` instructions for
//! the v0 memory dialect. It does not choose page-map strategy, backend route,
//! product activation, or provider/replacement-front policy.

mod branch;
mod calls;

use super::{MirBuilder, MirInstruction, ValueId};
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::builder::vars::assignment_resolver::AssignmentResolverBox;
use crate::mir::function::{
    FastMemBlockNextFact, FastMemBlockNextProofKind, FastMemFreeHeadNonEmptyFact,
    FastMemFreeHeadNonEmptyProofKind, FastMemLocalFreeNonEmptyFact,
    FastMemLocalFreeNonEmptyProofKind, FastMemRegionMetadata, FastMemRegionOrigin,
    FastMemRemoteOwnerFact, FastMemRemoteOwnerProofKind, FastMemSameOwnerFact,
    FastMemSameOwnerProofKind, FastMemTableLengthFact, FastMemTableLengthPolicyKind,
    RangeIndexFact, RangeIndexFactOriginKind,
};
use crate::mir::instruction::{FastMemRegionId, MemOpAccess, MemOpKind};
use crate::mir::loop_api::LoopBuilderApi;
use crate::mir::MirType;

use branch::lower_fastmem_if;
use calls::{lower_fastmem_function_call, lower_fastmem_method_call};

pub(in crate::mir::builder) fn build_fastmem_region(
    builder: &mut MirBuilder,
    contract: String,
    body: Vec<ASTNode>,
    span: Span,
) -> Result<ValueId, String> {
    let region = builder.register_fastmem_region(contract, span, body.len())?;
    let mut last_value = None;
    for stmt in body {
        last_value = Some(lower_fastmem_stmt(builder, region, stmt)?);
    }
    match last_value {
        Some(value) => Ok(value),
        None => crate::mir::builder::emission::constant::emit_void(builder),
    }
}

fn lower_fastmem_stmt(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    stmt: ASTNode,
) -> Result<ValueId, String> {
    builder.metadata_ctx.set_current_span(stmt.span());
    match stmt {
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => lower_fastmem_local(builder, region, variables, initial_values),
        ASTNode::Assignment { target, value, .. } => {
            lower_fastmem_assignment(builder, region, *target, *value)
        }
        ASTNode::Print { expression, .. }
        | ASTNode::Return {
            value: Some(expression),
            ..
        } => lower_fastmem_expr(builder, region, *expression),
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => lower_fastmem_if(builder, region, *condition, then_body, else_body),
        ASTNode::Return { value: None, .. } => {
            crate::mir::builder::emission::constant::emit_void(builder)
        }
        other => lower_fastmem_expr(builder, region, other),
    }
}

fn lower_fastmem_local(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    variables: Vec<String>,
    initial_values: Vec<Option<Box<ASTNode>>>,
) -> Result<ValueId, String> {
    let mut last = None;
    for (index, name) in variables.iter().enumerate() {
        let Some(Some(init)) = initial_values.get(index) else {
            return Err(format!(
                "[freeze:contract][fastmem/local_missing_initializer] name={}",
                name
            ));
        };
        let value = lower_fastmem_expr(builder, region, *init.clone())?;
        builder.declare_local_in_current_scope(name, value)?;
        last = Some(value);
    }
    last.ok_or_else(|| "[freeze:contract][fastmem/local_empty]".to_string())
}

fn lower_fastmem_assignment(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    target: ASTNode,
    value: ASTNode,
) -> Result<ValueId, String> {
    match target {
        ASTNode::Variable { name, .. } => {
            let value_id = lower_fastmem_expr(builder, region, value)?;
            AssignmentResolverBox::ensure_declared(builder, &name)?;
            builder.variable_ctx.variable_map.insert(name, value_id);
            Ok(value_id)
        }
        ASTNode::FieldAccess { object, field, .. } => {
            let base = lower_fastmem_expr(builder, region, *object)?;
            let value_id = lower_fastmem_expr(builder, region, value)?;
            builder.emit_fastmem_memop(
                region,
                MemOpKind::FieldStore,
                None,
                vec![base, value_id],
                Some(MemOpAccess::field(field)),
            )?;
            Ok(value_id)
        }
        ASTNode::Index {
            target,
            index,
            span,
        } => {
            let slot = lower_fastmem_expr(
                builder,
                region,
                ASTNode::Index {
                    target,
                    index,
                    span,
                },
            )?;
            let value_id = lower_fastmem_expr(builder, region, value)?;
            builder.emit_fastmem_memop(
                region,
                MemOpKind::FieldStore,
                None,
                vec![slot, value_id],
                None,
            )?;
            Ok(value_id)
        }
        other => Err(format!(
            "[freeze:contract][fastmem/unsupported_assignment_target] node={}",
            other.node_type()
        )),
    }
}

fn lower_fastmem_expr(
    builder: &mut MirBuilder,
    region: FastMemRegionId,
    expr: ASTNode,
) -> Result<ValueId, String> {
    builder.metadata_ctx.set_current_span(expr.span());
    match expr {
        ASTNode::Literal { value, .. } => lower_fastmem_literal(builder, value),
        ASTNode::Variable { name, .. } => builder.build_variable_access(name),
        ASTNode::Me { .. } => super::stmts::variable_stmt::build_me_expression(builder),
        ASTNode::BinaryOp {
            operator,
            left,
            right,
            ..
        } => {
            let lhs = lower_fastmem_expr(builder, region, *left)?;
            let rhs = lower_fastmem_expr(builder, region, *right)?;
            let kind = memop_kind_for_binary_operator(operator)?;
            builder.emit_fastmem_value_memop(region, kind, vec![lhs, rhs])
        }
        ASTNode::FunctionCall {
            name, arguments, ..
        } => lower_fastmem_function_call(builder, region, name, arguments),
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => lower_fastmem_method_call(builder, region, *object, method, arguments),
        ASTNode::Index { target, index, .. } => {
            let access = fastmem_table_access(&target);
            let base = lower_fastmem_expr(builder, region, *target)?;
            let idx = lower_fastmem_expr(builder, region, *index)?;
            builder.emit_fastmem_value_memop_with_access(
                region,
                MemOpKind::TableIndex,
                vec![base, idx],
                access,
            )
        }
        ASTNode::FieldAccess { object, field, .. } => {
            let base = lower_fastmem_expr(builder, region, *object)?;
            builder.emit_fastmem_value_memop_with_access(
                region,
                MemOpKind::FieldLoad,
                vec![base],
                Some(MemOpAccess::field(field)),
            )
        }
        other => Err(format!(
            "[freeze:contract][fastmem/unsupported_expr] node={}",
            other.node_type()
        )),
    }
}

fn lower_fastmem_literal(builder: &mut MirBuilder, value: LiteralValue) -> Result<ValueId, String> {
    match value {
        LiteralValue::Integer(_)
        | LiteralValue::TypedInteger { .. }
        | LiteralValue::Bool(_)
        | LiteralValue::Null
        | LiteralValue::Void => builder.build_literal(value),
        _ => Err("[freeze:contract][fastmem/unsupported_literal]".to_string()),
    }
}

fn memop_kind_for_binary_operator(operator: BinaryOperator) -> Result<MemOpKind, String> {
    match operator {
        BinaryOperator::Shr => Ok(MemOpKind::LogicalShr),
        BinaryOperator::BitAnd => Ok(MemOpKind::BitAnd),
        BinaryOperator::Add => Ok(MemOpKind::Add),
        BinaryOperator::Subtract => Ok(MemOpKind::Sub),
        _ => Err(format!(
            "[freeze:contract][fastmem/unsupported_binary_op] op={}",
            operator
        )),
    }
}

fn fastmem_table_access(target: &ASTNode) -> Option<MemOpAccess> {
    match target {
        ASTNode::Variable { name, .. } => Some(MemOpAccess::table(name.clone())),
        _ => None,
    }
}

impl MirBuilder {
    fn register_fastmem_region(
        &mut self,
        contract: String,
        source_span: Span,
        body_statement_count: usize,
    ) -> Result<FastMemRegionId, String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let id = FastMemRegionId::new(function.metadata.fastmem_regions.len() as u32);
        function
            .metadata
            .fastmem_regions
            .push(FastMemRegionMetadata {
                id,
                contract,
                source_span,
                origin: FastMemRegionOrigin::SourceFastMemBlock,
                body_statement_count,
                emitted_memop_count: 0,
            });
        Ok(id)
    }

    fn note_fastmem_memop(&mut self, region: FastMemRegionId) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let Some(metadata) = function
            .metadata
            .fastmem_regions
            .iter_mut()
            .find(|entry| entry.id == region)
        else {
            return Err(format!(
                "[freeze:contract][fastmem/unknown_region] region={}",
                region.0
            ));
        };
        metadata.emitted_memop_count += 1;
        Ok(())
    }

    fn emit_fastmem_value_memop(
        &mut self,
        region: FastMemRegionId,
        kind: MemOpKind,
        operands: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.emit_fastmem_value_memop_with_access(region, kind, operands, None)
    }

    fn emit_fastmem_value_memop_with_access(
        &mut self,
        region: FastMemRegionId,
        kind: MemOpKind,
        operands: Vec<ValueId>,
        access: Option<MemOpAccess>,
    ) -> Result<ValueId, String> {
        let dst = self.next_value_id();
        self.emit_fastmem_memop(region, kind, Some(dst), operands, access)?;
        self.type_ctx.value_types.insert(dst, MirType::Integer);
        Ok(dst)
    }

    fn emit_fastmem_memop(
        &mut self,
        region: FastMemRegionId,
        kind: MemOpKind,
        dst: Option<ValueId>,
        operands: Vec<ValueId>,
        access: Option<MemOpAccess>,
    ) -> Result<(), String> {
        self.note_fastmem_memop(region)?;
        self.emit_instruction(MirInstruction::MemOp {
            region,
            kind,
            dst,
            operands,
            access,
            effects: kind.effect_mask(),
        })
    }

    fn add_fastmem_table_length_fact(
        &mut self,
        region: FastMemRegionId,
        table_id: String,
        table_value: ValueId,
        length_value: ValueId,
        resolved_length: Option<u64>,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_table_length_facts.len() as u32;
        function
            .metadata
            .fastmem_table_length_facts
            .push(FastMemTableLengthFact {
                fact_id,
                region,
                table_id,
                table_value,
                length_value,
                resolved_length,
                policy: FastMemTableLengthPolicyKind::ExplicitConstLen,
            });
        Ok(())
    }

    fn add_fastmem_same_owner_fact(
        &mut self,
        region: FastMemRegionId,
        page_value: ValueId,
        proof_value: ValueId,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_same_owner_facts.len() as u32;
        function
            .metadata
            .fastmem_same_owner_facts
            .push(FastMemSameOwnerFact {
                fact_id,
                region,
                page_value,
                proof_value,
                proof_kind: FastMemSameOwnerProofKind::SourceAssumeOwnerEq,
                remote_owner_rejected: true,
            });
        Ok(())
    }

    fn add_fastmem_remote_owner_fact(
        &mut self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_remote_owner_facts.len() as u32;
        function
            .metadata
            .fastmem_remote_owner_facts
            .push(FastMemRemoteOwnerFact {
                fact_id,
                region,
                page_value,
                proof_kind: FastMemRemoteOwnerProofKind::SourceAssumeRemoteOwner,
                same_owner_rejected: true,
            });
        Ok(())
    }

    fn add_fastmem_block_next_fact(
        &mut self,
        region: FastMemRegionId,
        block_value: ValueId,
        proof_kind: FastMemBlockNextProofKind,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_block_next_facts.len() as u32;
        function
            .metadata
            .fastmem_block_next_facts
            .push(FastMemBlockNextFact {
                fact_id,
                region,
                block_value,
                next_field_id: "next".to_string(),
                proof_kind,
                writable: true,
                provenance_valid: true,
            });
        Ok(())
    }

    fn add_fastmem_local_free_non_empty_fact(
        &mut self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_local_free_non_empty_facts.len() as u32;
        function
            .metadata
            .fastmem_local_free_non_empty_facts
            .push(FastMemLocalFreeNonEmptyFact {
                fact_id,
                region,
                page_value,
                proof_kind: FastMemLocalFreeNonEmptyProofKind::SourceAssumeLocalFreeNonEmpty,
                non_empty: true,
            });
        Ok(())
    }

    fn add_fastmem_free_head_non_empty_fact(
        &mut self,
        region: FastMemRegionId,
        page_value: ValueId,
    ) -> Result<(), String> {
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.fastmem_free_head_non_empty_facts.len() as u32;
        function
            .metadata
            .fastmem_free_head_non_empty_facts
            .push(FastMemFreeHeadNonEmptyFact {
                fact_id,
                region,
                page_value,
                proof_kind: FastMemFreeHeadNonEmptyProofKind::SourceAssumeFreeHeadNonEmpty,
                non_empty: true,
            });
        Ok(())
    }

    fn add_fastmem_range_index_fact(
        &mut self,
        index_value: ValueId,
        upper_exclusive_value: ValueId,
    ) -> Result<(), String> {
        let body_bb = self.current_block()?;
        let lower_value = self.build_literal(LiteralValue::Integer(0))?;
        let function = self
            .scope_ctx
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        let fact_id = function.metadata.range_index_facts.len() as u32;
        function.metadata.range_index_facts.push(RangeIndexFact {
            fact_id,
            origin_kind: RangeIndexFactOriginKind::FastMemAssume,
            index_value,
            lower_value,
            upper_exclusive_value,
            body_bb,
            step: 1,
            end_exclusive: true,
            index_body_read_only: true,
            loop_carried_writes_supported: false,
        });
        Ok(())
    }

    fn canonical_fastmem_range_upper_value(
        &mut self,
        region: FastMemRegionId,
        resolved_upper: Option<u64>,
        fallback: ValueId,
    ) -> Result<ValueId, String> {
        let Some(resolved_upper) = resolved_upper else {
            return Ok(fallback);
        };
        let function = self
            .scope_ctx
            .current_function
            .as_ref()
            .ok_or_else(|| "[freeze:contract][fastmem/outside_function]".to_string())?;
        Ok(function
            .metadata
            .fastmem_table_length_facts
            .iter()
            .find(|fact| fact.region == region && fact.resolved_length == Some(resolved_upper))
            .map(|fact| fact.length_value)
            .unwrap_or(fallback))
    }
}

#[cfg(test)]
mod tests;
