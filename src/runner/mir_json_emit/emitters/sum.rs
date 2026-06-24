use crate::mir::{boxed_sum_abi_plan, MirType, ValueId};
use serde_json::json;

pub(crate) fn emit_variant_make(
    dst: &ValueId,
    enum_name: &str,
    variant: &str,
    tag: u32,
    payload: Option<&ValueId>,
    payload_type: Option<&MirType>,
    site_plan: Option<&boxed_sum_abi_plan::BoxedSumSitePlan>,
) -> serde_json::Value {
    let mut obj = json!({
        "op": "variant_make",
        "dst": dst.as_u32(),
        "enum": enum_name,
        "variant": variant,
        "tag": tag,
    });
    if let Some(payload) = payload {
        obj["payload"] = json!(payload.as_u32());
    }
    if let Some(payload_type) = payload_type.and_then(type_hint_to_json) {
        obj["payload_type"] = payload_type;
    }
    annotate_boxed_sum_site_plan(&mut obj, site_plan);
    annotate_variant_binding(&mut obj, dst, enum_name, tag, payload, site_plan.is_some());
    obj
}

pub(crate) fn emit_variant_tag(
    dst: &ValueId,
    value: &ValueId,
    enum_name: &str,
    site_plan: Option<&boxed_sum_abi_plan::BoxedSumSitePlan>,
) -> serde_json::Value {
    let mut obj = json!({
        "op": "variant_tag",
        "dst": dst.as_u32(),
        "value": value.as_u32(),
        "enum": enum_name,
    });
    annotate_boxed_sum_site_plan(&mut obj, site_plan);
    obj
}

pub(crate) fn emit_variant_project(
    boxed_sum_abi_plans: &[boxed_sum_abi_plan::BoxedSumAbiPlanV1],
    dst: &ValueId,
    value: &ValueId,
    enum_name: &str,
    variant: &str,
    tag: u32,
    payload_type: Option<&MirType>,
) -> serde_json::Value {
    let mut obj = json!({
        "op": "variant_project",
        "dst": dst.as_u32(),
        "value": value.as_u32(),
        "enum": enum_name,
        "variant": variant,
        "tag": tag,
    });
    if let Some(payload_type) = payload_type.and_then(type_hint_to_json) {
        obj["payload_type"] = payload_type;
    }
    annotate_boxed_sum_site(&mut obj, boxed_sum_abi_plans, enum_name, tag, payload_type);
    obj
}

fn annotate_boxed_sum_site(
    obj: &mut serde_json::Value,
    plans: &[boxed_sum_abi_plan::BoxedSumAbiPlanV1],
    enum_name: &str,
    tag: u32,
    payload_type: Option<&MirType>,
) {
    let Some(plan) =
        boxed_sum_abi_plan::find_boxed_sum_site_plan(plans, enum_name, tag, payload_type)
    else {
        return;
    };
    let Some(variant) = plan.variants.get(tag as usize) else {
        return;
    };
    annotate_boxed_sum_site_plan(
        obj,
        Some(&boxed_sum_abi_plan::BoxedSumSitePlan {
            plan_id: plan.plan_id,
            payload_storage: Some(variant.payload_storage.clone()),
        }),
    );
}

fn annotate_boxed_sum_site_plan(
    obj: &mut serde_json::Value,
    site_plan: Option<&boxed_sum_abi_plan::BoxedSumSitePlan>,
) {
    let Some(site_plan) = site_plan else {
        return;
    };
    obj["boxed_sum_abi_plan_id"] = json!(site_plan.plan_id);
    if let Some(payload_storage) = &site_plan.payload_storage {
        obj["boxed_sum_payload_storage"] = json!(payload_storage.as_str());
    }
}

fn annotate_variant_binding(
    obj: &mut serde_json::Value,
    dst: &ValueId,
    enum_name: &str,
    tag: u32,
    payload: Option<&ValueId>,
    has_boxed_site: bool,
) {
    obj["variant_binding"] = json!({
        "dst": dst.as_u32(),
        "tag_const": tag,
        "tag_reg": 0,
        "payload_reg": payload.map(|value| value.as_u32()).unwrap_or(0),
        "has_payload": payload.is_some(),
        "enum_name": enum_name,
        "copy_alias_payload": payload.is_some() && !has_boxed_site,
        "const_zero_result": payload.is_none() && !has_boxed_site,
        "boxed_sum_abi_plan_id": obj
            .get("boxed_sum_abi_plan_id")
            .and_then(|value| value.as_i64()),
    });
}

fn type_hint_to_json(ty: &MirType) -> Option<serde_json::Value> {
    match ty {
        MirType::Integer => Some(json!("Integer")),
        MirType::Float => Some(json!("Float")),
        MirType::Bool => Some(json!("Bool")),
        MirType::String => Some(json!("String")),
        MirType::Void => Some(json!("Void")),
        MirType::Box(name) => Some(json!(name)),
        MirType::Array(_) | MirType::Future(_) | MirType::WeakRef | MirType::Unknown => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::boxed_sum_abi_plan::{
        BoxedSumAbiPlanV1, BoxedSumAbiVariantPlan, BoxedSumPayloadStorage,
        BOXED_SUM_TAG_STORAGE_I64,
    };

    fn option_i64_plan() -> BoxedSumAbiPlanV1 {
        BoxedSumAbiPlanV1 {
            plan_id: 7,
            shape_key: "Option|0:none,1:i64".to_string(),
            enum_name: "Option".to_string(),
            runtime_type_id: 700_007,
            runtime_box_name: "__NyVariant_Option".to_string(),
            tag_storage: BOXED_SUM_TAG_STORAGE_I64,
            variants: vec![
                BoxedSumAbiVariantPlan {
                    name: "None".to_string(),
                    tag: 0,
                    payload_storage: BoxedSumPayloadStorage::None,
                },
                BoxedSumAbiVariantPlan {
                    name: "Some".to_string(),
                    tag: 1,
                    payload_storage: BoxedSumPayloadStorage::I64,
                },
            ],
        }
    }

    #[test]
    fn variant_make_emits_boxed_sum_site_plan_metadata() {
        let plan = option_i64_plan();
        let json = emit_variant_make(
            &ValueId::new(2),
            "Option",
            "Some",
            1,
            Some(&ValueId::new(1)),
            Some(&MirType::Integer),
            Some(&boxed_sum_abi_plan::BoxedSumSitePlan {
                plan_id: plan.plan_id,
                payload_storage: Some(BoxedSumPayloadStorage::I64),
            }),
        );

        assert_eq!(json["boxed_sum_abi_plan_id"], 7);
        assert_eq!(json["boxed_sum_payload_storage"], "i64");
        assert_eq!(json["variant_binding"]["dst"], 2);
        assert_eq!(json["variant_binding"]["tag_const"], 1);
        assert_eq!(json["variant_binding"]["payload_reg"], 1);
        assert_eq!(json["variant_binding"]["has_payload"], true);
        assert_eq!(json["variant_binding"]["enum_name"], "Option");
        assert_eq!(json["variant_binding"]["copy_alias_payload"], false);
        assert_eq!(json["variant_binding"]["const_zero_result"], false);
        assert_eq!(json["variant_binding"]["boxed_sum_abi_plan_id"], 7);
    }

    #[test]
    fn variant_make_emits_local_unit_binding_fact() {
        let json = emit_variant_make(&ValueId::new(2), "Probe", "None", 0, None, None, None);

        assert_eq!(json["variant_binding"]["dst"], 2);
        assert_eq!(json["variant_binding"]["tag_const"], 0);
        assert_eq!(json["variant_binding"]["payload_reg"], 0);
        assert_eq!(json["variant_binding"]["has_payload"], false);
        assert_eq!(json["variant_binding"]["copy_alias_payload"], false);
        assert_eq!(json["variant_binding"]["const_zero_result"], true);
    }

    #[test]
    fn variant_make_none_keeps_instantiation_hint_but_uses_resolved_site_plan() {
        let plan = option_i64_plan();
        let json = emit_variant_make(
            &ValueId::new(2),
            "Option",
            "None",
            0,
            None,
            Some(&MirType::Integer),
            Some(&boxed_sum_abi_plan::BoxedSumSitePlan {
                plan_id: plan.plan_id,
                payload_storage: Some(BoxedSumPayloadStorage::None),
            }),
        );

        assert_eq!(json["payload_type"], "Integer");
        assert_eq!(json["boxed_sum_abi_plan_id"], 7);
        assert_eq!(json["boxed_sum_payload_storage"], "none");
        assert_eq!(json["variant_binding"]["has_payload"], false);
        assert_eq!(json["variant_binding"]["payload_reg"], 0);
        assert_eq!(json["variant_binding"]["const_zero_result"], false);
        assert_eq!(json["variant_binding"]["boxed_sum_abi_plan_id"], 7);
    }

    #[test]
    fn variant_make_some_zero_payload_is_not_unit_variant() {
        let plan = option_i64_plan();
        let json = emit_variant_make(
            &ValueId::new(2),
            "Option",
            "Some",
            1,
            Some(&ValueId::new(0)),
            Some(&MirType::Integer),
            Some(&boxed_sum_abi_plan::BoxedSumSitePlan {
                plan_id: plan.plan_id,
                payload_storage: Some(BoxedSumPayloadStorage::I64),
            }),
        );

        assert_eq!(json["variant_binding"]["payload_reg"], 0);
        assert_eq!(json["variant_binding"]["has_payload"], true);
        assert_eq!(json["variant_binding"]["copy_alias_payload"], false);
        assert_eq!(json["variant_binding"]["const_zero_result"], false);
    }

    #[test]
    fn variant_project_emits_boxed_sum_site_plan_metadata() {
        let plan = option_i64_plan();
        let json = emit_variant_project(
            &[plan],
            &ValueId::new(3),
            &ValueId::new(2),
            "Option",
            "Some",
            1,
            Some(&MirType::Integer),
        );

        assert_eq!(json["boxed_sum_abi_plan_id"], 7);
        assert_eq!(json["boxed_sum_payload_storage"], "i64");
    }

    #[test]
    fn variant_tag_emits_boxed_sum_site_plan_id_when_proven() {
        let json = emit_variant_tag(
            &ValueId::new(4),
            &ValueId::new(2),
            "Option",
            Some(&boxed_sum_abi_plan::BoxedSumSitePlan {
                plan_id: 7,
                payload_storage: None,
            }),
        );

        assert_eq!(json["boxed_sum_abi_plan_id"], 7);
        assert!(json.get("boxed_sum_payload_storage").is_none());
    }
}
