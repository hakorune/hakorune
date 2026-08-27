use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiteOwnerMapReferenceFailureV1 {
    InvalidRevision,
    RevisionNotCommit,
    RevisionNotAncestor,
    RepositoryUnavailable,
    PathMissing,
    PathNotBlob,
    BlobNonUtf8,
    AnchorUnsupported,
    RangeOutOfBounds,
    SymbolMissing,
    SymbolAmbiguous,
    EdgeUnsupported,
}

impl fmt::Display for SiteOwnerMapReferenceFailureV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let tag = match self {
            Self::InvalidRevision => "invalid-revision",
            Self::RevisionNotCommit => "revision-not-commit",
            Self::RevisionNotAncestor => "revision-not-ancestor",
            Self::RepositoryUnavailable => "repository-unavailable",
            Self::PathMissing => "path-missing",
            Self::PathNotBlob => "path-not-blob",
            Self::BlobNonUtf8 => "blob-non-utf8",
            Self::AnchorUnsupported => "anchor-unsupported",
            Self::RangeOutOfBounds => "range-out-of-bounds",
            Self::SymbolMissing => "symbol-missing",
            Self::SymbolAmbiguous => "symbol-ambiguous",
            Self::EdgeUnsupported => "edge-unsupported",
        };
        formatter.write_str(tag)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChronicScanErrorV1 {
    EmptyManifestPath,
    ManifestRead {
        detail: String,
    },
    ManifestParse {
        detail: String,
    },
    InvalidManifest {
        detail: String,
    },
    EmptyScope,
    DuplicateScopePath {
        path: String,
    },
    PathEscape {
        path: String,
    },
    SymlinkInput {
        path: String,
    },
    ScopeEntryMissing {
        path: String,
    },
    ScopeEntryKindMismatch {
        path: String,
    },
    DirectoryRead {
        path: String,
        detail: String,
    },
    SourceRead {
        path: String,
        detail: String,
    },
    NonUtf8Source {
        path: String,
    },
    ParseFailed {
        path: String,
        detail: String,
    },
    MalformedAttribute {
        path: String,
        detail: String,
    },
    UnsupportedTokenShape {
        path: String,
        detail: String,
    },
    DuplicateObservation {
        path: String,
        key: String,
    },
    SourceChangedDuringObservation {
        path: String,
    },
    ScopeDrift {
        detail: String,
    },
    ReportSerialize {
        detail: String,
    },
    InvalidSourceCommit {
        detail: String,
    },
    ObservationReceiptInvalid {
        detail: String,
    },
    ObservationReceiptDuplicateKey {
        key: String,
    },
    ObservationReceiptOutOfOrder {
        previous: String,
        current: String,
    },
    ObservationReceiptCountDrift {
        expected: usize,
        actual: usize,
    },
    ObservationReceiptHashDrift {
        expected: String,
        actual: String,
    },
    SiteOwnerMapInvalid {
        detail: String,
    },
    SiteOwnerMapCoverageDrift {
        detail: String,
    },
    SiteOwnerMapHashDrift {
        expected: String,
        actual: String,
    },
    SiteOwnerMapReference {
        reference: String,
        failure: SiteOwnerMapReferenceFailureV1,
        detail: String,
    },
}

impl fmt::Display for ChronicScanErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyManifestPath => write!(formatter, "[chronic-scan/empty-manifest-path]"),
            Self::ManifestRead { detail } => {
                write!(formatter, "[chronic-scan/manifest-read] {detail}")
            }
            Self::ManifestParse { detail } => {
                write!(formatter, "[chronic-scan/manifest-parse] {detail}")
            }
            Self::InvalidManifest { detail } => {
                write!(formatter, "[chronic-scan/invalid-manifest] {detail}")
            }
            Self::EmptyScope => write!(formatter, "[chronic-scan/empty-scope]"),
            Self::DuplicateScopePath { path } => {
                write!(formatter, "[chronic-scan/duplicate-scope-path] {path}")
            }
            Self::PathEscape { path } => {
                write!(formatter, "[chronic-scan/path-escape] {path}")
            }
            Self::SymlinkInput { path } => {
                write!(formatter, "[chronic-scan/symlink-input] {path}")
            }
            Self::ScopeEntryMissing { path } => {
                write!(formatter, "[chronic-scan/scope-entry-missing] {path}")
            }
            Self::ScopeEntryKindMismatch { path } => {
                write!(formatter, "[chronic-scan/scope-entry-kind-mismatch] {path}")
            }
            Self::DirectoryRead { path, detail } => {
                write!(formatter, "[chronic-scan/directory-read] {path}: {detail}")
            }
            Self::SourceRead { path, detail } => {
                write!(formatter, "[chronic-scan/source-read] {path}: {detail}")
            }
            Self::NonUtf8Source { path } => {
                write!(formatter, "[chronic-scan/non-utf8-source] {path}")
            }
            Self::ParseFailed { path, detail } => {
                write!(formatter, "[chronic-scan/parse-failed] {path}: {detail}")
            }
            Self::MalformedAttribute { path, detail } => {
                write!(
                    formatter,
                    "[chronic-scan/malformed-attribute] {path}: {detail}"
                )
            }
            Self::UnsupportedTokenShape { path, detail } => {
                write!(
                    formatter,
                    "[chronic-scan/unsupported-token-shape] {path}: {detail}"
                )
            }
            Self::DuplicateObservation { path, key } => {
                write!(
                    formatter,
                    "[chronic-scan/duplicate-observation] {path}: {key}"
                )
            }
            Self::SourceChangedDuringObservation { path } => {
                write!(
                    formatter,
                    "[chronic-scan/source-changed-during-observation] {path}"
                )
            }
            Self::ScopeDrift { detail } => {
                write!(formatter, "[chronic-scan/scope-drift] {detail}")
            }
            Self::ReportSerialize { detail } => {
                write!(formatter, "[chronic-scan/report-serialize] {detail}")
            }
            Self::InvalidSourceCommit { detail } => {
                write!(formatter, "[chronic-scan/invalid-source-commit] {detail}")
            }
            Self::ObservationReceiptInvalid { detail } => {
                write!(formatter, "[chronic-scan/observation-receipt-invalid] {detail}")
            }
            Self::ObservationReceiptDuplicateKey { key } => {
                write!(formatter, "[chronic-scan/observation-receipt-duplicate-key] {key}")
            }
            Self::ObservationReceiptOutOfOrder { previous, current } => write!(
                formatter,
                "[chronic-scan/observation-receipt-out-of-order] previous={previous} current={current}"
            ),
            Self::ObservationReceiptCountDrift { expected, actual } => write!(
                formatter,
                "[chronic-scan/observation-receipt-count-drift] expected={expected} actual={actual}"
            ),
            Self::ObservationReceiptHashDrift { expected, actual } => write!(
                formatter,
                "[chronic-scan/observation-receipt-hash-drift] expected={expected} actual={actual}"
            ),
            Self::SiteOwnerMapInvalid { detail } => {
                write!(formatter, "[chronic-scan/site-owner-map-invalid] {detail}")
            }
            Self::SiteOwnerMapCoverageDrift { detail } => {
                write!(formatter, "[chronic-scan/site-owner-map-coverage-drift] {detail}")
            }
            Self::SiteOwnerMapHashDrift { expected, actual } => write!(
                formatter,
                "[chronic-scan/site-owner-map-hash-drift] expected={expected} actual={actual}"
            ),
            Self::SiteOwnerMapReference {
                reference,
                failure,
                detail,
            } => write!(
                formatter,
                "[chronic-scan/site-owner-map-reference/{failure}] {reference}: {detail}"
            ),
        }
    }
}

impl std::error::Error for ChronicScanErrorV1 {}
