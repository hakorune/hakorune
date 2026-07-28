//! Neutral contracts shared by the explicit Raw VM-reference runner and MIR.
//!
//! The runner selects these profiles once.  Compiler and execution owners
//! consume the resulting requests; neither layer reconstructs NarrowV1 policy.

use crate::ast::ASTNode;

use super::compiler::raw_public_ingress::RawPublicImportDispositionV1;
use super::compiler::raw_root_helper_coverage::RawPublicEligibilityProfileV1;
use super::compiler::raw_source_binding::RawCallableMainSelectionV1;
use super::compiler::source_entry_result::{CanonicalProcessExitV1, ProcessExitProfileV1};

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawVmReferenceExecutionProfileV1 {
    CanonicalV1,
}

impl RawVmReferenceExecutionProfileV1 {
    pub(in crate::mir) const fn process_profile(self) -> ProcessExitProfileV1 {
        match self {
            Self::CanonicalV1 => ProcessExitProfileV1::Canonical(CanonicalProcessExitV1::V1),
        }
    }
}

/// The closed downstream profile shared by supported Raw consumers.
///
/// This owner keeps compile and execution policy paired until a source owner
/// has produced one exact AST. Consumers may issue an invocation from it, but
/// may not reconstruct NarrowV1, entry, VM freshness, or process policy.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RawVmReferenceSupportProfileV1 {
    compile: RawPublishedCompileProfileV1,
    execution: RawVmReferenceExecutionProfileV1,
}

impl RawVmReferenceSupportProfileV1 {
    pub(crate) const fn canonical_v1() -> Self {
        Self {
            compile: RawPublishedCompileProfileV1::narrow_v1(),
            execution: RawVmReferenceExecutionProfileV1::CanonicalV1,
        }
    }

    pub(crate) fn into_invocation(
        self,
        ast: ASTNode,
        source_file: Option<Box<str>>,
    ) -> RawVmReferenceInvocationV1 {
        RawVmReferenceInvocationV1::new(
            RawPublishedCompileRequestV1::new(ast, source_file, "main", self.compile),
            self.execution,
        )
    }
}

#[derive(Debug)]
pub(crate) struct RawVmReferenceInvocationV1 {
    pub(crate) compile: RawPublishedCompileRequestV1,
    pub(crate) execution: RawVmReferenceExecutionProfileV1,
}

impl RawVmReferenceInvocationV1 {
    pub(crate) fn narrow_v1(ast: ASTNode, source_file: Option<&str>) -> Self {
        RawVmReferenceSupportProfileV1::canonical_v1().into_invocation(
            ast,
            source_file.map(str::to_owned).map(String::into_boxed_str),
        )
    }

    pub(crate) fn new(
        compile: RawPublishedCompileRequestV1,
        execution: RawVmReferenceExecutionProfileV1,
    ) -> Self {
        Self { compile, execution }
    }
}
