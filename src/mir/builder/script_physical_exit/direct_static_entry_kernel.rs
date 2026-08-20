//! Detached Script direct-static entry kernel.
//!
//! This helper consumes an AST-free physical input.  It does not open or
//! finish a session; the existing entry session remains the only candidate,
//! Return, signature, verifier, and finish owner.

use crate::mir::builder::normal_script_direct_static_join_handoff::{
    ScalarBinaryOperatorV1, ScalarOperandRecipeNodeV1, ScalarUnaryOperatorV1,
    VerifiedScriptDirectStaticPhysicalInputV1,
};
use crate::mir::builder::normal_script_direct_static_physical_publication::
    PreparedScriptDirectStaticResultPublicationV1;
use crate::mir::builder::normal_script_direct_static_recipe::ScriptDirectStaticRecipeKeyV1;
use crate::mir::builder::calls::emit_static_global_value_terminal_with_receipt_v1;
use crate::mir::{BinaryOp, MirBuilder, MirInstruction, MirType, UnaryOp, ValueId};

use super::{
    CompletedScriptPhysicalFunctionV1, OpenScriptPhysicalEntrySessionV1,
    ScriptPhysicalEntrySessionErrorV1,
};

pub(in crate::mir) fn lower_direct_static_physical_input_v1(
    mut session: OpenScriptPhysicalEntrySessionV1,
    input: &VerifiedScriptDirectStaticPhysicalInputV1,
    key: ScriptDirectStaticRecipeKeyV1,
) -> Result<CompletedScriptPhysicalFunctionV1, (OpenScriptPhysicalEntrySessionV1, String)> {
    let Some(row) = input.row(key) else {
        return Err((
            session,
            "[freeze:contract][script-direct-static/input-row-missing]".to_owned(),
        ));
    };
    let target = row.target().clone();
    if target.namespace() != crate::mir::builder::SameModuleCallableNamespaceV1::StaticBoxMethod
        || target.arity() as usize != row.arguments().len()
    {
        return Err((
            session,
            "[freeze:contract][script-direct-static/input-target]".to_owned(),
        ));
    }

    let argument_values = {
        let builder = session.builder_mut();
        row.arguments()
            .iter()
            .map(|argument| lower_node(builder, argument.tree()))
            .collect::<Result<Vec<_>, _>>()
    };
    let argument_values = match argument_values {
        Ok(values) => values,
        Err(error) => return Err((session, error)),
    };
    if argument_values.len() != target.arity() as usize {
        return Err((
            session,
            "[freeze:contract][script-direct-static/input-arity]".to_owned(),
        ));
    }

    let emission = match emit_static_global_value_terminal_with_receipt_v1(
        session.builder_mut(),
        target.owner(),
        target.name(),
        target.arity(),
        argument_values,
    ) {
        Ok(emission) => emission,
        Err(error) => {
            return Err((
                session,
                format!("[freeze:contract][script-direct-static/input-call] {error:?}"),
            ))
        }
    };
    let publication = match PreparedScriptDirectStaticResultPublicationV1::prepare(
        row.representation(),
        emission,
    ) {
        Ok(publication) => publication,
        Err(error) => return Err((session, error)),
    };
    let value = match publication.commit(session.builder_mut()) {
        Ok(value) => value,
        Err(error) => return Err((session, error)),
    };

    // The source terminal is validated by the Join/Recipe products before any
    // physical effect.  Both accepted forms lower to the existing value
    // terminal; the session's sole exit owner performs the final Return.
    let _ = row.destination();
    session
        .complete_lowered_terminal_v1(super::LoweredScriptTerminalV1::Value { value })
        .map_err(|(session, error)| (session, format_session_error(error)))
}

fn lower_node(builder: &mut MirBuilder, node: &ScalarOperandRecipeNodeV1) -> Result<ValueId, String> {
    match node {
        ScalarOperandRecipeNodeV1::Literal { value, .. } => {
            crate::mir::builder::emission::constant::emit_integer(builder, *value)
        }
        ScalarOperandRecipeNodeV1::Unary {
            operator, operand, ..
        } => {
            let operand = lower_node(builder, operand)?;
            let dst = builder.next_value_id();
            let op = match operator {
                ScalarUnaryOperatorV1::Minus => UnaryOp::Neg,
                ScalarUnaryOperatorV1::BitNot => UnaryOp::BitNot,
            };
            builder.emit_instruction(MirInstruction::UnaryOp { dst, op, operand })?;
            builder.function_state.type_ctx.set_type(dst, MirType::Integer);
            Ok(dst)
        }
        ScalarOperandRecipeNodeV1::Binary {
            operator, lhs, rhs, ..
        } => {
            let lhs = lower_node(builder, lhs)?;
            let rhs = lower_node(builder, rhs)?;
            let dst = builder.next_value_id();
            let op = match operator {
                ScalarBinaryOperatorV1::Add => BinaryOp::Add,
                ScalarBinaryOperatorV1::Subtract => BinaryOp::Sub,
                ScalarBinaryOperatorV1::Multiply => BinaryOp::Mul,
                ScalarBinaryOperatorV1::BitAnd => BinaryOp::BitAnd,
                ScalarBinaryOperatorV1::BitOr => BinaryOp::BitOr,
                ScalarBinaryOperatorV1::BitXor => BinaryOp::BitXor,
            };
            builder.emit_instruction(MirInstruction::BinOp { dst, op, lhs, rhs })?;
            builder.function_state.type_ctx.set_type(dst, MirType::Integer);
            Ok(dst)
        }
    }
}

fn format_session_error(error: ScriptPhysicalEntrySessionErrorV1) -> String {
    format!("[freeze:contract][script-direct-static/input-exit] {error:?}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::normal_script_direct_static_join_handoff::{
        ScalarOperandRecipeArgumentV1, VerifiedScriptDirectStaticJoinRowV1,
        VerifiedScriptDirectStaticPhysicalInputRowV1,
    };
    use crate::mir::builder::normal_script_direct_static_recipe::{
        ScriptDirectStaticRecipeDestinationV1, ScriptDirectStaticRecipeKeyV1,
    };
    use crate::mir::builder::{canonical_normal_main_entry_target, CanonicalSameModuleCallableKeyV1};
    use crate::mir::callable_result_representation::VerifiedCallableResultRepresentationV1;
    use crate::mir::resolved_semantics::{FunctionOwnerIssuerV1, SourcePathSegmentV1, SourcePathV1};
    use std::collections::BTreeMap;

    #[test]
    fn detached_kernel_emits_one_receipted_call_and_completes_session() {
        crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            let mut issuer = FunctionOwnerIssuerV1::new_for_compilation().expect("owner issuer");
            let owner = issuer.issue().expect("source owner");
            let statement = SourcePathV1::program_body()
                .child(SourcePathSegmentV1::ProgramBody(0))
                .stmt();
            let call_site = SourcePathV1::from_node(statement.node()).expr();
            let receiver_site = SourcePathV1::from_node(call_site.node())
                .child(SourcePathSegmentV1::Receiver)
                .expr();
            let argument_site = SourcePathV1::from_node(call_site.node())
                .child(SourcePathSegmentV1::Argument(0))
                .expr();
            let key = ScriptDirectStaticRecipeKeyV1::from_ordinal_for_test(0);
            let target = CanonicalSameModuleCallableKeyV1::test_static_box_method(
                "Helpers", "run", 1,
            );
            let join = VerifiedScriptDirectStaticJoinRowV1::from_parts_for_test(
                key,
                owner,
                call_site.clone(),
                receiver_site,
                vec![argument_site.clone()].into_boxed_slice(),
                call_site,
                Box::new([]),
                ScriptDirectStaticRecipeDestinationV1::FinalSequence { statement },
                target,
                VerifiedCallableResultRepresentationV1::ExactI64,
                Box::new([]),
            );
            let argument = ScalarOperandRecipeArgumentV1::from_parts_for_test(
                0,
                argument_site.clone(),
                ScalarOperandRecipeNodeV1::Literal {
                    site: argument_site,
                    value: 7,
                },
            );
            let row = VerifiedScriptDirectStaticPhysicalInputRowV1::from_parts_for_test(
                key,
                join,
                vec![argument].into_boxed_slice(),
            );
            let input = VerifiedScriptDirectStaticPhysicalInputV1::from_parts_for_test(
                owner,
                41,
                BTreeMap::from([(key, row)]),
            );
            let live = MirBuilder::new();
            let session = OpenScriptPhysicalEntrySessionV1::open(
                &live,
                canonical_normal_main_entry_target(),
            )
            .expect("detached Script session");
            let completed = match lower_direct_static_physical_input_v1(session, &input, key) {
                Ok(completed) => completed,
                Err((_session, error)) => panic!("detached direct-static lowering: {error}"),
            };
            let function = completed.draft();
            let block = function
                .blocks
                .values()
                .next()
                .expect("entry block");
            assert_eq!(
                block
                    .instructions
                    .iter()
                    .filter(|instruction| matches!(instruction, MirInstruction::Call { .. }))
                    .count(),
                1
            );
            assert_eq!(function.signature.return_type, MirType::Integer);
            assert!(matches!(
                block.terminator.as_ref(),
                Some(MirInstruction::Return { value: Some(_) })
            ));
        });
    }
}
