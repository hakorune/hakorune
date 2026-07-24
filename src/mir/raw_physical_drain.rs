//! DRAIN0 neutral vocabulary for the Raw route.
//!
//! This module carries only physical identity and provenance.  It does not
//! expose compiler source authority or Builder-owned collector keys.

use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawPhysicalDrainRouteV1 {
    Script,
    App,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawPhysicalCallableMainDispositionV1 {
    NotSelected,
    Selected,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RawPhysicalDrainKeyV1 {
    RootMain,
    RequiredCondition,
    LegacySymbol(Box<str>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawPhysicalDrainRoleV1 {
    StaticHelper,
    CallableMainCompatibility,
    RootMain,
    RequiredCondition,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum RawPhysicalDrainPolicyV1 {
    LegacyReplaceWholePair,
    CanonicalRejectDuplicate,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum RawPhysicalReceiptProvenanceV1 {
    Inserted,
    ReplacedWholePair {
        previous_key: RawPhysicalDrainKeyV1,
        previous_symbol: Box<str>,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RawPhysicalDrainRowV1 {
    ordinal: u32,
    role: RawPhysicalDrainRoleV1,
    key: RawPhysicalDrainKeyV1,
    symbol: Box<str>,
    arity: usize,
    policy: RawPhysicalDrainPolicyV1,
    provenance: RawPhysicalReceiptProvenanceV1,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct RawPhysicalDrainManifestV1 {
    brand: ModuleInvocationBrandV1,
    route: RawPhysicalDrainRouteV1,
    rows: Box<[RawPhysicalDrainRowV1]>,
    callable_main: RawPhysicalCallableMainDispositionV1,
    _seal: RawPhysicalDrainManifestSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct RawPhysicalDrainManifestSealV1;

impl RawPhysicalDrainManifestV1 {
    pub(crate) fn new(
        brand: ModuleInvocationBrandV1,
        route: RawPhysicalDrainRouteV1,
        rows: Box<[RawPhysicalDrainRowV1]>,
        callable_main: RawPhysicalCallableMainDispositionV1,
    ) -> Self {
        Self {
            brand,
            route,
            rows,
            callable_main,
            _seal: RawPhysicalDrainManifestSealV1,
        }
    }

    pub(crate) const fn brand(&self) -> ModuleInvocationBrandV1 {
        self.brand
    }

    pub(crate) const fn route(&self) -> RawPhysicalDrainRouteV1 {
        self.route
    }

    pub(crate) const fn callable_main(&self) -> RawPhysicalCallableMainDispositionV1 {
        self.callable_main
    }

    pub(crate) fn rows(&self) -> &[RawPhysicalDrainRowV1] {
        &self.rows
    }
}

impl RawPhysicalDrainRowV1 {
    pub(crate) fn new(
        ordinal: u32,
        role: RawPhysicalDrainRoleV1,
        key: RawPhysicalDrainKeyV1,
        symbol: Box<str>,
        arity: usize,
        policy: RawPhysicalDrainPolicyV1,
        provenance: RawPhysicalReceiptProvenanceV1,
    ) -> Self {
        Self {
            ordinal,
            role,
            key,
            symbol,
            arity,
            policy,
            provenance,
        }
    }

    pub(crate) const fn ordinal(&self) -> u32 {
        self.ordinal
    }

    pub(crate) const fn role(&self) -> RawPhysicalDrainRoleV1 {
        self.role
    }

    pub(crate) fn key(&self) -> &RawPhysicalDrainKeyV1 {
        &self.key
    }

    pub(crate) fn symbol(&self) -> &str {
        &self.symbol
    }

    pub(crate) const fn arity(&self) -> usize {
        self.arity
    }

    pub(crate) const fn policy(&self) -> RawPhysicalDrainPolicyV1 {
        self.policy
    }

    pub(crate) fn provenance(&self) -> &RawPhysicalReceiptProvenanceV1 {
        &self.provenance
    }
}
