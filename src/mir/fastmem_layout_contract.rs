/*!
 * FastMemory memory-profile layout/table contracts.
 *
 * This is the verifier-side owner for concrete layout/table facts consumed by
 * `FastMemAccessPlan`.  It is intentionally memory-specific; the future
 * ContractRegion envelope stays common, while MemOp layout truth lives here.
 */

use crate::mir::fastmem_access_plan::FastMemFieldAccessMode;
use crate::mir::raw_layout::{build_repr_c_v0_raw_layout, RawLayoutFieldDecl};

pub const PAGE_MAP_CONTRACT_V0: &str = "PageMapV0";
pub const PAGE_META_LAYOUT_V0: &str = "PageMetaLayoutV0";
pub const FREE_BLOCK_NODE_LAYOUT_V0: &str = "FreeBlockNodeLayoutV0";
pub const PAGE_TABLE_V0: &str = "page_table";

const FIELD_OWNER_WORKER_ID: &str = "owner_worker_id";
const FIELD_BLOCK_SIZE: &str = "block_size";
const FIELD_FREE_HEAD: &str = "free_head";
const FIELD_LOCAL_FREE_HEAD: &str = "local_free_head";
const FIELD_REMOTE_HEAD: &str = "remote_head";
const FIELD_CAPACITY: &str = "capacity";
const FIELD_USED: &str = "used";
const FIELD_NEXT: &str = "next";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedFastMemFieldContract {
    pub layout_id: String,
    pub field_id: String,
    pub byte_offset: u32,
    pub field_size: u32,
    pub field_type: String,
    pub alignment: u32,
    pub mutability: String,
    pub field_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedFastMemBlockNextContract {
    pub layout_id: String,
    pub field_id: String,
    pub byte_offset: u32,
    pub field_size: u32,
    pub field_type: String,
    pub alignment: u32,
    pub mutability: String,
    pub field_class: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolvedFastMemTableContract {
    pub table_id: String,
    pub element_layout_id: String,
    pub element_repr: String,
    pub element_stride: u32,
    pub element_size: u32,
    pub length: Option<u64>,
    pub alignment: u32,
    pub index_policy: String,
    pub lowerable: bool,
    pub non_lowerable_reason: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FastMemContractError {
    UnknownContract { contract: String },
    UnknownField { contract: String, field_id: String },
    UnknownTable { contract: String, table_id: String },
    AtomicFieldPlainStore { contract: String, field_id: String },
    LayoutBuild { contract: String, reason: String },
}

impl FastMemContractError {
    pub fn reason(&self) -> String {
        match self {
            Self::UnknownContract { contract } => {
                format!("unknown-fastmem-contract:{contract}")
            }
            Self::UnknownField { field_id, .. } => {
                format!("unknown-field-id:{field_id}")
            }
            Self::UnknownTable { table_id, .. } => {
                format!("unknown-table-id:{table_id}")
            }
            Self::AtomicFieldPlainStore { field_id, .. } => {
                format!("atomic-field-plain-store:{field_id}")
            }
            Self::LayoutBuild { reason, .. } => {
                format!("layout-build-failed:{reason}")
            }
        }
    }
}

pub fn resolve_fastmem_field_contract(
    contract: &str,
    field_id: &str,
    mode: FastMemFieldAccessMode,
) -> Result<ResolvedFastMemFieldContract, FastMemContractError> {
    ensure_page_map_contract(contract)?;

    let canonical =
        canonical_page_meta_field(field_id).ok_or_else(|| FastMemContractError::UnknownField {
            contract: contract.to_string(),
            field_id: field_id.to_string(),
        })?;
    let spec = page_meta_field_spec(canonical).expect("canonical field has spec");
    if mode == FastMemFieldAccessMode::Store && spec.field_class == "atomic_remote_head" {
        return Err(FastMemContractError::AtomicFieldPlainStore {
            contract: contract.to_string(),
            field_id: canonical.to_string(),
        });
    }

    let raw = page_meta_raw_layout(contract)?;
    let raw_field = raw
        .fields
        .iter()
        .find(|field| field.name == canonical)
        .expect("canonical field exists in raw layout");

    Ok(ResolvedFastMemFieldContract {
        layout_id: PAGE_META_LAYOUT_V0.to_string(),
        field_id: canonical.to_string(),
        byte_offset: raw_field.offset_bytes,
        field_size: raw_field.size_bytes,
        field_type: raw_field.declared_type_name.clone(),
        alignment: raw_field.align_bytes,
        mutability: spec.mutability.to_string(),
        field_class: spec.field_class.to_string(),
    })
}

pub fn resolve_fastmem_table_contract(
    contract: &str,
    table_id: &str,
) -> Result<ResolvedFastMemTableContract, FastMemContractError> {
    ensure_page_map_contract(contract)?;
    if table_id != PAGE_TABLE_V0 {
        return Err(FastMemContractError::UnknownTable {
            contract: contract.to_string(),
            table_id: table_id.to_string(),
        });
    }
    let pointer_bytes = usize::BITS / 8;
    let element_size = page_meta_raw_layout(contract)?.size_bytes;
    Ok(ResolvedFastMemTableContract {
        table_id: PAGE_TABLE_V0.to_string(),
        element_layout_id: PAGE_META_LAYOUT_V0.to_string(),
        element_repr: "pointer_to_element".to_string(),
        element_stride: pointer_bytes,
        element_size,
        length: None,
        alignment: pointer_bytes,
        index_policy: "explicit_check".to_string(),
        lowerable: false,
        non_lowerable_reason: Some("table-length-unresolved".to_string()),
    })
}

pub fn resolve_fastmem_block_next_contract(
    contract: &str,
    field_id: &str,
) -> Result<ResolvedFastMemBlockNextContract, FastMemContractError> {
    ensure_page_map_contract(contract)?;
    if field_id != FIELD_NEXT {
        return Err(FastMemContractError::UnknownField {
            contract: contract.to_string(),
            field_id: field_id.to_string(),
        });
    }
    let pointer_bytes = usize::BITS / 8;
    Ok(ResolvedFastMemBlockNextContract {
        layout_id: FREE_BLOCK_NODE_LAYOUT_V0.to_string(),
        field_id: FIELD_NEXT.to_string(),
        byte_offset: 0,
        field_size: pointer_bytes,
        field_type: "usize".to_string(),
        alignment: pointer_bytes,
        mutability: "mutable".to_string(),
        field_class: "local_free_block_next".to_string(),
    })
}

fn ensure_page_map_contract(contract: &str) -> Result<(), FastMemContractError> {
    if contract == PAGE_MAP_CONTRACT_V0 {
        Ok(())
    } else {
        Err(FastMemContractError::UnknownContract {
            contract: contract.to_string(),
        })
    }
}

fn canonical_page_meta_field(field_id: &str) -> Option<&'static str> {
    match field_id {
        "owner_id" | FIELD_OWNER_WORKER_ID => Some(FIELD_OWNER_WORKER_ID),
        FIELD_BLOCK_SIZE => Some(FIELD_BLOCK_SIZE),
        FIELD_FREE_HEAD => Some(FIELD_FREE_HEAD),
        FIELD_LOCAL_FREE_HEAD => Some(FIELD_LOCAL_FREE_HEAD),
        FIELD_REMOTE_HEAD => Some(FIELD_REMOTE_HEAD),
        FIELD_CAPACITY => Some(FIELD_CAPACITY),
        FIELD_USED => Some(FIELD_USED),
        _ => None,
    }
}

struct PageMetaFieldSpec {
    name: &'static str,
    type_name: &'static str,
    mutability: &'static str,
    field_class: &'static str,
}

fn page_meta_field_specs() -> &'static [PageMetaFieldSpec] {
    &[
        PageMetaFieldSpec {
            name: FIELD_OWNER_WORKER_ID,
            type_name: "u64",
            mutability: "page_claim_only",
            field_class: "plain_scalar",
        },
        PageMetaFieldSpec {
            name: FIELD_BLOCK_SIZE,
            type_name: "usize",
            mutability: "immutable_after_claim",
            field_class: "plain_scalar",
        },
        PageMetaFieldSpec {
            name: FIELD_FREE_HEAD,
            type_name: "usize",
            mutability: "mutable",
            field_class: "plain_pointer",
        },
        PageMetaFieldSpec {
            name: FIELD_LOCAL_FREE_HEAD,
            type_name: "usize",
            mutability: "mutable",
            field_class: "local_free_head",
        },
        PageMetaFieldSpec {
            name: FIELD_REMOTE_HEAD,
            type_name: "usize",
            mutability: "atomic_only",
            field_class: "atomic_remote_head",
        },
        PageMetaFieldSpec {
            name: FIELD_CAPACITY,
            type_name: "usize",
            mutability: "immutable_after_claim",
            field_class: "plain_scalar",
        },
        PageMetaFieldSpec {
            name: FIELD_USED,
            type_name: "usize",
            mutability: "mutable",
            field_class: "plain_scalar",
        },
    ]
}

fn page_meta_field_spec(field_id: &str) -> Option<&'static PageMetaFieldSpec> {
    page_meta_field_specs()
        .iter()
        .find(|spec| spec.name == field_id)
}

fn page_meta_raw_layout(
    contract: &str,
) -> Result<crate::mir::raw_layout::RawLayoutPlan, FastMemContractError> {
    let decls: Vec<RawLayoutFieldDecl<'_>> = page_meta_field_specs()
        .iter()
        .map(|spec| RawLayoutFieldDecl {
            name: spec.name,
            type_name: spec.type_name,
        })
        .collect();
    build_repr_c_v0_raw_layout(PAGE_META_LAYOUT_V0, &decls).map_err(|err| {
        FastMemContractError::LayoutBuild {
            contract: contract.to_string(),
            reason: err.to_string(),
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resolves_owner_alias_to_canonical_page_meta_field() {
        let field = resolve_fastmem_field_contract(
            PAGE_MAP_CONTRACT_V0,
            "owner_id",
            FastMemFieldAccessMode::Load,
        )
        .expect("owner alias resolves");

        assert_eq!(field.layout_id, PAGE_META_LAYOUT_V0);
        assert_eq!(field.field_id, "owner_worker_id");
        assert_eq!(field.field_type, "u64");
        assert_eq!(field.field_size, 8);
        assert_eq!(field.field_class, "plain_scalar");
        assert_eq!(field.byte_offset, 0);
    }

    #[test]
    fn rejects_plain_store_to_atomic_remote_head() {
        let err = resolve_fastmem_field_contract(
            PAGE_MAP_CONTRACT_V0,
            "remote_head",
            FastMemFieldAccessMode::Store,
        )
        .expect_err("remote_head plain store is forbidden");

        assert_eq!(err.reason(), "atomic-field-plain-store:remote_head");
    }

    #[test]
    fn resolves_page_table_contract_as_non_lowerable_shell() {
        let table =
            resolve_fastmem_table_contract(PAGE_MAP_CONTRACT_V0, PAGE_TABLE_V0).expect("table");

        assert_eq!(table.element_layout_id, PAGE_META_LAYOUT_V0);
        assert_eq!(table.element_repr, "pointer_to_element");
        assert_eq!(table.element_stride, usize::BITS / 8);
        assert!(table.element_size >= 56);
        assert_eq!(table.length, None);
        assert!(!table.lowerable);
        assert_eq!(
            table.non_lowerable_reason.as_deref(),
            Some("table-length-unresolved")
        );
    }
}
