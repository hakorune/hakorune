//! FastMemory MIRBuilder extension methods.
//!
//! These helpers own the mechanical side-table/fact updates and MemOp emission
//! used by the source lowering modules. They do not inspect source AST shape.

use crate::ast::{LiteralValue, Span};
use crate::mir::builder::{MirBuilder, MirInstruction, ValueId};
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

impl MirBuilder {
    pub(super) fn register_fastmem_region(
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

    pub(super) fn emit_fastmem_value_memop(
        &mut self,
        region: FastMemRegionId,
        kind: MemOpKind,
        operands: Vec<ValueId>,
    ) -> Result<ValueId, String> {
        self.emit_fastmem_value_memop_with_access(region, kind, operands, None)
    }

    pub(super) fn emit_fastmem_value_memop_with_access(
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

    pub(super) fn emit_fastmem_memop(
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

    pub(super) fn add_fastmem_table_length_fact(
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

    pub(super) fn add_fastmem_same_owner_fact(
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

    pub(super) fn add_fastmem_remote_owner_fact(
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

    pub(super) fn add_fastmem_block_next_fact(
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

    pub(super) fn add_fastmem_local_free_non_empty_fact(
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

    pub(super) fn add_fastmem_free_head_non_empty_fact(
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

    pub(super) fn add_fastmem_range_index_fact(
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

    pub(super) fn canonical_fastmem_range_upper_value(
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
}
