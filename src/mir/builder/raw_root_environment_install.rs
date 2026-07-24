//! DECLACCESS-COINSTALL0: Builder-owned Raw-root environment handoff.
//!
//! This module owns the narrow aggregate boundary between the compiler's
//! source manifest and the Builder physical owner.  It is intentionally not
//! wired to a production compiler consumer yet.  The disconnected
//! DECLACCESS-S0 slice proves source-fact projection and Builder/shell
//! co-installation without opening a public ingress.
//!
//! In particular, this module must not open another session, expose raw
//! shell/collector/ledger parts, or reopen a Builder session.  The
//! `commit` below is therefore a private one-shot handoff of the already
//! paired owners.  It performs no semantic publication until the named
//! projection primitives are available.

use super::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use super::module_declaration_facts::SealedModuleDeclarationFactsV1;
use super::module_invocation_identity::{ModuleInvocationBrandV1, ModuleInvocationFamilyV1};
use super::module_invocation_session::ModuleBuilderInvocationSessionV1;
use super::raw_root_physical::RawRootPhysicalStateV1;

/// Route vocabulary for the two Raw-root environment lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum RawRootEnvironmentInstallRouteV1 {
    Script,
    App,
}

/// Named projection handoff owned by the Builder-side aggregate.
///
/// The payload is deliberately route-specific even before declaration facts
/// are installed.  This prevents a later caller from pairing Script facts
/// with an App physical owner or from reintroducing a caller-selected route
/// flag.  The exact catalog/declaration payload is added by the manifest
/// projection sub-row; no AST or ambient module lookup belongs here.
#[derive(Debug)]
pub(in crate::mir) enum RawRootEnvironmentProjectionV1 {
    Script(RawScriptEnvironmentProjectionV1),
    App(RawAppEnvironmentProjectionV1),
}

#[derive(Debug)]
pub(in crate::mir) struct RawScriptEnvironmentProjectionV1 {
    source_file: Option<Box<str>>,
    catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    declaration_facts: SealedModuleDeclarationFactsV1,
    _seal: RawRootEnvironmentProjectionSealV1,
}

#[derive(Debug)]
pub(in crate::mir) struct RawAppEnvironmentProjectionV1 {
    source_file: Option<Box<str>>,
    catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    declaration_facts: SealedModuleDeclarationFactsV1,
    _seal: RawRootEnvironmentProjectionSealV1,
}

#[derive(Debug)]
struct RawRootEnvironmentProjectionSealV1;

impl RawRootEnvironmentProjectionV1 {
    pub(in crate::mir) fn from_parts(
        route: RawRootEnvironmentInstallRouteV1,
        source_file: Option<&str>,
        catalog: VerifiedSameModuleCallableDeclarationCatalogV1,
    ) -> Self {
        let mut user_box_decls = std::collections::BTreeMap::new();
        if matches!(route, RawRootEnvironmentInstallRouteV1::App) {
            user_box_decls.insert("Main".to_owned(), Vec::new());
        }
        let declaration_facts = SealedModuleDeclarationFactsV1::new(
            user_box_decls,
            std::collections::BTreeMap::new(),
            std::collections::BTreeMap::new(),
            std::collections::BTreeMap::new(),
        );
        let source_file = source_file.map(str::to_owned).map(Into::into);
        match route {
            RawRootEnvironmentInstallRouteV1::Script => {
                Self::Script(RawScriptEnvironmentProjectionV1 {
                    source_file,
                    catalog,
                    declaration_facts,
                    _seal: RawRootEnvironmentProjectionSealV1,
                })
            }
            RawRootEnvironmentInstallRouteV1::App => Self::App(RawAppEnvironmentProjectionV1 {
                source_file,
                catalog,
                declaration_facts,
                _seal: RawRootEnvironmentProjectionSealV1,
            }),
        }
    }

    pub(in crate::mir::builder) fn route(&self) -> RawRootEnvironmentInstallRouteV1 {
        match self {
            Self::Script(_) => RawRootEnvironmentInstallRouteV1::Script,
            Self::App(_) => RawRootEnvironmentInstallRouteV1::App,
        }
    }

    pub(in crate::mir::builder) fn source_file(&self) -> Option<&str> {
        match self {
            Self::Script(projection) => projection.source_file.as_deref(),
            Self::App(projection) => projection.source_file.as_deref(),
        }
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        RawRootEnvironmentInstallRouteV1,
        Option<Box<str>>,
        VerifiedSameModuleCallableDeclarationCatalogV1,
        SealedModuleDeclarationFactsV1,
    ) {
        match self {
            Self::Script(projection) => (
                RawRootEnvironmentInstallRouteV1::Script,
                projection.source_file,
                projection.catalog,
                projection.declaration_facts,
            ),
            Self::App(projection) => (
                RawRootEnvironmentInstallRouteV1::App,
                projection.source_file,
                projection.catalog,
                projection.declaration_facts,
            ),
        }
    }
}

impl RawScriptEnvironmentProjectionV1 {
    /// Test/disconnected constructor.  The production manifest projection
    /// will be the sole non-test producer once DECLACCESS-S0 is connected.
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(source_file: Option<&str>) -> Self {
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(
            &crate::ast::ASTNode::Program {
                statements: Vec::new(),
                span: crate::ast::Span::unknown(),
            },
        )
        .expect("empty test catalog");
        Self {
            source_file: source_file.map(str::to_owned).map(Into::into),
            catalog,
            declaration_facts: SealedModuleDeclarationFactsV1::new(
                std::collections::BTreeMap::new(),
                std::collections::BTreeMap::new(),
                std::collections::BTreeMap::new(),
                std::collections::BTreeMap::new(),
            ),
            _seal: RawRootEnvironmentProjectionSealV1,
        }
    }
}

impl RawAppEnvironmentProjectionV1 {
    /// Test/disconnected constructor.  The production manifest projection
    /// will be the sole non-test producer once DECLACCESS-S0 is connected.
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(source_file: Option<&str>) -> Self {
        let catalog = VerifiedSameModuleCallableDeclarationCatalogV1::seal_program(
            &crate::ast::ASTNode::Program {
                statements: Vec::new(),
                span: crate::ast::Span::unknown(),
            },
        )
        .expect("empty test catalog");
        let mut user_box_decls = std::collections::BTreeMap::new();
        user_box_decls.insert("Main".to_owned(), Vec::new());
        Self {
            source_file: source_file.map(str::to_owned).map(Into::into),
            catalog,
            declaration_facts: SealedModuleDeclarationFactsV1::new(
                user_box_decls,
                std::collections::BTreeMap::new(),
                std::collections::BTreeMap::new(),
                std::collections::BTreeMap::new(),
            ),
            _seal: RawRootEnvironmentProjectionSealV1,
        }
    }
}

/// One aggregate owns the candidate Builder session, physical shell/collector
/// carrier, source manifest, and route-specific projection.  No loose tuple
/// can cross the compiler/Builder boundary.
#[derive(Debug)]
pub(in crate::mir) struct RawRootEnvironmentInstallOwnerV1 {
    session: ModuleBuilderInvocationSessionV1,
    physical: RawRootPhysicalStateV1,
    projection: RawRootEnvironmentProjectionV1,
}

impl RawRootEnvironmentInstallOwnerV1 {
    pub(in crate::mir) fn new(
        session: ModuleBuilderInvocationSessionV1,
        physical: RawRootPhysicalStateV1,
        projection: RawRootEnvironmentProjectionV1,
    ) -> Self {
        Self {
            session,
            physical,
            projection,
        }
    }

    pub(in crate::mir) fn prepare(
        self,
    ) -> Result<PreparedRawRootEnvironmentInstallV1, RejectedRawRootEnvironmentInstallV1> {
        if let Err(error) = self.preflight() {
            return Err(RejectedRawRootEnvironmentInstallV1 {
                owner: self,
                error,
                _seal: RejectedRawRootEnvironmentInstallSealV1,
            });
        }
        let Self {
            session,
            physical,
            projection,
        } = self;
        Ok(PreparedRawRootEnvironmentInstallV1 {
            session,
            physical,
            projection,
            _seal: PreparedRawRootEnvironmentInstallSealV1,
        })
    }

    fn preflight(&self) -> Result<(), RawRootEnvironmentInstallErrorV1> {
        let session_brand = self.session.brand();
        let physical_brand = self.physical.brand();
        if session_brand != physical_brand {
            return Err(RawRootEnvironmentInstallErrorV1::ForeignBrand {
                session: session_brand,
                physical: physical_brand,
            });
        }
        if self.session.family() != ModuleInvocationFamilyV1::Raw {
            return Err(RawRootEnvironmentInstallErrorV1::FamilyMismatch {
                family: self.session.family(),
            });
        }
        if let Err(error) = self.session.environment_ready() {
            return Err(RawRootEnvironmentInstallErrorV1::SessionNotClosed(error));
        }
        if self.physical.ledger_brand() != physical_brand
            || self.physical.tracker_brand() != physical_brand
        {
            return Err(RawRootEnvironmentInstallErrorV1::PhysicalBrandMismatch);
        }
        if !self.physical.shell_is_empty() {
            return Err(RawRootEnvironmentInstallErrorV1::ShellAlreadyPublished {
                count: self.physical.published_function_count(),
            });
        }
        if self.physical.tracker_completed_children() != 0 {
            return Err(RawRootEnvironmentInstallErrorV1::RootTrackerNotFresh {
                completed_children: self.physical.tracker_completed_children(),
            });
        }
        if !self.physical.environment_lanes_are_vacant() {
            return Err(RawRootEnvironmentInstallErrorV1::PhysicalEnvironmentNotVacant);
        }
        let route = self.projection.route();
        if !self.session.raw_root_environment_lanes_are_vacant(route) {
            return Err(RawRootEnvironmentInstallErrorV1::BuilderEnvironmentNotVacant);
        }
        if self.projection.source_file() != self.session.config().source_file() {
            return Err(RawRootEnvironmentInstallErrorV1::SourceFileMismatch);
        }
        if self.projection.route() != route {
            return Err(RawRootEnvironmentInstallErrorV1::RouteMismatch);
        }
        Ok(())
    }
}

/// All semantic checks have completed.  The product remains non-Clone and can
/// only move once into the next owner.  Builder/shell publication is a private
/// infallible co-install; no production ingress consumes it yet.
#[derive(Debug)]
pub(in crate::mir) struct PreparedRawRootEnvironmentInstallV1 {
    session: ModuleBuilderInvocationSessionV1,
    physical: RawRootPhysicalStateV1,
    projection: RawRootEnvironmentProjectionV1,
    _seal: PreparedRawRootEnvironmentInstallSealV1,
}

#[derive(Debug)]
struct PreparedRawRootEnvironmentInstallSealV1;

/// Successful one-shot handoff.  This is intentionally a named product, not
/// `(session, physical, manifest, projection)` and not a bare Builder/module.
#[derive(Debug)]
pub(in crate::mir) struct InstalledRawRootEnvironmentV1 {
    session: ModuleBuilderInvocationSessionV1,
    physical: RawRootPhysicalStateV1,
    _seal: InstalledRawRootEnvironmentSealV1,
}

#[derive(Debug)]
struct InstalledRawRootEnvironmentSealV1;

impl InstalledRawRootEnvironmentV1 {
    pub(in crate::mir) fn catalog_installed(&self) -> bool {
        self.session
            .builder()
            .comp_ctx
            .callable_declaration_catalog()
            .is_ok()
    }

    pub(in crate::mir) fn app_main_declaration_installed(&self) -> bool {
        self.session.builder().comp_ctx.is_user_defined_box("Main")
    }

    pub(in crate::mir) fn tracker_completed_children(&self) -> usize {
        self.physical.tracker_completed_children()
    }

    pub(in crate::mir) fn session_brand(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        self.session.brand()
    }

    pub(in crate::mir) fn physical_brand(
        &self,
    ) -> crate::mir::module_invocation_identity::ModuleInvocationBrandV1 {
        self.physical.brand()
    }
}

impl PreparedRawRootEnvironmentInstallV1 {
    /// Private infallible transition.  No lookup, allocation, retry, or
    /// fallback is permitted after preparation succeeds.
    pub(in crate::mir) fn commit(self) -> InstalledRawRootEnvironmentV1 {
        let Self {
            mut session,
            physical,
            projection,
            _seal: _,
        } = self;
        let (route, source_file, catalog, declaration_facts) = projection.into_parts();
        session.install_raw_root_environment_preflighted(route, catalog);
        let physical = physical.install_environment_preflighted(declaration_facts, source_file);
        InstalledRawRootEnvironmentV1 {
            session,
            physical,
            _seal: InstalledRawRootEnvironmentSealV1,
        }
    }
}

/// Typed failure retains the complete unpublished aggregate.  There is no
/// `into_owner`, retry, resume, or partial-parts terminal.
#[derive(Debug)]
pub(in crate::mir) struct RejectedRawRootEnvironmentInstallV1 {
    owner: RawRootEnvironmentInstallOwnerV1,
    error: RawRootEnvironmentInstallErrorV1,
    _seal: RejectedRawRootEnvironmentInstallSealV1,
}

#[derive(Debug)]
struct RejectedRawRootEnvironmentInstallSealV1;

impl RejectedRawRootEnvironmentInstallV1 {
    pub(in crate::mir) fn error(&self) -> &RawRootEnvironmentInstallErrorV1 {
        &self.error
    }

    pub(in crate::mir) fn discard(self) {}
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) enum RawRootEnvironmentInstallErrorV1 {
    ForeignBrand {
        session: ModuleInvocationBrandV1,
        physical: ModuleInvocationBrandV1,
    },
    FamilyMismatch {
        family: ModuleInvocationFamilyV1,
    },
    PhysicalBrandMismatch,
    ShellAlreadyPublished {
        count: usize,
    },
    RootTrackerNotFresh {
        completed_children: usize,
    },
    RouteMismatch,
    SessionNotClosed(super::module_invocation_session::BuilderCommitReadinessErrorV1),
    PhysicalEnvironmentNotVacant,
    BuilderEnvironmentNotVacant,
    SourceFileMismatch,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::raw_root_source_facts::RawRootSourceRouteV1;
    use crate::mir::module_invocation_identity::ModuleInvocationTokenV1;
    use crate::mir::MirBuilder;
    use std::num::NonZeroU64;

    fn token() -> ModuleInvocationTokenV1 {
        ModuleInvocationTokenV1::from_issued(
            NonZeroU64::new(1).unwrap(),
            NonZeroU64::new(7).unwrap(),
            ModuleInvocationFamilyV1::Raw,
        )
    }

    fn owner(route: RawRootSourceRouteV1) -> RawRootEnvironmentInstallOwnerV1 {
        let token = token();
        let current = MirBuilder::new();
        let config =
            super::super::module_invocation_session::BuilderInvocationConfigV1::snapshot_for_raw(
                &current, None,
            );
        let source_file = config
            .source_file()
            .map(str::to_owned)
            .map(String::into_boxed_str);
        let session = ModuleBuilderInvocationSessionV1::open_for_token(&token, &current, config);
        let physical = RawRootPhysicalStateV1::open(
            &token,
            "coinstall-test".to_owned(),
            super::super::raw_expansion_receipt_ledger::RawCallableMainCompatibilityDispositionV1::NotSelected,
        )
        .unwrap();
        let (projection, _post_install_manifest) = crate::mir::compiler::raw_root_environment_manifest::RawRootPhysicalManifestV1::from_test(route).into_install_parts(source_file.as_deref());
        RawRootEnvironmentInstallOwnerV1::new(session, physical, projection)
    }

    fn dirty_builder_owner(route: RawRootSourceRouteV1) -> RawRootEnvironmentInstallOwnerV1 {
        let token = token();
        let current = MirBuilder::new();
        let config =
            super::super::module_invocation_session::BuilderInvocationConfigV1::snapshot_for_raw(
                &current, None,
            );
        let source_file = config
            .source_file()
            .map(str::to_owned)
            .map(String::into_boxed_str);
        let mut session =
            ModuleBuilderInvocationSessionV1::open_for_token(&token, &current, config);
        session
            .builder_mut()
            .comp_ctx
            .user_defined_boxes
            .insert("AlreadyInstalled".to_owned(), Vec::new());
        let physical = RawRootPhysicalStateV1::open(
            &token,
            "coinstall-dirty-test".to_owned(),
            super::super::raw_expansion_receipt_ledger::RawCallableMainCompatibilityDispositionV1::NotSelected,
        )
        .unwrap();
        let (projection, _post_install_manifest) = crate::mir::compiler::raw_root_environment_manifest::RawRootPhysicalManifestV1::from_test(route).into_install_parts(source_file.as_deref());
        RawRootEnvironmentInstallOwnerV1::new(session, physical, projection)
    }

    #[test]
    fn script_projection_installs_catalog_and_empty_declaration_lanes() {
        let installed = owner(RawRootSourceRouteV1::Script)
            .prepare()
            .unwrap()
            .commit();
        assert!(installed
            .session
            .builder()
            .comp_ctx
            .callable_declaration_catalog()
            .is_ok());
        // Script has no module declaration facts in the first slice, so its
        // physical declaration lanes remain empty by design.  The installed
        // catalog above is the observable co-install evidence for this route.
    }

    #[test]
    fn app_projection_installs_the_exact_static_main_declaration_lane() {
        let installed = owner(RawRootSourceRouteV1::App).prepare().unwrap().commit();
        assert!(installed
            .session
            .builder()
            .comp_ctx
            .is_user_defined_box("Main"));
        assert!(!installed.physical.environment_lanes_are_vacant());
    }

    #[test]
    fn dirty_builder_destination_is_rejected_before_coinstall() {
        let rejected = dirty_builder_owner(RawRootSourceRouteV1::App)
            .prepare()
            .expect_err("dirty Builder lane must reject before commit");
        assert!(matches!(
            rejected.error(),
            RawRootEnvironmentInstallErrorV1::BuilderEnvironmentNotVacant
        ));
        rejected.discard();
    }
}
