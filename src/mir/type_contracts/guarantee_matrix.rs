#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum AnnotationSite {
    LocalSlot,
    ParameterEntry,
    ReturnExit,
    BoxFieldWrite,
    RecordConstruction,
    StaticTableElement,
    CollectionElement,
    TypedArrayElement,
    WeakField,
    FfiBoundary,
    BackendPreservation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GuaranteeClass {
    AnyDefault,
    MetadataOnlyNonGuarantee,
    RuntimeCheckedContract,
    VerifierProvenContract,
    VerifiedRuntimeGuardedContract,
    UnsupportedFailFast,
    RepresentationFactNonContract,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum EnforcementOwner {
    LocalSlotContract,
    FunctionEntryContract,
    FunctionReturnContract,
    ExactNumericBoxFieldContract,
    RecordConstructionContract,
    StaticTableElementContract,
    CollectionElementContract,
    TypedArrayElementContract,
    WeakFieldContract,
    FfiBoundaryContract,
    BackendCapabilityPreflight,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ActivationScope {
    Transitional,
    ExistingNarrow,
    ExactNumericFirstSlice,
    AnyDefault,
    RepresentationBoundary,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum UnsupportedBackendPolicy {
    NotApplicable,
    RejectBeforeEffects,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct GuaranteeMatrixRow {
    pub site: AnnotationSite,
    pub current: GuaranteeClass,
    pub target: GuaranteeClass,
    pub owner: EnforcementOwner,
    pub activation: ActivationScope,
    pub unsupported_backend: UnsupportedBackendPolicy,
}

pub(crate) const GUARANTEE_MATRIX: [GuaranteeMatrixRow; 11] = [
    GuaranteeMatrixRow {
        site: AnnotationSite::LocalSlot,
        current: GuaranteeClass::MetadataOnlyNonGuarantee,
        target: GuaranteeClass::VerifiedRuntimeGuardedContract,
        owner: EnforcementOwner::LocalSlotContract,
        activation: ActivationScope::Transitional,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::ParameterEntry,
        current: GuaranteeClass::RuntimeCheckedContract,
        target: GuaranteeClass::VerifiedRuntimeGuardedContract,
        owner: EnforcementOwner::FunctionEntryContract,
        activation: ActivationScope::ExactNumericFirstSlice,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::ReturnExit,
        current: GuaranteeClass::MetadataOnlyNonGuarantee,
        target: GuaranteeClass::VerifiedRuntimeGuardedContract,
        owner: EnforcementOwner::FunctionReturnContract,
        activation: ActivationScope::Transitional,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::BoxFieldWrite,
        current: GuaranteeClass::VerifiedRuntimeGuardedContract,
        target: GuaranteeClass::VerifiedRuntimeGuardedContract,
        owner: EnforcementOwner::ExactNumericBoxFieldContract,
        activation: ActivationScope::ExactNumericFirstSlice,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::RecordConstruction,
        current: GuaranteeClass::VerifierProvenContract,
        target: GuaranteeClass::VerifierProvenContract,
        owner: EnforcementOwner::RecordConstructionContract,
        activation: ActivationScope::ExistingNarrow,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::StaticTableElement,
        current: GuaranteeClass::VerifierProvenContract,
        target: GuaranteeClass::VerifierProvenContract,
        owner: EnforcementOwner::StaticTableElementContract,
        activation: ActivationScope::ExistingNarrow,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::CollectionElement,
        current: GuaranteeClass::AnyDefault,
        target: GuaranteeClass::AnyDefault,
        owner: EnforcementOwner::CollectionElementContract,
        activation: ActivationScope::AnyDefault,
        unsupported_backend: UnsupportedBackendPolicy::NotApplicable,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::TypedArrayElement,
        current: GuaranteeClass::RuntimeCheckedContract,
        target: GuaranteeClass::VerifiedRuntimeGuardedContract,
        owner: EnforcementOwner::TypedArrayElementContract,
        activation: ActivationScope::ExistingNarrow,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::WeakField,
        current: GuaranteeClass::VerifierProvenContract,
        target: GuaranteeClass::VerifierProvenContract,
        owner: EnforcementOwner::WeakFieldContract,
        activation: ActivationScope::ExistingNarrow,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::FfiBoundary,
        current: GuaranteeClass::MetadataOnlyNonGuarantee,
        target: GuaranteeClass::RuntimeCheckedContract,
        owner: EnforcementOwner::FfiBoundaryContract,
        activation: ActivationScope::Transitional,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
    GuaranteeMatrixRow {
        site: AnnotationSite::BackendPreservation,
        current: GuaranteeClass::RepresentationFactNonContract,
        target: GuaranteeClass::UnsupportedFailFast,
        owner: EnforcementOwner::BackendCapabilityPreflight,
        activation: ActivationScope::RepresentationBoundary,
        unsupported_backend: UnsupportedBackendPolicy::RejectBeforeEffects,
    },
];

pub(crate) fn guarantee_for(site: AnnotationSite) -> &'static GuaranteeMatrixRow {
    GUARANTEE_MATRIX
        .iter()
        .find(|row| row.site == site)
        .expect("closed annotation-site matrix must contain every site")
}

pub(crate) fn exact_numeric_box_field_contract_is_active() -> bool {
    let row = guarantee_for(AnnotationSite::BoxFieldWrite);
    row.current == GuaranteeClass::VerifiedRuntimeGuardedContract
        && row.owner == EnforcementOwner::ExactNumericBoxFieldContract
        && row.activation == ActivationScope::ExactNumericFirstSlice
        && row.unsupported_backend == UnsupportedBackendPolicy::RejectBeforeEffects
}

pub(crate) fn exact_numeric_parameter_entry_contract_is_active() -> bool {
    let row = guarantee_for(AnnotationSite::ParameterEntry);
    row.current == GuaranteeClass::RuntimeCheckedContract
        && row.owner == EnforcementOwner::FunctionEntryContract
        && row.activation == ActivationScope::ExactNumericFirstSlice
        && row.unsupported_backend == UnsupportedBackendPolicy::RejectBeforeEffects
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn matrix_closes_eleven_unique_annotation_sites() {
        let sites: BTreeSet<_> = GUARANTEE_MATRIX.iter().map(|row| row.site).collect();
        assert_eq!(GUARANTEE_MATRIX.len(), 11);
        assert_eq!(sites.len(), 11);
    }

    #[test]
    fn no_target_keeps_metadata_only_as_a_permanent_guarantee() {
        assert!(GUARANTEE_MATRIX
            .iter()
            .all(|row| row.target != GuaranteeClass::MetadataOnlyNonGuarantee));
    }

    #[test]
    fn exact_numeric_box_field_and_parameter_entry_are_active_slices() {
        let rows: Vec<_> = GUARANTEE_MATRIX
            .iter()
            .filter(|row| row.activation == ActivationScope::ExactNumericFirstSlice)
            .collect();
        assert_eq!(rows.len(), 2);
        assert!(rows.iter().any(|row| {
            row.site == AnnotationSite::BoxFieldWrite
                && row.owner == EnforcementOwner::ExactNumericBoxFieldContract
        }));
        assert!(rows.iter().any(|row| {
            row.site == AnnotationSite::ParameterEntry
                && row.owner == EnforcementOwner::FunctionEntryContract
        }));
        assert!(rows.iter().all(|row| {
            row.unsupported_backend == UnsupportedBackendPolicy::RejectBeforeEffects
        }));
        assert!(exact_numeric_parameter_entry_contract_is_active());
    }

    #[test]
    fn lookup_returns_the_single_site_owner() {
        assert_eq!(
            guarantee_for(AnnotationSite::ParameterEntry).owner,
            EnforcementOwner::FunctionEntryContract
        );
    }
}
