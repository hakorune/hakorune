//! Source-only checked-input closure for original Static borrowed formals.
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
    let formals: BTreeSet<_> = contracts
        .iter()
        .filter(|row| row.mode == CallableParameterDeclarationModeV1::StaticBoxMethod)
        .flat_map(|row| row.parameters.iter())
        .filter(|row| row.kind.is_ordinary_borrowed_handle())
        .map(|row| row.binding)
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
