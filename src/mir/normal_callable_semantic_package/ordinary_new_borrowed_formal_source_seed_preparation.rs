//! Prepare original formal actuals before result and receiver eligibility.
use super::*;

/// Actual sites are classified once, before result/receiver eligibility.
/// PendingStored retains its complete original ordered input without a loan.
#[allow(clippy::too_many_arguments)]
pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call::borrowed_formal_source) fn prepare_original_formal_seeds_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    selected: &VerifiedSelectedCallableBatchMapV1,
    entry_home_loans: &crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1,
    constructors: &crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1,
    results: &super::super::super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    candidates: &BTreeMap<
        u32,
        Result<
            Vec<super::super::super::super::candidate::OrdinaryNewCandidate>,
            super::super::super::super::OrdinaryNewCoSealIssueV1,
        >,
    >,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    inventory: &BorrowedIncomingInventoryV1,
    guarded: &BTreeMap<(OwnedExprSiteV1, u32), BorrowedGuardedActualV1>,
    static_claims: Option<&crate::mir::normal_callable_semantic_package::qualified_static_call_claim::QualifiedStaticCallClaimIndexV1>,
    array_i64_fields: &BTreeMap<Box<str>, BTreeSet<hakorune_mir_defs::CanonicalFieldRefV1>>,
    app_main: Option<AppMainSourceRefV1<'_>>,
    omitted: &BTreeMap<FunctionOwnerIdV1, CanonicalSameModuleCallableKeyV1>,
    fields: Option<&crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_write_claim::OrdinaryNewFieldWriteClaimsV1>,
) -> Result<OriginalFormalSeedProductV1, String> {
    // Borrow a checked source Array read before selecting Text operand premises.
    // No SUM/Completion lookup, second inventory or target is created here.
    let field_array_access = |input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        method: &std::sync::Arc<crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1>| {
        let Some(fields) = fields else { return Ok(None); };
        let mut declarations = batch.declarations().filter(|row| row.owner() == input.owner());
        let Some(declaration) = declarations.next() else { return Ok(None); };
        if declarations.next().is_some() { return Err(freeze("array-read/caller-source-drift")); }
        let Some(crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(caller)) = selected.key_for_batch_slot(declaration.batch_slot())
            else { return Ok(None); };
        // An existing exact Integer Element source keeps its own result
        // owner; a generic Dynamic receipt must not supersede that source.
        if crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_array_read_source::capture_exact_get(
            input, caller, method.site(), selected, batch, constructors, array_i64_fields)?.is_some() { return Ok(None); }
        crate::mir::normal_callable_semantic_package::ordinary_new_coseal::field_array_read_source::capture_exact_read(
            input, caller, method.site(), selected, batch, constructors, fields)
    };
    let mut rows = Vec::new();
    let mut seen = BTreeSet::new();
    for (call, legacy) in inventory.source_rows_with_legacy_v1() {
        if !seen.insert(call.call.clone()) {
            return Err(freeze("borrowed-view/duplicate-original-call"));
        }
        let caller = call.call.owner();
        let seeds = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, caller, |context, input| -> Result<Vec<_>, String> {
                let original = input.function().method_call(call.call.site())
                    .ok_or_else(|| freeze("borrowed-view/original-call"))?;
                context.constructor_key_for_call(&call.call, original)?;
                if original.selector() != call.source.target().name()
                    || original.arity() != call.source.target().arity()
                    || original.arguments().iter().map(|argument| argument.site()).ne(call.source.argument_sites().iter())
                { return Err(freeze("borrowed-view/original-call-identity")); }
                let source_context = OriginalActualSourceContextV1::checked(context, input, batch, selected, contracts,
                    static_claims, app_main, omitted)?.map(|context| context.with_field_array_accesses_v1(&field_array_access)).map(|context| context.for_point_premise());
                let slot = context.declaration_slot();
                let candidates = slot.and_then(|slot| candidates.get(&slot)).and_then(|rows| rows.as_ref().ok())
                    .map(Vec::as_slice).unwrap_or(&[]);
                let receiver = slot.and_then(|slot| contracts.iter().find(|row| row.owner == caller)
                    .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
                    .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                        selected, batch, entry_home_loans.for_batch_slot(slot), slot)));
                call.arguments.iter().map(|(ordinal, site, _)| {
                    if let Some(source) = classify_returned_field_seed_v1(
                        results, batch, selected, constructors, inventory,
                        &OwnedExprSiteV1::new(caller, site.clone()),
                    )? { return Ok(source); }
                    if slot.is_none() {
                        if let Some(origin) = new_seeds::stable_constructor_formal_v1(input, site, constructors) {
                            return Ok(OriginalFormalActualSeedV1::Forward { origin, declared_constraint: None });
                        }
                    }
                    Ok(classify_original_actual_seed_in_scope(input, batch, selected, results, caller,
                        candidates, receiver, definitions, contracts, site,
                        guarded.get(&(call.call.clone(), *ordinal)).map(|fact| (fact, &call.call, *ordinal)), Some(inventory), source_context.as_ref()))
                }).collect::<Result<Vec<_>, String>>()
            })??;
        rows.push(OriginalCallSeedsV1 {
            incoming: call.clone(),
            legacy,
            seeds: seeds.into_boxed_slice(),
        });
    }
    let mut new_calls = new_seeds::OriginalNewSeedsV1::default();
    for declaration in batch.declarations() {
        let slot = declaration.batch_slot();
        let root_owner = declaration.owner();
        let local_candidates = candidates
            .get(&slot)
            .and_then(|rows| rows.as_ref().ok())
            .map(Vec::as_slice)
            .unwrap_or(&[]);
        let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
            selected, batch, entry_home_loans.for_batch_slot(slot), slot,
        );
        let observed = batch.with_lowering_input(slot, |root| {
            for (owner, _) in root.forest().semantic_owners() {
                match batch.with_owner_lowering_input(slot, owner, |input| {
                    let source_context = if owner == root_owner {
                        OriginalActualSourceContextV1::checked(
                            crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1::Declaration { batch_slot: slot },
                            input, batch, selected, contracts, static_claims, app_main, omitted)?.map(|context| context.with_field_array_accesses_v1(&field_array_access))
                    } else { None };
                    new_calls.observe(input, batch, constructors, static_claims, selected, array_i64_fields, |site| {
                        classify_original_actual_seed_in_scope(input, batch, selected, results, owner,
                            if owner == root_owner { local_candidates } else { &[] },
                            if owner == root_owner { receiver } else { None },
                            definitions, contracts, site, None, Some(inventory), source_context.as_ref())
                    })
                }) {
                    Ok(result) => result?,
                    Err(_) => new_calls.missing_owner(),
                }
            }
            Ok::<(), String>(())
        });
        match observed {
            Ok(result) => result?,
            Err(_) => new_calls.missing_owner(),
        }
    }
    let observed = batch.with_normal_program_source_loan(|loan| {
        for constructor in constructors.rows() {
            for (owner, _) in constructor.forest().semantic_owners() {
                match constructor.lowering_input_for_owner(loan.program(), owner) {
                    Ok(input) => {
                    let source_context = if constructor.kind() == crate::parser::ConstructorSourceKindV1::Birth
                        && constructor.forest().roots() == [owner] {
                        OriginalActualSourceContextV1::checked(
                            crate::mir::normal_callable_semantic_package::original_call_source::OriginalCallSourceCallerV1::Constructor { row: constructor, input },
                            input, batch, selected, contracts, static_claims, app_main, omitted)?.map(|context| context.with_field_array_accesses_v1(&field_array_access))
                    } else { None };
                    new_calls.observe(input, batch, constructors, static_claims, selected, array_i64_fields, |site| {
                        if let Some(origin) = new_seeds::stable_constructor_formal_v1(input, site, constructors) {
                            OriginalFormalActualSeedV1::Forward { origin, declared_constraint: None }
                        } else {
                            classify_original_actual_seed_in_scope(input, batch, selected, results, owner,
                                &[], None, definitions, contracts, site, None, Some(inventory), source_context.as_ref())
                        }
                    })?; },
                    Err(_) => new_calls.missing_owner(),
                }
            }
        }
        Ok::<(), String>(())
    });
    match observed {
        Ok(result) => result?,
        Err(_) => new_calls.missing_owner(),
    }
    let mut product = OriginalFormalSeedProductV1 {
        original_uses: None,
        original_use_owners: contracts
            .iter()
            .filter(|row| {
                matches!(
                    row.mode,
                    CallableParameterDeclarationModeV1::StaticBoxMethod
                        | CallableParameterDeclarationModeV1::InstanceBoxMethod
                ) && row
                    .parameters
                    .iter()
                    .any(|formal| formal.kind.is_ordinary_borrowed_handle())
            })
            .map(|row| row.owner)
            .collect(),
        rows: rows.into_boxed_slice(),
        owners: inventory.registered_owners_v1().clone(),
        source_owners: inventory.source_registered_owners_v1().clone(),
        sum_vetoes: inventory.sum_vetoed_owners_v1(),
        source_refusals: inventory.source_refusals_v1(),
        source_vetoes: inventory.source_vetoed_owners_v1(),
        definition_owners: definitions.keys().copied().collect(),
        source_integers: None,
        source_i64s: None,
        source_objects: None,
        source_sums: None,
        ordinary_core: BTreeMap::new(),
        new_calls: Rc::new(new_calls),
    };
    product.source_integers = Some(SourceIntegerDomainV1 {
        members: product
            .project_v1(
                batch,
                constructors,
                contracts,
                inventory,
                FormalDomainProjectionV1::SourceInteger,
            )?
            .1,
        new_calls: Rc::clone(&product.new_calls),
    });
    product.source_i64s = Some(SourceIntegerDomainV1 {
        members: product
            .project_v1(
                batch,
                constructors,
                contracts,
                inventory,
                FormalDomainProjectionV1::SourceI64,
            )?
            .1,
        new_calls: Rc::clone(&product.new_calls),
    });
    product.source_objects = Some(Rc::new(SourceObjectDomainV1 {
        views: product
            .project_v1(
                batch,
                constructors,
                contracts,
                inventory,
                FormalDomainProjectionV1::SourceObject,
            )?
            .0,
        new_calls: Rc::clone(&product.new_calls),
    }));
    // These roots demand original expression analysis only. They contribute no
    // incoming argument, formal value or constraint to the source domain.
    let mut expression_roots = Vec::new();
    let mut source_flows = BTreeMap::new();
    for declaration in batch.declarations().filter(|row| product.source_owners.contains(&row.owner())
        || selected.key_for_batch_slot(row.batch_slot()).is_some()
            && contracts.iter().any(|contract| contract.owner == row.owner() && contract.batch_slot == row.batch_slot())
            && selected.identity_for_batch_slot(row.batch_slot()).is_some_and(|identity| identity.same_as(row.identity()))
        || app_main.is_some_and(|main| main.checked_caller_key(batch, contracts, row.owner(), row.batch_slot()).is_ok())
        || inventory.source_rows_v1().any(|incoming| incoming.call.owner() == row.owner()
            && matches!(&incoming.source, super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original)
                if original.parameters().iter().any(|formal| matches!(formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()))
                    && selected.batch_slot(&crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(original.caller().clone())) == Some(row.batch_slot())
                    && selected.identity_for_batch_slot(row.batch_slot()).is_some_and(|identity| identity.same_as(row.identity()))))) {
        crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, declaration.owner(), |context, input| -> Result<(), String> {
                let Some(source_context) = OriginalActualSourceContextV1::checked(context, input, batch, selected, contracts,
                    static_claims, app_main, omitted)?.map(|context| context.with_field_array_accesses_v1(&field_array_access)).map(|context| context.for_point_premise()) else { return Ok(()) };
                if let Some(flow) = input.function().expression_source().value_flow() {
                    source_flows.insert(input.owner(), std::sync::Arc::clone(flow));
                }
                let local_candidates = context.declaration_slot().and_then(|slot| candidates.get(&slot))
                    .and_then(|rows| rows.as_ref().ok()).map(Vec::as_slice).unwrap_or(&[]);
                let receiver = context.declaration_slot().and_then(|slot| contracts.iter().find(|row| row.owner == input.owner())
                    .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
                    .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                        selected, batch, entry_home_loans.for_batch_slot(slot), slot)));
                for initializer in input.function().expression_source().initializers().filter(|row|
                    matches!(row.declaration_site(), crate::mir::resolved_semantics::SourceBindingSiteV1::Local { .. })) {
                    let Some(site) = initializer.initializer_site() else { continue };
                    expression_roots.push(classify_original_actual_seed_in_scope(input, batch, selected, results, input.owner(),
                        local_candidates, receiver, definitions, contracts, site, None, Some(inventory), Some(&source_context)));
                }
                // Direct ReturnValue uses the same original Core/SUM analysis;
                // a source result is not a physical Return or ownership grant.
                if let Some(shape) = input.body_shape() {
                    for site in shape.return_value_sites() {
                        expression_roots.push(classify_original_actual_seed_in_scope(input, batch, selected, results, input.owner(),
                            local_candidates, receiver, definitions, contracts, site, None, Some(inventory), Some(&source_context)));
                    }
                }
                // Conditions demand original Core analysis too; their Bool
                // root is not an incoming value or execution permission.
                use crate::mir::resolved_value_profile::expression_plan::{
                    condition_expression_source_plan_v1, loop_condition_expression_source_plan_v1,
                };
                for plan in input.function().if_region_sites().map(|site| condition_expression_source_plan_v1(input, site))
                    .chain(input.function().loop_sites().map(|site| loop_condition_expression_source_plan_v1(input, site))) {
                    let Ok(plan) = plan else { continue };
                    expression_roots.push(classify_original_actual_seed_in_scope(input, batch, selected, results, input.owner(),
                        local_candidates, receiver, definitions, contracts, plan.nodes()[0].site(), None, Some(inventory), Some(&source_context)));
                }
                // Plain rebind RHSs demand the same original expression/Core
                // analysis as Local initializers. This grants no destination,
                // Home or execution permission; the current Prefix seals those.
                if let Some(shape) = input.body_shape() {
                    for assignment in shape.assignment_sources().iter().filter(|assignment|
                        assignment.form() == crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain
                            && matches!(input.function().assignment_target(assignment.target_site()),
                                Some(crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(binding)) if binding.owner() == input.owner())) {
                        expression_roots.push(classify_original_actual_seed_in_scope(input, batch, selected, results, input.owner(),
                            local_candidates, receiver, definitions, contracts, assignment.value_site(), None, Some(inventory), Some(&source_context)));
                    }
                }
                // Field RHS reads borrow solved original points from this SAME
                // graph; no new formal incoming or independent classifier.
                if let Some(shape) = input.body_shape() {
                    for assignment in shape.assignment_sources().iter().filter(|assignment|
                        assignment.form() == crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain
                            && matches!(input.function().assignment_target(assignment.target_site()),
                                Some(crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::FieldWrite { .. }))) {
                        let prefix = assignment.value_site().node().segments();
                        for (site, binding) in input.function().variable_refs().filter(|(site, _)|
                            site.node().segments().starts_with(prefix)) {
                            let crate::mir::resolved_semantics::ResolvedLexicalRefV1::Local(binding) = binding else { continue };
                            let legacy = classify_original_actual_seed_in_scope(input, batch, selected, results, input.owner(),
                                local_candidates, receiver, definitions, contracts, site, None, Some(inventory), Some(&source_context));
                            let value = source_flows.get(&input.owner()).filter(|flow| flow.is_complete()
                                && super::super::actual_seed::uncaptured_source_binding_v1(input, *binding))
                                .and_then(|flow| flow.read(site, *binding));
                            expression_roots.push(OriginalFormalActualSeedV1::PointValue { owner: input.owner(), value, legacy: Box::new(legacy) });
                        }
                    }
                }
                // Static result classification does not walk its actuals.
                // Demand those original typed argument roots separately; this
                // adds no incoming constraint or formal-domain membership.
                for incoming in inventory.source_rows_v1().filter(|row| row.call.owner() == input.owner()) {
                    let super::super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) = &incoming.source else { continue };
                    if !input.function().method_call_source_v1(original.call_site().site())
                        .is_some_and(|call| std::sync::Arc::ptr_eq(call, original.method_source())) {
                        return Err("[freeze:contract][source-argument-demand/original-call]".into());
                    }
                    for (formal, site) in original.parameters().iter().zip(original.argument_sites()) {
                        if !matches!(formal.kind, CallableParameterContractKindV1::ExactTrivial(abi) if abi.is_i64()) { continue; }
                        expression_roots.push(classify_original_actual_seed_in_scope(input, batch, selected, results, input.owner(),
                            local_candidates, receiver, definitions, contracts, site, None, Some(inventory), Some(&source_context)));
                    }
                }
                Ok(())
            })??;
    }
    // Whole original RHSs from the retained all-writer census join the SAME
    // source graph. Missing/unsupported writers veto the field, never vanish.
    let mut field_roots = Vec::<source_sum::FieldSourceSeedsV1>::new();
    if let Some(fields) = fields {
        for (_, _, source) in fields.value_sources() {
            let Ok(source) = source else { continue; };
            if !source.same_inventory(fields) { return Err(freeze("source-sum/caller-owner")); }
            let mut seeds = Vec::new();
            for write in source.writers() {
                let seed = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
                    batch, constructors, write.owner(), |context, input| -> Result<_, String> {
                        let assignment = write.assignment();
                        let Some(shape) = input.body_shape() else { return Ok(OriginalFormalActualSeedV1::Conflict); };
                        let Ok(plan) = write.source() else { return Ok(OriginalFormalActualSeedV1::Conflict); };
                        if assignment.form() != crate::mir::resolved_semantics::ResolvedAssignmentFormV1::Plain
                            || plan.owner() != input.owner() || plan.root().statement() != assignment.statement_site()
                            || plan.nodes().first().is_none_or(|node| node.site() != assignment.value_site())
                            || shape.assignment_sources().iter().filter(|row| *row == assignment).count() != 1 {
                            return Ok(OriginalFormalActualSeedV1::Conflict);
                        }
                        let source_context = OriginalActualSourceContextV1::checked(context, input, batch, selected, contracts,
                            static_claims, app_main, omitted)?.map(|context| context.with_field_array_accesses_v1(&field_array_access)).map(|context| context.for_point_premise());
                        let slot = context.declaration_slot();
                        let local_candidates = slot.and_then(|slot| candidates.get(&slot))
                            .and_then(|rows| rows.as_ref().ok()).map(Vec::as_slice).unwrap_or(&[]);
                        let receiver = slot.and_then(|slot| contracts.iter().find(|row| row.owner == input.owner())
                            .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
                            .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                                selected, batch, entry_home_loans.for_batch_slot(slot), slot)));
                        Ok(classify_original_actual_seed_in_scope(input, batch, selected, results, input.owner(),
                            local_candidates, receiver, definitions, contracts, assignment.value_site(), None, Some(inventory), source_context.as_ref()))
                    })??;
                seeds.push(seed);
            }
            field_roots.push((Rc::clone(source), seeds));
        }
    }
    let mut expand = |owner, value| -> Result<source_sum::SourcePointExpansionV1, String> {
        use crate::mir::resolved_semantics::source_value_flow::SourceValueOriginV1 as Origin;
        use source_sum::SourcePointExpansionV1 as Expansion;
        crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, owner, |context, input| -> Result<Expansion, String> {
                let flow = input.function().expression_source().value_flow()
                    .filter(|flow| flow.owner() == owner);
                let origin = flow.and_then(|flow| flow.origin(value));
                let Some(origin) = origin else {
                    return Ok(Expansion::Unknown);
                };
                Ok(match origin {
                    Origin::Unknown => Expansion::Unknown,
                    Origin::Join { incoming, .. } => Expansion::Join(incoming.iter().map(|(_, value)| *value).collect()),
                    Origin::Entry(binding) => {
                        let receiver = context.declaration_slot().and_then(|slot| {
                            contracts
                                .iter()
                                .find(|row| row.owner == owner)
                                .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
                                .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                                    selected, batch, entry_home_loans.for_batch_slot(slot), slot,
                                ))
                        });
                        if let Some(seed) = super::super::actual_seed::source_entry_seed_v1(
                            input, constructors, contracts, owner, *binding, receiver,
                        )? {
                            Expansion::Seed(seed)
                        } else {
                            Expansion::Entry(*binding)
                        }
                    }
                    Origin::Expression(site) => {
                        let source_context = OriginalActualSourceContextV1::checked(context, input, batch, selected, contracts,
                            static_claims, app_main, omitted)?.map(|context| context.with_field_array_accesses_v1(&field_array_access)).map(|context| context.for_point_premise());
                        let slot = context.declaration_slot();
                        let local_candidates = slot.and_then(|slot| candidates.get(&slot)).and_then(|rows| rows.as_ref().ok())
                            .map(Vec::as_slice).unwrap_or(&[]);
                        let receiver = slot.and_then(|slot| contracts.iter().find(|row| row.owner == owner)
                            .filter(|row| row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod)
                            .and_then(|_| crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(
                                selected, batch, entry_home_loans.for_batch_slot(slot), slot)));
                        if let Some(seed) = super::super::actual_seed::exact_entry_object_field_seed_v1(
                            input, batch.ordinary_box_coverage(), constructors, site, receiver,
                        )? {
                            Expansion::Seed(seed)
                        } else if let Some(seed) = super::super::actual_seed::exact_entry_i64_field_seed_v1(
                            input, constructors, site, receiver,
                        )? {
                            Expansion::Seed(seed)
                        } else {
                            Expansion::Seed(classify_original_actual_seed_in_scope(input, batch, selected, results, owner,
                                local_candidates, receiver, definitions, contracts, site, None, Some(inventory), source_context.as_ref()))
                        }
                    }
                })
            })?
    };
    let mut expand_returns = |source: &Rc<crate::mir::normal_callable_semantic_package::qualified_static_call_claim::incoming_source::StaticIncomingSourceV1>, need_body: bool|
        -> Result<Option<Vec<(OwnedExprSiteV1, OriginalFormalActualSeedV1)>>, String> {
        let reject = || freeze("source-sum/caller-contract");
        crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, source.callee_owner(), |context, input| {
                let slot = context.declaration_slot().ok_or_else(reject)?;
                OriginalActualSourceContextV1::require_static_result_input_v1(source, input, slot, batch, selected, contracts, inventory, omitted)?;
                let Some(flow) = input.function().expression_source().value_flow().filter(|flow| flow.is_complete()) else { return Ok(None) };
                if source_flows.get(&input.owner()).is_none_or(|retained| !std::sync::Arc::ptr_eq(retained, flow)) { return Ok(None); }
                if !need_body { return Ok(Some(Vec::new())); }
                let Some(shape) = input.body_shape() else { return Ok(None) };
                let Some(sites) = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::verified_value_return_sites(input, shape)
                    else { return Ok(None) };
                let unique: BTreeSet<_> = sites.iter().collect();
                if sites.is_empty() || unique.len() != sites.len() { return Err(reject()); }
                let Some(source_context) = OriginalActualSourceContextV1::checked(context, input, batch, selected, contracts,
                    static_claims, app_main, omitted)?.map(|context| context.with_field_array_accesses_v1(&field_array_access)).map(|context| context.for_point_premise()) else { return Ok(None) };
                let local_candidates = candidates.get(&slot).and_then(|rows| rows.as_ref().ok()).map(Vec::as_slice).unwrap_or(&[]);
                Ok(Some(sites.iter().map(|site| (OwnedExprSiteV1::new(input.owner(), site.clone()),
                    classify_original_actual_seed_in_scope(input, batch, selected, results,
                    input.owner(), local_candidates, None, definitions, contracts, site, None, Some(inventory), Some(&source_context)))).collect()))
            })?
    };
    let mut map_receiver = |method: &std::sync::Arc<
        crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    >| {
        crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, method.owner(), |context, input| {
                let fields = fields?;
                let key = context.declaration_slot().and_then(|slot| selected.key_for_batch_slot(slot))
                    .and_then(|key| match key { crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) => Some(key), _ => None })?;
                let slot = context.declaration_slot()?;
                let entry = entry_home_loans.for_batch_slot(slot)?;
                let receiver = crate::mir::normal_callable_semantic_package::ordinary_new_coseal::entry_receiver_box_proof(selected, batch, Some(entry), slot)?;
                crate::mir::normal_callable_semantic_package::core_method_source::FieldMapReceiverSourceV1::issue(
                    input, key, method, constructors, fields, receiver, entry).ok().map(Rc::new)
            }).ok().flatten()
    };
    let sums = source_sum::resolve_original_source_sums_v1(
        &product,
        contracts,
        constructors,
        &expression_roots,
        &field_roots,
        source_flows.clone(),
        &mut expand,
        &mut expand_returns,
        &mut map_receiver,
    )?;
    for (site, operands) in &sums.core_calls {
        let receipt = crate::mir::normal_callable_semantic_package::original_call_source::with_original_source_input_v1(
            batch, constructors, site.owner(), |_, input|
                crate::mir::normal_callable_semantic_package::core_method_source::issue_ordinary_core_source_v1(input, operands))??;
        product.ordinary_core.insert(site.clone(), receipt);
    }
    product.source_sums = Some(sums);
    Ok(product)
}
