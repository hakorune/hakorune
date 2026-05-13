use super::*;

pub(super) fn collect_sorted_user_box_decl_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    let mut names = std::collections::BTreeSet::new();
    names.extend(module.metadata.user_box_decls.keys().cloned());
    names.extend(module.metadata.user_box_field_decls.keys().cloned());

    names
        .into_iter()
        .map(|name| {
            let field_decls = module
                .metadata
                .user_box_field_decls
                .get(&name)
                .cloned()
                .unwrap_or_default();
            let fields = module
                .metadata
                .user_box_decls
                .get(&name)
                .cloned()
                .unwrap_or_else(|| field_decls.iter().map(|decl| decl.name.clone()).collect());

            json!({
                "name": name,
                "fields": fields,
                "field_decls": field_decls.into_iter().map(|decl| {
                    let field_name = decl.name;
                    let mut obj = serde_json::Map::new();
                    obj.insert("name".to_string(), json!(field_name.as_str()));
                    obj.insert("declared_type".to_string(), json!(decl.declared_type_name));
                    obj.insert("is_weak".to_string(), json!(decl.is_weak));

                    if let Some((layout_id, field_index, storage)) =
                        field_index_fast_path(module, &name, &field_name)
                    {
                        obj.insert("field_index_fast_path".to_string(), json!(true));
                        obj.insert("layout_id".to_string(), json!(layout_id));
                        obj.insert("field_index".to_string(), json!(field_index));
                        obj.insert("storage".to_string(), json!(storage));
                    } else {
                        obj.insert("field_index_fast_path".to_string(), json!(false));
                    }

                    serde_json::Value::Object(obj)
                }).collect::<Vec<_>>(),
            })
        })
        .collect()
}

pub(super) fn collect_sorted_record_decl_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .record_decls
        .iter()
        .map(|(_, decl)| {
            json!({
                "name": decl.name,
                "type_parameters": decl.type_parameters,
                "fields": decl.fields.iter().map(|field| field.name.clone()).collect::<Vec<_>>(),
                "field_decls": decl.fields.iter().enumerate().map(|(index, field)| json!({
                    "name": field.name,
                    "declared_type": field.declared_type_name,
                    "is_weak": field.is_weak,
                    "field_index": index,
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
}

fn field_index_fast_path(
    module: &crate::mir::MirModule,
    box_name: &str,
    field_name: &str,
) -> Option<(u32, u32, &'static str)> {
    let plan = module
        .metadata
        .typed_object_plans
        .iter()
        .find(|plan| plan.box_name == box_name)?;
    let field = plan.fields.iter().find(|field| field.name == field_name)?;
    if field.is_weak {
        return None;
    }
    Some((plan.type_id, field.slot, field.storage.as_str()))
}

pub(super) fn collect_sorted_enum_decl_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .enum_decls
        .iter()
        .map(|(name, decl)| {
            json!({
                "name": name,
                "type_parameters": decl.type_parameters,
                "variants": decl.variants.iter().map(|variant| json!({
                    "name": variant.name,
                    "payload_type": variant.payload_type_name,
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
}

pub(super) fn collect_typed_object_plan_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .typed_object_plans
        .iter()
        .map(|plan| {
            json!({
                "box_name": plan.box_name,
                "type_id": plan.type_id,
                "layout_kind": plan.layout_kind,
                "field_count": plan.field_count,
                "fields": plan.fields.iter().map(|field| json!({
                    "name": field.name,
                    "slot": field.slot,
                    "declared_type": field.declared_type_name,
                    "storage": field.storage.as_str(),
                    "weak": field.is_weak,
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
}

pub(super) fn collect_record_layout_plan_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .record_layout_plans
        .iter()
        .map(|plan| {
            json!({
                "record_name": plan.record_name,
                "layout_id": plan.layout_id,
                "layout_kind": plan.layout_kind,
                "field_count": plan.field_count,
                "fields": plan.fields.iter().map(|field| json!({
                    "name": field.name,
                    "slot": field.slot,
                    "declared_type": field.declared_type_name,
                    "storage": field.storage.as_str(),
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
}

pub(super) fn collect_array_record_storage_plan_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .array_record_storage_plans
        .iter()
        .map(|plan| {
            json!({
                "record_name": plan.record_name,
                "layout_id": plan.layout_id,
                "storage_kind": plan.storage_kind,
                "field_count": plan.field_count,
                "columns": plan.columns.iter().map(|column| json!({
                    "name": column.name,
                    "column": column.column,
                    "storage": column.storage.as_str(),
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
}

pub(super) fn collect_array_record_autouse_eligibility_plan_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .array_record_autouse_eligibility_plans
        .iter()
        .map(|plan| {
            json!({
                "record_name": plan.record_name,
                "layout_id": plan.layout_id,
                "storage_kind": plan.storage_kind,
                "decision": plan.decision,
                "reason": plan.reason,
                "field_count": plan.field_count,
                "integer_lane_columns": plan.integer_lane_columns,
                "required_backend_capability": plan.required_backend_capability,
                "production_auto_use_enabled": plan.production_auto_use_enabled,
            })
        })
        .collect()
}

pub(super) fn collect_array_record_materialization_boundary_plan_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .array_record_materialization_boundary_plans
        .iter()
        .map(|plan| {
            json!({
                "record_name": plan.record_name,
                "layout_id": plan.layout_id,
                "boundary_kind": plan.boundary_kind,
                "source_decision": plan.source_decision,
                "direct_indexed_field_reads_allowed": plan.direct_indexed_field_reads_allowed,
                "visible_record_materialization_enabled": plan.visible_record_materialization_enabled,
                "public_array_get_action": plan.public_array_get_action,
                "returned_element_action": plan.returned_element_action,
                "host_backend_escape_action": plan.host_backend_escape_action,
                "diagnostic": plan.diagnostic,
                "runtime_auto_use_enabled": plan.runtime_auto_use_enabled,
            })
        })
        .collect()
}

pub(super) fn collect_array_record_packed_autouse_pilot_plan_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .array_record_packed_autouse_pilot_plans
        .iter()
        .map(|plan| {
            json!({
                "record_name": plan.record_name,
                "layout_id": plan.layout_id,
                "pilot_kind": plan.pilot_kind,
                "source_boundary_kind": plan.source_boundary_kind,
                "integer_lane_columns": plan.integer_lane_columns,
                "direct_indexed_field_reads_enabled": plan.direct_indexed_field_reads_enabled,
                "private_runtime_storage_enabled": plan.private_runtime_storage_enabled,
                "public_array_get_materialization_enabled": plan.public_array_get_materialization_enabled,
                "hako_alloc_migration_enabled": plan.hako_alloc_migration_enabled,
                "backend_lowering_enabled": plan.backend_lowering_enabled,
            })
        })
        .collect()
}

pub(super) fn collect_static_data_plan_values(
    module: &crate::mir::MirModule,
) -> Vec<serde_json::Value> {
    module
        .metadata
        .static_data_plans
        .iter()
        .map(|plan| {
            json!({
                "source_name": plan.source_name,
                "symbol": plan.symbol,
                "element": plan.element,
                "align": plan.align,
                "linkage": plan.linkage,
                "unnamed_addr": plan.unnamed_addr,
                "values": plan.values,
            })
        })
        .collect()
}
