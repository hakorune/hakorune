//! Physical origin projection for one source-backed Dynamic callable.
//!
//! The source product owns semantic classification. This state only binds
//! that classification to values issued by the existing callable-entry and
//! local-statement terminals.

use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceBindingSiteV1};
use crate::mir::ValueId;

use super::normal_callable_binding_materialization_port::PreparedCallableEntryValuesV1;
use super::normal_callable_dynamic_source::VerifiedSourceBackedDynamicCallableV1;
use super::stmts::CompletedLocalBindingV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CallableDynamicOriginErrorV1 {
    ForeignFormal(BindingRefV1),
    DuplicateFormalOrdinal(u32),
    DuplicateFormalBinding(BindingRefV1),
    ForeignLocal(BindingRefV1),
    MissingFormalOrigin(BindingRefV1),
    DuplicateLocalBinding(BindingRefV1),
    DuplicateEntryInstall,
    EntryShapeMismatch,
    FormalOrdinalMismatch(u32),
    DuplicatePhysicalValue(ValueId),
    LocalShapeMismatch,
    LocalOrdinalMismatch(u32),
    LocalBindingMismatch(BindingRefV1),
    AliasExpectationMissing {
        binding: BindingRefV1,
        expected_locals: Box<[BindingRefV1]>,
    },
    AliasDeclarationMismatch(BindingRefV1),
    AliasFormalMismatch(BindingRefV1),
    AliasOrdinalMismatch {
        binding: BindingRefV1,
        expected: u32,
        actual: u32,
    },
    AliasAlreadyCompleted(BindingRefV1),
    InitializerOriginMismatch(BindingRefV1),
    DuplicateLocalCompletion(BindingRefV1),
    StaleRebindOrigin(BindingRefV1),
    RebindOriginMismatch(BindingRefV1),
    RebindResultReusesCurrent(ValueId),
    DuplicateRebindResult(ValueId),
    IncompleteConsumption,
}

impl std::fmt::Display for CallableDynamicOriginErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][callable-dynamic-origin/{self:?}]"
        )
    }
}

impl std::error::Error for CallableDynamicOriginErrorV1 {}

#[derive(Debug, Clone)]
struct DynamicLocalExpectationV1 {
    formal: BindingRefV1,
    declaration: SourceBindingSiteV1,
}

/// Exact completed-local relation retained for a later canonical SSA handoff.
///
/// This row allocates no value and owns no reaching-value decision. It keeps
/// the existing local terminal's source/materialization relation intact until
/// the canonical identity owner can adopt the same entry value.
#[derive(Debug, PartialEq, Eq)]
pub(super) struct PreparedDynamicLocalEntryV1 {
    declaration: SourceBindingSiteV1,
    formal: BindingRefV1,
    binding: BindingRefV1,
    initializer: ValueId,
    local: ValueId,
}

impl PreparedDynamicLocalEntryV1 {
    pub(super) const fn declaration(&self) -> &SourceBindingSiteV1 {
        &self.declaration
    }

    pub(super) const fn formal(&self) -> BindingRefV1 {
        self.formal
    }

    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn initializer(&self) -> ValueId {
        self.initializer
    }

    pub(super) const fn local(&self) -> ValueId {
        self.local
    }
}

#[derive(Debug)]
pub(super) struct CallableDynamicOriginLoweringStateV1 {
    owner: FunctionOwnerIdV1,
    source: Rc<VerifiedSourceBackedDynamicCallableV1>,
    formal_by_ordinal: BTreeMap<u32, BindingRefV1>,
    local_expectations: BTreeMap<BindingRefV1, DynamicLocalExpectationV1>,
    active_origins: BTreeMap<BindingRefV1, (ValueId, BindingRefV1)>,
    value_origins: BTreeMap<ValueId, BindingRefV1>,
    local_entries: BTreeMap<BindingRefV1, PreparedDynamicLocalEntryV1>,
    completed_locals: BTreeSet<BindingRefV1>,
    entry_installed: bool,
}

#[derive(Debug)]
pub(super) struct PreparedDynamicOriginRebindV1 {
    binding: BindingRefV1,
    previous: ValueId,
    result: ValueId,
    origin: BindingRefV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) struct CurrentDynamicBindingReceiptV1 {
    binding: BindingRefV1,
    previous: ValueId,
    current: ValueId,
    origin: BindingRefV1,
}

impl CurrentDynamicBindingReceiptV1 {
    pub(super) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn previous(&self) -> ValueId {
        self.previous
    }

    pub(super) const fn current(&self) -> ValueId {
        self.current
    }

    pub(super) const fn origin(&self) -> BindingRefV1 {
        self.origin
    }
}

impl CallableDynamicOriginLoweringStateV1 {
    pub(super) fn from_source(
        source: VerifiedSourceBackedDynamicCallableV1,
    ) -> Result<Self, CallableDynamicOriginErrorV1> {
        Self::from_shared_source(Rc::new(source))
    }

    pub(super) fn from_shared_source(
        source: Rc<VerifiedSourceBackedDynamicCallableV1>,
    ) -> Result<Self, CallableDynamicOriginErrorV1> {
        let owner = source.owner();
        let mut formal_by_ordinal = BTreeMap::new();
        let mut formal_bindings = BTreeSet::new();
        for row in source.formals() {
            if row.binding().owner() != owner {
                return Err(CallableDynamicOriginErrorV1::ForeignFormal(row.binding()));
            }
            if formal_by_ordinal
                .insert(row.parameter_ordinal(), row.binding())
                .is_some()
            {
                return Err(CallableDynamicOriginErrorV1::DuplicateFormalOrdinal(
                    row.parameter_ordinal(),
                ));
            }
            if !formal_bindings.insert(row.binding()) {
                return Err(CallableDynamicOriginErrorV1::DuplicateFormalBinding(
                    row.binding(),
                ));
            }
        }

        let mut local_expectations = BTreeMap::new();
        for row in source.local_initializations() {
            if row.local().owner() != owner {
                return Err(CallableDynamicOriginErrorV1::ForeignLocal(row.local()));
            }
            if !formal_bindings.contains(&row.formal()) {
                return Err(CallableDynamicOriginErrorV1::MissingFormalOrigin(
                    row.formal(),
                ));
            }
            if local_expectations
                .insert(
                    row.local(),
                    DynamicLocalExpectationV1 {
                        formal: row.formal(),
                        declaration: row.declaration().clone(),
                    },
                )
                .is_some()
            {
                return Err(CallableDynamicOriginErrorV1::DuplicateLocalBinding(
                    row.local(),
                ));
            }
        }

        Ok(Self {
            owner,
            source,
            formal_by_ordinal,
            local_expectations,
            active_origins: BTreeMap::new(),
            value_origins: BTreeMap::new(),
            local_entries: BTreeMap::new(),
            completed_locals: BTreeSet::new(),
            entry_installed: false,
        })
    }

    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(super) fn source(&self) -> &VerifiedSourceBackedDynamicCallableV1 {
        &self.source
    }

    pub(super) fn current_binding(&self, binding: BindingRefV1) -> Option<(ValueId, BindingRefV1)> {
        self.active_origins.get(&binding).copied()
    }

    pub(super) fn local_entry(
        &self,
        binding: BindingRefV1,
    ) -> Option<&PreparedDynamicLocalEntryV1> {
        self.local_entries.get(&binding)
    }

    pub(super) fn install_entry(
        &mut self,
        parameter_bindings: &[BindingRefV1],
        entry: &PreparedCallableEntryValuesV1,
    ) -> Result<(), CallableDynamicOriginErrorV1> {
        if self.entry_installed {
            return Err(CallableDynamicOriginErrorV1::DuplicateEntryInstall);
        }
        if parameter_bindings.len() != entry.parameters().len() {
            return Err(CallableDynamicOriginErrorV1::EntryShapeMismatch);
        }
        for (&ordinal, &formal) in &self.formal_by_ordinal {
            let index = usize::try_from(ordinal)
                .map_err(|_| CallableDynamicOriginErrorV1::FormalOrdinalMismatch(ordinal))?;
            if parameter_bindings.get(index) != Some(&formal) {
                return Err(CallableDynamicOriginErrorV1::FormalOrdinalMismatch(ordinal));
            }
            let value = *entry
                .parameters()
                .get(index)
                .ok_or(CallableDynamicOriginErrorV1::FormalOrdinalMismatch(ordinal))?;
            if self.value_origins.insert(value, formal).is_some() {
                return Err(CallableDynamicOriginErrorV1::DuplicatePhysicalValue(value));
            }
            self.active_origins.insert(formal, (value, formal));
        }
        self.entry_installed = true;
        Ok(())
    }

    pub(super) fn record_local(
        &mut self,
        statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        bindings: &[BindingRefV1],
        completed: &[CompletedLocalBindingV1],
    ) -> Result<(), CallableDynamicOriginErrorV1> {
        if bindings.len() != completed.len() {
            return Err(CallableDynamicOriginErrorV1::LocalShapeMismatch);
        }
        for (index, (&binding, physical)) in bindings.iter().zip(completed).enumerate() {
            let ordinal = u32::try_from(index)
                .map_err(|_| CallableDynamicOriginErrorV1::LocalShapeMismatch)?;
            if physical.ordinal() != ordinal {
                return Err(CallableDynamicOriginErrorV1::LocalOrdinalMismatch(ordinal));
            }
            let Some(expected) = self.local_expectations.get(&binding) else {
                continue;
            };
            let SourceBindingSiteV1::Local {
                statement: expected_statement,
                ordinal: expected_ordinal,
            } = &expected.declaration
            else {
                return Err(CallableDynamicOriginErrorV1::LocalBindingMismatch(binding));
            };
            if expected_statement.node() != statement
                || *expected_ordinal != ordinal
                || physical.local() == physical.initializer()
            {
                return Err(CallableDynamicOriginErrorV1::LocalBindingMismatch(binding));
            }
            if !self.completed_locals.insert(binding) {
                return Err(CallableDynamicOriginErrorV1::DuplicateLocalCompletion(
                    binding,
                ));
            }
            let Some(&(formal_value, source_formal)) = self.active_origins.get(&expected.formal)
            else {
                return Err(CallableDynamicOriginErrorV1::MissingFormalOrigin(
                    expected.formal,
                ));
            };
            if source_formal != expected.formal
                || physical.initializer() != formal_value
                || self.value_origins.get(&physical.initializer()) != Some(&source_formal)
            {
                return Err(CallableDynamicOriginErrorV1::InitializerOriginMismatch(
                    binding,
                ));
            }
            if self
                .value_origins
                .insert(physical.local(), source_formal)
                .is_some()
            {
                return Err(CallableDynamicOriginErrorV1::DuplicatePhysicalValue(
                    physical.local(),
                ));
            }
            self.active_origins
                .insert(binding, (physical.local(), source_formal));
            let entry = PreparedDynamicLocalEntryV1 {
                declaration: expected.declaration.clone(),
                formal: expected.formal,
                binding,
                initializer: physical.initializer(),
                local: physical.local(),
            };
            if self.local_entries.insert(binding, entry).is_some() {
                return Err(CallableDynamicOriginErrorV1::DuplicateLocalCompletion(
                    binding,
                ));
            }
        }
        Ok(())
    }

    pub(super) fn record_alias_local(
        &mut self,
        statement: &crate::mir::resolved_semantics::SourceNodeSiteV1,
        binding: BindingRefV1,
        formal: BindingRefV1,
        value: ValueId,
        expected_ordinal: u32,
    ) -> Result<(), CallableDynamicOriginErrorV1> {
        let Some(expected) = self.local_expectations.get(&binding) else {
            return Err(CallableDynamicOriginErrorV1::AliasExpectationMissing {
                binding,
                expected_locals: self.local_expectations.keys().copied().collect(),
            });
        };
        let SourceBindingSiteV1::Local {
            statement: expected_statement,
            ordinal,
        } = &expected.declaration
        else {
            return Err(CallableDynamicOriginErrorV1::AliasDeclarationMismatch(
                binding,
            ));
        };
        if expected.formal != formal {
            return Err(CallableDynamicOriginErrorV1::AliasFormalMismatch(binding));
        }
        if expected_statement.node() != statement {
            return Err(CallableDynamicOriginErrorV1::AliasDeclarationMismatch(
                binding,
            ));
        }
        if *ordinal != expected_ordinal {
            return Err(CallableDynamicOriginErrorV1::AliasOrdinalMismatch {
                binding,
                expected: *ordinal,
                actual: expected_ordinal,
            });
        }
        if !self.completed_locals.insert(binding) {
            return Err(CallableDynamicOriginErrorV1::AliasAlreadyCompleted(binding));
        }
        let Some(&(formal_value, source_formal)) = self.active_origins.get(&formal) else {
            return Err(CallableDynamicOriginErrorV1::MissingFormalOrigin(formal));
        };
        if source_formal != formal || formal_value != value {
            return Err(CallableDynamicOriginErrorV1::InitializerOriginMismatch(
                binding,
            ));
        }
        if self
            .active_origins
            .insert(binding, (value, formal))
            .is_some()
        {
            return Err(CallableDynamicOriginErrorV1::DuplicateLocalCompletion(
                binding,
            ));
        }
        let entry = PreparedDynamicLocalEntryV1 {
            declaration: expected.declaration.clone(),
            formal,
            binding,
            initializer: value,
            local: value,
        };
        if self.local_entries.insert(binding, entry).is_some() {
            return Err(CallableDynamicOriginErrorV1::DuplicateLocalCompletion(
                binding,
            ));
        }
        Ok(())
    }

    pub(super) fn invalidate_rebind(
        &mut self,
        binding: BindingRefV1,
        previous: ValueId,
    ) -> Result<(), CallableDynamicOriginErrorV1> {
        let Some((active, _)) = self.active_origins.get(&binding).copied() else {
            return Ok(());
        };
        if active != previous {
            return Err(CallableDynamicOriginErrorV1::StaleRebindOrigin(binding));
        }
        self.active_origins.remove(&binding);
        Ok(())
    }

    pub(super) fn prepare_current_rebind(
        &self,
        binding: BindingRefV1,
        previous: ValueId,
        result: ValueId,
        expected_origin: BindingRefV1,
    ) -> Result<PreparedDynamicOriginRebindV1, CallableDynamicOriginErrorV1> {
        let Some((active, origin)) = self.active_origins.get(&binding).copied() else {
            return Err(CallableDynamicOriginErrorV1::StaleRebindOrigin(binding));
        };
        if active != previous {
            return Err(CallableDynamicOriginErrorV1::StaleRebindOrigin(binding));
        }
        if origin != expected_origin {
            return Err(CallableDynamicOriginErrorV1::RebindOriginMismatch(binding));
        }
        if result == previous {
            return Err(CallableDynamicOriginErrorV1::RebindResultReusesCurrent(
                result,
            ));
        }
        if self.value_origins.contains_key(&result) {
            return Err(CallableDynamicOriginErrorV1::DuplicateRebindResult(result));
        }
        Ok(PreparedDynamicOriginRebindV1 {
            binding,
            previous,
            result,
            origin,
        })
    }

    pub(super) fn commit_current_rebind(
        &mut self,
        prepared: PreparedDynamicOriginRebindV1,
    ) -> CurrentDynamicBindingReceiptV1 {
        debug_assert_eq!(
            self.active_origins.get(&prepared.binding),
            Some(&(prepared.previous, prepared.origin))
        );
        debug_assert!(!self.value_origins.contains_key(&prepared.result));
        self.value_origins.insert(prepared.result, prepared.origin);
        self.active_origins
            .insert(prepared.binding, (prepared.result, prepared.origin));
        CurrentDynamicBindingReceiptV1 {
            binding: prepared.binding,
            previous: prepared.previous,
            current: prepared.result,
            origin: prepared.origin,
        }
    }

    pub(super) fn current_origin(
        &self,
        binding: BindingRefV1,
        value: ValueId,
    ) -> Option<BindingRefV1> {
        self.active_origins
            .get(&binding)
            .and_then(|(active, origin)| (*active == value).then_some(*origin))
    }

    pub(super) fn value_origin(&self, value: ValueId) -> Option<BindingRefV1> {
        self.value_origins.get(&value).copied()
    }

    pub(super) fn finish(self) -> Result<(), CallableDynamicOriginErrorV1> {
        if !self.entry_installed || self.completed_locals.len() != self.local_expectations.len() {
            return Err(CallableDynamicOriginErrorV1::IncompleteConsumption);
        }
        let _ = self.source;
        Ok(())
    }
}

#[cfg(test)]
#[path = "normal_callable_dynamic_origin_tests.rs"]
mod tests;
