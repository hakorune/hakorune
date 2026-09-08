//! Final root handoff after source and physical validation are complete.
use super::*;
use std::collections::BTreeSet;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn seal_finalized_root_birth_handoff(
        &self,
        root_key: String,
        construction_keys: &BTreeSet<CanonicalSameModuleCallableKeyV1>,
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
        let mut root_source = self
            .terminal_relation
            .as_ref()
            .map(|terminal| {
                Ok::<_, String>(FinalizedRootSourceHandoffV1 {
                    birth_actuals: Box::new([]),
                    app_main_identity: self
                        .app_main_identity
                        .as_ref()
                        .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?
                        .clone(),
                    terminal: terminal.clone(),
                })
            })
            .transpose()?;
        let mut keys = BTreeSet::new();
        let mut births = Vec::new();
        let mut actuals = Vec::new();
        for (site, row) in self
            .local_commits
            .borrow()
            .iter()
            .filter(|(_, row)| row.binding.owner() == owner)
        {
            if !row.is_complete() {
                return Err(freeze("artifact-local-commit-incomplete"));
            }
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
            if relation.target() != &key || relation.owner() == owner {
                return Err(freeze("artifact-birth-abi-drift"));
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
        if let Some(source) = root_source.as_mut() {
            source.birth_actuals = actuals.into_boxed_slice();
        } else if !actuals.is_empty() {
            return Err(freeze("artifact-actual-root-source-missing"));
        }
        Ok(if births.is_empty() {
            FinalizedRootHandoffV1::NoBirth {
                root_key,
                root_source,
            }
        } else {
            FinalizedRootHandoffV1::Births {
                root_key,
                root_source,
                keys: keys.into_iter().collect(),
                births: births.into_boxed_slice(),
            }
        })
    }
}
