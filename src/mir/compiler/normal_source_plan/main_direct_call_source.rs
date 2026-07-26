//! Main semantic resolution through one complete helper catalog.
//!
//! Main is not a catalog member. Its owner is issued from the catalog's
//! retained resolver continuation, so Main and every helper share one
//! compilation brand without a second index or resolver.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::compiler::source_projection::{
    SourceNavigationErrorV1, VerifiedSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, CatalogSealedResolverContinuationV1, ResolveOwnerForestErrorV1,
    VerifiedCallableCatalogSourceUnitV1, VerifiedSemanticOwnerForestV1,
};

use super::callable_catalog_source::VerifiedNormalCallableCatalogSourceUnitV1;
use super::main_resolved_source::VerifiedNormalMainRoleV1;
use super::product::{NormalMainMethodSiteV1, NormalSourceIdentityV1, NormalTopLevelSiteV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum NormalMainDirectCallSourceStageV1 {
    MainSyntax,
    OwnerForest,
    SourceProjection,
}

#[derive(Debug)]
pub(crate) enum NormalMainDirectCallSourceErrorV1 {
    MainSyntaxDrift,
    OwnerForest(ResolveOwnerForestErrorV1),
    SourceProjection(SourceNavigationErrorV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalMainDirectCallSourceUnitV1 {
    source: VerifiedCallableCatalogSourceUnitV1,
    continuation: CatalogSealedResolverContinuationV1,
    identity: NormalSourceIdentityV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    role: VerifiedNormalMainRoleV1,
    _seal: VerifiedNormalMainDirectCallSourceUnitSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainDirectCallSourceUnitSealV1;

impl VerifiedNormalMainDirectCallSourceUnitV1 {
    pub(crate) fn source_identity(&self) -> &str {
        self.identity.display_name()
    }

    pub(crate) fn borrow_function_input(
        &self,
    ) -> Result<ResolvedFunctionLoweringInputV1<'_>, CanonicalLoweringErrorV1> {
        let function = self
            .source
            .embedded_function(
                self.main_box.statement_index(),
                self.main_method.method_key(),
            )
            .ok_or_else(|| CanonicalLoweringErrorV1::SourceNavigation {
                detail: "normal_main_embedded_function_disappeared".to_owned(),
            })?;
        ResolvedFunctionLoweringInputV1::from_exact_parts_with_callable_index(
            function.function_ast(),
            &self.forest,
            &self.projection,
            self.source.catalog().index(),
        )
    }

    pub(crate) const fn role(&self) -> VerifiedNormalMainRoleV1 {
        self.role
    }

    pub(crate) fn helper_count(&self) -> usize {
        self.source.catalog().len()
    }

    pub(super) fn into_parts(
        self,
    ) -> (
        CallableCatalogSealOutcomeV1,
        NormalSourceIdentityV1,
        NormalTopLevelSiteV1,
        NormalMainMethodSiteV1,
        VerifiedSemanticOwnerForestV1,
        VerifiedSourceProjectionV1,
        VerifiedNormalMainRoleV1,
    ) {
        (
            CallableCatalogSealOutcomeV1::restore(self.source, self.continuation),
            self.identity,
            self.main_box,
            self.main_method,
            self.forest,
            self.projection,
            self.role,
        )
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMainDirectCallSourceV1 {
    owner: VerifiedNormalCallableCatalogSourceUnitV1,
    stage: NormalMainDirectCallSourceStageV1,
    error: NormalMainDirectCallSourceErrorV1,
}

impl RejectedNormalMainDirectCallSourceV1 {
    pub(crate) const fn stage(&self) -> NormalMainDirectCallSourceStageV1 {
        self.stage
    }

    pub(crate) const fn error(&self) -> &NormalMainDirectCallSourceErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }
}

impl VerifiedNormalCallableCatalogSourceUnitV1 {
    pub(crate) fn prepare_main_with_helper_catalog(
        self,
    ) -> Result<VerifiedNormalMainDirectCallSourceUnitV1, RejectedNormalMainDirectCallSourceV1>
    {
        let (catalog, identity, main_box, main_method) = self.into_parts();
        let (source, continuation) = catalog.into_parts();
        let Some(function) =
            source.embedded_function(main_box.statement_index(), main_method.method_key())
        else {
            return Err(reject_parts(
                source,
                continuation,
                identity,
                main_box,
                main_method,
                NormalMainDirectCallSourceStageV1::MainSyntax,
                NormalMainDirectCallSourceErrorV1::MainSyntaxDrift,
            ));
        };
        let mut resolver = continuation.into_resolver();
        let forest = match resolver.resolve_forest_with_callable_index(
            function.function().function(),
            source.catalog().index(),
        ) {
            Ok(forest) => forest,
            Err(error) => {
                return Err(reject_parts(
                    source,
                    CatalogSealedResolverContinuationV1::restore(resolver),
                    identity,
                    main_box,
                    main_method,
                    NormalMainDirectCallSourceStageV1::OwnerForest,
                    NormalMainDirectCallSourceErrorV1::OwnerForest(error),
                ))
            }
        };
        let projection = match VerifiedSourceProjectionV1::seal(function.function_ast(), &forest) {
            Ok(projection) => projection,
            Err(error) => {
                return Err(reject_parts(
                    source,
                    CatalogSealedResolverContinuationV1::restore(resolver),
                    identity,
                    main_box,
                    main_method,
                    NormalMainDirectCallSourceStageV1::SourceProjection,
                    NormalMainDirectCallSourceErrorV1::SourceProjection(error),
                ))
            }
        };
        Ok(VerifiedNormalMainDirectCallSourceUnitV1 {
            source,
            continuation: CatalogSealedResolverContinuationV1::restore(resolver),
            identity,
            main_box,
            main_method,
            forest,
            projection,
            role: VerifiedNormalMainRoleV1::seal_for_direct_call(),
            _seal: VerifiedNormalMainDirectCallSourceUnitSealV1,
        })
    }
}

#[allow(clippy::too_many_arguments)]
fn reject_parts(
    source: VerifiedCallableCatalogSourceUnitV1,
    continuation: CatalogSealedResolverContinuationV1,
    identity: NormalSourceIdentityV1,
    main_box: NormalTopLevelSiteV1,
    main_method: NormalMainMethodSiteV1,
    stage: NormalMainDirectCallSourceStageV1,
    error: NormalMainDirectCallSourceErrorV1,
) -> RejectedNormalMainDirectCallSourceV1 {
    RejectedNormalMainDirectCallSourceV1 {
        owner: VerifiedNormalCallableCatalogSourceUnitV1::restore(
            CallableCatalogSealOutcomeV1::restore(source, continuation),
            identity,
            main_box,
            main_method,
        ),
        stage,
        error,
    }
}

#[cfg(test)]
#[path = "main_direct_call_source_tests.rs"]
mod tests;
