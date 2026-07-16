use super::{
    verify_with_normalized_test_view, SameRootReceiverProofErrorV1, VerifiedSameRootReceiverValueV1,
};
use crate::ast::FieldDecl;
use crate::mir::builder::MirBuilder;
use crate::mir::function::{FunctionSignature, MirFunction, MirParamDecl};
use crate::mir::verification::MirVerifier;
use crate::mir::{
    BasicBlock, BasicBlockId, ConstValue, EffectMask, MirInstruction, MirType, ValueId,
};
use hakorune_mir_core::MirValueKind;

mod p0;
mod real_fixture;

const OWNER: &str = "DeclaredFieldOwnerV1";

fn block(id: u32) -> BasicBlockId {
    BasicBlockId::new(id)
}

struct FixtureV1 {
    builder: MirBuilder,
}

impl FixtureV1 {
    fn new() -> Self {
        let mut builder = MirBuilder::new();
        let signature = FunctionSignature {
            name: "DeclaredFieldOwnerV1.probe/1".to_string(),
            params: vec![
                MirType::Box(OWNER.to_string()),
                MirType::Box(OWNER.to_string()),
            ],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, block(0));
        function.metadata.declared_param_decls = vec![
            MirParamDecl {
                name: "me".to_string(),
                declared_type_name: None,
                implicit_receiver: true,
            },
            MirParamDecl {
                name: "other".to_string(),
                declared_type_name: Some(OWNER.to_string()),
                implicit_receiver: false,
            },
        ];
        function
            .metadata
            .value_types
            .insert(ValueId::new(0), MirType::Box(OWNER.to_string()));
        function
            .metadata
            .value_types
            .insert(ValueId::new(1), MirType::Box(OWNER.to_string()));
        builder.scope_ctx.current_function = Some(function);
        builder.current_block = Some(block(0));
        builder.register_value_kind(ValueId::new(0), MirValueKind::Parameter(0));
        builder.register_value_kind(ValueId::new(1), MirValueKind::Parameter(1));
        builder
            .type_ctx
            .value_types
            .insert(ValueId::new(0), MirType::Box(OWNER.to_string()));
        builder
            .type_ctx
            .value_types
            .insert(ValueId::new(1), MirType::Box(OWNER.to_string()));
        builder
            .type_ctx
            .value_origin_newbox
            .insert(ValueId::new(0), OWNER.to_string());
        builder.comp_ctx.register_user_box_with_field_decls(
            OWNER.to_string(),
            vec![FieldDecl {
                name: "items".to_string(),
                declared_type_name: Some("ArrayBox".to_string()),
                is_weak: false,
                default_value: None,
            }],
        );
        Self { builder }
    }

    fn receiver(&self) -> ValueId {
        ValueId::new(0)
    }

    fn foreign_parameter(&self) -> ValueId {
        ValueId::new(1)
    }

    fn add_block(&mut self, id: u32) {
        self.function_mut().add_block(BasicBlock::new(block(id)));
    }

    fn function_mut(&mut self) -> &mut MirFunction {
        self.builder
            .scope_ctx
            .current_function
            .as_mut()
            .expect("test function")
    }

    fn typed_value(&mut self) -> ValueId {
        let value = self.function_mut().next_value_id();
        self.set_value_type(value, MirType::Box(OWNER.to_string()));
        value
    }

    fn set_value_type(&mut self, value: ValueId, ty: MirType) {
        self.builder.type_ctx.value_types.insert(value, ty.clone());
        self.function_mut().metadata.value_types.insert(value, ty);
    }

    fn add_copy(&mut self, block_id: u32, source: ValueId) -> ValueId {
        let destination = self.typed_value();
        self.add_instruction(
            block_id,
            MirInstruction::Copy {
                dst: destination,
                src: source,
            },
        );
        destination
    }

    fn add_phi(&mut self, block_id: u32, inputs: Vec<(u32, ValueId)>) -> ValueId {
        let destination = self.typed_value();
        self.add_instruction(
            block_id,
            MirInstruction::Phi {
                dst: destination,
                inputs: inputs
                    .into_iter()
                    .map(|(predecessor, value)| (block(predecessor), value))
                    .collect(),
                type_hint: Some(MirType::Box(OWNER.to_string())),
            },
        );
        destination
    }

    fn add_unsupported_const(&mut self, block_id: u32) -> ValueId {
        let destination = self.typed_value();
        self.add_instruction(
            block_id,
            MirInstruction::Const {
                dst: destination,
                value: ConstValue::Integer(0),
            },
        );
        destination
    }

    fn add_instruction(&mut self, block_id: u32, instruction: MirInstruction) {
        self.function_mut()
            .get_block_mut(block(block_id))
            .expect("test block")
            .add_instruction(instruction);
    }

    fn branch(&mut self, from: u32, then_block: u32, else_block: u32) {
        let condition = self.function_mut().next_value_id();
        self.set_value_type(condition, MirType::Bool);
        self.add_instruction(
            from,
            MirInstruction::Const {
                dst: condition,
                value: ConstValue::Bool(true),
            },
        );
        self.function_mut()
            .get_block_mut(block(from))
            .expect("branch source")
            .set_terminator(MirInstruction::Branch {
                condition,
                then_bb: block(then_block),
                else_bb: block(else_block),
                then_edge_args: None,
                else_edge_args: None,
            });
    }

    fn jump(&mut self, from: u32, to: u32) {
        self.function_mut()
            .get_block_mut(block(from))
            .expect("jump source")
            .set_terminator(MirInstruction::Jump {
                target: block(to),
                edge_args: None,
            });
    }

    fn use_in(&mut self, block_id: u32) {
        self.builder.current_block = Some(block(block_id));
    }

    fn verify(
        &self,
        seed: ValueId,
    ) -> Result<VerifiedSameRootReceiverValueV1, SameRootReceiverProofErrorV1> {
        VerifiedSameRootReceiverValueV1::verify(&self.builder, seed)
    }

    fn normalized(&self, seed: ValueId) -> Result<String, SameRootReceiverProofErrorV1> {
        verify_with_normalized_test_view(&self.builder, seed).map(|(_, shape)| shape)
    }

    fn declared_field_type_after_proof(&self, seed: ValueId, field: &str) -> Option<&str> {
        let proof = self.verify(seed).ok()?;
        self.builder
            .comp_ctx
            .declared_field_type_name(proof.receiver().owner_box(), field)
    }

    fn assert_final_verifier_accepts(&self, returned: ValueId) {
        let mut function = self
            .builder
            .scope_ctx
            .current_function
            .clone()
            .expect("synthetic function");
        function.signature.return_type = MirType::Box(OWNER.to_string());
        let block_ids: Vec<_> = function.blocks.keys().copied().collect();
        for block_id in block_ids {
            let block = function.get_block_mut(block_id).expect("synthetic block");
            if block.terminator.is_none() {
                block.set_terminator(MirInstruction::Return {
                    value: Some(returned),
                });
            }
        }
        if let Err(errors) = MirVerifier::new().verify_function(&function) {
            panic!("accepted same-root fixture must pass final verifier: {errors:?}");
        }
    }

    fn diamond(&mut self) -> ValueId {
        for id in 1..=3 {
            self.add_block(id);
        }
        self.branch(0, 1, 2);
        self.jump(1, 3);
        self.jump(2, 3);
        let receiver = self.receiver();
        let phi = self.add_phi(3, vec![(1, receiver), (2, receiver)]);
        self.use_in(3);
        phi
    }
}

fn assert_error(
    result: Result<VerifiedSameRootReceiverValueV1, SameRootReceiverProofErrorV1>,
    expected: SameRootReceiverProofErrorV1,
) {
    assert_eq!(result.unwrap_err(), expected);
}

#[test]
fn accepts_receiver_and_copy_chain_without_persistent_metadata() {
    let mut fixture = FixtureV1::new();
    let receiver = fixture.receiver();
    assert_eq!(fixture.normalized(receiver).unwrap(), "R");
    fixture.assert_final_verifier_accepts(receiver);

    let first = fixture.add_copy(0, receiver);
    let second = fixture.add_copy(0, first);
    assert_eq!(fixture.normalized(second).unwrap(), "R");
    let proof = fixture.verify(second).unwrap();
    assert_eq!(proof.value(), second);
    assert_eq!(proof.receiver().receiver_parameter(), receiver);
    assert_eq!(proof.receiver().owner_box(), OWNER);
    assert!(!fixture
        .builder
        .type_ctx
        .value_origin_newbox
        .contains_key(&second));
    fixture.assert_final_verifier_accepts(second);
}

#[test]
fn accepts_one_and_nested_acyclic_phi_with_deterministic_shape() {
    let mut fixture = FixtureV1::new();
    let first = fixture.diamond();
    assert_eq!(fixture.normalized(first).unwrap(), "P[R,R]");
    fixture.assert_final_verifier_accepts(first);

    for id in 4..=6 {
        fixture.add_block(id);
    }
    fixture.branch(3, 4, 5);
    fixture.jump(4, 6);
    fixture.jump(5, 6);
    let receiver = fixture.receiver();
    let nested = fixture.add_phi(6, vec![(4, first), (5, receiver)]);
    fixture.use_in(6);
    assert_eq!(fixture.normalized(nested).unwrap(), "P[P[R,R],R]");
    fixture.assert_final_verifier_accepts(nested);
}

#[test]
fn shared_phi_subgraph_is_memoized_and_preserves_multiplicity() {
    let mut fixture = FixtureV1::new();
    let first = fixture.diamond();
    for id in 4..=6 {
        fixture.add_block(id);
    }
    fixture.branch(3, 4, 5);
    fixture.jump(4, 6);
    fixture.jump(5, 6);
    let shared = fixture.add_phi(6, vec![(4, first), (5, first)]);
    fixture.use_in(6);
    assert_eq!(fixture.normalized(shared).unwrap(), "P[P[R,R],P[R,R]]");
    fixture.assert_final_verifier_accepts(shared);
}

#[test]
fn rejects_foreign_and_unsupported_terminal_definitions() {
    let mut foreign = FixtureV1::new();
    for id in 1..=3 {
        foreign.add_block(id);
    }
    foreign.branch(0, 1, 2);
    foreign.jump(1, 3);
    foreign.jump(2, 3);
    let phi = foreign.add_phi(
        3,
        vec![(1, foreign.receiver()), (2, foreign.foreign_parameter())],
    );
    foreign.use_in(3);
    assert_error(
        foreign.verify(phi),
        SameRootReceiverProofErrorV1::ForeignParameter,
    );

    let mut unsupported = FixtureV1::new();
    let value = unsupported.add_unsupported_const(0);
    assert_error(
        unsupported.verify(value),
        SameRootReceiverProofErrorV1::UnsupportedDefinitionKind,
    );
}

#[test]
fn rejects_value_definition_cycle_before_copy_availability() {
    let mut fixture = FixtureV1::new();
    let first = fixture.typed_value();
    let second = fixture.typed_value();
    fixture.add_instruction(
        0,
        MirInstruction::Copy {
            dst: first,
            src: second,
        },
    );
    fixture.add_instruction(
        0,
        MirInstruction::Copy {
            dst: second,
            src: first,
        },
    );
    assert_error(
        fixture.verify(first),
        SameRootReceiverProofErrorV1::ValueDefinitionCycle,
    );
}

#[test]
fn rejects_forward_copy_and_unavailable_seed() {
    let mut forward = FixtureV1::new();
    let outer = forward.typed_value();
    let inner = forward.typed_value();
    forward.add_instruction(
        0,
        MirInstruction::Copy {
            dst: outer,
            src: inner,
        },
    );
    forward.add_instruction(
        0,
        MirInstruction::Copy {
            dst: inner,
            src: forward.receiver(),
        },
    );
    assert_error(
        forward.verify(outer),
        SameRootReceiverProofErrorV1::CopySourceUnavailable,
    );

    let mut sibling = FixtureV1::new();
    sibling.add_block(1);
    sibling.add_block(2);
    sibling.branch(0, 1, 2);
    let seed = sibling.add_copy(1, sibling.receiver());
    sibling.use_in(2);
    assert_error(
        sibling.verify(seed),
        SameRootReceiverProofErrorV1::SeedUnavailable,
    );
}

#[test]
fn rejects_duplicate_phantom_missing_and_unreachable_phi_rows() {
    let mut duplicate = FixtureV1::new();
    let _ = duplicate.diamond();
    let receiver = duplicate.receiver();
    let phi = duplicate.add_phi(3, vec![(1, receiver), (1, receiver)]);
    assert_error(
        duplicate.verify(phi),
        SameRootReceiverProofErrorV1::DuplicatePhiPredecessor,
    );

    let mut phantom = FixtureV1::new();
    let _ = phantom.diamond();
    let receiver = phantom.receiver();
    let phi = phantom.add_phi(3, vec![(1, receiver), (0, receiver)]);
    assert_error(
        phantom.verify(phi),
        SameRootReceiverProofErrorV1::PhantomPhiPredecessor,
    );

    let mut missing = FixtureV1::new();
    for id in 1..=4 {
        missing.add_block(id);
    }
    missing.branch(0, 1, 2);
    missing.jump(1, 4);
    missing.branch(2, 3, 4);
    missing.jump(3, 4);
    let receiver = missing.receiver();
    let phi = missing.add_phi(4, vec![(1, receiver), (2, receiver)]);
    missing.use_in(4);
    assert_error(
        missing.verify(phi),
        SameRootReceiverProofErrorV1::MissingPhiPredecessor,
    );

    let mut unreachable = FixtureV1::new();
    for id in 1..=4 {
        unreachable.add_block(id);
    }
    unreachable.branch(0, 1, 2);
    unreachable.jump(1, 3);
    unreachable.jump(2, 3);
    unreachable.jump(4, 3);
    let receiver = unreachable.receiver();
    let phi = unreachable.add_phi(3, vec![(1, receiver), (4, receiver)]);
    unreachable.use_in(3);
    assert_error(
        unreachable.verify(phi),
        SameRootReceiverProofErrorV1::UnreachablePhiPredecessor,
    );
}

#[test]
fn rejects_incoming_definition_unavailable_on_attached_edge() {
    let mut fixture = FixtureV1::new();
    for id in 1..=3 {
        fixture.add_block(id);
    }
    fixture.branch(0, 1, 2);
    let sibling_value = fixture.add_copy(2, fixture.receiver());
    fixture.jump(1, 3);
    fixture.jump(2, 3);
    let phi = fixture.add_phi(3, vec![(1, sibling_value), (2, fixture.receiver())]);
    fixture.use_in(3);
    assert_error(
        fixture.verify(phi),
        SameRootReceiverProofErrorV1::PhiIncomingUnavailable,
    );
}

#[test]
fn rejects_self_natural_and_irreducible_cfg_cycles() {
    let mut self_loop = FixtureV1::new();
    self_loop.add_block(1);
    self_loop.jump(0, 1);
    self_loop.jump(1, 1);
    let receiver = self_loop.receiver();
    let phi = self_loop.add_phi(1, vec![(0, receiver), (1, receiver)]);
    self_loop.use_in(1);
    assert_error(
        self_loop.verify(phi),
        SameRootReceiverProofErrorV1::CfgCycleOrBackedge,
    );

    let mut natural = FixtureV1::new();
    for id in 1..=3 {
        natural.add_block(id);
    }
    natural.jump(0, 1);
    natural.branch(1, 2, 3);
    natural.jump(2, 1);
    let receiver = natural.receiver();
    let phi = natural.add_phi(1, vec![(0, receiver), (2, receiver)]);
    natural.use_in(1);
    assert_error(
        natural.verify(phi),
        SameRootReceiverProofErrorV1::CfgCycleOrBackedge,
    );

    let mut irreducible = FixtureV1::new();
    for id in 1..=3 {
        irreducible.add_block(id);
    }
    irreducible.branch(0, 1, 2);
    irreducible.jump(1, 3);
    irreducible.jump(2, 3);
    irreducible.jump(3, 1);
    let receiver = irreducible.receiver();
    let phi = irreducible.add_phi(3, vec![(1, receiver), (2, receiver)]);
    irreducible.use_in(3);
    assert_error(
        irreducible.verify(phi),
        SameRootReceiverProofErrorV1::CfgCycleOrBackedge,
    );
}

#[test]
fn rejects_receiver_identity_contract_drift() {
    let cases = [
        SameRootReceiverProofErrorV1::MissingImplicitReceiverMetadata,
        SameRootReceiverProofErrorV1::NotInstanceMethod,
        SameRootReceiverProofErrorV1::ReceiverKindMismatch,
        SameRootReceiverProofErrorV1::ReceiverOwnerMismatch,
        SameRootReceiverProofErrorV1::ReceiverRegistryMissing,
    ];
    for expected in cases {
        let mut fixture = FixtureV1::new();
        match expected {
            SameRootReceiverProofErrorV1::MissingImplicitReceiverMetadata => {
                fixture.function_mut().metadata.declared_param_decls.clear();
            }
            SameRootReceiverProofErrorV1::NotInstanceMethod => {
                fixture.function_mut().metadata.declared_param_decls[0].implicit_receiver = false;
            }
            SameRootReceiverProofErrorV1::ReceiverKindMismatch => {
                fixture
                    .builder
                    .register_value_kind(fixture.receiver(), MirValueKind::Temporary);
            }
            SameRootReceiverProofErrorV1::ReceiverOwnerMismatch => {
                fixture
                    .builder
                    .type_ctx
                    .value_origin_newbox
                    .insert(fixture.receiver(), "OtherOwner".to_string());
            }
            SameRootReceiverProofErrorV1::ReceiverRegistryMissing => {
                fixture.builder.comp_ctx.user_box_field_decls.clear();
            }
            _ => unreachable!(),
        }
        assert_error(fixture.verify(fixture.receiver()), expected);
    }
}

#[test]
fn rejects_type_origin_definition_and_cfg_cache_drift() {
    let mut missing_type = FixtureV1::new();
    missing_type
        .builder
        .type_ctx
        .value_types
        .remove(&missing_type.receiver());
    assert_error(
        missing_type.verify(missing_type.receiver()),
        SameRootReceiverProofErrorV1::ReceiverOwnerMismatch,
    );

    let mut foreign_origin = FixtureV1::new();
    let copy = foreign_origin.add_copy(0, foreign_origin.receiver());
    foreign_origin
        .builder
        .type_ctx
        .value_origin_newbox
        .insert(copy, "OtherOwner".to_string());
    assert_error(
        foreign_origin.verify(copy),
        SameRootReceiverProofErrorV1::ForeignOrigin,
    );

    let mut duplicate = FixtureV1::new();
    let destination = duplicate.add_copy(0, duplicate.receiver());
    duplicate.add_instruction(
        0,
        MirInstruction::Copy {
            dst: destination,
            src: duplicate.receiver(),
        },
    );
    assert_error(
        duplicate.verify(destination),
        SameRootReceiverProofErrorV1::MultipleDefinition,
    );

    let mut stale_cfg = FixtureV1::new();
    stale_cfg.add_block(1);
    stale_cfg.jump(0, 1);
    stale_cfg
        .function_mut()
        .get_block_mut(block(0))
        .unwrap()
        .successors
        .clear();
    assert_error(
        stale_cfg.verify(stale_cfg.receiver()),
        SameRootReceiverProofErrorV1::CfgSuccessorCacheMismatch,
    );
}
