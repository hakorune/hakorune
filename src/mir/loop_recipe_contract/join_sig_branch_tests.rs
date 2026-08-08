use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::join_sig::{
    LoopJoinBranchArmV1, LoopJoinEdgeRoleV1, LoopJoinSigElaboratorV1, LoopJoinSigRejectReasonV1,
};
use super::schema::{
    LoopConditionV1, LoopExitKindV1, LoopNodeV1, LoopRecipeBindingV1, LoopRecipeBlockV1,
    LoopRecipeCarrierV1, LoopRecipeExitV1, LoopRecipeItemRowV1, LoopRecipeItemV1, LoopRecipeV1,
    LoopRecipeValueV1, LoopValueClassV1,
};
use super::verify::LoopRecipeVerifierV1;

fn branch_recipe() -> LoopRecipeV1 {
    LoopRecipeV1 {
        root_loop: LoopNodeKeyV1::new(0),
        loops: vec![LoopNodeV1 {
            key: LoopNodeKeyV1::new(0),
            parent: None,
            condition: LoopConditionV1::Always,
            body: LoopBlockKeyV1::new(0),
        }],
        blocks: vec![
            LoopRecipeBlockV1 {
                key: LoopBlockKeyV1::new(0),
                owner_loop: LoopNodeKeyV1::new(0),
                items: vec![LoopItemKeyV1::new(0)],
            },
            LoopRecipeBlockV1 {
                key: LoopBlockKeyV1::new(1),
                owner_loop: LoopNodeKeyV1::new(0),
                items: vec![LoopItemKeyV1::new(1)],
            },
            LoopRecipeBlockV1 {
                key: LoopBlockKeyV1::new(2),
                owner_loop: LoopNodeKeyV1::new(0),
                items: vec![LoopItemKeyV1::new(2)],
            },
        ],
        items: vec![
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(0),
                item: LoopRecipeItemV1::If {
                    condition: LoopValueKeyV1::new(0),
                    then_block: LoopBlockKeyV1::new(1),
                    else_block: Some(LoopBlockKeyV1::new(2)),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(1),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(0),
                },
            },
            LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(2),
                item: LoopRecipeItemV1::Exit {
                    exit: LoopExitKeyV1::new(1),
                },
            },
        ],
        bindings: Vec::new(),
        values: vec![LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(0),
            class: LoopValueClassV1::Bool,
        }],
        inputs: vec![LoopValueKeyV1::new(0)],
        carriers: Vec::new(),
        exits: vec![
            LoopRecipeExitV1 {
                key: LoopExitKeyV1::new(0),
                owner_loop: LoopNodeKeyV1::new(0),
                kind: LoopExitKindV1::Break {
                    target_loop: LoopNodeKeyV1::new(0),
                },
            },
            LoopRecipeExitV1 {
                key: LoopExitKeyV1::new(1),
                owner_loop: LoopNodeKeyV1::new(0),
                kind: LoopExitKindV1::Continue {
                    target_loop: LoopNodeKeyV1::new(0),
                },
            },
        ],
    }
}

#[test]
fn loop_true_branch_exit_join_sig_is_deterministic_and_has_no_backedge() {
    let left = LoopRecipeVerifierV1::verify(branch_recipe()).expect("branch recipe verifies");
    let right = LoopRecipeVerifierV1::verify(branch_recipe()).expect("branch recipe verifies");
    let left = LoopJoinSigElaboratorV1::elaborate(&left).expect("branch JoinSig");
    let right = LoopJoinSigElaboratorV1::elaborate(&right).expect("branch JoinSig");
    assert_eq!(left.as_sig(), right.as_sig());
    assert_eq!(left.as_sig().branches.len(), 1);
    let branch = &left.as_sig().branches[0];
    assert_eq!(branch.owner_loop, LoopNodeKeyV1::new(0));
    assert_eq!(branch.if_item, LoopItemKeyV1::new(0));
    let LoopJoinBranchArmV1::Exit(then_exit) = &branch.then_arm else {
        panic!("then arm must be a direct exit");
    };
    let LoopJoinBranchArmV1::Exit(else_exit) = &branch.else_arm else {
        panic!("else arm must be a direct exit");
    };
    assert_eq!(then_exit.exit_item, LoopItemKeyV1::new(1));
    assert_eq!(then_exit.role, LoopJoinEdgeRoleV1::Break);
    assert_eq!(else_exit.exit_item, LoopItemKeyV1::new(2));
    assert_eq!(else_exit.role, LoopJoinEdgeRoleV1::Continue);
    let edges = &left.as_sig().loops[0].edges;
    assert!(edges
        .iter()
        .any(|edge| edge.role == LoopJoinEdgeRoleV1::Break));
    assert!(edges
        .iter()
        .any(|edge| edge.role == LoopJoinEdgeRoleV1::Continue));
    assert!(!edges
        .iter()
        .any(|edge| edge.role == LoopJoinEdgeRoleV1::Backedge));
}

#[test]
fn loop_true_branch_exit_join_sig_records_implicit_else_fallthrough() {
    let mut recipe = branch_recipe();
    recipe.blocks.pop();
    recipe.items.pop();
    recipe.exits.pop();
    recipe.values.push(LoopRecipeValueV1 {
        key: LoopValueKeyV1::new(1),
        class: LoopValueClassV1::I64,
    });
    recipe.items.push(LoopRecipeItemRowV1 {
        key: LoopItemKeyV1::new(2),
        item: LoopRecipeItemV1::Operation {
            operation: super::schema::LoopOperationV1::ConstI64 {
                result: LoopValueKeyV1::new(1),
                value: 7,
            },
        },
    });
    recipe.items[0].item = LoopRecipeItemV1::If {
        condition: LoopValueKeyV1::new(0),
        then_block: LoopBlockKeyV1::new(1),
        else_block: None,
    };
    recipe.blocks[0].items.push(LoopItemKeyV1::new(2));
    let verified = LoopRecipeVerifierV1::verify(recipe).expect("implicit else shape verifies");
    let sig = LoopJoinSigElaboratorV1::elaborate(&verified).expect("implicit fallthrough");
    let branch = &sig.as_sig().branches[0];
    assert!(matches!(
        branch.then_arm,
        LoopJoinBranchArmV1::Exit(ref exit) if exit.role == LoopJoinEdgeRoleV1::Break
    ));
    assert!(matches!(
        branch.else_arm,
        LoopJoinBranchArmV1::Fallthrough { .. }
    ));
    assert_eq!(
        sig.as_sig().loops[0]
            .edges
            .iter()
            .map(|edge| edge.role)
            .collect::<Vec<_>>(),
        vec![
            LoopJoinEdgeRoleV1::Enter,
            LoopJoinEdgeRoleV1::BodyEntry,
            LoopJoinEdgeRoleV1::Break,
            LoopJoinEdgeRoleV1::Backedge,
        ]
    );
}

#[test]
fn loop_true_branch_join_sig_rejects_divergent_normal_arm_state() {
    let mut recipe = branch_recipe();
    recipe.bindings.push(LoopRecipeBindingV1 {
        key: LoopBindingKeyV1::new(0),
        label: "carrier".to_owned(),
        class: LoopValueClassV1::I64,
    });
    recipe.values.extend([
        LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(1),
            class: LoopValueClassV1::I64,
        },
        LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(2),
            class: LoopValueClassV1::I64,
        },
        LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(3),
            class: LoopValueClassV1::I64,
        },
    ]);
    recipe.inputs.extend([
        LoopValueKeyV1::new(1),
        LoopValueKeyV1::new(2),
        LoopValueKeyV1::new(3),
    ]);
    recipe.carriers.push(LoopRecipeCarrierV1 {
        key: LoopCarrierKeyV1::new(0),
        owner_loop: LoopNodeKeyV1::new(0),
        binding: LoopBindingKeyV1::new(0),
        class: LoopValueClassV1::I64,
        entry_value: LoopValueKeyV1::new(1),
    });
    recipe.exits.clear();
    recipe.items = vec![
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(0),
            item: LoopRecipeItemV1::If {
                condition: LoopValueKeyV1::new(0),
                then_block: LoopBlockKeyV1::new(1),
                else_block: Some(LoopBlockKeyV1::new(2)),
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(1),
            item: LoopRecipeItemV1::Operation {
                operation: super::schema::LoopOperationV1::WriteBinding {
                    binding: LoopBindingKeyV1::new(0),
                    value: LoopValueKeyV1::new(2),
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(2),
            item: LoopRecipeItemV1::Operation {
                operation: super::schema::LoopOperationV1::WriteBinding {
                    binding: LoopBindingKeyV1::new(0),
                    value: LoopValueKeyV1::new(3),
                },
            },
        },
    ];
    recipe.blocks[0].items = vec![LoopItemKeyV1::new(0)];
    recipe.blocks[1].items = vec![LoopItemKeyV1::new(1)];
    recipe.blocks[2].items = vec![LoopItemKeyV1::new(2)];
    let verified = LoopRecipeVerifierV1::verify(recipe).expect("normal branch shape verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch {
            item: LoopItemKeyV1::new(0),
        })
    );
}

#[test]
fn loop_true_branch_exit_join_sig_keeps_terminal_arm_payloads_separate() {
    let mut recipe = branch_recipe();
    recipe.bindings.push(LoopRecipeBindingV1 {
        key: LoopBindingKeyV1::new(0),
        label: "carrier".to_owned(),
        class: LoopValueClassV1::I64,
    });
    recipe.values.extend([
        LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(1),
            class: LoopValueClassV1::I64,
        },
        LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(2),
            class: LoopValueClassV1::I64,
        },
        LoopRecipeValueV1 {
            key: LoopValueKeyV1::new(3),
            class: LoopValueClassV1::I64,
        },
    ]);
    recipe.inputs.extend([
        LoopValueKeyV1::new(1),
        LoopValueKeyV1::new(2),
        LoopValueKeyV1::new(3),
    ]);
    recipe.carriers.push(LoopRecipeCarrierV1 {
        key: LoopCarrierKeyV1::new(0),
        owner_loop: LoopNodeKeyV1::new(0),
        binding: LoopBindingKeyV1::new(0),
        class: LoopValueClassV1::I64,
        entry_value: LoopValueKeyV1::new(1),
    });
    recipe.items = vec![
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(0),
            item: LoopRecipeItemV1::If {
                condition: LoopValueKeyV1::new(0),
                then_block: LoopBlockKeyV1::new(1),
                else_block: Some(LoopBlockKeyV1::new(2)),
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(1),
            item: LoopRecipeItemV1::Operation {
                operation: super::schema::LoopOperationV1::WriteBinding {
                    binding: LoopBindingKeyV1::new(0),
                    value: LoopValueKeyV1::new(2),
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(2),
            item: LoopRecipeItemV1::Exit {
                exit: LoopExitKeyV1::new(0),
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(3),
            item: LoopRecipeItemV1::Operation {
                operation: super::schema::LoopOperationV1::WriteBinding {
                    binding: LoopBindingKeyV1::new(0),
                    value: LoopValueKeyV1::new(3),
                },
            },
        },
        LoopRecipeItemRowV1 {
            key: LoopItemKeyV1::new(4),
            item: LoopRecipeItemV1::Exit {
                exit: LoopExitKeyV1::new(1),
            },
        },
    ];
    recipe.blocks[1].items = vec![LoopItemKeyV1::new(1), LoopItemKeyV1::new(2)];
    recipe.blocks[2].items = vec![LoopItemKeyV1::new(3), LoopItemKeyV1::new(4)];
    let verified = LoopRecipeVerifierV1::verify(recipe).expect("branch-write shape verifies");
    let sig = LoopJoinSigElaboratorV1::elaborate(&verified).expect("terminal arm payloads");
    let branch = &sig.as_sig().branches[0];
    let LoopJoinBranchArmV1::Exit(then_exit) = &branch.then_arm else {
        panic!("then arm must be a direct exit");
    };
    let LoopJoinBranchArmV1::Exit(else_exit) = &branch.else_arm else {
        panic!("else arm must be a direct exit");
    };
    assert_ne!(then_exit.payload, else_exit.payload);
    let break_edge = sig.as_sig().loops[0]
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Break)
        .expect("break edge");
    let continue_edge = sig.as_sig().loops[0]
        .edges
        .iter()
        .find(|edge| edge.role == LoopJoinEdgeRoleV1::Continue)
        .expect("continue edge");
    assert_ne!(break_edge.payload, continue_edge.payload);
}

#[test]
fn loop_true_branch_exit_join_sig_rejects_return_arm() {
    let mut recipe = branch_recipe();
    recipe.exits[1].kind = LoopExitKindV1::Return { value: None };
    let verified = LoopRecipeVerifierV1::verify(recipe).expect("return-arm shape verifies");
    assert_eq!(
        LoopJoinSigElaboratorV1::elaborate(&verified),
        Err(LoopJoinSigRejectReasonV1::BranchMergeMismatch {
            item: LoopItemKeyV1::new(2),
        })
    );
}
