//! Owner-closed input derived only from a verified resolved source unit.

use crate::mir::resolved_semantics::{
    FunctionOwnerIdV1, VerifiedCallableIndexV1, VerifiedResolvedFunctionV1,
    VerifiedSemanticOwnerForestV1,
};

use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::source_view::FunctionSourceViewV1;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ResolvedFunctionLoweringInputV1<'a> {
    owner: FunctionOwnerIdV1,
    source: FunctionSourceViewV1<'a>,
    function: &'a VerifiedResolvedFunctionV1,
    forest: &'a VerifiedSemanticOwnerForestV1,
    callable_index: Option<&'a VerifiedCallableIndexV1>,
}

impl VerifiedResolvedSourceUnitV1 {
    pub(crate) fn root_function_input(
        &self,
    ) -> Result<ResolvedFunctionLoweringInputV1<'_>, CanonicalLoweringErrorV1> {
        let [owner] = self.forest().roots() else {
            return Err(CanonicalLoweringErrorV1::SourceUnitResolution {
                detail: "verified_forest_root_count_is_not_one".to_string(),
            });
        };
        let function = self.forest().owner(*owner).ok_or_else(|| {
            CanonicalLoweringErrorV1::SourceUnitResolution {
                detail: "verified_forest_root_product_missing".to_string(),
            }
        })?;
        let source = self.function_source_view(*owner).map_err(|error| {
            CanonicalLoweringErrorV1::SourceNavigation {
                detail: error.to_string(),
            }
        })?;
        Ok(ResolvedFunctionLoweringInputV1 {
            owner: *owner,
            source,
            function,
            forest: self.forest(),
            callable_index: self.callable_index(),
        })
    }
}

impl<'a> ResolvedFunctionLoweringInputV1<'a> {
    pub(crate) const fn owner(self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn source(self) -> FunctionSourceViewV1<'a> {
        self.source
    }

    pub(crate) const fn function(self) -> &'a VerifiedResolvedFunctionV1 {
        self.function
    }

    pub(crate) const fn forest(self) -> &'a VerifiedSemanticOwnerForestV1 {
        self.forest
    }

    pub(crate) const fn callable_index(self) -> Option<&'a VerifiedCallableIndexV1> {
        self.callable_index
    }
}
