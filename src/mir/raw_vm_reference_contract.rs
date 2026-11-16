//! Compile-profile and request carriers for source-bound raw ingress.
//!
//! Source, import and callable-main policies are selected once and travel
//! with the AST to the current compiler-owned raw boundary.

use crate::ast::ASTNode;

use super::compiler::raw_public_ingress::RawPublicImportDispositionV1;
use super::compiler::raw_root_helper_coverage::RawPublicEligibilityProfileV1;
use super::compiler::raw_source_binding::RawCallableMainSelectionV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVmReferenceSourceProfileV1 {
    NarrowV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVmReferenceImportProfileV1 {
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVmReferenceCallableMainProfileV1 {
    Omitted,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct RawPublishedCompileProfileV1 {
    pub(crate) source: RawVmReferenceSourceProfileV1,
    pub(crate) imports: RawVmReferenceImportProfileV1,
    pub(crate) callable_main: RawVmReferenceCallableMainProfileV1,
}

impl RawPublishedCompileProfileV1 {
    pub(crate) const fn narrow_v1() -> Self {
        Self {
            source: RawVmReferenceSourceProfileV1::NarrowV1,
            imports: RawVmReferenceImportProfileV1::None,
            callable_main: RawVmReferenceCallableMainProfileV1::Omitted,
        }
    }

    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        RawPublicEligibilityProfileV1,
        RawPublicImportDispositionV1,
        RawCallableMainSelectionV1,
    ) {
        let eligibility = match self.source {
            RawVmReferenceSourceProfileV1::NarrowV1 => RawPublicEligibilityProfileV1::narrow_v1(),
        };
        let imports = match self.imports {
            RawVmReferenceImportProfileV1::None => RawPublicImportDispositionV1::None,
        };
        let callable_main = match self.callable_main {
            RawVmReferenceCallableMainProfileV1::Omitted => RawCallableMainSelectionV1::Omitted,
        };
        (eligibility, imports, callable_main)
    }
}

#[derive(Debug)]
pub(crate) struct RawPublishedCompileRequestV1 {
    pub(crate) ast: ASTNode,
    pub(crate) source_file: Option<Box<str>>,
    pub(crate) module_name: Box<str>,
    pub(crate) profile: RawPublishedCompileProfileV1,
}

impl RawPublishedCompileRequestV1 {
    pub(crate) fn narrow_v1(ast: ASTNode, source_file: Option<&str>) -> Self {
        Self::new(
            ast,
            source_file.map(str::to_owned).map(Box::<str>::from),
            "main",
            RawPublishedCompileProfileV1::narrow_v1(),
        )
    }

    pub(crate) fn new(
        ast: ASTNode,
        source_file: Option<Box<str>>,
        module_name: impl Into<Box<str>>,
        profile: RawPublishedCompileProfileV1,
    ) -> Self {
        Self {
            ast,
            source_file,
            module_name: module_name.into(),
            profile,
        }
    }
}
