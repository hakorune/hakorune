use super::*;
use crate::ast::FieldDecl;
use crate::mir::instruction::MemOpKind;
use crate::mir::{MirInstruction, MirType, ValueId};
use std::collections::BTreeSet;

const DECLARED_OWNER: &str = "FastMemFieldLoadDeclaredOwnerV1";
const MISSING_OWNER: &str = "FastMemFieldLoadMissingOwnerV1";

fn builder_with_field_load_region(
    name: &str,
    owner: &str,
    declared_type: Option<&str>,
) -> (MirBuilder, ValueId) {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder.comp_ctx.register_user_box_with_field_decls(
        owner.to_string(),
        vec![FieldDecl {
            name: "items".to_string(),
            declared_type_name: declared_type.map(str::to_string),
            is_weak: false,
            default_value: None,
        }],
    );
    if declared_type.is_some() {
        builder.comp_ctx.set_field_origin_by_box(
            owner.to_string(),
            "items".to_string(),
            "ArrayBox".to_string(),
        );
    }
    let base = builder.next_value_id();
    builder
        .function_state
        .type_ctx
        .set_origin_box(base, owner.to_string());
    let region = builder
        .register_fastmem_region("FieldLoadV1".to_string(), span(), 0)
        .unwrap();
    builder.push_fastmem_region(region);
    (builder, base)
}

fn fastmem_field_site_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .metadata
        .fastmem_field_access_sites
        .len()
}

fn field_load_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter(|instruction| {
            matches!(
                instruction,
                MirInstruction::MemOp {
                    kind: MemOpKind::FieldLoad,
                    ..
                }
            )
        })
        .count()
}

fn region_receipt_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .metadata
        .fastmem_regions[0]
        .emitted_memop_count
}

fn types_added_after(builder: &MirBuilder, before: &BTreeSet<ValueId>) -> Vec<MirType> {
    builder
        .function_state
        .type_ctx
        .value_types
        .iter()
        .filter(|(value, _)| !before.contains(value))
        .map(|(_, ty)| ty.clone())
        .collect()
}

#[test]
fn declared_fieldload_failure_keeps_pre_emission_site_and_type_reservation() {
    let (mut builder, base) = builder_with_field_load_region(
        "fastmem_fieldload_declared_failure/0",
        DECLARED_OWNER,
        Some("ArrayBox"),
    );
    let before = builder
        .function_state
        .type_ctx
        .value_types
        .keys()
        .copied()
        .collect::<BTreeSet<_>>();
    let origins_before = builder.function_state.type_ctx.value_origin_newbox.clone();
    builder.function_state.current_block = None;

    let error = builder
        .build_field_access_from_value(base, "items".to_string())
        .unwrap_err();

    assert_eq!(error, "No current basic block");
    assert_eq!(fastmem_field_site_count(&builder), 1);
    assert_eq!(
        types_added_after(&builder, &before),
        vec![MirType::Box("ArrayBox".to_string())]
    );
    assert_eq!(
        builder.function_state.type_ctx.value_origin_newbox,
        origins_before
    );
    assert_eq!(field_load_count(&builder), 0);
    assert_eq!(region_receipt_count(&builder), 0);
}

#[test]
fn missing_fieldload_failure_keeps_site_without_type_or_origin_completion() {
    let (mut builder, base) =
        builder_with_field_load_region("fastmem_fieldload_missing_failure/0", MISSING_OWNER, None);
    let before = builder
        .function_state
        .type_ctx
        .value_types
        .keys()
        .copied()
        .collect::<BTreeSet<_>>();
    let origins_before = builder.function_state.type_ctx.value_origin_newbox.clone();
    builder.function_state.current_block = None;

    let error = builder
        .build_field_access_from_value(base, "items".to_string())
        .unwrap_err();

    assert_eq!(error, "No current basic block");
    assert_eq!(fastmem_field_site_count(&builder), 1);
    assert!(types_added_after(&builder, &before).is_empty());
    assert_eq!(
        builder.function_state.type_ctx.value_origin_newbox,
        origins_before
    );
    assert_eq!(field_load_count(&builder), 0);
    assert_eq!(region_receipt_count(&builder), 0);
}

#[test]
fn declared_fieldload_success_keeps_reservation_and_completes_origin() {
    let (mut builder, base) = builder_with_field_load_region(
        "fastmem_fieldload_declared_success/0",
        DECLARED_OWNER,
        Some("ArrayBox"),
    );

    let destination = builder
        .build_field_access_from_value(base, "items".to_string())
        .unwrap();

    assert_eq!(fastmem_field_site_count(&builder), 1);
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_types
            .get(&destination),
        Some(&MirType::Box("ArrayBox".to_string()))
    );
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&destination)
            .map(String::as_str),
        Some("ArrayBox")
    );
    assert_eq!(field_load_count(&builder), 1);
    assert_eq!(region_receipt_count(&builder), 1);
}

#[test]
fn missing_fieldload_success_completes_integer_compatibility() {
    let (mut builder, base) =
        builder_with_field_load_region("fastmem_fieldload_missing_success/0", MISSING_OWNER, None);

    let destination = builder
        .build_field_access_from_value(base, "items".to_string())
        .unwrap();

    assert_eq!(fastmem_field_site_count(&builder), 1);
    assert_eq!(
        builder
            .function_state
            .type_ctx
            .value_types
            .get(&destination),
        Some(&MirType::Integer)
    );
    assert!(!builder
        .function_state
        .type_ctx
        .value_origin_newbox
        .contains_key(&destination));
    assert_eq!(field_load_count(&builder), 1);
    assert_eq!(region_receipt_count(&builder), 1);
}
