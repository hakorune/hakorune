//! Owner-closed input derived only from a verified resolved source unit.

use crate::mir::resolved_semantics::{
    locate_catalog_function_v1, CanonicalCallableKeyV1, FunctionOwnerIdV1,
    VerifiedCallableHeaderV1, VerifiedCallableIndexV1, VerifiedResolvedFunctionV1,
    VerifiedSemanticOwnerForestV1,
};

use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use super::source_view::FunctionSourceViewV1;

#[derive(Debug, Clone, Copy)]
pub(crate) struct ResolvedFunctionLoweringInputV1<'a> {
    owner: FunctionOwnerIdV1,
    source: FunctionSourceViewV1<'a>,
    function: &'a VerifiedResolvedFunctionV1,
    forest: &'a VerifiedSemanticOwnerForestV1,
    callable_index: Option<&'a VerifiedCallableIndexV1>,
    callable_header: Option<&'a VerifiedCallableHeaderV1>,
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
        let callable_index = self.callable_index();
        let callable_header = match callable_index {
            Some(index) => Some(index.sole_header().map_err(|error| {
                CanonicalLoweringErrorV1::SourceUnitResolution {
                    detail: format!("root_callable_header_cardinality={}", error.actual()),
                }
            })?),
            None => None,
        };
        Ok(ResolvedFunctionLoweringInputV1 {
            owner: *owner,
            source,
            function,
            forest: self.forest(),
            callable_index,
            callable_header,
        })
    }
}

impl VerifiedResolvedCallableModuleV1 {
    pub(crate) fn function_input(
        &self,
        key: &CanonicalCallableKeyV1,
    ) -> Result<ResolvedFunctionLoweringInputV1<'_>, CanonicalLoweringErrorV1> {
        let unit =
            self.function(key)
                .ok_or_else(|| CanonicalLoweringErrorV1::SourceUnitResolution {
                    detail: "resolved_callable_module_function_missing".to_string(),
                })?;
        let header = self.source().catalog().index().lookup(key).ok_or_else(|| {
            CanonicalLoweringErrorV1::SourceUnitResolution {
                detail: "resolved_callable_module_header_missing".to_string(),
            }
        })?;
        let located = locate_catalog_function_v1(self.source(), unit.declaration_site())
            .ok_or_else(|| CanonicalLoweringErrorV1::SourceNavigation {
                detail: "resolved_callable_module_function_syntax_missing".to_string(),
            })?;
        let owner = header.callable().owner();
        let function = unit.forest().owner(owner).ok_or_else(|| {
            CanonicalLoweringErrorV1::SourceUnitResolution {
                detail: "resolved_callable_module_root_product_missing".to_string(),
            }
        })?;
        let source = FunctionSourceViewV1::from_exact_parts(
            located.root(),
            owner,
            unit.forest(),
            unit.projection(),
        )
        .map_err(|error| CanonicalLoweringErrorV1::SourceNavigation {
            detail: error.to_string(),
        })?;
        Ok(ResolvedFunctionLoweringInputV1 {
            owner,
            source,
            function,
            forest: unit.forest(),
            callable_index: Some(self.source().catalog().index()),
            callable_header: Some(header),
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

    pub(crate) const fn callable_header(self) -> Option<&'a VerifiedCallableHeaderV1> {
        self.callable_header
    }
}
