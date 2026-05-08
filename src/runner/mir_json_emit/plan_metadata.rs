use crate::mir::function::FunctionMetadata;
use serde_json::json;

pub(super) fn insert_plan_metadata_json(
    obj: &mut serde_json::Map<String, serde_json::Value>,
    metadata: &FunctionMetadata,
) {
    obj.insert(
        "inline_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .inline_plans
                .iter()
                .map(|plan| {
                    json!({
                        "function": plan.function.as_str(),
                        "request": plan.request.as_str(),
                        "hotness": plan.hotness.as_ref().map(|hotness| hotness.as_str()),
                        "max_ir": plan.max_ir,
                        "requires": &plan.requires,
                        "verified": plan.verified,
                        "fallback": plan.fallback.as_str(),
                        "source": plan.source.as_str(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "effect_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .effect_plans
                .iter()
                .map(|plan| {
                    json!({
                        "function": plan.function.as_str(),
                        "requires": plan
                            .requires
                            .iter()
                            .map(|requirement| requirement.as_str())
                            .collect::<Vec<_>>(),
                        "verified": plan.verified,
                        "source": plan.source.as_str(),
                    })
                })
                .collect(),
        ),
    );
    obj.insert(
        "capability_plans".to_string(),
        serde_json::Value::Array(
            metadata
                .capability_plans
                .iter()
                .map(|plan| {
                    json!({
                        "function": plan.function.as_str(),
                        "allow": &plan.allow,
                        "verified": plan.verified,
                        "source": plan.source.as_str(),
                    })
                })
                .collect(),
        ),
    );
}
