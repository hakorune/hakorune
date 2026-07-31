//! Exact source-root contract for one semantic execution owner.
//!
//! The execution owner brand is shape-neutral. This profile carries the
//! source-root and body-root contract so consumers never infer it by scanning
//! arena origins.

use super::function_view::ReceiverPolicyV1;
use super::source_site::SourcePathSegmentV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SemanticOwnerRootProfileV1 {
    DeclaredFunction { receiver_policy: ReceiverPolicyV1 },
    Script,
    Lambda,
}

impl SemanticOwnerRootProfileV1 {
    pub(crate) const fn source_kind(self) -> super::SemanticOwnerSourceKindV1 {
        match self {
            Self::DeclaredFunction { .. } => super::SemanticOwnerSourceKindV1::DeclaredFunction,
            Self::Script => super::SemanticOwnerSourceKindV1::Script,
            Self::Lambda => super::SemanticOwnerSourceKindV1::Lambda,
        }
    }

    pub(crate) const fn body_root(self) -> SourcePathSegmentV1 {
        match self {
            Self::DeclaredFunction { .. } => SourcePathSegmentV1::FunctionBody,
            Self::Script => SourcePathSegmentV1::ProgramBodyRoot,
            Self::Lambda => SourcePathSegmentV1::LambdaBodyRoot,
        }
    }

    pub(crate) const fn receiver_policy(self) -> ReceiverPolicyV1 {
        match self {
            Self::DeclaredFunction { receiver_policy } => receiver_policy,
            Self::Script | Self::Lambda => ReceiverPolicyV1::Absent,
        }
    }

    pub(crate) fn matches_body_root(self, segments: &[SourcePathSegmentV1]) -> bool {
        matches!(segments, [segment] if *segment == self.body_root())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn profiles_have_disjoint_exact_body_roots() {
        let profiles = [
            SemanticOwnerRootProfileV1::DeclaredFunction {
                receiver_policy: ReceiverPolicyV1::Absent,
            },
            SemanticOwnerRootProfileV1::Script,
            SemanticOwnerRootProfileV1::Lambda,
        ];
        let roots = [
            [SourcePathSegmentV1::FunctionBody],
            [SourcePathSegmentV1::ProgramBodyRoot],
            [SourcePathSegmentV1::LambdaBodyRoot],
        ];

        let kinds = [
            super::super::SemanticOwnerSourceKindV1::DeclaredFunction,
            super::super::SemanticOwnerSourceKindV1::Script,
            super::super::SemanticOwnerSourceKindV1::Lambda,
        ];
        for ((profile, root), kind) in profiles.into_iter().zip(roots.iter()).zip(kinds) {
            assert!(profile.matches_body_root(root));
            assert_eq!(profile.source_kind(), kind);
        }
        assert!(!profiles[1].matches_body_root(&roots[0]));
        assert!(!profiles[0].matches_body_root(&roots[1]));
        assert!(!profiles[2].matches_body_root(&roots[1]));
    }
}
