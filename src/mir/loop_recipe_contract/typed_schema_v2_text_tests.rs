//! Wire/logical evidence only; no resolver or physical profile is activated.
use super::ids::*;
use super::join_sig::{issue_sole_root_carrier_join_closure_v2, LoopJoinEdgeRoleV1};
use super::schema_v2::*;
use super::typed_schema_v2::{LoopRecipeV2RejectReason, LoopRecipeVerifierV2};

fn recipe(text: &str) -> LoopRecipeV2 {
    let root = LoopNodeKeyV1::new(0);
    let binding = LoopBindingKeyV1::new(0);
    LoopRecipeV2 {
        root_loop: root,
        loops: vec![LoopNodeV2 {
            key: root,
            parent: None,
            condition: LoopConditionV2::Predicate {
                block: LoopBlockKeyV1::new(0),
                value: LoopValueKeyV1::new(1),
            },
            body: LoopBlockKeyV1::new(1),
        }],
        blocks: vec![
            LoopRecipeBlockV2 {
                key: LoopBlockKeyV1::new(0),
                owner_loop: root,
                items: vec![],
            },
            LoopRecipeBlockV2 {
                key: LoopBlockKeyV1::new(1),
                owner_loop: root,
                items: vec![LoopItemKeyV1::new(0), LoopItemKeyV1::new(1)],
            },
        ],
        items: vec![
            LoopRecipeItemRowV2 {
                key: LoopItemKeyV1::new(0),
                item: LoopRecipeItemV2::Operation {
                    operation: LoopOperationV2::ConstText {
                        result: LoopValueKeyV1::new(2),
                        value: text.into(),
                    },
                },
            },
            LoopRecipeItemRowV2 {
                key: LoopItemKeyV1::new(1),
                item: LoopRecipeItemV2::Operation {
                    operation: LoopOperationV2::WriteBinding {
                        binding,
                        value: LoopValueKeyV1::new(2),
                    },
                },
            },
        ],
        bindings: vec![LoopRecipeBindingV2 {
            key: binding,
            label: "text".into(),
            class: LoopValueClassV2::Text,
        }],
        values: [
            LoopValueClassV2::Text,
            LoopValueClassV2::Bool,
            LoopValueClassV2::Text,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, class)| LoopRecipeValueV2 {
            key: LoopValueKeyV1::new(index as u32),
            class,
        })
        .collect(),
        inputs: vec![LoopValueKeyV1::new(0), LoopValueKeyV1::new(1)],
        carriers: vec![LoopRecipeCarrierV2 {
            key: LoopCarrierKeyV1::new(0),
            owner_loop: root,
            binding,
            class: LoopValueClassV2::Text,
            entry_value: LoopValueKeyV1::new(0),
        }],
        exits: vec![],
    }
}

#[test]
fn const_text_wire_preserves_bytes_and_join_sig_tracks_the_normal_result() {
    for text in ["", "猫🐾", "left\0right"] {
        let input = recipe(text);
        let wire = serde_json::to_string(&input).expect("serialize");
        let decoded: LoopRecipeV2 = serde_json::from_str(&wire).expect("deserialize");
        assert_eq!(decoded, input);
        let LoopRecipeItemV2::Operation { operation } = &decoded.items[0].item else {
            panic!("operation")
        };
        assert_eq!(
            operation.execution_class_v2(),
            LoopOperationExecutionClassV2::NonFaulting
        );
        let verified = LoopRecipeVerifierV2::verify(decoded).expect("typed Text result");
        let closure =
            issue_sole_root_carrier_join_closure_v2(&verified).expect("logical transfers");
        assert_eq!(closure.after_class(), LoopValueClassV2::Text);
        let signature = closure.join_sig().as_sig();
        let backedge = signature.loops[0]
            .edges
            .iter()
            .find(|edge| edge.role == LoopJoinEdgeRoleV1::Backedge)
            .expect("backedge");
        assert_eq!(backedge.payload.len(), 1);
        assert_eq!(backedge.payload[0].value, LoopValueKeyV1::new(2));
        assert_eq!(backedge.payload[0].class, LoopValueClassV2::Text);
    }
}

#[test]
fn const_text_rejects_wrong_or_missing_result_class() {
    for class in [
        None,
        Some(LoopValueClassV2::I64),
        Some(LoopValueClassV2::Dynamic),
    ] {
        let mut input = recipe("text");
        match class {
            Some(class) => input.values[2].class = class,
            None => {
                input.values.pop();
            }
        }
        assert!(matches!(LoopRecipeVerifierV2::verify(input),
            Err(LoopRecipeV2RejectReason::ValueClassMismatch { key }) if key == LoopValueKeyV1::new(2)));
    }
}

#[test]
fn const_text_rejects_duplicate_definition_including_input_redefinition() {
    for duplicate_input in [false, true] {
        let mut input = recipe("text");
        if duplicate_input {
            input.inputs.push(LoopValueKeyV1::new(2));
        } else {
            input.items[1].item = input.items[0].item.clone();
        }
        assert!(matches!(LoopRecipeVerifierV2::verify(input),
            Err(LoopRecipeV2RejectReason::DuplicateValueDefinition { key }) if key == LoopValueKeyV1::new(2)));
    }
}

#[test]
fn const_text_wire_requires_exact_text_field() {
    for value in [serde_json::json!(null), serde_json::json!(7)] {
        let mut wire = serde_json::to_value(recipe("text")).unwrap();
        wire["items"][0]["item"]["operation"]["value"] = value;
        assert!(serde_json::from_value::<LoopRecipeV2>(wire).is_err());
    }
}
