//! Whole-holder handback from existing progress, not a second emission registry.
use super::*;
use super::super::physical_boundary::FinishedBindings;
use crate::mir::normal_callable_semantic_package::{EmittedNamedArrayRequirementV1, EmittedOwnedArrayReadV1};
use crate::mir::MirModule;
use std::cell::OnceCell;
type Binding = (BasicBlockId, MirInstruction);
type Recipe = super::super::super::ArrayHolderLifecycleRecipeV1;

#[derive(Debug, PartialEq)]
struct FinishedOwnedArrayHolderV1 {
    symbol: String,
    is_root: bool,
    bindings: Vec<Binding>,
}

#[derive(Debug)]
pub(crate) struct EmittedOwnedArrayHolderV1 {
    recipe: Rc<Recipe>,
    array: ValueId,
    construction: Vec<Binding>,
    groups: Vec<RootLifecyclePrefixBindingGroupV1>,
    reads: BTreeMap<OwnedExprSiteV1, Rc<EmittedOwnedArrayReadV1>>,
    ends: Vec<(SourceStmtSiteV1, Binding)>,
    finished: OnceCell<FinishedOwnedArrayHolderV1>,
}
impl EmittedOwnedArrayHolderV1 {
    pub(crate) fn array_value(&self) -> ValueId { self.array }
    pub(crate) fn recipe(&self) -> &Rc<Recipe> { &self.recipe }
    pub(crate) fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 { self.recipe.caller() }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 { self.recipe.owner() }
    pub(crate) fn construction(&self) -> &crate::mir::resolved_semantics::SourceExprSiteV1 { self.recipe.construction() }

    fn check_source_coverage(&self, ledger: &OrdinaryNewClaimLedgerV1) -> Result<(), String> {
        if ledger.live_owned_array_recipe_value_v1(&self.recipe)? != self.array {
            return Err(freeze("owned-holder/value-drift"));
        }
        let appends: Vec<_> = self.groups.iter().filter_map(|group| group.owned_array_append())
            .filter(|packet| Rc::ptr_eq(&packet.source().recipe, &self.recipe)).collect();
        let expected: Vec<_> = self.recipe.original_append_sites_v1().collect();
        if appends.len() != expected.len() || expected.iter().any(|site|
            appends.iter().filter(|packet| packet.source().site() == site).count() != 1) {
            return Err(freeze("owned-holder/append-source-coverage"));
        }
        let reads: Vec<_> = self.recipe.original_read_sites_v1().collect();
        if reads.len() != self.reads.len() || reads.iter().any(|site| !self.reads.contains_key(*site)) {
            return Err(format!("{}: owner={:?}, expected={reads:?}, emitted={:?}",
                freeze("owned-holder/read-source-coverage"), self.owner(), self.reads.keys().collect::<Vec<_>>()));
        }
        let ends = self.recipe.original_end_sites_v1()?;
        if ends.len() != self.ends.len() || ends.iter().any(|site|
            self.ends.iter().filter(|(observed, _)| observed == *site).count() != 1) {
            return Err(freeze("owned-holder/end-source-coverage"));
        }
        self.check_read_packets(ledger, None)
    }

    fn check_read_packets(
        &self, ledger: &OrdinaryNewClaimLedgerV1,
        finished: Option<(&MirFunction, &FinishedBindings)>,
    ) -> Result<(), String> {
        let mut seen = std::collections::BTreeSet::new();
        for group in &self.groups {
            for packet in group.lexical_packets() {
                let arguments = ledger.original_local_packet_arguments_v1(packet)?;
                packet.visit_original_nodes_v1(self.owner(), arguments, ledger, &mut |node, _| {
                    for (ordinal, read) in node.array_read_records_v1() {
                        if !Rc::ptr_eq(&read.source().recipe, &self.recipe) { continue; }
                        if !self.reads.get(read.source().site()).is_some_and(|original| Rc::ptr_eq(original, read))
                            || !seen.insert(read.source().site().clone()) {
                            return Err(freeze("owned-holder/read-packet-identity"));
                        }
                        if let Some((function, projection)) = finished {
                            let outer = projection.binding(node.invoke_binding().0, &node.invoke_binding().1)?
                                .ok_or_else(|| freeze("owned-holder/read-call-removed"))?;
                            let coordinate = locate(function, &outer)?;
                            read.validate_finished_v1(function, projection, ledger, node.call_site(),
                                node.invoke_binding(), coordinate, ordinal)?;
                        }
                    }
                    Ok(())
                })?;
            }
        }
        if seen.len() != self.reads.len() { return Err(freeze("owned-holder/read-packet-coverage")); }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::local_commit) fn project_final_v1(
        &self, ledger: &OrdinaryNewClaimLedgerV1, function: &MirFunction,
        projection: &FinishedBindings,
    ) -> Result<(), String> {
        self.check_source_coverage(ledger)?;
        let is_root = match &*ledger.root_validation.borrow() {
            RootNewValidation::FinishingChecked { owner, symbol, projection: original }
                if *owner == self.owner() => {
                let (selected, _) = ledger.original_instance_input_issuance.as_ref()
                    .ok_or_else(|| freeze("owned-holder/original-issuance-missing"))?;
                let contracts = ledger.original_parameter_contracts.as_deref()
                    .ok_or_else(|| freeze("owned-holder/original-catalog-missing"))?;
                if symbol != &function.signature.name || !std::ptr::eq(original.as_ref(), projection)
                    || ledger.checked_root_array_holder_caller_v1(self.owner(), selected, contracts)? != Some(self.caller()) {
                    return Err(freeze("owned-holder/root-finished-identity"));
                }
                true
            }
            _ => {
                ledger.with_finished_child_projection(self.owner(), |symbol, original| {
                    if symbol != function.signature.name || !std::ptr::eq(original, projection) {
                        return Err(freeze("owned-holder/child-finished-identity"));
                    }
                    Ok(())
                })?;
                false
            }
        };
        let site = OwnedExprSiteV1::new(self.owner(), self.construction().clone());
        ledger.validate_owned_array_construction(&site, function, Some(projection))?;
        for group in &self.groups { group.validate_recorded(ledger, function, Some(projection))?; }
        self.check_read_packets(ledger, Some((function, projection)))?;
        for original in self.construction.iter().chain(self.ends.iter().map(|(_, binding)| binding)) {
            if !super::super::physical_boundary::check_binding(function, Some(projection), original.0, &original.1)? {
                return Err(freeze("owned-holder/construction-or-end-drift"));
            }
        }
        // The existing owner inventory includes source-checked prefix/constructor
        // Fault End operations as well as the explicit exit End obligations.
        let mut bindings = projection.bindings(&ledger.lifecycle_bindings(self.owner())?)?;
        let mut unique = Vec::new();
        for binding in bindings.drain(..) {
            if !unique.contains(&binding) { unique.push(binding); }
        }
        self.check_physical(function, &unique)?;
        let finished = FinishedOwnedArrayHolderV1 {
            symbol: function.signature.name.clone(), is_root, bindings: unique,
        };
        if let Some(previous) = self.finished.get() {
            if previous != &finished { return Err(freeze("owned-holder/finished-projection-drift")); }
        } else {
            self.finished.set(finished).map_err(|_| freeze("owned-holder/finished-duplicate"))?;
        }
        Ok(())
    }
    fn check_physical(&self, function: &MirFunction, bindings: &[Binding]) -> Result<(), String> {
        for binding in bindings { locate(function, binding)?; }
        let expected: Vec<_> = bindings.iter().filter(|(_, row)| matches!(row,
            MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(operation), .. }
                if operation.array() == self.array)).collect();
        let mut claim = 0;
        let mut appends = 0;
        let mut reads = 0;
        for (block, body) in &function.blocks {
            for row in body.all_instructions() {
                if let MirInstruction::Invoke { operation: InvokeOperation::OwnedArray(operation), .. } = row {
                    if operation.array() != self.array { continue; }
                    if operation.descriptor() != self.recipe.slot_descriptor_v1()
                        || expected.iter().filter(|binding| binding.0 == *block && &binding.1 == row).count() != 1 {
                        return Err(freeze("owned-holder/reverse-physical-coverage"));
                    }
                    match operation { Array::Claim { .. } => claim += 1, Array::Append { .. } => appends += 1,
                        Array::Get { .. } => reads += 1, Array::End { .. } => {} }
                }
            }
        }
        if claim != 1 || appends != self.recipe.original_append_sites_v1().count()
            || reads != self.recipe.original_read_sites_v1().count() {
            return Err(freeze("owned-holder/physical-source-cardinality"));
        }
        Ok(())
    }
    /// Lend the same finalized physical facts after named-array validation.
    pub(crate) fn finished_bindings_v1(&self) -> Result<&[(BasicBlockId, MirInstruction)], String> {
        self.finished.get().map(|row| row.bindings.as_slice()).ok_or_else(|| freeze("owned-holder/not-finished"))
    }
    pub(crate) fn finished_symbol_v1(&self) -> Result<&str, String> {
        self.finished.get().map(|row| row.symbol.as_str()).ok_or_else(|| freeze("owned-holder/not-finished"))
    }
    pub(crate) fn validate<'m>(&self, module: &'m MirModule) -> Result<&'m str, String> {
        let finished = self.finished.get().ok_or_else(|| freeze("owned-holder/not-finished"))?;
        let canonical = module.canonical_callable_definition_symbol(self.caller());
        if (!finished.is_root && canonical != Some(finished.symbol.as_str()))
            || canonical.is_some_and(|symbol| symbol != finished.symbol) {
            return Err(freeze("owned-holder/canonical-caller-mismatch"));
        }
        let (symbol, function) = module.functions.get_key_value(&finished.symbol)
            .ok_or_else(|| freeze("owned-holder/function-missing"))?;
        if function.signature.name != *symbol { return Err(freeze("owned-holder/function-identity")); }
        self.check_physical(function, &finished.bindings)?;
        Ok(symbol)
    }
}
fn locate(function: &MirFunction, binding: &Binding) -> Result<(BasicBlockId, usize), String> {
    let block = function.blocks.get(&binding.0).ok_or_else(|| freeze("owned-holder/block-missing"))?;
    let mut rows = block.all_instructions().enumerate().filter(|(_, row)| *row == &binding.1);
    let (position, _) = rows.next().ok_or_else(|| freeze("owned-holder/instruction-missing"))?;
    if rows.next().is_some() { return Err(freeze("owned-holder/instruction-duplicate")); }
    Ok((binding.0, position))
}
impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn capture_owned_array_holders_v1(&self, owner: FunctionOwnerIdV1) -> Result<Vec<EmittedNamedArrayRequirementV1>, String> {
        let recipes: Vec<_> = self.array_holder_lifecycles.borrow().values()
            .filter(|recipe| recipe.owner() == owner).cloned().collect();
        let groups = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(self)
            .with_local_call_group_tree_v2(owner, |groups| Ok(groups.to_vec()))?;
        let mut emitted = Vec::new();
        for recipe in recipes {
            let array = self.live_owned_array_recipe_value_v1(&recipe)?;
            let site = OwnedExprSiteV1::new(owner, recipe.construction().clone());
            let (construction, reads) = {
                let rows = self.local_commits.borrow();
                let Some(LocalCommitV1::OwnedArray(row)) = rows.get(&site) else { return Err(freeze("owned-holder/progress-missing")); };
                (row.emitted_bindings().to_vec(), row.reads.clone())
            };
            let mut ends = Vec::new();
            for exit in recipe.original_end_sites_v1()? {
                let exits = self.root_exits.borrow();
                let Some(RootHomeExitProgress::Emitted { origins, bindings, .. }) = exits.get(&(owner, exit.clone().into())) else {
                    return Err(freeze("owned-holder/end-not-emitted"));
                };
                let mut matching = origins.iter().filter(|emission|
                    emission.origin().subject() == &RootHomeReleaseSubjectV1::Binding(recipe.holder()));
                let end = matching.next().ok_or_else(|| freeze("owned-holder/end-origin-missing"))?;
                let expected = InvokeOperation::OwnedArray(Array::End { array, descriptor: recipe.slot_descriptor_v1() });
                let binding = end.binding();
                if matching.next().is_some() || end.origin().exit() != exit || end.origin().operation() != &expected
                    || !matches!(&binding.1, MirInstruction::Invoke { operation, .. } if operation == &expected)
                    || bindings.iter().filter(|row| **row == binding).count() != 1 {
                    return Err(freeze("owned-holder/end-origin-drift"));
                }
                ends.push((exit.clone(), binding));
            }
            let row = EmittedOwnedArrayHolderV1 { recipe, array, construction, groups: groups.clone(), reads, ends, finished: OnceCell::new() };
            row.check_source_coverage(self)?;
            emitted.push(EmittedNamedArrayRequirementV1::OwnedHolder(row));
        }
        Ok(emitted)
    }
}
