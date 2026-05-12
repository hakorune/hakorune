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
