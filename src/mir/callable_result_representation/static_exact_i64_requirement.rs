//! Borrowed projection of one exact static callable-result requirement.
//!
//! This is not the ordinary activation source gate. The selected Stage-B row
//! intentionally has no general call-result row because its required nested
//! instance argument is sealed by a separate owner.

use crate::mir::builder::{
    CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1,
    VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::resolved_semantics::SourceExprSiteV1;
use crate::mir::source_call_target::VerifiedSourceStaticCallTargetCatalogV1;

use super::{VerifiedCallableResultDispositionV1, VerifiedSameModuleCallableResultCatalogV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum StaticExactI64RequirementErrorV1 {
    TargetCatalogBrandMismatch,
    ResultCatalogBrandMismatch,
    CallerOutsideCatalog,
    SourceTargetUnavailable,
    TargetMustBeStatic,
    TargetResultUnavailable,
    GeneralCallResultAlreadyAvailable,
}

/// Small borrowed view issued after exact catalog/target/result verification.
#[derive(Debug)]
pub(crate) struct VerifiedStaticExactI64RequirementV1<'result, 'catalog> {
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &'catalog CanonicalSameModuleCallableKeyV1,
    site: SourceExprSiteV1,
    target: CanonicalSameModuleCallableKeyV1,
    required_i64_arguments: &'result [u32],
    _seal: VerifiedStaticExactI64RequirementSealV1,
}

#[derive(Debug)]
struct VerifiedStaticExactI64RequirementSealV1(());

impl VerifiedStaticExactI64RequirementSealV1 {
    const fn new() -> Self {
        Self(())
    }
}

impl<'result, 'catalog> VerifiedStaticExactI64RequirementV1<'result, 'catalog> {
    pub(crate) const fn caller(&self) -> &'catalog CanonicalSameModuleCallableKeyV1 {
        self.caller
    }

    pub(crate) const fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn target(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.target
    }

    pub(crate) const fn required_i64_arguments(&self) -> &'result [u32] {
        self.required_i64_arguments
    }

    pub(crate) fn is_branded_by(
        &self,
        declarations: &VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> bool {
        std::ptr::eq(self.declarations, declarations)
    }

    #[cfg(test)]
    pub(crate) fn with_required_i64_arguments_for_test<'override_result>(
        self,
        required_i64_arguments: &'override_result [u32],
    ) -> VerifiedStaticExactI64RequirementV1<'override_result, 'catalog> {
        VerifiedStaticExactI64RequirementV1 {
            declarations: self.declarations,
            caller: self.caller,
            site: self.site,
            target: self.target,
            required_i64_arguments,
            _seal: VerifiedStaticExactI64RequirementSealV1::new(),
        }
    }
}

pub(crate) fn project_static_exact_i64_requirement_v1<'result, 'target, 'catalog>(
    declarations: &'catalog VerifiedSameModuleCallableDeclarationCatalogV1,
    caller: &CanonicalSameModuleCallableKeyV1,
    site: &SourceExprSiteV1,
    targets: &'target VerifiedSourceStaticCallTargetCatalogV1<'catalog>,
    results: &'result VerifiedSameModuleCallableResultCatalogV1<'target, 'catalog>,
) -> Result<VerifiedStaticExactI64RequirementV1<'result, 'catalog>, StaticExactI64RequirementErrorV1>
{
    if !targets.is_branded_by(declarations) {
        return Err(StaticExactI64RequirementErrorV1::TargetCatalogBrandMismatch);
    }
    if !results.is_branded_by(declarations, targets) {
        return Err(StaticExactI64RequirementErrorV1::ResultCatalogBrandMismatch);
    }
    let canonical_caller = declarations
        .declaration(caller)
        .ok_or(StaticExactI64RequirementErrorV1::CallerOutsideCatalog)?
        .key();
    let source_target = targets
        .target(canonical_caller, site)
        .ok_or(StaticExactI64RequirementErrorV1::SourceTargetUnavailable)?;
    let target = source_target.target();
    if target.namespace() != SameModuleCallableNamespaceV1::StaticBoxMethod {
        return Err(StaticExactI64RequirementErrorV1::TargetMustBeStatic);
    }
    let Some(VerifiedCallableResultDispositionV1::ExactI64 {
        required_i64_arguments,
    }) = results.disposition(target)
    else {
        return Err(StaticExactI64RequirementErrorV1::TargetResultUnavailable);
    };
    if results.call_result(canonical_caller, site).is_some() {
        return Err(StaticExactI64RequirementErrorV1::GeneralCallResultAlreadyAvailable);
    }

    Ok(VerifiedStaticExactI64RequirementV1 {
        declarations,
        caller: canonical_caller,
        site: site.clone(),
        target: target.clone(),
        required_i64_arguments,
        _seal: VerifiedStaticExactI64RequirementSealV1::new(),
    })
}
