//! Builder-private consuming bridge for an installed normal callable package.
//!
//! The bridge owns the installed package and exposes only the scoped
//! source/lowering views needed by the selected normal root.  The selected
//! direct-call inventory is moved out exactly once and remains coupled to
//! that lowering scope until the raw consumer finishes it.

use std::cell::Cell;

use crate::mir::normal_callable_semantic_package::{
    BuilderInstallTokenV1, DeclaredInstanceCallLocatorViewV1, DirectCallDispositionLoansV1,
    InstalledNormalCallableSemanticPackageV1, MapLifecycleConsumerCapabilityV1,
    MapLifecycleOperationV1, NormalCallableSemanticPackageInstallIssueV1,
    NormalCallableSemanticPackagePortV1,
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

    /// The Map lifecycle operations the selected lowering lanes execute
    /// today: literal create, scalar/transferred entry stores (including
    /// overwrite release of a displaced entry), self-rooted handle
    /// borrows on in-owner maps (stored through the existing
    /// `InstallValue` i64 lane — ordinary-role formals are physically
    /// i64 and the payload end is a no-op, so the map never owns the
    /// handle), local-binding and return-boundary handoff, and the
    /// Normal/Fault cleanup chains. Opaque entry classes (string,
    /// `[...]`, `%{...}` child values), live map-local and kind-less
    /// local borrows, borrows on escaping maps (`BorrowedEntryEscape`),
    /// and slot/argument/containment handoffs stay unimplemented —
    /// obligations demanding them keep failing admission at
    /// `verify_map_lifecycle_undertaking`. This declaration is the
    /// consumer's own claim; it is not inferred from registry presence.
    pub(in crate::mir) fn map_lifecycle_capability() -> MapLifecycleConsumerCapabilityV1 {
        MapLifecycleConsumerCapabilityV1::covering([
            MapLifecycleOperationV1::ValueCreate,
            MapLifecycleOperationV1::EntryStore(
                crate::mir::resolved_semantics::home_new_prefix::MapEntryStoreClassV1::Scalar,
            ),
            MapLifecycleOperationV1::EntryStore(
                crate::mir::resolved_semantics::home_new_prefix::MapEntryStoreClassV1::Transferred,
            ),
            MapLifecycleOperationV1::EntryDisplace,
            MapLifecycleOperationV1::OwnershipTransfer,
            MapLifecycleOperationV1::OwnershipShare(
                crate::mir::resolved_semantics::home_new_prefix::MapEntryBorrowKindV1::Handle,
            ),
            MapLifecycleOperationV1::ReturnHandoff,
            MapLifecycleOperationV1::NormalCleanup,
            MapLifecycleOperationV1::FaultCleanup,
        ])
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
            direct_call_loans: None,
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
    direct_call_loans: Option<DirectCallDispositionLoansV1>,
    lowering_started: Cell<bool>,
}

impl BuilderPrivateCallableLoweringScopeV1 {
    pub(in crate::mir::builder) fn finish(
        self,
    ) -> Result<
        crate::mir::normal_callable_semantic_package::VerifiedCallableResultContractCohortV1,
        NormalCallableSemanticPackageInstallIssueV1,
    > {
        self.installed.finish_lowering()
    }

    pub(in crate::mir::builder) fn with_declared_instance_call_locators<R>(
        &self,
        callback: impl for<'view> FnOnce(DeclaredInstanceCallLocatorViewV1<'view>) -> R,
    ) -> R {
        self.installed
            .with_declared_instance_call_locators(callback)
    }

    pub(in crate::mir::builder) fn with_normal_program_source_loan<R>(
        &self,
        callback: impl for<'source> FnOnce(ParserNormalProgramSourceLoanV1<'source>) -> R,
    ) -> Result<R, ParserNormalProgramSourceLoanRejectV1> {
        self.installed.with_normal_program_source_loan(callback)
    }

    pub(in crate::mir::builder) fn open_lowering_once(
        &mut self,
        context: &CompilationContext,
    ) -> Result<NormalCallableSemanticPackagePortV1<'_>, NormalCallableSemanticPackageInstallIssueV1>
    {
        if self.lowering_started.replace(true) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::LoweringAlreadyStarted);
        }
        if self.direct_call_loans.is_none() {
            self.direct_call_loans = self.installed.take_direct_call_loans();
        }
        self.installed
            .open_lowering_port(context, self.direct_call_loans.take())
    }

    pub(in crate::mir::builder) fn with_lowering_once_and_program_source_loan<R>(
        &mut self,
        callback: impl for<'package, 'source> FnOnce(
            NormalCallableSemanticPackagePortV1<'package>,
            ParserNormalProgramSourceLoanV1<'source>,
        ) -> R,
    ) -> Result<R, NormalCallableSemanticPackageInstallIssueV1> {
        if self.lowering_started.replace(true) {
            return Err(NormalCallableSemanticPackageInstallIssueV1::LoweringAlreadyStarted);
        }
        if self.direct_call_loans.is_none() {
            self.direct_call_loans = self.installed.take_direct_call_loans();
        }
        let package_port = self
            .installed
            .open_lowering_port_after_install(self.direct_call_loans.take())?;
        self.installed
            .with_normal_program_source_loan(|source| callback(package_port, source))
            .map_err(|_| NormalCallableSemanticPackageInstallIssueV1::BatchLoan)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
    use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;
    use crate::parser::{NyashParser, ParserBuildConfig, VerifiedFinalCallableProgramSourceV1};

    pub(super) fn source() -> VerifiedFinalCallableProgramSourceV1 {
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
        let mut scope = package
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

    #[test]
    fn installed_scope_lends_declared_instance_locator_without_dropping_it() {
        let parsed = NyashParser::parse_normal_callable_program_with_build_config(
            "box Counter { call() { return me.value() } value() { return 1 } }",
            ParserBuildConfig::default(),
        )
        .expect("declared-instance source");
        let source = crate::test_support::with_env_var("NYASH_MACRO_DISABLE", "1", || {
            let transformed = crate::r#macro::transform_normal_callable_program_v1(parsed)
                .expect("source-backed transform");
            let crate::r#macro::NormalCallableTransformOutcomeV1::SourceBacked(source) =
                transformed
            else {
                panic!("fixture must remain source-backed")
            };
            source
        });
        let mut resolver = FunctionSemanticResolverSessionV1::new(1_072).expect("resolver");
        let package = crate::mir::normal_callable_semantic_package::
            issue_normal_callable_semantic_package_v1(&mut resolver, source)
            .expect("semantic package");
        let mut context = CompilationContext::new();
        let mut scope = package
            .with_normal_callable_install_once(&mut context, BuilderInstallConsumerV1::new())
            .expect("package install")
            .into_lowering_scope();
        let mut port = scope.open_lowering_once(&context).expect("package port");
        let count = port.with_declared_instance_call_locators(|view| view.row_count());
        assert_eq!(count, 1);
    }

    #[test]
    fn installed_scope_lends_explicit_no_root_locator_state() {
        let package = issue_normal_callable_semantic_package_v1(
            &mut FunctionSemanticResolverSessionV1::new(1_073).expect("resolver"),
            source(),
        )
        .expect("semantic package");
        let mut context = CompilationContext::new();
        let mut scope = package
            .with_normal_callable_install_once(&mut context, BuilderInstallConsumerV1::new())
            .expect("package install")
            .into_lowering_scope();
        let mut port = scope.open_lowering_once(&context).expect("package port");
        let (is_no_root, count) =
            port.with_declared_instance_call_locators(|view| (view.is_no_root(), view.row_count()));
        assert!(is_no_root);
        assert_eq!(count, 0);
    }
}

#[cfg(test)]
#[path = "normal_callable_package_finish_tests.rs"]
mod finish_tests;
