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
                "field_decls": field_decls.into_iter().map(|decl| json!({
                    "name": decl.name,
                    "declared_type": decl.declared_type_name,
                    "is_weak": decl.is_weak,
                })).collect::<Vec<_>>(),
            })
        })
        .collect()
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
