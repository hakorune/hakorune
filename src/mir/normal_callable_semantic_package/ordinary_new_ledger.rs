//! Construction, affine takes and root identity access for the New ledger.
use super::*;
use crate::mir::normal_callable_semantic_package::{BirthFormalDeclarationClassV1, BirthFormalUseCoverageV1};

impl OrdinaryNewClaimLedgerV1 {
    /// An original New requires lifecycle context even while its execution
    /// premise is unavailable. Keep that failure in the claim for its consumer;
    /// neither membership nor this query grants construction permission.
    pub(in crate::mir::normal_callable_semantic_package) fn requires_new_context_v1(&self, owner: FunctionOwnerIdV1) -> bool {
        self.claims.borrow().keys().chain(self.result_claims.borrow().keys()).any(|site| site.owner() == owner)
    }

    /// Borrow the original SUM only for this original Birth formal and store.
    /// Its use census stays Unqualified; this is not a physical ABI grant.
    pub(in crate::mir::normal_callable_semantic_package) fn constructor_text_store_sum_v1(
        &self, row: &VerifiedInstanceConstructorSemanticRowV1,
        store: &crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreV1,
    ) -> Option<Rc<super::SourceValueSumV1>> {
        let crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreRhsV1::Parameter { site, binding, provided: None } = store.rhs() else { return None };
        let source = row.construction_source().as_ref().ok()?;
        let (constructor, owner) = source.constructor()?;
        if !constructor.same_as(row.source_id()) || *owner != binding.owner()
            || source.object() != row.object() || !source.stores().iter().any(|original| std::ptr::eq(original, store)) {
            return None;
        }
        let formal = row.formal_contracts().iter().enumerate().find_map(|(index, formal)|
            (formal.binding() == *binding && formal.ordinal() as usize == index).then_some(formal))?;
        if !matches!(formal.declaration(), BirthFormalDeclarationClassV1::Unannotated | BirthFormalDeclarationClassV1::ExactText)
            || !matches!(formal.uses(), BirthFormalUseCoverageV1::UnqualifiedFieldStores { sites } if sites.contains(site)) {
            return None;
        }
        self.borrowed_formal_source.as_ref()?.as_ref().ok()?.lend_source_value_sum_v1(*binding)
            .filter(|sum| sum.text_only())
    }

    /// Select the executable lane for one untyped Birth store from the
    /// retained formal source: a signed source sum initializes I64, while a
    /// complete same-field constructor relation initializes an exact object
    /// residence. No MIR carrier or field spelling selects the tag.
    pub(in crate::mir::normal_callable_semantic_package) fn constructor_dynamic_store_mode_v1(
        &self,
        row: &VerifiedInstanceConstructorSemanticRowV1,
        store: &crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreV1,
    ) -> Option<crate::mir::instruction::FieldSetModeV1> {
        use crate::mir::instruction::{
            DynamicFieldResidenceV1 as Residence, DynamicFieldSetContractV1 as Contract,
            DynamicFieldValueTagV1 as Tag, DynamicFieldWriteIntentV1 as Intent,
            FieldSetModeV1 as Mode,
        };
        use crate::mir::normal_callable_semantic_package::instance_construction::ConstructionStoreRhsV1;
        use crate::mir::normal_callable_semantic_package::instance_constructor_semantic::BirthFormalUseCoverageV1 as Uses;

        let ConstructionStoreRhsV1::Parameter { site, binding, provided: None } = store.rhs() else {
            return store_mode_declined(0);
        };
        let source = row.construction_source().as_ref().ok()?;
        let (constructor, owner) = source.constructor()?;
        if !constructor.same_as(row.source_id()) || *owner != binding.owner()
            || source.object() != row.object()
            || !source.stores().iter().any(|original| std::ptr::eq(original, store))
            || source.stores().iter().filter(|other| other.field() == store.field()).count() != 1
        {
            return store_mode_declined(1);
        }
        let formal = row.formal_contracts().iter().find(|formal| formal.binding() == *binding).or_else(|| { let _ = store_mode_declined::<()>(50); None })?;
        let Uses::UnqualifiedFieldStores { sites } = formal.uses() else { return None };
        if formal.binding() != *binding || formal.ordinal() as usize >= row.source_arity() as usize
            || sites.len() != 1 || !sites.contains(site)
        {
            return store_mode_declined(2);
        }

        let mut child = None;
        let mut nullable = false;
        let mut outer_sites = std::collections::BTreeSet::new();
        let relations: Vec<_> = self.callable_result_classes.construction_child_relations()
            .filter(|relation| relation.is_unqualified_dynamic_v1()
                && relation.field() == store.field()
                && relation.constructor_source().same_as(constructor)
                && relation.constructor_owner() == *owner
                && relation.formal_binding() == *binding
                && relation.formal_ordinal() == formal.ordinal())
            .collect();
        for relation in &relations {
            if relation.field().object() != row.object()
                || !outer_sites.insert(relation.outer().clone())
                || relation.unqualified_field_source_v1(row.box_name()).is_err()
                || relation.witnesses().is_empty()
            {
                return store_mode_declined(3);
            }
            for witness in relation.witnesses() {
                match witness.origin() {
                    ResultValueOriginV1::Null => {
                        nullable = true;
                    }
                    ResultValueOriginV1::Fresh(_) => {}
                    _ => return None,
                }
            }
            if child.is_some_and(|prior| prior != relation.child()) {
                return store_mode_declined(4);
            }
            child = Some(relation.child());
        }
        if let Some(child) = child {
            let tag = if nullable { Tag::NullableObject } else { Tag::Object };
            let contract = Contract::issue(Intent::Initialize, tag, Residence::ObjectHome(child))?;
            return Some(Mode::DynamicValue(contract));
        }

        self.borrowed_formal_source.as_ref()?.as_ref().ok()?
            .lend_source_value_sum_v1(*binding)
            .filter(|sum| sum.signed_only())
            .map(|_| Mode::DynamicI64Initialize)
    }

    /// Existing source New claims already cloned initial eligibility. Update
    /// both families and every early ABI copy before any physical take,
    /// retaining all other core products.
    pub(in crate::mir::normal_callable_semantic_package) fn install_late_constructor_plan_v1(
        &mut self,
        row: &VerifiedInstanceConstructorSemanticRowV1,
        plan: &crate::mir::normal_callable_semantic_package::instance_construction::ConstructionPlanV1,
    ) -> Result<(), String> {
        if !self.local_commits.get_mut().is_empty() || !self.provider_births.get_mut().is_empty() {
            return Err("[freeze:contract][constructor-unit/late-after-take]".into());
        }
        let matches = |core: &OrdinaryNewClaimCoreV1| -> Result<bool, String> {
            if !core.box_source.same_source_as(row.box_source_v1())
                || &core.construction != row.construction() { return Ok(false); }
            let OrdinaryNewConstructorDispositionV1::Birth(birth) = &core.constructor else {
                return Err("[freeze:contract][constructor-unit/new-birth-missing]".into());
            };
            if !birth.source_id().same_as(row.source_id())
                || row.published_birth_key() != Some(birth.target_ref())
                || core.object != row.object() || core.arity != row.source_arity() as usize
                || core.class.as_ref() != row.box_name() || plan.object() != core.object
                || !matches!((&core.construction_source, row.construction_source()), (Ok(core), Ok(source))
                    if Rc::ptr_eq(core, source) && Rc::ptr_eq(core, plan.source_v1())) {
                return Err("[freeze:contract][constructor-unit/new-source-drift]".into());
            }
            Ok(true)
        };
        // Preflight both maps before either mutates.
        for claim in self.claims.get_mut().values() { matches(&claim.core)?; }
        for claim in self.result_claims.get_mut().values() { matches(&claim.core)?; }
        // Providers retain another early handoff in their destination-less
        // index. Prepare every replacement before mutating any claim or index.
        let handoffs = self.birth_abi_handoffs.get_mut().iter()
            .filter(|(_, handoff)| handoff.source_id().same_as(row.source_id()))
            .map(|(site, handoff)| Ok((site.clone(), handoff.with_late_construction_plan_v1(row, plan)?)))
            .collect::<Result<Vec<_>, String>>()?;
        let providers = self.birth_site_index.get_mut().iter()
            .filter(|(_, (_, handoff))| handoff.source_id().same_as(row.source_id()))
            .map(|(site, (_, handoff))| Ok((site.clone(), handoff.with_late_construction_plan_v1(row, plan)?)))
            .collect::<Result<Vec<_>, String>>()?;
        let mut updated_local = 0usize;
        let mut updated_result = 0usize;
        for claim in self.claims.get_mut().values_mut() {
            if matches(&claim.core)? { claim.core.construction = Ok(plan.clone()); updated_local += 1; }
        }
        for claim in self.result_claims.get_mut().values_mut() {
            if matches(&claim.core)? { claim.core.construction = Ok(plan.clone()); updated_result += 1; }
        }
        for (site, handoff) in handoffs {
            *self.birth_abi_handoffs.get_mut().get_mut(&site).expect("preflight retained ABI site") = handoff;
        }
        for (site, handoff) in providers {
            self.birth_site_index.get_mut().get_mut(&site).expect("preflight retained provider site").1 = handoff;
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn retain_original_parameter_contracts_v1(
        &mut self,
        contracts: &Rc<[super::super::model::OwnedCallableParameterContractDeclarationV1]>,
    ) -> Result<(), String> {
        if self.original_parameter_contracts.is_some() {
            return Err(
                "[freeze:contract][ordinary-new/borrowed-entry/catalog-already-retained]"
                    .to_owned(),
            );
        }
        self.original_parameter_contracts = Some(Rc::clone(contracts));
        Ok(())
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn original_parameter_contracts_v1(
        &self,
    ) -> Option<&Rc<[super::super::model::OwnedCallableParameterContractDeclarationV1]>> {
        self.original_parameter_contracts.as_ref()
    }

    pub(in crate::mir::normal_callable_semantic_package) fn requires_map_lifecycle_consumer(
        &self,
    ) -> bool {
        fn carries_map(
            flow: &crate::mir::resolved_semantics::home_new_prefix::RootHomeFlow,
        ) -> bool {
            !flow.maps().is_empty()
                || flow.local_calls().iter().any(|call| {
                    call.result()
                    == crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Map
                })
        }
        let indexed = self.completion_index.values().any(|row| {
            row.as_ref()
                .ok()
                .and_then(|completion| completion.cleanup().root_flow())
                .is_some_and(carries_map)
        });
        indexed
            || self
                .root_completion
                .as_ref()
                .and_then(|row| row.as_ref().ok())
                .and_then(|completion| completion.cleanup().root_flow())
                .is_some_and(carries_map)
    }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn root_completion_for_test(
        &self,
    ) -> &crate::mir::resolved_control_flow::VerifiedFunctionCompletionV1 {
        self.root_completion
            .as_ref()
            .expect("selected root")
            .as_ref()
            .expect("verified completion")
            .as_ref()
    }
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn pending_claims_for_test(
        &self,
    ) -> std::cell::Ref<'_, BTreeMap<OwnedExprSiteV1, OrdinaryNewAdmissionClaimV1>> {
        self.claims.borrow()
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn pending_result_claims_for_test(
        &self,
    ) -> std::cell::Ref<'_, BTreeMap<OwnedExprSiteV1, OrdinaryNewResultClaimV1>> {
        self.result_claims.borrow()
    }

    /// The sealed `ArrayBox` element-integer census for one box — the
    /// canonical fields whose whole-Box write inventory proved integer
    /// stores. Missing/empty means unproven; consumers see no row at all.
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn array_i64_fields_for_test(
        &self,
        box_name: &str,
    ) -> usize {
        self.array_i64_fields.get(box_name).map_or(0, BTreeSet::len)
    }

    /// The per-owner borrowed-return source proofs folded to their i64
    /// outcome: `Ok(())` is a sealed i64 return source, `Err` is the
    /// named unavailability the proof recorded. Owners outside the armed
    /// call graph carry no row at all.
    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn borrowed_i64_results_for_test(
        &self,
    ) -> Vec<(
        crate::mir::resolved_semantics::FunctionOwnerIdV1,
        Result<(), String>,
    )> {
        self.borrowed_i64_results
            .iter()
            .map(|(owner, row)| {
                (
                    *owner,
                    row.as_ref()
                        .map_err(Clone::clone)
                        .and_then(|proof| proof.require_source_i64_v1()),
                )
            })
            .collect()
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn local_commit_complete_for_test(
        &self,
        site: &OwnedExprSiteV1,
    ) -> bool {
        self.local_commits
            .borrow()
            .get(site)
            .is_some_and(|row| row.is_complete())
    }

    pub(crate) fn issue(
        claims: Box<[OrdinaryNewAdmissionClaimV1]>,
        result_claims: Box<[OrdinaryNewResultClaimV1]>,
        ordinary_box_names: Box<[Box<str>]>,
    ) -> Self {
        Self {
            claims: RefCell::new(
                claims
                    .into_vec()
                    .into_iter()
                    .map(|claim| (claim.site().clone(), claim))
                    .collect(),
            ),
            result_claims: RefCell::new(
                result_claims
                    .into_vec()
                    .into_iter()
                    .map(|claim| (claim.core.site.clone(), claim))
                    .collect(),
            ),
            ordinary_box_names,
            original_parameter_contracts: None,
            original_instance_input_issuance: None,
            array_holder_lifecycles: RefCell::new(BTreeMap::new()),
            local_commits: RefCell::new(BTreeMap::new()),
            root_validation: RefCell::new(local_commit::RootNewValidation::Unregistered),
            main_arguments: RefCell::new(None),
            child_physical_validation: RefCell::new(BTreeMap::new()),
            root_exits: RefCell::new(BTreeMap::new()),
            root_local_call_bindings: RefCell::new(BTreeMap::new()),
            loop_cond_after_roles: RefCell::new(BTreeMap::new()),
            lifecycle_local_call_sites: RefCell::new(BTreeMap::new()),
            map_read_bindings: RefCell::new(BTreeMap::new()),
            root_instance_calls: RefCell::new(BTreeMap::new()),
            lexical_source_targets: None,
            source_dynamic_member_calls: None,
            borrowed_formal_source: None,
            selected_loop_products: BTreeMap::new(),
            loop_static_source_loans: RefCell::new(BTreeMap::new()),
            loop_static_body_scalar_sources: RefCell::new(BTreeMap::new()),
            selected_loop_body_packets: RefCell::new(BTreeMap::new()),
            loop_static_source_loop_sites: BTreeSet::new(),
            preissued_static_i64_products: RefCell::new(BTreeMap::new()),
            loop_entry_static_i64_source_loans: RefCell::new(BTreeMap::new()),
            loop_tail_static_i64_source_loans: RefCell::new(BTreeMap::new()),
            loop_static_home_neutral: RefCell::new(BTreeMap::new()),
            loop_static_tagged_entry: RefCell::new(BTreeMap::new()),
            loop_static_i64_results: RefCell::new(BTreeMap::new()),
            loop_static_packet_sources: RefCell::new(BTreeMap::new()),
            borrowed_static_source_sites: None,
            borrowed_formal_actuals: BTreeMap::new(),
            borrowed_i64_results: lexical_instance_call::BorrowedResultCohortV1::default(),
            borrowed_entry_values: RefCell::new(BTreeMap::new()),
            scalar_operation_appends: RefCell::new(BTreeMap::new()),
            scalar_operation_consumers: RefCell::new(BTreeMap::new()),
            lexical_instance_calls: RefCell::new(BTreeMap::new()),
            root_instance_call_expected: RefCell::new(BTreeSet::new()),
            field_reads: RefCell::new(BTreeMap::new()),
            argument_field_reads: RefCell::new(BTreeMap::new()),
            local_field_reads: RefCell::new(BTreeMap::new()),
            birth_abi_handoffs: RefCell::new(BTreeMap::new()),
            birth_site_index: RefCell::new(BTreeMap::new()),
            provider_births: RefCell::new(Vec::new()),
            field_write_claims: field_write_claim::OrdinaryNewFieldWriteClaimsV1::default(),
            owned_field_children: BTreeMap::new(),
            callable_result_classes: result_class_claim::OrdinaryNewResultClassClaimsV1::new(),
            array_i64_fields: BTreeMap::new(),
            receiver_call_observations: BTreeMap::new(),
            terminal_relation: Rc::new(BTreeMap::new()),
            terminal_relation_index: BTreeMap::new(),
            normal_return_dispositions: None,
            terminal_scalar_literal_value: RefCell::new(BTreeMap::new()),
            terminal_scalar_literal_values: RefCell::new(BTreeMap::new()),
            terminal_i64_field_value: RefCell::new(BTreeMap::new()),
            terminal_i64_field_values: RefCell::new(BTreeMap::new()),
            terminal_result_progress: RefCell::new(BTreeMap::new()),
            root_completion: None,
            completion_index: BTreeMap::new(),
            main_arguments_source: None,
            app_main_identity: None,
            app_main_catalog_key: None,
        }
    }

    pub(crate) fn retain_source_dynamic_member_calls_v1(
        &mut self,
        calls: &[crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1],
    ) -> Result<(), &'static str> {
        if self.source_dynamic_member_calls.is_some() {
            return Err("Dynamic source-call catalog already retained");
        }
        let mut seen = BTreeSet::new();
        if calls
            .iter()
            .any(|call| !seen.insert((call.owner(), call.call_site().clone())))
        {
            return Err("duplicate Dynamic source-call site");
        }
        self.source_dynamic_member_calls = Some(calls.to_vec().into_boxed_slice());
        Ok(())
    }

    pub(in crate::mir) fn source_dynamic_member_call_v1(
        &self,
        owner: FunctionOwnerIdV1,
        site: &crate::mir::resolved_semantics::SourceExprSiteV1,
    ) -> Result<
        Option<crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1>,
        &'static str,
    > {
        let Some(calls) = self.source_dynamic_member_calls.as_deref() else {
            return Ok(None);
        };
        let mut matches = calls
            .iter()
            .filter(|call| call.owner() == owner && call.call_site() == site);
        let Some(call) = matches.next() else {
            return Ok(None);
        };
        if matches.next().is_some() {
            return Err("duplicate Dynamic source-call owner/site");
        }
        Ok(Some(call.clone()))
    }

    pub(crate) fn try_take(
        &self,
        site: &OwnedExprSiteV1,
        class: &str,
        arity: usize,
    ) -> Result<Option<OrdinaryNewAdmissionClaimV1>, OrdinaryNewClaimTakeErrorV1> {
        if !self
            .ordinary_box_names
            .iter()
            .any(|name| name.as_ref() == class)
        {
            return Ok(None);
        }
        let mut claims = self.claims.borrow_mut();
        let claim = claims
            .get(site)
            .ok_or(OrdinaryNewClaimTakeErrorV1::Unavailable)?;
        if claim.class() != class || claim.arity() != arity {
            return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
        }
        let mut commits = self.local_commits.borrow_mut();
        if let Ok(prefix) = &claim.home_prefix {
            if prefix.destination() != claim.destination
                || prefix.required_unwind() != site
                || prefix
                    .prior_homes()
                    .iter()
                    .any(|binding| local_commit::installed_source_home(&commits, binding).is_err())
            {
                return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
            }
        }
        let birth_target = match &claim.core.constructor {
            OrdinaryNewConstructorDispositionV1::NoBirthZero => None,
            OrdinaryNewConstructorDispositionV1::Birth(recipe) => Some(recipe.target_ref().clone()),
        };
        let birth_abi = self.birth_abi_handoffs.borrow_mut().remove(site);
        if birth_abi.as_ref().map(BirthAbiHandoffV1::target) != birth_target.as_ref() {
            return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
        }
        commits.insert(
            site.clone(),
            local_commit::LocalCommitV1::Ordinary(local_commit::NewLocalCommitV1::pending(
                claim.destination,
                claim.declaration.clone(),
                claim.home_prefix.clone(),
                claim.box_source().clone(),
                claim.core.construction.clone(),
                claim.core.construction_source.clone(),
                claim.core.allocation_execution.clone(),
                claim.core.object,
                claim.core.destruction,
                birth_target,
                birth_abi,
                claim.core.argument_rows.clone(),
                claim.core.children.clone(),
            )),
        );
        Ok(Some(
            claims
                .remove(site)
                .expect("claim remained present after the checked lookup"),
        ))
    }

    /// Whether the callee's result-position claim either still awaits
    /// take or already committed — the co-seal coverage arm for a
    /// `Construction` call result uses it as the transfer proof. The
    /// claim row itself stays affine; this peek consumes nothing.
    pub(crate) fn result_transfer_proven(&self, site: &OwnedExprSiteV1) -> bool {
        self.result_claims.borrow().contains_key(site) || self.has_result_new_commit(site)
    }

    /// Whether a taken return-position commit row exists at this exact
    /// site. Used by the caller's completion-forwarding gate — a `Value`
    /// segment is shared by assignment/print/nowait children that hold no
    /// claim, so shape alone cannot decide.
    pub(crate) fn has_result_new_commit(&self, site: &OwnedExprSiteV1) -> bool {
        matches!(
            self.local_commits.borrow().get(site),
            Some(local_commit::LocalCommitV1::Result(_))
        )
    }

    /// Affine take of a return-position claim. Absence is `Ok(None)` —
    /// non-return `new` positions and uncovered classes stay outside this
    /// lane and keep their existing terminal.
    pub(crate) fn try_take_result(
        &self,
        site: &OwnedExprSiteV1,
        class: &str,
        arity: usize,
    ) -> Result<Option<OrdinaryNewResultClaimV1>, OrdinaryNewClaimTakeErrorV1> {
        if !self
            .ordinary_box_names
            .iter()
            .any(|name| name.as_ref() == class)
        {
            return Ok(None);
        }
        let mut claims = self.result_claims.borrow_mut();
        let Some(claim) = claims.get(site) else {
            return Ok(None);
        };
        if claim.site() != site
            || claim.class() != class
            || claim.arity() != arity
            || claim.class() != claim.box_source().name()
        {
            return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
        }
        let mut commits = self.local_commits.borrow_mut();
        if commits.contains_key(site) {
            return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
        }
        if let Ok(prefix) = &claim.home_prefix {
            if prefix.required_unwind() != site
                || prefix
                    .prior_homes()
                    .iter()
                    .any(|binding| local_commit::installed_source_home(&commits, binding).is_err())
            {
                return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
            }
        }
        let birth_target = match &claim.core.constructor {
            OrdinaryNewConstructorDispositionV1::NoBirthZero => None,
            OrdinaryNewConstructorDispositionV1::Birth(recipe) => Some(recipe.target_ref().clone()),
        };
        let birth_abi = self.birth_abi_handoffs.borrow_mut().remove(site);
        if birth_abi.as_ref().map(BirthAbiHandoffV1::target) != birth_target.as_ref() {
            return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
        }
        commits.insert(
            site.clone(),
            local_commit::LocalCommitV1::Result(local_commit::NewResultCommitV1::pending(
                site.clone(),
                claim.arity(),
                claim.home_prefix.clone(),
                claim.box_source().clone(),
                claim.core.construction.clone(),
                claim.core.object,
                claim.core.destruction,
                birth_target,
                birth_abi,
                claim.core.argument_rows.clone(),
                claim.core.children.clone(),
            )),
        );
        Ok(Some(claims.remove(site).expect(
            "result claim remained present after the checked lookup",
        )))
    }

    /// Affine take of a verified `Birth` recipe for a non-`[Body,
    /// Initializer]` `new` site.  The index admits no destination or lifecycle
    /// authority; a missing entry is `Ok(None)` and leaves the caller's
    /// existing terminal unchanged.
    pub(crate) fn take_birth_site_recipe(
        &self,
        site: &OwnedExprSiteV1,
        class: &str,
        arity: usize,
    ) -> Result<
        Option<(VerifiedOrdinaryNewBirthRecipeV1, BirthAbiHandoffV1)>,
        OrdinaryNewClaimTakeErrorV1,
    > {
        {
            let index = self.birth_site_index.borrow();
            let Some((recipe, _)) = index.get(site) else {
                return Ok(None);
            };
            if recipe.target_ref().owner() != class
                || usize::try_from(recipe.target_ref().arity()).ok() != Some(arity)
            {
                return Err(OrdinaryNewClaimTakeErrorV1::Mismatch);
            }
        }
        Ok(Some(
            self.birth_site_index
                .borrow_mut()
                .remove(site)
                .expect("birth-site recipe remained present after the checked lookup"),
        ))
    }

    /// The sealed owned-field inventory of one canonical object. Rows
    /// exist for admitted owned-disposition objects and for nested
    /// `OwnedArrayFieldsNoHook` provider children sealed through their
    /// parent claims; `Some(None)` marks an owned object whose residences
    /// stayed unproven — consumers must not release its slots as owned.
    pub(crate) fn owned_field_children_for(
        &self,
        object: hakorune_mir_defs::CanonicalObjectIdV1,
    ) -> Option<Option<&[OwnedFieldChildV1]>> {
        self.owned_field_children
            .get(&object)
            .map(|children| children.as_deref())
    }

    #[cfg(test)]
    pub(in crate::mir::normal_callable_semantic_package) fn field_write_sources_for_test(
        &self,
    ) -> &[field_write_claim::ObservedFieldWriteV1] {
        self.field_write_claims.writes()
    }

    /// Original field/writer/constructor correspondence, without a value-class
    /// or execution grant. Numeric preservation must close every writer later.
    pub(crate) fn field_value_source(
        &self, class: &str, field: &str,
    ) -> Option<&Result<Rc<field_write_claim::DeclaredFieldValueSourceV1>, field_write_claim::FieldValueSourceIssueV1>> {
        self.field_write_claims.value_source(class, field)
    }

    /// Class provenance for `me.f` reads: the agreed `new` class of field
    /// `field` on `owner_box`, when every package write to that field is an
    /// attributed `me.` write of one ordinary box.
    pub(crate) fn field_write_claim(&self, owner_box: &str, field: &str) -> Option<&str> {
        self.field_write_claims
            .get(&(owner_box.into(), field.into()))
            .map(|class| class.as_ref())
    }

    /// Result-class provenance for one selected callable: the agreed `new`
    /// class constructed by every `return`, when the body cannot exit
    /// normally without executing one. A `NullableObject` claim is never
    /// returned here — Handle/lifecycle consumers only see definite object
    /// results, so a callable that can `return null` stays unclaimed for
    /// owned-result purposes.
    pub(crate) fn callable_result_class(
        &self,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    ) -> Option<&str> {
        match self.callable_result_classes.get(key) {
            Some(result_class_claim::OrdinaryNewResultClassV1::Object(class)) => {
                Some(class.as_ref())
            }
            _ => None,
        }
    }

    /// The live-arm class of a `NullableObject` result claim — the box the
    /// non-`Void` exit constructs. `Object` claims and unclaimed callables
    /// answer `None`; consumers of the checked-release lane read only this.
    pub(crate) fn nullable_result_class(
        &self,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    ) -> Option<&str> {
        match self.callable_result_classes.get(key) {
            Some(result_class_claim::OrdinaryNewResultClassV1::NullableObject(class)) => {
                Some(class.as_ref())
            }
            _ => None,
        }
    }

    /// Consumes no source product: it only checks that the selected emitter is
    /// at the exact Completion-backed bare-return site already retained here.
    pub(crate) fn prepare_terminal_unit_return(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        site: &SourceNodeSiteV1,
    ) -> Result<bool, String> {
        if self.root_owner() != Some(owner) && self.unit_completion_source_v1(owner)?.is_none() {
            return Ok(false);
        }
        let stmt_site = SourceStmtSiteV1::from_node(site.clone());
        let Some(TerminalRelationV1::Unit(relation)) =
            self.terminal_relation_for_owner_at(owner, &stmt_site)
        else {
            return Ok(false);
        };
        let Some(completion) = self.completion_for_owner(owner) else {
            return Err(
                "[freeze:contract][ordinary-new/unit-return-completion-missing]".to_owned(),
            );
        };
        if relation.owner() != owner
            || completion.owner() != owner
            || !completion.explicit_sites().contains(&stmt_site)
        {
            return Err("[freeze:contract][ordinary-new/unit-return-source-drift]".to_owned());
        }
        Ok(true)
    }

    /// Retain the same co-sealed Main key alongside its declaration identity.
    /// Result Facts must not be selected by a callee key or by a class scan.
    pub(in crate::mir::normal_callable_semantic_package) fn retain_app_main_source_v1(
        &mut self,
        source: Option<&crate::mir::builder::AppMainCatalogCoSealV1>,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        contracts: &[super::super::model::OwnedCallableParameterContractDeclarationV1],
    ) -> Result<(), OrdinaryNewCoSealIssueV1> {
        self.main_arguments_source = lexical_instance_call::borrow_app_main_source_v1(batch, source)?
            .map(|loan| loan.arguments_source_v1(batch, contracts)).transpose()
            .map_err(|reason| OrdinaryNewCoSealIssueV1::CompletionIndexRetention { reason })?;
        self.app_main_identity = source.map(|main| main.parser_identity().clone());
        self.app_main_catalog_key = source.map(|main| main.catalog_key().clone());
        Ok(())
    }

    pub(crate) fn register_app_main_root(
        &self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        identity: &crate::parser::CallableDeclarationIdentityV1,
    ) -> Result<(), String> {
        let Some(expected) = self.app_main_identity.as_ref() else {
            return Err("[freeze:contract][ordinary-new/app-main-identity-missing]".to_owned());
        };
        if !expected.same_as(identity) {
            return Err("[freeze:contract][ordinary-new/app-main-identity-mismatch]".to_owned());
        }
        self.register_new_root(owner)
    }

    /// Whether this declaration identity is the co-sealed App Main. The
    /// same-source dependency harness uses it to skip root registration
    /// for non-main owners — `register_new_root` would reject them
    /// against the root completion anyway.
    #[cfg(test)]
    pub(crate) fn is_app_main_identity(
        &self,
        identity: &crate::parser::CallableDeclarationIdentityV1,
    ) -> bool {
        self.app_main_identity
            .as_ref()
            .is_some_and(|expected| expected.same_as(identity))
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Passive caller-branded return origins; never a Home or emission capability.
    pub(in crate::mir::normal_callable_semantic_package) fn callable_result_origins(
        &self,
        key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1,
    ) -> Option<&[result_class_claim::ResultExitOriginV1]> {
        self.callable_result_classes.outcomes(key)
    }
}

/// Default-off (builder debug flag): which numbered check declined a dynamic constructor store mode.
#[inline(never)]
fn store_mode_declined<T>(check: u32) -> Option<T> {
    if crate::config::env::builder_debug_enabled() { eprintln!("[constructor-store-mode] declined check={check}"); }
    None
}
