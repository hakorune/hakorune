//! D3-S2-P3: independent Generic/resolved-family overlap census.
//!
//! This is a cfg(test)-only report product. Raw Generic observations and
//! resolved whole-unit family observations live in separate columns. Fixture
//! labels are reporting labels only; no cross-authority capability, winner,
//! or selector is constructed here.

use super::generic_selection_matrix_tests::{both_body, progression_condition};
use super::route_id::LoopRouteId;
use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::builder::control_flow::plan::facts::{
    GenericLoopCarrierObservationV1, try_build_loop_facts,
};
use crate::mir::builder::control_flow::lower::normalize::canonicalize_loop_facts;
use crate::mir::compiler::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoopFamilyPlanV1, CanonicalLoweringPreflightV1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CensusModeV1 {
    Release,
    Strict,
    StrictPlannerRequired,
}

impl CensusModeV1 {
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
        ])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RawGenericDispositionV1 {
    ObservedOverlap,
    NotYetObserved,
    RawFragmentAbsent,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RawGenericObservationRowV1 {
    fixture_label: &'static str,
    mode: CensusModeV1,
    v0_present: bool,
    v1_present: bool,
    carrier: Option<GenericLoopCarrierObservationV1>,
    raw_schedule: Box<[LoopRouteId]>,
    disposition: RawGenericDispositionV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ResolvedFamilyDispositionV1 {
    ResolvedNestedPredicate,
    ResolvedDirectAccum,
    ResolvedAPlus,
    CanonicalRejected,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResolvedFamilyObservationRowV1 {
    fixture_label: &'static str,
    disposition: ResolvedFamilyDispositionV1,
}

#[derive(Debug, PartialEq, Eq)]
struct FamilyOverlapCensusV1 {
    raw_generic: Box<[RawGenericObservationRowV1]>,
    resolved_family: Box<[ResolvedFamilyObservationRowV1]>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CrossAuthorityDispositionV1 {
    UnresolvedStopFamilyOverlap,
    NotYetObserved,
}

impl FamilyOverlapCensusV1 {
    fn cross_authority_disposition(&self) -> CrossAuthorityDispositionV1 {
        let raw_overlap = self
            .raw_generic
            .iter()
            .any(|row| row.disposition == RawGenericDispositionV1::ObservedOverlap);
        if !raw_overlap || self.resolved_family.is_empty() {
            CrossAuthorityDispositionV1::NotYetObserved
        } else {
            CrossAuthorityDispositionV1::UnresolvedStopFamilyOverlap
        }
    }
}

fn observe_raw_generic(mode: CensusModeV1) -> RawGenericObservationRowV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let _config = mode.config();
    let condition = progression_condition();
    let body = both_body();
    let Some(facts) = try_build_loop_facts(&condition, &body)
        .expect("P3 raw Generic census must not freeze")
    else {
        return RawGenericObservationRowV1 {
            fixture_label: "generic-natural-both",
            mode,
            v0_present: false,
            v1_present: false,
            carrier: None,
            raw_schedule: Box::new([]),
            disposition: RawGenericDispositionV1::RawFragmentAbsent,
        };
    };
    let v0_present = facts.generic_loop_v0().is_some();
    let v1 = facts.generic_loop_v1();
    let v1_present = v1.is_some();
    let carrier = v1.map(|facts| facts.carrier_observation.clone());
    let canonical = canonicalize_loop_facts(facts);
    let selection = super::selection::select_recipe_first_routes(Some(&canonical));
    let raw_schedule = selection.raw_execution_routes().to_vec().into_boxed_slice();
    let disposition = if v0_present && v1_present {
        RawGenericDispositionV1::ObservedOverlap
    } else if v1_present {
        RawGenericDispositionV1::NotYetObserved
    } else {
        RawGenericDispositionV1::RawFragmentAbsent
    };
    RawGenericObservationRowV1 {
        fixture_label: "generic-natural-both",
        mode,
        v0_present,
        v1_present,
        carrier,
        raw_schedule,
        disposition,
    }
}

fn observe_resolved_family(
    fixture_label: &'static str,
    tree: ASTNode,
) -> ResolvedFamilyObservationRowV1 {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let disposition = match VerifiedResolvedSourceUnitV1::resolve_function(tree) {
        Ok(unit) => match CanonicalLoweringPreflightV1::verify(&unit) {
            Ok(CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::NestedPredicate(_))) => {
                ResolvedFamilyDispositionV1::ResolvedNestedPredicate
            }
            Ok(CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::DirectAccum(_))) => {
                ResolvedFamilyDispositionV1::ResolvedDirectAccum
            }
            Ok(CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(_)) => {
                ResolvedFamilyDispositionV1::ResolvedAPlus
            }
            Ok(_) | Err(_) => ResolvedFamilyDispositionV1::CanonicalRejected,
        },
        Err(_) => ResolvedFamilyDispositionV1::CanonicalRejected,
    };
    ResolvedFamilyObservationRowV1 {
        fixture_label,
        disposition,
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn a_plus_non_loop_function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "p3_a_plus_non_loop".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            ASTNode::Local {
                variables: vec!["x".into()],
                initial_values: vec![Some(Box::new(integer(0)))],
                declared_type_names: vec![None],
                span: Span::unknown(),
            },
            ASTNode::If {
                condition: Box::new(integer(1)),
                then_body: vec![ASTNode::Assignment {
                    target: Box::new(variable("x")),
                    value: Box::new(integer(1)),
                    span: Span::unknown(),
                }],
                else_body: None,
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn canonical_rejected_function() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "p3_canonical_rejected".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![ASTNode::ImportStatement {
            path: "unsupported".into(),
            alias: None,
            span: Span::unknown(),
        }],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn direct_accum_function() -> ASTNode {
    crate::mir::compiler::direct_accum_projection::direct_accum_function_for_test()
}

fn nested_function() -> ASTNode {
    crate::mir::compiler::nested_function_for_p3_test()
}

#[test]
fn generic_d3_s2_p3_census_keeps_raw_and_resolved_columns_independent() {
    let raw_generic = Vec::from([
        observe_raw_generic(CensusModeV1::Release),
        observe_raw_generic(CensusModeV1::Strict),
        observe_raw_generic(CensusModeV1::StrictPlannerRequired),
    ])
    .into_boxed_slice();
    let resolved_family = Vec::from([
        observe_resolved_family("nested-overlap-envelope", nested_function()),
        observe_resolved_family("direct-accum-capability", direct_accum_function()),
        observe_resolved_family("a-plus-non-loop", a_plus_non_loop_function()),
        observe_resolved_family("canonical-rejected", canonical_rejected_function()),
    ])
    .into_boxed_slice();
    let census = FamilyOverlapCensusV1 {
        raw_generic,
        resolved_family,
    };

    assert!(census.raw_generic.iter().all(|row| {
        row.v1_present
            && row.carrier
                == Some(GenericLoopCarrierObservationV1::CompleteRecursiveCarrier(vec![
                    "j".into(),
                ]))
    }));
    assert!(census.raw_generic[..2].iter().all(|row| {
        row.v0_present && row.disposition == RawGenericDispositionV1::ObservedOverlap
    }));
    assert_eq!(census.raw_generic[2].v0_present, false);
    assert_eq!(
        census.raw_generic[2].disposition,
        RawGenericDispositionV1::NotYetObserved
    );
    assert_eq!(
        census.raw_generic[0].raw_schedule.as_ref(),
        [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert_eq!(
        census.raw_generic[1].raw_schedule.as_ref(),
        [LoopRouteId::GenericLoopV0, LoopRouteId::GenericLoopV1]
    );
    assert_eq!(
        census.raw_generic[2].raw_schedule.as_ref(),
        [LoopRouteId::GenericLoopV1]
    );
    assert_eq!(
        census
            .resolved_family
            .iter()
            .map(|row| row.disposition)
            .collect::<Vec<_>>(),
        vec![
            ResolvedFamilyDispositionV1::ResolvedNestedPredicate,
            ResolvedFamilyDispositionV1::ResolvedDirectAccum,
            ResolvedFamilyDispositionV1::ResolvedAPlus,
            ResolvedFamilyDispositionV1::CanonicalRejected,
        ]
    );
    assert_eq!(
        census.cross_authority_disposition(),
        CrossAuthorityDispositionV1::UnresolvedStopFamilyOverlap
    );
}

#[test]
fn generic_d3_s2_p3_empty_column_stays_not_yet_observed() {
    let census = FamilyOverlapCensusV1 {
        raw_generic: Box::new([]),
        resolved_family: Box::new([]),
    };
    assert_eq!(
        census.cross_authority_disposition(),
        CrossAuthorityDispositionV1::NotYetObserved
    );
}
