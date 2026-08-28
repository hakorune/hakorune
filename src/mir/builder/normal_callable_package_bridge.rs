//! Builder-private consuming bridge for an installed normal callable package.
//!
//! The bridge is deliberately package-only for the BridgeReady row.  It owns
//! the installed package and exposes only the scoped source/lowering views
//! needed by the selected normal root.  Direct-call inventory and target
//! loans belong to a later, separately-issued Cataloged row.

use std::cell::Cell;

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
    pub(in crate::mir::builder) fn into_lowering_scope(
        self,
    ) -> BuilderPrivateCallableLoweringScopeV1 {
        BuilderPrivateCallableLoweringScopeV1 {
            installed: self.installed,
            lowering_started: Cell::new(false),
        }
    }
}

/// Builder-private one-shot scope for the installed package.
///
/// The scope owns the installed package and keeps both source and package
/// ports behind callbacks. No package-only reference or port can escape the
/// selected root lifecycle, and a failed/finished lowering cannot be retried.
#[must_use]
pub(in crate::mir) struct BuilderPrivateCallableLoweringScopeV1 {
    installed: InstalledNormalCallableSemanticPackageV1,
    lowering_started: Cell<bool>,
}

impl BuilderPrivateCallableLoweringScopeV1 {
    pub(in crate::mir::builder) fn with_normal_program_source_loan<R>(
        &self,
        callback: impl for<'source> FnOnce(ParserNormalProgramSourceLoanV1<'source>) -> R,
    ) -> Result<R, ParserNormalProgramSourceLoanRejectV1> {
        self.installed.with_normal_program_source_loan(callback)
    }

    pub(in crate::mir::builder) fn open_lowering_once(
        &self,
        context: &CompilationContext,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        if self.lowering_started.replace(true) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::LoweringAlreadyStarted);
        }
        self.installed.open_lowering_port(context)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
    use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
    use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

    fn source() -> VerifiedFinalCallableProgramSourceV1 {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            "static box Scan { run(value) { return value } }",
            ParserBuildConfig::default(),
        )
        .expect("normal callable source");
        crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("exact callable transform");
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                transformed
            else {
                panic!("fixture must remain source-backed")
            };
            source
        })
    }

    #[test]
    fn lowering_scope_rejects_a_second_port_opening() {
        let mut resolver = FunctionSemanticResolverSessionV1::new(101).expect("resolver");
        let package = issue_normal_callable_semantic_package_v1(&mut resolver, source())
            .expect("semantic package");
        let mut context = CompilationContext::new();
        let scope = package
            .with_normal_callable_install_once(&mut context, BuilderInstallConsumerV1::new())
            .expect("package install")
            .into_lowering_scope();

        let first = scope
            .open_lowering_once(&context)
            .expect("first package port");
        drop(first);
        assert!(matches!(
            scope.open_lowering_once(&context),
            Err(NormalCallableSemanticPackageInstallIssueV1::LoweringAlreadyStarted)
        ));
    }
}
