//! Builder-private consuming bridge for an installed normal callable package.
//!
//! The bridge is deliberately package-only for the BridgeReady row.  It owns
//! the installed package and exposes only the scoped source/lowering views
//! needed by the selected normal root.  Direct-call inventory and target
//! loans belong to a later, separately-issued Cataloged row.

use crate::mir::normal_callable_semantic_package::{
    BuilderInstallTokenV1, InstalledNormalCallableSemanticPackageV1,
    NormalCallableSemanticPackageInstallIssueV1, NormalCallableSemanticPackagePortV1,
};
use crate::parser::{ParserNormalProgramSourceLoanRejectV1, ParserNormalProgramSourceLoanV1};

use super::CompilationContext;

/// One-shot Builder capability accepted by the package install bridge.
///
/// The constructor is Builder-private; the package bridge consumes the value
/// and seals the installed package into a non-splittable bundle.
#[must_use]
pub(in crate::mir) struct BuilderInstallConsumerV1 {
    _private: (),
}

impl BuilderInstallConsumerV1 {
    pub(in crate::mir::builder) const fn new() -> Self {
        Self { _private: () }
    }

    pub(in crate::mir) fn seal(
        self,
        installed: InstalledNormalCallableSemanticPackageV1,
        token: BuilderInstallTokenV1,
    ) -> BuilderPrivateInstalledCallablePackageBundleV1 {
        let _ = (self, token);
        BuilderPrivateInstalledCallablePackageBundleV1 { installed }
    }
}

/// Installed package plus its Builder-only lifecycle boundary.
///
/// Fields intentionally remain private and there is no `Clone`, `Copy`,
/// `into_parts`, or package getter.  The only way to use the package in the
/// selected production path is through the scoped methods below.
#[must_use]
pub(in crate::mir) struct BuilderPrivateInstalledCallablePackageBundleV1 {
    installed: InstalledNormalCallableSemanticPackageV1,
}

impl BuilderPrivateInstalledCallablePackageBundleV1 {
    pub(in crate::mir::builder) fn with_normal_program_source_loan<R>(
        &self,
        callback: impl for<'source> FnOnce(ParserNormalProgramSourceLoanV1<'source>) -> R,
    ) -> Result<R, ParserNormalProgramSourceLoanRejectV1> {
        self.installed.with_normal_program_source_loan(callback)
    }

    pub(in crate::mir::builder) fn begin_lowering(
        &self,
        context: &CompilationContext,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        self.installed.begin_lowering(context)
    }
}
