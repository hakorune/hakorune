//! Source predicates selecting the existing homes-aware completion walk.
//! These borrow source claims; they issue no new type authority or ledger row.
use super::{
    terminal_home, BindingRefV1, OrdinaryNewCandidate, OrdinaryNewCoSealIssueV1, OwnedExprSiteV1,
    VerifiedInstanceConstructorSemanticBatchV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;

pub(super) fn has_print_expression_v1(input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
    input.body_shape().is_some_and(|shape| {
        shape.statements().iter().any(|statement| {
            matches!(
                statement,
                crate::mir::resolved_semantics::BodyStatementShapeV1::Print { .. }
            )
        })
    })
}

pub(super) fn has_condition_expression_v1(input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
    crate::mir::resolved_semantics::home_new_prefix::requires_condition_expression_walk_v1(input)
}

pub(super) fn has_nested_call_argument_walk_v1(input: ResolvedFunctionLoweringInputV1<'_>) -> bool {
    crate::mir::resolved_semantics::home_new_prefix::requires_nested_call_argument_walk_v1(input)
}

/// Candidate collection and the completion walk select the same original
/// declarations. An unselected nested-call source still needs its own input
/// inventory; this grants no callable execution role.
pub(super) fn needs_declaration_walk_v1(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    app_main: Option<u32>, slot: u32,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    if selected.role_for_batch_slot(slot).is_some() || app_main == Some(slot) { return Ok(true); }
    batch.with_lowering_input(slot, has_nested_call_argument_walk_v1)
        .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)
}

pub(super) fn has_map_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    has_map_formal: impl FnOnce() -> bool,
) -> bool {
    input.body_shape().is_some_and(|shape| {
        shape.expressions().iter().any(|row| {
            matches!(
                row,
                crate::mir::resolved_semantics::BodyExpressionShapeV1::MapLiteral { .. }
            )
        })
    }) || has_map_formal()
}

/// A me object-field initializer needs the verified walk's staged Alias
/// proof, which the plain seed / route-decision FieldGet cannot issue.
pub(super) fn has_me_object_field_read_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    instance_constructors: &VerifiedInstanceConstructorSemanticBatchV1,
    candidates: &[OrdinaryNewCandidate],
    coverage: &crate::parser::ParserOrdinaryBoxSourceCoverageV1,
    receiver_proof: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let owner = input.owner();
    let function = input.function();
    let mut found = false;
    if let Some((me, _)) = receiver_proof {
        if let Some(shape) = input.body_shape() {
            for initializer in function.expression_source().initializers() {
                let Some(site) = initializer.initializer_site() else {
                    continue;
                };
                let Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::FieldAccess {
                    object,
                    field,
                    ..
                }) = shape.expression_shape(site)
                else {
                    continue;
                };
                let receiver = match function.variable_ref(object) {
                    Some(crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(receiver)) => {
                        receiver
                    }
                    _ => match shape.expression_shape(object) {
                        Some(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                            receiver:
                                crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding),
                            ..
                        }) => *binding,
                        _ => continue,
                    },
                };
                if receiver != me {
                    continue;
                }
                let owned = OwnedExprSiteV1::new(owner, site.clone());
                match terminal_home::local_read_field(
                    instance_constructors,
                    candidates,
                    coverage,
                    receiver_proof,
                    &owned,
                    me,
                    None,
                    field,
                )? {
                    Some(source) if matches!(source.result(),
                        crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Alias(_)
                    ) => {
                        found = true;
                        break;
                    }
                    _ => continue,
                }
            }
        }
    }
    Ok(found)
}

/// An original opaque Static local continuation must retain its observation in
/// the canonical verified root flow. Only original initializer/fact membership
/// selects the walk; no new local classifier or completed ledger is consulted.
pub(super) fn has_static_source_local_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    source: &Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
) -> bool {
    let Ok(source) = source else {
        return false;
    };
    input
        .function()
        .expression_source()
        .initializers()
        .any(|initializer| {
            initializer.initializer_site().is_some_and(|site| {
                source.contains(&OwnedExprSiteV1::new(input.owner(), site.clone()))
            })
        })
}

/// A source-bound Dynamic result stored in a Local must pass through the
/// completion walk: that walk owns both its invocation actual and any
/// source-loop product selected from the same body. This predicate only
/// matches the already-admitted exact result site to its original Local
/// initializer; it issues no carrier or execution authority.
pub(super) fn has_dynamic_local_initializer_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
) -> bool {
    calls.iter().any(|call| {
        call.owner() == input.owner()
            && input.function().expression_source().initializers().any(|initializer| {
                initializer.initializer_site() == Some(call.result_site())
                    && matches!(
                        initializer.declaration_site(),
                        crate::mir::resolved_semantics::SourceBindingSiteV1::Local { .. }
                    )
            })
    })
}

/// Keep the existing plain root literal path separate from a selected Static
/// continuation's canonical root flow. This predicates source membership only.
pub(super) fn plain_main_integer_result_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    is_main: bool,
    no_sites: bool,
    no_map_or_owner_loan: bool,
    source: &Result<std::collections::BTreeSet<OwnedExprSiteV1>, String>,
) -> bool {
    is_main
        && no_sites
        && no_map_or_owner_loan
        && !has_static_source_local_v1(input, source)
        && !input.function().declaration_sites().any(|site| {
            matches!(
                site,
                crate::mir::resolved_semantics::SourceBindingSiteV1::Receiver
            )
        })
        && input
            .forest()
            .ordered_capture_demands(input.owner())
            .is_empty()
}

/// Original Instance input and borrowed LOCAL results require the same full walk.
/// No finished input or physical entry is used to choose its own prerequisites.
pub(super) fn has_original_borrowed_walk_v1(
    source: &Result<super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    catalog: &[super::super::super::model::OwnedCallableParameterContractDeclarationV1],
    targets: &super::super::lexical_instance_call::PreparedLexicalInstanceCallSourceTargetsV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> Result<bool, OrdinaryNewCoSealIssueV1> {
    let local = targets.as_ref().is_ok_and(|rows| {
        rows.iter()
            .filter_map(|row| row.as_ref().ok()?.as_ref())
            .any(|target| {
                target.call_site().owner() == owner
                    && target.borrowed_nullable_local_source().is_some()
            })
    });
    let Ok(source) = source else {
        return Ok(local);
    };
    // The original Unit return demand needs the same complete source Home
    // walk even for an otherwise empty body or a bare Return. Plain control
    // Completion alone does not supply an exit's cleanup obligations.
    let unit = source.requires_unit_return_walk_v1(owner);
    super::super::lexical_instance_call::original_borrowed_instance_input_completion_needed_v1(
        source, catalog, owner,
    )
    .map(|input| {
        input
            || local
            || unit
            || source.requires_named_array_holder_walk_v1(owner)
            || source.borrowed_formal_return_target_v1(owner)
    })
    .map_err(|(site, issue)| OrdinaryNewCoSealIssueV1::BorrowedFormalIngress { site, issue })
}

/// Original typed Static edges demand complete caller/callee source analysis.
/// This is preparation only: it grants no execution row or module participant.
pub(super) fn typed_static_walk_owners_v1(
    source: &Result<super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    main: Option<&super::super::lexical_instance_call::BorrowedAppMainSourceLoanV1<'_>>,
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
) -> Result<
    std::collections::BTreeSet<crate::mir::resolved_semantics::FunctionOwnerIdV1>,
    OrdinaryNewCoSealIssueV1,
> {
    let mut owners = std::collections::BTreeSet::new();
    let Ok(source) = source else {
        return Ok(owners);
    };
    for original in source.original_static_sources_v1() {
        let Ok(original) = original else { continue };
        if (!original.is_qualified_i64_source_v1() && !original.is_current_owner_i64_source_v1())
            || original.parameters().iter().any(|formal| formal.kind != crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(
                crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64)) { continue; }
        // This consumer owns real Local initializers in checked Static/Main
        // declarations. Terminal/condition/Instance/Birth sources keep theirs.
        let Some(caller) = batch
            .declarations()
            .find(|row| row.owner() == original.call_site().owner())
        else {
            continue;
        };
        if caller.mode() != crate::mir::callable_semantic_batch::ResolvedCallableDeclarationModeV1::StaticBoxMethod { continue; }
        let genuine_local = batch
            .with_lowering_input(caller.batch_slot(), |input| {
                input
                    .function()
                    .expression_source()
                    .initializers()
                    .any(|initializer| {
                        initializer.initializer_site() == Some(original.call_site().site())
                            && matches!(
                                initializer.declaration_site(),
                                crate::mir::resolved_semantics::SourceBindingSiteV1::Local { .. }
                            )
                    })
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
        if !genuine_local {
            continue;
        }
        let target =
            crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(original.target().clone());
        if selected.batch_slot(&target) != Some(original.target_batch_slot()) {
            continue;
        }
        let Some(callee) = batch
            .declarations()
            .find(|row| row.batch_slot() == original.target_batch_slot())
        else {
            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
        };
        if callee.owner() != original.callee_owner()
            || !selected
                .identity_for_batch_slot(callee.batch_slot())
                .is_some_and(|identity| identity.same_as(callee.identity()))
            || !original.catalog_brand().is_same(selected.catalog_brand())
        {
            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
        }
        let caller_key =
            crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(original.caller().clone());
        let selected_caller = selected.batch_slot(&caller_key) == Some(caller.batch_slot())
            && selected
                .identity_for_batch_slot(caller.batch_slot())
                .is_some_and(|identity| identity.same_as(caller.identity()));
        let checked_main = main.is_some_and(|main| {
            main.caller_source_ref_v1()
                .checked_caller_key(batch, contracts, caller.owner(), caller.batch_slot())
                .is_ok_and(|key| key == original.caller())
        });
        if selected_caller || checked_main {
            owners.extend([caller.owner(), callee.owner()]);
        }
    }
    Ok(owners)
}

/// One declaration walk, with original Text-result dependencies before callers.
/// The graph orders existing loans; it creates no Completion or result authority.
pub(super) fn text_completion_order_v1<'a>(
    source: &Result<super::super::lexical_instance_call::PreparedBorrowedFormalIngressV1, String>,
    results: &super::super::lexical_instance_call::BorrowedResultCohortV1,
    selected: &crate::mir::normal_callable_semantic_package::selected_mapping::VerifiedSelectedCallableBatchMapV1,
    main: Option<&super::super::lexical_instance_call::BorrowedAppMainSourceLoanV1<'_>>,
    batch: &'a crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    contracts: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
) -> Result<
    Vec<crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticDeclarationRefV1<'a>>,
    OrdinaryNewCoSealIssueV1,
> {
    let declarations: Vec<_> = batch.declarations().collect();
    let Ok(source) = source else {
        return Ok(declarations);
    };
    let mut dependencies = std::collections::BTreeMap::<u32, Vec<u32>>::new();
    for original in source.original_static_input_sources_v1() {
        let Some(caller) = declarations
            .iter()
            .find(|row| row.owner() == original.call_site().owner())
        else {
            continue;
        };
        let key =
            crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(original.target().clone());
        if selected.batch_slot(&key) != Some(original.target_batch_slot()) {
            continue;
        }
        let Some(callee) = declarations
            .iter()
            .find(|row| row.batch_slot() == original.target_batch_slot())
        else {
            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
        };
        if callee.owner() != original.callee_owner()
            || !selected
                .identity_for_batch_slot(callee.batch_slot())
                .is_some_and(|identity| identity.same_as(callee.identity()))
            || !original.catalog_brand().is_same(selected.catalog_brand())
        {
            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
        }
        let caller_key =
            crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(original.caller().clone());
        let selected_caller = selected.batch_slot(&caller_key) == Some(caller.batch_slot())
            && selected
                .identity_for_batch_slot(caller.batch_slot())
                .is_some_and(|identity| identity.same_as(caller.identity()));
        let checked_main = main.is_some_and(|main| {
            main.caller_source_ref_v1()
                .checked_caller_key(batch, contracts, caller.owner(), caller.batch_slot())
                .is_ok_and(|key| key == original.caller())
        });
        if !(selected_caller || checked_main) {
            continue;
        }
        let required = batch
            .with_lowering_input(callee.batch_slot(), |input| {
                source.requires_text_return_walk_v1(input, results)
            })
            .map_err(|_| OrdinaryNewCoSealIssueV1::BatchLoan)?;
        if required && caller.batch_slot() != callee.batch_slot() {
            let children = dependencies.entry(caller.batch_slot()).or_default();
            if !children.contains(&callee.batch_slot()) {
                children.push(callee.batch_slot());
            }
        }
    }
    for target in source.original_instance_source_targets_v1() {
        let Some(caller) = declarations.iter().find(|row| row.owner() == target.call_site().owner()) else { continue; };
        let key = crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(target.target().clone());
        if selected.batch_slot(&key) != Some(target.target_batch_slot()) { continue; }
        let selected_caller = selected.key_for_batch_slot(caller.batch_slot()).is_some_and(|key|
            selected.batch_slot(&key) == Some(caller.batch_slot())
                && selected.identity_for_batch_slot(caller.batch_slot()).is_some_and(|identity| identity.same_as(caller.identity())));
        let checked_main = main.is_some_and(|main| main.caller_source_ref_v1()
            .checked_caller_key(batch, contracts, caller.owner(), caller.batch_slot()).is_ok());
        if !(selected_caller || checked_main) { continue; }
        let Some(callee) = declarations.iter().find(|row| row.batch_slot() == target.target_batch_slot()) else {
            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
        };
        if callee.owner() != target.callee_owner()
            || !selected.identity_for_batch_slot(callee.batch_slot()).is_some_and(|identity| identity.same_as(callee.identity())) {
            return Err(OrdinaryNewCoSealIssueV1::BatchLoan);
        }
        // Optional ordering information is never an admission or a new veto.
        if !results.original_return_source_v1(&callee.owner()).is_ok_and(|row|
            row.is_some_and(|row| row.is_text_return_v1(source))) { continue; }
        if caller.batch_slot() != callee.batch_slot() {
            let children = dependencies.entry(caller.batch_slot()).or_default();
            if !children.contains(&callee.batch_slot()) { children.push(callee.batch_slot()); }
        }
    }
    fn visit(
        slot: u32,
        dependencies: &std::collections::BTreeMap<u32, Vec<u32>>,
        visited: &mut std::collections::BTreeSet<u32>,
        order: &mut Vec<u32>,
    ) {
        // Visiting a cycle grants nothing: its unfinished callee still has no
        // result row. Existing source/result refusals remain responsible for it.
        if !visited.insert(slot) {
            return;
        }
        if let Some(children) = dependencies.get(&slot) {
            for &child in children {
                visit(child, dependencies, visited, order);
            }
        }
        order.push(slot);
    }
    let mut visited = std::collections::BTreeSet::new();
    let mut order = Vec::with_capacity(declarations.len());
    for declaration in &declarations {
        visit(
            declaration.batch_slot(),
            &dependencies,
            &mut visited,
            &mut order,
        );
    }
    order
        .into_iter()
        .map(|slot| {
            declarations
                .iter()
                .find(|row| row.batch_slot() == slot)
                .copied()
                .ok_or(OrdinaryNewCoSealIssueV1::BatchLoan)
        })
        .collect()
}
