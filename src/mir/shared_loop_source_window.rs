//! Test-only bridge witness for one canonical Loop source window.
//!
//! This seam is intentionally owned by `mir`, below the compiler products and
//! resolver products it borrows. It proves only that one canonical source unit
//! can lend paired raw/resolved views through one non-Clone receipt. It does
//! not classify a family or publish a Builder/MIR artifact.

use crate::ast::ASTNode;
use crate::mir::compiler::located::LocatedStmtV1;
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{
    FunctionOriginV1, FunctionOwnerIdV1, LoopExecutionFrameKeyV1, SemanticOwnerSourceKindV1,
    SourceStmtSiteV1, VerifiedResolvedLoopSourceForestV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum SharedLoopSourceWindowRejectV1 {
    ForeignOwner,
    NotLoop,
    SourceNavigation,
    SourceLookup,
    SourceForest,
    ForestEmpty,
    ForestRootMismatch,
    FrameMismatch,
    UnsupportedSourceKind(SemanticOwnerSourceKindV1),
}

/// The sole receipt for one resolver-owned source window. It is deliberately
/// non-`Clone` and non-`Copy`; `with_views` is the only paired-view exit.
#[derive(Debug)]
pub(crate) struct VerifiedSharedLoopSourceWindowV1<'a> {
    source_unit: &'a VerifiedResolvedSourceUnitV1,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    forest: VerifiedResolvedLoopSourceForestV1,
    condition: &'a ASTNode,
    body: &'a [ASTNode],
}

/// Raw view: the exact source AST borrowed from the canonical unit. No
/// flattening, reparse, AST rewrite, or route-local identity is allowed here.
#[derive(Debug, Clone)]
pub(crate) struct SharedRawLoopViewV1<'a> {
    owner: FunctionOwnerIdV1,
    loop_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    condition: &'a ASTNode,
    body: &'a [ASTNode],
}

impl<'a> SharedRawLoopViewV1<'a> {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) const fn condition(&self) -> &'a ASTNode {
        self.condition
    }

    pub(crate) const fn body(&self) -> &'a [ASTNode] {
        self.body
    }
}

/// Resolver view: the same owner/site/frame plus the consumed source forest.
#[derive(Debug)]
pub(crate) struct SharedResolvedLoopViewV1<'a> {
    source_unit: &'a VerifiedResolvedSourceUnitV1,
    owner: FunctionOwnerIdV1,
    function_origin: FunctionOriginV1,
    source_kind: SemanticOwnerSourceKindV1,
    loop_site: SourceStmtSiteV1,
    frame: LoopExecutionFrameKeyV1,
    forest: VerifiedResolvedLoopSourceForestV1,
}

impl<'a> SharedResolvedLoopViewV1<'a> {
    pub(crate) const fn source_unit(&self) -> &'a VerifiedResolvedSourceUnitV1 {
        self.source_unit
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) const fn function_origin(&self) -> FunctionOriginV1 {
        self.function_origin
    }

    pub(crate) const fn source_kind(&self) -> SemanticOwnerSourceKindV1 {
        self.source_kind
    }

    pub(crate) fn site(&self) -> &SourceStmtSiteV1 {
        &self.loop_site
    }

    pub(crate) fn frame(&self) -> &LoopExecutionFrameKeyV1 {
        &self.frame
    }

    pub(crate) fn forest(&self) -> &VerifiedResolvedLoopSourceForestV1 {
        &self.forest
    }
}

impl<'a> VerifiedSharedLoopSourceWindowV1<'a> {
    /// Consume the only receipt and lend both views from the same source unit.
    pub(crate) fn with_views<R>(
        self,
        f: impl FnOnce(SharedRawLoopViewV1<'a>, SharedResolvedLoopViewV1<'a>) -> R,
    ) -> R {
        let Self {
            source_unit,
            owner,
            function_origin,
            source_kind,
            loop_site,
            frame,
            forest,
            condition,
            body,
        } = self;
        f(
            SharedRawLoopViewV1 {
                owner,
                loop_site: loop_site.clone(),
                frame: frame.clone(),
                condition,
                body,
            },
            SharedResolvedLoopViewV1 {
                source_unit,
                owner,
                function_origin,
                source_kind,
                loop_site,
                frame,
                forest,
            },
        )
    }
}

/// Issue one receipt only after exact owner, Loop syntax, forest, and frame
/// validation. The input statement is borrowed from the same canonical source
/// lifetime, but the owner check still rejects a foreign located statement.
pub(crate) fn issue_shared_loop_source_window_v1<'a>(
    source_unit: &'a VerifiedResolvedSourceUnitV1,
    loop_stmt: &LocatedStmtV1<'a>,
) -> Result<VerifiedSharedLoopSourceWindowV1<'a>, SharedLoopSourceWindowRejectV1> {
    let input = source_unit
        .root_function_input()
        .map_err(|_| SharedLoopSourceWindowRejectV1::SourceNavigation)?;
    if loop_stmt.owner() != input.owner() {
        return Err(SharedLoopSourceWindowRejectV1::ForeignOwner);
    }
    let (condition, body) = match loop_stmt.node() {
        ASTNode::Loop {
            condition, body, ..
        } => (condition.as_ref(), body.as_slice()),
        _ => return Err(SharedLoopSourceWindowRejectV1::NotLoop),
    };

    let function = input.function();
    let source_kind = function.source_kind();
    if source_kind != SemanticOwnerSourceKindV1::DeclaredFunction {
        return Err(SharedLoopSourceWindowRejectV1::UnsupportedSourceKind(
            source_kind,
        ));
    }
    let loop_site = loop_stmt.site().clone();
    let loop_source = function
        .resolved_loop_source(&loop_site)
        .map_err(|_| SharedLoopSourceWindowRejectV1::SourceLookup)?;
    let frame = loop_source.frame_key();
    let function_origin = function.function_origin();
    let forest = function
        .resolved_loop_source_forest(&loop_site)
        .map_err(map_forest_reject)?;
    let Some(root) = forest.members().first() else {
        return Err(SharedLoopSourceWindowRejectV1::ForestEmpty);
    };
    if root.parent_index().is_some()
        || !root
            .source()
            .matches_identity(function_origin, source_kind, &loop_site)
    {
        return Err(SharedLoopSourceWindowRejectV1::ForestRootMismatch);
    }
    if !root.source().frame_key().matches(&frame) {
        return Err(SharedLoopSourceWindowRejectV1::FrameMismatch);
    }
    Ok(VerifiedSharedLoopSourceWindowV1 {
        source_unit,
        owner: input.owner(),
        function_origin,
        source_kind,
        loop_site,
        frame,
        forest,
        condition,
        body,
    })
}

fn map_forest_reject(
    reject: crate::mir::resolved_semantics::ResolvedLoopSourceForestRejectV1,
) -> SharedLoopSourceWindowRejectV1 {
    use crate::mir::resolved_semantics::ResolvedLoopSourceForestRejectV1;

    match reject {
        ResolvedLoopSourceForestRejectV1::UnsupportedOwnerRoot(kind) => {
            SharedLoopSourceWindowRejectV1::UnsupportedSourceKind(kind)
        }
        ResolvedLoopSourceForestRejectV1::MissingRoot(_) => {
            SharedLoopSourceWindowRejectV1::ForestRootMismatch
        }
        _ => SharedLoopSourceWindowRejectV1::SourceForest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::capability::{
        CanonicalFirstFamilyPlanV1, CanonicalLoopFamilyPlanV1, CanonicalLoweringPreflightV1,
    };
    use crate::mir::compiler::direct_accum_capability::DirectAccumSourceUnitProbeV1;
    use crate::mir::compiler::nested_function_for_p3_test;
    use crate::mir::compiler::CanonicalLoweringErrorV1;
    use crate::mir::compiler::{direct_accum_capability, direct_accum_projection};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum LegacySameSourceModeV1 {
        Release,
        Strict,
        StrictPlannerRequired,
    }

    impl LegacySameSourceModeV1 {
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
    enum LegacyResolvedFamilyV1 {
        NestedPredicate,
        DirectAccum,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct LegacySameSourceRowV1 {
        fixture_label: &'static str,
        mode: LegacySameSourceModeV1,
        owner: FunctionOwnerIdV1,
        site: SourceStmtSiteV1,
        frame: LoopExecutionFrameKeyV1,
        legacy_raw_status: crate::mir::builder::LegacyGenericFactsStatusV1,
        legacy_v0_present: bool,
        legacy_v1_present: bool,
        legacy_carrier: Option<crate::mir::builder::LegacyGenericCarrierSummaryV1>,
        legacy_raw_schedule: Box<[crate::mir::loop_recipe_contract::route_id::LoopRouteId]>,
        legacy_resolved_family: LegacyResolvedFamilyV1,
    }

    fn unit() -> VerifiedResolvedSourceUnitV1 {
        VerifiedResolvedSourceUnitV1::resolve_function(nested_function_for_p3_test())
            .expect("nested source unit resolves")
    }

    fn direct_accum_unit() -> VerifiedResolvedSourceUnitV1 {
        VerifiedResolvedSourceUnitV1::resolve_function(
            direct_accum_projection::direct_accum_function_for_test(),
        )
        .expect("DirectAccum source unit resolves")
    }

    fn body_stmt<'a>(
        source_unit: &'a VerifiedResolvedSourceUnitV1,
        index: usize,
    ) -> LocatedStmtV1<'a> {
        let input = source_unit.root_function_input().expect("root input");
        let body = input.source().root_body().expect("root body");
        input
            .source()
            .body_stmt(&body, index)
            .expect("body statement")
    }

    fn legacy_same_source_row(
        fixture_label: &'static str,
        tree: ASTNode,
        mode: LegacySameSourceModeV1,
    ) -> LegacySameSourceRowV1 {
        crate::runtime::ring0::ensure_global_ring0_initialized();
        let _config = mode.config();
        let source_unit = VerifiedResolvedSourceUnitV1::resolve_function(tree)
            .expect("same-source census fixture resolves");
        let loop_stmt = body_stmt(&source_unit, 1);
        let receipt = issue_shared_loop_source_window_v1(&source_unit, &loop_stmt)
            .expect("same-source census loop window");
        receipt.with_views(|raw, resolved| {
            assert_eq!(raw.owner(), resolved.owner());
            assert_eq!(raw.site(), resolved.site());
            assert!(raw.frame().matches(resolved.frame()));
            let legacy = crate::mir::builder::observe_legacy_generic_loop_for_test(
                raw.condition(),
                raw.body(),
            );
            let legacy_resolved_family =
                match CanonicalLoweringPreflightV1::verify(resolved.source_unit()) {
                    Ok(CanonicalFirstFamilyPlanV1::Loop(
                        CanonicalLoopFamilyPlanV1::NestedPredicate(_),
                    )) => LegacyResolvedFamilyV1::NestedPredicate,
                    Ok(CanonicalFirstFamilyPlanV1::Loop(
                        CanonicalLoopFamilyPlanV1::DirectAccum(_),
                    )) => LegacyResolvedFamilyV1::DirectAccum,
                    Ok(_) | Err(_) => panic!("bounded Loop fixture must resolve to a Loop family"),
                };
            LegacySameSourceRowV1 {
                fixture_label,
                mode,
                owner: raw.owner(),
                site: raw.site().clone(),
                frame: raw.frame().clone(),
                legacy_raw_status: legacy.status,
                legacy_v0_present: legacy.v0_present,
                legacy_v1_present: legacy.v1_present,
                legacy_carrier: legacy.carrier,
                legacy_raw_schedule: legacy.raw_schedule,
                legacy_resolved_family,
            }
        })
    }

    fn legacy_same_source_census() -> Box<[LegacySameSourceRowV1]> {
        [
            LegacySameSourceModeV1::Release,
            LegacySameSourceModeV1::Strict,
            LegacySameSourceModeV1::StrictPlannerRequired,
        ]
        .into_iter()
        .flat_map(|mode| {
            [
                ("nested-predicate", nested_function_for_p3_test()),
                (
                    "direct-accum",
                    direct_accum_projection::direct_accum_function_for_test(),
                ),
            ]
            .into_iter()
            .map(move |(fixture_label, tree)| legacy_same_source_row(fixture_label, tree, mode))
        })
        .collect::<Vec<_>>()
        .into_boxed_slice()
    }

    #[test]
    fn d4_s2_s0_records_six_legacy_same_source_rows() {
        let rows = legacy_same_source_census();
        assert_eq!(rows.len(), 6);
        assert_eq!(
            rows.iter().map(|row| row.mode).collect::<Vec<_>>(),
            vec![
                LegacySameSourceModeV1::Release,
                LegacySameSourceModeV1::Release,
                LegacySameSourceModeV1::Strict,
                LegacySameSourceModeV1::Strict,
                LegacySameSourceModeV1::StrictPlannerRequired,
                LegacySameSourceModeV1::StrictPlannerRequired,
            ]
        );
        for (index, row) in rows.iter().enumerate() {
            assert_eq!(row.owner.slot(), 0);
            assert_eq!(row.site.node(), rows[0].site.node());
            assert!(row.frame.matches(&rows[0].frame));
            assert_eq!(
                row.legacy_raw_status,
                crate::mir::builder::LegacyGenericFactsStatusV1::Available
            );
            assert!(!row.legacy_v0_present);
            assert!(row.legacy_v1_present);
            match index {
                0 | 2 | 4 => {
                    assert_eq!(row.fixture_label, "nested-predicate");
                    assert_eq!(
                        row.legacy_resolved_family,
                        LegacyResolvedFamilyV1::NestedPredicate
                    );
                    assert_eq!(
                        row.legacy_carrier,
                        Some(
                            crate::mir::builder::LegacyGenericCarrierSummaryV1::CompleteRecursive(
                                vec!["j".to_owned(), "sum".to_owned()].into_boxed_slice(),
                            )
                        ),
                    );
                    assert_eq!(
                        row.legacy_raw_schedule.as_ref(),
                        [
                            crate::mir::loop_recipe_contract::route_id::LoopRouteId::NestedLoopMinimal,
                            crate::mir::loop_recipe_contract::route_id::LoopRouteId::GenericLoopV1,
                        ],
                    );
                }
                1 | 3 | 5 => {
                    assert_eq!(row.fixture_label, "direct-accum");
                    assert_eq!(
                        row.legacy_resolved_family,
                        LegacyResolvedFamilyV1::DirectAccum
                    );
                    assert_eq!(
                        row.legacy_carrier,
                        Some(
                            crate::mir::builder::LegacyGenericCarrierSummaryV1::CompleteNoRecursive
                        ),
                    );
                    assert_eq!(
                        row.legacy_raw_schedule.as_ref(),
                        [crate::mir::loop_recipe_contract::route_id::LoopRouteId::AccumConstLoop],
                    );
                }
                _ => unreachable!("six-row census index"),
            }
        }
    }

    #[test]
    fn d4_witness_lends_one_canonical_nested_loop_pair() {
        let source_unit = unit();
        let loop_stmt = body_stmt(&source_unit, 1);
        let receipt = issue_shared_loop_source_window_v1(&source_unit, &loop_stmt)
            .expect("canonical nested loop window");
        receipt.with_views(|raw, resolved| {
            assert_eq!(raw.owner(), resolved.owner());
            assert_eq!(raw.site(), resolved.site());
            assert_eq!(
                resolved.source_kind(),
                SemanticOwnerSourceKindV1::DeclaredFunction
            );
            assert_eq!(resolved.forest().members().len(), 2);
            assert!(resolved.forest().members()[0]
                .source()
                .frame_key()
                .matches(resolved.frame()));
            assert!(matches!(raw.condition(), ASTNode::BinaryOp { .. }));
            assert_eq!(raw.body().len(), 4);
            assert!(matches!(raw.body()[0], ASTNode::Local { .. }));
        });
    }

    #[test]
    fn d4_witness_rejects_foreign_located_statement() {
        let foreign_unit = unit();
        let source_unit = unit();
        let foreign_loop = body_stmt(&foreign_unit, 1);
        assert!(matches!(
            issue_shared_loop_source_window_v1(&source_unit, &foreign_loop),
            Err(SharedLoopSourceWindowRejectV1::ForeignOwner)
        ));
    }

    #[test]
    fn d4_witness_rejects_non_loop_statement() {
        let source_unit = unit();
        let local = body_stmt(&source_unit, 0);
        assert!(matches!(
            issue_shared_loop_source_window_v1(&source_unit, &local),
            Err(SharedLoopSourceWindowRejectV1::NotLoop)
        ));
    }

    #[test]
    fn d4_witness_keeps_equal_shape_sessions_distinct() {
        let left_unit = unit();
        let right_unit = unit();
        let left_stmt = body_stmt(&left_unit, 1);
        let right_stmt = body_stmt(&right_unit, 1);
        let left_owner = left_unit.root_function_input().expect("left input").owner();
        let right_owner = right_unit
            .root_function_input()
            .expect("right input")
            .owner();
        assert_ne!(left_owner, right_owner);
        let left = issue_shared_loop_source_window_v1(&left_unit, &left_stmt).expect("left window");
        let right =
            issue_shared_loop_source_window_v1(&right_unit, &right_stmt).expect("right window");
        left.with_views(|left_raw, _| {
            right.with_views(|right_raw, _| {
                assert_ne!(left_raw.owner(), right_raw.owner());
                assert_eq!(left_raw.site(), right_raw.site());
            });
        });
    }

    #[test]
    fn d4_s1_witness_accepts_existing_direct_accum_probe() {
        let source_unit = direct_accum_unit();
        let loop_stmt = body_stmt(&source_unit, 1);
        let receipt = issue_shared_loop_source_window_v1(&source_unit, &loop_stmt)
            .expect("DirectAccum source window");

        receipt.with_views(|raw, resolved| {
            assert_eq!(raw.body().len(), 2);
            assert_eq!(raw.owner(), resolved.owner());
            assert_eq!(raw.site(), resolved.site());

            let canonical_input = resolved
                .source_unit()
                .root_function_input()
                .expect("canonical DirectAccum input");
            let canonical_body = canonical_input
                .source()
                .root_body()
                .expect("canonical DirectAccum body");
            let canonical_loop = canonical_input
                .source()
                .body_stmt(&canonical_body, 1)
                .expect("canonical DirectAccum loop");
            assert_eq!(canonical_loop.site(), resolved.site());

            let probe =
                direct_accum_capability::probe_direct_accum_source_unit_v1(resolved.source_unit())
                    .expect("exact DirectAccum envelope must be admitted");
            assert!(matches!(probe, DirectAccumSourceUnitProbeV1::Candidate(_)));
        });
    }

    #[test]
    fn d4_s1_witness_rejects_direct_accum_foreign_and_non_loop_rows() {
        let source_unit = direct_accum_unit();
        let foreign_unit = direct_accum_unit();
        let foreign_loop = body_stmt(&foreign_unit, 1);
        assert!(matches!(
            issue_shared_loop_source_window_v1(&source_unit, &foreign_loop),
            Err(SharedLoopSourceWindowRejectV1::ForeignOwner)
        ));

        let local = body_stmt(&source_unit, 0);
        assert!(matches!(
            issue_shared_loop_source_window_v1(&source_unit, &local),
            Err(SharedLoopSourceWindowRejectV1::NotLoop)
        ));
    }

    #[test]
    fn d4_s1_witness_keeps_direct_accum_shape_reject_pre_effect() {
        let mut tree = direct_accum_projection::direct_accum_function_for_test();
        let ASTNode::FunctionDeclaration { body, .. } = &mut tree else {
            unreachable!("DirectAccum fixture is a function");
        };
        let ASTNode::Loop { body, .. } = &mut body[1] else {
            unreachable!("DirectAccum fixture has a root loop");
        };
        body.pop();

        let source_unit = VerifiedResolvedSourceUnitV1::resolve_function(tree)
            .expect("shape-negative DirectAccum source unit resolves");
        let loop_stmt = body_stmt(&source_unit, 1);
        let receipt = issue_shared_loop_source_window_v1(&source_unit, &loop_stmt)
            .expect("source window identity remains valid for shape-negative row");

        receipt.with_views(|raw, resolved| {
            assert_eq!(raw.body().len(), 1);
            assert_eq!(raw.site(), resolved.site());
            assert!(matches!(
                direct_accum_capability::probe_direct_accum_source_unit_v1(resolved.source_unit()),
                Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape { .. })
            ));
        });
    }
}
