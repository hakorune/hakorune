//! Source-only checked-input closure for formals forwarded to Static callees.
//! A forwarded carrier stays tagged; this never proves its payload is I64.
use super::borrowed_formal_uses::BorrowedFormalUseDraftKindV1 as Use;
use super::*;

impl PreparedBorrowedFormalIngressV1 {
    /// Source-only formal origin for a selected entry request. This never
    /// promotes the owner into the executable borrowed transport profile.
    pub(in crate::mir::normal_callable_semantic_package) fn source_only_formal_origin(
        &self,
        formal: BindingRefV1,
    ) -> bool {
        self.source_only_definitions
            .get(&formal.owner())
            .and_then(|draft| draft.origins.get(&formal))
            == Some(&formal)
    }

    /// Conditional Normal-use requirement, not an Integer classification or
    /// entry/transport grant for any incoming actual.
    pub(in crate::mir::normal_callable_semantic_package) fn checked_static_input(
        &self,
        formal: BindingRefV1,
    ) -> bool {
        self.checked_static_inputs.contains(&formal)
    }
}

pub(super) fn issue_checked_static_inputs_v1(
    contracts: &[OwnedCallableParameterContractDeclarationV1],
    definitions: &BTreeMap<FunctionOwnerIdV1, BorrowedFormalUsesDraftV1>,
    static_arguments: &BTreeMap<
        (OwnedExprSiteV1, u32),
        super::borrowed_static_argument::StaticArgumentSourceV1,
    >,
) -> Result<BTreeSet<BindingRefV1>, String> {
    // Instance candidates come only from retained Static argument source rows.
    // This lends a conditional Normal-use requirement, never entry/type authority.
    let instance_forwarders: BTreeSet<_> = static_arguments
        .values()
        .map(|argument| argument.formal())
        .collect();
    let formals: BTreeSet<_> = contracts
        .iter()
        .flat_map(|contract| {
            let instance_forwarders = &instance_forwarders;
            contract.parameters.iter().filter(move |parameter| {
                parameter.kind.is_ordinary_borrowed_handle()
                    && (contract.mode == CallableParameterDeclarationModeV1::StaticBoxMethod
                        || (contract.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
                            && instance_forwarders.contains(&parameter.binding)))
            })
        })
        .map(|parameter| parameter.binding)
        .collect();
    let mut candidates = BTreeMap::<BindingRefV1, (bool, BTreeSet<BindingRefV1>)>::new();
    for formal in &formals {
        let Some(draft) = definitions.get(&formal.owner()) else {
            continue;
        };
        if draft.origins.get(formal) != Some(formal) {
            return Err(freeze("checked-input/formal-origin"));
        }
        let mut checked_use = false;
        let mut forwards = BTreeSet::new();
        let mut covered = true;
        for row in draft.uses.iter().filter(|row| row.formal == *formal) {
            if draft.origins.get(&row.binding) != Some(formal) {
                return Err(freeze("checked-input/use-origin"));
            }
            match &row.kind {
                Use::Copy { .. } => {}
                Use::CompareOperand { .. } => checked_use = true,
                Use::IntegerReturn { guard, .. } => {
                    if !draft.uses.iter().any(|compare| match &compare.kind {
                        Use::CompareOperand { binary, source } => {
                            guard.matches_compare(*formal, binary, source)
                        }
                        _ => false,
                    }) {
                        return Err(freeze("checked-input/return-guard"));
                    }
                }
                Use::AddOperand { .. } | Use::MulOperand { .. } => {}
                Use::UnresolvedArgument { call, ordinal } => {
                    let Some(fact) = static_arguments.get(&(call.clone(), *ordinal)) else {
                        covered = false;
                        break;
                    };
                    if fact.call() != call
                        || fact.ordinal() != *ordinal
                        || fact.use_site() != &row.site
                        || fact.binding() != row.binding
                        || fact.formal() != *formal
                    {
                        return Err(freeze("checked-input/forward-source"));
                    }
                    forwards.insert(fact.target_formal());
                }
                _ => {
                    covered = false;
                    break;
                }
            }
        }
        if covered && (checked_use || !forwards.is_empty()) {
            candidates.insert(*formal, (checked_use, forwards));
        }
    }
    // Least fixed point: a forward-only cycle cannot assert its own check.
    let mut issued = BTreeSet::new();
    loop {
        let before = issued.len();
        for (formal, (checked_use, forwards)) in &candidates {
            if (*checked_use || !forwards.is_empty())
                && forwards.iter().all(|target| issued.contains(target))
            {
                issued.insert(*formal);
            }
        }
        if issued.len() == before {
            break;
        }
    }
    Ok(issued)
}

#[cfg(test)]
mod tests {
    use super::super::borrowed_formal_uses;
    use super::*;

    #[test]
    fn instance_static_forward_closure_requires_retained_source_and_every_use() {
        let package = crate::mir::normal_callable_semantic_package::brand_catalog_tests::issue_with_brand_catalog(
            "static box Checked { pick(p) { if p > 0 { return 0 } return 0 } } box Forward { pass(q) { local n = Checked.pick(q) return 0 } } static box Main { main() { local f = new Forward() local n = f.pass(7) return 0 } }",
        ).unwrap();
        let source = package
            .ordinary_new_claim_ledger
            .borrowed_formal_source
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
        let formal = package
            .parameter_contracts
            .iter()
            .find(|row| {
                row.mode == CallableParameterDeclarationModeV1::InstanceBoxMethod
                    && !row.parameters.is_empty()
            })
            .unwrap()
            .parameters[0]
            .binding;
        let mut definitions = BTreeMap::new();
        for contract in &package.parameter_contracts {
            if contract.parameters.is_empty() {
                continue;
            }
            let draft = package
                .batch()
                .with_lowering_input(contract.batch_slot, |input| {
                    borrowed_formal_uses::draft_borrowed_formal_uses_v1(
                        input,
                        contract,
                        &package.instance_constructors,
                        None,
                    )
                    .unwrap()
                })
                .unwrap();
            definitions.insert(contract.owner, draft);
        }
        let issue = |definitions: &BTreeMap<_, _>, arguments: &BTreeMap<_, _>| {
            issue_checked_static_inputs_v1(&package.parameter_contracts, definitions, arguments)
        };
        assert!(issue(&definitions, &source.static_arguments)
            .unwrap()
            .contains(&formal));
        assert!(
            !issue(&definitions, &BTreeMap::new())
                .unwrap()
                .contains(&formal),
            "no retained forward source means no Instance candidate"
        );
        let saved = definitions.remove(&formal.owner()).unwrap();
        assert!(!issue(&definitions, &source.static_arguments)
            .unwrap()
            .contains(&formal));
        definitions.insert(formal.owner(), saved);

        let draft = definitions.get_mut(&formal.owner()).unwrap();
        let site = draft.uses[0].site.clone();
        let call = source
            .static_arguments
            .values()
            .find(|row| row.formal() == formal)
            .unwrap()
            .call()
            .clone();
        let mut uses = std::mem::take(&mut draft.uses).into_vec();
        uses.push(borrowed_formal_uses::BorrowedFormalUseDraftRowV1 {
            site,
            binding: formal,
            formal,
            kind: Use::UnresolvedArgument {
                call,
                ordinal: u32::MAX,
            },
        });
        draft.uses = uses.into_boxed_slice();
        assert!(
            !issue(&definitions, &source.static_arguments)
                .unwrap()
                .contains(&formal),
            "one uncovered use vetoes the whole conditional closure"
        );
        let draft = definitions.get_mut(&formal.owner()).unwrap();
        let mut uses = std::mem::take(&mut draft.uses).into_vec();
        uses.pop().unwrap();
        draft.uses = uses.into_boxed_slice();
        let target = source
            .static_arguments
            .values()
            .find(|row| row.formal() == formal)
            .unwrap()
            .target_formal();
        let target_uses = std::mem::replace(
            &mut definitions.get_mut(&target.owner()).unwrap().uses,
            Box::new([]),
        );
        assert!(
            !issue(&definitions, &source.static_arguments)
                .unwrap()
                .contains(&formal),
            "a forwarding requirement cannot manufacture its target's check"
        );
        definitions.get_mut(&target.owner()).unwrap().uses = target_uses;
        definitions
            .get_mut(&formal.owner())
            .unwrap()
            .origins
            .remove(&formal);
        assert!(issue(&definitions, &source.static_arguments)
            .unwrap_err()
            .contains("checked-input/formal-origin"));
    }
}
