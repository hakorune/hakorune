//! HEADERPORT0-REENTRANT-TERM0-I0-WIRING-P0: disconnected route parity.
//!
//! This product derives one wiring-surface row from the existing invocation
//! route matrix.  It does not recreate route identity, publication policy, or
//! failure law, and it owns no Builder, collector, module, or retry authority.
//! The future I0 cutover must satisfy these surfaces before any route is wired.

use super::module_invocation_route_matrix::{
    InvocationEntryV1, InvocationRootFamilyV1, InvocationRouteMatrixRowV1, InvocationRouteMatrixV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum WiringSurfaceV1 {
    MainExpansion,
    RootBody,
    ChildCapture,
    HeaderLoan,
    RootBatch,
    FinalizerLookup,
    InvocationDrain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum WiringSourceAnchorV1 {
    RawBuildModule,
    RawMainExpansion,
    RawRecursiveChildPort,
    RawConditionFnMaterializer,
    CanonicalAPlusIngress,
    BindingSsaTrivialIngress,
    BindingSsaAcyclicIngress,
    BindingSsaRecursiveIngress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum WiringOwnerV1 {
    LegacyCurrentModule,
    InvocationCollector,
    ModuleShell,
    CanonicalDraftTransaction,
    CanonicalCallableCatalog,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum WiringObservationV1 {
    Entered,
    Changed,
    HeaderLookup,
    Publication,
    FallbackForbidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(in crate::mir::builder) enum ConditionPolicyObservationV1 {
    NotApplicable,
    RawScriptRequiredOrMainOptional,
    RequiredSynthetic,
    CanonicalForbidden,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct WiringSourceSiteV1 {
    path: &'static str,
    operation: &'static str,
}

impl WiringSourceSiteV1 {
    pub(in crate::mir::builder) fn path(&self) -> &'static str {
        self.path
    }

    pub(in crate::mir::builder) fn operation(&self) -> &'static str {
        self.operation
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct WiringParityRowV1 {
    route: InvocationRouteMatrixRowV1,
    surfaces: Box<[WiringSurfaceV1]>,
    source_anchors: Box<[WiringSourceAnchorV1]>,
    source_sites: Box<[WiringSourceSiteV1]>,
    legacy_owners: Box<[WiringOwnerV1]>,
    target_owners: Box<[WiringOwnerV1]>,
    observations: Box<[WiringObservationV1]>,
    condition_policy: ConditionPolicyObservationV1,
    _seal: WiringParityRowSealV1,
}

#[derive(Debug)]
struct WiringParityRowSealV1;

impl WiringParityRowV1 {
    pub(in crate::mir::builder) fn route(&self) -> InvocationRouteMatrixRowV1 {
        self.route
    }

    pub(in crate::mir::builder) fn surfaces(&self) -> &[WiringSurfaceV1] {
        &self.surfaces
    }

    pub(in crate::mir::builder) fn requires(&self, surface: WiringSurfaceV1) -> bool {
        self.surfaces.contains(&surface)
    }

    pub(in crate::mir::builder) fn source_anchors(&self) -> &[WiringSourceAnchorV1] {
        &self.source_anchors
    }

    pub(in crate::mir::builder) fn source_sites(&self) -> &[WiringSourceSiteV1] {
        &self.source_sites
    }

    pub(in crate::mir::builder) fn legacy_owners(&self) -> &[WiringOwnerV1] {
        &self.legacy_owners
    }

    pub(in crate::mir::builder) fn target_owners(&self) -> &[WiringOwnerV1] {
        &self.target_owners
    }

    pub(in crate::mir::builder) fn observations(&self) -> &[WiringObservationV1] {
        &self.observations
    }

    pub(in crate::mir::builder) fn condition_policy(&self) -> ConditionPolicyObservationV1 {
        self.condition_policy
    }
}

#[derive(Debug)]
pub(in crate::mir::builder) struct HeaderPortWiringParityV1 {
    rows: Box<[WiringParityRowV1]>,
    _seal: HeaderPortWiringParitySealV1,
}

#[derive(Debug)]
struct HeaderPortWiringParitySealV1;

impl HeaderPortWiringParityV1 {
    pub(in crate::mir::builder) fn derive() -> Self {
        let rows = InvocationRouteMatrixV1::rows()
            .iter()
            .map(|route| WiringParityRowV1 {
                route: *route,
                surfaces: surfaces_for(*route).into_boxed_slice(),
                source_anchors: anchors_for(*route).into_boxed_slice(),
                source_sites: source_sites_for(*route).into_boxed_slice(),
                legacy_owners: legacy_owners_for(*route).into_boxed_slice(),
                target_owners: target_owners_for(*route).into_boxed_slice(),
                observations: observations_for(*route).into_boxed_slice(),
                condition_policy: condition_policy_for(*route),
                _seal: WiringParityRowSealV1,
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
        Self {
            rows,
            _seal: HeaderPortWiringParitySealV1,
        }
    }

    pub(in crate::mir::builder) fn rows(&self) -> &[WiringParityRowV1] {
        &self.rows
    }

    pub(in crate::mir::builder) fn route_count(&self) -> usize {
        self.rows.len()
    }
}

fn surfaces_for(route: InvocationRouteMatrixRowV1) -> Vec<WiringSurfaceV1> {
    match route.entry() {
        InvocationEntryV1::MainRoot => vec![
            WiringSurfaceV1::MainExpansion,
            WiringSurfaceV1::RootBody,
            WiringSurfaceV1::HeaderLoan,
            WiringSurfaceV1::RootBatch,
            WiringSurfaceV1::InvocationDrain,
            WiringSurfaceV1::FinalizerLookup,
        ],
        InvocationEntryV1::RawStaticChild | InvocationEntryV1::RawInstanceConstructorChild => {
            vec![WiringSurfaceV1::ChildCapture, WiringSurfaceV1::HeaderLoan]
        }
        InvocationEntryV1::SyntheticConditionFn => {
            vec![WiringSurfaceV1::RootBatch, WiringSurfaceV1::InvocationDrain]
        }
        InvocationEntryV1::CanonicalRoot => vec![
            WiringSurfaceV1::HeaderLoan,
            WiringSurfaceV1::FinalizerLookup,
            WiringSurfaceV1::InvocationDrain,
        ],
        InvocationEntryV1::CanonicalChild => vec![
            WiringSurfaceV1::ChildCapture,
            WiringSurfaceV1::HeaderLoan,
            WiringSurfaceV1::FinalizerLookup,
        ],
        InvocationEntryV1::CallableModuleBatch => vec![
            WiringSurfaceV1::ChildCapture,
            WiringSurfaceV1::HeaderLoan,
            WiringSurfaceV1::FinalizerLookup,
            WiringSurfaceV1::InvocationDrain,
        ],
    }
}

fn anchors_for(route: InvocationRouteMatrixRowV1) -> Vec<WiringSourceAnchorV1> {
    match route.entry() {
        InvocationEntryV1::MainRoot => vec![
            WiringSourceAnchorV1::RawBuildModule,
            WiringSourceAnchorV1::RawMainExpansion,
        ],
        InvocationEntryV1::RawStaticChild | InvocationEntryV1::RawInstanceConstructorChild => {
            vec![WiringSourceAnchorV1::RawRecursiveChildPort]
        }
        InvocationEntryV1::SyntheticConditionFn => {
            vec![WiringSourceAnchorV1::RawConditionFnMaterializer]
        }
        InvocationEntryV1::CanonicalRoot | InvocationEntryV1::CanonicalChild => {
            vec![match route.family() {
                InvocationRootFamilyV1::CanonicalAPlus => {
                    WiringSourceAnchorV1::CanonicalAPlusIngress
                }
                InvocationRootFamilyV1::BindingSsaTrivial => {
                    WiringSourceAnchorV1::BindingSsaTrivialIngress
                }
                _ => unreachable!("canonical single-function route has a sealed family"),
            }]
        }
        InvocationEntryV1::CallableModuleBatch => vec![match route.family() {
            InvocationRootFamilyV1::BindingSsaAcyclic => {
                WiringSourceAnchorV1::BindingSsaAcyclicIngress
            }
            InvocationRootFamilyV1::BindingSsaRecursive => {
                WiringSourceAnchorV1::BindingSsaRecursiveIngress
            }
            _ => unreachable!("callable module route has a sealed family"),
        }],
    }
}

fn source_sites_for(route: InvocationRouteMatrixRowV1) -> Vec<WiringSourceSiteV1> {
    match route.entry() {
        InvocationEntryV1::MainRoot => vec![
            WiringSourceSiteV1 {
                path: "src/mir/builder/module_lifecycle.rs",
                operation: "lower_root",
            },
            WiringSourceSiteV1 {
                path: "src/mir/builder/decls.rs",
                operation: "build_static_main_box",
            },
        ],
        InvocationEntryV1::RawStaticChild | InvocationEntryV1::RawInstanceConstructorChild => {
            vec![WiringSourceSiteV1 {
                path: "src/mir/builder/recursive_child_lowering.rs",
                operation: "RawInvocationChildPortV1 child terminal",
            }]
        }
        InvocationEntryV1::SyntheticConditionFn => vec![WiringSourceSiteV1 {
            path: "src/mir/builder/calls/materializer.rs",
            operation: "condition_fn",
        }],
        InvocationEntryV1::CanonicalRoot => vec![WiringSourceSiteV1 {
            path: "src/mir/compiler/mod.rs",
            operation: "compile_resolved_first_family",
        }],
        InvocationEntryV1::CanonicalChild => vec![WiringSourceSiteV1 {
            path: "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
            operation: "lower_resolved_trivial_function_draft",
        }],
        InvocationEntryV1::CallableModuleBatch => vec![WiringSourceSiteV1 {
            path: "src/mir/builder/resolved_lowering/callable_module_transaction.rs",
            operation: "build_recursive_callable_module_candidate",
        }],
    }
}

fn legacy_owners_for(route: InvocationRouteMatrixRowV1) -> Vec<WiringOwnerV1> {
    match route.family() {
        InvocationRootFamilyV1::Raw => vec![WiringOwnerV1::LegacyCurrentModule],
        InvocationRootFamilyV1::CanonicalAPlus | InvocationRootFamilyV1::BindingSsaTrivial => {
            vec![WiringOwnerV1::CanonicalDraftTransaction]
        }
        InvocationRootFamilyV1::BindingSsaAcyclic | InvocationRootFamilyV1::BindingSsaRecursive => {
            vec![
                WiringOwnerV1::CanonicalCallableCatalog,
                WiringOwnerV1::CanonicalDraftTransaction,
            ]
        }
    }
}

fn target_owners_for(route: InvocationRouteMatrixRowV1) -> Vec<WiringOwnerV1> {
    match route.family() {
        InvocationRootFamilyV1::Raw => vec![
            WiringOwnerV1::InvocationCollector,
            WiringOwnerV1::ModuleShell,
        ],
        InvocationRootFamilyV1::CanonicalAPlus | InvocationRootFamilyV1::BindingSsaTrivial => vec![
            WiringOwnerV1::CanonicalDraftTransaction,
            WiringOwnerV1::InvocationCollector,
        ],
        InvocationRootFamilyV1::BindingSsaAcyclic | InvocationRootFamilyV1::BindingSsaRecursive => {
            vec![
                WiringOwnerV1::CanonicalCallableCatalog,
                WiringOwnerV1::InvocationCollector,
            ]
        }
    }
}

fn observations_for(route: InvocationRouteMatrixRowV1) -> Vec<WiringObservationV1> {
    let mut observations = vec![WiringObservationV1::Entered, WiringObservationV1::Changed];
    if route.entry() != InvocationEntryV1::SyntheticConditionFn {
        observations.push(WiringObservationV1::HeaderLookup);
    }
    observations.push(WiringObservationV1::Publication);
    observations.push(WiringObservationV1::FallbackForbidden);
    observations
}

fn condition_policy_for(route: InvocationRouteMatrixRowV1) -> ConditionPolicyObservationV1 {
    match route.entry() {
        InvocationEntryV1::MainRoot => {
            ConditionPolicyObservationV1::RawScriptRequiredOrMainOptional
        }
        InvocationEntryV1::SyntheticConditionFn => ConditionPolicyObservationV1::RequiredSynthetic,
        _ if route.family() == InvocationRootFamilyV1::Raw => {
            ConditionPolicyObservationV1::NotApplicable
        }
        _ => ConditionPolicyObservationV1::CanonicalForbidden,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeSet;

    #[test]
    fn parity_derives_all_route_rows_without_redeclaring_route_identity() {
        let matrix = HeaderPortWiringParityV1::derive();
        assert_eq!(matrix.route_count(), 9);
        let names = matrix
            .rows()
            .iter()
            .map(|row| row.route().name())
            .collect::<BTreeSet<_>>();
        assert_eq!(names.len(), matrix.route_count());
        assert!(matrix.rows().iter().all(|row| {
            !row.surfaces().is_empty()
                && !row.source_anchors().is_empty()
                && !row.source_sites().is_empty()
                && !row.legacy_owners().is_empty()
                && !row.target_owners().is_empty()
                && !row.observations().is_empty()
        }));
    }

    #[test]
    fn main_and_condition_routes_keep_root_batch_and_drain_boundaries() {
        let matrix = HeaderPortWiringParityV1::derive();
        let main = matrix
            .rows()
            .iter()
            .find(|row| row.route().entry() == InvocationEntryV1::MainRoot)
            .unwrap();
        assert!(main.requires(WiringSurfaceV1::MainExpansion));
        assert!(main.requires(WiringSurfaceV1::RootBody));
        assert!(main.requires(WiringSurfaceV1::RootBatch));
        assert!(main.requires(WiringSurfaceV1::InvocationDrain));
        assert_eq!(
            main.source_sites()[0].path(),
            "src/mir/builder/module_lifecycle.rs"
        );
        assert_eq!(main.source_sites()[1].operation(), "build_static_main_box");

        let condition = matrix
            .rows()
            .iter()
            .find(|row| row.route().entry() == InvocationEntryV1::SyntheticConditionFn)
            .unwrap();
        assert!(condition.requires(WiringSurfaceV1::RootBatch));
        assert!(condition.requires(WiringSurfaceV1::InvocationDrain));
        assert_eq!(
            condition.condition_policy(),
            ConditionPolicyObservationV1::RequiredSynthetic
        );
    }

    #[test]
    fn canonical_and_raw_children_require_capture_without_a_fallback_surface() {
        let matrix = HeaderPortWiringParityV1::derive();
        for row in matrix.rows().iter().filter(|row| {
            matches!(
                row.route().entry(),
                InvocationEntryV1::RawStaticChild
                    | InvocationEntryV1::RawInstanceConstructorChild
                    | InvocationEntryV1::CanonicalChild
                    | InvocationEntryV1::CallableModuleBatch
            )
        }) {
            assert!(row.requires(WiringSurfaceV1::ChildCapture));
            assert!(row.requires(WiringSurfaceV1::HeaderLoan));
            assert!(row.observations().contains(&WiringObservationV1::Entered));
            assert!(row.observations().contains(&WiringObservationV1::Changed));
            assert!(row
                .observations()
                .contains(&WiringObservationV1::FallbackForbidden));
        }
    }
}
