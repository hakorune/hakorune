use crate::mir::fastmem_layout_contract::{
    resolve_fastmem_block_next_contract, resolve_fastmem_field_contract,
};

use super::types::FastMemFieldAccessMode;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ResolvedHeadAccess {
    pub(super) failure_reason: Option<String>,
    pub(super) layout_id: Option<String>,
    pub(super) field_id: Option<String>,
    pub(super) field_class: Option<String>,
    pub(super) byte_offset: Option<u32>,
    pub(super) field_size: Option<u32>,
    pub(super) field_type: Option<String>,
    pub(super) alignment: Option<u32>,
}

impl ResolvedHeadAccess {
    pub(super) fn is_resolved(&self) -> bool {
        self.failure_reason.is_none()
            && self.byte_offset.is_some()
            && self.field_size.is_some()
            && self.field_type.is_some()
            && self.alignment.is_some()
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(super) struct ResolvedBlockNextAccess {
    pub(super) layout_id: Option<String>,
    pub(super) field_id: Option<String>,
    pub(super) field_class: Option<String>,
    pub(super) byte_offset: Option<u32>,
    pub(super) field_size: Option<u32>,
    pub(super) field_type: Option<String>,
    pub(super) alignment: Option<u32>,
}

impl ResolvedBlockNextAccess {
    pub(super) fn is_resolved(&self) -> bool {
        self.layout_id.is_some()
            && self.field_id.is_some()
            && self.byte_offset.is_some()
            && self.field_size.is_some()
            && self.field_type.is_some()
            && self.alignment.is_some()
    }
}

pub(super) fn resolve_head_access(
    contract: Option<&str>,
    field_id: &str,
    mode: FastMemFieldAccessMode,
) -> ResolvedHeadAccess {
    match contract.map(|contract| {
        resolve_fastmem_field_contract(contract, field_id, mode).map_err(|err| err.reason())
    }) {
        Some(Ok(resolved)) => ResolvedHeadAccess {
            failure_reason: None,
            layout_id: Some(resolved.layout_id),
            field_id: Some(resolved.field_id),
            field_class: Some(resolved.field_class),
            byte_offset: Some(resolved.byte_offset),
            field_size: Some(resolved.field_size),
            field_type: Some(resolved.field_type),
            alignment: Some(resolved.alignment),
        },
        Some(Err(reason)) => ResolvedHeadAccess {
            failure_reason: Some(reason),
            layout_id: None,
            field_id: None,
            field_class: None,
            byte_offset: None,
            field_size: None,
            field_type: None,
            alignment: None,
        },
        None => ResolvedHeadAccess {
            failure_reason: Some("layout-field-contract-unresolved".to_string()),
            layout_id: None,
            field_id: None,
            field_class: None,
            byte_offset: None,
            field_size: None,
            field_type: None,
            alignment: None,
        },
    }
}

pub(super) fn resolve_block_next_access(
    contract: Option<&str>,
    field_id: &str,
) -> ResolvedBlockNextAccess {
    let Some(resolved) =
        contract.and_then(|contract| resolve_fastmem_block_next_contract(contract, field_id).ok())
    else {
        return ResolvedBlockNextAccess::default();
    };

    ResolvedBlockNextAccess {
        layout_id: Some(resolved.layout_id),
        field_id: Some(resolved.field_id),
        field_class: Some(resolved.field_class),
        byte_offset: Some(resolved.byte_offset),
        field_size: Some(resolved.field_size),
        field_type: Some(resolved.field_type),
        alignment: Some(resolved.alignment),
    }
}
