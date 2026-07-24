//! DRAIN-COLLECTOR0: Raw ledger-keyed collector parity and one-shot drain.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::module_invocation_identity::ModuleInvocationBrandV1;
use crate::mir::builder::module_invocation_owner_chain::InvocationBranded;
use crate::mir::raw_physical_drain::{RawPhysicalDrainKeyV1, RawPhysicalDrainManifestV1};
use crate::mir::MirFunction;

use super::{FunctionDraftKeyV1, ModuleDraftCollectorV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawCollectorDrainErrorV1 {
    BrandMismatch,
    CountMismatch {
        expected: usize,
        actual: usize,
    },
    MissingKey(FunctionDraftKeyV1),
    SurplusKey(FunctionDraftKeyV1),
    SymbolIndexDrift(String),
    SymbolMismatch {
        expected: String,
        actual: String,
    },
    ArityMismatch {
        symbol: String,
        expected: usize,
        actual: usize,
    },
    DuplicateManifestKey,
    DuplicateManifestSymbol(String),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RejectedRawCollectorDrainV1 {
    collector: ModuleDraftCollectorV1,
    error: RawCollectorDrainErrorV1,
}

impl RejectedRawCollectorDrainV1 {
    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (ModuleDraftCollectorV1, RawCollectorDrainErrorV1) {
        (self.collector, self.error)
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedRawCollectorDrainV1 {
    collector: ModuleDraftCollectorV1,
    ordered_keys: Box<[FunctionDraftKeyV1]>,
    _seal: PreparedRawCollectorDrainSealV1,
}

#[derive(Debug)]
struct PreparedRawCollectorDrainSealV1;

impl PreparedRawCollectorDrainV1 {
    pub(in crate::mir::builder) fn drain(self) -> Vec<MirFunction> {
        let Self {
            mut collector,
            ordered_keys,
            _seal: _,
        } = self;
        ordered_keys
            .into_vec()
            .into_iter()
            .map(|key| {
                collector
                    .drafts
                    .remove(&key)
                    .expect("prepared Raw collector key must own one draft")
                    .draft
            })
            .collect()
    }
}

impl ModuleDraftCollectorV1 {
    pub(in crate::mir::builder) fn prepare_raw_drain(
        self,
        manifest: &RawPhysicalDrainManifestV1,
        brand: ModuleInvocationBrandV1,
    ) -> Result<PreparedRawCollectorDrainV1, RejectedRawCollectorDrainV1> {
        if self.receipt_brand != Some(brand) {
            return Err(reject(self, RawCollectorDrainErrorV1::BrandMismatch));
        }
        let mut expected = Vec::with_capacity(manifest.rows().len());
        let mut expected_keys = BTreeSet::new();
        let mut expected_symbols = BTreeSet::new();
        for row in manifest.rows() {
            let key = map_key(row.key());
            if !expected_keys.insert(key.clone()) {
                return Err(reject(self, RawCollectorDrainErrorV1::DuplicateManifestKey));
            }
            if !expected_symbols.insert(row.symbol().to_owned()) {
                return Err(reject(
                    self,
                    RawCollectorDrainErrorV1::DuplicateManifestSymbol(row.symbol().to_owned()),
                ));
            }
            expected.push((key, row.symbol().to_owned(), row.arity()));
        }
        if self.drafts.len() != expected.len() || self.key_by_symbol.len() != expected.len() {
            let actual = self.drafts.len();
            return Err(reject(
                self,
                RawCollectorDrainErrorV1::CountMismatch {
                    expected: expected.len(),
                    actual,
                },
            ));
        }
        let mut expected_by_key = BTreeMap::new();
        for (key, symbol, arity) in &expected {
            expected_by_key.insert(key.clone(), (symbol.as_str(), *arity));
            let Some((actual_symbol, actual_arity)) = self.drafts.get(key).map(|entry| {
                (
                    entry.draft.signature.name.clone(),
                    entry.draft.signature.params.len(),
                )
            }) else {
                return Err(reject(
                    self,
                    RawCollectorDrainErrorV1::MissingKey(key.clone()),
                ));
            };
            if actual_symbol != *symbol {
                return Err(reject(
                    self,
                    RawCollectorDrainErrorV1::SymbolMismatch {
                        expected: symbol.clone(),
                        actual: actual_symbol,
                    },
                ));
            }
            if actual_arity != *arity {
                return Err(reject(
                    self,
                    RawCollectorDrainErrorV1::ArityMismatch {
                        symbol: symbol.clone(),
                        expected: *arity,
                        actual: actual_arity,
                    },
                ));
            }
            if self.key_by_symbol.get(symbol) != Some(key) {
                return Err(reject(
                    self,
                    RawCollectorDrainErrorV1::SymbolIndexDrift(symbol.clone()),
                ));
            }
        }
        if self
            .drafts
            .keys()
            .any(|key| !expected_by_key.contains_key(key))
        {
            let key = self
                .drafts
                .keys()
                .find(|key| !expected_by_key.contains_key(*key))
                .cloned()
                .expect("surplus key exists");
            return Err(reject(self, RawCollectorDrainErrorV1::SurplusKey(key)));
        }
        Ok(PreparedRawCollectorDrainV1 {
            collector: self,
            ordered_keys: expected.into_iter().map(|(key, _, _)| key).collect(),
            _seal: PreparedRawCollectorDrainSealV1,
        })
    }
}

fn map_key(key: &RawPhysicalDrainKeyV1) -> FunctionDraftKeyV1 {
    match key {
        RawPhysicalDrainKeyV1::RootMain => FunctionDraftKeyV1::Main,
        RawPhysicalDrainKeyV1::RequiredCondition => FunctionDraftKeyV1::SyntheticConditionFn,
        RawPhysicalDrainKeyV1::LegacySymbol(symbol) => {
            FunctionDraftKeyV1::LegacySymbol(symbol.to_string())
        }
    }
}

fn reject(
    collector: ModuleDraftCollectorV1,
    error: RawCollectorDrainErrorV1,
) -> RejectedRawCollectorDrainV1 {
    RejectedRawCollectorDrainV1 { collector, error }
}

pub(in crate::mir::builder) fn raw_collector_from_branded(
    collector: InvocationBranded<ModuleDraftCollectorV1>,
    manifest: &RawPhysicalDrainManifestV1,
    brand: ModuleInvocationBrandV1,
) -> Result<
    PreparedRawCollectorDrainV1,
    (
        InvocationBranded<ModuleDraftCollectorV1>,
        RawCollectorDrainErrorV1,
    ),
> {
    if collector.brand() != brand {
        return Err((collector, RawCollectorDrainErrorV1::BrandMismatch));
    }
    match collector.into_payload().prepare_raw_drain(manifest, brand) {
        Ok(prepared) => Ok(prepared),
        Err(rejected) => {
            let (collector, error) = rejected.into_parts();
            Err((InvocationBranded::from_source(brand, collector), error))
        }
    }
}
