//! Owner-closed input derived only from a verified resolved source unit.

use crate::mir::resolved_semantics::{
    locate_catalog_function_v1, CanonicalCallableKeyV1, FunctionOwnerIdV1,
    VerifiedCallableHeaderV1, VerifiedCallableIndexV1, VerifiedResolvedFunctionV1,
    VerifiedSemanticOwnerForestV1,
};

use super::lowering_input::{CanonicalLoweringErrorV1, VerifiedResolvedSourceUnitV1};
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;
use super::source_projection::VerifiedSourceProjectionV1;
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
        ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            self.syntax_root(),
            self.forest(),
            self.projection(),
        )
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
    pub(in crate::mir) fn from_exact_parts_without_callable(
        syntax_root: &'a crate::ast::ASTNode,
        forest: &'a VerifiedSemanticOwnerForestV1,
        projection: &'a VerifiedSourceProjectionV1,
    ) -> Result<Self, CanonicalLoweringErrorV1> {
        let [owner] = forest.roots() else {
            return Err(CanonicalLoweringErrorV1::SourceUnitResolution {
                detail: "verified_forest_root_count_is_not_one".to_string(),
            });
        };
        let function =
            forest
                .owner(*owner)
                .ok_or_else(|| CanonicalLoweringErrorV1::SourceUnitResolution {
                    detail: "verified_forest_root_product_missing".to_string(),
                })?;
        let source =
            FunctionSourceViewV1::from_exact_parts(syntax_root, *owner, forest, projection)
                .map_err(|error| CanonicalLoweringErrorV1::SourceNavigation {
                    detail: error.to_string(),
                })?;
        Ok(Self {
            owner: *owner,
            source,
            function,
            forest,
            callable_index: None,
            callable_header: None,
        })
    }

    pub(super) fn from_exact_parts_with_callable_index(
        syntax_root: &'a crate::ast::ASTNode,
        forest: &'a VerifiedSemanticOwnerForestV1,
        projection: &'a VerifiedSourceProjectionV1,
        callable_index: &'a VerifiedCallableIndexV1,
    ) -> Result<Self, CanonicalLoweringErrorV1> {
        let mut input = Self::from_exact_parts_without_callable(syntax_root, forest, projection)?;
        input.callable_index = Some(callable_index);
        Ok(input)
    }

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
