//! Program-owned semantic forest and source projection for Main.main/0.
//!
//! The source unit remains the sole AST owner. Resolution borrows the exact
//! method selected by the normal source-family classifier and stores only
//! owner/source-navigation products.

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::lowering_input::CanonicalLoweringErrorV1;
use crate::mir::compiler::source_projection::{
    SourceNavigationErrorV1, VerifiedSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    FunctionSemanticResolverSessionV1, FunctionSyntaxViewV1, ResolveFunctionErrorV1,
    ResolveOwnerForestErrorV1, VerifiedSemanticOwnerForestV1,
};

use super::main_source::{NormalMainFunctionSourceViewV1, VerifiedNormalMainFunctionSourceUnitV1};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct VerifiedNormalMainRoleV1 {
    _seal: VerifiedNormalMainRoleSealV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct VerifiedNormalMainRoleSealV1;

impl VerifiedNormalMainRoleV1 {
    fn seal() -> Self {
        Self {
            _seal: VerifiedNormalMainRoleSealV1,
        }
    }

    pub(super) fn seal_for_direct_call() -> Self {
        Self::seal()
    }
}

#[derive(Debug)]
pub(crate) enum NormalMainResolvedSourceErrorV1 {
    FunctionShapeDrift,
    ResolverSession(ResolveFunctionErrorV1),
    OwnerForest(ResolveOwnerForestErrorV1),
    SourceProjection(SourceNavigationErrorV1),
}

#[derive(Debug)]
pub(crate) struct VerifiedNormalMainResolvedSourceUnitV1 {
    source: VerifiedNormalMainFunctionSourceUnitV1,
    forest: VerifiedSemanticOwnerForestV1,
    projection: VerifiedSourceProjectionV1,
    role: VerifiedNormalMainRoleV1,
    _seal: VerifiedNormalMainResolvedSourceUnitSealV1,
}

#[derive(Debug)]
struct VerifiedNormalMainResolvedSourceUnitSealV1;

impl VerifiedNormalMainFunctionSourceUnitV1 {
    pub(crate) fn prepare_embedded_resolved_main(
        self,
    ) -> Result<VerifiedNormalMainResolvedSourceUnitV1, RejectedNormalMainResolvedSourceV1> {
        let source = self.borrow_exact_function();
        match resolve_normal_main_loan_v1(&source) {
            Ok((forest, projection, role)) => Ok(VerifiedNormalMainResolvedSourceUnitV1 {
                source: self,
                forest,
                projection,
                role,
                _seal: VerifiedNormalMainResolvedSourceUnitSealV1,
            }),
            Err(error) => Err(RejectedNormalMainResolvedSourceV1 { owner: self, error }),
        }
    }
}

impl VerifiedNormalMainResolvedSourceUnitV1 {
    pub(crate) fn borrow_function_input(
        &self,
    ) -> Result<ResolvedFunctionLoweringInputV1<'_>, CanonicalLoweringErrorV1> {
        let source = self.source.borrow_exact_function();
        ResolvedFunctionLoweringInputV1::from_exact_parts_without_callable(
            source.function(),
            &self.forest,
            &self.projection,
        )
    }

    pub(crate) const fn role(&self) -> VerifiedNormalMainRoleV1 {
        self.role
    }

    pub(in crate::mir) fn into_source(self) -> VerifiedNormalMainFunctionSourceUnitV1 {
        self.source
    }

    #[cfg(test)]
    fn source_function_for_test(&self) -> &crate::ast::ASTNode {
        self.source.borrow_exact_function().function()
    }
}

#[derive(Debug)]
pub(crate) struct RejectedNormalMainResolvedSourceV1 {
    owner: VerifiedNormalMainFunctionSourceUnitV1,
    error: NormalMainResolvedSourceErrorV1,
}

impl RejectedNormalMainResolvedSourceV1 {
    pub(crate) fn error(&self) -> &NormalMainResolvedSourceErrorV1 {
        &self.error
    }

    pub(crate) fn discard(self) {
        drop(self);
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        VerifiedNormalMainFunctionSourceUnitV1,
        NormalMainResolvedSourceErrorV1,
    ) {
        (self.owner, self.error)
    }
}

pub(super) fn resolve_normal_main_loan_v1(
    function: &NormalMainFunctionSourceViewV1<'_>,
) -> Result<
    (
        VerifiedSemanticOwnerForestV1,
        VerifiedSourceProjectionV1,
        VerifiedNormalMainRoleV1,
    ),
    NormalMainResolvedSourceErrorV1,
> {
    let syntax = FunctionSyntaxViewV1::from_ast(function.function())
        .ok_or(NormalMainResolvedSourceErrorV1::FunctionShapeDrift)?;
    let mut session = FunctionSemanticResolverSessionV1::new(0)
        .map_err(NormalMainResolvedSourceErrorV1::ResolverSession)?;
    let forest = session
        .resolve_forest(syntax)
        .map_err(NormalMainResolvedSourceErrorV1::OwnerForest)?;
    let projection = VerifiedSourceProjectionV1::seal(function.function(), &forest)
        .map_err(NormalMainResolvedSourceErrorV1::SourceProjection)?;
    Ok((forest, projection, VerifiedNormalMainRoleV1::seal()))
}

#[cfg(test)]
#[path = "main_resolved_source_tests.rs"]
mod tests;
