//! Construction, affine takes and root identity access for the New ledger.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
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
            local_commits: RefCell::new(BTreeMap::new()),
            root_validation: RefCell::new(local_commit::RootNewValidation::Unregistered),
            child_physical_validation: RefCell::new(BTreeMap::new()),
            root_exits: RefCell::new(BTreeMap::new()),
            root_local_call_bindings: RefCell::new(BTreeMap::new()),
            lifecycle_local_call_sites: RefCell::new(BTreeMap::new()),
            map_read_bindings: RefCell::new(BTreeMap::new()),
            root_instance_calls: RefCell::new(BTreeMap::new()),
            lexical_source_targets: None,
            borrowed_formal_source: None,
            loop_static_source_loans: RefCell::new(BTreeMap::new()),
            loop_static_source_loop_sites: BTreeSet::new(),
            loop_entry_static_i64_source_loans: RefCell::new(BTreeMap::new()),
            loop_tail_static_i64_source_loans: RefCell::new(BTreeMap::new()),
            loop_static_home_neutral: RefCell::new(BTreeMap::new()),
            loop_static_tagged_entry: RefCell::new(BTreeMap::new()),
            loop_static_i64_results: RefCell::new(BTreeMap::new()),
            loop_static_packet_sources: RefCell::new(BTreeMap::new()),
            borrowed_static_source_sites: None,
            borrowed_formal_actuals: BTreeMap::new(),
            borrowed_i64_results: BTreeMap::new(),
            borrowed_entry_values: RefCell::new(BTreeMap::new()),
            lexical_instance_calls: RefCell::new(BTreeMap::new()),
            root_instance_call_expected: RefCell::new(BTreeSet::new()),
            field_reads: RefCell::new(BTreeMap::new()),
            argument_field_reads: RefCell::new(BTreeMap::new()),
            local_field_reads: RefCell::new(BTreeMap::new()),
            birth_abi_handoffs: RefCell::new(BTreeMap::new()),
            birth_site_index: RefCell::new(BTreeMap::new()),
            provider_births: RefCell::new(Vec::new()),
            field_write_claims: BTreeMap::new(),
            owned_field_children: BTreeMap::new(),
            callable_result_classes: result_class_claim::OrdinaryNewResultClassClaimsV1::new(),
            array_i64_fields: BTreeMap::new(),
            receiver_call_observations: BTreeMap::new(),
            terminal_relation: Rc::new(BTreeMap::new()),
            terminal_relation_index: BTreeMap::new(),
            normal_return_dispositions: None,
            terminal_integer_literal_value: RefCell::new(BTreeMap::new()),
            terminal_integer_literal_values: RefCell::new(BTreeMap::new()),
            terminal_i64_field_value: RefCell::new(BTreeMap::new()),
            terminal_i64_field_values: RefCell::new(BTreeMap::new()),
            terminal_result_progress: RefCell::new(BTreeMap::new()),
            root_completion: None,
            completion_index: BTreeMap::new(),
            app_main_identity: None,
            app_main_catalog_key: None,
        }
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
                    .any(|binding| local_commit::installed_home(&commits, *binding).is_err())
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
                    .any(|binding| local_commit::installed_home(&commits, *binding).is_err())
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
        if self.root_owner() != Some(owner) {
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
    ) {
        self.app_main_identity = source.map(|main| main.parser_identity().clone());
        self.app_main_catalog_key = source.map(|main| main.catalog_key().clone());
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
