//! Normalized `hako.text.scan@1` provider contract.
//!
//! This is the semantic contract input to the generic provider-admission
//! spine.  It is intentionally cold: no LLVM hook, runtime lookup, generic
//! String surface, or Rust-VM consumer is allowed to depend on this module
//! until the complete AOT activation cell is ready.

use std::collections::BTreeSet;

use super::admitted::{
    AdmittedBoxCallableRegistryV1, BoxCallableAdmissionContextV1,
    BoxCallableAdmissionRejectV1, BoxCallableProviderAdmissionSealV1,
    BoxCallableProviderExportV1,
    BoxCallableRegistryDraftV1,
};
use super::{BoxCallableKey, BoxCallableRole, BoxCallableSource, BoxCallableTarget, BoxKey};

pub(crate) const HAKO_TEXT_SCAN_CONTRACT_ID_V1: &str = "hako.text.scan@1";
pub(crate) const HAKO_TEXT_SCAN_PROFILE_V1: &str = "utf8-codepoint-clamped-v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum TextScanRoleV1 {
    TextSliceRange,
    TextFindNeedle,
}

impl TextScanRoleV1 {
    pub(crate) const ALL: [Self; 2] = [Self::TextSliceRange, Self::TextFindNeedle];

    pub(crate) const fn method_name(self) -> &'static str {
        match self {
            Self::TextSliceRange => "substring",
            Self::TextFindNeedle => "indexOf",
        }
    }

    pub(crate) const fn arity(self) -> u8 {
        match self {
            Self::TextSliceRange => 2,
            Self::TextFindNeedle => 1,
        }
    }

    pub(crate) const fn result_lane(self) -> &'static str {
        match self {
            Self::TextSliceRange => "host_handle_end_authorized",
            Self::TextFindNeedle => "immediate_i64_no_lease",
        }
    }

    fn key_for(self, receiver_alias: &BoxKey) -> BoxCallableKey {
        BoxCallableKey::new(
            receiver_alias.as_str(),
            BoxCallableRole::Method,
            self.method_name(),
            self.arity(),
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TextScanProviderContractV1 {
    pub(crate) contract_id: &'static str,
    pub(crate) profile: &'static str,
    pub(crate) roles: &'static [TextScanRoleV1; 2],
}

pub(crate) const HAKO_TEXT_SCAN_CONTRACT_V1: TextScanProviderContractV1 =
    TextScanProviderContractV1 {
        contract_id: HAKO_TEXT_SCAN_CONTRACT_ID_V1,
        profile: HAKO_TEXT_SCAN_PROFILE_V1,
        roles: &TextScanRoleV1::ALL,
    };

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct SelectedTextScanRequirementV1 {
    pub(crate) contract_id: &'static str,
    pub(crate) roles: &'static [TextScanRoleV1; 2],
}

pub(crate) const SELECTED_TEXT_SCAN_REQUIREMENT_V1: SelectedTextScanRequirementV1 =
    SelectedTextScanRequirementV1 {
        contract_id: HAKO_TEXT_SCAN_CONTRACT_ID_V1,
        roles: &TextScanRoleV1::ALL,
    };

/// Cold provider export fact.  The callable key is derived from the role and
/// alias here; callers cannot substitute a different selector/arity pair.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TextScanProviderExportV1 {
    receiver_alias: BoxKey,
    role: TextScanRoleV1,
    source: BoxCallableSource,
    target: BoxCallableTarget,
    provider_id: String,
    image_pin: String,
    plan_stamp: u64,
}

impl TextScanProviderExportV1 {
    pub(crate) fn new(
        receiver_alias: BoxKey,
        role: TextScanRoleV1,
        source: BoxCallableSource,
        target: BoxCallableTarget,
        provider_id: impl Into<String>,
        image_pin: impl Into<String>,
        plan_stamp: u64,
    ) -> Self {
        Self {
            receiver_alias,
            role,
            source,
            target,
            provider_id: provider_id.into(),
            image_pin: image_pin.into(),
            plan_stamp,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum TextScanContractRejectV1 {
    ContractMismatch,
    IncompleteRoleSet,
    AliasMismatch,
    AliasRouteMismatch,
    DuplicateRoleAlias,
    EmptyProviderId,
    EmptyImagePin,
    Admission(BoxCallableAdmissionRejectV1),
}

/// Complete two-role contract after alias canonicalization.  The registry is
/// immutable and non-Clone; runtime/provider selection is deliberately absent.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AdmittedTextScanProviderV1 {
    contract: TextScanProviderContractV1,
    registry: AdmittedBoxCallableRegistryV1,
    aliases: Box<[BoxKey]>,
    canonical_receiver: BoxKey,
}

impl AdmittedTextScanProviderV1 {
    pub(crate) fn contract(&self) -> TextScanProviderContractV1 {
        self.contract
    }

    pub(crate) fn registry(&self) -> &AdmittedBoxCallableRegistryV1 {
        &self.registry
    }

    pub(crate) fn aliases(&self) -> &[BoxKey] {
        &self.aliases
    }

    pub(crate) fn canonical_receiver(&self) -> &BoxKey {
        &self.canonical_receiver
    }

    pub(crate) fn plan_for(
        &self,
        role: TextScanRoleV1,
        receiver_alias: &BoxKey,
    ) -> Option<&super::admitted::AdmittedBoxCallableEntryV1> {
        if !self.aliases.iter().any(|alias| alias == receiver_alias) {
            return None;
        }
        self.registry.get(&role.key_for(receiver_alias))
    }
}

pub(crate) struct TextScanProviderAdmissionV1;

impl TextScanProviderAdmissionV1 {
    pub(crate) fn admit(
        contract: TextScanProviderContractV1,
        requirement: SelectedTextScanRequirementV1,
        exports: impl IntoIterator<Item = TextScanProviderExportV1>,
        generation: u64,
    ) -> Result<AdmittedTextScanProviderV1, TextScanContractRejectV1> {
        if contract.contract_id != requirement.contract_id
            || contract.profile != HAKO_TEXT_SCAN_PROFILE_V1
            || contract.roles != requirement.roles
            || contract.roles != &TextScanRoleV1::ALL
        {
            return Err(TextScanContractRejectV1::ContractMismatch);
        }

        let exports = exports.into_iter().collect::<Vec<_>>();
        if exports.len() != 4 {
            return Err(TextScanContractRejectV1::IncompleteRoleSet);
        }

        let expected_aliases =
            BTreeSet::from([BoxKey::new("String"), BoxKey::new("StringBox")]);
        let mut role_aliases = std::collections::BTreeMap::<TextScanRoleV1, BTreeSet<BoxKey>>::new();
        let mut role_targets =
            std::collections::BTreeMap::<TextScanRoleV1, BoxCallableTarget>::new();
        let mut provider_id: Option<String> = None;
        let mut image_pin: Option<String> = None;
        let mut plan_stamp: Option<u64> = None;
        let mut draft = BoxCallableRegistryDraftV1::new();

        for export in exports {
            if !expected_aliases.contains(&export.receiver_alias) {
                return Err(TextScanContractRejectV1::AliasMismatch);
            }
            if export.provider_id.is_empty() {
                return Err(TextScanContractRejectV1::EmptyProviderId);
            }
            if export.image_pin.is_empty() {
                return Err(TextScanContractRejectV1::EmptyImagePin);
            }
            if !role_aliases
                .entry(export.role)
                .or_default()
                .insert(export.receiver_alias.clone())
            {
                return Err(TextScanContractRejectV1::DuplicateRoleAlias);
            }
            match role_targets.get(&export.role) {
                Some(expected) if expected != &export.target => {
                    return Err(TextScanContractRejectV1::AliasRouteMismatch);
                }
                Some(_) => {}
                None => {
                    role_targets.insert(export.role, export.target.clone());
                }
            }

            match provider_id {
                None => provider_id = Some(export.provider_id.clone()),
                Some(ref expected) if expected != &export.provider_id => {
                    return Err(TextScanContractRejectV1::Admission(
                        BoxCallableAdmissionRejectV1::ProviderMismatch,
                    ))
                }
                Some(_) => {}
            }
            match image_pin {
                None => image_pin = Some(export.image_pin.clone()),
                Some(ref expected) if expected != &export.image_pin => {
                    return Err(TextScanContractRejectV1::Admission(
                        BoxCallableAdmissionRejectV1::ImageMismatch,
                    ))
                }
                Some(_) => {}
            }
            match plan_stamp {
                None => plan_stamp = Some(export.plan_stamp),
                Some(expected) if expected != export.plan_stamp => {
                    return Err(TextScanContractRejectV1::Admission(
                        BoxCallableAdmissionRejectV1::PlanStampMismatch,
                    ))
                }
                Some(_) => {}
            }

            let key = export.role.key_for(&export.receiver_alias);
            draft.push(BoxCallableProviderExportV1::new(
                key,
                export.source,
                export.target,
                export.provider_id,
                export.image_pin,
                export.plan_stamp,
            ));
        }

        if role_aliases.len() != TextScanRoleV1::ALL.len()
            || role_aliases.values().any(|aliases| aliases != &expected_aliases)
        {
            return Err(TextScanContractRejectV1::IncompleteRoleSet);
        }

        let provider_id = provider_id.ok_or(TextScanContractRejectV1::EmptyProviderId)?;
        let image_pin = image_pin.ok_or(TextScanContractRejectV1::EmptyImagePin)?;
        let plan_stamp = plan_stamp.ok_or(TextScanContractRejectV1::IncompleteRoleSet)?;
        let registry = BoxCallableProviderAdmissionSealV1::admit(
            BoxCallableAdmissionContextV1::new(
                provider_id.as_str(),
                image_pin.as_str(),
                generation,
                plan_stamp,
            ),
            draft,
        )
        .map_err(TextScanContractRejectV1::Admission)?;

        Ok(AdmittedTextScanProviderV1 {
            contract,
            registry,
            aliases: expected_aliases.into_iter().collect::<Vec<_>>().into_boxed_slice(),
            canonical_receiver: BoxKey::new("Text"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDER: &str = "hako.text.scan.native@1";
    const IMAGE: &str = "hako.text.scan.native.image@1";

    fn exports() -> Vec<TextScanProviderExportV1> {
        TextScanRoleV1::ALL
            .into_iter()
            .flat_map(|role| {
                ["String", "StringBox"].into_iter().map(move |alias| {
                    TextScanProviderExportV1::new(
                        BoxKey::new(alias),
                        role,
                        BoxCallableSource::Manual,
                        BoxCallableTarget::InternalSlot {
                            slot: if role == TextScanRoleV1::TextSliceRange {
                                310
                            } else {
                                311
                            },
                        },
                        PROVIDER,
                        IMAGE,
                        17,
                    )
                })
            })
            .collect()
    }

    #[test]
    fn complete_two_role_contract_admits_one_provider_profile() {
        let admitted = TextScanProviderAdmissionV1::admit(
            HAKO_TEXT_SCAN_CONTRACT_V1,
            SELECTED_TEXT_SCAN_REQUIREMENT_V1,
            exports(),
            23,
        )
        .expect("complete text scan contract");
        assert_eq!(admitted.contract().profile, HAKO_TEXT_SCAN_PROFILE_V1);
        assert_eq!(admitted.registry().generation(), 23);
        assert_eq!(admitted.aliases().len(), 2);
        assert_eq!(admitted.canonical_receiver().as_str(), "Text");
        assert!(admitted
            .plan_for(TextScanRoleV1::TextFindNeedle, &BoxKey::new("String"))
            .is_some());
        assert_eq!(
            TextScanRoleV1::TextFindNeedle.result_lane(),
            "immediate_i64_no_lease"
        );
    }

    #[test]
    fn contract_rejects_partial_role_or_alias_set() {
        let mut partial = exports();
        partial.pop();
        assert_eq!(
            TextScanProviderAdmissionV1::admit(
                HAKO_TEXT_SCAN_CONTRACT_V1,
                SELECTED_TEXT_SCAN_REQUIREMENT_V1,
                partial,
                23,
            ),
            Err(TextScanContractRejectV1::IncompleteRoleSet)
        );

        let mut wrong_alias = exports();
        wrong_alias[0].receiver_alias = BoxKey::new("Bytes");
        assert_eq!(
            TextScanProviderAdmissionV1::admit(
                HAKO_TEXT_SCAN_CONTRACT_V1,
                SELECTED_TEXT_SCAN_REQUIREMENT_V1,
                wrong_alias,
                23,
            ),
            Err(TextScanContractRejectV1::AliasMismatch)
        );
    }

    #[test]
    fn contract_rejects_foreign_requirement() {
        let foreign = SelectedTextScanRequirementV1 {
            contract_id: "hako.text.other@1",
            roles: &TextScanRoleV1::ALL,
        };
        assert_eq!(
            TextScanProviderAdmissionV1::admit(
                HAKO_TEXT_SCAN_CONTRACT_V1,
                foreign,
                exports(),
                23,
            ),
            Err(TextScanContractRejectV1::ContractMismatch)
        );
    }

    #[test]
    fn contract_rejects_alias_route_drift() {
        let mut drift = exports();
        drift[1].target = BoxCallableTarget::InternalSlot { slot: 399 };
        assert_eq!(
            TextScanProviderAdmissionV1::admit(
                HAKO_TEXT_SCAN_CONTRACT_V1,
                SELECTED_TEXT_SCAN_REQUIREMENT_V1,
                drift,
                23,
            ),
            Err(TextScanContractRejectV1::AliasRouteMismatch)
        );
    }
}
