//! Wire/logical evidence only; no resolver or physical profile is activated.
use super::ids::*;
use super::join_sig::{issue_sole_root_carrier_join_closure_v2, LoopJoinEdgeRoleV1};
use super::schema_v2::*;
use super::typed_schema_v2::{LoopRecipeV2RejectReason, LoopRecipeVerifierV2};

fn recipe(text: &str) -> LoopRecipeV2 {
    let root = LoopNodeKeyV1::new(0);
    let binding = LoopBindingKeyV1::new(0);
    LoopRecipeV2 {
        read_only_inputs: Vec::new(),
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

fn bool_recipe() -> LoopRecipeV2 {
    use super::bool_region::{LoopBoolRegionV1, LoopBoolNodeV1 as N, LoopBoolCompareV1 as C, LoopBoolCompareDomainV1 as D};
    let mut input = recipe("unused");
    input.values[2].class = LoopValueClassV2::Bool;
    input.items[0].item = LoopRecipeItemV2::Operation { operation: LoopOperationV2::BoolExpression {
        inputs: vec![LoopValueKeyV1::new(0); 2], result: LoopValueKeyV1::new(2),
        region: LoopBoolRegionV1 { root: 6, nodes: vec![
            N::Import { ordinal: 0, class: LoopValueClassV2::Text }, N::Text { value: "0".into() },
            N::Compare { op: C::Less, domain: D::Text, left: 0, right: 1 },
            N::Import { ordinal: 1, class: LoopValueClassV2::Text }, N::Text { value: "9".into() },
            N::Compare { op: C::Greater, domain: D::Text, left: 3, right: 4 },
            N::Or { left: 2, right: 5 },
        ] },
    } };
    input.items[1].item = LoopRecipeItemV2::If { condition: LoopValueKeyV1::new(2),
        then_block: LoopBlockKeyV1::new(2), else_block: None };
    input.blocks.push(LoopRecipeBlockV2 { key: LoopBlockKeyV1::new(2), owner_loop: input.root_loop,
        items: vec![LoopItemKeyV1::new(2)] });
    input.items.push(LoopRecipeItemRowV2 { key: LoopItemKeyV1::new(2),
        item: LoopRecipeItemV2::Exit { exit: LoopExitKeyV1::new(0) } });
    input.exits.push(LoopRecipeExitV2 { key: LoopExitKeyV1::new(0), owner_loop: input.root_loop,
        kind: LoopExitKindV2::Break { target_loop: input.root_loop } });
    // A source If's fallthrough continues at the next real Loop item.
    input.blocks[1].items.push(LoopItemKeyV1::new(3));
    input.items.push(LoopRecipeItemRowV2 { key: LoopItemKeyV1::new(3),
        item: LoopRecipeItemV2::Operation { operation: LoopOperationV2::WriteBinding {
            binding: LoopBindingKeyV1::new(0), value: LoopValueKeyV1::new(0),
        } } });
    input
}

#[test]
fn closed_bool_region_preserves_short_circuit_and_fault_before_one_normal_result() {
    let input = bool_recipe();
    let wire = serde_json::to_string(&input).unwrap();
    assert_eq!(serde_json::from_str::<LoopRecipeV2>(&wire).unwrap(), input);
    let LoopRecipeItemV2::Operation { operation } = &input.items[0].item else { unreachable!() };
    assert_eq!(operation.execution_class_v2(), LoopOperationExecutionClassV2::FaultBeforeNormalResult {
        family: LoopOperationFaultFamilyV2::BoolExpression, normal_result: LoopValueKeyV1::new(2),
    });
    let verified = LoopRecipeVerifierV2::verify(input).unwrap();
    let closure = issue_sole_root_carrier_join_closure_v2(&verified).unwrap();
    let sig = closure.join_sig().as_sig();
    assert_eq!(sig.branches.len(), 1, "Bool's internal merge is not another source If or Break");
    assert_eq!(sig.loops[0].edges.iter().filter(|edge| edge.role == LoopJoinEdgeRoleV1::Break).count(), 1);
}

#[test]
fn closed_bool_region_rejects_missing_extra_wrong_class_and_non_tree_inputs() {
    use super::bool_region::{LoopBoolNodeV1 as N, LoopBoolCompareV1 as C, LoopBoolCompareDomainV1 as D};
    for mutation in 0..14 {
        let mut input = bool_recipe();
        let LoopRecipeItemV2::Operation { operation: LoopOperationV2::BoolExpression { region, inputs, result } }
            = &mut input.items[0].item else { unreachable!() };
        match mutation {
            0 => { inputs.pop(); }
            1 => inputs.push(LoopValueKeyV1::new(0)),
            2 => inputs[0] = LoopValueKeyV1::new(1),
            3 => inputs[0] = LoopValueKeyV1::new(99),
            4 => region.nodes[3] = N::Import { ordinal: 0, class: LoopValueClassV2::Text },
            5 => region.nodes[2] = N::Compare { op: C::Less, domain: D::I64, left: 0, right: 1 },
            6 => region.nodes[6] = N::Or { left: 2, right: 2 },
            7 => region.nodes[6] = N::Or { left: 2, right: 6 },
            8 => region.root = 2,
            9 => { region.nodes.push(N::Bool { value: true }); region.root = 7; }
            10 => { region.nodes[6] = N::I64 { value: 1 }; }
            11 => *result = LoopValueKeyV1::new(0),
            12 => input.values[2].class = LoopValueClassV2::Text,
            13 => region.nodes[6] = N::Or { left: 5, right: 2 },
            _ => unreachable!(),
        }
        assert!(LoopRecipeVerifierV2::verify(input).is_err(), "mutation={mutation}");
    }
}
