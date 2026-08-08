use super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1, LoopValueKeyV1};
use super::producer_id::LoopRecipeProducerIdV1;
use super::schema::{
    LoopNodeSourceBindingV1, LoopRecipeProvenanceV1, LoopRecipeSourceBindingV1,
    LoopRecipeSourceOwnerV1, LoopSourcePathStepV1, LoopSourcePathV1,
};
use super::schema_v2::{
    LoopConditionV2, LoopOperationV2, LoopRecipeArtifactV2, LoopRecipeBlockV2, LoopRecipeItemRowV2,
    LoopRecipeItemV2, LoopRecipeV2, LoopRecipeValueV2, LoopValueClassV2,
};
use super::typed_schema_v2::{LoopRecipeV2RejectReason, LoopRecipeVerifierV2};

fn minimal_typed_recipe() -> LoopRecipeArtifactV2 {
    let subject = LoopValueKeyV1::new(0);
    let needle = LoopValueKeyV1::new(1);
    let call_value = LoopValueKeyV1::new(2);
    let equal = LoopValueKeyV1::new(3);
    let recipe = LoopRecipeV2 {
        root_loop: LoopNodeKeyV1::new(0),
        loops: vec![super::schema_v2::LoopNodeV2 {
            key: LoopNodeKeyV1::new(0),
            parent: None,
            condition: LoopConditionV2::Always,
            body: LoopBlockKeyV1::new(0),
        }],
        blocks: vec![LoopRecipeBlockV2 {
            key: LoopBlockKeyV1::new(0),
            owner_loop: LoopNodeKeyV1::new(0),
            items: vec![LoopItemKeyV1::new(0), LoopItemKeyV1::new(1)],
        }],
        items: vec![
            LoopRecipeItemRowV2 {
                key: LoopItemKeyV1::new(0),
                item: LoopRecipeItemV2::Operation {
                    operation: LoopOperationV2::CallSlot {
                        receiver: Some(subject),
                        args: vec![],
                        result: Some(call_value),
                    },
                },
            },
            LoopRecipeItemRowV2 {
                key: LoopItemKeyV1::new(1),
                item: LoopRecipeItemV2::Operation {
                    operation: LoopOperationV2::TextEq {
                        left: call_value,
                        right: needle,
                        result: equal,
                    },
                },
            },
        ],
        bindings: vec![],
        values: vec![
            LoopRecipeValueV2 {
                key: subject,
                class: LoopValueClassV2::Text,
            },
            LoopRecipeValueV2 {
                key: needle,
                class: LoopValueClassV2::Text,
            },
            LoopRecipeValueV2 {
                key: call_value,
                class: LoopValueClassV2::Text,
            },
            LoopRecipeValueV2 {
                key: equal,
                class: LoopValueClassV2::Bool,
            },
        ],
        inputs: vec![subject, needle],
        carriers: vec![],
        exits: vec![],
    };
    let source_binding = LoopRecipeSourceBindingV1::new(
        LoopRecipeSourceOwnerV1::function_body(0, 0),
        vec![LoopNodeSourceBindingV1::new(
            LoopNodeKeyV1::new(0),
            LoopSourcePathV1::new(vec![LoopSourcePathStepV1::BodyItem { index: 0 }]),
        )],
    );
    LoopRecipeArtifactV2::new(
        LoopRecipeProvenanceV1::new(LoopRecipeProducerIdV1::CallableSingleLoopV1),
        source_binding,
        recipe,
    )
}

#[test]
fn v2_typed_call_and_text_eq_round_trip() {
    let artifact = minimal_typed_recipe();
    let json = serde_json::to_string(&artifact).expect("V2 artifact encodes");
    assert!(json.contains("\"schema_version\":2"));
    assert!(json.contains("\"call_slot\""));
    assert!(json.contains("\"text_eq\""));
    let decoded: LoopRecipeArtifactV2 = serde_json::from_str(&json).expect("V2 artifact decodes");
    let verified = LoopRecipeVerifierV2::verify_artifact(decoded).expect("V2 verifies");
    assert_eq!(verified.recipe().as_recipe().values.len(), 4);
}

#[test]
fn v2_rejects_text_eq_with_non_text_operand() {
    let mut artifact = minimal_typed_recipe();
    artifact.recipe.values[1].class = LoopValueClassV2::I64;
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::TextEqOperandClassMismatch {
            item: LoopItemKeyV1::new(1),
        })
    );
}

#[test]
fn v2_rejects_unknown_wire_fields() {
    let artifact = minimal_typed_recipe();
    let mut value: serde_json::Value =
        serde_json::from_str(&serde_json::to_string(&artifact).expect("encode")).expect("value");
    value["recipe"]["unknown"] = serde_json::json!(true);
    let json = serde_json::to_string(&value).expect("mutated JSON");
    assert!(serde_json::from_str::<LoopRecipeArtifactV2>(&json).is_err());
}

#[test]
fn v2_rejects_duplicate_result_definition() {
    let mut artifact = minimal_typed_recipe();
    if let LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::TextEq { result, .. },
    } = &mut artifact.recipe.items[1].item
    {
        *result = LoopValueKeyV1::new(2);
    }
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::DuplicateValueDefinition {
            key: LoopValueKeyV1::new(2),
        })
    );
}

#[test]
fn v2_rejects_wrong_schema_version_before_recipe_use() {
    let mut artifact = minimal_typed_recipe();
    artifact.schema_version = 1;
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::UnsupportedVersion { found: 1 })
    );
}

#[test]
fn v2_rejects_unknown_call_slot_argument() {
    let mut artifact = minimal_typed_recipe();
    if let LoopRecipeItemV2::Operation {
        operation: LoopOperationV2::CallSlot { args, .. },
    } = &mut artifact.recipe.items[0].item
    {
        args.push(LoopValueKeyV1::new(99));
    }
    assert_eq!(
        LoopRecipeVerifierV2::verify_artifact(artifact),
        Err(LoopRecipeV2RejectReason::UnknownValue {
            key: LoopValueKeyV1::new(99),
        })
    );
}
