//! Installed-package accounting for exact source obligations and their emissions.
use super::{EmittedNamedArrayRequirementV1, SelectedSourceCoreMethodCallV1};
use crate::mir::builder::{CanonicalSameModuleCallableKeyV1, SelectedNormalCallableKeyV1};
use crate::mir::named_array_obligation::fault;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};
use std::{cell::RefCell, collections::BTreeMap};

type Key = (CanonicalSameModuleCallableKeyV1, SourceExprSiteV1);

#[derive(Debug)]
pub(crate) struct NamedArrayEmissionCollectorV1 {
    expected: BTreeMap<Key, FunctionOwnerIdV1>,
    emitted: RefCell<BTreeMap<Key, EmittedNamedArrayRequirementV1>>,
}

impl NamedArrayEmissionCollectorV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn from_source_rows(
        rows: &BTreeMap<
            SelectedNormalCallableKeyV1,
            BTreeMap<SourceExprSiteV1, SelectedSourceCoreMethodCallV1>,
        >,
    ) -> Self {
        let expected = rows
            .values()
            .flat_map(|rows| rows.values())
            .filter_map(|row| {
                row.contract().named_array_requirement().map(|requirement| {
                    (
                        (row.caller.clone(), requirement.call().clone()),
                        requirement.owner(),
                    )
                })
            })
            .collect();
        Self {
            expected,
            emitted: RefCell::new(BTreeMap::new()),
        }
    }

    pub(crate) fn hand_back(
        &self,
        rows: Vec<EmittedNamedArrayRequirementV1>,
    ) -> Result<(), String> {
        let mut emitted = self.emitted.borrow_mut();
        for row in rows {
            let key = (row.caller().clone(), row.call_site().clone());
            if self.expected.get(&key) != Some(&row.owner()) {
                return Err(fault("foreign-emission-handback"));
            }
            if emitted.contains_key(&key) {
                return Err(fault("duplicate-emission-handback"));
            }
            emitted.insert(key, row);
        }
        Ok(())
    }

    pub(in crate::mir::normal_callable_semantic_package) fn finish(
        self,
    ) -> Result<Box<[EmittedNamedArrayRequirementV1]>, String> {
        let emitted = self.emitted.into_inner();
        if emitted.len() != self.expected.len()
            || self.expected.keys().any(|key| !emitted.contains_key(key))
        {
            return Err(fault("unconsumed-package-obligations"));
        }
        Ok(emitted.into_values().collect())
    }
}
