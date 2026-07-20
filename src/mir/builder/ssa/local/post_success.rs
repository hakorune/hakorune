//! Pure LocalSSA post-success metadata decisions.
//!
//! This module deliberately owns no `MirBuilder`, `ValueId`, or fact map.  It
//! prepares the behavior-preserving transfer decision that a later lifecycle
//! row will commit only after materialization succeeds.

use crate::mir::MirType;

use super::LocalKind;

/// The three observable source entries in the current transient type map.
///
/// `Unknown` is a stored compatibility sentinel, not an exact type fact.  All
/// other `MirType` variants, including `Void`, remain exact facts.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum LocalSsaSourceTypeEntryV1 {
    Missing,
    StoredUnknown,
    Exact(MirType),
}

impl LocalSsaSourceTypeEntryV1 {
    pub(super) fn classify(entry: Option<&MirType>) -> Self {
        match entry {
            None => Self::Missing,
            Some(MirType::Unknown) => Self::StoredUnknown,
            Some(ty) => Self::Exact(ty.clone()),
        }
    }
}

/// The successful instruction family that materialized a LocalSSA value.
///
/// This is an observation-only classification.  COPY0 will later consume only
/// `PhysicalCopy`; non-Copy rematerializations intentionally remain outside
/// that exact-fact publisher.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LocalSsaMaterializationKindV1 {
    RematerializedConst,
    RematerializedBinOp,
    RematerializedCompare,
    RematerializedSelect,
    PhysicalCopy(LocalSsaPhysicalCopyReasonV1),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LocalSsaPhysicalCopyReasonV1 {
    RematerializedCopy,
    DominatingFallbackCopy,
}

/// The receiver-only compatibility result for a copied origin.
///
/// It makes the existing stored-`Unknown` suppression explicit rather than
/// relying on incidental map-entry presence to block `Box(owner)` synthesis.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ReceiverOriginCompatibilityV1 {
    Inactive,
    PublishBoxFromMissingType { owner: String },
    SuppressedByStoredUnknown { owner: String },
    SuppressedByExactType { owner: String },
}

/// A map-free decision prepared before LocalSSA materialization.
///
/// The later lifecycle row may commit this only after its instruction has
/// succeeded.  `exact_type` and `legacy_unknown` are intentionally mutually
/// exclusive so the future exact FACT0 lane cannot absorb the sentinel.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct PreparedLocalSsaPostSuccessV1 {
    materialization: LocalSsaMaterializationKindV1,
    exact_type: Option<MirType>,
    legacy_unknown: bool,
    origin: Option<String>,
    receiver_compat: ReceiverOriginCompatibilityV1,
}

impl PreparedLocalSsaPostSuccessV1 {
    pub(super) fn prepare(
        source_type: &LocalSsaSourceTypeEntryV1,
        source_origin: Option<&str>,
        materialization: LocalSsaMaterializationKindV1,
        local_kind: LocalKind,
    ) -> Self {
        let (exact_type, legacy_unknown) = match source_type {
            LocalSsaSourceTypeEntryV1::Missing => (None, false),
            LocalSsaSourceTypeEntryV1::StoredUnknown => (None, true),
            LocalSsaSourceTypeEntryV1::Exact(ty) => (Some(ty.clone()), false),
        };
        let origin = source_origin.map(str::to_owned);
        let receiver_compat = match (source_origin, local_kind, source_type) {
            (Some(owner), LocalKind::Recv, LocalSsaSourceTypeEntryV1::Missing) => {
                ReceiverOriginCompatibilityV1::PublishBoxFromMissingType {
                    owner: owner.to_owned(),
                }
            }
            (Some(owner), LocalKind::Recv, LocalSsaSourceTypeEntryV1::StoredUnknown) => {
                ReceiverOriginCompatibilityV1::SuppressedByStoredUnknown {
                    owner: owner.to_owned(),
                }
            }
            (Some(owner), LocalKind::Recv, LocalSsaSourceTypeEntryV1::Exact(_)) => {
                ReceiverOriginCompatibilityV1::SuppressedByExactType {
                    owner: owner.to_owned(),
                }
            }
            _ => ReceiverOriginCompatibilityV1::Inactive,
        };

        Self {
            materialization,
            exact_type,
            legacy_unknown,
            origin,
            receiver_compat,
        }
    }

    #[cfg(test)]
    fn materialization(&self) -> LocalSsaMaterializationKindV1 {
        self.materialization
    }

    #[cfg(test)]
    fn exact_type(&self) -> Option<&MirType> {
        self.exact_type.as_ref()
    }

    #[cfg(test)]
    fn has_legacy_unknown(&self) -> bool {
        self.legacy_unknown
    }

    #[cfg(test)]
    fn origin(&self) -> Option<&str> {
        self.origin.as_deref()
    }

    #[cfg(test)]
    fn receiver_compat(&self) -> &ReceiverOriginCompatibilityV1 {
        &self.receiver_compat
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LocalSsaMaterializationKindV1, LocalSsaPhysicalCopyReasonV1, LocalSsaSourceTypeEntryV1,
        PreparedLocalSsaPostSuccessV1, ReceiverOriginCompatibilityV1,
    };
    use crate::mir::builder::ssa::local::LocalKind;
    use crate::mir::MirType;

    #[derive(Clone, Debug, Default, Eq, PartialEq)]
    struct SyntheticCommittedPostSuccessV1 {
        type_entry: Option<MirType>,
        origin: Option<String>,
        cache_inserted: bool,
    }

    fn prepare(
        source_type: Option<&MirType>,
        source_origin: Option<&str>,
        local_kind: LocalKind,
    ) -> PreparedLocalSsaPostSuccessV1 {
        PreparedLocalSsaPostSuccessV1::prepare(
            &LocalSsaSourceTypeEntryV1::classify(source_type),
            source_origin,
            LocalSsaMaterializationKindV1::PhysicalCopy(
                LocalSsaPhysicalCopyReasonV1::DominatingFallbackCopy,
            ),
            local_kind,
        )
    }

    fn commit_on_synthetic_cache_miss(
        prepared: &PreparedLocalSsaPostSuccessV1,
        cache_hit: bool,
        state: &mut SyntheticCommittedPostSuccessV1,
    ) -> bool {
        if cache_hit {
            return false;
        }

        if let Some(ty) = prepared.exact_type() {
            state.type_entry = Some(ty.clone());
        }
        if prepared.has_legacy_unknown() {
            state.type_entry = Some(MirType::Unknown);
        }
        state.origin = prepared.origin().map(str::to_owned);
        if let ReceiverOriginCompatibilityV1::PublishBoxFromMissingType { owner } =
            prepared.receiver_compat()
        {
            state.type_entry = Some(MirType::Box(owner.clone()));
        }
        state.cache_inserted = true;
        true
    }

    fn commit_after_synthetic_emission(
        prepared: &PreparedLocalSsaPostSuccessV1,
        emission: Result<(), ()>,
        state: &mut SyntheticCommittedPostSuccessV1,
    ) -> bool {
        if emission.is_err() {
            return false;
        }
        commit_on_synthetic_cache_miss(prepared, false, state)
    }

    #[test]
    fn classifies_missing_unknown_and_every_other_type_once() {
        assert_eq!(
            LocalSsaSourceTypeEntryV1::classify(None),
            LocalSsaSourceTypeEntryV1::Missing
        );
        assert_eq!(
            LocalSsaSourceTypeEntryV1::classify(Some(&MirType::Unknown)),
            LocalSsaSourceTypeEntryV1::StoredUnknown
        );
        assert_eq!(
            LocalSsaSourceTypeEntryV1::classify(Some(&MirType::Void)),
            LocalSsaSourceTypeEntryV1::Exact(MirType::Void)
        );
    }

    #[test]
    fn stored_unknown_receiver_origin_keeps_the_legacy_sentinel_and_suppresses_box() {
        let prepared = prepare(Some(&MirType::Unknown), Some("Owner"), LocalKind::Recv);

        assert_eq!(prepared.exact_type(), None);
        assert!(prepared.has_legacy_unknown());
        assert_eq!(prepared.origin(), Some("Owner"));
        assert_eq!(
            prepared.receiver_compat(),
            &ReceiverOriginCompatibilityV1::SuppressedByStoredUnknown {
                owner: "Owner".to_string(),
            }
        );
    }

    #[test]
    fn missing_receiver_origin_prepares_only_the_receiver_box_fallback() {
        let prepared = prepare(None, Some("Owner"), LocalKind::Recv);

        assert_eq!(prepared.exact_type(), None);
        assert!(!prepared.has_legacy_unknown());
        assert_eq!(prepared.origin(), Some("Owner"));
        assert_eq!(
            prepared.receiver_compat(),
            &ReceiverOriginCompatibilityV1::PublishBoxFromMissingType {
                owner: "Owner".to_string(),
            }
        );
    }

    #[test]
    fn exact_receiver_origin_suppresses_fallback_without_losing_the_exact_type() {
        let prepared = prepare(Some(&MirType::Integer), Some("Owner"), LocalKind::Recv);

        assert_eq!(prepared.exact_type(), Some(&MirType::Integer));
        assert!(!prepared.has_legacy_unknown());
        assert_eq!(prepared.origin(), Some("Owner"));
        assert_eq!(
            prepared.receiver_compat(),
            &ReceiverOriginCompatibilityV1::SuppressedByExactType {
                owner: "Owner".to_string(),
            }
        );
    }

    #[test]
    fn field_base_never_inherits_receiver_box_synthesis() {
        let prepared = prepare(None, Some("Owner"), LocalKind::FieldBase);

        assert_eq!(prepared.exact_type(), None);
        assert!(!prepared.has_legacy_unknown());
        assert_eq!(prepared.origin(), Some("Owner"));
        assert_eq!(
            prepared.receiver_compat(),
            &ReceiverOriginCompatibilityV1::Inactive
        );
    }

    #[test]
    fn ordinary_consumers_preserve_missing_unknown_and_exact_entry_distinctions() {
        let missing = prepare(None, None, LocalKind::Arg);
        assert_eq!(missing.exact_type(), None);
        assert!(!missing.has_legacy_unknown());
        assert_eq!(missing.origin(), None);
        assert_eq!(
            missing.receiver_compat(),
            &ReceiverOriginCompatibilityV1::Inactive
        );

        let unknown = prepare(Some(&MirType::Unknown), Some("Owner"), LocalKind::Arg);
        assert_eq!(unknown.exact_type(), None);
        assert!(unknown.has_legacy_unknown());
        assert_eq!(unknown.origin(), Some("Owner"));
        assert_eq!(
            unknown.receiver_compat(),
            &ReceiverOriginCompatibilityV1::Inactive
        );

        let exact = prepare(Some(&MirType::Void), Some("Owner"), LocalKind::Arg);
        assert_eq!(exact.exact_type(), Some(&MirType::Void));
        assert!(!exact.has_legacy_unknown());
        assert_eq!(exact.origin(), Some("Owner"));
        assert_eq!(
            exact.receiver_compat(),
            &ReceiverOriginCompatibilityV1::Inactive
        );
    }

    #[test]
    fn materialization_classification_keeps_copy_and_non_copy_surfaces_distinct() {
        let copy = prepare(None, None, LocalKind::Arg);
        assert_eq!(
            copy.materialization(),
            LocalSsaMaterializationKindV1::PhysicalCopy(
                LocalSsaPhysicalCopyReasonV1::DominatingFallbackCopy
            )
        );
        assert_ne!(
            LocalSsaMaterializationKindV1::RematerializedSelect,
            LocalSsaMaterializationKindV1::PhysicalCopy(
                LocalSsaPhysicalCopyReasonV1::RematerializedCopy
            )
        );
    }

    #[test]
    fn synthetic_success_commit_preserves_the_full_type_origin_receiver_matrix() {
        struct Case {
            source_type: Option<MirType>,
            origin: Option<&'static str>,
            kind: LocalKind,
            expected_type: Option<MirType>,
            expected_origin: Option<&'static str>,
        }

        let cases = [
            Case {
                source_type: None,
                origin: None,
                kind: LocalKind::Arg,
                expected_type: None,
                expected_origin: None,
            },
            Case {
                source_type: Some(MirType::Integer),
                origin: None,
                kind: LocalKind::Arg,
                expected_type: Some(MirType::Integer),
                expected_origin: None,
            },
            Case {
                source_type: Some(MirType::Unknown),
                origin: Some("Owner"),
                kind: LocalKind::Arg,
                expected_type: Some(MirType::Unknown),
                expected_origin: Some("Owner"),
            },
            Case {
                source_type: None,
                origin: Some("Owner"),
                kind: LocalKind::Recv,
                expected_type: Some(MirType::Box("Owner".to_string())),
                expected_origin: Some("Owner"),
            },
            Case {
                source_type: Some(MirType::Unknown),
                origin: Some("Owner"),
                kind: LocalKind::Recv,
                expected_type: Some(MirType::Unknown),
                expected_origin: Some("Owner"),
            },
            Case {
                source_type: Some(MirType::Integer),
                origin: Some("Owner"),
                kind: LocalKind::Recv,
                expected_type: Some(MirType::Integer),
                expected_origin: Some("Owner"),
            },
            Case {
                source_type: None,
                origin: Some("Owner"),
                kind: LocalKind::FieldBase,
                expected_type: None,
                expected_origin: Some("Owner"),
            },
        ];

        for case in cases {
            let prepared = prepare(case.source_type.as_ref(), case.origin, case.kind);
            let mut committed = SyntheticCommittedPostSuccessV1::default();

            assert!(commit_on_synthetic_cache_miss(
                &prepared,
                false,
                &mut committed
            ));
            assert_eq!(committed.type_entry, case.expected_type);
            assert_eq!(committed.origin.as_deref(), case.expected_origin);
            assert!(committed.cache_inserted);
        }
    }

    #[test]
    fn synthetic_failure_commits_no_type_origin_or_cache_state() {
        let prepared = prepare(Some(&MirType::Unknown), Some("Owner"), LocalKind::Recv);
        let mut committed = SyntheticCommittedPostSuccessV1::default();

        assert!(!commit_after_synthetic_emission(
            &prepared,
            Err(()),
            &mut committed
        ));

        assert_eq!(committed, SyntheticCommittedPostSuccessV1::default());
    }

    #[test]
    fn synthetic_cache_hit_skips_metadata_republication() {
        let prepared = prepare(Some(&MirType::Integer), Some("Owner"), LocalKind::Recv);
        let mut committed = SyntheticCommittedPostSuccessV1 {
            type_entry: Some(MirType::Unknown),
            origin: Some("Existing".to_string()),
            cache_inserted: true,
        };
        let before = committed.clone();

        assert!(!commit_on_synthetic_cache_miss(
            &prepared,
            true,
            &mut committed
        ));
        assert_eq!(committed, before);
    }

    #[test]
    fn every_materialization_kind_uses_the_same_prepared_legacy_unknown_state() {
        let kinds = [
            LocalSsaMaterializationKindV1::RematerializedConst,
            LocalSsaMaterializationKindV1::RematerializedBinOp,
            LocalSsaMaterializationKindV1::RematerializedCompare,
            LocalSsaMaterializationKindV1::RematerializedSelect,
            LocalSsaMaterializationKindV1::PhysicalCopy(
                LocalSsaPhysicalCopyReasonV1::RematerializedCopy,
            ),
            LocalSsaMaterializationKindV1::PhysicalCopy(
                LocalSsaPhysicalCopyReasonV1::DominatingFallbackCopy,
            ),
        ];

        for materialization in kinds {
            let prepared = PreparedLocalSsaPostSuccessV1::prepare(
                &LocalSsaSourceTypeEntryV1::StoredUnknown,
                Some("Owner"),
                materialization,
                LocalKind::Recv,
            );
            let mut committed = SyntheticCommittedPostSuccessV1::default();

            assert!(commit_on_synthetic_cache_miss(
                &prepared,
                false,
                &mut committed
            ));
            assert_eq!(committed.type_entry, Some(MirType::Unknown));
            assert_eq!(committed.origin.as_deref(), Some("Owner"));
        }
    }
}
