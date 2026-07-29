//! Throw statement implementation.
//!
//! This module implements the throw statement for exception raising,
//! with proper cleanup block validation.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::cleanup::{
    ensure_cleanup_exit_allowed_v1, CleanupExitKindV1,
};
use crate::mir::builder::function_lowering_state::FunctionLoweringStateV1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};
use crate::mir::builder::{Effect, EffectMask, MirInstruction, ValueId};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum RawThrowCompletionRouteV1 {
    DebugTraceCompatibility,
    Throw,
}

pub(in crate::mir::builder) struct PreparedRawThrowV1 {
    expression: ASTNode,
    completion: RawThrowCompletionRouteV1,
}

impl PreparedRawThrowV1 {
    pub(in crate::mir::builder) fn prepare(
        state: &FunctionLoweringStateV1,
        expression: ASTNode,
    ) -> Result<Self, String> {
        ensure_cleanup_exit_allowed_v1(state, CleanupExitKindV1::Throw)?;
        let completion = if crate::config::env::builder_disable_throw() {
            RawThrowCompletionRouteV1::DebugTraceCompatibility
        } else {
            RawThrowCompletionRouteV1::Throw
        };
        Ok(Self {
            expression,
            completion,
        })
    }
}

pub(in crate::mir::builder) fn lower_prepared_raw_throw_with_port_v1<Port>(
    builder: &mut super::super::super::MirBuilder,
    port: &mut Port,
    prepared: PreparedRawThrowV1,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    let exception_value = drive_legacy_expression_v1(builder, port, prepared.expression)?;
    match prepared.completion {
        RawThrowCompletionRouteV1::DebugTraceCompatibility => {
            builder.emit_extern_call_with_effects(
                "env.debug",
                "trace",
                vec![exception_value],
                None,
                EffectMask::PURE.add(Effect::Debug),
            )?;
        }
        RawThrowCompletionRouteV1::Throw => {
            builder.emit_instruction(MirInstruction::Throw {
                exception: exception_value,
                effects: EffectMask::PANIC,
            })?;
        }
    }
    Ok(exception_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RawLegacyChildLoweringPortV1;
    use crate::mir::MirBuilder;

    fn literal(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn instructions(builder: &MirBuilder) -> Vec<&MirInstruction> {
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter().chain(block.terminator.iter()))
            .collect()
    }

    #[test]
    fn prepared_throw_completion_route_is_not_reselected_after_child_descent() {
        for (completion, expects_throw) in [
            (RawThrowCompletionRouteV1::Throw, true),
            (RawThrowCompletionRouteV1::DebugTraceCompatibility, false),
        ] {
            let mut builder = MirBuilder::new();
            builder.enter_function_for_test("prepared_throw/0".to_string());
            let prepared = PreparedRawThrowV1 {
                expression: literal(7),
                completion,
            };
            let mut port = RawLegacyChildLoweringPortV1;
            lower_prepared_raw_throw_with_port_v1(&mut builder, &mut port, prepared).unwrap();
            let rows = instructions(&builder);
            assert_eq!(
                rows.iter()
                    .filter(|row| matches!(row, MirInstruction::Throw { .. }))
                    .count(),
                usize::from(expects_throw),
                "{rows:?}"
            );
            assert_eq!(
                rows.iter()
                    .filter(|row| matches!(row, MirInstruction::Call { .. }))
                    .count(),
                usize::from(!expects_throw),
                "{rows:?}"
            );
        }
    }
}
