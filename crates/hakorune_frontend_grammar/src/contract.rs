//! Parser-neutral Language v1 grammar contract data and witness comparison.
//!
//! Parser implementations remain independent. This module only owns the
//! generated contract projection and the fail-fast comparison boundary.

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum GrammarProfile {
    Canonical,
    Compat2025,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GrammarStatus {
    Canonical,
    CompatibilityOnly,
    Reserved,
    Rejected,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NormalizationMode {
    CanonicalShape,
    CompatibilityAlias,
    CompatibilityTransport,
    None,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceParticipant {
    HakoSemanticParser,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransportOwner {
    RustMigrationToolingOnly,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConformanceScope {
    Required,
    ExplicitlyExcluded { transport_owner: TransportOwner },
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GrammarContractRow {
    pub row_id: &'static str,
    pub family: &'static str,
    pub spelling_id: &'static str,
    pub profile: GrammarProfile,
    pub status: GrammarStatus,
    pub production: &'static str,
    pub normalization_mode: NormalizationMode,
    pub normalized_shape: &'static str,
    pub semantic_owner: &'static str,
    pub stable_reject_tag: &'static str,
    pub positive_fixture_ids: &'static [&'static str],
    pub negative_fixture_ids: &'static [&'static str],
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NormalizedSyntaxNode {
    pub kind: String,
    pub children: Vec<NormalizedSyntaxNode>,
}

impl NormalizedSyntaxNode {
    pub fn leaf(kind: impl Into<String>) -> Self {
        Self {
            kind: kind.into(),
            children: Vec::new(),
        }
    }

    pub fn branch(
        kind: impl Into<String>,
        children: impl IntoIterator<Item = NormalizedSyntaxNode>,
    ) -> Self {
        Self {
            kind: kind.into(),
            children: children.into_iter().collect(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ParseWitness {
    pub row_id: String,
    pub profile: GrammarProfile,
    pub accepted: bool,
    pub normalized_form: Option<NormalizedSyntaxNode>,
    pub stable_reject_tag: String,
    pub migration_transport_ref: Option<String>,
}

impl ParseWitness {
    pub fn accepted(
        row_id: impl Into<String>,
        profile: GrammarProfile,
        normalized_form: NormalizedSyntaxNode,
    ) -> Self {
        Self {
            row_id: row_id.into(),
            profile,
            accepted: true,
            normalized_form: Some(normalized_form),
            stable_reject_tag: String::new(),
            migration_transport_ref: None,
        }
    }

    pub fn rejected(
        row_id: impl Into<String>,
        profile: GrammarProfile,
        stable_reject_tag: impl Into<String>,
    ) -> Self {
        Self {
            row_id: row_id.into(),
            profile,
            accepted: false,
            normalized_form: None,
            stable_reject_tag: stable_reject_tag.into(),
            migration_transport_ref: None,
        }
    }

    pub fn accepted_transport(
        row_id: impl Into<String>,
        profile: GrammarProfile,
        transport_ref: impl Into<String>,
    ) -> Self {
        Self {
            row_id: row_id.into(),
            profile,
            accepted: true,
            normalized_form: Some(NormalizedSyntaxNode::leaf("CompatibilityTransport")),
            stable_reject_tag: String::new(),
            migration_transport_ref: Some(transport_ref.into()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WitnessComparisonError {
    RegistryRowMissing,
    WitnessMissing,
    StableRejectTagMissing,
    ProfileMismatch,
    CompatibilityTransportOnly,
    WitnessDrift { field: &'static str },
}

impl WitnessComparisonError {
    pub const fn stable_reject_tag(&self) -> &'static str {
        match self {
            Self::RegistryRowMissing => "parser/registry_row_missing",
            Self::WitnessMissing => "parser/witness_missing",
            Self::StableRejectTagMissing => "parser/stable_reject_tag_missing",
            Self::ProfileMismatch => "parser/profile_mismatch",
            Self::CompatibilityTransportOnly => "parser/from_compat_transport_only",
            Self::WitnessDrift { .. } => "parser/witness_drift",
        }
    }
}

pub fn find_row(row_id: &str, profile: GrammarProfile) -> Option<&'static GrammarContractRow> {
    crate::generated_contract::LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS
        .iter()
        .find(|row| row.row_id == row_id && row.profile == profile)
}

pub fn conformance_scope(
    row: &GrammarContractRow,
    participant: ConformanceParticipant,
) -> ConformanceScope {
    match (participant, row.normalization_mode) {
        (ConformanceParticipant::HakoSemanticParser, NormalizationMode::CompatibilityTransport) => {
            ConformanceScope::ExplicitlyExcluded {
                transport_owner: TransportOwner::RustMigrationToolingOnly,
            }
        }
        _ => ConformanceScope::Required,
    }
}

pub fn require_canonical_semantic_entry(
    row: &GrammarContractRow,
) -> Result<(), WitnessComparisonError> {
    if row.normalization_mode == NormalizationMode::CompatibilityTransport {
        return Err(WitnessComparisonError::CompatibilityTransportOnly);
    }
    Ok(())
}

pub fn compare_witnesses(
    row: Option<&GrammarContractRow>,
    expected: Option<&ParseWitness>,
    observed: Option<&ParseWitness>,
) -> Result<(), WitnessComparisonError> {
    let row = row.ok_or(WitnessComparisonError::RegistryRowMissing)?;
    let expected = expected.ok_or(WitnessComparisonError::WitnessMissing)?;
    let observed = observed.ok_or(WitnessComparisonError::WitnessMissing)?;
    if expected.profile != row.profile || observed.profile != row.profile {
        return Err(WitnessComparisonError::ProfileMismatch);
    }
    if expected.row_id != row.row_id || observed.row_id != row.row_id {
        return Err(WitnessComparisonError::WitnessDrift { field: "row_id" });
    }
    if expected.accepted != observed.accepted {
        return Err(WitnessComparisonError::WitnessDrift { field: "accepted" });
    }
    if expected.normalized_form != observed.normalized_form {
        return Err(WitnessComparisonError::WitnessDrift {
            field: "normalized_form",
        });
    }
    if !expected.accepted && expected.stable_reject_tag.is_empty() {
        return Err(WitnessComparisonError::StableRejectTagMissing);
    }
    if expected.stable_reject_tag != observed.stable_reject_tag {
        return Err(WitnessComparisonError::WitnessDrift {
            field: "stable_reject_tag",
        });
    }
    if expected.migration_transport_ref != observed.migration_transport_ref {
        return Err(WitnessComparisonError::WitnessDrift {
            field: "migration_transport_ref",
        });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::generated_contract::LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS;

    fn row() -> GrammarContractRow {
        GrammarContractRow {
            row_id: "try_statement",
            family: "exception",
            spelling_id: "try_statement",
            profile: GrammarProfile::Canonical,
            status: GrammarStatus::Reserved,
            production: "try block",
            normalization_mode: NormalizationMode::None,
            normalized_shape: "",
            semantic_owner: "grammar-contract",
            stable_reject_tag: "parser/try_reserved",
            positive_fixture_ids: &[],
            negative_fixture_ids: &["try_canonical_reject"],
        }
    }

    fn rejected(tag: &str) -> ParseWitness {
        ParseWitness::rejected("try_statement", GrammarProfile::Canonical, tag)
    }

    #[test]
    fn comparator_rejects_missing_row() {
        assert_eq!(
            compare_witnesses(
                None,
                Some(&rejected("parser/try_reserved")),
                Some(&rejected("parser/try_reserved"))
            ),
            Err(WitnessComparisonError::RegistryRowMissing)
        );
    }

    #[test]
    fn comparator_rejects_missing_witness() {
        assert_eq!(
            compare_witnesses(Some(&row()), None, Some(&rejected("parser/try_reserved"))),
            Err(WitnessComparisonError::WitnessMissing)
        );
    }

    #[test]
    fn comparator_rejects_missing_reject_tag() {
        assert_eq!(
            compare_witnesses(Some(&row()), Some(&rejected("")), Some(&rejected(""))),
            Err(WitnessComparisonError::StableRejectTagMissing)
        );
    }

    #[test]
    fn comparator_rejects_profile_drift() {
        let mut observed = rejected("parser/try_reserved");
        observed.profile = GrammarProfile::Compat2025;
        assert_eq!(
            compare_witnesses(
                Some(&row()),
                Some(&rejected("parser/try_reserved")),
                Some(&observed)
            ),
            Err(WitnessComparisonError::ProfileMismatch)
        );
    }

    #[test]
    fn comparator_rejects_normalized_shape_drift() {
        let expected = ParseWitness::accepted(
            "try_statement",
            GrammarProfile::Canonical,
            NormalizedSyntaxNode::branch("PostfixCatch", [NormalizedSyntaxNode::leaf("Body")]),
        );
        let observed = ParseWitness::accepted(
            "try_statement",
            GrammarProfile::Canonical,
            NormalizedSyntaxNode::branch("TryCatch", [NormalizedSyntaxNode::leaf("Body")]),
        );
        assert_eq!(
            compare_witnesses(Some(&row()), Some(&expected), Some(&observed)),
            Err(WitnessComparisonError::WitnessDrift {
                field: "normalized_form"
            })
        );
    }

    #[test]
    fn comparator_rejects_nested_normalized_form_drift() {
        let expected = ParseWitness::accepted(
            "try_statement",
            GrammarProfile::Canonical,
            NormalizedSyntaxNode::branch(
                "Match",
                [NormalizedSyntaxNode::branch(
                    "OrderedArms",
                    [NormalizedSyntaxNode::branch(
                        "MatchArm",
                        [NormalizedSyntaxNode::leaf("Pattern")],
                    )],
                )],
            ),
        );
        let observed = ParseWitness::accepted(
            "try_statement",
            GrammarProfile::Canonical,
            NormalizedSyntaxNode::branch(
                "Match",
                [NormalizedSyntaxNode::branch(
                    "OrderedArms",
                    [NormalizedSyntaxNode::branch(
                        "MatchArm",
                        [NormalizedSyntaxNode::leaf("ArmBody")],
                    )],
                )],
            ),
        );

        assert_eq!(
            compare_witnesses(Some(&row()), Some(&expected), Some(&observed)),
            Err(WitnessComparisonError::WitnessDrift {
                field: "normalized_form"
            })
        );
    }

    #[test]
    fn generated_rows_close_the_four_families_for_both_profiles() {
        let pairs = LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS
            .iter()
            .map(|row| (row.row_id, row.profile))
            .collect::<BTreeSet<_>>();
        assert_eq!(LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS.len(), 22);
        assert_eq!(pairs.len(), 22);
        for row_id in [
            "guard_expr_else",
            "guard_let_else",
            "postfix_catch",
            "postfix_cleanup",
            "fini",
            "try_statement",
            "match",
            "peek",
            "delegate_exposes",
            "box_from_inheritance",
            "from_super_call",
        ] {
            assert!(find_row(row_id, GrammarProfile::Canonical).is_some());
            assert!(find_row(row_id, GrammarProfile::Compat2025).is_some());
        }
    }

    #[test]
    fn compat_transport_cannot_enter_canonical_semantics() {
        let row = find_row("from_super_call", GrammarProfile::Compat2025)
            .expect("generated from transport row");
        let error = require_canonical_semantic_entry(row).unwrap_err();
        assert_eq!(error, WitnessComparisonError::CompatibilityTransportOnly);
        assert_eq!(
            error.stable_reject_tag(),
            "parser/from_compat_transport_only"
        );
    }

    #[test]
    fn hako_semantic_conformance_explicitly_excludes_only_transport_rows() {
        let scopes = LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS
            .iter()
            .map(|row| {
                (
                    row,
                    conformance_scope(row, ConformanceParticipant::HakoSemanticParser),
                )
            })
            .collect::<Vec<_>>();
        let excluded = scopes
            .iter()
            .filter(|(_, scope)| matches!(scope, ConformanceScope::ExplicitlyExcluded { .. }))
            .collect::<Vec<_>>();
        let transport_row_count = LANGUAGE_V1_GRAMMAR_CONTRACT_ROWS
            .iter()
            .filter(|row| row.normalization_mode == NormalizationMode::CompatibilityTransport)
            .count();

        assert!(transport_row_count > 0);
        assert_eq!(excluded.len(), transport_row_count);
        for (row, scope) in excluded {
            assert_eq!(row.profile, GrammarProfile::Compat2025);
            assert_eq!(
                row.normalization_mode,
                NormalizationMode::CompatibilityTransport
            );
            assert_eq!(
                *scope,
                ConformanceScope::ExplicitlyExcluded {
                    transport_owner: TransportOwner::RustMigrationToolingOnly,
                }
            );
        }
        assert!(scopes.iter().all(|(row, scope)| {
            row.normalization_mode == NormalizationMode::CompatibilityTransport
                || *scope == ConformanceScope::Required
        }));
    }
}
