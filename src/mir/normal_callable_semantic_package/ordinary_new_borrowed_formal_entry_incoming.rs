//! Complete incoming validation for the existing borrowed entry owner.
//! The default requires execution globally; only qualified Static scopes execution.
use super::*;

impl OrdinaryNewClaimLedgerV1 {
    /// Lend completed finite inputs through the same entry-use and incoming
    /// closure. This is an actual loan, never an artificial legacy Uses draft.
    pub(super) fn completed_finite_entry_call_actuals_v1(
        &self, row: &crate::mir::normal_callable_semantic_package::LexicalInstanceCallDispositionRowV1,
    ) -> Result<Option<&[PreparedBorrowedFormalActualV1]>, String> {
        let source = self.borrowed_formal_source.as_ref().ok_or_else(|| freeze("borrowed-call/source-missing"))?
            .as_ref().map_err(Clone::clone)?;
        let catalog = self.original_parameter_contracts.as_deref().ok_or_else(|| freeze("borrowed-entry/original-catalog-missing"))?;
        if !Self::original_finite_instance_input_request_v1(source, catalog, row.callee_owner())? { return Ok(None); }
        let contract = catalog.iter().find(|contract| contract.owner == row.callee_owner())
            .ok_or_else(|| freeze("borrowed-entry/entry-owner"))?;
        let parameters: Vec<_> = contract.parameters.iter().map(|formal|
            (formal.ordinal, formal.binding, formal.kind.clone())).collect();
        let entry = self.borrowed_entry_source_for_contract(row.callee_owner(), &parameters)?
            .ok_or_else(|| freeze("borrowed-call/incoming-missing"))?;
        let mut incoming = entry.incoming_rows.iter().copied().filter(|call| &call.call == row.call_site());
        let call = incoming.next().ok_or_else(|| freeze("borrowed-call/incoming-missing"))?;
        if incoming.next().is_some() || call.callee != row.callee_owner()
            || call.source.require_instance()? != row.source_target()
            || row.argument_sites().len() != row.target().arity() as usize
            || call.arguments.iter().any(|(ordinal, site, _)| row.argument_sites().get(*ordinal as usize) != Some(site)) {
            return Err(freeze("borrowed-call/disposition-identity"));
        }
        let actuals = self.borrowed_formal_actuals.get(row.call_site())
            .ok_or_else(|| freeze("borrowed-call/incoming-missing"))?.as_ref().map_err(Clone::clone)?;
        actuals.ordered_arguments_for_v1(call)?;
        Ok(Some(&actuals.opaque_actuals))
    }


    pub(super) fn checked_borrowed_entry_incoming_core_v1<'a>(
        &'a self,
        source: &'a PreparedBorrowedFormalIngressV1,
        owner: FunctionOwnerIdV1,
        scoped_static: bool,
    ) -> Result<Box<[(&'a OwnedExprSiteV1, &'a [PreparedBorrowedFormalActualV1])]>, String> {
        let mut seen = BTreeSet::new();
        let mut incoming = Vec::new();
        let owner_view = self.checked_entry_owner_view_v1(source, owner)?;
        let mut calls: Vec<_> = source.incoming.iter().collect();
        if let Some(cohort) = source.target_static.get(&owner) {
            calls.extend(cohort.incoming.iter());
        } else if source.source_only_definitions.contains_key(&owner) {
            if let Some(view) = &owner_view {
                calls.retain(|call| call.callee != owner);
                calls.extend(view.incoming.iter().copied());
            }
        }
        if scoped_static {
            calls = self.original_static_execution_calls_v1(source, owner, calls)?;
        }
        let execution = scoped_static
            .then(|| self.checked_static_execution_owners_v1(source, owner, &calls))
            .transpose()?;
        for call in calls {
            let definition = if let Some(definition) = source.definitions.get(&call.callee) {
                Some(definition)
            } else if call.callee == owner {
                owner_view.as_ref().map(|view| view.definition)
            } else if scoped_static {
                self.checked_mixed_instance_entry_v1(source, call.callee)?
                    .map(|view| view.definition)
            } else {
                None
            };
            let Some(definition) = definition else {
                return Err(freeze("borrowed-entry/incoming-identity"));
            };
            if !seen.insert(&call.call) {
                return Err(freeze("borrowed-entry/incoming-identity"));
            }
            let target_roots: BTreeSet<_> = definition.origins.values().copied().collect();
            let argument_roots: BTreeSet<_> = call
                .arguments
                .iter()
                .map(|(_, _, formal)| *formal)
                .collect();
            if target_roots != argument_roots
                || argument_roots.len() != call.arguments.len()
                || call.arguments.windows(2).any(|rows| rows[0].0 >= rows[1].0)
            {
                return Err(freeze("borrowed-entry/incoming-cardinality"));
            }
            let actuals = self
                .borrowed_formal_actuals
                .get(&call.call)
                .ok_or_else(|| freeze("borrowed-entry/actuals-missing"))?
                .as_ref()
                .map_err(Clone::clone)?;
            if let super::super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) = &call.source {
                self.checked_completed_static_expression_input_v1(original)?;
            }
            if actuals.is_entry_only_static_bool_v1() {
                self.checked_completed_static_bool_entry_v1(source, owner)?
                    .ok_or_else(|| freeze("borrowed-entry/bool-entry-proof-missing"))?;
            }
            if execution
                .as_ref()
                .is_some_and(|owners| !owners.contains(&call.callee))
            {
                let contracts = self
                    .original_parameter_contracts
                    .as_deref()
                    .ok_or_else(|| freeze("borrowed-static/original-catalog-missing"))?;
                if super::super::borrowed_formal_actuals::revalidate_passive_object_incoming_v1(
                    source, contracts, call, actuals,
                    Some(self),
                )? || super::super::borrowed_formal_actuals::revalidate_passive_instance_incoming_with_array_context_v1(
                    source, contracts, call, actuals, &self.callable_result_classes, Some(self),
                )? {
                    continue;
                }
            }
            if call.source.instance().is_some_and(|target| {
                super::super::borrowed_formal_actuals::selected_source_only_object_forward_v1(
                    source, target,
                )
            }) {
                let catalog = self
                    .original_parameter_contracts
                    .as_deref()
                    .ok_or_else(|| freeze("borrowed-guarded/original-catalog-missing"))?;
                super::super::borrowed_formal_actuals::revalidate_passive_object_incoming_v1(
                    source, catalog, call, actuals,
                    Some(self),
                )?;
            }
            actuals
                .ordered_arguments_for_entry_v1(call)
                .map_err(|issue| {
                    if actuals.is_source_only_object_v1() {
                        let detail = self
                            .typed_object_input_completion_diagnostic_v1(call.callee)
                            .unwrap_or_else(|issue| issue);
                        format!("{issue}; object-input: {detail}")
                    } else {
                        issue
                    }
                })?;
            let actuals = &actuals.opaque_actuals;
            self.check_entry_actual_domains_v1(source, definition, actuals)?;
            if call.callee == owner {
                incoming.push((&call.call, actuals.as_ref()));
            }
        }
        if incoming.is_empty() {
            return Err(freeze("borrowed-entry/incoming-missing"));
        }
        Ok(incoming.into_boxed_slice())
    }

    pub(super) fn check_entry_actual_domains_v1(
        &self,
        source: &PreparedBorrowedFormalIngressV1,
        definition: &super::super::borrowed_formal_uses::BorrowedFormalUsesDraftV1,
        actuals: &[PreparedBorrowedFormalActualV1],
    ) -> Result<(), String> {
        for actual in actuals.iter() {
            if definition.origins.get(&actual.formal) != Some(&actual.formal) {
                return Err(freeze("borrowed-entry/actuals-identity"));
            }
            // A minted object view binds every incoming actual to one
            // agreed class — a sealed actual naming a different class
            // (or no class at all) is drift, never a second authority.
            if let Some(view) = source.source_object_view_v1(actual.formal)? {
                let agrees = match &actual.source {
                    super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Read(loan) => loan.class() == view.class(),
                    super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::ReceivedNullable { class, .. }
                    | super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::ReceivedMixed { class, .. }
                    | super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::TypedHome { class, .. }
                    | super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::EntryReceiver { class, .. }
                    | super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::DeclaredFormal { class, .. } => {
                        class.as_ref() == view.class()
                    }
                    super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Null => true,
                    super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1::Forwarded { formal, .. } => {
                        source
                            .object_views
                            .get(formal)
                            .is_some_and(|origin| origin.class() == view.class())
                    }
                    _ => false,
                };
                if !agrees {
                    return Err(freeze("borrowed-entry/object-view-drift"));
                }
            }
        }
        Ok(())
    }
}


impl OrdinaryNewClaimLedgerV1 {
    /// Observe the SAME source-checked substitution while the immediate callee
    /// ordinal is still available. A composed outer ordinal cannot replace it.
    pub(in crate::mir::normal_callable_semantic_package::ordinary_new_coseal) fn compose_object_return_field_supports_with_v1(
        &self,
        acquisition: &super::super::super::completion_index::VerifiedObjectReturnSourceAcquisitionV1,
        required: &[super::super::super::completion_index::ObjectReturnFieldSupportV1],
        mut visit: impl FnMut(&super::super::super::completion_index::ObjectReturnFieldSupportV1,
            &super::super::super::completion_index::ObjectReturnFieldSupportV1) -> Result<(), String>,
    ) -> Result<Option<Box<[super::super::super::completion_index::ObjectReturnFieldSupportV1]>>, String> {
        use super::super::borrowed_formal_actuals::BorrowedFormalActualSourceV1 as Source;
        use crate::mir::resolved_semantics::home_new_prefix::LocalCallArgumentV1;
        if required.is_empty() { return Ok(Some(Box::new([]))); }
        let loan = acquisition.original().qualification();
        let source = self.borrowed_formal_source.as_ref()
            .ok_or_else(|| freeze("object-return/support-source"))?.as_ref().map_err(Clone::clone)?;
        let target = source.object_source_target_at_v1(loan.call())?
            .ok_or_else(|| freeze("object-return/support-target"))?;
        let mut calls = source.source_incoming.exact_rows().filter(|call| &call.call == loan.call());
        let call = calls.next().ok_or_else(|| freeze("object-return/support-incoming"))?;
        if calls.next().is_some() || call.source.require_instance()? != target
            || call.callee != target.callee_owner() || target.target() != loan.key()
        { return Err(freeze("object-return/support-target")); }
        let Some(actuals) = self.borrowed_formal_actuals.get(loan.call()) else { return Ok(None); };
        let actuals = actuals.as_ref().map_err(Clone::clone)?;
        if actuals.ordered_arguments_for_v1(call)? != acquisition.arguments() {
            return Err(freeze("object-return/support-argument-snapshot"));
        }
        let catalog = self.original_parameter_contracts.as_deref()
            .ok_or_else(|| freeze("object-return/support-catalog"))?;
        let mut callers = catalog.iter().filter(|row| row.owner == loan.call().owner());
        let caller = callers.next().ok_or_else(|| freeze("object-return/support-caller"))?;
        if callers.next().is_some() { return Err(freeze("object-return/support-caller")); }
        let mut callees = catalog.iter().filter(|row| row.owner == target.callee_owner());
        let callee = callees.next().ok_or_else(|| freeze("object-return/support-callee"))?;
        if callees.next().is_some() || callee.batch_slot != target.target_batch_slot() {
            return Err(freeze("object-return/support-callee"));
        }
        let mut composed = Vec::new();
        let mut missing = false;
        for support in required {
            let mut parameters = callee.parameters.iter().filter(|row| row.ordinal == support.ordinal);
            let parameter = parameters.next().ok_or_else(|| freeze("object-return/support-callee-formal"))?;
            if parameters.next().is_some() || parameter.binding != support.formal
                || support.formal.owner() != target.callee_owner()
            { return Err(freeze("object-return/support-callee-formal")); }
            let mut matching = actuals.opaque_actuals.iter().filter(|row| row.ordinal == support.ordinal);
            let actual = matching.next().ok_or_else(|| freeze("object-return/support-actual"))?;
            if matching.next().is_some() || actual.formal != support.formal
                || target.argument_sites().get(support.ordinal as usize) != Some(&actual.site)
                || !matches!(acquisition.arguments().get(support.ordinal as usize),
                    Some(LocalCallArgumentV1::BorrowedActual { ordinal, site })
                    if *ordinal == support.ordinal && site == &actual.site)
            { return Err(freeze("object-return/support-actual-identity")); }
            let (binding, root, declared) = match &actual.source {
                Source::Null => continue,
                Source::Forwarded { binding, formal } => (*binding, *formal, None),
                Source::DeclaredFormal { binding, root, class } => (*binding, *root, Some(class)),
                // Local acquisitions, receiver and Array reads require an
                // explicit outgoing support role. BorrowForCall is insufficient.
                // In particular, a dying local Home cannot support an escape.
                _ => { missing = true; continue; },
            };
            if root.owner() != caller.owner || binding.owner() != caller.owner {
                return Err(freeze("object-return/support-caller-root"));
            }
            if binding != root && !source.definitions.get(&caller.owner)
                .or_else(|| source.source_only_definitions.get(&caller.owner))
                .is_some_and(|definition| definition.origins.get(&binding) == Some(&root))
            { return Err(freeze("object-return/support-caller-origin")); }
            let mut parameters = caller.parameters.iter().filter(|row| row.binding == root);
            let parameter = parameters.next().ok_or_else(|| freeze("object-return/support-caller-formal"))?;
            if parameters.next().is_some() || caller.parameters.iter()
                .filter(|row| row.ordinal == parameter.ordinal).count() != 1
                || !matches!(parameter.kind, CallableParameterContractKindV1::OpaqueHandle
                    | CallableParameterContractKindV1::DeclaredObject(_))
            { return Err(freeze("object-return/support-caller-formal")); }
            if let Some(class) = declared {
                if !matches!(&parameter.kind, CallableParameterContractKindV1::DeclaredObject(name) if name == class) {
                    return Err(freeze("object-return/support-declared-class"));
                }
            }
            // An admitted/declared view is a constraint only. An untyped
            // caller formal remains class-free; this never mints its view.
            let view = source.formal_object_view(root).or(source.source_object_view_v1(root)?
                .filter(|view| view.is_declared()));
            if view.is_some_and(|view| view.object() != support.relation.child()) {
                return Err(freeze("object-return/support-object"));
            }
            let mut output = support.clone();
            output.formal = root;
            output.ordinal = parameter.ordinal;
            visit(support, &output)?;
            composed.push(output);
        }
        Ok(if missing { None } else { Some(composed.into_boxed_slice()) })
    }
}

impl OrdinaryNewClaimLedgerV1 {
    /// Terminal and storage purposes contribute their original entry reads to
    /// the same whole-use closure, without granting physical entry/slot ABI.
    /// Borrow the SAME completed New actuals for the common input-use census.
    /// No claim is consumed and no source read is accepted by its kind alone.
    pub(super) fn new_input_evaluations_v1(
        &self, owner: FunctionOwnerIdV1,
    ) -> Result<Vec<Rc<crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>>, String> {
        use crate::mir::resolved_value_profile::expression_plan::ExpressionPlanRootV1;
        let local = self.claims.borrow();
        let returned = self.result_claims.borrow();
        let taken = self.local_commits.borrow();
        let mut sites = BTreeSet::new();
        let mut evaluations = Vec::new();
        for (site, rows) in local.values().map(|claim| (claim.site(), claim.argument_rows()))
            .chain(returned.values().map(|claim| (claim.site(), claim.argument_rows())))
            .chain(taken.iter().filter_map(|(site, row)| row.new_argument_rows().map(|rows| (site, rows.as_deref()))))
            .filter(|(site, _)| site.owner() == owner) {
            // The original row moves to its existing commit on take. Multiple
            // locations violate that transfer even when evaluations are equal.
            if !sites.insert(site) { return Err(freeze("static-input/qualified-use-identity")); }
            // Unqualified rows lend nothing; original reads remain uncovered.
            let Ok(rows) = rows else { continue };
            for (index, argument) in rows.iter().enumerate() {
                let crate::mir::normal_callable_semantic_package::OrdinaryNewTrivialArgumentKindV1::Expression { evaluation } = argument.kind()
                    else { continue };
                let ExpressionPlanRootV1::NewArgument { construction, ordinal, .. } = evaluation.source().root()
                    else { return Err(freeze("static-input/qualified-use-identity")) };
                if argument.owner() != owner || argument.new_site() != site || evaluation.source().owner() != owner
                    || construction.site() != site.site() || *ordinal != argument.ordinal() || *ordinal as usize != index
                    || construction.arguments().get(index) != Some(argument.site())
                    || evaluation.source().nodes().first().map(|node| node.site()) != Some(argument.site()) {
                    return Err(freeze("static-input/qualified-use-identity"));
                }
                if !evaluations.iter().any(|old| Rc::ptr_eq(old, evaluation)) { evaluations.push(Rc::clone(evaluation)); }
            }
        }
        Ok(evaluations)
    }

    pub(super) fn collect_terminal_input_evaluations_v1<'a>(
        &'a self, owner: FunctionOwnerIdV1,
        uses: &super::super::borrowed_formal_uses::OriginalFormalUseSourceV1,
        origins: &BTreeMap<BindingRefV1, BindingRefV1>,
        covered: &mut BTreeSet<(crate::mir::resolved_semantics::SourceExprSiteV1, BindingRefV1)>,
        evaluations: &mut Vec<&'a crate::mir::resolved_semantics::home_new_prefix::CheckedExpressionEvaluationV1>,
    ) -> Result<(), String> {
        use crate::mir::resolved_semantics::ResolvedLexicalRefV1;
        use crate::mir::resolved_semantics::home_new_prefix::MapHomeObservation;
        let flow = self.completion_for_owner(owner).and_then(|completion| completion.cleanup().root_flow());
        for install in flow.into_iter().flat_map(|flow| flow.maps()).filter_map(|row| match row {
            MapHomeObservation::FieldInstall(install) => Some(install), _ => None,
        }) {
            let field = install.source().receiver_v1();
            field.require_entry_source_v1(self)?;
            let (binding, construction, set) = install.source().installed_local_v1(install.site())
                .ok_or_else(|| freeze("static-input/qualified-use-identity"))?;
            if install.site().owner() != owner || binding != install.binding()
                || construction.site() != install.acquisition().site() || set.site() != install.site().site()
                || origins.contains_key(&field.receiver())
                || !uses.reads().any(|(at, original)| at == install.receiver_read()
                    && *original == ResolvedLexicalRefV1::Local(field.receiver()))
                || !covered.insert((install.receiver_read().clone(), field.receiver())) {
                return Err(freeze("static-input/read-use-duplicate-or-foreign"));
            }
            // The same installed candidate owns this source read. The Map
            // receiver/key do not cover its original value argument.
            let value = set.arguments().get(1)
                .ok_or_else(|| freeze("static-input/qualified-use-identity"))?;
            if origins.contains_key(&binding)
                || !uses.reads().any(|(at, original)| at == value.site()
                    && *original == ResolvedLexicalRefV1::Local(binding))
                || !covered.insert((value.site().clone(), binding)) {
                return Err(freeze("static-input/read-use-duplicate-or-foreign"));
            }
            let evaluation = install.key().as_ref();
            if !evaluations.iter().any(|old| std::ptr::eq(*old, evaluation)) { evaluations.push(evaluation); }
        }
        for append in flow.into_iter().flat_map(|flow| flow.named_arrays()).filter_map(|row| match row {
            crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::FieldTextAppend(frame) => Some(frame), _ => None,
        }) {
            if append.source().call().owner() != owner || origins.contains_key(&append.receiver())
                || !uses.reads().any(|(site, original)| site == append.receiver_read()
                    && *original == ResolvedLexicalRefV1::Local(append.receiver()))
                || !covered.insert((append.receiver_read().clone(), append.receiver())) {
                return Err(freeze("static-input/read-use-duplicate-or-foreign"));
            }
            let source = self.field_array_text_append_source_v1(append.source().call())?;
            if !source.matches_text_frame(append) { return Err(freeze("static-input/qualified-use-identity")); }
            let evaluation = append.evaluation().as_ref();
            if !evaluations.iter().any(|old| std::ptr::eq(*old, evaluation)) { evaluations.push(evaluation); }
        }
        for terminal in self.terminal_relations_for_owner(owner) {
            use crate::mir::resolved_semantics::home_new_prefix::{TerminalRelationV1, TerminalReturnedSourceV1};
            if let TerminalRelationV1::Value(value) = terminal {
                let evaluation = match value.returned() {
                    TerminalReturnedSourceV1::TextExpression(evaluation) => Some(evaluation),
                    TerminalReturnedSourceV1::StoredMapGet { source, key, receiver_read } => {
                        let field = source.receiver();
                        field.require_entry_source_v1(self)?;
                        if origins.contains_key(&field.receiver())
                            || source.storage().present_get_v1(&OwnedExprSiteV1::new(owner, value.value_site().clone())).is_none()
                            || !uses.reads().any(|(at, original)| at == receiver_read
                                && *original == ResolvedLexicalRefV1::Local(field.receiver()))
                            || !covered.insert((receiver_read.clone(), field.receiver())) {
                            return Err(freeze("static-input/read-use-duplicate-or-foreign"));
                        }
                        Some(key)
                    }
                    TerminalReturnedSourceV1::StoredMapInstalled(install) => {
                        if !flow.is_some_and(|flow| flow.maps().iter().any(|row|
                            matches!(row, MapHomeObservation::FieldInstall(same) if Rc::ptr_eq(same, install)))) {
                            return Err(freeze("static-input/qualified-use-identity"));
                        }
                        let at = OwnedExprSiteV1::new(owner, value.value_site().clone());
                        let (binding, construction, set) = install.source().installed_local_v1(&at)
                            .ok_or_else(|| freeze("static-input/qualified-use-identity"))?;
                        if binding != install.binding() || construction.site() != install.acquisition().site()
                            || set.owner() != install.site().owner() || set.site() != install.site().site()
                            || origins.contains_key(&binding)
                            || !uses.reads().any(|(site, original)| site == value.value_site()
                                && *original == ResolvedLexicalRefV1::Local(binding))
                            || !covered.insert((value.value_site().clone(), binding)) {
                            return Err(freeze("static-input/read-use-duplicate-or-foreign"));
                        }
                        None
                    }
                    _ => None,
                };
                if let Some(evaluation) = evaluation {
                    if !evaluations.iter().any(|old| std::ptr::eq(*old, evaluation.as_ref())) {
                        evaluations.push(evaluation);
                    }
                }
            }
        }
        Ok(())
    }
}
