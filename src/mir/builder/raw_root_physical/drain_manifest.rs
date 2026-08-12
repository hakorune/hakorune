//! DRAIN-MANIFEST0: Raw inventory projected from sealed ledger final events.

use std::collections::BTreeSet;

use crate::mir::builder::module_draft_collector::{DraftPublicationPolicyV1, FunctionDraftKeyV1};
use crate::mir::builder::raw_expansion_receipt_ledger::{
    RawCallableMainCompatibilityDispositionV1, RawExpansionDraftRoleV1,
    RawExpansionReceiptLedgerErrorV1, RawExpansionReplacementEventV1,
    SealedRawExpansionReceiptLedgerV1,
};
use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::raw_physical_drain::{
    RawPhysicalCallableMainDispositionV1, RawPhysicalDrainKeyV1, RawPhysicalDrainManifestV1,
    RawPhysicalDrainPolicyV1, RawPhysicalDrainRoleV1, RawPhysicalDrainRouteV1,
    RawPhysicalDrainRowV1, RawPhysicalReceiptProvenanceV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawDrainManifestErrorV1 {
    Ledger(RawExpansionReceiptLedgerErrorV1),
    UnsupportedRole(RawExpansionDraftRoleV1),
    UnsupportedKey,
    DuplicateOrdinal(u32),
    DuplicateSymbol(String),
    DuplicateKey,
    MissingRootMain,
    MissingCondition,
    CallableMainDispositionMismatch,
    IllegalRoleOrder,
    PolicyMismatch,
}

impl From<RawExpansionReceiptLedgerErrorV1> for RawDrainManifestErrorV1 {
    fn from(error: RawExpansionReceiptLedgerErrorV1) -> Self {
        Self::Ledger(error)
    }
}

pub(in crate::mir::builder) fn project_raw_drain_manifest(
    ledger: &SealedRawExpansionReceiptLedgerV1,
    route: RawPhysicalDrainRouteV1,
    callable_main: RawPhysicalCallableMainDispositionV1,
) -> Result<RawPhysicalDrainManifestV1, RawDrainManifestErrorV1> {
    if ledger.callable_main() != map_callable_main(callable_main) {
        return Err(RawDrainManifestErrorV1::CallableMainDispositionMismatch);
    }

    let mut ordinals = BTreeSet::new();
    let mut symbols = BTreeSet::new();
    let mut keys = BTreeSet::new();
    let mut rows = Vec::new();
    for event in ledger.final_events_in_ordinal_order() {
        if !ordinals.insert(event.ordinal()) {
            return Err(RawDrainManifestErrorV1::DuplicateOrdinal(event.ordinal()));
        }
        if !symbols.insert(event.symbol().to_owned()) {
            return Err(RawDrainManifestErrorV1::DuplicateSymbol(
                event.symbol().to_owned(),
            ));
        }
        let key = map_key(event.key())?;
        if !keys.insert(key_debug_key(&key)) {
            return Err(RawDrainManifestErrorV1::DuplicateKey);
        }
        let role = map_role(event.role())?;
        let policy = map_policy(event.policy());
        if !role_policy_matches(role, policy) {
            return Err(RawDrainManifestErrorV1::PolicyMismatch);
        }
        let provenance = map_provenance(event.replacement())?;
        rows.push(RawPhysicalDrainRowV1::new(
            event.ordinal(),
            role,
            key,
            event.symbol().to_owned().into_boxed_str(),
            event.arity(),
            policy,
            provenance,
        ));
    }

    validate_topology(&rows, route, callable_main)?;
    Ok(RawPhysicalDrainManifestV1::new(
        ledger.brand(),
        route,
        rows.into_boxed_slice(),
        callable_main,
    ))
}

fn map_callable_main(
    disposition: RawPhysicalCallableMainDispositionV1,
) -> RawCallableMainCompatibilityDispositionV1 {
    match disposition {
        RawPhysicalCallableMainDispositionV1::NotSelected => {
            RawCallableMainCompatibilityDispositionV1::NotSelected
        }
        RawPhysicalCallableMainDispositionV1::Selected => {
            RawCallableMainCompatibilityDispositionV1::Selected
        }
    }
}

fn map_key(key: &FunctionDraftKeyV1) -> Result<RawPhysicalDrainKeyV1, RawDrainManifestErrorV1> {
    match key {
        FunctionDraftKeyV1::Main => Ok(RawPhysicalDrainKeyV1::RootMain),
        FunctionDraftKeyV1::SyntheticConditionFn => Ok(RawPhysicalDrainKeyV1::RequiredCondition),
        FunctionDraftKeyV1::LegacySymbol(symbol) => Ok(RawPhysicalDrainKeyV1::LegacySymbol(
            symbol.clone().into_boxed_str(),
        )),
        FunctionDraftKeyV1::CanonicalResolvedOwner(_)
        | FunctionDraftKeyV1::CanonicalCallable(_)
        | FunctionDraftKeyV1::CatalogedBoxMethod(_) => {
            Err(RawDrainManifestErrorV1::UnsupportedKey)
        }
    }
}

fn key_debug_key(key: &RawPhysicalDrainKeyV1) -> String {
    match key {
        RawPhysicalDrainKeyV1::RootMain => "root-main".to_owned(),
        RawPhysicalDrainKeyV1::RequiredCondition => "required-condition".to_owned(),
        RawPhysicalDrainKeyV1::LegacySymbol(symbol) => format!("legacy:{symbol}"),
    }
}

fn map_role(
    role: RawExpansionDraftRoleV1,
) -> Result<RawPhysicalDrainRoleV1, RawDrainManifestErrorV1> {
    match role {
        RawExpansionDraftRoleV1::StaticMethod => Ok(RawPhysicalDrainRoleV1::StaticHelper),
        RawExpansionDraftRoleV1::CallableMainCompatibility => {
            Ok(RawPhysicalDrainRoleV1::CallableMainCompatibility)
        }
        RawExpansionDraftRoleV1::RootMain => Ok(RawPhysicalDrainRoleV1::RootMain),
        RawExpansionDraftRoleV1::SyntheticConditionFn => {
            Ok(RawPhysicalDrainRoleV1::RequiredCondition)
        }
        other => Err(RawDrainManifestErrorV1::UnsupportedRole(other)),
    }
}

fn map_policy(policy: DraftPublicationPolicyV1) -> RawPhysicalDrainPolicyV1 {
    match policy {
        DraftPublicationPolicyV1::LegacyReplaceWholePair => {
            RawPhysicalDrainPolicyV1::LegacyReplaceWholePair
        }
        DraftPublicationPolicyV1::CanonicalRejectDuplicate => {
            RawPhysicalDrainPolicyV1::CanonicalRejectDuplicate
        }
    }
}

fn role_policy_matches(role: RawPhysicalDrainRoleV1, policy: RawPhysicalDrainPolicyV1) -> bool {
    match role {
        RawPhysicalDrainRoleV1::RequiredCondition => {
            policy == RawPhysicalDrainPolicyV1::CanonicalRejectDuplicate
        }
        _ => policy == RawPhysicalDrainPolicyV1::LegacyReplaceWholePair,
    }
}

fn map_provenance(
    replacement: &RawExpansionReplacementEventV1,
) -> Result<RawPhysicalReceiptProvenanceV1, RawDrainManifestErrorV1> {
    match replacement {
        RawExpansionReplacementEventV1::Inserted => Ok(RawPhysicalReceiptProvenanceV1::Inserted),
        RawExpansionReplacementEventV1::ReplacedWholePair {
            previous_key,
            previous_symbol,
        } => Ok(RawPhysicalReceiptProvenanceV1::ReplacedWholePair {
            previous_key: map_key(previous_key)?,
            previous_symbol: previous_symbol.clone(),
        }),
    }
}

fn validate_topology(
    rows: &[RawPhysicalDrainRowV1],
    route: RawPhysicalDrainRouteV1,
    callable_main: RawPhysicalCallableMainDispositionV1,
) -> Result<(), RawDrainManifestErrorV1> {
    let root_count = rows
        .iter()
        .filter(|row| row.role() == RawPhysicalDrainRoleV1::RootMain)
        .count();
    let condition_count = rows
        .iter()
        .filter(|row| row.role() == RawPhysicalDrainRoleV1::RequiredCondition)
        .count();
    let callable_count = rows
        .iter()
        .filter(|row| row.role() == RawPhysicalDrainRoleV1::CallableMainCompatibility)
        .count();
    if root_count != 1 {
        return Err(RawDrainManifestErrorV1::MissingRootMain);
    }
    if condition_count != 1 {
        return Err(RawDrainManifestErrorV1::MissingCondition);
    }
    let expected_callable =
        usize::from(callable_main == RawPhysicalCallableMainDispositionV1::Selected);
    if callable_count != expected_callable
        || (route == RawPhysicalDrainRouteV1::Script && callable_count != 0)
    {
        return Err(RawDrainManifestErrorV1::CallableMainDispositionMismatch);
    }
    let mut seen_root = false;
    let mut seen_condition = false;
    let mut seen_callable = false;
    for row in rows {
        match row.role() {
            RawPhysicalDrainRoleV1::StaticHelper if seen_root => {
                return Err(RawDrainManifestErrorV1::IllegalRoleOrder)
            }
            RawPhysicalDrainRoleV1::CallableMainCompatibility if seen_root || seen_callable => {
                return Err(RawDrainManifestErrorV1::IllegalRoleOrder)
            }
            RawPhysicalDrainRoleV1::CallableMainCompatibility => seen_callable = true,
            RawPhysicalDrainRoleV1::RootMain if seen_root || seen_condition => {
                return Err(RawDrainManifestErrorV1::IllegalRoleOrder)
            }
            RawPhysicalDrainRoleV1::RootMain => seen_root = true,
            RawPhysicalDrainRoleV1::RequiredCondition if !seen_root || seen_condition => {
                return Err(RawDrainManifestErrorV1::IllegalRoleOrder)
            }
            RawPhysicalDrainRoleV1::RequiredCondition => seen_condition = true,
            RawPhysicalDrainRoleV1::StaticHelper => {}
        }
    }
    if !seen_condition {
        return Err(RawDrainManifestErrorV1::MissingCondition);
    }
    Ok(())
}
