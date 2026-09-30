//! Final root handoff after source and physical validation are complete.
use super::*;
use std::collections::BTreeSet;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn seal_finalized_root_birth_handoff(
        &self,
        root_key: String,
        construction_keys: &BTreeSet<CanonicalSameModuleCallableKeyV1>,
        callables: Option<
            crate::mir::normal_callable_semantic_package::VerifiedCallableResultContractCohortV1,
        >,
    ) -> Result<FinalizedRootHandoffV1, String> {
        match *self.root_validation.borrow() {
            RootNewValidation::FinishingChecked => {}
            _ => return Err(freeze("artifact-root-not-finished")),
        }
        let owner = match self.root_completion.as_ref() {
            Some(Ok(completion)) => completion.owner(),
            _ => return Err(freeze("artifact-root-completion-unavailable")),
        };
        // Structural exclusivity replaces collision checks, not physical
        // progress. Every exit row stands on its own site: no relation may
        // satisfy, or be satisfied by, a sibling exit's evidence.
        for (exit, terminal) in &self.terminal_relation {
            if terminal.return_site() != exit {
                return Err(freeze("artifact-root-site-drift"));
            }
            match terminal {
                TerminalRelationV1::Call(relation) => {
                    if relation.owner() != owner {
                        return Err(freeze("artifact-call-owner-drift"));
                    }
                }
                TerminalRelationV1::I64Add(relation) => {
                    if relation.owner() != owner {
                        return Err(freeze("artifact-root-result-unavailable"));
                    }
                }
                TerminalRelationV1::Unit(relation) => {
                    if relation.owner() != owner {
                        return Err(freeze("artifact-root-unit-owner-drift"));
                    }
                    let completion = self
                        .root_completion
                        .as_ref()
                        .and_then(|row| row.as_ref().ok())
                        .ok_or_else(|| freeze("artifact-root-completion-unavailable"))?;
                    if !completion.explicit_sites().contains(relation.return_site()) {
                        return Err(freeze("artifact-root-unit-site-drift"));
                    }
                }
                TerminalRelationV1::IntegerLiteral(relation) => {
                    if relation.owner() != owner
                        || !self
                            .terminal_integer_literal_value
                            .borrow()
                            .contains_key(relation.return_site())
                    {
                        return Err(freeze("artifact-root-literal-unavailable"));
                    }
                }
                TerminalRelationV1::I64Field(relation) => {
                    if relation.owner() != owner {
                        return Err(freeze("artifact-root-field-unavailable"));
                    }
                }
                TerminalRelationV1::Value(relation) => {
                    if relation.owner() != owner {
                        return Err(freeze("artifact-root-value-owner-drift"));
                    }
                }
                TerminalRelationV1::OpaqueCall(relation) => {
                    if relation.owner() != owner {
                        return Err(freeze("artifact-root-opaque-call-owner-drift"));
                    }
                }
                TerminalRelationV1::MapGet(relation) => {
                    if relation.owner() != owner
                        || !self.terminal_map_get_return_emitted_at(owner, exit)
                    {
                        return Err(freeze("artifact-root-map-get-unavailable"));
                    }
                }
            }
        }
        if self
            .terminal_relation
            .values()
            .any(|terminal| matches!(terminal, TerminalRelationV1::I64Add(_)))
            && !self.terminal_result_complete()
        {
            return Err(freeze("artifact-root-result-unavailable"));
        }
        if self
            .terminal_relation
            .values()
            .any(|terminal| matches!(terminal, TerminalRelationV1::I64Field(_)))
            && !self.terminal_i64_field_return_complete()
        {
            return Err(freeze("artifact-root-field-unavailable"));
        }
        let mut call_entries = std::collections::BTreeMap::new();
        for (site, terminal) in &self.terminal_relation {
            if !matches!(terminal, TerminalRelationV1::Call(_)) {
                continue;
            }
            let (entry, cleanup) = self
                .take_finalized_root_call(owner, site)?
                .ok_or_else(|| freeze("artifact-call-physical-missing"))?;
            call_entries.insert(site.clone(), (entry, cleanup));
        }
        // A Call payload emitted at an exit whose relation is not a Call is
        // drift: the physical entry must never exist without its source row.
        {
            let exits = self.root_exits.borrow();
            for ((row_owner, site), progress) in exits.iter() {
                if *row_owner != owner {
                    continue;
                }
                if matches!(
                    progress,
                    RootHomeExitProgress::Emitted {
                        entry: RootHomeExitEntry::Call { .. },
                        ..
                    }
                ) && !call_entries.contains_key(site)
                {
                    return Err(freeze("artifact-call-terminal-drift"));
                }
            }
        }
        if self.terminal_relation.is_empty() && !call_entries.is_empty() {
            return Err(freeze("artifact-call-root-source-missing"));
        }
        let root_source = (!self.terminal_relation.is_empty())
            .then(|| {
                Ok::<_, String>(FinalizedRootSourceHandoffV1 {
                    app_main_identity: self
                        .app_main_identity
                        .as_ref()
                        .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?
                        .clone(),
                    terminals: self.terminal_relation.clone(),
                    call_entries,
                })
            })
            .transpose()?;
        let mut keys = BTreeSet::new();
        let mut births = Vec::new();
        let mut actuals = Vec::new();
        for (site, row) in self.local_commits.borrow().iter() {
            if !row.is_complete() {
                return Err(freeze("artifact-local-commit-incomplete"));
            }
            let (birth_target, birth_abi, object, binding, emission) = match row {
                LocalCommitV1::Ordinary(row) => (
                    &row.birth_target,
                    &row.birth_abi,
                    row.object,
                    Some(row.binding),
                    &row.emission,
                ),
                // A return-position `new` is the same sealed Birth edge: it
                // installs no local destination, so the site owner is the
                // only owner authority.
                LocalCommitV1::Result(row) => {
                    if row.site != *site {
                        return Err(freeze("artifact-birth-site-drift"));
                    }
                    (
                        &row.birth_target,
                        &row.birth_abi,
                        row.object,
                        None,
                        &row.emission,
                    )
                }
                LocalCommitV1::Map(_) | LocalCommitV1::CallReceived(_) => continue,
            };
            let Some(key) = birth_target else {
                if birth_abi.is_some() {
                    return Err(freeze("artifact-birth-abi-without-target"));
                }
                continue;
            };
            let key = key.clone();
            let relation = birth_abi
                .as_ref()
                .ok_or_else(|| freeze("artifact-birth-abi-missing"))?;
            if relation.target() != &key || relation.owner() == site.owner() {
                return Err(freeze("artifact-birth-abi-drift"));
            }
            if let Some(binding) = binding {
                if binding.owner() != site.owner() {
                    return Err(freeze("artifact-birth-owner-drift"));
                }
            }
            if relation.object() != object {
                return Err(freeze("artifact-birth-object-drift"));
            }
            if !construction_keys.contains(&key) {
                return Err(freeze("artifact-birth-construction-missing"));
            }
            if let NewEmissionProgress::Emitted {
                result,
                arguments,
                progress: EmittedLocalProgress::Checked { .. },
                ..
            } = emission
            {
                actuals.push(FinalizedBirthActualsV1 {
                    site: site.clone(),
                    destination: binding,
                    target: key.clone(),
                    receiver: *result,
                    arguments: arguments.clone(),
                });
            }
            // RetainedUnavailable remains unavailable: never invent an empty actual.
            // Multiple exact New sites may invoke one canonical Birth
            // definition. Local emission validation above remains per site;
            // the final handoff retains each definition relation once.
            if keys.insert(key.clone()) {
                births.push(relation.clone());
            } else if !births.iter().any(|existing| existing == relation) {
                return Err(freeze("artifact-birth-abi-duplicate-drift"));
            }
        }
        if root_source.is_none() && !actuals.is_empty() {
            return Err(freeze("artifact-actual-root-source-missing"));
        }
        let birth_actuals = actuals.into_boxed_slice();
        Ok(if births.is_empty() {
            FinalizedRootHandoffV1::NoBirth {
                named_arrays: Box::new([]),
                callables,
                root_key,
                root_source,
                birth_actuals,
            }
        } else {
            FinalizedRootHandoffV1::Births {
                named_arrays: Box::new([]),
                callables,
                root_key,
                root_source,
                birth_actuals,
                keys: keys.into_iter().collect(),
                births: births.into_boxed_slice(),
            }
        })
    }
}
