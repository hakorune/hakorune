//! Move-only admission boundary for one provider-backed callable cohort.
//!
//! `BoxCallableRegistry` remains the legacy mutable compatibility projection.
//! This module is the cold provider spine used by a future selected AOT
//! consumer: provider export facts are collected in a draft, consumed once by
//! `BoxCallableProviderAdmissionSealV1`, and published as an immutable,
//! deterministic admitted snapshot.  No runtime lookup or selector recovery
//! is provided here.

use std::collections::BTreeMap;

use super::{
    BoxCallableEntry, BoxCallableKey, BoxCallableRole, BoxCallableSource, BoxCallableTarget,
};
use super::route_plan::MethodCallRoutePlan;

/// Provider/image facts that must agree for one admitted cohort.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BoxCallableAdmissionContextV1 {
    provider_id: String,
    image_pin: String,
    generation: u64,
    plan_stamp: u64,
}

impl BoxCallableAdmissionContextV1 {
    pub(crate) fn new(
        provider_id: impl Into<String>,
        image_pin: impl Into<String>,
        generation: u64,
        plan_stamp: u64,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            image_pin: image_pin.into(),
            generation,
            plan_stamp,
        }
    }

    pub(crate) fn provider_id(&self) -> &str {
        &self.provider_id
    }

    pub(crate) fn image_pin(&self) -> &str {
        &self.image_pin
    }

    pub(crate) fn generation(&self) -> u64 {
        self.generation
    }

    pub(crate) fn plan_stamp(&self) -> u64 {
        self.plan_stamp
    }
}

/// A provider export is an input fact.  It is not a route plan and cannot be
/// invoked until the consuming admission seal accepts the whole draft.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct BoxCallableProviderExportV1 {
    key: BoxCallableKey,
    entry: BoxCallableEntry,
    provider_id: String,
    image_pin: String,
    plan_stamp: u64,
}

impl BoxCallableProviderExportV1 {
    pub(crate) fn new(
        key: BoxCallableKey,
        source: BoxCallableSource,
        target: BoxCallableTarget,
        provider_id: impl Into<String>,
        image_pin: impl Into<String>,
        plan_stamp: u64,
    ) -> Self {
        Self {
            key,
            entry: BoxCallableEntry::new(source, target),
            provider_id: provider_id.into(),
            image_pin: image_pin.into(),
            plan_stamp,
        }
    }
}

/// Mutable provider-only collection.  It has no executable lookup API.
#[derive(Debug, Default, PartialEq, Eq)]
pub(crate) struct BoxCallableRegistryDraftV1 {
    exports: Vec<BoxCallableProviderExportV1>,
}

impl BoxCallableRegistryDraftV1 {
    pub(crate) fn new() -> Self {
        Self::default()
    }

    pub(crate) fn push(&mut self, export: BoxCallableProviderExportV1) {
        self.exports.push(export);
    }

    pub(crate) fn len(&self) -> usize {
        self.exports.len()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum BoxCallableAdmissionRejectV1 {
    EmptyDraft,
    EmptyProviderId,
    EmptyImagePin,
    DuplicateKey,
    ProviderMismatch,
    ImageMismatch,
    PlanStampMismatch,
    MissingMethodRoute,
}

/// An admitted entry contains a route projection, never a function pointer.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AdmittedBoxCallableEntryV1 {
    entry: BoxCallableEntry,
    method_route: Option<MethodCallRoutePlan>,
}

impl AdmittedBoxCallableEntryV1 {
    pub(crate) fn entry(&self) -> &BoxCallableEntry {
        &self.entry
    }

    pub(crate) fn method_route(&self) -> Option<&MethodCallRoutePlan> {
        self.method_route.as_ref()
    }
}

/// Immutable, deterministic, non-Clone provider admission result.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct AdmittedBoxCallableRegistryV1 {
    context: BoxCallableAdmissionContextV1,
    entries: BTreeMap<BoxCallableKey, AdmittedBoxCallableEntryV1>,
}

impl AdmittedBoxCallableRegistryV1 {
    pub(crate) fn provider_id(&self) -> &str {
        self.context.provider_id()
    }

    pub(crate) fn image_pin(&self) -> &str {
        self.context.image_pin()
    }

    pub(crate) fn generation(&self) -> u64 {
        self.context.generation()
    }

    pub(crate) fn plan_stamp(&self) -> u64 {
        self.context.plan_stamp()
    }

    pub(crate) fn len(&self) -> usize {
        self.entries.len()
    }

    pub(crate) fn get(&self, key: &BoxCallableKey) -> Option<&AdmittedBoxCallableEntryV1> {
        self.entries.get(key)
    }

    pub(crate) fn iter(
        &self,
    ) -> impl Iterator<Item = (&BoxCallableKey, &AdmittedBoxCallableEntryV1)> {
        self.entries.iter()
    }
}

/// The only constructor for an admitted provider cohort.
pub(crate) struct BoxCallableProviderAdmissionSealV1;

impl BoxCallableProviderAdmissionSealV1 {
    pub(crate) fn admit(
        context: BoxCallableAdmissionContextV1,
        draft: BoxCallableRegistryDraftV1,
    ) -> Result<AdmittedBoxCallableRegistryV1, BoxCallableAdmissionRejectV1> {
        if draft.exports.is_empty() {
            return Err(BoxCallableAdmissionRejectV1::EmptyDraft);
        }
        if context.provider_id.is_empty() {
            return Err(BoxCallableAdmissionRejectV1::EmptyProviderId);
        }
        if context.image_pin.is_empty() {
            return Err(BoxCallableAdmissionRejectV1::EmptyImagePin);
        }

        let mut entries = BTreeMap::new();
        for export in draft.exports {
            if export.provider_id != context.provider_id {
                return Err(BoxCallableAdmissionRejectV1::ProviderMismatch);
            }
            if export.image_pin != context.image_pin {
                return Err(BoxCallableAdmissionRejectV1::ImageMismatch);
            }
            if export.plan_stamp != context.plan_stamp {
                return Err(BoxCallableAdmissionRejectV1::PlanStampMismatch);
            }
            let method_route = match export.key.role {
                BoxCallableRole::Method | BoxCallableRole::StaticMethod => {
                    Some(MethodCallRoutePlan::from_entry(&export.entry, None).ok_or(
                        BoxCallableAdmissionRejectV1::MissingMethodRoute,
                    )?)
                }
                _ => None,
            };
            if entries
                .insert(
                    export.key,
                    AdmittedBoxCallableEntryV1 {
                        entry: export.entry,
                        method_route,
                    },
                )
                .is_some()
            {
                return Err(BoxCallableAdmissionRejectV1::DuplicateKey);
            }
        }

        Ok(AdmittedBoxCallableRegistryV1 { context, entries })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const PROVIDER: &str = "provider.text.scan@1";
    const IMAGE: &str = "image.text.scan@1";

    fn context() -> BoxCallableAdmissionContextV1 {
        BoxCallableAdmissionContextV1::new(PROVIDER, IMAGE, 9, 17)
    }

    fn export(name: &str, arity: u8, slot: u16) -> BoxCallableProviderExportV1 {
        BoxCallableProviderExportV1::new(
            BoxCallableKey::new("Text", BoxCallableRole::Method, name, arity),
            BoxCallableSource::Manual,
            BoxCallableTarget::InternalSlot { slot },
            PROVIDER,
            IMAGE,
            17,
        )
    }

    #[test]
    fn admission_consumes_draft_into_deterministic_immutable_snapshot() {
        let mut draft = BoxCallableRegistryDraftV1::new();
        draft.push(export("indexOf", 1, 311));
        draft.push(export("substring", 2, 310));
        assert_eq!(draft.len(), 2);

        let admitted = BoxCallableProviderAdmissionSealV1::admit(context(), draft)
            .expect("complete provider draft");
        assert_eq!(admitted.provider_id(), PROVIDER);
        assert_eq!(admitted.image_pin(), IMAGE);
        assert_eq!(admitted.generation(), 9);
        assert_eq!(admitted.plan_stamp(), 17);
        assert_eq!(admitted.len(), 2);

        let keys = admitted
            .iter()
            .map(|(key, _)| key.name.as_str())
            .collect::<Vec<_>>();
        assert_eq!(keys, vec!["indexOf", "substring"]);
    }

    #[test]
    fn admission_rejects_duplicate_and_foreign_provider_facts() {
        let duplicate = {
            let mut draft = BoxCallableRegistryDraftV1::new();
            draft.push(export("indexOf", 1, 311));
            draft.push(export("indexOf", 1, 311));
            BoxCallableProviderAdmissionSealV1::admit(context(), draft)
        };
        assert_eq!(duplicate, Err(BoxCallableAdmissionRejectV1::DuplicateKey));

        let foreign = {
            let mut draft = BoxCallableRegistryDraftV1::new();
            let mut item = export("indexOf", 1, 311);
            item.provider_id = "foreign@1".to_owned();
            draft.push(item);
            BoxCallableProviderAdmissionSealV1::admit(context(), draft)
        };
        assert_eq!(foreign, Err(BoxCallableAdmissionRejectV1::ProviderMismatch));
    }

    #[test]
    fn admission_rejects_non_callable_method_target() {
        let mut draft = BoxCallableRegistryDraftV1::new();
        draft.push(BoxCallableProviderExportV1::new(
            BoxCallableKey::new("Text", BoxCallableRole::Method, "indexOf", 1),
            BoxCallableSource::Manual,
            BoxCallableTarget::PluginLifecycle {
                type_id: 1,
                birth_id: None,
                fini_id: None,
            },
            PROVIDER,
            IMAGE,
            17,
        ));
        assert_eq!(
            BoxCallableProviderAdmissionSealV1::admit(context(), draft),
            Err(BoxCallableAdmissionRejectV1::MissingMethodRoute)
        );
    }
}
