//! Construct checked actuals from the same original class, entry and Home evidence.
use super::*;

pub(super) fn construct_borrowed_call_actuals_v1(
    prepared: &PreparedBorrowedFormalIngressV1,
    incoming_row: &super::borrowed_formal_uses::BorrowedIncomingCallDraftV1,
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    call: &OwnedExprSiteV1,
    actuals: &[BorrowedCallActualCandidateV1],
    homes: object_source::ObjectActualHomeSourceV1<'_>,
    receiver: Option<(BindingRefV1, &crate::parser::ParserOrdinaryBoxSourceRowV1)>,
    nullable_class: &mut impl FnMut(BindingRefV1) -> Option<Box<str>>,
    static_zero: Option<static_input_finish::StaticInputFinishV1>,
    guarded_definition: Option<&super::borrowed_formal_uses::BorrowedFormalUsesDraftV1>,
    source_actual: &mut dyn FnMut(
        u32,
        &SourceExprSiteV1,
        BindingRefV1,
    ) -> Result<Option<BorrowedFormalActualSourceV1>, String>,
) -> Result<Option<PreparedBorrowedCallActualsV1>, String> {
    let mut callee_contracts = contracts
        .iter()
        .filter(|row| row.owner == incoming_row.callee);
    let contract = callee_contracts
        .next()
        .ok_or_else(|| freeze("borrowed-actual/formal-missing"))?;
    if callee_contracts.next().is_some() || actuals.len() != contract.parameters.len() {
        return Err(freeze("borrowed-actual/arity"));
    }
    if incoming_row.source.call_site() != call
        || incoming_row.source.callee_owner() != contract.owner
        || incoming_row.source.target_batch_slot() != contract.batch_slot
        || incoming_row.source.argument_sites().len() != contract.parameters.len()
    {
        return Err(freeze("borrowed-actual/source-identity"));
    }
    if incoming_row.source.as_loan()?.declaration_mode() != contract.mode {
        return Err(freeze("borrowed-actual/source-mode"));
    }
    if let super::borrowed_formal_uses::BorrowedIncomingSourceV1::Static(original) =
        &incoming_row.source
    {
        if let Some(finish) = &static_zero {
            if !finish.corroborates_source_v1(original) {
                return Err(freeze("static-zero/input-finish-source-drift"));
            }
        } else {
            original.require_qualified_scalar_source_v1()?;
        }
        let retained = if static_zero.is_some() {
            static_input_finish::original_zero_input_source_v1(prepared, original)?
                .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
        } else {
            prepared
                .source_incoming
                .static_observations()
                .get(call)
                .ok_or_else(|| freeze("borrowed-static/source-observation-missing"))?
                .as_ref()
                .map_err(Clone::clone)?
        };
        if !std::rc::Rc::ptr_eq(original, retained)
            || original.parameters() != contract.parameters.as_ref()
        {
            return Err(freeze("borrowed-static/executable-source-identity"));
        }
    }
    // The opaque subset cannot prove the unchanged scalar arguments beside
    // it. Cover every source ordinal before lending any successful actuals
    // to entry adoption or a continuation of this selected definition.
    for (index, (formal, actual)) in contract.parameters.iter().zip(actuals).enumerate() {
        if formal.ordinal as usize != index
            || formal.binding.owner() != contract.owner
            || actual.ordinal != formal.ordinal
            || actual.site != incoming_row.source.argument_sites()[index]
        {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        match &formal.kind {
            kind if kind.is_ordinary_borrowed_handle() => {}
            CallableParameterContractKindV1::ExactTrivial(_) => match &actual.value {
                BorrowedCallActualValueV1::Integer(_) => {}
                BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer)
                    if binding.owner() == call.owner() => {}
                _ => return Err(freeze("borrowed-actual/nonopaque-scalar-unproved")),
            },
            // Existing Map/Text/declared-handle contracts need their own
            // source consumer proof; an opaque neighbour cannot provide it.
            _ => return Err(freeze("borrowed-actual/nonopaque-contract-unproved")),
        }
    }
    let mut rows = Vec::new();
    for (ordinal, site, formal) in &incoming_row.arguments {
        let actual = actuals
            .get(*ordinal as usize)
            .ok_or_else(|| freeze("borrowed-actual/ordinal"))?;
        if actual.ordinal != *ordinal
            || actual.site != *site
            || formal.owner() != incoming_row.callee
            || contract
                .parameters
                .get(*ordinal as usize)
                .is_none_or(|row| {
                    row.ordinal != *ordinal
                        || row.binding != *formal
                        || !row.kind.is_ordinary_borrowed_handle()
                })
        {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        let source = match &actual.value {
            BorrowedCallActualValueV1::DeclaredRead(read) => {
                let source = BorrowedFormalActualSourceV1::Read(crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1::from_declared_v1(read.clone()));
                model::validate_computed_actual_source_v1(&source, actual, call, &contract.parameters[*ordinal as usize])?;
                source
            }
            BorrowedCallActualValueV1::Integer(value) => {
                BorrowedFormalActualSourceV1::Integer(*value)
            }
            BorrowedCallActualValueV1::Bool(value) => BorrowedFormalActualSourceV1::Bool(*value),
            BorrowedCallActualValueV1::Null => BorrowedFormalActualSourceV1::Null,
            BorrowedCallActualValueV1::FieldAccess => {
                let (read, get, declared) = prepared
                    .original_map_field_actual_source_v1(call, *ordinal, site)?
                    .ok_or_else(|| freeze("borrowed-actual/map-field-source"))?;
                if read.site() != site {
                    return Err(freeze("borrowed-actual/map-field-original-identity"));
                }
                let class = prepared
                    .field_value_class_v1(declared)
                    .ok_or_else(|| freeze("borrowed-actual/map-field-class"))?;
                BorrowedFormalActualSourceV1::Read(
                    crate::mir::normal_callable_semantic_package::ReadActualSourceLoanV1::from_map_field_v1(
                        read.clone(), class, get.clone(), Rc::clone(declared),
                    ),
                )
            }
            BorrowedCallActualValueV1::String(_)
            | BorrowedCallActualValueV1::Expression
            | BorrowedCallActualValueV1::Binding(_)
            | BorrowedCallActualValueV1::MethodCall(_)
            | BorrowedCallActualValueV1::ReceivedField { .. }
            | BorrowedCallActualValueV1::Unknown => {
                let source = source_actual(*ordinal, site, *formal)?
                    .ok_or_else(|| freeze("borrowed-actual/unsupported-or-unavailable"))?;
                model::validate_computed_actual_source_v1(
                    &source,
                    actual,
                    call,
                    &contract.parameters[*ordinal as usize],
                )?;
                source
            }
            BorrowedCallActualValueV1::ReceivedNullable(binding)
                if binding.owner() == call.owner() =>
            {
                // The sealed `NullableObject` claim is the sole class
                // authority — a fake, foreign, or unclaimed producer can
                // never mint the nullable typed-object actual.
                let class = match homes {
                    object_source::ObjectActualHomeSourceV1::Ledger(ledger) => {
                        let Some(class) = ledger.borrowed_nullable_actual_class_v1(*binding)?
                        else {
                            return Ok(None);
                        };
                        class
                    }
                    _ => nullable_class(*binding)
                        .ok_or_else(|| freeze("borrowed-actual/nullable-class-unavailable"))?,
                };
                BorrowedFormalActualSourceV1::ReceivedNullable {
                    binding: *binding,
                    class,
                }
            }
            BorrowedCallActualValueV1::ReceivedMixed(binding)
                if binding.owner() == call.owner() =>
            {
                let original = prepared
                    .mixed_received_actual_source_v1(*binding)?
                    .ok_or_else(|| freeze("borrowed-actual/mixed-source-unavailable"))?;
                let class: Box<str> = match homes {
                    object_source::ObjectActualHomeSourceV1::Ledger(ledger) => {
                        let Some(class) = ledger.borrowed_mixed_actual_class_v1(*binding)? else {
                            return Ok(None);
                        };
                        class
                    }
                    _ => original.fresh_class().into(),
                };
                if class.as_ref() != original.fresh_class() {
                    return Err(freeze("borrowed-actual/mixed-class-drift"));
                }
                BorrowedFormalActualSourceV1::ReceivedMixed {
                    binding: *binding,
                    class,
                }
            }
            BorrowedCallActualValueV1::Scalar(binding, kind) if binding.owner() == call.owner() => {
                BorrowedFormalActualSourceV1::Scalar {
                    binding: *binding,
                    kind: *kind,
                }
            }
            BorrowedCallActualValueV1::FreshConstruction(construction)
                if prepared.has_original_fresh_take_v1(call, actual.ordinal, construction, contracts)? =>
            {
                BorrowedFormalActualSourceV1::FreshTakenHome {
                    acquisition: OwnedExprSiteV1::new(call.owner(), actual.site.clone()),
                    class: construction.class().into(),
                }
            }
            BorrowedCallActualValueV1::Home { .. } => {
                let Some(source) = homes.prepare_home_actual_v1(call, &contract.parameters[*ordinal as usize], actual)? else {
                    return Ok(None);
                };
                source
            }
            BorrowedCallActualValueV1::SelfRooted { binding, root } => {
                if let Some(formal_origin) = prepared
                    .definitions
                    .get(&call.owner())
                    .and_then(|draft| draft.origins.get(binding))
                {
                    if root != formal_origin
                        || !prepared.forwards.iter().any(|row| {
                            &row.call == call
                                && row.ordinal == *ordinal
                                && row.site.site() == site
                                && row.binding == *binding
                                && row.source_formal == *formal_origin
                                && row.callee_formal == *formal
                        })
                    {
                        return Err(freeze("borrowed-actual/forward-identity"));
                    }
                    BorrowedFormalActualSourceV1::Forwarded {
                        binding: *binding,
                        formal: *formal_origin,
                    }
                } else if let Some(class) = instance_source::declared_source_actual_class_v1(
                    prepared, contracts, call, *binding, *root,
                )? {
                    BorrowedFormalActualSourceV1::DeclaredFormal {
                        binding: *binding,
                        root: *root,
                        class,
                    }
                } else if let Some(draft) =
                    guarded_definition.filter(|draft| draft.origins.get(binding) == Some(root))
                {
                    let selected = !prepared
                        .guarded_actuals
                        .contains_key(&(call.clone(), *ordinal))
                        && prepared
                            .original_mixed_tagged_forward_v1(
                                contracts, call, *ordinal, site, *binding, *root,
                            )?
                            .is_some_and(|original| std::ptr::eq(original, draft));
                    if !selected {
                        let fact = prepared
                            .guarded_actuals
                            .get(&(call.clone(), *ordinal))
                            .ok_or_else(|| freeze("borrowed-guarded/actual-guard-missing"))?;
                        let mut uses = draft.uses.iter().filter(|row| row.site.site() == site);
                        let row = uses
                            .next()
                            .ok_or_else(|| freeze("borrowed-guarded/actual-use-missing"))?;
                        if uses.next().is_some()
                            || row.binding != *binding
                            || row.formal != *root
                            || !fact.matches_draft_argument(row, call, *ordinal, draft)
                        {
                            return Err(freeze("borrowed-guarded/actual-use-identity"));
                        }
                    }
                    BorrowedFormalActualSourceV1::Forwarded {
                        binding: *binding,
                        formal: *root,
                    }
                } else {
                    let (entry, class) =
                        receiver.ok_or_else(|| freeze("borrowed-actual/entry-domain"))?;
                    if *root != entry
                        || binding.owner() != call.owner()
                        || entry.owner() != call.owner()
                    {
                        return Err(freeze("borrowed-actual/entry-source"));
                    }
                    BorrowedFormalActualSourceV1::EntryReceiver {
                        binding: *binding,
                        root: entry,
                        class: class.name().into(),
                    }
                }
            }
            _ => return Err(freeze("borrowed-actual/unsupported-or-unavailable")),
        };
        if contract.parameters[*ordinal as usize].demand == crate::mir::resolved_semantics::HomeDemandV1::Home
            && !source.transfers_home_at_entry_v1() {
            return Err(freeze("borrowed-actual/source-identity"));
        }
        if let CallableParameterContractKindV1::DeclaredObject(expected) =
            &contract.parameters[*ordinal as usize].kind
        {
            let class = match &source {
                BorrowedFormalActualSourceV1::Null => None,
                BorrowedFormalActualSourceV1::Read(loan) => Some(loan.class()),
                BorrowedFormalActualSourceV1::TypedHome { class, .. }
                | BorrowedFormalActualSourceV1::FreshTakenHome { class, .. }
                | BorrowedFormalActualSourceV1::EntryReceiver { class, .. }
                | BorrowedFormalActualSourceV1::LocalObjectBinding { class, .. }
                | BorrowedFormalActualSourceV1::ReceivedNullable { class, .. }
                | BorrowedFormalActualSourceV1::ReceivedMixed { class, .. }
                | BorrowedFormalActualSourceV1::DeclaredFormal { class, .. } => {
                    Some(class.as_ref())
                }
                BorrowedFormalActualSourceV1::Forwarded { formal, .. } => Some(
                    prepared
                        .object_views
                        .get(formal)
                        .ok_or_else(|| freeze("borrowed-actual/declared-forward-class"))?
                        .class(),
                ),
                _ => return Err(freeze("borrowed-actual/declared-object-domain")),
            };
            if class.is_some_and(|actual| actual != expected.as_ref()) {
                return Err(freeze("borrowed-actual/declared-object-class"));
            }
        }
        rows.push(PreparedBorrowedFormalActualV1 {
            ordinal: *ordinal,
            site: site.clone(),
            formal: *formal,
            source,
        });
    }
    let ordered_arguments = actuals
        .iter()
        .map(|actual| {
            if rows.iter().any(|row| row.ordinal == actual.ordinal) {
                Ok(LocalCallArgumentV1::BorrowedActual {
                    ordinal: actual.ordinal,
                    site: actual.site.clone(),
                })
            } else {
                match actual.value {
                    BorrowedCallActualValueV1::Integer(value) => {
                        Ok(LocalCallArgumentV1::Integer(value))
                    }
                    BorrowedCallActualValueV1::Scalar(binding, SourceScalarKind::Integer) => {
                        Ok(LocalCallArgumentV1::Scalar(binding))
                    }
                    _ => Err(freeze("borrowed-actual/nonopaque-scalar-unproved")),
                }
            }
        })
        .collect::<Result<Vec<_>, String>>()?;
    Ok(Some(PreparedBorrowedCallActualsV1 {
        phase: match static_zero {
            Some(finish) => BorrowedCallActualEvidencePhaseV1::ExecutableStaticInput(finish),
            None => BorrowedCallActualEvidencePhaseV1::Executable,
        },
        opaque_actuals: rows.into_boxed_slice(),
        ordered_arguments: ordered_arguments.into_boxed_slice(),
        dynamic_invocation_arguments: None,
    }))
}
