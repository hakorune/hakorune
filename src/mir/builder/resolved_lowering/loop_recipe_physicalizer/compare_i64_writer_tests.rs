use super::compare_i64_writer::CanonicalLoopCompareI64WriterV1;
use crate::mir::builder::emission::compare_type::PreparedCanonicalCompareBoolTypeV1;
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedCanonicalOpenInstructionTargetV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    CanonicalSameBlockIntegerRequestV1, CanonicalSsaFunctionSessionV2,
    ReservedCanonicalCompareDestinationV1, VerifiedCanonicalSameBlockIntegerOperandV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_fixture_for_test;
use crate::mir::resolved_control_flow::if_control::VerifiedResolvedFunctionIfControlV1;
use crate::mir::resolved_control_flow::verify_function_completion_v1;
use crate::mir::{BasicBlockId, ConstValue, MirInstruction, MirType, ValueId};

fn with_fixture<R>(
    use_fixture: impl FnOnce(
        &mut MirBuilder,
        &mut CanonicalSsaFunctionSessionV2<'_>,
        VerifiedCanonicalOpenInstructionTargetV1,
        VerifiedCanonicalSameBlockIntegerOperandV1,
        VerifiedCanonicalSameBlockIntegerOperandV1,
        ReservedCanonicalCompareDestinationV1,
    ) -> R,
) -> R {
    let fixture = callable_operation_fixture_for_test();
    let input = fixture.unit.root_function_input().expect("root input");
    let completion = verify_function_completion_v1(input).expect("completion");
    let if_control =
        VerifiedResolvedFunctionIfControlV1::empty_for_loop_profile(input).expect("if control");
    let mut canonical =
        CanonicalSsaFunctionSessionV2::new(input, if_control, completion, 0).expect("session");

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("strict_compare_writer/0".to_owned());
    let target_block = BasicBlockId::new(1);
    let lhs_value = ValueId::new(10);
    let rhs_value = ValueId::new(11);
    {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .expect("function");
        canonical
            .cfg
            .create_block(function, target_block)
            .expect("target creation");
        let target = function.get_block_mut(target_block).expect("target");
        target.add_instruction(MirInstruction::Const {
            dst: lhs_value,
            value: ConstValue::Integer(1),
        });
        target.add_instruction(MirInstruction::Const {
            dst: rhs_value,
            value: ConstValue::Integer(2),
        });
    }
    builder
        .function_state
        .type_ctx
        .set_type(lhs_value, MirType::Integer);
    builder
        .function_state
        .type_ctx
        .set_type(rhs_value, MirType::Integer);

    let owner = input.owner();
    let target = canonical
        .cfg
        .prepare_created_open_instruction_target(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("function"),
            owner,
            target_block,
        )
        .expect("open target");
    let lhs = canonical
        .prepare_existing_same_block_integer(
            &builder,
            CanonicalSameBlockIntegerRequestV1::from_parts(owner, target_block, lhs_value),
        )
        .expect("lhs witness");
    let rhs = canonical
        .prepare_existing_same_block_integer(
            &builder,
            CanonicalSameBlockIntegerRequestV1::from_parts(owner, target_block, rhs_value),
        )
        .expect("rhs witness");
    let destination = canonical
        .reserve_compare_destination(&mut builder)
        .expect("destination");
    use_fixture(&mut builder, &mut canonical, target, lhs, rhs, destination)
}

#[test]
fn strict_writer_appends_one_compare_and_returns_definition_source() {
    with_fixture(|builder, _canonical, target, lhs, rhs, destination| {
        let owner = target.owner();
        let target_block = target.block();
        let destination_value = destination.value();
        let bool_plan = PreparedCanonicalCompareBoolTypeV1::prepare(None).expect("Bool plan");
        let source = CanonicalLoopCompareI64WriterV1::emit(
            builder,
            target,
            lhs,
            rhs,
            destination,
            crate::mir::CompareOp::Lt,
            bool_plan,
        )
        .expect("strict Compare");

        let function = builder
            .function_state
            .current_function
            .as_ref()
            .expect("function");
        let target = function.get_block(target_block).expect("target");
        assert_eq!(target.instructions.len(), 3);
        assert!(matches!(
            target.instructions.last(),
            Some(MirInstruction::Compare {
                dst,
                op: crate::mir::CompareOp::Lt,
                lhs,
                rhs,
            }) if *dst == destination_value && *lhs == ValueId::new(10) && *rhs == ValueId::new(11)
        ));
        assert_eq!(source.owner(), owner);
        assert_eq!(source.target(), target_block);
        assert_eq!(source.physical_value(), destination_value);
        assert_eq!(
            builder.function_state.type_ctx.get_type(destination_value),
            Some(&MirType::Bool)
        );
    });
}

#[test]
fn sealed_target_rejects_before_append_and_type_publication() {
    with_fixture(|builder, _canonical, target, lhs, rhs, destination| {
        let destination_value = destination.value();
        let target_block = target.block();
        builder
            .function_state
            .current_function
            .as_mut()
            .expect("function")
            .get_block_mut(target_block)
            .expect("target")
            .seal();
        let before = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .get_block(target_block)
            .unwrap()
            .instructions
            .len();
        let bool_plan = PreparedCanonicalCompareBoolTypeV1::prepare(None).expect("Bool plan");
        let result = CanonicalLoopCompareI64WriterV1::emit(
            builder,
            target,
            lhs,
            rhs,
            destination,
            crate::mir::CompareOp::Eq,
            bool_plan,
        );
        assert!(matches!(
            result,
            Err(crate::mir::builder::builder_emit::CanonicalCompareAppendRejectV1::TargetSealed(
                block
            )) if block == target_block
        ));
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .get_block(target_block)
                .unwrap()
                .instructions
                .len(),
            before
        );
        assert_eq!(
            builder.function_state.type_ctx.get_type(destination_value),
            None
        );
    });
}

#[test]
fn operand_definition_drift_rejects_without_partial_effect() {
    with_fixture(|builder, _canonical, target, lhs, rhs, destination| {
        let target_block = target.block();
        let destination_value = destination.value();
        builder
            .function_state
            .current_function
            .as_mut()
            .expect("function")
            .get_block_mut(target_block)
            .expect("target")
            .instructions[0] = MirInstruction::Const {
            dst: ValueId::new(99),
            value: ConstValue::Integer(1),
        };
        let bool_plan = PreparedCanonicalCompareBoolTypeV1::prepare(None).expect("Bool plan");
        let result = CanonicalLoopCompareI64WriterV1::emit(
            builder,
            target,
            lhs,
            rhs,
            destination,
            crate::mir::CompareOp::Le,
            bool_plan,
        );
        assert_eq!(
            result,
            Err(crate::mir::builder::builder_emit::CanonicalCompareAppendRejectV1::OperandDefinitionDrift)
        );
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .unwrap()
                .get_block(target_block)
                .unwrap()
                .instructions
                .len(),
            2
        );
        assert_eq!(
            builder.function_state.type_ctx.get_type(destination_value),
            None
        );
    });
}
