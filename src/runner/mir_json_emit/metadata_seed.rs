use crate::mir::concat_const_suffix_micro_seed_plan::ConcatConstSuffixMicroSeedRoute;
use crate::mir::exact_seed_backend_route::ExactSeedBackendRoute;
use crate::mir::substring_views_micro_seed_plan::SubstringViewsMicroSeedRoute;
use crate::mir::sum_variant_project_seed_plan::{
    SumVariantProjectSeedPayload, SumVariantProjectSeedRoute,
};
use crate::mir::sum_variant_tag_seed_plan::SumVariantTagSeedRoute;
use crate::mir::userbox_local_scalar_seed_plan::{
    UserBoxLocalScalarSeedKind, UserBoxLocalScalarSeedPayload, UserBoxLocalScalarSeedRoute,
    UserBoxLocalScalarSeedSinglePayload,
};
use serde_json::json;

pub(super) fn build_concat_const_suffix_micro_seed_route_json(
    route: &ConcatConstSuffixMicroSeedRoute,
) -> serde_json::Value {
    json!({
        "seed": route.seed(),
        "seed_len": route.seed_len(),
        "suffix": route.suffix(),
        "suffix_len": route.suffix_len(),
        "ops": route.ops(),
        "result_len": route.result_len(),
        "proof": route.proof(),
        "consumer_capability": "direct_concat_const_suffix_loop",
        "publication_boundary": "none",
    })
}

pub(super) fn build_substring_views_micro_seed_route_json(
    route: &SubstringViewsMicroSeedRoute,
) -> serde_json::Value {
    json!({
        "source": route.source(),
        "source_len": route.source_len(),
        "loop_bound": route.loop_bound(),
        "proof": route.proof(),
        "consumer_capability": "direct_substring_views_exit_len",
        "publication_boundary": "none",
    })
}

pub(super) fn build_sum_variant_tag_seed_route_json(
    route: &SumVariantTagSeedRoute,
) -> serde_json::Value {
    json!({
        "kind": route.kind().to_string(),
        "enum": route.enum_name(),
        "variant": route.variant(),
        "subject": route.subject(),
        "layout": route.layout().to_string(),
        "variant_tag": route.variant_tag(),
        "make_block": route.make_block().as_u32(),
        "make_instruction_index": route.make_instruction_index(),
        "tag_block": route.tag_block().as_u32(),
        "tag_instruction_index": route.tag_instruction_index(),
        "sum_value": route.sum_value().as_u32(),
        "tag_value": route.tag_value().as_u32(),
        "tag_source_value": route.tag_source_value().as_u32(),
        "copy_value": route.copy_value().map(|value| value.as_u32()),
        "payload_value": route.payload_value().map(|value| value.as_u32()),
        "proof": route.proof(),
        "consumer_capability": "direct_sum_variant_tag_local",
        "publication_boundary": "none",
    })
}

pub(super) fn build_sum_variant_project_seed_route_json(
    route: &SumVariantProjectSeedRoute,
) -> serde_json::Value {
    let (payload_i64, payload_f64, payload_string) = match route.payload() {
        SumVariantProjectSeedPayload::I64(value) => (Some(*value), None, None),
        SumVariantProjectSeedPayload::F64(value) => (None, Some(*value), None),
        SumVariantProjectSeedPayload::String(value) => (None, None, Some(value.as_str())),
    };
    let mut obj = serde_json::Map::new();
    obj.insert("kind".to_string(), json!(route.kind().to_string()));
    obj.insert("enum".to_string(), json!(route.enum_name()));
    obj.insert("variant".to_string(), json!(route.variant()));
    obj.insert("subject".to_string(), json!(route.subject()));
    obj.insert("layout".to_string(), json!(route.layout().to_string()));
    obj.insert("variant_tag".to_string(), json!(route.variant_tag()));
    obj.insert("make_block".to_string(), json!(route.make_block().as_u32()));
    obj.insert(
        "make_instruction_index".to_string(),
        json!(route.make_instruction_index()),
    );
    obj.insert(
        "project_block".to_string(),
        json!(route.project_block().as_u32()),
    );
    obj.insert(
        "project_instruction_index".to_string(),
        json!(route.project_instruction_index()),
    );
    obj.insert("sum_value".to_string(), json!(route.sum_value().as_u32()));
    obj.insert(
        "project_value".to_string(),
        json!(route.project_value().as_u32()),
    );
    obj.insert(
        "project_source_value".to_string(),
        json!(route.project_source_value().as_u32()),
    );
    obj.insert(
        "copy_value".to_string(),
        json!(route.copy_value().map(|value| value.as_u32())),
    );
    obj.insert(
        "payload_value".to_string(),
        json!(route.payload_value().as_u32()),
    );
    obj.insert(
        "payload_literal_kind".to_string(),
        json!(route.payload().kind()),
    );
    obj.insert("payload_i64".to_string(), json!(payload_i64));
    obj.insert("payload_f64".to_string(), json!(payload_f64));
    obj.insert("payload_string".to_string(), json!(payload_string));
    obj.insert("proof".to_string(), json!(route.proof()));
    obj.insert(
        "consumer_capability".to_string(),
        json!("direct_sum_variant_project_local"),
    );
    obj.insert("publication_boundary".to_string(), json!("none"));
    serde_json::Value::Object(obj)
}

pub(super) fn build_userbox_local_scalar_seed_route_json(
    route: &UserBoxLocalScalarSeedRoute,
) -> serde_json::Value {
    let mut obj = serde_json::Map::new();
    obj.insert("kind".to_string(), json!(route.kind().to_string()));
    obj.insert("box".to_string(), json!(route.box_name()));
    obj.insert("block".to_string(), json!(route.block().as_u32()));
    obj.insert(
        "newbox_instruction_index".to_string(),
        json!(route.newbox_instruction_index()),
    );
    obj.insert("box_value".to_string(), json!(route.box_value().as_u32()));
    obj.insert(
        "copy_value".to_string(),
        json!(route.copy_value().map(|value| value.as_u32())),
    );
    obj.insert(
        "result_value".to_string(),
        json!(route.result_value().as_u32()),
    );
    obj.insert("proof".to_string(), json!(route.proof()));
    let consumer_capability = match route.kind() {
        UserBoxLocalScalarSeedKind::PointLocalI64
        | UserBoxLocalScalarSeedKind::PointCopyLocalI64 => "direct_userbox_point_local_scalar",
        UserBoxLocalScalarSeedKind::FlagLocalBool
        | UserBoxLocalScalarSeedKind::FlagCopyLocalBool => "direct_userbox_flag_local_scalar",
        UserBoxLocalScalarSeedKind::PointFLocalF64
        | UserBoxLocalScalarSeedKind::PointFCopyLocalF64 => "direct_userbox_pointf_local_scalar",
    };
    obj.insert(
        "consumer_capability".to_string(),
        json!(consumer_capability),
    );
    obj.insert("publication_boundary".to_string(), json!("none"));
    match route.payload() {
        UserBoxLocalScalarSeedPayload::PointI64Pair {
            x_field,
            y_field,
            set_x_instruction_index,
            set_y_instruction_index,
            get_x_instruction_index,
            get_y_instruction_index,
            x_value,
            y_value,
            get_x_value,
            get_y_value,
            x_i64,
            y_i64,
        } => {
            obj.insert("x_field".to_string(), json!(x_field.as_str()));
            obj.insert("y_field".to_string(), json!(y_field.as_str()));
            obj.insert(
                "set_x_instruction_index".to_string(),
                json!(*set_x_instruction_index),
            );
            obj.insert(
                "set_y_instruction_index".to_string(),
                json!(*set_y_instruction_index),
            );
            obj.insert(
                "get_x_instruction_index".to_string(),
                json!(*get_x_instruction_index),
            );
            obj.insert(
                "get_y_instruction_index".to_string(),
                json!(*get_y_instruction_index),
            );
            obj.insert("point_value".to_string(), json!(route.box_value().as_u32()));
            obj.insert("x_value".to_string(), json!(x_value.as_u32()));
            obj.insert("y_value".to_string(), json!(y_value.as_u32()));
            obj.insert("get_x_value".to_string(), json!(get_x_value.as_u32()));
            obj.insert("get_y_value".to_string(), json!(get_y_value.as_u32()));
            obj.insert("x_i64".to_string(), json!(*x_i64));
            obj.insert("y_i64".to_string(), json!(*y_i64));
        }
        UserBoxLocalScalarSeedPayload::SingleField {
            field,
            set_instruction_index,
            get_instruction_index,
            field_value,
            get_field_value,
            payload,
        } => {
            let (payload_i64, payload_f64) = match payload {
                UserBoxLocalScalarSeedSinglePayload::I64(value) => (Some(*value), None),
                UserBoxLocalScalarSeedSinglePayload::F64Bits(bits) => {
                    (None, Some(f64::from_bits(*bits)))
                }
            };
            obj.insert("field".to_string(), json!(field.as_str()));
            obj.insert(
                "set_field_instruction_index".to_string(),
                json!(*set_instruction_index),
            );
            obj.insert(
                "get_field_instruction_index".to_string(),
                json!(*get_instruction_index),
            );
            obj.insert("field_value".to_string(), json!(field_value.as_u32()));
            obj.insert(
                "get_field_value".to_string(),
                json!(get_field_value.as_u32()),
            );
            obj.insert("payload_i64".to_string(), json!(payload_i64));
            obj.insert("payload_f64".to_string(), json!(payload_f64));
        }
    }
    serde_json::Value::Object(obj)
}

pub(super) fn build_exact_seed_backend_route_json(
    route: &ExactSeedBackendRoute,
) -> serde_json::Value {
    json!({
        "tag": route.tag(),
        "source_route": route.source_route(),
        "proof": route.proof(),
        "selected_value": route.selected_value().map(|value| value.as_u32()),
    })
}
