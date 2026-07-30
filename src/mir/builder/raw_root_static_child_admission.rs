//! Source-keyed admission for one raw-root static child draft.
//!
//! The source projection already issued and verified `RawSourceLocatorV1`.
//! This owner adds only the physical demand role, then consumes the locator
//! once into the unchanged legacy collector tuple. It issues no second source
//! identity, callable catalog row, or publication policy.

use super::module_draft_collector::FunctionDraftKeyV1;
use super::raw_root_child_work::{
    RawCallableMainWorkV1, RawRootStaticChildLoweringPartsV1, RawRootStaticChildWorkV1,
};
use super::raw_source_projection::RawSourceLocatorV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) enum RawRootStaticChildSourceRoleV1 {
    StaticHelper { schedule_ordinal: usize },
    CallableMain,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct RawRootStaticChildDraftAdmissionV1 {
    locator: RawSourceLocatorV1,
    role: RawRootStaticChildSourceRoleV1,
    _seal: RawRootStaticChildDraftAdmissionSealV1,
}

#[derive(Debug)]
struct RawRootStaticChildDraftAdmissionSealV1;

#[derive(Debug)]
pub(in crate::mir) struct PreparedRawRootStaticChildDraftV1 {
    admission: RawRootStaticChildDraftAdmissionV1,
    lowering: RawRootStaticChildLoweringPartsV1,
}

impl RawRootStaticChildWorkV1 {
    pub(in crate::mir) fn into_static_helper_draft(self) -> PreparedRawRootStaticChildDraftV1 {
        let (schedule_ordinal, locator, lowering) = self.into_source_parts();
        PreparedRawRootStaticChildDraftV1::new(
            locator,
            RawRootStaticChildSourceRoleV1::StaticHelper { schedule_ordinal },
            lowering,
        )
    }
}

impl RawCallableMainWorkV1 {
    pub(in crate::mir) fn into_callable_main_draft(self) -> PreparedRawRootStaticChildDraftV1 {
        let (_ordinal, locator, lowering) = self.into_inner().into_source_parts();
        PreparedRawRootStaticChildDraftV1::new(
            locator,
            RawRootStaticChildSourceRoleV1::CallableMain,
            lowering,
        )
    }
}

impl PreparedRawRootStaticChildDraftV1 {
    fn new(
        locator: RawSourceLocatorV1,
        role: RawRootStaticChildSourceRoleV1,
        lowering: RawRootStaticChildLoweringPartsV1,
    ) -> Self {
        Self {
            admission: RawRootStaticChildDraftAdmissionV1 {
                locator,
                role,
                _seal: RawRootStaticChildDraftAdmissionSealV1,
            },
            lowering,
        }
    }

    pub(in crate::mir) fn symbol(&self) -> &str {
        self.admission.locator.symbol()
    }

    pub(in crate::mir) const fn arity(&self) -> usize {
        self.admission.locator.arity()
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        RawRootStaticChildDraftAdmissionV1,
        RawRootStaticChildLoweringPartsV1,
    ) {
        (self.admission, self.lowering)
    }
}

impl RawRootStaticChildDraftAdmissionV1 {
    pub(in crate::mir::builder) fn into_collector_parts(
        self,
    ) -> (FunctionDraftKeyV1, String, usize) {
        let Self {
            locator,
            role: _,
            _seal: _,
        } = self;
        let symbol = locator.symbol().to_owned();
        let arity = locator.arity();
        (
            FunctionDraftKeyV1::LegacySymbol(symbol.clone()),
            symbol,
            arity,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn admission(role: RawRootStaticChildSourceRoleV1) -> RawRootStaticChildDraftAdmissionV1 {
        RawRootStaticChildDraftAdmissionV1 {
            locator: RawSourceLocatorV1::for_test(3, "Main", "helper", "Main.helper/2", 2),
            role,
            _seal: RawRootStaticChildDraftAdmissionSealV1,
        }
    }

    #[test]
    fn roles_keep_the_same_verified_legacy_physical_projection() {
        for role in [
            RawRootStaticChildSourceRoleV1::StaticHelper {
                schedule_ordinal: 4,
            },
            RawRootStaticChildSourceRoleV1::CallableMain,
        ] {
            let (key, symbol, arity) = admission(role).into_collector_parts();
            assert_eq!(
                key,
                FunctionDraftKeyV1::LegacySymbol("Main.helper/2".into())
            );
            assert_eq!(symbol, "Main.helper/2");
            assert_eq!(arity, 2);
        }
    }
}
