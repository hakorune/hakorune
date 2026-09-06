//! Final root handoff after source and physical validation are complete.
use super::*;
use std::collections::BTreeSet;

impl OrdinaryNewClaimLedgerV1 {
    pub(crate) fn seal_finalized_root_birth_handoff(
        &self,
        root_key: String,
        construction_keys: &BTreeSet<CanonicalSameModuleCallableKeyV1>,
    ) -> Result<FinalizedRootBirthHandoffV1, String> {
        match *self.root_validation.borrow() {
            RootNewValidation::FinishingChecked => {}
            _ => return Err(freeze("artifact-root-not-finished")),
        }
        let owner = match self.root_completion.as_ref() {
            Some(Ok(completion)) => completion.owner(),
            _ => return Err(freeze("artifact-root-completion-unavailable")),
        };
        let root_source = match (
            &self.terminal_result,
            &self.terminal_unit_return,
            &self.terminal_integer_literal,
            &self.terminal_i64_field_return,
        ) {
            (Some(_), Some(_), _, _)
            | (Some(_), _, Some(_), _)
            | (Some(_), _, _, Some(_))
            | (_, Some(_), Some(_), _)
            | (_, Some(_), _, Some(_))
            | (_, _, Some(_), Some(_)) => return Err(freeze("artifact-root-result-conflict")),
            (Some(relation), None, None, None) => Some({
                if relation.owner() != owner || !self.terminal_result_complete() {
                    return Err(freeze("artifact-root-result-unavailable"));
                }
                let app_main_identity = self
                    .app_main_identity
                    .as_ref()
                    .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?
                    .clone();
                FinalizedRootSourceHandoffV1 {
                    app_main_identity,
                    terminal_i64_add: Some(relation.clone()),
                    terminal_unit_return: None,
                    terminal_integer_literal: None,
                    terminal_i64_field_return: None,
                }
            }),
            (None, Some(relation), None, None) => Some({
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
                let app_main_identity = self
                    .app_main_identity
                    .as_ref()
                    .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?
                    .clone();
                FinalizedRootSourceHandoffV1 {
                    app_main_identity,
                    terminal_i64_add: None,
                    terminal_unit_return: Some(relation.clone()),
                    terminal_integer_literal: None,
                    terminal_i64_field_return: None,
                }
            }),
            (None, None, Some(relation), None) => Some({
                if relation.owner() != owner
                    || self.terminal_integer_literal_value.borrow().is_none()
                {
                    return Err(freeze("artifact-root-literal-unavailable"));
                }
                FinalizedRootSourceHandoffV1 {
                    app_main_identity: self
                        .app_main_identity
                        .as_ref()
                        .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?
                        .clone(),
                    terminal_i64_add: None,
                    terminal_unit_return: None,
                    terminal_integer_literal: Some(relation.clone()),
                    terminal_i64_field_return: None,
                }
            }),
            (None, None, None, Some(relation)) => Some({
                if relation.owner() != owner || !self.terminal_i64_field_return_complete() {
                    return Err(freeze("artifact-root-field-unavailable"));
                }
                FinalizedRootSourceHandoffV1 {
                    app_main_identity: self
                        .app_main_identity
                        .as_ref()
                        .ok_or_else(|| freeze("artifact-root-identity-unavailable"))?
                        .clone(),
                    terminal_i64_add: None,
                    terminal_unit_return: None,
                    terminal_integer_literal: None,
                    terminal_i64_field_return: Some(relation.clone()),
                }
            }),
            (None, None, None, None) => None,
        };
        let root_result = root_source.as_ref().map(|source| {
            if source.terminal_i64_add().is_some() {
                FinalizedRootResultAbiV1::I64AddReturn { owner }
            } else if source.terminal_unit_return().is_some() {
                FinalizedRootResultAbiV1::UnitReturn { owner }
            } else if source.terminal_integer_literal().is_some() {
                FinalizedRootResultAbiV1::IntegerLiteralReturn { owner }
            } else {
                FinalizedRootResultAbiV1::I64FieldReturn { owner }
            }
        });
        let mut keys = BTreeSet::new();
        let mut births = Vec::new();
        for (_, row) in self
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
            // Multiple exact New sites may invoke one canonical Birth
            // definition. Local emission validation above remains per site;
            // the final handoff retains each definition relation once.
            if keys.insert(key.clone()) {
                births.push(relation.clone());
            } else if !births.iter().any(|existing| existing == relation) {
                return Err(freeze("artifact-birth-abi-duplicate-drift"));
            }
        }
        Ok(if births.is_empty() {
            FinalizedRootBirthHandoffV1::NoBirth {
                root_key,
                root_source,
                root_result,
            }
        } else {
            FinalizedRootBirthHandoffV1::Births {
                root_key,
                root_source,
                root_result,
                keys: keys.into_iter().collect(),
                births: births.into_boxed_slice(),
            }
        })
    }
}
