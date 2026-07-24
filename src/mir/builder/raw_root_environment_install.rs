//! DECLACCESS-COINSTALL0: Builder-owned Raw-root environment handoff.
//!
//! This module owns the narrow aggregate boundary between the compiler's
//! source manifest and the Builder physical owner.  It is intentionally not
//! wired to a compiler consumer yet.  The source-facts projection and the
//! actual Builder/shell installation primitives land in the following
//! DECLACCESS row; this row only fixes the ownership and rejection algebra.
//!
//! In particular, this module must not open another session, expose raw
//! shell/collector/ledger parts, or call `prepare_module_session`.  The
//! `commit` below is therefore a private one-shot handoff of the already
//! paired owners.  It performs no semantic publication until the named
//! projection primitives are available.

use crate::mir::compiler::raw_root_environment_manifest::RawRootPhysicalManifestV1;

use super::module_invocation_identity::{
    ModuleInvocationBrandV1, ModuleInvocationFamilyV1,
};
use super::module_invocation_session::ModuleBuilderInvocationSessionV1;
use super::raw_root_physical::RawRootPhysicalStateV1;

/// Route vocabulary for the two Raw-root environment lanes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawRootEnvironmentInstallRouteV1 {
    Script,
    App,
}

/// Named projection handoff owned by the Builder-side aggregate.
///
/// The payload is deliberately route-specific even before declaration facts
/// are installed.  This prevents a later caller from pairing Script facts
/// with an App physical owner or from reintroducing a caller-selected route
/// flag.  The exact catalog/declaration payload is added by the manifest
/// projection sub-row; no AST or `current_module` lookup belongs here.
#[derive(Debug)]
pub(in crate::mir::builder) enum RawRootEnvironmentProjectionV1 {
    Script(RawScriptEnvironmentProjectionV1),
    App(RawAppEnvironmentProjectionV1),
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawScriptEnvironmentProjectionV1 {
    source_file: Option<Box<str>>,
    _seal: RawRootEnvironmentProjectionSealV1,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawAppEnvironmentProjectionV1 {
    source_file: Option<Box<str>>,
    _seal: RawRootEnvironmentProjectionSealV1,
}

#[derive(Debug)]
struct RawRootEnvironmentProjectionSealV1;

impl RawRootEnvironmentProjectionV1 {
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
}

impl RawScriptEnvironmentProjectionV1 {
    /// Test/disconnected constructor.  The production manifest projection
    /// will be the sole non-test producer once DECLACCESS-S0 is connected.
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(source_file: Option<&str>) -> Self {
        Self {
            source_file: source_file.map(str::to_owned).map(Into::into),
            _seal: RawRootEnvironmentProjectionSealV1,
        }
    }
}

impl RawAppEnvironmentProjectionV1 {
    /// Test/disconnected constructor.  The production manifest projection
    /// will be the sole non-test producer once DECLACCESS-S0 is connected.
    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test(source_file: Option<&str>) -> Self {
        Self {
            source_file: source_file.map(str::to_owned).map(Into::into),
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
    manifest: RawRootPhysicalManifestV1,
    projection: RawRootEnvironmentProjectionV1,
}

impl RawRootEnvironmentInstallOwnerV1 {
    pub(in crate::mir) fn new(
        session: ModuleBuilderInvocationSessionV1,
        physical: RawRootPhysicalStateV1,
        manifest: RawRootPhysicalManifestV1,
        projection: RawRootEnvironmentProjectionV1,
    ) -> Self {
        Self {
            session,
            physical,
            manifest,
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
            manifest,
            projection,
        } = self;
        Ok(PreparedRawRootEnvironmentInstallV1 {
            session,
            physical,
            manifest,
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
        if self.projection.route() != route_from_manifest(&self.manifest) {
            return Err(RawRootEnvironmentInstallErrorV1::RouteMismatch);
        }
        Ok(())
    }
}

/// All semantic checks have completed.  The product remains non-Clone and can
/// only move once into the next owner.  Builder/shell publication is kept out
/// of this disconnected row; the following DECLACCESS row will add the
/// preflighted projection installation primitive.
#[derive(Debug)]
pub(in crate::mir) struct PreparedRawRootEnvironmentInstallV1 {
    session: ModuleBuilderInvocationSessionV1,
    physical: RawRootPhysicalStateV1,
    manifest: RawRootPhysicalManifestV1,
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
    manifest: RawRootPhysicalManifestV1,
    projection: RawRootEnvironmentProjectionV1,
    _seal: InstalledRawRootEnvironmentSealV1,
}

#[derive(Debug)]
struct InstalledRawRootEnvironmentSealV1;

impl PreparedRawRootEnvironmentInstallV1 {
    /// Private infallible transition.  No lookup, allocation, retry, or
    /// fallback is permitted after preparation succeeds.
    pub(in crate::mir::builder) fn commit(self) -> InstalledRawRootEnvironmentV1 {
        let Self {
            session,
            physical,
            manifest,
            projection,
            _seal: _,
        } = self;
        InstalledRawRootEnvironmentV1 {
            session,
            physical,
            manifest,
            projection,
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
}

fn route_from_manifest(
    manifest: &RawRootPhysicalManifestV1,
) -> RawRootEnvironmentInstallRouteV1 {
    match manifest.facts().route() {
        crate::mir::compiler::raw_root_source_facts::RawRootSourceRouteV1::Script => {
            RawRootEnvironmentInstallRouteV1::Script
        }
        crate::mir::compiler::raw_root_source_facts::RawRootSourceRouteV1::App => {
            RawRootEnvironmentInstallRouteV1::App
        }
    }
}

