//! Original declaration and full incoming correspondence, before actual readiness.
//! This structure grants no executable actual, object view or compiled marker.
use super::*;
use super::owner_view::BorrowedEntryOwnerViewV1;

pub(super) struct BorrowedEntryStructureV1<'a> {
    owner: FunctionOwnerIdV1,
    source: &'a PreparedBorrowedFormalIngressV1,
    evidence: EntryStructureEvidenceV1<'a>,
    formals: Box<[(u32, BindingRefV1)]>,
}

#[derive(Debug)]
pub(super) enum EntryStructureEvidenceV1<'a> {
    Uses(BorrowedEntryOwnerViewV1<'a>),
    FiniteInput {
        uses: Rc<super::super::borrowed_formal_uses::OriginalFormalUseSourceV1>,
        incoming: Box<[&'a super::super::borrowed_formal_uses::BorrowedIncomingCallDraftV1]>,
        domains: BTreeMap<BindingRefV1, Rc<super::super::borrowed_formal_source::SourceValueSumV1>>,
    },
}

impl OrdinaryNewClaimLedgerV1 {
    /// Source-use closure shared by the original Static and Instance input owners. Every original
    /// read is accounted for by a qualified purpose or an exact entry alias;
    /// physical entry/use correspondence remains a separate final obligation.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn checked_input_use_source_v1(
        &self, contract: &crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1,
    ) -> Result<Option<Rc<super::super::borrowed_formal_uses::OriginalFormalUseSourceV1>>, String> {
        use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1 as Reference;
        use crate::mir::resolved_semantics::ResolvedLexicalRefV1;
        macro_rules! input_use_pending {
            ($stage:literal) => {{
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[entry-input-use] owner={:?} pending at {}", contract.owner, $stage);
                }
                return Ok(None);
            }};
        }
        let parameters: Vec<_> = contract.parameters.iter().map(|row|
            (row.ordinal, row.binding, row.kind.clone())).collect();
        let Some(structure) = self.borrowed_entry_structure_for_contract(contract.owner, &parameters)? else { input_use_pending!("structure") };
        let uses = structure.source.original_formal_uses_v1(contract.owner)?;
        if !uses.matches_contract(contract) || uses.roots().keys().copied().collect::<BTreeSet<_>>()
            != structure.formals.iter().map(|(_, formal)| *formal).collect() {
            return Err(freeze("static-input/original-use-contract"));
        }
        if let EntryStructureEvidenceV1::FiniteInput { uses: retained, .. } = &structure.evidence {
            if !Rc::ptr_eq(uses, retained) { return Err(freeze("static-input/original-use-identity")); }
        }
        // These remain explicit obligations until their own source/control
        // owners can lend a complete proof; no empty-use draft substitutes.
        if !uses.captures().is_empty() { input_use_pending!("captures") }
        let mut origins = uses.roots().clone();
        let mut covered = BTreeSet::new();
        let flow = self.completion_for_owner(contract.owner).and_then(|completion| completion.cleanup().root_flow());
        for initializer in uses.initializers() {
            let Some(site) = initializer.initializer_site() else { continue };
            // A source-qualified numeric initializer projects a value, rather
            // than installing another borrowed handle alias. Its own frame
            // below accounts for the original entry read exactly once.
            if flow.is_some_and(|flow| flow.expression_plans().iter()
                .filter_map(|plan| plan.initializer_frame()).any(|frame|
                    frame.binding() == initializer.binding() && frame.is_i64()
                        && frame.entry_reads().iter().any(|read| read.requirement().node() == 0 && read.signed_only()))) {
                continue;
            }
            let Some(binding) = uses.reads().find_map(|(at, reference)| match reference {
                ResolvedLexicalRefV1::Local(binding) if at == site => Some(*binding), _ => None,
            }) else { continue };
            if let Some(formal) = uses.initializer_entry_alias_v1(initializer, Some(binding))
                .map_err(|_| freeze("static-input/alias-source"))? {
                if origins.insert(initializer.binding(), formal).is_some()
                    || !covered.insert((site.clone(), binding)) {
                    return Err(freeze("static-input/alias-duplicate"));
                }
            }
        }
        let mut plans: Vec<_> = flow.into_iter().flat_map(|flow| flow.expression_plans()).collect();
        let loops: Vec<_> = plans.iter().filter_map(|plan| plan.loop_recipe_v2()).collect();
        let mut calls: Vec<_> = flow.into_iter().flat_map(|flow| flow.local_calls()).collect();
        for product in &loops {
            if product.source.source.owner() != contract.owner { return Err(freeze("static-input/qualified-use-identity")); }
            plans.extend(product.source.body_expressions.iter());
            for step in product.schedule().calls_v2() {
                if let crate::mir::loop_recipe_contract::SourceLoopScheduledStepV2::Call { observation, .. } = step {
                    if !calls.contains(&observation) { calls.push(observation); }
                }
            }
        }
        let mut assignments = BTreeSet::new();
        let mut evaluations: Vec<&crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1> = Vec::new();
        let mut evaluation_reads = BTreeSet::new();
        for plan in plans {
            if let Some(frame) = plan.rebind_frame() {
                let crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1::Assignment { assignment }
                    = frame.source().root() else { return Err(freeze("static-input/qualified-use-identity")) };
                if origins.contains_key(&frame.target())
                    || !uses.assignments().iter().any(|(site, target)| site == assignment.target_site()
                        && *target == crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(frame.target())) {
                    input_use_pending!("rebind-frame")
                }
                assignments.insert(assignment.target_site().clone());
            }
            if let Some(frame) = plan.field_write_frame().filter(|frame| frame.entry().is_some()) {
                use crate::mir::resolved_value_profile::expression_plan::{ExpressionPlanRootV1, ExpressionNodeSourceV1};
                let entry = frame.entry().expect("selected entry writer");
                let ExpressionPlanRootV1::Assignment { assignment } = frame.source().root()
                    else { return Err(freeze("static-input/qualified-use-identity")) };
                let Some(rhs) = plan.field_write_rhs() else { input_use_pending!("field-write-rhs") };
                if !rhs.is_consistent(frame.source()) || !Rc::ptr_eq(entry.source(), frame.source())
                    || frame.source().owner() != contract.owner || !uses.assignments().iter().any(|(site, target)|
                        site == assignment.target_site() && matches!(target,
                            crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::FieldWrite { receiver }
                                if receiver == entry.original().write().receiver_site()))
                    || !assignments.insert(assignment.target_site().clone()) {
                    return Err(freeze("static-input/qualified-use-identity"));
                }
                entry.recheck(self)?;
                let mut reads = vec![(entry.original().write().receiver_site(), entry.receiver())];
                let mut entry_reads = BTreeSet::new();
                // Text references/call inputs belong to the same common
                // evaluation closure below. Only the target receiver is extra.
                if rhs.evaluation().is_none() {
                    for (node, original) in frame.source().nodes().iter().enumerate() {
                        let binding = match original.source() {
                            ExpressionNodeSourceV1::Reference(ResolvedLexicalRefV1::Local(binding)) => {
                                if rhs.reference(node).is_none() && *binding != entry.receiver() { input_use_pending!("entry-write-reference") }
                                *binding
                            },
                            ExpressionNodeSourceV1::Shape(crate::mir::resolved_semantics::BodyExpressionShapeV1::Me {
                                receiver: crate::mir::resolved_semantics::BodyMeReceiverV1::Lexical(binding), .. })
                                if *binding == entry.receiver() => *binding,
                            _ => continue,
                        };
                        if origins.contains_key(&binding) {
                            let Some(Reference::FiniteEntry { read, exclusion: None }) = rhs.reference_source(node)
                                else {
                                    if crate::config::env::builder_debug_enabled() {
                                        eprintln!("[entry-input-use] entry-write-finite-read node={node} binding={binding:?} reference={}",
                                            reference_kind_name(rhs.reference_source(node)));
                                    }
                                    input_use_pending!("entry-write-finite-read")
                                };
                            if !read.signed_only() || !read.requirement().corroborates_original_uses_v1(uses)
                                || origins.get(&binding) != Some(&read.requirement().formal().1) {
                                return Err(freeze("static-input/entry-read-source"));
                            }
                            entry_reads.insert((original.site().clone(), binding));
                        }
                        reads.push((original.site(), binding));
                    }
                }
                for (site, binding) in reads {
                    if origins.contains_key(&binding) && !entry_reads.contains(&(site.clone(), binding)) { input_use_pending!("entry-write-read-uncovered") }
                    if uses.reads().any(|(at, reference)| at == site && *reference == ResolvedLexicalRefV1::Local(binding))
                        && !covered.insert((site.clone(), binding)) {
                        return Err(freeze("static-input/read-use-duplicate-or-foreign"));
                    }
                }
            }
            if let Some(evaluation) = plan.evaluation() {
                if !evaluations.iter().any(|old| std::ptr::eq(*old, evaluation)) { evaluations.push(evaluation); }
            }
        }
        let new_evaluations = self.new_input_evaluations_v1(
            contract.owner,
            uses,
            &mut covered,
        )?;
        for evaluation in &new_evaluations {
            if !evaluations.iter().any(|old| std::ptr::eq(*old, evaluation.as_ref())) { evaluations.push(evaluation); }
        }
        self.collect_terminal_input_evaluations_v1(contract.owner, uses, &origins, &mut covered, &mut evaluations)?;
        for product in &loops {
            let evaluation = product.source.condition.evaluation();
            if !evaluations.iter().any(|old| std::ptr::eq(*old, evaluation)) { evaluations.push(evaluation); }
        }
        for evaluation in evaluations {
            if evaluation.source().owner() != contract.owner || !evaluation.reference_coverage_is_exact() {
                return Err(freeze("static-input/qualified-use-identity"));
            }
            // Core owns only its exact transferred Me receiver. Account for
            // that original read here; key references keep their own proofs.
            for core in evaluation.cores().values() {
                let Some(field) = core.field_map() else { continue };
                if !core.matches_plan(evaluation.source(), core.node()) || origins.contains_key(&field.receiver()) {
                    return Err(freeze("static-input/qualified-use-identity"));
                }
                field.require_entry_source_v1(self)?;
                if !uses.reads().any(|(at, original)| at == field.receiver_site()
                    && *original == ResolvedLexicalRefV1::Local(field.receiver()))
                    || !covered.insert((field.receiver_site().clone(), field.receiver())) {
                    return Err(freeze("static-input/read-use-duplicate-or-foreign"));
                }
            }
            for (&node, reference) in evaluation.references() {
                let site = evaluation.source().nodes()[node].site();
                let scalar_read = if let Reference::Scalar(
                    crate::mir::resolved_semantics::home_new_prefix::CheckedScalarReferenceV1::Original(read),
                ) = reference {
                    Some(read)
                } else {
                    None
                };
                let current_sum = scalar_read.and_then(|read| self.borrowed_formal_source.as_ref()
                    .and_then(|source| source.as_ref().ok())
                    .and_then(|source| source.lend_current_scalar_sum_v1(read)));
                let formal_domain_signed = origins.get(&reference.binding()).is_some_and(|formal|
                    match &structure.evidence {
                        EntryStructureEvidenceV1::FiniteInput { domains, .. } => domains
                            .get(formal).is_some_and(|sum| sum.signed_only()),
                        EntryStructureEvidenceV1::Uses(_) => structure.source
                            .lend_source_value_sum_v1(*formal).is_some_and(|sum| sum.signed_only()),
                    });
                let direct_formal = origins.get(&reference.binding()) == Some(&reference.binding());
                let original_scalar_read = scalar_read.is_some_and(|read|
                    read.read() == site && read.binding() == reference.binding());
                let scalar_obligation = matches!(reference,
                    Reference::Scalar(crate::mir::resolved_semantics::home_new_prefix::CheckedScalarReferenceV1::Obligation { binding, .. })
                        if *binding == reference.binding());
                let source_scalar_entry = formal_domain_signed
                    && uses.reads().any(|(at, original)| at == site
                        && *original == ResolvedLexicalRefV1::Local(reference.binding()))
                    && ((direct_formal && (original_scalar_read || scalar_obligation))
                        || (original_scalar_read && current_sum.as_ref().is_some_and(|sum| sum.signed_only())));
                let entry = match reference {
                    Reference::ArgumentEntry(read) => Some(read),
                    Reference::FiniteEntry { read, .. } => Some(read.requirement()),
                    _ => None,
                };
                if let Some(entry) = entry {
                    if !entry.corroborates_original_uses_v1(uses)
                        || origins.get(&entry.binding()) != Some(&entry.formal().1) {
                        return Err(freeze("static-input/entry-read-source"));
                    }
                } else if origins.contains_key(&reference.binding()) && !source_scalar_entry {
                    input_use_pending!("evaluation-entry-reference")
                }
                // Nested call-argument evaluations may expose the same source
                // reference as their containing expression evaluation. The
                // source node is one read, so account for that identity once.
                if !evaluation_reads.insert((site.clone(), reference.binding())) {
                    continue;
                }
                let source_read = uses.reads().any(|(at, original)| at == site && *original == ResolvedLexicalRefV1::Local(reference.binding()));
                let unique_read = covered.insert((site.clone(), reference.binding()));
                if !source_read || !unique_read {
                    return Err(freeze("static-input/read-use-duplicate-or-foreign"));
                }
            }
        }
        if let EntryStructureEvidenceV1::FiniteInput { incoming, .. } = &structure.evidence {
            if uses.cover_exact_bool_static_return_reads_v1(contract.owner, contract.batch_slot,
                incoming, &origins, &mut covered)? == Some(false) { input_use_pending!("bool-static-return") }
        }
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
        for observation in calls {
            // Borrow the same checked finite-source census as entry/actuals;
            // legacy-only membership cannot cover a qualified source-only call.
            let original = Self::original_instance_input_call_v1(structure.source, catalog, observation.site())?;
            let original = original.as_ref();
            // Finite Static and exact-i64 dependencies retain their own source
            // issuer; source-use closure never substitutes completed actuals.
            let scalar_static = if original.is_none() { self.original_static_input_use_source_v1(observation.site())? } else { None };
            let target = original.map(|row| row.source.as_loan()).transpose()?;
            let argument_sites = match (target.as_ref(), scalar_static.as_ref()) {
                (Some(target), None) => {
                    if target.call_site() != observation.site() { return Err(freeze("static-input/qualified-use-identity")); }
                    target.argument_sites()
                }
                (None, Some(original)) => original.argument_sites(),
                _ => {
                    if crate::config::env::builder_debug_enabled() {
                        eprintln!("[input-use] owner={:?} original call source unavailable at {:?}", contract.owner, observation.site());
                    }
                    input_use_pending!("original-call-source")
                }
            };
            if argument_sites.len() != observation.arguments().len()
                || self.local_call_for_owner(contract.owner, observation.site().site()) != Some(observation) {
                return Err(freeze("static-input/qualified-use-identity"));
            }
            if let Some(instance) = original.and_then(|row| row.source.instance()) {
                // A stored Provider child is read through its parent `me.<field>`.
                let (binding, read_site) = match instance.stored_receiver() {
                    Some((parent, parent_site, _, _)) => (parent, parent_site),
                    None => (instance.receiver_binding()?, instance.receiver_site()),
                };
                if origins.contains_key(&binding) { input_use_pending!("instance-receiver-is-entry") }
                if uses.reads().any(|(site, reference)| site == read_site
                    && *reference == ResolvedLexicalRefV1::Local(binding)) {
                    covered.insert((read_site.clone(), binding));
                }
            }
            let scalar_reads = match original.and_then(|row| row.source.instance()) {
                Some(target) => super::super::borrowed_formal_actuals::source_instance_scalar_reads_v1(
                    self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("borrowed-instance/source-unavailable"))?,
                    &self.borrowed_formal_actuals, catalog, target, self)?,
                None => Vec::new(),
            };
            for (argument, site) in observation.arguments().iter().zip(argument_sites) {
                use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1 as Argument;
                if let (Argument::BorrowedActual { ordinal, site: actual }, Some(original)) = (argument, scalar_static.as_ref()) {
                    if actual != site { return Err(freeze("static-input/qualified-use-identity")); }
                    if let Some((binding, formal)) = Self::original_static_forwarded_use_v1(
                        structure.source, uses, original, *ordinal, site)? {
                        // The containing expression can already own this exact
                        // argument read. Both proofs must retain the same formal;
                        // unrelated pre-covered reads still refuse duplication.
                        if origins.get(&binding) != Some(&formal)
                            || (!covered.insert((site.clone(), binding))
                                && !evaluation_reads.contains(&(site.clone(), binding))) {
                            return Err(freeze("static-input/read-use-duplicate-or-foreign"));
                        }
                        continue;
                    }
                }
                let binding = match argument {
                    Argument::Scalar(binding) => Some(*binding),
                    Argument::BorrowedActual { site: actual, .. } if actual == site => scalar_reads.iter()
                        .find(|(read, _, _)| read == site).map(|(_, binding, _)| *binding),
                    _ => None,
                };
                if let Some(binding) = binding {
                    if origins.contains_key(&binding) { input_use_pending!("call-argument-is-entry") }
                    if !uses.reads().any(|(at, reference)| at == site && *reference == ResolvedLexicalRefV1::Local(binding)) {
                        return Err(freeze("static-input/read-use-duplicate-or-foreign"));
                    }
                    covered.insert((site.clone(), binding));
                }
            }
        }
        // Retained scalar terminals own exact Return reads, including Loop After.
        for terminal in self.terminal_relations_for_owner(contract.owner) {
            if let crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::Value(row) = terminal {
                if let crate::mir::resolved_semantics::home_new_prefix::TerminalReturnedSourceV1::TextResidence { binding, acquisition } = row.returned() {
                    if !uses.corroborates_text_residence_return_v1(row) || origins.contains_key(binding) {
                        input_use_pending!("text-return-origin")
                    }
                    let Some(Ok(exit)) = flow.and_then(|flow| flow.exit_row(row.return_site())) else { input_use_pending!("text-return-cleanup") };
                    if exit.homes().iter().filter(|home| home.binding() == acquisition.binding()).count() != 1
                        || !exit.homes().contains(&crate::mir::resolved_semantics::home_new_prefix::LiveHomeV1::Text(acquisition.clone()))
                        || !covered.insert((row.value_site().clone(), *binding)) {
                        return Err(freeze("static-input/read-use-duplicate-or-foreign"));
                    }
                }
            }
            if let crate::mir::resolved_semantics::home_new_prefix::TerminalRelationV1::I64Scalar(row) = terminal {
                // The same evaluation's references were covered above, including LoopAfter.
                if row.evaluation().is_some() { continue; }
                for (site, reference) in uses.reads().filter(|(site, _)| *site == row.value_site()) {
                    let ResolvedLexicalRefV1::Local(binding) = reference else { input_use_pending!("terminal-nonlocal-reference") };
                    if let Some(original) = row.entry_read() {
                        let required = original.requirement();
                        if !original.signed_only() || !required.corroborates_original_uses_v1(uses)
                            || required.read() != site || required.binding() != *binding
                            || origins.get(binding) != Some(&required.formal().1) {
                            return Err(freeze("static-input/entry-read-source"));
                        }
                    } else if origins.contains_key(binding) { input_use_pending!("terminal-entry-without-read") }
                    covered.insert((site.clone(), *binding));
                }
            }
        }
        for (target_site, target) in uses.assignments() {
            if assignments.contains(target_site) {
                continue;
            }
            let crate::mir::resolved_semantics::ResolvedAssignmentTargetV1::BindingRebind(
                target_binding,
            ) = target
            else {
                continue;
            };
            if origins.contains_key(target_binding)
                || !target_site
                    .node()
                    .segments()
                    .iter()
                    .any(|segment| matches!(
                        segment,
                        crate::mir::resolved_semantics::SourcePathSegmentV1::LoopBody(_)
                    ))
            {
                continue;
            }
            let Some(_value_site) =
                crate::mir::resolved_semantics::assignment_value_sibling_v1(target_site)
            else {
                continue;
            };
            // The borrowed-input census owns formal mutation/escape checks.
            // The selected Loop route owns execution semantics for this exact
            // local rebind; source-flow read closure below proves it cannot
            // carry one of the borrowed entries.
            assignments.insert(target_site.clone());
        }
        if assignments.len() != uses.assignments().len()
            || uses.assignments().iter().any(|(site, _)| !assignments.contains(site)) {
            if crate::config::env::builder_debug_enabled() {
                let missing: Vec<_> = uses.assignments().iter().filter(|(site, _)| !assignments.contains(site)).collect();
                eprintln!("[input-use] owner={:?} missing-assignments={missing:?}", contract.owner);
            }
            input_use_pending!("assignments")
        }
        if let Some(draft) = structure
            .source
            .definitions
            .get(&contract.owner)
            .or_else(|| structure.source.source_only_definitions.get(&contract.owner))
        {
            use crate::mir::resolved_semantics::SourcePathSegmentV1;
            for row in &draft.uses {
                let super::super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1::LoopConditionOperand {
                    loop_site,
                    binary,
                } = &row.kind
                else {
                    continue;
                };
                let loop_path = loop_site.node().segments();
                let binary_path = binary.site().node().segments();
                let read_path = row.site.site().node().segments();
                let exact_condition = binary.owner() == contract.owner
                    && row.site.owner() == contract.owner
                    && row.binding.owner() == contract.owner
                    && row.formal.owner() == contract.owner
                    && origins.get(&row.binding) == Some(&row.formal)
                    && structure
                        .source
                        .source_value_sum_v1(row.formal)
                        .is_some_and(|sum| sum.signed_only())
                    && binary_path.len() == loop_path.len() + 1
                    && binary_path.starts_with(loop_path)
                    && matches!(binary_path.last(), Some(SourcePathSegmentV1::LoopCondition))
                    && read_path.starts_with(binary_path)
                    && uses
                        .reads()
                        .filter(|(site, reference)| {
                            *site == row.site.site()
                                && **reference
                                    == ResolvedLexicalRefV1::Local(row.binding)
                        })
                        .count()
                        == 1;
                if !exact_condition {
                    return Err(freeze("static-input/loop-condition-use-source"));
                }
                covered.insert((row.site.site().clone(), row.binding));
            }
        }
        for (site, reference) in uses.reads() {
            let ResolvedLexicalRefV1::Local(binding) = reference else {
                input_use_pending!("missing-nonlocal-read")
            };
            if covered.contains(&(site.clone(), *binding)) {
                continue;
            }
            if origins.contains_key(binding) {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[entry-input-use] uncovered entry read site={site:?} binding={binding:?}");
                }
                input_use_pending!("missing-read-source")
            }
            if !uses.corroborates_non_entry_source_read_v1(site, *binding, &origins,
                &|read| structure.source.read_consumed_by_scalar_core_v1(contract.owner, read)) {
                if crate::config::env::builder_debug_enabled() {
                    eprintln!("[entry-input-use] uncorroborated non-entry read site={site:?} binding={binding:?}");
                }
                input_use_pending!("missing-read-source")
            }
            covered.insert((site.clone(), *binding));
        }
        Ok(Some(Rc::clone(uses)))
    }

    pub(super) fn borrowed_entry_structure_for_contract(
        &self,
        owner: FunctionOwnerIdV1,
        parameters: &[(u32, BindingRefV1, CallableParameterContractKindV1)],
    ) -> Result<Option<BorrowedEntryStructureV1<'_>>, String> {
        if !parameters
            .iter()
            .any(|(_, _, kind)| kind.is_ordinary_borrowed_handle())
        {
            return Ok(None);
        }
        let source = self
            .borrowed_formal_source
            .as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?
            .as_ref()
            .map_err(Clone::clone)?;
        if let Some(structure) = Self::finite_input_entry_structure_v1(source, self.original_parameter_contracts.as_deref(), owner, parameters)? {
            return Ok(Some(structure));
        }
        let Some(view) = self.checked_entry_owner_view_v1(source, owner)? else {
            // A source use outside the closed transport profile was excluded
            // before selection. This is not retry after a selected failure.
            return Ok(None);
        };
        if view.mixed
            || self.selected_instance_input_protocol_v1(source, owner)?.is_some()
            || self
                .checked_guarded_instance_definition_v1(source, owner)?
                .is_some()
        {
            self.check_original_entry_parameters_v1(owner, parameters)?;
        }
        let definition = view.definition;
        let mut formals = Vec::new();
        for (index, (ordinal, binding, kind)) in parameters.iter().enumerate() {
            if *ordinal as usize != index || binding.owner() != owner {
                return Err(freeze("borrowed-entry/formal-identity"));
            }
            if kind.is_ordinary_borrowed_handle() {
                if definition.origins.get(binding) != Some(binding) {
                    return Err(freeze("borrowed-entry/formal-origin"));
                }
                formals.push((*ordinal, *binding));
            }
        }
        let roots: BTreeSet<_> = definition.origins.values().copied().collect();
        if roots != formals.iter().map(|(_, binding)| *binding).collect() {
            return Err(freeze("borrowed-entry/formal-cardinality"));
        }
        for call in &view.incoming {
            if call.arguments.len() != formals.len()
                || call.arguments.iter().zip(formals.iter()).any(
                    |((ordinal, _, formal), (expected, binding))| {
                        ordinal != expected || formal != binding
                    },
                )
            {
                return Err(freeze("borrowed-entry/incoming-formals"));
            }
        }
        Ok(Some(BorrowedEntryStructureV1 {
            owner, source, evidence: EntryStructureEvidenceV1::Uses(view), formals: formals.into_boxed_slice(),
        }))
    }

    /// Final domain loan only after the SAME Uses owner completed its actuals.
    /// Passive structures/SelectedTagged declarations grant no Text marker.
    pub(super) fn checked_entry_domains_v1(&self, source: &PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1, view: &BorrowedEntryOwnerViewV1<'_>, incoming: &BorrowedEntryActualsV1<'_>,
    ) -> Result<Box<[BorrowedFormalDomainV1]>, String> {
        use crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1 as Mode;
        let catalog = self.original_parameter_contracts.as_deref().ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let contract = contracts.next().ok_or_else(|| freeze("borrowed-entry/entry-owner"))?;
        if contracts.next().is_some() { return Err(freeze("borrowed-entry/original-contract-duplicate")); }
        if contract.mode == Mode::StaticBoxMethod { return self.checked_static_input_domains_for_owner_v1(owner); }
        let sums: Vec<_> = contract.parameters.iter().filter(|row| row.kind.is_ordinary_borrowed_handle())
            .map(|row| (row, source.lend_source_value_sum_v1(row.binding))).collect();
        if !sums.iter().any(|(_, sum)| sum.as_ref().is_some_and(|sum| sum.text_only() || sum.signed_only() || sum.bool_only())) { return Ok(Box::new([])); }
        if !matches!(incoming, BorrowedEntryActualsV1::Executable(rows) if !rows.is_empty())
            || !source.source_incoming.source_registered_owners_v1().contains(&owner)
            || source.source_incoming.sum_vetoed_owners_v1().contains(&owner) {
            return Err(freeze("borrowed-entry/text-domain-unfinished"));
        }
        let originals: Vec<_> = source.source_incoming.source_rows_v1().filter(|row| row.callee == owner).collect();
        if originals.len() != view.incoming.len() || originals.iter().any(|original|
            !view.incoming.iter().any(|row| row.call == original.call && row.callee == original.callee
                && row.arguments == original.arguments && row.source.same_original_source_v1(&original.source))) {
            return Err(freeze("borrowed-entry/text-domain-incoming"));
        }
        // Enrich the existing Uses domain column only where the original
        // source supplies a sum. Unrelated Object formals keep their own
        // authority and never acquire a new scalar-domain prerequisite.
        Ok(sums.into_iter().filter_map(|(formal, sum)| sum.filter(|sum| !sum.alternatives().is_empty())
            .map(|sum| BorrowedFormalDomainV1::from_source_v1(formal.ordinal, formal.binding, sum)))
            .collect::<Vec<_>>().into_boxed_slice())
    }

    pub(super) fn finish_borrowed_entry_source_v1<'a>(
        &'a self, structure: BorrowedEntryStructureV1<'a>,
    ) -> Result<BorrowedOrdinaryEntrySourceRefV1<'a>, String> {
        let (domains, incoming_rows, incoming) = match &structure.evidence {
            EntryStructureEvidenceV1::Uses(view) => {
                let incoming = self.entry_actual_loans_v1(structure.source, structure.owner, view)?;
                (self.checked_entry_domains_v1(structure.source, structure.owner, view, &incoming)?,
                    view.incoming.clone(), incoming)
            }
            EntryStructureEvidenceV1::FiniteInput { uses, incoming, domains } => {
                let catalog = self.original_parameter_contracts.as_deref().ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
                let contract = catalog.iter().find(|row| row.owner == structure.owner)
                    .ok_or_else(|| freeze("borrowed-entry/entry-owner"))?;
                let complete = self.checked_input_use_source_v1(contract)?
                    .ok_or_else(|| format!("{}: owner={:?}",
                        freeze("borrowed-entry/source-use-completion-pending"), structure.owner))?;
                if !Rc::ptr_eq(uses, &complete) { return Err(freeze("static-input/original-use-identity")); }
                let mut actuals = Vec::new();
                for call in incoming {
                    match &call.source {
                        super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Instance(target) => {
                            let (selected, signatures) = self.original_instance_input_issuance.as_ref()
                                .ok_or_else(|| freeze("borrowed-entry/original-issuance-missing"))?;
                            if self.checked_completed_original_instance_target_arguments_v1(target, selected, catalog, signatures)?.is_none() {
                                if crate::config::env::builder_debug_enabled() {
                                    eprintln!("[entry-input] owner={:?} call={:?} pending={:?}", structure.owner, call.call,
                                        self.typed_object_input_completion_diagnostic_v1(structure.owner));
                                }
                                return Err(freeze("borrowed-entry/original-input-not-ready"));
                            }
                        }
                        super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) => {
                            self.checked_completed_static_expression_input_v1(original)?
                                .ok_or_else(|| {
                                    if crate::config::env::builder_debug_enabled() {
                                        eprintln!("[entry-input] owner={:?} static-call={:?}", structure.owner, call.call);
                                    }
                                    freeze("borrowed-entry/original-input-not-ready")
                                })?;
                        }
                        _ => return Err(freeze("borrowed-entry/incoming-source-identity")),
                    }
                    let row = self.borrowed_formal_actuals.get(&call.call)
                        .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?.as_ref().map_err(Clone::clone)?;
                    row.ordered_arguments_for_v1(call)?;
                    let mut represented = BTreeSet::new();
                    if row.opaque_actuals.iter().any(|actual| {
                        let identity = (actual.ordinal, actual.formal);
                        !structure.formals.contains(&identity) || !represented.insert(identity)
                    }) || structure.formals.iter().any(|identity| {
                        !domains.contains_key(&identity.1) && !represented.contains(identity)
                    }) {
                        // Finite scalar input is represented by its checked
                        // ordered argument source and retained domain. It need
                        // not masquerade as an opaque object actual; object
                        // formals still require their exact actual row.
                        return Err(freeze("borrowed-entry/incoming-formals"));
                    }
                    actuals.push((&call.call, row.opaque_actuals.as_ref()));
                }
                let domains = structure.formals.iter().map(|(ordinal, binding)| {
                    let domain = domains.get(binding).ok_or_else(|| freeze("borrowed-entry/domain-formal"))?;
                    Ok(BorrowedFormalDomainV1::from_source_v1(*ordinal, *binding, Rc::clone(domain)))
                }).collect::<Result<Vec<_>, String>>()?.into_boxed_slice();
                (domains, incoming.clone(), BorrowedEntryActualsV1::Executable(actuals.into_boxed_slice()))
            }
        };
        Ok(BorrowedOrdinaryEntrySourceRefV1 {
            owner: structure.owner, formals: structure.formals, source: structure.source,
            domains, evidence: structure.evidence, incoming_rows, incoming,
        })
    }

    pub(super) fn borrowed_ordinary_entry_structure_v1(
        &self,
        input: &SelectedCallableLoweringInputRefV1<'_>,
    ) -> Result<Option<BorrowedEntryStructureV1<'_>>, String> {
        if !matches!(input.semantic(), SelectedCallableSemanticRefV1::Ordinary) {
            return Ok(None);
        }
        let crate::mir::builder::SelectedNormalCallableKeyV1::Cataloged(key) = input.selected_key()
        else {
            return Ok(None);
        };
        let receiver_required = match key.namespace() {
            hakorune_mir_defs::SameModuleCallableNamespaceV1::InstanceBoxMethod => true,
            hakorune_mir_defs::SameModuleCallableNamespaceV1::StaticBoxMethod => false,
            _ => return Ok(None),
        };
        let has_receiver = input.source().function().declaration_sites().any(|site| {
            matches!(
                site,
                crate::mir::resolved_semantics::SourceBindingSiteV1::Receiver
            )
        });
        if has_receiver != receiver_required {
            return Err(freeze("borrowed-entry/source-mode-shape"));
        }
        let parameters = input.parameter_contracts().collect::<Vec<_>>();
        if !parameters
            .iter()
            .any(|(_, _, kind)| kind.is_ordinary_borrowed_handle())
        {
            return Ok(None);
        }
        if !self
            .completion_index
            .get(&input.source().owner())
            .is_some_and(|row| {
                row.as_ref()
                    .is_ok_and(|completion| completion.owner() == input.source().owner())
            })
        {
            return Err(freeze("borrowed-entry/foreign-source-loan"));
        }
        self.borrowed_entry_structure_for_contract(input.source().owner(), &parameters)
    }


}

impl OrdinaryNewClaimLedgerV1 {
    /// Original finite Instance membership selects preparation only. Completion,
    /// all use purposes and executable incoming remain separate prerequisites.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal::lexical_instance_call) fn original_finite_instance_input_request_v1(
        source: &PreparedBorrowedFormalIngressV1,
        catalog: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let Some(contract) = contracts.next() else { return Ok(false) };
        if contracts.next().is_some() { return Err(freeze("borrowed-entry/original-contract-duplicate")); }
        if contract.mode != crate::mir::callable_parameter_contract::CallableParameterDeclarationModeV1::InstanceBoxMethod { return Ok(false); }
        Self::original_finite_input_use_request_v1(source, catalog, owner)
    }

    /// Select the existing finite input's full-use responsibility, independently
    /// of body/actual completion. Admitted Uses owners retain their original authority.
    pub(in crate::mir::normal_callable_semantic_package) fn original_finite_input_use_request_v1(
        source: &PreparedBorrowedFormalIngressV1,
        catalog: &[crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
        owner: FunctionOwnerIdV1,
    ) -> Result<bool, String> {
        let mut contracts = catalog.iter().filter(|row| row.owner == owner);
        let Some(contract) = contracts.next() else { return Ok(false) };
        if contracts.next().is_some() { return Err(freeze("borrowed-entry/original-contract-duplicate")); }
        let parameters: Vec<_> = contract.parameters.iter().map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
        Ok(Self::finite_input_entry_structure_v1(source, Some(catalog), owner, &parameters)?.is_some())
    }

    pub(super) fn finite_entry_for_owner_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Option<BorrowedEntryStructureV1<'_>>, String> {
        let source = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("borrowed-entry/source-missing"))?.as_ref().map_err(Clone::clone)?;
        let Some(contract) = self.original_parameter_contracts.as_deref()
            .and_then(|rows| rows.iter().find(|row| row.owner == owner)) else { return Ok(None) };
        let parameters: Vec<_> = contract.parameters.iter()
            .map(|row| (row.ordinal, row.binding, row.kind.clone())).collect();
        Self::finite_input_entry_structure_v1(source, self.original_parameter_contracts.as_deref(), owner, &parameters)
    }

    /// Only source-scope staging consumes the structure. Object/field consumers
    /// continue to require their original completed use/actual products.
    pub(crate) fn stage_borrowed_ordinary_entry_structure_v1(
        &self, input: &SelectedCallableLoweringInputRefV1<'_>,
    ) -> Result<Option<Box<[(u32, BindingRefV1)]>>, String> {
        let Some(structure) = self.borrowed_ordinary_entry_structure_v1(input)? else { return Ok(None) };
        let formals = structure.formals.clone();
        if matches!(structure.evidence, EntryStructureEvidenceV1::Uses(_)) {
            let entry = self.finish_borrowed_entry_source_v1(structure)?;
            self.stage_borrowed_entry_field_reads_v1(&entry, input)?;
        }
        Ok(Some(formals))
    }
}

#[path = "ordinary_new_borrowed_formal_entry_finite.rs"]
mod finite;

/// Default-off diagnostic: the variant of a checked expression reference.
fn reference_kind_name(
    reference: Option<&crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1>,
) -> &'static str {
    use crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionReferenceV1 as Reference;
    match reference {
        None => "none",
        Some(Reference::StoredMapGet(_)) => "StoredMapGet",
        Some(Reference::DynamicInvocationResult(_)) => "DynamicInvocationResult",
        Some(Reference::StaticText(_)) => "StaticText",
        Some(Reference::Residence(_)) => "Residence",
        Some(Reference::LoopText(_)) => "LoopText",
        Some(Reference::ArgumentEntry(_)) => "ArgumentEntry",
        Some(Reference::DynamicEntry(_)) => "DynamicEntry",
        Some(Reference::LoopDynamic(_)) => "LoopDynamic",
        Some(Reference::FiniteEntry { exclusion: Some(_), .. }) => "FiniteEntry+exclusion",
        Some(Reference::FiniteEntry { .. }) => "FiniteEntry",
        Some(Reference::Scalar(_)) => "Scalar",
    }
}
