use crate::mir::function::ArrayStateTermKind;
use crate::mir::MirFunction;
use serde_json::json;

pub(super) fn insert_array_write_metadata_json(
    object: &mut serde_json::Map<String, serde_json::Value>,
    function: &MirFunction,
) {
    object.insert(
        "array_element_write_witnesses".to_string(),
        json!(function
            .metadata
            .array_element_write_witnesses
            .iter()
            .map(|witness| json!({
                "site_id": witness.site_id.0,
                "kind": witness.kind.as_str(),
                "producer": witness.producer.as_str(),
                "receiver": witness.receiver.as_u32(),
                "index": witness.index.map(|value| value.as_u32()),
                "value": witness.value.as_u32(),
                "state_term": witness.state_term.0,
            }))
            .collect::<Vec<_>>()),
    );
    object.insert(
        "array_state_terms".to_string(),
        json!(function
            .metadata
            .array_state_terms
            .iter()
            .map(|term| {
                let relation = match &term.kind {
                    ArrayStateTermKind::Fresh { allocation_site } => json!({
                        "kind": "fresh", "allocation_site": allocation_site.as_u32(),
                    }),
                    ArrayStateTermKind::SameAs { source } => json!({
                        "kind": "same_as", "source": source.as_u32(),
                    }),
                    ArrayStateTermKind::Select { inputs } => json!({
                        "kind": "select",
                        "inputs": inputs.iter().map(|value| value.as_u32()).collect::<Vec<_>>(),
                    }),
                    ArrayStateTermKind::DynamicBoundary { value } => json!({
                        "kind": "dynamic_boundary", "value": value.as_u32(),
                    }),
                };
                json!({
                    "term_id": term.term_id.0,
                    "value": term.value.as_u32(),
                    "relation": relation,
                })
            })
            .collect::<Vec<_>>()),
    );
}
