//! Existing lexical receiver class provenance; no call issuance or consumption.
use super::*;

/// Provenance chains through caller edges could loop on mutually
/// recursive callables; the join simply proves nothing past this depth.
const MAX_PROVENANCE_DEPTH: u32 = 16;

pub(super) struct LexicalReceiverClassSourceV1<'a> {
    new_classes: &'a std::collections::BTreeMap<OwnedExprSiteV1, Box<str>>,
    ordinary_box_names: &'a [Box<str>],
    field_write_claims: &'a super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
    callable_result_classes: &'a super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
    parameters: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
    app_main: Option<AppMainSourceRefV1<'a>>,
    constructors: Option<&'a crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1>,
    entries: Option<&'a crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1>,
}

impl<'a> LexicalReceiverClassSourceV1<'a> {
    pub(super) fn prepared(
        new_classes: &'a std::collections::BTreeMap<OwnedExprSiteV1, Box<str>>,
        ordinary_box_names: &'a [Box<str>],
        field_write_claims: &'a super::super::field_write_claim::OrdinaryNewFieldWriteClaimsV1,
        callable_result_classes: &'a super::super::result_class_claim::OrdinaryNewResultClassClaimsV1,
        parameters: &'a [crate::mir::normal_callable_semantic_package::model::OwnedCallableParameterContractDeclarationV1],
        app_main: Option<AppMainSourceRefV1<'a>>,
        constructors: Option<&'a crate::mir::normal_callable_semantic_package::VerifiedInstanceConstructorSemanticBatchV1>,
        entries: Option<&'a crate::mir::resolved_semantics::VerifiedInstanceEntryHomeCatalogV1>,
    ) -> Self {
        Self {
            new_classes,
            ordinary_box_names,
            field_write_claims,
            callable_result_classes, parameters, app_main, constructors, entries,
        }
    }

    fn field_write_claim(&self, owner_box: &str, field: &str) -> Option<&str> {
        self.field_write_claims
            .get(&(owner_box.into(), field.into()))
            .map(|class| class.as_ref())
    }

    fn callable_result_class(&self, key: &CanonicalSameModuleCallableKeyV1) -> Option<&str> {
        match self.callable_result_classes.get(key) {
            Some(super::super::result_class_claim::OrdinaryNewResultClassV1::Object(class)) => {
                Some(class.as_ref())
            }
            _ => None,
        }
    }
}

impl LexicalReceiverClassSourceV1<'_> {
    /// Prove one callee `Parameter{index}` receiver's class from caller
    /// edges. The callee must be the only selected declaration carrying its
    /// name+arity, otherwise candidate edges cannot be attributed. Every
    /// lexical/current-owner call site spelled with that selector+arity then
    /// contributes its argument binding's sole-initializer claim class; the
    /// parameter is proven only when every observed claim class collapses to
    /// exactly one. Edges whose argument is rebound, lacks a sole
    /// initializer, or is not a claim-proven local read carry no evidence —
    /// they neither prove nor veto the parameter.
    pub(super) fn prove_parameter_class(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        need: &LexicalInstanceCallNeedV1,
        index: u32,
    ) -> Result<Option<Box<str>>, String> {
        self.parameter_class(batch, selected, need.callee_slot, index, 0)
    }

    /// The `prove_parameter_class` join keyed on the containing function's
    /// batch slot instead of a call-site need, so initializer-call receiver
    /// bindings can reuse the same caller-edge proof.
    fn parameter_class(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        callee_slot: u32,
        index: u32,
        depth: u32,
    ) -> Result<Option<Box<str>>, String> {
        let Some(callee_key) = selected.keys().find_map(|selected_key| {
            let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                return None;
            };
            (selected.batch_slot(selected_key) == Some(callee_slot)).then(|| key.clone())
        }) else {
            return Ok(None);
        };
        let callee_declarations = selected
            .keys()
            .filter(|selected_key| {
                let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                    return false;
                };
                key.name() == callee_key.name() && key.arity() == callee_key.arity()
            })
            .count();
        if callee_declarations != 1 {
            return Ok(None);
        }
        // Universal quantification: every package call site spelled with the
        // callee's selector+arity is a potential edge into it. Arming the
        // parameter receiver requires every such edge to pass an argument
        // that is a sole-initializer, never-rebound, claim-proven local read
        // of one ordinary box — and all observed classes must collapse to a
        // single name. Any edge we cannot prove vetoes the parameter; a
        // vetoed or unproven parameter simply stays unarmed.
        let mut classes = std::collections::BTreeSet::<Box<str>>::new();
        for declaration in batch.declarations() {
            let slot = declaration.batch_slot();
            let edge_proven = batch
                .with_lowering_input(slot, |input| -> Result<bool, String> {
                    let owner = input.owner();
                    for (_site, call) in input.function().method_calls() {
                        if call.selector() != callee_key.name()
                            || call.arity() != callee_key.arity()
                        {
                            continue;
                        }
                        let Some(class) = ({
                            if !matches!(
                                call.receiver(),
                                ResolvedMethodCallReceiverSourceV1::Lexical(
                                    ResolvedLexicalRefV1::Local(_)
                                ) | ResolvedMethodCallReceiverSourceV1::CurrentOwner
                            ) {
                                None
                            } else {
                                let Some(argument) = call
                                    .arguments()
                                    .iter()
                                    .find(|argument| argument.ordinal() == index)
                                else {
                                    return Ok(false);
                                };
                                let Some(ResolvedLexicalRefV1::Local(argument_binding)) =
                                    input.function().variable_ref(argument.site())
                                else {
                                    return Ok(false);
                                };
                                if argument_binding.owner() != owner
                                    || input.function().assignment_targets().any(|(_, target)| {
                                        matches!(
                                            target,
                                            ResolvedAssignmentTargetV1::BindingRebind(rebound)
                                                if *rebound == argument_binding
                                        )
                                    })
                                {
                                    return Ok(false);
                                }
                                let mut initializers =
                                    input.function().expression_source().initializers().filter(
                                        |initializer| initializer.binding() == argument_binding,
                                    );
                                let Some(initializer) = initializers.next() else {
                                    return Ok(false);
                                };
                                if initializers.next().is_some() {
                                    return Ok(false);
                                }
                                let Some(initializer_site) = initializer.initializer_site() else {
                                    return Ok(false);
                                };
                                self.initializer_class(
                                    batch,
                                    selected,
                                    slot,
                                    owner,
                                    input,
                                    initializer_site,
                                    depth + 1,
                                )?
                            }
                        }) else {
                            return Ok(false);
                        };
                        let class: Box<str> = class;
                        if !self
                            .ordinary_box_names
                            .iter()
                            .any(|name| name.as_ref() == class.as_ref())
                        {
                            return Ok(false);
                        }
                        classes.insert(class);
                    }
                    Ok(true)
                })
                .map_err(|_| freeze("lexical-instance-call/batch-loan"))??;
            if !edge_proven {
                return Ok(None);
            }
        }
        match classes.len() {
            1 => Ok(classes.into_iter().next()),
            _ => Ok(None),
        }
    }

    /// Structural source precondition only. Numeric writer closure and every
    /// physical borrow still belong to the late Call and final consumers.
    pub(super) fn borrowed_receiver_interval_v1(
        &self, batch: &VerifiedResolvedCallableSemanticBatchV1,
        need: &LexicalInstanceCallNeedV1, callee_slot: u32,
        loan: &BorrowedReceiverSourceV1,
    ) -> Result<bool, String> {
        use crate::mir::resolved_semantics::{BodyEffectKindV1 as Effect,
            BodyExpressionShapeV1 as Shape, BodyMeReceiverV1, ResolvedAssignmentTargetV1,
            ResolvedLexicalRefV1 as Ref, SourcePathSegmentV1 as Part};
        if matches!(loan, BorrowedReceiverSourceV1::OrdinaryCall(source) if !source.is_map_slot())
            || loan.destination() != need.receiver_binding
            || loan.acquisition().owner() != need.owner { return Ok(false); }
        let caller = batch.with_lowering_input(need.callee_slot, |input| {
            let Some(shape) = input.body_shape() else { return false; };
            let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return false; };
            let [Part::Body(start), Part::Initializer(_)] = loan.acquisition().site().node().segments() else { return false; };
            let end = match (loan, need.call_site.node().segments()) {
                (_, [Part::Body(end)]) => end,
                (BorrowedReceiverSourceV1::DirectMapGet(_), [Part::Body(end), Part::Initializer(_)]) => end,
                _ => return false,
            };
            let inside = |site: &SourceExprSiteV1| matches!(site.node().segments().first(),
                Some(Part::Body(index)) if start <= index && index <= end);
            let boundary = |site: &SourceExprSiteV1| site == loan.acquisition().site() || site == &need.call_site;
            input.owner() == need.owner && start < end
                && input.function().variable_ref(&need.receiver_site) == Some(Ref::Local(loan.destination()))
                && ledger.uncaptured_binding(loan.destination())
                && shape.effects().iter().filter(|effect| inside(&effect.site))
                    .all(|effect| effect.kind == Effect::Call && boundary(&effect.site)
                        || matches!(loan, BorrowedReceiverSourceV1::DirectMapGet(_))
                            && effect.kind == Effect::NonLocalControl && shape.statements().iter().any(|row|
                                matches!(row, crate::mir::resolved_semantics::BodyStatementShapeV1::Return { site, .. }
                                    if site.node() == effect.site.node())))
                && !ledger.direct_call_observations().any(|(site, _)| inside(site))
                && ledger.method_calls().filter(|(site, _)| inside(site)).all(|(site, _)| boundary(site))
                && !input.function().expression_source().constructions().any(|new| inside(new.site()))
                && ledger.capture_demands().is_empty()
        }).map_err(|_| freeze("lexical-instance-call/batch-loan"))?;
        if !caller { return Ok(false); }
        batch.with_lowering_input(callee_slot, |input| {
            let Some(shape) = input.body_shape() else { return false; };
            let Ok(ledger) = input.forest().callable_source_ledger(input.owner()) else { return false; };
            if ledger.method_calls().next().is_some() || ledger.direct_call_observations().next().is_some()
                || !ledger.capture_demands().is_empty()
                || input.function().expression_source().constructions().next().is_some()
                || shape.effects().iter().any(|effect| effect.kind != Effect::Write
                    && !(matches!(loan, BorrowedReceiverSourceV1::DirectMapGet(_)) && effect.kind == Effect::NonLocalControl
                        && shape.statements().iter().any(|row| matches!(row,
                            crate::mir::resolved_semantics::BodyStatementShapeV1::Return { site, .. }
                                if site.node() == effect.site.node())))) { return false; }
            let assignments = shape.assignment_sources();
            let writes: Vec<_> = self.field_write_claims.writes().iter().filter(|write| write.owner() == input.owner()).collect();
            if assignments.len() != writes.len() { return false; }
            assignments.iter().all(|assignment| {
                let Some(ResolvedAssignmentTargetV1::FieldWrite { receiver }) = input.function().assignment_target(assignment.target_site())
                    else { return false; };
                matches!(shape.expression_shape(receiver), Some(Shape::Me { receiver: BodyMeReceiverV1::Lexical(binding), .. })
                    if writes.iter().filter(|write| write.assignment() == assignment && write.receiver() == Some(*binding)).count() == 1)
            })
        }).map_err(|_| freeze("lexical-instance-call/batch-loan"))
    }

    /// Prove one claim-local receiver's class from its sole initializer
    /// inside the callee itself: an ordinary-new claim (`local x = new
    /// C()`), a `me.f` field read whose field carries a field-write
    /// claim (`local x = me.f` where `me.f = new C()` in birth and no
    /// other write anywhere stores another shape), or a method call
    /// whose proven receiver's callee carries a uniform `return new C`
    /// result claim (`local x = builder.make(...)`).
    pub(super) fn claim_local_class(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        need: &LexicalInstanceCallNeedV1,
    ) -> Result<Option<(Box<str>, Option<Box<BorrowedReceiverSourceV1>>)>, String> {
        batch
            .with_lowering_input(
                need.callee_slot,
                |input| -> Result<Option<(Box<str>, Option<Box<BorrowedReceiverSourceV1>>)>, String> {
                    if input.owner() != need.owner {
                        return Ok(None);
                    }
                    let mut initializers = input
                        .function()
                        .expression_source()
                        .initializers()
                        .filter(|initializer| initializer.binding() == need.receiver_binding);
                    let Some(initializer) = initializers.next() else {
                        return Ok(None);
                    };
                    if initializers.next().is_some() {
                        return Ok(None);
                    }
                    let Some(initializer_site) = initializer.initializer_site() else {
                        return Ok(None);
                    };
                    let class = self.initializer_class(
                        batch,
                        selected,
                        need.callee_slot,
                        need.owner,
                        input,
                        initializer_site,
                        0,
                    )?;
                    let (class, borrowed) = if let Some(class) = class { (class, None) } else {
                        if input.function().method_call(initializer_site).is_none()
                            || need.rebound
                            || !input.forest().callable_source_ledger(input.owner())
                                .is_ok_and(|ledger| ledger.uncaptured_binding(need.receiver_binding)) { return Ok(None); }
                        let call = OwnedExprSiteV1::new(need.owner, initializer_site.clone());
                        let direct = if let (Some(constructors), Some(entries)) = (self.constructors, self.entries) {
                            let entry = entries.for_batch_slot(need.callee_slot);
                            let receiver = super::super::entry_receiver_box_proof(selected, batch, entry, need.callee_slot);
                            super::super::StoredMapGetSourceLoanV1::issue_for_site_v1(input, &call, batch, selected,
                                constructors, receiver, entry, self.callable_result_classes, self.field_write_claims)?
                                .and_then(|source| DirectMapReceiverSourceV1::issue(input, need.receiver_binding, source))
                        } else { None };
                        if let Some(source) = direct {
                            let class = source.source().payload_source_v1().payload_class_v1().into();
                            (class, Some(Box::new(BorrowedReceiverSourceV1::DirectMapGet(source))))
                        } else {
                            let Some(loan) = self.callable_result_classes.borrowed_map_local_receiver_source_v1(
                                batch, selected, self.field_write_claims, self.parameters, &call,
                                need.receiver_binding, self.app_main).map_err(freeze)? else { return Ok(None); };
                            let (class, _) = loan.map_payload_v1().ok_or_else(|| freeze("borrowed-nullable/source-identity"))?;
                            (class.into(), Some(Box::new(BorrowedReceiverSourceV1::OrdinaryCall(loan))))
                        }
                    };
                    if !self
                        .ordinary_box_names
                        .iter()
                        .any(|name| name.as_ref() == class.as_ref())
                    {
                        return Ok(None);
                    }
                    Ok(Some((class, borrowed)))
                },
            )
            .map_err(|_| freeze("lexical-instance-call/batch-loan"))?
    }

    /// Class provenance for one sole-initializer expression site inside
    /// the function owning it. `local x = new C()` carries the direct
    /// claim; `local x = me.f` carries the field-write claim sealed for
    /// (`owner_box`, `f`) — `me` must resolve to this function's lexical
    /// receiver binding and the function must be an `InstanceBoxMethod`
    /// so its selected key names the owning box; `local x = recv.m(...)`
    /// carries the callee's result-class claim when `recv`'s binding
    /// resolves to a proven class and `m` uniquely selects an
    /// `InstanceBoxMethod` that returns `new` of one class on every
    /// path. Anything else (literals, arithmetic, unknown receivers)
    /// proves nothing.
    fn initializer_class(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        initializer_site: &SourceExprSiteV1,
        depth: u32,
    ) -> Result<Option<Box<str>>, String> {
        // Provenance chains cross functions through caller edges; mutually
        // recursive callables could loop this join forever, so the walk
        // gives up past a bounded depth and simply proves nothing.
        if depth > MAX_PROVENANCE_DEPTH {
            return Ok(None);
        }
        if let Some(class) = self
            .new_classes
            .get(&OwnedExprSiteV1::new(owner, initializer_site.clone()))
        {
            return Ok(Some(class.clone()));
        }
        let Some(shape) = input.body_shape() else {
            return Ok(None);
        };
        match shape.expression_shape(initializer_site) {
            Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) => {
                let owner_box: Box<str> = if let Some(BodyExpressionShapeV1::Me {
                    receiver: BodyMeReceiverV1::Lexical(receiver),
                    ..
                }) = shape.expression_shape(object)
                {
                    if input
                        .function()
                        .binding(*receiver)
                        .is_none_or(|record| record.kind() != BindingKindV1::Receiver)
                    {
                        return Ok(None);
                    }
                    let Some(class) = selected.keys().find_map(|selected_key| {
                        let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                            return None;
                        };
                        (key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
                            && selected.batch_slot(selected_key) == Some(batch_slot))
                        .then(|| key.owner().into())
                    }) else {
                        return Ok(None);
                    };
                    class
                } else {
                    let Some(ResolvedLexicalRefV1::Local(receiver)) =
                        input.function().variable_ref(object)
                    else {
                        return Ok(None);
                    };
                    // Reuse sole-initializer/caller-edge provenance, including
                    // its rebound veto and recursion bound. Field declarations
                    // alone cannot choose a receiver class or call target.
                    let Some(class) = self.binding_class(
                        batch,
                        selected,
                        batch_slot,
                        owner,
                        input,
                        receiver,
                        depth + 1,
                    )?
                    else {
                        return Ok(None);
                    };
                    class
                };
                Ok(self
                    .field_write_claim(&owner_box, field.as_ref())
                    .map(|class| class.into()))
            }
            Some(BodyExpressionShapeV1::MethodCall {
                object,
                method,
                arity,
                ..
            }) => {
                let Some(ResolvedLexicalRefV1::Local(receiver_binding)) =
                    input.function().variable_ref(object)
                else {
                    return Ok(None);
                };
                if receiver_binding.owner() != owner {
                    return Ok(None);
                }
                let Some(receiver_class) = self.binding_class(
                    batch,
                    selected,
                    batch_slot,
                    owner,
                    input,
                    receiver_binding,
                    depth,
                )?
                else {
                    return Ok(None);
                };
                let Some((key, _)) = unique_instance_target(
                    selected,
                    receiver_class.as_ref(),
                    method.as_ref(),
                    *arity,
                ) else {
                    return Ok(None);
                };
                Ok(self.callable_result_class(&key).map(|class| class.into()))
            }
            _ => Ok(None),
        }
    }

    /// Class provenance for one lexical binding inside the containing
    /// function: a parameter proven by the universal caller-edge join,
    /// or a local proven by its sole initializer. Rebound bindings and
    /// non-lexical kinds prove nothing.
    fn binding_class(
        &self,
        batch: &VerifiedResolvedCallableSemanticBatchV1,
        selected: &VerifiedSelectedCallableBatchMapV1,
        batch_slot: u32,
        owner: FunctionOwnerIdV1,
        input: crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1<'_>,
        binding: BindingRefV1,
        depth: u32,
    ) -> Result<Option<Box<str>>, String> {
        if binding.owner() != owner
            || input.function().assignment_targets().any(|(_, target)| {
                matches!(
                    target,
                    ResolvedAssignmentTargetV1::BindingRebind(rebound) if *rebound == binding
                )
            })
        {
            return Ok(None);
        }
        let Some(record) = input.function().binding(binding) else {
            return Ok(None);
        };
        match record.kind() {
            BindingKindV1::Parameter { index } => {
                self.parameter_class(batch, selected, batch_slot, index, depth + 1)
            }
            BindingKindV1::Local { .. } => {
                let mut initializers = input
                    .function()
                    .expression_source()
                    .initializers()
                    .filter(|initializer| initializer.binding() == binding);
                let Some(initializer) = initializers.next() else {
                    return Ok(None);
                };
                if initializers.next().is_some() {
                    return Ok(None);
                }
                let Some(initializer_site) = initializer.initializer_site() else {
                    return Ok(None);
                };
                let Some(class) = self.initializer_class(
                    batch,
                    selected,
                    batch_slot,
                    owner,
                    input,
                    initializer_site,
                    depth + 1,
                )?
                else {
                    return Ok(None);
                };
                Ok(self
                    .ordinary_box_names
                    .iter()
                    .any(|name| name.as_ref() == class.as_ref())
                    .then_some(class))
            }
            _ => Ok(None),
        }
    }
}
