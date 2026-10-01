//! Existing lexical receiver class provenance; no call issuance or consumption.
use super::*;

/// Provenance chains through caller edges could loop on mutually
/// recursive callables; the join simply proves nothing past this depth.
const MAX_PROVENANCE_DEPTH: u32 = 16;

impl OrdinaryNewClaimLedgerV1 {
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
    ) -> Result<Option<Box<str>>, String> {
        batch
            .with_lowering_input(
                need.callee_slot,
                |input| -> Result<Option<Box<str>>, String> {
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
                    let Some(class) = self.initializer_class(
                        batch,
                        selected,
                        need.callee_slot,
                        need.owner,
                        input,
                        initializer_site,
                        0,
                    )?
                    else {
                        return Ok(None);
                    };
                    if !self
                        .ordinary_box_names
                        .iter()
                        .any(|name| name.as_ref() == class.as_ref())
                    {
                        return Ok(None);
                    }
                    Ok(Some(class))
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
        if let Some(claim) = self
            .claims
            .borrow()
            .get(&OwnedExprSiteV1::new(owner, initializer_site.clone()))
        {
            return Ok(Some(claim.class().into()));
        }
        let Some(shape) = input.body_shape() else {
            return Ok(None);
        };
        match shape.expression_shape(initializer_site) {
            Some(BodyExpressionShapeV1::FieldAccess { object, field, .. }) => {
                let Some(BodyExpressionShapeV1::Me {
                    receiver: BodyMeReceiverV1::Lexical(receiver),
                    ..
                }) = shape.expression_shape(object)
                else {
                    return Ok(None);
                };
                if input
                    .function()
                    .binding(*receiver)
                    .is_none_or(|record| record.kind() != BindingKindV1::Receiver)
                {
                    return Ok(None);
                }
                let Some(owner_box) = selected.keys().find_map(|selected_key| {
                    let SelectedNormalCallableKeyV1::Cataloged(key) = selected_key else {
                        return None;
                    };
                    (key.namespace() == SameModuleCallableNamespaceV1::InstanceBoxMethod
                        && selected.batch_slot(selected_key) == Some(batch_slot))
                    .then(|| key.owner())
                }) else {
                    return Ok(None);
                };
                Ok(self
                    .field_write_claim(owner_box, field.as_ref())
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
