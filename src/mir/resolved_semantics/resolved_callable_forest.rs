//! Co-sealed source-unit callable catalog and semantic owner forest.

use super::{
    CallableCatalogCardinalityErrorV1, FunctionOwnerIdV1, VerifiedCallableIndexV1,
    VerifiedSemanticOwnerForestV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum ResolvedCallableForestVerificationErrorV1 {
    CallableCatalogCardinality(CallableCatalogCardinalityErrorV1),
    IndexOwnerIsNotSoleRoot {
        index_owner: FunctionOwnerIdV1,
        root: FunctionOwnerIdV1,
    },
    TargetOutsideIndex {
        function: FunctionOwnerIdV1,
        target: FunctionOwnerIdV1,
    },
}

/// Source-unit authority for callable headers and function-local resolution.
#[derive(Debug)]
pub(crate) struct VerifiedResolvedCallableForestV1 {
    forest: VerifiedSemanticOwnerForestV1,
    callable_index: VerifiedCallableIndexV1,
}

impl VerifiedResolvedCallableForestV1 {
    pub(super) fn seal(
        forest: VerifiedSemanticOwnerForestV1,
        callable_index: VerifiedCallableIndexV1,
    ) -> Result<Self, ResolvedCallableForestVerificationErrorV1> {
        let root = forest.roots()[0];
        let index_owner = callable_index
            .sole_header()
            .map_err(ResolvedCallableForestVerificationErrorV1::CallableCatalogCardinality)?
            .callable()
            .owner();
        if forest.owner_count() != 1 || index_owner != root {
            return Err(
                ResolvedCallableForestVerificationErrorV1::IndexOwnerIsNotSoleRoot {
                    index_owner,
                    root,
                },
            );
        }
        for (function, product) in forest.owners() {
            for (_, target) in product.direct_call_targets() {
                let target_owner = target.callable().owner();
                if callable_index
                    .header_for_callable(target.callable())
                    .is_err()
                {
                    return Err(
                        ResolvedCallableForestVerificationErrorV1::TargetOutsideIndex {
                            function,
                            target: target_owner,
                        },
                    );
                }
            }
        }
        Ok(Self {
            forest,
            callable_index,
        })
    }

    pub(crate) const fn forest(&self) -> &VerifiedSemanticOwnerForestV1 {
        &self.forest
    }

    pub(crate) const fn callable_index(&self) -> &VerifiedCallableIndexV1 {
        &self.callable_index
    }

    #[cfg(test)]
    pub(super) fn into_parts(self) -> (VerifiedSemanticOwnerForestV1, VerifiedCallableIndexV1) {
        (self.forest, self.callable_index)
    }
}
