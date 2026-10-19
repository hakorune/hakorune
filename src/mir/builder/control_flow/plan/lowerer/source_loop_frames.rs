//! Execution of original scheduled frames borrows the invocation's sole writers.
//! The whole-Loop driver still owns topology, joins, cleanup and completion.
use crate::mir::builder::condition_materialization::ConditionMaterializationCaptureV1;
use crate::mir::builder::raw_invocation_source_transport::RawInvocationSourceContextV1;
use crate::mir::builder::raw_structured_child_scope::PreparedRawChildSourceV1;
use crate::mir::builder::recursive_child_lowering::{
    RawInvocationChildPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::builder::{ConditionMaterializationV1, MirBuilder};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::loop_recipe_contract::VerifiedSourceLoopRecipeProductV2 as Product;
use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1 as Prefix;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, SourceNodeSiteV1};
use crate::mir::{BasicBlockId, ValueId};
use std::rc::Rc;

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
        source: holder, call, candidate, ..
    } = observation else { return Err(freeze("array-append-purpose")); };
    if call != &site { return Err(freeze("array-append-source-drift")); }
    let original = input.function().method_calls().find(|(original, _)| *original == site.site())
        .map(|(_, call)| call).ok_or_else(|| freeze("array-append-original-missing"))?;
    let source = context(port, statement.node())?;
    port.with_prepared_child_source_v1(source, |port| {
        let packet = port.emit_owned_array_append_v1(builder, original.selector(), original.arity() as usize)?
            .ok_or_else(|| freeze("array-append-source-missing"))?;
        if !packet.source().matches_original_v1(holder, candidate) {
            return Err(freeze("array-append-source-drift"));
        }
        port.ordinary_new_claim_ledger.as_ref().ok_or_else(|| freeze("claim-ledger-missing"))?
            .source_loop_effect_group_v2(product, &site)
    })
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
            let source = context(port, statement.node())?;
            let completed = port.with_prepared_child_source_v1(source, |port| {
                port.lower_callable_local_with_receipt_v2(builder, original.node().clone())
            })?;
            // The sole Local owner already installed state and consumed/published
            // its checked lease. Borrow its real ordinal, not a synthetic receipt.
            let receipt = completed
                .bindings()
                .iter()
                .find(|row| row.ordinal() == *ordinal)
                .ok_or_else(|| freeze("initializer-completed-ordinal"))?;
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
            let source = context(port, statement.node())?;
            port.with_prepared_child_source_v1(source, |port| {
                port.lower_callable_binding_rebind_v1(builder, original.node().clone())
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
