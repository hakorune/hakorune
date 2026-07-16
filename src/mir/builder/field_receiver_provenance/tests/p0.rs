use super::{assert_error, FixtureV1, OWNER};
use crate::ast::FieldDecl;
use crate::mir::builder::field_receiver_provenance::SameRootReceiverProofErrorV1;
use crate::mir::{Callee, EffectMask, MirInstruction, MirType, ValueId};

#[test]
fn p0_accepts_copy_wrapped_and_three_level_nested_phi() {
    let mut wrapped = FixtureV1::new();
    for id in 1..=3 {
        wrapped.add_block(id);
    }
    wrapped.branch(0, 1, 2);
    let left = wrapped.add_copy(1, wrapped.receiver());
    let right = wrapped.add_copy(2, wrapped.receiver());
    wrapped.jump(1, 3);
    wrapped.jump(2, 3);
    let phi = wrapped.add_phi(3, vec![(1, left), (2, right)]);
    let seed = wrapped.add_copy(3, phi);
    wrapped.use_in(3);
    assert_eq!(wrapped.normalized(seed).unwrap(), "P[R,R]");
    wrapped.assert_final_verifier_accepts(seed);

    let mut nested = FixtureV1::new();
    let first = nested.diamond();
    for id in 4..=9 {
        nested.add_block(id);
    }
    nested.branch(3, 4, 5);
    nested.jump(4, 6);
    nested.jump(5, 6);
    let second = nested.add_phi(6, vec![(4, first), (5, nested.receiver())]);
    nested.branch(6, 7, 8);
    nested.jump(7, 9);
    nested.jump(8, 9);
    let third = nested.add_phi(9, vec![(7, second), (8, nested.receiver())]);
    nested.use_in(9);
    assert_eq!(nested.normalized(third).unwrap(), "P[P[P[R,R],R],R]");
    nested.assert_final_verifier_accepts(third);
}

#[test]
fn p0_accepts_three_predecessors_and_normalizes_reorder() {
    let mut three_way = FixtureV1::new();
    for id in 1..=5 {
        three_way.add_block(id);
    }
    three_way.branch(0, 1, 2);
    three_way.jump(1, 5);
    three_way.branch(2, 3, 4);
    three_way.jump(3, 5);
    three_way.jump(4, 5);
    let receiver = three_way.receiver();
    let phi = three_way.add_phi(5, vec![(1, receiver), (3, receiver), (4, receiver)]);
    three_way.use_in(5);
    assert_eq!(three_way.normalized(phi).unwrap(), "P[R,R,R]");
    three_way.assert_final_verifier_accepts(phi);

    let left = reordered_nested_shape(false);
    let right = reordered_nested_shape(true);
    assert_eq!(left, "P[P[R,R],R]");
    assert_eq!(left, right);
}

fn reordered_nested_shape(reverse: bool) -> String {
    let mut fixture = FixtureV1::new();
    let first = fixture.diamond();
    for id in 4..=6 {
        fixture.add_block(id);
    }
    if reverse {
        fixture.branch(3, 5, 4);
        fixture.jump(5, 6);
        fixture.jump(4, 6);
    } else {
        fixture.branch(3, 4, 5);
        fixture.jump(4, 6);
        fixture.jump(5, 6);
    }
    let receiver = fixture.receiver();
    let inputs = if reverse {
        vec![(5, receiver), (4, first)]
    } else {
        vec![(4, first), (5, receiver)]
    };
    let seed = fixture.add_phi(6, inputs);
    fixture.use_in(6);
    let shape = fixture.normalized(seed).unwrap();
    fixture.assert_final_verifier_accepts(seed);
    shape
}

#[test]
fn p0_rejects_every_unsupported_value_definition_family() {
    let cases: Vec<Box<dyn Fn(&mut FixtureV1) -> ValueId>> = vec![
        Box::new(|fixture| {
            let dst = fixture.typed_value();
            fixture.add_instruction(
                0,
                MirInstruction::NewBox {
                    dst,
                    box_type: OWNER.to_string(),
                    args: Vec::new(),
                },
            );
            dst
        }),
        Box::new(|fixture| {
            let dst = fixture.typed_value();
            fixture.add_instruction(
                0,
                MirInstruction::Call {
                    dst: Some(dst),
                    func: fixture.receiver(),
                    callee: Some(Callee::Global("probe/0".to_string())),
                    args: Vec::new(),
                    effects: EffectMask::PURE,
                },
            );
            dst
        }),
        Box::new(|fixture| {
            let dst = fixture.typed_value();
            fixture.add_instruction(
                0,
                MirInstruction::FieldGet {
                    dst,
                    base: fixture.receiver(),
                    field: "items".to_string(),
                    declared_type: Some(MirType::Box(OWNER.to_string())),
                },
            );
            dst
        }),
        Box::new(|fixture| {
            let dst = fixture.typed_value();
            fixture.add_instruction(
                0,
                MirInstruction::Select {
                    dst,
                    cond: fixture.receiver(),
                    then_val: fixture.receiver(),
                    else_val: fixture.receiver(),
                },
            );
            dst
        }),
        Box::new(|fixture| {
            let dst = fixture.typed_value();
            fixture.add_instruction(
                0,
                MirInstruction::CopyOwned {
                    dst,
                    src: fixture.receiver(),
                },
            );
            dst
        }),
    ];

    for build in cases {
        let mut fixture = FixtureV1::new();
        let seed = build(&mut fixture);
        assert_error(
            fixture.verify(seed),
            SameRootReceiverProofErrorV1::UnsupportedDefinitionKind,
        );
    }
}

#[test]
fn p0_rejects_nested_foreign_mixed_owner_and_non_receiver_parameters() {
    let mut nested_foreign = FixtureV1::new();
    let first = nested_foreign.diamond();
    for id in 4..=6 {
        nested_foreign.add_block(id);
    }
    nested_foreign.branch(3, 4, 5);
    nested_foreign.jump(4, 6);
    nested_foreign.jump(5, 6);
    let seed = nested_foreign.add_phi(6, vec![(4, first), (5, nested_foreign.foreign_parameter())]);
    nested_foreign.use_in(6);
    assert_error(
        nested_foreign.verify(seed),
        SameRootReceiverProofErrorV1::ForeignParameter,
    );

    let mut mixed_owner = FixtureV1::new();
    let mixed = mixed_owner.add_copy(0, mixed_owner.receiver());
    mixed_owner.set_value_type(mixed, MirType::Box("OtherOwner".to_string()));
    assert_error(
        mixed_owner.verify(mixed),
        SameRootReceiverProofErrorV1::SeedTypeMismatch,
    );

    let direct_foreign = FixtureV1::new();
    assert_error(
        direct_foreign.verify(direct_foreign.foreign_parameter()),
        SameRootReceiverProofErrorV1::ForeignParameter,
    );
}

#[test]
fn p0_rejects_static_param_missing_type_and_mismatched_type() {
    let mut static_param = FixtureV1::new();
    static_param.function_mut().metadata.declared_param_decls[0].implicit_receiver = false;
    assert_error(
        static_param.verify(static_param.receiver()),
        SameRootReceiverProofErrorV1::NotInstanceMethod,
    );

    let mut missing = FixtureV1::new();
    let missing_seed = missing.add_copy(0, missing.receiver());
    missing.builder.type_ctx.value_types.remove(&missing_seed);
    assert_error(
        missing.verify(missing_seed),
        SameRootReceiverProofErrorV1::SeedTypeMissing,
    );

    let mut mismatch = FixtureV1::new();
    let mismatch_seed = mismatch.add_copy(0, mismatch.receiver());
    mismatch.set_value_type(mismatch_seed, MirType::Bool);
    assert_error(
        mismatch.verify(mismatch_seed),
        SameRootReceiverProofErrorV1::SeedTypeMismatch,
    );
}

#[test]
fn p0_test_only_declared_field_adapter_rejects_missing_and_untyped_fields() {
    let mut fixture = FixtureV1::new();
    let receiver = fixture.receiver();
    assert_eq!(
        fixture.declared_field_type_after_proof(receiver, "items"),
        Some("ArrayBox")
    );
    assert_eq!(
        fixture.declared_field_type_after_proof(receiver, "missing"),
        None
    );

    fixture
        .builder
        .comp_ctx
        .user_box_field_decls
        .get_mut(OWNER)
        .expect("owner registry")
        .push(FieldDecl {
            name: "untyped".to_string(),
            declared_type_name: None,
            is_weak: false,
            default_value: None,
        });
    assert_eq!(
        fixture.declared_field_type_after_proof(receiver, "untyped"),
        None
    );
}
