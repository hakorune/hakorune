//! Execution of original scheduled frames borrows the invocation's sole writers.
//! The whole-Loop driver still owns topology, joins, cleanup and completion.
use crate::mir::builder::condition_materialization::ConditionMaterializationCaptureV1;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1;
use crate::mir::builder::recursive_child_lowering::{
    RawInvocationChildPortV1, RecursiveChildLoweringPortV1, DirectCallDispositionPortV1,
    RawOrdinaryNewClaimPortV1,
};
use crate::mir::builder::{ConditionMaterializationV1, MirBuilder};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2 as Product;
use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1 as Prefix;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, ValueId};
use std::rc::Rc;

/// Reuse the exact lexical read and the sole root FunctionExit writer. The
/// Loop retains observations only; it never substitutes Loop-port cleanup.
pub(super) fn emit_return(
    builder: &mut MirBuilder, port: &mut RawInvocationChildPortV1<'_, '_>, product: &Rc<Product>,
    source: &Rc<crate::mir::resolved_semantics::home_new_prefix::SourceLoopReturnV1>,
) -> Result<(ValueId, Option<crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1>,
    Vec<(BasicBlockId, crate::mir::MirInstruction)>), String> {
    use crate::mir::normal_callable_semantic_package::{ExpressionReferenceReadV1, CallValidationContextV1};
    let input = input(port, product)?;
    if !source.corroborates(input) || !product.schedule().returns_v2().any(|row| Rc::ptr_eq(row, source)) {
        return Err(freeze("call-source-drift"));
    }
    crate::mir::builder::control_flow::cleanup::ensure_cleanup_exit_allowed_v1(
        &builder.function_state, crate::mir::builder::control_flow::cleanup::CleanupExitKindV1::Return)?;
    let claims = port.ordinary_new_claim_ledger.clone().ok_or_else(|| freeze("claim-ledger-missing"))?;
    let mut exit_prepared = false;
    let (value, read) = if let Some(source_read) = &source.read {
        let state = port.callable_ledger.clone().ok_or_else(|| freeze("state-missing"))?;
        let mut state = state.borrow_mut();
        let prepared = state.prepare_exact_lexical_read(input.owner(), source_read.read().node(), source_read.binding())
            .map_err(|error| error.to_string())?;
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
        let producer = ExpressionReferenceReadV1::prepare_scalar_read_v2(&source.source, source_read,
            prepared.observation_v1(), CallValidationContextV1::Lowering(&claims), function)?;
        let read = state.commit_exact_lexical_reads(vec![prepared]).map_err(|error| error.to_string())?
            .pop().ok_or_else(|| freeze("call-source-missing"))?;
        let observed = producer.finish(read);
        let value = observed.scalar_read_value_v2(&source.source, source_read,
            CallValidationContextV1::Lowering(&claims), function)?;
        (value, Some(observed))
    } else if let (true, crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Value(terminal)) =
        (source.is_null_literal_v1(), &source.terminal) {
        // The Null producer is recorded on the prepared exit order itself.
        if !claims.prepare_root_home_exit(input.owner(), source.site.node())? { return Err(freeze("call-source-missing")); }
        exit_prepared = true;
        let at = OwnedExprSiteV1::new(input.owner(), terminal.value_site().clone());
        let loan = claims.prepare_terminal_null_literal_v1(at.clone())?
            .ok_or_else(|| freeze("literal-return-source"))?;
        let completed = crate::mir::builder::emission::constant::emit_null_recorded(builder)?;
        claims.complete_terminal_null_literal_v1(loan, at, &completed)?;
        (completed.value(), None)
    } else {
        let expected = claims.prepare_terminal_scalar_literal_return(input.owner(), source.site.node())?
            .ok_or_else(|| freeze("literal-return-source"))?;
        let crate::mir::ConstValue::Integer(integer) = expected else {
            return Err(freeze("literal-return-class"));
        };
        let value = crate::mir::builder::emission::constant::emit_integer(builder, integer)?;
        claims.record_terminal_scalar_literal_return(input.owner(), source.site.node(), value)?;
        let block = builder.function_state.current_block.ok_or_else(|| freeze("literal-return-block"))?;
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
        let instruction = function.blocks.get(&block)
            .and_then(|block| block.instructions.last())
            .ok_or_else(|| freeze("literal-return-producer"))?.clone();
        claims.record_terminal_literal_producer_v1(
            input.owner(), &source.site, (block, instruction), function,
        )?;
        (value, None)
    };
    if !exit_prepared && !claims.prepare_root_home_exit(input.owner(), source.site.node())? { return Err(freeze("call-source-missing")); }
    let state = port.callable_ledger.clone().ok_or_else(|| freeze("state-missing"))?;
    let mut state = state.borrow_mut();
    // Text carriers not owned as Homes (Header/branch/transferred Residence) still owe
    // their End here; the Ends join the exit order before it is emitted.
    let ends = text_carrier_ends(&claims, &state, product.return_text_carriers_v2(source)?,
        &source.homes.homes().iter().map(|home| home.binding()).collect::<Vec<_>>())?;
    let prior = product.source.condition.prior_homes().iter().map(|home| home.binding()).collect();
    claims.attach_loop_return_text_carrier_ends_v2(input.owner(), &source.site, prior, ends)?;
    crate::mir::builder::ordinary_new_admission::selected::emit_root_home_exit(
        builder, &mut state, &claims, input.owner(), &source.site, value)?;
    Ok((value, read, claims.source_loop_return_bindings_v2(product, source)?))
}

pub(super) fn emit_array_append(
    builder: &mut MirBuilder, port: &mut RawInvocationChildPortV1<'_, '_>, product: &Rc<Product>,
    statement: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    observation: &crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1,
) -> Result<crate::mir::normal_callable_semantic_package::RootLifecyclePrefixBindingGroupV1, String> {
    let input = input(port, product)?;
    let site = OwnedExprSiteV1::new(input.owner(),
        crate::mir::resolved_semantics::SourceExprSiteV1::from_node(statement.node().clone()));
    if product.owned_array_append_v2(&site) != Some(observation) {
        return Err(freeze("array-append-source-drift"));
    }
    let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
        source: holder, call, candidate, prior_homes, ..
    } = observation else { return Err(freeze("array-append-purpose")); };
    if call != &site { return Err(freeze("array-append-source-drift")); }
    let original = input.function().method_calls().find(|(original, _)| *original == site.site())
        .map(|(_, call)| call).ok_or_else(|| freeze("array-append-original-missing"))?;
    let (argument_frame, nested_argument_frames) = if holder.is_dynamic_carrier_append_only_v1() {
        let frames = product
            .source
            .body_expressions
            .iter()
            .filter(|plan| plan.source().root().statement() == statement)
            .filter_map(|plan| plan.argument_frame())
            .cloned()
            .collect::<Vec<_>>();
        let source_ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|_| freeze("array-append/source-ledger"))?;
        let mut append_frames = Vec::new();
        for frame in &frames {
            let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::CallArgument {
                call: argument_call,
                ordinal,
                ..
            } = frame.source().root() else {
                return Err(freeze("array-append/dynamic-argument-purpose"));
            };
            let argument_matches = argument_call
                .arguments()
                .get(*ordinal as usize)
                .is_some_and(|argument| {
                    argument.ordinal() == *ordinal
                        && frame
                            .source()
                            .nodes()
                            .first()
                            .is_some_and(|root| root.site() == argument.site())
                });
            let source_identity = source_ledger
                .method_call_source_v1(argument_call.site())
                .is_some_and(|original| std::sync::Arc::ptr_eq(original, argument_call));
            let append_frame = argument_call.owner() == call.owner()
                && argument_call.site() == call.site()
                && *ordinal == 0
                && argument_call
                    .arguments()
                    .first()
                    .is_some_and(|argument| argument.site() == candidate.site())
                && frame.is_dynamic_carrier_append_v1()
                && frame.source_loan().dynamic_array_carrier_append_v1() == Some(holder);
            let candidate_frame = argument_call.owner() == call.owner()
                && argument_call.site() == candidate.site();
            if !source_identity
                || !argument_matches
                || !(append_frame || candidate_frame)
                || !std::rc::Rc::ptr_eq(frame.source(), frame.source_loan().plan())
                || !frame.evaluation().reference_coverage_is_exact()
                || frame.prior_homes() != prior_homes.as_ref()
            {
                return Err(freeze("array-append/dynamic-argument-frame-drift"));
            }
            if append_frame {
                append_frames.push(std::rc::Rc::clone(frame));
            }
        }
        if append_frames.len() != 1 {
            return Err(freeze("array-append/dynamic-argument-frame-missing"));
        }
        (append_frames.pop(), frames)
    } else {
        (None, Vec::new())
    };
    let source = context(port, statement.node())?;
    port.with_prepared_child_source_v1(source, |port| {
        let packet = port.emit_owned_array_append_v1(
            builder,
            original.selector(),
            original.arity() as usize,
            argument_frame,
            &nested_argument_frames,
        )?
            .ok_or_else(|| freeze("array-append-source-missing"))?;
        if !packet.source().matches_original_v1(holder, candidate) {
            return Err(freeze("array-append-source-drift"));
        }
        port.ordinary_new_claim_ledger.as_ref().ok_or_else(|| freeze("claim-ledger-missing"))?
            .source_loop_effect_group_v2(product, &site)
    })
}

/// Delegate the original Local statement or discarded Call to its sole writer.
/// The schedule's CallSlot is a slot, never an empty receiver/argument list.
/// The End each Text carrier still owes at one Fault/exit point: conditionally
/// owned Headers and transferred Residences have no Home of their own.
fn text_carrier_ends(
    claims: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
    state: &crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState,
    carriers: Vec<(crate::mir::resolved_semantics::BindingRefV1, crate::mir::resolved_semantics::home_new_prefix::SourceLoopTextProducerV1)>,
    homes: &[crate::mir::resolved_semantics::BindingRefV1],
) -> Result<Vec<(crate::mir::resolved_semantics::BindingRefV1, crate::mir::instruction::InvokeOperation)>, String> {
    use crate::mir::resolved_semantics::home_new_prefix::{LiveHomeV1, SourceLoopTextProducerV1 as Producer, SourceLoopValueProducerV1 as ValueProducer};
    use crate::mir::instruction::{InvokeOperation, StringInvokeOperation as Op};
    let mut ends = Vec::new();
    for (binding, producer) in carriers {
        let read = state.observe_binding_value_v2(binding).map_err(|error| error.to_string())?;
        let end = match &producer {
            Producer::Static { .. } => continue,
            Producer::Header(basis) => match basis.entry_producer() {
                ValueProducer::Text(Producer::Static { .. }) =>
                    InvokeOperation::String(Op::EndIfOwned { value: read.loan_text_join_v1(&producer)?.value() }),
                _ => continue,
            },
            Producer::Residence(acquisition) if !homes.contains(&binding) => {
                let operations = claims.prior_home_unwind_for(&[LiveHomeV1::Text(acquisition.clone())])?;
                let [end] = operations.as_slice() else { return Err(freeze("loop-text-carrier/end")); };
                end.clone()
            }
            Producer::Residence(_) => continue,
            Producer::BranchNormal(_) =>
                InvokeOperation::String(Op::EndIfOwned { value: read.loan_text_branch_v1(&producer)?.relation().values().0 }),
            _ => return Err(freeze("loop-text-carrier/producer")),
        };
        ends.push((binding, end));
    }
    Ok(ends)
}

pub(super) fn emit_call(
    builder: &mut MirBuilder, port: &mut RawInvocationChildPortV1<'_, '_>, product: &Rc<Product>,
    statement: &crate::mir::resolved_semantics::SourceStmtSiteV1,
    observation: &crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
) -> Result<(crate::mir::normal_callable_semantic_package::RootLifecyclePrefixBindingGroupV1,
    Option<(ValueId, ValueId)>), String> {
    use crate::mir::resolved_semantics::{SourceBindingSiteV1, home_new_prefix::LocalCallResultClassV1 as Class};
    let input = input(port, product)?;
    if !product.schedule().calls_v2().any(|step| matches!(step,
        crate::mir::loop_recipe_contract::SourceLoopScheduledStepV2::Call { site, observation: original, .. }
            if site == statement && original == observation)) {
        return Err(freeze("call-source-drift"));
    }
    let claims = port.ordinary_new_claim_ledger.clone().ok_or_else(|| freeze("claim-ledger-missing"))?;
    {
        // The Call's Fault unwind also owes the live Text carriers' Ends.
        let state = port.callable_ledger.clone().ok_or_else(|| freeze("state-missing"))?;
        let state = state.borrow();
        let ends = text_carrier_ends(&claims, &state, product.call_text_carriers_v2(observation.site())?,
            &observation.prior_homes().iter().map(|home| home.binding()).collect::<Vec<_>>())?;
        claims.record_loop_call_text_carrier_ends_v2(observation.site().clone(),
            product.source.condition.prior_homes().len(), ends.into_iter().map(|(_, end)| end).collect())?;
    }
        let value = if let Some((declaration, binding)) = observation.local_binding() {
        let SourceBindingSiteV1::Local { statement: original_statement, ordinal } = declaration
            else { return Err(freeze("call-local-declaration")); };
        if original_statement != statement || input.function().declaration_binding(declaration) != Some(binding) {
            return Err(freeze("call-local-declaration"));
        }
        let original = input.source().exact_stmt(statement).map_err(|error| format!("{error:?}"))?;
        let source = context(port, statement.node())?;
        let completed = port.with_prepared_child_source_v1(source, |port|
            port.lower_callable_local_with_receipt_v2(builder, original.node().clone()))?;
        let receipt = completed.bindings().iter().find(|row| row.ordinal() == *ordinal)
            .ok_or_else(|| freeze("call-completed-local"))?;
        if receipt.initializer() != receipt.local() {
            let (block, _) = receipt.copy().ok_or_else(|| freeze("call-completed-local"))?;
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
            builder.function_state.local_ssa_correspondence.record_pin_copy(
                function, receipt.initializer(), receipt.local(), *block)?;
        }
        Some((receipt.initializer(), receipt.local()))
        } else {
        let original = input.function().method_calls().find(|(site, _)| *site == observation.site().site())
            .map(|(_, call)| call).ok_or_else(|| freeze("call-original-missing"))?;
        let source = context(port, observation.site().site().node())?;
        port.with_prepared_child_source_v1(source, |port| {
            if observation.result() == Class::Unit {
                    if observation.is_expression_value() {
                        return Err(freeze("call-unit-expression-result"));
                    }
                if !port.try_emit_unit_discard(builder, original.selector(), original.arity() as usize)? {
                    return Err(freeze("call-source-missing"));
                }
                    Ok(None)
            } else if let Some(selected) = claims.selected_static_local_source_v1(observation.site())? {
                    let value = port.emit_local_static_lifecycle_call_v1(builder, selected.target().owner(),
                        original.selector(), original.arity() as usize)?.ok_or_else(|| freeze("call-source-missing"))?;
                    Ok(observation.is_expression_value().then_some((value, value)))
                } else {
                    let value = port.emit_local_lexical_lifecycle_call_v1(builder, original.selector())?
                        .ok_or_else(|| freeze("call-source-missing"))?;
                    Ok(observation.is_expression_value().then_some((value, value)))
                }
            })?
    };
    let packet = claims.source_loop_effect_group_v2(product, observation.site())?;
    Ok((packet, value))
}

/// Validate the existing invocation and immutable product BEFORE opening a
/// narrower source scope. No module port, scope or child evaluator is rebuilt.
pub(super) fn input<'source>(
    port: &RawInvocationChildPortV1<'source, '_>,
    product: &Rc<Product>,
) -> Result<ResolvedFunctionLoweringInputV1<'source>, String> {
    let input = port
        .source_input
        .ok_or_else(|| freeze("source-input-missing"))?;
    let claims = port
        .ordinary_new_claim_ledger
        .as_ref()
        .ok_or_else(|| freeze("claim-ledger-missing"))?;
    let state = port
        .callable_ledger
        .as_ref()
        .ok_or_else(|| freeze("state-missing"))?;
    if port.callable_loop_root_scope.is_none()
        || input.owner() != product.source.source.owner()
        || state.borrow().owner() != input.owner()
        || state
            .borrow()
            .ordinary_new_claim_ledger()
            .is_none_or(|actual| !Rc::ptr_eq(actual, claims))
        || port.active_source.as_ref().and_then(|source| source.site())
            != Some(product.source.source.site().node())
        || claims
            .retained_source_loop_recipe_v2(input.owner(), product.source.source.site())?
            .is_none_or(|original| !Rc::ptr_eq(original, product))
    {
        return Err(freeze("invocation-source-drift"));
    }
    Ok(input)
}

/// Reopen only a checked frame's original statement/expression. The source view
/// separately validates owner inventory and projects the exact syntax node.
fn context(
    port: &RawInvocationChildPortV1<'_, '_>,
    site: &SourceNodeSiteV1,
) -> Result<PreparedRawChildSourceV1, String> {
    let root = port
        .active_source
        .as_ref()
        .and_then(|source| source.root_lineage())
        .ok_or_else(|| freeze("root-lineage-missing"))?;
    Ok(PreparedRawChildSourceV1::Exact(
        RawInvocationSourceContextV1::Located {
            root: root.clone(),
            site: site.clone(),
            body_kind: None,
        },
    ))
}

pub(super) fn prepare_if_observer(
    builder: &MirBuilder, port: &mut RawInvocationChildPortV1<'_, '_>,
    product: &Rc<Product>, condition: usize,
) -> Result<Option<Box<dyn crate::mir::builder::if_form::IfFormPhysicalObserverV1>>, String> {
    input(port, product)?;
    let frame = product.schedule().frames().get(condition).ok_or_else(|| freeze("frame-index"))?;
    let source = context(port, frame.prefix().statement().node())?;
    port.with_prepared_child_source_v1(source, |port| port.prepare_if_state_observer_v1(builder))
}

pub(super) fn emit_binding(
    builder: &mut MirBuilder,
    port: &mut RawInvocationChildPortV1<'_, '_>,
    product: &Rc<Product>,
    index: usize,
) -> Result<ValueId, String> {
    let input = input(port, product)?;
    let scheduled = product
        .schedule()
        .frames()
        .get(index)
        .ok_or_else(|| freeze("frame-index"))?;
    let claims = port
        .ordinary_new_claim_ledger
        .clone()
        .ok_or_else(|| freeze("claim-ledger-missing"))?;
    let statement = scheduled.prefix().statement();
    let original = input
        .source()
        .exact_stmt(statement)
        .map_err(|error| format!("{error:?}"))?;
    match scheduled.prefix() {
        Prefix::InitializerExpression { frame, .. } => {
            let selected = claims
                .initializer_expression_frame_v1(input.owner(), statement.node())?
                .ok_or_else(|| freeze("initializer-source-missing"))?;
            if !Rc::ptr_eq(frame, &selected) {
                return Err(freeze("initializer-source-drift"));
            }
            claims.validate_initializer_expression_source_v1(frame)?;
            let relation = frame
                .source()
                .root()
                .initializer()
                .ok_or_else(|| freeze("initializer-relation"))?;
            let crate::mir::resolved_semantics::SourceBindingSiteV1::Local {
                statement: declaration,
                ordinal,
            } = relation.declaration_site()
            else {
                return Err(freeze("initializer-declaration"));
            };
            if declaration != statement || relation.binding() != frame.binding() {
                return Err(freeze("initializer-declaration"));
            }
            let completed = port.with_source_loop_product_v2(Rc::clone(product), |port| {
                let source = context(port, statement.node())?;
                port.with_prepared_child_source_v1(source, |port| {
                    port.lower_callable_local_with_receipt_v2(builder, original.node().clone())
                })
            })?;
            // The sole Local owner already installed state and consumed/published
            // its checked lease. Borrow its real ordinal, not a synthetic receipt.
            let receipt = completed
                .bindings()
                .iter()
                .find(|row| row.ordinal() == *ordinal)
                .ok_or_else(|| freeze("initializer-completed-ordinal"))?;
            if receipt.initializer() != receipt.local() {
                let (block, _) = receipt.copy().ok_or_else(|| freeze("initializer-completed-local"))?;
                let function = builder.function_state.current_function.as_ref().ok_or_else(|| freeze("no-function"))?;
                builder.function_state.local_ssa_correspondence.record_pin_copy(
                    function, receipt.initializer(), receipt.local(), *block)?;
            }
            Ok(receipt.initializer())
        }
        Prefix::BindingRebind { frame, .. } => {
            let selected = claims
                .rebind_expression_frame_v1(input.owner(), statement.node())?
                .ok_or_else(|| freeze("rebind-source-missing"))?;
            if !Rc::ptr_eq(frame, &selected) {
                return Err(freeze("rebind-source-drift"));
            }
            claims.validate_rebind_expression_source_v1(frame)?;
            // Rebind consumes this same Loop's lifecycle transition. Keep the
            // original product in scope just as for initializer descent.
            port.with_source_loop_product_v2(Rc::clone(product), |port| {
                let source = context(port, statement.node())?;
                port.with_prepared_child_source_v1(source, |port| {
                    port.lower_callable_binding_rebind_v1(builder, original.node().clone())
                })
            })
        }
        _ => Err(freeze("binding-frame-purpose")),
    }
}

pub(super) fn emit_condition(
    builder: &mut MirBuilder,
    port: &mut RawInvocationChildPortV1<'_, '_>,
    product: &Rc<Product>,
    index: usize,
    targets: (BasicBlockId, BasicBlockId),
) -> Result<Rc<ConditionMaterializationV1>, String> {
    let input = input(port, product)?;
    let scheduled = product
        .schedule()
        .frames()
        .get(index)
        .ok_or_else(|| freeze("frame-index"))?;
    let Prefix::BoolCondition { frame, .. } = scheduled.prefix() else {
        return Err(freeze("condition-frame-purpose"));
    };
    if targets.0 == targets.1 {
        return Err(freeze("condition-targets"));
    }
    let claims = port
        .ordinary_new_claim_ledger
        .clone()
        .ok_or_else(|| freeze("claim-ledger-missing"))?;
    crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(&claims)
        .validate_source_loop_bool_frame_v2(product, index, frame)?;
    claims.validate_bool_condition_source_v1(frame)?;
    let site = OwnedExprSiteV1::new(input.owner(), frame.source().nodes()[0].site().clone());
    let original = input
        .source()
        .expr_at(&site)
        .map_err(|error| format!("{error:?}"))?;
    let source = context(port, site.site().node())?;
    let mut lowered = port.with_prepared_child_source_v1(source, |port| {
        port.lower_checked_bool_condition_v1(builder, Rc::clone(frame), original.node().clone())
    })?;
    let capture = ConditionMaterializationCaptureV1::begin(builder, lowered.value())?;
    let mut value = crate::mir::builder::ssa::local::try_cond(builder, lowered.value())?;
    crate::mir::builder::ssa::local::finalize_branch_cond(builder, &mut value)?;
    let from = builder
        .function_state
        .current_block
        .ok_or_else(|| freeze("condition-block"))?;
    let materialization = capture.finish(builder, from, value)?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("function-missing"))?;
    lowered.prepare_source_loop_branch_v2(
        function,
        Rc::clone(product),
        index,
        Rc::clone(&materialization),
        targets,
    )?;
    crate::mir::builder::emission::branch::emit_carriers_only_conditional_v2(
        builder, from, value, targets,
    )?;
    let function = builder
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| freeze("function-missing"))?;
    let branch = function
        .blocks
        .get(&from)
        .and_then(|body| body.terminator.clone())
        .ok_or_else(|| freeze("condition-branch-missing"))?;
    lowered.seal_source_loop_branch_v2(function, &materialization, (from, branch))?;
    Ok(materialization)
}

fn freeze(reason: &str) -> String {
    format!("[freeze:contract][source-loop/frame-writer/{reason}]")
}
