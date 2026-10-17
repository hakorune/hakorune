//! Join selected Static source edges into the existing affine execution inventory.
//! The finite context closure chooses an ABI; it never admits module roots or
//! infers result types. Pure components retain their original scalar protocol.
use super::*;
use crate::mir::builder::{AppMainCatalogCoSealV1, CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1};
use crate::mir::callable_semantic_batch::{ResolvedCallableDeclarationModeV1, VerifiedResolvedCallableSemanticBatchV1};
use crate::mir::normal_callable_semantic_package::{model::OwnedCallableParameterContractDeclarationV1,
    ordinary_new_coseal::OrdinaryNewClaimLedgerV1, selected_mapping::VerifiedSelectedCallableBatchMapV1};
use crate::mir::resolved_semantics::home_new_prefix::{LifecyclePrefixSourceV1, LocalCallResultClassV1};
use crate::mir::exact_trivial_scalar_abi::ExactTrivialScalarAbiV1;
use crate::mir::resolved_semantics::ExactCallableParamAbiV1;
use std::{collections::BTreeSet, rc::Rc};

/// Original Static source and the SAME package-issued physical lane product.
/// Exact-I64 source formals, not OrdinaryScalar words alone, admit this emitter.
#[derive(Debug)]
pub(super) struct StaticExecutionEmissionV1 {
    original: Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1>,
    publication: Option<crate::mir::callable_result_representation::StaticCallResultPublicationTakeV1>,
    signatures: Rc<crate::mir::normal_callable_semantic_package::physical_signature::VerifiedCallablePhysicalSignatureCohortV1>,
}
impl StaticExecutionEmissionV1 {
    fn corroborates_publication(&self, publication: &crate::mir::callable_result_representation::StaticCallResultPublicationTakeV1) -> bool {
        use crate::mir::callable_result_representation::StaticCallResultPublicationTakeV1 as Publication;
        match publication {
            Publication::Selected(handoff) => self.original.matches_publication_handoff_identity(handoff),
            Publication::TargetOnly(target) => target.target() == self.original.target(),
            Publication::NoExactStaticTarget => false,
        }
    }
    pub(super) fn retain_publication(&mut self, publication: crate::mir::callable_result_representation::StaticCallResultPublicationTakeV1) -> Result<(), String> {
        if self.publication.is_some() || !self.corroborates_publication(&publication) { return Err(reject("publication-source")); }
        self.publication = Some(publication);
        Ok(())
    }
    pub(super) fn validate_publication(&self) -> Result<(), String> {
        if !self.publication.as_ref().is_some_and(|publication| self.corroborates_publication(publication)) {
            return Err(reject("publication-missing-or-drift"));
        }
        Ok(())
    }
    pub(super) fn source(&self) -> &Rc<crate::mir::normal_callable_semantic_package::StaticIncomingSourceV1> { &self.original }
    pub(super) fn signatures(&self) -> &Rc<crate::mir::normal_callable_semantic_package::physical_signature::VerifiedCallablePhysicalSignatureCohortV1> { &self.signatures }
    pub(super) fn validate(&self) -> Result<(), String> {
        use crate::mir::callable_parameter_contract::{CallableParameterContractKindV1 as Kind, CallableParameterDeclarationModeV1 as Mode};
        use crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1;
        use crate::mir::normal_callable_semantic_package::physical_signature::PhysicalCallableLaneRoleV1;
        let source = &self.original;
        let signature = self.signatures.row(source.target_batch_slot()).ok_or_else(|| reject("signature-missing"))?;
        if signature.owner() != source.callee_owner() || signature.mode() != Mode::StaticBoxMethod
            || signature.receiver_lane_count() != 0 || signature.source_logical_arity() != source.target().arity()
            || signature.physical_formal_lane_count() != source.target().arity()
            || signature.physical_callable_lane_count() != source.target().arity()
            || signature.lanes().len() != source.parameters().len()
            || (!source.is_qualified_i64_source_v1() && !source.is_current_owner_i64_source_v1())
            || source.parameters().iter().zip(signature.lanes()).enumerate().any(|(ordinal, (formal, lane))|
                formal.ordinal as usize != ordinal || formal.binding.owner() != source.callee_owner()
                    || formal.kind != Kind::ExactTrivial(ExactTrivialParameterAbiV1::I64)
                    || lane.logical_ordinal() != Some(formal.ordinal) || lane.index() != formal.ordinal
                    || lane.binding() != formal.binding || lane.role() != PhysicalCallableLaneRoleV1::OrdinaryScalar) {
            return Err(reject("signature-source"));
        }
        Ok(())
    }
    pub(super) fn materialize_call(&self, dst: Option<crate::mir::ValueId>, args: Vec<crate::mir::ValueId>)
        -> Result<crate::mir::definitions::MirCall, String> {
        self.validate()?;
        self.validate_publication()?;
        if args.len() != self.original.argument_sites().len() { return Err(reject("argument-cardinality")); }
        let target = self.original.target().canonical_global_target_v1().map_err(|_| reject("target"))?;
        let mut call = crate::mir::definitions::MirCall::new(dst, crate::mir::Callee::Global(target), args);
        call.effects = crate::mir::canonical_direct_call::materialize_direct_call_effect_v1(
            crate::mir::canonical_direct_call_contract::VerifiedDirectCallEffectV1::ConservativeBarrier);
        Ok(call)
    }
}

fn reject(reason: &str) -> String { format!("[freeze:contract][static-execution/{reason}]") }
fn exact_i64(row: &DirectCallDispositionRowV1) -> bool {
    match &row.emission {
        DirectCallEmissionV1::Static(emission) => emission.validate().is_ok(),
        DirectCallEmissionV1::ResolverDirect(emission) => emission.target().signature().result() == Some(ExactTrivialScalarAbiV1::I64)
            && emission.target().signature().params().iter().all(|kind| *kind == ExactCallableParamAbiV1::I64),
    }
}
fn seal_local(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    parameters: &[OwnedCallableParameterContractDeclarationV1], ledger: &OrdinaryNewClaimLedgerV1,
    site: &OwnedExprSiteV1, row: &mut DirectCallDispositionRowV1,
) -> Result<(), String> {
    let local = ledger.local_call_for_owner(site.owner(), site.site()).ok_or_else(|| reject("local-source"))?;
    if local.owner() != site.owner() || local.site() != site || local.result() != LocalCallResultClassV1::I64
        || !local.prior_homes().is_empty() || local.local_binding().is_none()
        || local.arguments().len() != row.argument_sites.len() || !exact_i64(row)
        || (row.static_source_v1().is_none() && !lifecycle::exact_formals(batch, parameters, row)) {
        return Err(reject("local-contract"));
    }
    for owner in [site.owner(), row.callee_owner_v1()] {
        let completion = ledger.completion_for_owner(owner).ok_or_else(|| reject("completion"))?;
        if !completion.returns_value() || !completion.cleanup().root_flow().is_some_and(|flow| flow.all_exits_ready()) {
            return Err(reject("completion-not-ready"));
        }
    }
    row.execution = DirectCallExecutionV1::Lifecycle;
    ledger.record_lifecycle_local_call_site(site.owner(), site.clone());
    Ok(())
}

impl DirectCallDispositionLoansV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn join_static_execution_v1(
        loans: &mut Option<Self>, batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1, main: Option<&AppMainCatalogCoSealV1>,
        parameters: &[OwnedCallableParameterContractDeclarationV1], ledger: &OrdinaryNewClaimLedgerV1,
        signatures: &Rc<crate::mir::normal_callable_semantic_package::physical_signature::VerifiedCallablePhysicalSignatureCohortV1>,
        results: &crate::mir::normal_callable_semantic_package::result_contract::VerifiedCallableResultContractCohortV1,
    ) -> Result<(), String> {
        let mut owners = BTreeMap::<FunctionOwnerIdV1, (u32, CanonicalSameModuleCallableKeyV1)>::new();
        for key in selected.keys() {
            let SelectedNormalCallableKeyV1::Cataloged(cataloged) = key else { continue };
            let slot = selected.batch_slot(key).ok_or_else(|| reject("selected-slot"))?;
            let declaration = batch.declarations().find(|row| row.batch_slot() == slot).ok_or_else(|| reject("selected-declaration"))?;
            if declaration.mode() != ResolvedCallableDeclarationModeV1::StaticBoxMethod { continue; }
            if !selected.identity_for_batch_slot(slot).is_some_and(|identity| identity.same_as(declaration.identity())) {
                return Err(reject("selected-identity"));
            }
            owners.insert(declaration.owner(), (slot, cataloged.clone()));
        }
        let mut context = BTreeSet::new();
        if let Some(main) = main {
            let declaration = batch.declarations().find(|row| row.identity().same_as(main.parser_identity()))
                .ok_or_else(|| reject("main-identity"))?;
            let source = batch.with_lowering_input(declaration.batch_slot(), |input|
                ledger.main_arguments_source_v1(input, main.parser_identity())).map_err(|_| reject("main-input"))??;
            if !source.matches_catalog_v1(main) { return Err(reject("main-catalog")); }
            if source.requires_lifecycle() { context.insert(declaration.owner()); }
            owners.insert(declaration.owner(), (declaration.batch_slot(), main.catalog_key().clone()));
        }
        for &owner in owners.keys() {
            if !ledger.call_relations_for_owner(owner).is_empty() || ledger.requires_new_context_v1(owner) { context.insert(owner); }
            if let Some(flow) = ledger.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow()) {
                let prefixes = flow.lifecycle_prefixes(&[]).map_err(reject)?;
                if !flow.maps().is_empty() || !flow.named_arrays().is_empty()
                    || prefixes.iter().any(|prefix| !matches!(prefix, LifecyclePrefixSourceV1::Call { .. })) {
                    context.insert(owner);
                }
            }
        }
        let mut additions = Vec::new();
        for (&owner, (slot, caller)) in &owners {
            batch.with_lowering_input(*slot, |input| -> Result<(), String> {
                for (site, _) in input.function().method_calls() {
                    let site = OwnedExprSiteV1::new(owner, site.clone());
                    if !ledger.local_call_for_owner(owner, site.site()).is_some_and(|local| local.result() == LocalCallResultClassV1::I64 && local.local_binding().is_some()) { continue; }
                    let Some(original) = ledger.borrow_original_static_source_v1(&site)? else { continue };
                    if !original.is_qualified_i64_source_v1() && !original.is_current_owner_i64_source_v1() { continue; }
                    if !owners.contains_key(&original.callee_owner()) { continue; }
                    if original.caller() != caller || !original.catalog_brand().is_same(selected.catalog_brand()) {
                        return Err(reject("source-catalog"));
                    }
                    let emission = StaticExecutionEmissionV1 { original: Rc::clone(original), signatures: Rc::clone(signatures), publication: None };
                    // This typed execution family cannot admit opaque storage words.
                    if original.parameters().iter().any(|formal| formal.kind != crate::mir::callable_parameter_contract::CallableParameterContractKindV1::ExactTrivial(
                        crate::mir::exact_trivial_parameter_abi::ExactTrivialParameterAbiV1::I64)) { continue; }
                    emission.validate()?;
                    let declaration = batch.declarations().find(|row| row.batch_slot() == original.target_batch_slot())
                        .ok_or_else(|| reject("target-declaration"))?;
                    let signature = signatures.row(original.target_batch_slot()).ok_or_else(|| reject("target-signature"))?;
                    if declaration.owner() != original.callee_owner() || !signature.identity().same_as(declaration.identity()) {
                        return Err(reject("target-identity"));
                    }
                    let result = results.row(original.target_batch_slot()).ok_or_else(|| reject("result-contract"))?.borrow();
                    let completion = ledger.completion_for_owner(original.callee_owner()).ok_or_else(|| reject("result-completion"))?;
                    if result.owner() != original.callee_owner() || !result.identity().same_as(declaration.identity())
                        || !result.declared_result_agrees_with_i64_source() || !std::ptr::eq(result.completion(), completion) {
                        return Err(reject("result-source"));
                    }
                    let row = DirectCallDispositionRowV1 {
                        argument_sites: original.argument_sites().to_vec().into_boxed_slice(), taken_site: None,
                        observed_arguments: None, argument_projections: None, emission: DirectCallEmissionV1::Static(emission),
                        execution: DirectCallExecutionV1::Lifecycle, result: crate::mir::instruction::InvokeCallResultKind::I64,
                    };
                    additions.push((site, row));
                }
                Ok(())
            }).map_err(|_| reject("source-input"))??;
        }
        let mut edges: Vec<_> = additions.iter().map(|(site, row)| (site.owner(), row.callee_owner_v1())).collect();
        let mut direct_sites = BTreeSet::new();
        if let Some(loans) = loans.as_ref() {
            for loan in loans.iter() {
                for (site, slot) in &loan.rows {
                    if let DirectCallDispositionSlotV1::Ready(row) = slot {
                        if exact_i64(row) && owners.contains_key(&site.owner()) && owners.contains_key(&row.callee_owner_v1())
                            && ledger.local_call_for_owner(site.owner(), site.site()).is_some_and(|local| local.local_binding().is_some()) {
                            direct_sites.insert(site.clone());
                            edges.push((site.owner(), row.callee_owner_v1()));
                        }
                    }
                }
            }
        }
        loop {
            let mut changed = false;
            for &(caller, callee) in &edges {
                if context.contains(&caller) || context.contains(&callee) {
                    changed |= context.insert(caller); changed |= context.insert(callee);
                }
            }
            if !changed { break; }
        }
        if loans.is_none() && !additions.iter().any(|(site, row)| context.contains(&site.owner())
            || context.contains(&row.callee_owner_v1())) { return Ok(()); }
        let inventory = loans.get_or_insert_with(|| Self { loans: BTreeMap::new() });
        for loan in inventory.iter_mut() {
            for (site, slot) in &mut loan.rows {
                if let DirectCallDispositionSlotV1::Ready(row) = slot {
                    if direct_sites.contains(site) && matches!(row.execution, DirectCallExecutionV1::Scalar)
                        && (context.contains(&site.owner()) || context.contains(&row.callee_owner_v1())) {
                        seal_local(batch, parameters, ledger, site, row)?;
                    }
                }
            }
        }
        for (site, mut row) in additions {
            if !context.contains(&site.owner()) && !context.contains(&row.callee_owner_v1()) { continue; }
            seal_local(batch, parameters, ledger, &site, &mut row)?;
            let loan = inventory.loans.entry(site.owner()).or_insert_with(|| DirectCallDispositionLoanV1 {
                owner: site.owner(), rows: BTreeMap::new(), canonical_route_bypassed: false,
            });
            if loan.rows.insert(site, DirectCallDispositionSlotV1::Ready(row)).is_some() { return Err(reject("duplicate-edge")); }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn original_static_execution_rows_follow_context_without_admitting_pure_components() {
        for (parameters, methods, body, expected) in [
            ("", "find(): i64 { return 1 }", "local out = Values.find()", false),
            ("args", "find(): i64 { return 1 }", "local out = Values.find()", true),
            ("args", "read(count: i64, other: i64): i64 { local n = count + \"d\".length() return 0 }", "local out = Values.read(1, 2)", true),
        ] {
            let text = format!("static box Values {{ {methods} }} static box Main {{ main({parameters}) {{ {body} return 0 }} }}");
            let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(&text).unwrap();
            let main = package.catalog.catalog().source_backed_app_main().unwrap();
            let declaration = package.batch().declarations().find(|row| row.identity().same_as(main.parser_identity())).unwrap();
            let actual = package.direct_call_loans.as_ref().is_some_and(|loans| loans.has_static_execution_for_owner_v1(declaration.owner()));
            let details = package.batch().with_lowering_input(declaration.batch_slot(), |input| {
                input.function().method_calls().map(|(site, _)| {
                    let owned = OwnedExprSiteV1::new(input.owner(), site.clone());
                    format!("site={owned:?} local={:?} original={:?}", package.ordinary_new_claim_ledger.local_call_for_owner(input.owner(), site),
                        package.ordinary_new_claim_ledger.borrow_original_static_source_v1(&owned))
                }).collect::<Vec<_>>()
            }).unwrap();
            assert_eq!(actual, expected, "{text}: selected={:?}; {details:?}", package.selected.keys().collect::<Vec<_>>());
        }
    }
}
