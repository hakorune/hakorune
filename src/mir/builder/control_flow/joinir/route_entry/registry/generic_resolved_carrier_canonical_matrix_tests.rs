//! D4-S3-S1 source-backed Generic observation matrix.
//!
//! This witness keeps the matrix below selection.  It borrows one resolver
//! owned source window, observes only the facts owner, and records typed
//! absence separately from a real `Neither` Generic presence.  No selector,
//! winner, Recipe, Builder, MIR, retry, or fallback authority is introduced.

use crate::ast::ASTNode;
use crate::mir::builder::control_flow::plan::facts::try_build_loop_facts;
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1,
};
use crate::mir::shared_loop_source_window::{
    issue_shared_loop_source_window_v1, SharedLoopSourceWindowRejectV1, SharedRawLoopViewV1,
    SharedResolvedLoopViewV1, VerifiedSharedLoopSourceWindowV1,
};
use crate::parser::NyashParser;

const BOTH_SOURCE: &str = r#"
function generic_matrix_both(j, m, n) {
    loop(j + m < n) {
        j = j + 1
    }
    return j
}
"#;

const V1_ONLY_SOURCE: &str = r#"
function generic_matrix_v1_only(i) {
    loop(i < 3) {
        local tmp = 0
        i = i + 1
    }
    return i
}
"#;

const NO_STANDALONE_SOURCE: &str = r#"
function generic_matrix_no_standalone(i, j) {
    loop(i < 3) {
        j += 1
        i = i + 1
    }
    return j
}
"#;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

impl MatrixModeV1 {
    fn config(self) -> crate::test_support::ScopedTestConfig {
        crate::test_support::ScopedTestConfig::apply(&[
            (
                "HAKO_JOINIR_STRICT",
                if matches!(self, Self::Release) {
                    None
                } else {
                    Some("1")
                },
            ),
            (
                "HAKO_JOINIR_PLANNER_REQUIRED",
                if matches!(self, Self::StrictPlannerRequired) {
                    Some("1")
                } else {
                    None
                },
            ),
            ("NYASH_JOINIR_STRICT", None),
            ("NYASH_SYNTAX_SUGAR_LEVEL", Some("basic")),
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixFixtureV1 {
    Both,
    V1Only,
    NoStandalone,
}

impl MatrixFixtureV1 {
    const ALL: [Self; 3] = [Self::Both, Self::V1Only, Self::NoStandalone];

    const fn source(self) -> &'static str {
        match self {
            Self::Both => BOTH_SOURCE,
            Self::V1Only => V1_ONLY_SOURCE,
            Self::NoStandalone => NO_STANDALONE_SOURCE,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GenericPresenceV1 {
    V0Only,
    V1Only,
    Both,
    Neither,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixDispositionV1 {
    UnresolvedV0Only,
    UnresolvedV1Only,
    UnresolvedOverlap,
    UnresolvedNeither,
    NoStandaloneRow,
    NotYetObserved,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MatrixFactsStatusV1 {
    Available,
    NoStandaloneRow,
    Frozen(&'static str),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MatrixSourceIdentityV1 {
    owner: FunctionOwnerIdV1,
    origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
}

impl MatrixSourceIdentityV1 {
    fn from_views(raw: &SharedRawLoopViewV1<'_>, resolved: &SharedResolvedLoopViewV1<'_>) -> Self {
        assert_eq!(raw.owner(), resolved.owner());
        assert_eq!(raw.site(), resolved.site());
        assert!(raw.frame().matches(resolved.frame()));
        Self {
            owner: raw.owner(),
            origin: resolved.function_origin(),
            source_kind: resolved.source_kind(),
            site: raw.site().clone(),
            frame: raw.frame().clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MatrixCellV1 {
    presence: GenericPresenceV1,
    disposition: MatrixDispositionV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct MatrixObservationV1 {
    fixture: MatrixFixtureV1,
    mode: MatrixModeV1,
    identity: MatrixSourceIdentityV1,
    status: MatrixFactsStatusV1,
    v0_present: bool,
    v1_present: bool,
    carrier:
        Option<crate::mir::builder::control_flow::plan::facts::GenericLoopCarrierObservationV1>,
    cells: Box<[MatrixCellV1]>,
}

#[derive(Debug)]
struct MatrixObservationSetV1<'a> {
    fixture: MatrixFixtureV1,
    mode: MatrixModeV1,
    receipt: VerifiedSharedLoopSourceWindowV1<'a>,
}

impl<'a> MatrixObservationSetV1<'a> {
    fn issue(
        fixture: MatrixFixtureV1,
        mode: MatrixModeV1,
        unit: &'a VerifiedResolvedSourceUnitV1,
        loop_stmt: &LocatedStmtV1<'a>,
    ) -> Result<Self, SharedLoopSourceWindowRejectV1> {
        Ok(Self {
            fixture,
            mode,
            receipt: issue_shared_loop_source_window_v1(unit, loop_stmt)?,
        })
    }

    fn observe(self) -> MatrixObservationV1 {
        let Self {
            fixture,
            mode,
            receipt,
        } = self;
        receipt.with_views(|raw, resolved| {
            let identity = MatrixSourceIdentityV1::from_views(&raw, &resolved);
            let facts = try_build_loop_facts(raw.condition(), raw.body());
            let (status, v0_present, v1_present, carrier) = match facts {
                Ok(Some(facts)) => {
                    let v0 = facts.generic_loop_v0().is_some();
                    let carrier = facts
                        .generic_loop_v1()
                        .map(|v1| v1.carrier_observation.clone());
                    (
                        MatrixFactsStatusV1::Available,
                        v0,
                        carrier.is_some(),
                        carrier,
                    )
                }
                Ok(None) => (MatrixFactsStatusV1::NoStandaloneRow, false, false, None),
                Err(freeze) => (MatrixFactsStatusV1::Frozen(freeze.tag), false, false, None),
            };
            let actual = match (v0_present, v1_present) {
                (true, false) => GenericPresenceV1::V0Only,
                (false, true) => GenericPresenceV1::V1Only,
                (true, true) => GenericPresenceV1::Both,
                (false, false) => GenericPresenceV1::Neither,
            };
            let actual_disposition = match status {
                MatrixFactsStatusV1::NoStandaloneRow => MatrixDispositionV1::NoStandaloneRow,
                MatrixFactsStatusV1::Available => match actual {
                    GenericPresenceV1::V0Only => MatrixDispositionV1::UnresolvedV0Only,
                    GenericPresenceV1::V1Only => MatrixDispositionV1::UnresolvedV1Only,
                    GenericPresenceV1::Both => MatrixDispositionV1::UnresolvedOverlap,
                    GenericPresenceV1::Neither => MatrixDispositionV1::UnresolvedNeither,
                },
                MatrixFactsStatusV1::Frozen(_) => MatrixDispositionV1::NotYetObserved,
            };
            let cells = [
                GenericPresenceV1::V0Only,
                GenericPresenceV1::V1Only,
                GenericPresenceV1::Both,
                GenericPresenceV1::Neither,
            ]
            .into_iter()
            .map(|presence| MatrixCellV1 {
                disposition: if presence == actual {
                    actual_disposition
                } else {
                    MatrixDispositionV1::NotYetObserved
                },
                presence,
            })
            .collect();
            MatrixObservationV1 {
                fixture,
                mode,
                identity,
                status,
                v0_present,
                v1_present,
                carrier,
                cells,
            }
        })
    }
}

fn parse_unit(source: &str) -> VerifiedResolvedSourceUnitV1 {
    let root = NyashParser::parse_from_string(source).expect("matrix source parses");
    let ASTNode::Program { statements, .. } = root else {
        panic!("matrix source must be a Program")
    };
    let function = statements
        .into_iter()
        .find(|node| matches!(node, ASTNode::FunctionDeclaration { .. }))
        .expect("matrix source has one function");
    VerifiedResolvedSourceUnitV1::resolve_function(function).expect("matrix source resolves")
}

fn body_stmt<'a>(unit: &'a VerifiedResolvedSourceUnitV1, index: usize) -> LocatedStmtV1<'a> {
    let input = unit.root_function_input().expect("root function input");
    let body = input.source().root_body().expect("root body");
    input
        .source()
        .body_stmt(&body, index)
        .expect("matrix body statement")
}

fn expected_presence(fixture: MatrixFixtureV1, mode: MatrixModeV1) -> GenericPresenceV1 {
    match fixture {
        MatrixFixtureV1::Both if matches!(mode, MatrixModeV1::StrictPlannerRequired) => {
            GenericPresenceV1::V1Only
        }
        MatrixFixtureV1::Both => GenericPresenceV1::Both,
        MatrixFixtureV1::V1Only => GenericPresenceV1::V1Only,
        MatrixFixtureV1::NoStandalone => GenericPresenceV1::Neither,
    }
}

#[test]
fn d4_s3_s1_seals_source_backed_presence_matrix_without_selection() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let modes = [
        MatrixModeV1::Release,
        MatrixModeV1::Strict,
        MatrixModeV1::StrictPlannerRequired,
    ];
    let mut observations = Vec::new();
    for fixture in MatrixFixtureV1::ALL {
        for mode in modes {
            let _config = mode.config();
            let unit = parse_unit(fixture.source());
            let loop_stmt = body_stmt(&unit, 0);
            let set = MatrixObservationSetV1::issue(fixture, mode, &unit, &loop_stmt)
                .expect("resolver-owned source window");
            observations.push(set.observe());
        }
    }
    assert_eq!(observations.len(), 9);
    for observation in &observations {
        assert_eq!(
            observation.identity.source_kind,
            SemanticOwnerSourceKindV1::DeclaredFunction
        );
        assert_eq!(observation.identity.owner.slot(), 0);
        assert_eq!(observation.cells.len(), 4);
        let observed_cell_count = observation
            .cells
            .iter()
            .filter(|cell| cell.disposition != MatrixDispositionV1::NotYetObserved)
            .count();
        let expected_cell_count = if matches!(observation.status, MatrixFactsStatusV1::Frozen(_)) {
            0
        } else {
            1
        };
        assert_eq!(
            observed_cell_count,
            expected_cell_count,
            "fixture={:?} mode={:?} status={:?} v0={} v1={}",
            observation.fixture,
            observation.mode,
            observation.status,
            observation.v0_present,
            observation.v1_present
        );
        if expected_cell_count == 0 {
            assert!(observation
                .cells
                .iter()
                .all(|cell| cell.disposition == MatrixDispositionV1::NotYetObserved));
            continue;
        }
        let expected = expected_presence(observation.fixture, observation.mode);
        let actual = observation
            .cells
            .iter()
            .find(|cell| cell.disposition != MatrixDispositionV1::NotYetObserved)
            .expect("one observed matrix cell");
        assert_eq!(
            actual.presence,
            expected,
            "fixture={:?} mode={:?} status={:?} v0={} v1={}",
            observation.fixture,
            observation.mode,
            observation.status,
            observation.v0_present,
            observation.v1_present
        );
        match observation.fixture {
            MatrixFixtureV1::Both => {
                assert_eq!(observation.status, MatrixFactsStatusV1::Available);
                assert!(observation.v1_present);
                assert!(observation.carrier.is_some());
                if matches!(observation.mode, MatrixModeV1::StrictPlannerRequired) {
                    assert!(!observation.v0_present);
                    assert_eq!(actual.disposition, MatrixDispositionV1::UnresolvedV1Only);
                } else {
                    assert!(observation.v0_present);
                    assert_eq!(actual.disposition, MatrixDispositionV1::UnresolvedOverlap);
                }
            }
            MatrixFixtureV1::V1Only => {
                assert_eq!(observation.status, MatrixFactsStatusV1::Available);
                assert!(!observation.v0_present && observation.v1_present);
                assert!(observation.carrier.is_some());
                assert_eq!(actual.disposition, MatrixDispositionV1::UnresolvedV1Only);
            }
            MatrixFixtureV1::NoStandalone => {
                assert_eq!(observation.status, MatrixFactsStatusV1::NoStandaloneRow);
                assert_eq!(actual.disposition, MatrixDispositionV1::NoStandaloneRow);
                assert!(observation.carrier.is_none());
            }
        }
    }
}

#[test]
fn d4_s3_s1_keeps_foreign_and_non_loop_rejects_outside_neither() {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let foreign_unit = parse_unit(BOTH_SOURCE);
    let source_unit = parse_unit(BOTH_SOURCE);
    let foreign_loop = body_stmt(&foreign_unit, 0);
    assert!(matches!(
        issue_shared_loop_source_window_v1(&source_unit, &foreign_loop),
        Err(SharedLoopSourceWindowRejectV1::ForeignOwner)
    ));
    let local = body_stmt(&source_unit, 1);
    assert!(matches!(
        issue_shared_loop_source_window_v1(&source_unit, &local),
        Err(SharedLoopSourceWindowRejectV1::NotLoop)
    ));
}
