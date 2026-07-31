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

    /// Returns whether `site` is one direct member of this owner's root body.
    ///
    /// A `Sequence` region is rooted at the profile's exact body-root receipt,
    /// not at a function-shaped fallback.  Keeping the root/member pairing
    /// here prevents verifier consumers from reconstructing Script membership
    /// with ad-hoc `ProgramBody` branches.
    pub(crate) fn contains_sequence_member(
        self,
        origin: &[SourcePathSegmentV1],
        site: &[SourcePathSegmentV1],
    ) -> bool {
        let Some((origin_root, prefix)) = origin.split_last() else {
            return false;
        };
        *origin_root == self.body_root()
            && site.len() > prefix.len()
            && site.starts_with(prefix)
            && matches!(
                (self, &site[prefix.len()]),
                (Self::DeclaredFunction { .. }, SourcePathSegmentV1::Body(_))
                    | (Self::Script, SourcePathSegmentV1::ProgramBody(_))
                    | (Self::Lambda, SourcePathSegmentV1::LambdaBody(_))
            )
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

    #[test]
    fn sequence_members_are_profile_exact() {
        assert!(SemanticOwnerRootProfileV1::Script.contains_sequence_member(
            &[SourcePathSegmentV1::ProgramBodyRoot],
            &[SourcePathSegmentV1::ProgramBody(3)],
        ));
        assert!(!SemanticOwnerRootProfileV1::Script.contains_sequence_member(
            &[SourcePathSegmentV1::ProgramBodyRoot],
            &[SourcePathSegmentV1::Body(3)],
        ));
    }
}
