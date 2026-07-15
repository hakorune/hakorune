//! Owned exact Program carrier for the P0c-B1 callable-module ingress.
//!
//! Program syntax, callable catalog, body resolution, and source projection
//! are sealed once. The compiler receives only a borrowed view of this product.

use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    CallableCatalogSealOutcomeV1, VerifiedCallableHeaderSourceUnitV1,
    VerifiedOwnerFreeCallableCatalogSourceUnitV1,
};

use super::lowering_input::CanonicalLoweringErrorV1;
use super::resolved_callable_module::VerifiedResolvedCallableModuleV1;

#[derive(Debug)]
pub struct VerifiedResolvedCallableProgramV1 {
    module: VerifiedResolvedCallableModuleV1,
}

impl VerifiedResolvedCallableProgramV1 {
    pub fn resolve(program: ASTNode) -> Result<Self, CanonicalLoweringErrorV1> {
        let source = VerifiedCallableHeaderSourceUnitV1::seal_header_surface(program)
            .map_err(|error| stage_error("header_surface", error))?;
        let owner_free = VerifiedOwnerFreeCallableCatalogSourceUnitV1::seal(source)
            .map_err(|error| stage_error("owner_free_catalog", error))?;
        let catalog = CallableCatalogSealOutcomeV1::seal(owner_free, 0)
            .map_err(|error| stage_error("catalog", error))?;
        let module = VerifiedResolvedCallableModuleV1::resolve(catalog)
            .map_err(|error| stage_error("module_resolution", error))?;
        Ok(Self { module })
    }

    pub fn lowering_input(&self) -> ResolvedCallableModuleLoweringInputV1<'_> {
        ResolvedCallableModuleLoweringInputV1 { source: self }
    }

    pub(crate) const fn module(&self) -> &VerifiedResolvedCallableModuleV1 {
        &self.module
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ResolvedCallableModuleLoweringInputV1<'a> {
    source: &'a VerifiedResolvedCallableProgramV1,
}

impl ResolvedCallableModuleLoweringInputV1<'_> {
    pub(crate) const fn program(&self) -> &VerifiedResolvedCallableProgramV1 {
        self.source
    }
}

fn stage_error(stage: &'static str, error: impl std::fmt::Debug) -> CanonicalLoweringErrorV1 {
    CanonicalLoweringErrorV1::SourceUnitResolution {
        detail: format!("[freeze:contract][canonical_callable_program/{stage}] {error:?}"),
    }
}
