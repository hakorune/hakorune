//! Session-private CheckedCallOut corridor model and emission facade.
//!
//! The model owns only opaque landing identities and the admitted site pair;
//! emission remains in the sibling module so the public entry stays unchanged.

use super::targets::DynamicV2OpaquePhysicalTargetV1;
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::checked_callout::{CheckedCallOutNormalShapeV1, CheckedCallOutSiteIdV1};

#[derive(Debug)]
pub(super) struct DynamicV2InstalledCallOutSitesV1 {
    i6: CheckedCallOutSiteIdV1,
    i7: CheckedCallOutSiteIdV1,
    i6_shape: CheckedCallOutNormalShapeV1,
    i7_shape: CheckedCallOutNormalShapeV1,
}

impl DynamicV2InstalledCallOutSitesV1 {
    pub(super) const fn new(
        i6: CheckedCallOutSiteIdV1,
        i7: CheckedCallOutSiteIdV1,
        i6_shape: CheckedCallOutNormalShapeV1,
        i7_shape: CheckedCallOutNormalShapeV1,
    ) -> Self {
        Self {
            i6,
            i7,
            i6_shape,
            i7_shape,
        }
    }

    pub(super) const fn i6(&self) -> CheckedCallOutSiteIdV1 {
        self.i6
    }

    pub(super) const fn i7(&self) -> CheckedCallOutSiteIdV1 {
        self.i7
    }

    pub(super) const fn i6_shape(&self) -> CheckedCallOutNormalShapeV1 {
        self.i6_shape
    }

    pub(super) const fn i7_shape(&self) -> CheckedCallOutNormalShapeV1 {
        self.i7_shape
    }
}

/// The session-private continuation produced by the two admitted CallOuts.
/// The Normal/Fault landing identities stay opaque and move with the session;
/// later leaves may not reconstruct them from logical target names.
#[derive(Debug)]
pub(super) struct DynamicV2CallOutCorridorV1 {
    i6_site: CheckedCallOutSiteIdV1,
    i6_normal: DynamicV2OpaquePhysicalTargetV1,
    i6_fault: DynamicV2OpaquePhysicalTargetV1,
    i7_site: CheckedCallOutSiteIdV1,
    i7_normal: DynamicV2OpaquePhysicalTargetV1,
    i7_fault: DynamicV2OpaquePhysicalTargetV1,
}

impl DynamicV2CallOutCorridorV1 {
    pub(super) fn new(
        i6_site: CheckedCallOutSiteIdV1,
        i6_normal: DynamicV2OpaquePhysicalTargetV1,
        i6_fault: DynamicV2OpaquePhysicalTargetV1,
        i7_site: CheckedCallOutSiteIdV1,
        i7_normal: DynamicV2OpaquePhysicalTargetV1,
        i7_fault: DynamicV2OpaquePhysicalTargetV1,
    ) -> Self {
        Self {
            i6_site,
            i6_normal,
            i6_fault,
            i7_site,
            i7_normal,
            i7_fault,
        }
    }

    pub(super) const fn i6_site(&self) -> CheckedCallOutSiteIdV1 {
        self.i6_site
    }

    pub(super) fn with_i6_normal<R>(
        &self,
        callback: impl FnOnce(&DynamicV2OpaquePhysicalTargetV1) -> R,
    ) -> R {
        callback(&self.i6_normal)
    }

    pub(super) fn with_i6_fault<R>(
        &self,
        callback: impl FnOnce(&DynamicV2OpaquePhysicalTargetV1) -> R,
    ) -> R {
        callback(&self.i6_fault)
    }

    pub(super) const fn i7_site(&self) -> CheckedCallOutSiteIdV1 {
        self.i7_site
    }

    pub(super) fn matches(&self, brand: &DynamicV2PhysicalSessionBrandV1) -> bool {
        self.i6_normal.matches(brand)
            && self.i6_fault.matches(brand)
            && self.i7_normal.matches(brand)
            && self.i7_fault.matches(brand)
    }

    pub(super) fn site_pair_matches(
        &self,
        i6_site: CheckedCallOutSiteIdV1,
        i7_site: CheckedCallOutSiteIdV1,
    ) -> bool {
        self.i6_site == i6_site && self.i7_site == i7_site
    }

    pub(super) fn with_i7_normal<R>(
        &self,
        callback: impl FnOnce(&DynamicV2OpaquePhysicalTargetV1) -> R,
    ) -> R {
        callback(&self.i7_normal)
    }

    pub(super) fn with_i7_fault<R>(
        &self,
        callback: impl FnOnce(&DynamicV2OpaquePhysicalTargetV1) -> R,
    ) -> R {
        callback(&self.i7_fault)
    }
}

pub(super) fn reject(message: impl Into<String>) -> DynamicV2I8EmitterRejectV1 {
    DynamicV2I8EmitterRejectV1::PhysicalCorridor(message.into())
}

mod emission;
pub(super) use emission::{emit, require_add, require_compare, require_const, require_read};
