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
        // Structural exclusivity replaces collision checks, not physical progress.
        if let Some(terminal) = &self.terminal_relation {
            match terminal {
                TerminalRelationV1::Call(relation) => {
                    if relation.owner() != owner {
                        return Err(freeze("artifact-call-owner-drift"));
                    }
                }
                TerminalRelationV1::I64Add(relation) => {
                    if relation.owner() != owner || !self.terminal_result_complete() {
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
                    if completion.explicit_site() != Some(relation.return_site()) {
                        return Err(freeze("artifact-root-unit-site-drift"));
                    }
                }
                TerminalRelationV1::IntegerLiteral(relation) => {
                    if relation.owner() != owner
                        || self.terminal_integer_literal_value.borrow().is_none()
                    {
                        return Err(freeze("artifact-root-literal-unavailable"));
                    }
                }
                TerminalRelationV1::I64Field(relation) => {
                    if relation.owner() != owner || !self.terminal_i64_field_return_complete() {
                        return Err(freeze("artifact-root-field-unavailable"));
                    }
                }
            }
        }
        let call_payload = if matches!(
            self.terminal_relation.as_ref(),
            Some(TerminalRelationV1::Call(_))
        ) {
            let (entry, cleanup) = self
                .take_finalized_root_call(owner)?
                .ok_or_else(|| freeze("artifact-call-physical-missing"))?;
            Some((entry, cleanup.into_boxed_slice()))
        } else {
            if self.take_finalized_root_call(owner)?.is_some() {
                return Err(freeze("artifact-call-terminal-drift"));
            }
            None
        };
        let (call_entry, call_cleanup) = match call_payload {
            Some((entry, cleanup)) => (Some(entry), cleanup),
            None => (
                None,
                Vec::<(BasicBlockId, MirInstruction)>::new().into_boxed_slice(),
            ),
        };
        if self.terminal_relation.is_none() && call_entry.is_some() {
            return Err(freeze("artifact-call-root-source-missing"));
        }
        let root_source = self
            .terminal_relation
            .as_ref()
            .map(|terminal| {
                Ok::<_, String>(FinalizedRootSourceHandoffV1 {
                    app_main_identity: self
                        .app_main_identity
                        .as_ref()
                        .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?
                        .clone(),
                    terminal: terminal.clone(),
                    call_entry,
                    call_cleanup,
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
            let Some(row) = row.ordinary() else {
                continue;
            };
            let Some(key) = &row.birth_target else {
                if row.birth_abi.is_some() {
                    return Err(freeze("artifact-birth-abi-without-target"));
                }
                continue;
            };
            let key = key.clone();
            let relation = row
                .birth_abi
                .as_ref()
                .ok_or_else(|| freeze("artifact-birth-abi-missing"))?;
            if relation.target() != &key || relation.owner() == site.owner() {
                return Err(freeze("artifact-birth-abi-drift"));
            }
            if row.binding.owner() != site.owner() {
                return Err(freeze("artifact-birth-owner-drift"));
            }
            if relation.object() != row.object {
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
            } = &row.emission
            {
                actuals.push(FinalizedBirthActualsV1 {
                    site: site.clone(),
                    destination: row.binding,
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
                callables,
                root_key,
                root_source,
                birth_actuals,
            }
        } else {
            FinalizedRootHandoffV1::Births {
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
