//! Physical consumer for the bounded source-issued Map read Facts.
//!
//! The resolver Facts remain the only source authority.  This state only
//! matches an exact source site to the already-issued row and records which
//! rows have crossed the selected lowering owner.  Keys, indexes, and result
//! classes are taken from the Fact; syntax is used only to confirm that the
//! selected `get` route is still the same shape.

use std::collections::BTreeSet;

use super::normal_callable_semantic_lowering_state::CallableSemanticLoweringState;
use super::raw_structured_child_scope::PreparedRawChildSourceV1;
use super::recursive_child_lowering::{
    drive_legacy_expression_v1, RawInvocationChildPortV1, RecursiveChildLoweringPortV1,
};
use crate::ast::ASTNode;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::normal_callable_semantic_package::{
    MapReadFactV1, MapReadFactsV1, MapReadOperandV1, MapReadOperationV1, MapReadResultClassV1,
};
use crate::mir::resolved_semantics::OwnedExprSiteV1;
use crate::mir::{MirBuilder, MirInstruction, MirType, ValueId};

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct MapReadPhysicalConsumerV1 {
    facts: MapReadFactsV1,
    consumed: BTreeSet<OwnedExprSiteV1>,
    pending_array: Option<PendingArrayReadV1>,
}

#[derive(Debug, Clone)]
struct PendingArrayReadV1 {
    index_site: OwnedExprSiteV1,
    parent_map: ValueId,
    key: Box<str>,
}

impl MapReadPhysicalConsumerV1 {
    pub(in crate::mir::builder) fn new(facts: MapReadFactsV1) -> Self {
        Self {
            facts,
            consumed: BTreeSet::new(),
            pending_array: None,
        }
    }

    pub(in crate::mir::builder) fn begin_get(
        &self,
        site: &OwnedExprSiteV1,
        method: &str,
        arguments: &[ASTNode],
    ) -> Result<Option<MapReadFactV1>, String> {
        let Some(row) = self.facts.rows().iter().find(|row| row.site() == site) else {
            return Ok(None);
        };
        if method != "get" || arguments.len() != 1 {
            return Err(freeze("source-shape"));
        }
        if self.consumed.contains(site) {
            return Err(freeze("duplicate-site"));
        }
        Ok(Some(row.clone()))
    }

    pub(in crate::mir::builder) fn begin_first(
        &mut self,
        row: &MapReadFactV1,
        parent_map: ValueId,
    ) -> Result<(), String> {
        if row.operation() != MapReadOperationV1::MapLookup
            || row.result() != MapReadResultClassV1::ArrayView
        {
            return Err(freeze("first-row-shape"));
        }
        let MapReadOperandV1::Key(key) = row.operand() else {
            return Err(freeze("first-row-operand"));
        };
        let index = self
            .facts
            .rows()
            .iter()
            .find(|candidate| candidate.receiver_site() == row.site())
            .ok_or_else(|| freeze("index-row-missing"))?;
        if index.operation() != MapReadOperationV1::ArrayIndex
            || index.result() != MapReadResultClassV1::MapView
        {
            return Err(freeze("index-row-shape"));
        }
        if self.pending_array.is_some() || !self.consumed.insert(row.site().clone()) {
            return Err(freeze("first-row-consume"));
        }
        self.pending_array = Some(PendingArrayReadV1 {
            index_site: index.site().clone(),
            parent_map,
            key: key.clone(),
        });
        Ok(())
    }

    pub(in crate::mir::builder) fn array_index(
        &mut self,
        row: &MapReadFactV1,
        receiver: ValueId,
    ) -> Result<(ValueId, String, i64), String> {
        let pending = self
            .pending_array
            .as_ref()
            .ok_or_else(|| freeze("index-without-first"))?;
        if pending.index_site != *row.site()
            || pending.parent_map != receiver
            || row.operation() != MapReadOperationV1::ArrayIndex
            || row.result() != MapReadResultClassV1::MapView
        {
            return Err(freeze("index-relation"));
        }
        let MapReadOperandV1::Index(index) = row.operand() else {
            return Err(freeze("index-operand"));
        };
        let key = pending.key.to_string();
        let parent_map = pending.parent_map;
        if !self.consumed.insert(row.site().clone()) {
            return Err(freeze("index-row-consume"));
        }
        self.pending_array = None;
        Ok((parent_map, key, i64::from(*index)))
    }

    pub(in crate::mir::builder) fn text_read(
        &mut self,
        row: &MapReadFactV1,
        receiver: ValueId,
    ) -> Result<(ValueId, String), String> {
        if self.pending_array.is_some()
            || row.operation() != MapReadOperationV1::MapLookup
            || row.result() != MapReadResultClassV1::TextView
        {
            return Err(freeze("text-relation"));
        }
        let MapReadOperandV1::Key(key) = row.operand() else {
            return Err(freeze("text-operand"));
        };
        if !self.consumed.insert(row.site().clone()) {
            return Err(freeze("text-row-consume"));
        }
        Ok((receiver, key.to_string()))
    }

    pub(in crate::mir::builder) fn finish(&self) -> Result<(), String> {
        if self.pending_array.is_some() || self.consumed.len() != self.facts.rows().len() {
            return Err(freeze("rows-unconsumed"));
        }
        Ok(())
    }
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][map-read/physical-consumer/{reason}]")
}

impl RawInvocationChildPortV1<'_, '_> {
    pub(in crate::mir::builder) fn try_lower_map_read_method_call_v1(
        &mut self,
        builder: &mut MirBuilder,
        receiver: &ASTNode,
        method: &str,
        arguments: &[ASTNode],
        receiver_source: PreparedRawChildSourceV1,
    ) -> Result<Option<ValueId>, String> {
        let Some(consumer) = self.map_read_consumer.as_ref().cloned() else {
            return Ok(None);
        };
        let owner = match self.callable_owner_v1() {
            Some(owner) => owner,
            None => return Ok(None),
        };
        let source = match self.current_source_site_v1() {
            Some(source) => source,
            None => return Ok(None),
        };
        let site = OwnedExprSiteV1::new(
            owner,
            crate::mir::resolved_semantics::SourceExprSiteV1::from_node(source),
        );
        let Some(row) = consumer.borrow().begin_get(&site, method, arguments)? else {
            return Ok(None);
        };
        let receiver_value = self.with_prepared_child_source_v1(receiver_source, |port| {
            drive_legacy_expression_v1(builder, port, receiver.clone())
        })?;
        let ledger = self
            .ordinary_new_claim_ledger
            .as_ref()
            .ok_or_else(|| freeze("claim-ledger-missing"))?;
        let state = self
            .callable_ledger
            .as_ref()
            .ok_or_else(|| freeze("callable-state-missing"))?;
        let mut state = state.borrow_mut();
        let value = match row.result() {
            MapReadResultClassV1::ArrayView => {
                consumer.borrow_mut().begin_first(&row, receiver_value)?;
                receiver_value
            }
            MapReadResultClassV1::MapView => {
                let (map, key, index) = consumer.borrow_mut().array_index(&row, receiver_value)?;
                emit_typed_map_read(
                    builder,
                    &mut state,
                    ledger,
                    owner,
                    &site,
                    crate::mir::instruction::MapInvokeOperation::ArrayIndexMap {
                        map,
                        utf8: key,
                        index,
                    },
                    MirType::Box("MapView".to_owned()),
                )?
            }
            MapReadResultClassV1::TextView => {
                let (map, key) = consumer.borrow_mut().text_read(&row, receiver_value)?;
                emit_typed_map_read(
                    builder,
                    &mut state,
                    ledger,
                    owner,
                    &site,
                    crate::mir::instruction::MapInvokeOperation::MapGetText { map, utf8: key },
                    MirType::Box("TextView".to_owned()),
                )?
            }
        };
        Ok(Some(value))
    }
}

fn emit_typed_map_read(
    builder: &mut MirBuilder,
    state: &mut CallableSemanticLoweringState,
    ledger: &OrdinaryNewClaimLedgerV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    site: &OwnedExprSiteV1,
    operation: crate::mir::instruction::MapInvokeOperation,
    result_type: MirType,
) -> Result<ValueId, String> {
    let frame = state.borrow_fault_frame(builder)?;
    let mut bindings =
        vec![super::ordinary_new_admission::selected::fault_frame_binding(builder, state, frame)?];
    let fault = builder.next_block_id();
    super::ordinary_new_admission::selected::append_block(
        builder,
        fault,
        MirInstruction::ReturnFault { fault_frame: frame },
        &mut bindings,
    )?;
    let origin = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("no-block"))?;
    let normal = builder.next_block_id();
    let result = builder.next_value_id();
    let invoke = MirInstruction::Invoke {
        operation: InvokeOperation::Map(operation),
        fault_frame: frame,
        normal_landing: normal,
        fault_landing: fault,
    };
    builder.emit_instruction(invoke.clone())?;
    bindings.push((origin, invoke));
    builder.start_new_block(normal)?;
    let projection = MirInstruction::InvokeNormalResult {
        invoke_block: origin,
        dst: result,
    };
    builder.emit_instruction(projection.clone())?;
    bindings.push((normal, projection));
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(result, result_type);
    ledger.record_map_read_bindings(owner, site.clone(), bindings)?;
    Ok(result)
}
