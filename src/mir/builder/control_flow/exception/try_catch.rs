//! Try/catch/finally exception handling implementation.
//!
//! This module implements the control flow for try/catch/finally blocks,
//! including proper handling of deferred returns and cleanup blocks.

use crate::ast::{ASTNode, CatchClause};
use crate::mir::builder::control_flow::cleanup::CleanupExitPolicyV1;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_body_v1, RawAstChildLoweringPortV1,
};
use crate::mir::builder::{MirInstruction, ValueId};

use super::try_catch_state::ActiveRawTryCatchFunctionStateV1;

pub(in crate::mir::builder) struct PreparedRawTryCatchV1 {
    try_body: Vec<ASTNode>,
    catch_clauses: Vec<CatchClause>,
    finally_body: Option<Vec<ASTNode>>,
    cleanup_exit_policy: CleanupExitPolicyV1,
}

impl PreparedRawTryCatchV1 {
    pub(in crate::mir::builder) fn prepare(
        try_body: Vec<ASTNode>,
        catch_clauses: Vec<CatchClause>,
        finally_body: Option<Vec<ASTNode>>,
        cleanup_exit_policy: CleanupExitPolicyV1,
    ) -> Self {
        Self {
            try_body,
            catch_clauses,
            finally_body,
            cleanup_exit_policy,
        }
    }
}

pub(in crate::mir::builder) fn lower_prepared_raw_try_catch_with_port_v1<Port>(
    builder: &mut super::super::super::MirBuilder,
    port: &mut Port,
    prepared: PreparedRawTryCatchV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let PreparedRawTryCatchV1 {
        try_body,
        catch_clauses,
        finally_body,
        cleanup_exit_policy,
    } = prepared;
    let try_block = builder.next_block_id();
    let catch_block = builder.next_block_id();
    let finally_block = if finally_body.is_some() {
        Some(builder.next_block_id())
    } else {
        None
    };
    let exit_block = builder.next_block_id();
    let ret_slot = builder.next_value_id();
    let transaction = ActiveRawTryCatchFunctionStateV1::begin(
        &mut builder.function_state,
        ret_slot,
        finally_block.unwrap_or(exit_block),
    );
    let mut catch_clauses = catch_clauses.into_iter();
    let first_catch = catch_clauses.next();

    let result = (|| -> Result<ValueId, String> {
        if let Some(catch_clause) = first_catch.as_ref() {
            if crate::config::env::builder_trycatch_debug() {
                let ring0 = crate::runtime::get_global_ring0();
                ring0.log.debug(&format!(
                    "[BUILDER] Emitting catch handler for {:?}",
                    catch_clause.exception_type
                ));
            }
            let exception_value = builder.next_value_id();
            builder.emit_instruction(MirInstruction::Catch {
                exception_type: catch_clause.exception_type.clone(),
                exception_value,
                handler_bb: catch_block,
            })?;
        }

        crate::mir::builder::emission::branch::emit_jump(builder, try_block)?;
        builder.start_new_block(try_block)?;
        let _try_result = drive_legacy_body_v1(builder, port, try_body)?;
        if !builder.is_current_block_terminated() {
            let next_target = finally_block.unwrap_or(exit_block);
            crate::mir::builder::emission::branch::emit_jump(builder, next_target)?;
        }

        builder.start_new_block(catch_block)?;
        if crate::config::env::builder_trycatch_debug() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0
                .log
                .debug(&format!("[BUILDER] Enter catch block {:?}", catch_block));
        }
        if let Some(catch_clause) = first_catch {
            drive_legacy_body_v1(builder, port, catch_clause.body)?;
        }
        if !builder.is_current_block_terminated() {
            let next_target = finally_block.unwrap_or(exit_block);
            crate::mir::builder::emission::branch::emit_jump(builder, next_target)?;
        }

        let mut cleanup_terminated = false;
        if let (Some(finally_block_id), Some(finally_statements)) = (finally_block, finally_body) {
            builder.start_new_block(finally_block_id)?;
            transaction.enter_cleanup(
                &mut builder.function_state,
                cleanup_exit_policy.allows_return(),
                cleanup_exit_policy.allows_throw(),
            );
            drive_legacy_body_v1(builder, port, finally_statements)?;
            cleanup_terminated = builder.is_current_block_terminated();
            if !cleanup_terminated {
                crate::mir::builder::emission::branch::emit_jump(builder, exit_block)?;
            }
            transaction.leave_cleanup(&mut builder.function_state);
        }

        builder.start_new_block(exit_block)?;
        if builder
            .function_state
            .protected_region
            .return_defer
            .emitted()
            && !cleanup_terminated
        {
            builder.emit_instruction(MirInstruction::Return {
                value: Some(ret_slot),
            })?;
        }
        crate::mir::builder::emission::constant::emit_void(builder)
    })();

    match result {
        Ok(value) => Ok(transaction
            .complete_success(&mut builder.function_state, value)
            .into_value()),
        Err(error) => {
            let rejected = transaction.reject(error);
            let error = rejected.error().to_string();
            rejected.discard();
            Err(error)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::RecursiveChildLoweringPortV1;
    use crate::mir::MirBuilder;

    struct RecordingBodyPortV1 {
        demands: Vec<i64>,
        cleanup_policies: Vec<(bool, bool)>,
        fail_on: Option<i64>,
    }

    impl RecordingBodyPortV1 {
        fn new(fail_on: Option<i64>) -> Self {
            Self {
                demands: Vec::new(),
                cleanup_policies: Vec::new(),
                fail_on,
            }
        }
    }

    impl RecursiveChildLoweringPortV1 for RecordingBodyPortV1 {
        type BodyInput = Vec<ASTNode>;
        type StatementInput = ASTNode;
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            builder: &mut MirBuilder,
            input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            let Some(ASTNode::Literal {
                value: LiteralValue::Integer(tag),
                ..
            }) = input.first()
            else {
                return Err("[try-catch/test-invalid-body]".to_string());
            };
            self.demands.push(*tag);
            if builder.function_state.protected_region.cleanup.active {
                self.cleanup_policies.push((
                    builder.function_state.protected_region.cleanup.allow_return,
                    builder.function_state.protected_region.cleanup.allow_throw,
                ));
            }
            if self.fail_on == Some(*tag) {
                return Err(format!("fail-{tag}"));
            }
            Ok(builder.next_value_id())
        }

        fn lower_statement(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            unreachable!("TryCatch test port lowers bodies only")
        }

        fn lower_expression(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            unreachable!("TryCatch test port lowers bodies only")
        }
    }

    fn body(tag: i64) -> Vec<ASTNode> {
        vec![ASTNode::Literal {
            value: LiteralValue::Integer(tag),
            span: Span::unknown(),
        }]
    }

    fn catch(tag: i64) -> CatchClause {
        CatchClause {
            exception_type: Some("Exception".to_string()),
            variable_name: None,
            body: body(tag),
            span: Span::unknown(),
        }
    }

    fn prepared(
        try_tag: i64,
        catch_tags: &[i64],
        finally_tag: Option<i64>,
    ) -> PreparedRawTryCatchV1 {
        PreparedRawTryCatchV1::prepare(
            body(try_tag),
            catch_tags.iter().copied().map(catch).collect(),
            finally_tag.map(body),
            CleanupExitPolicyV1::default(),
        )
    }

    fn builder() -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("try_catch_test/0".to_string());
        builder
    }

    #[test]
    fn enabled_route_uses_first_catch_and_finally_once_in_order() {
        let mut builder = builder();
        let mut port = RecordingBodyPortV1::new(None);
        lower_prepared_raw_try_catch_with_port_v1(
            &mut builder,
            &mut port,
            prepared(1, &[2, 99], Some(3)),
        )
        .unwrap();
        assert_eq!(port.demands, vec![1, 2, 3]);
    }

    #[test]
    fn try_and_catch_failures_stop_later_bodies_and_keep_inner_defer_state() {
        for (fail_on, expected_demands) in [(1, vec![1]), (2, vec![1, 2])] {
            let mut builder = builder();
            let mut port = RecordingBodyPortV1::new(Some(fail_on));
            let error = lower_prepared_raw_try_catch_with_port_v1(
                &mut builder,
                &mut port,
                prepared(1, &[2], Some(3)),
            )
            .unwrap_err();
            assert_eq!(error, format!("fail-{fail_on}"));
            assert_eq!(port.demands, expected_demands);
            assert!(builder
                .function_state
                .protected_region
                .return_defer
                .is_active());
            assert!(builder
                .function_state
                .protected_region
                .return_defer
                .retained_slot()
                .is_some());
            assert!(builder
                .function_state
                .protected_region
                .return_defer
                .retained_target()
                .is_some());
            assert!(!builder.function_state.protected_region.cleanup.active);
        }
    }

    #[test]
    fn finally_failure_keeps_cleanup_state_and_primary_error() {
        let mut builder = builder();
        let mut port = RecordingBodyPortV1::new(Some(3));
        let error = lower_prepared_raw_try_catch_with_port_v1(
            &mut builder,
            &mut port,
            prepared(1, &[2], Some(3)),
        )
        .unwrap_err();
        assert_eq!(error, "fail-3");
        assert_eq!(port.demands, vec![1, 2, 3]);
        assert!(builder.function_state.protected_region.cleanup.active);
        assert!(!builder
            .function_state
            .protected_region
            .return_defer
            .is_active());
    }

    #[test]
    fn cleanup_body_uses_the_policy_captured_before_lowering() {
        let policy = crate::test_support::with_env_vars(
            &[
                ("NYASH_CLEANUP_ALLOW_RETURN", Some("1")),
                ("NYASH_CLEANUP_ALLOW_THROW", Some("0")),
            ],
            CleanupExitPolicyV1::capture_from_environment,
        );
        let prepared =
            PreparedRawTryCatchV1::prepare(body(1), vec![catch(2)], Some(body(3)), policy);
        crate::test_support::with_env_vars(
            &[
                ("NYASH_CLEANUP_ALLOW_RETURN", Some("0")),
                ("NYASH_CLEANUP_ALLOW_THROW", Some("1")),
            ],
            || {
                let mut builder = builder();
                let mut port = RecordingBodyPortV1::new(None);
                lower_prepared_raw_try_catch_with_port_v1(&mut builder, &mut port, prepared)
                    .unwrap();
                assert_eq!(port.cleanup_policies, vec![(true, false)]);
            },
        );
    }
}
