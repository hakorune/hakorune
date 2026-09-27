//! One-shot resolver-owned Loop forest capability for a callable lowering scope.
//!
//! The capability owns projections made at the package scope.  It deliberately
//! does not retain `ResolvedFunctionLoweringInputV1` or a second resolver
//! ledger, so the lowering state can lend an exact forest row to a child
//! without introducing a third lifetime on `RawInvocationChildPortV1`.

use std::collections::{BTreeMap, BTreeSet};

use crate::mir::builder::normal_callable_loop_source_route::{
    CallableLoopSourceItemBindingV1, VerifiedCallableLoopCallFreeCoverageV1,
};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::loop_cond_break_continue_projection::{
    issue_loop_cond_break_continue_source_forest_projection_v1,
    LoopCondBreakContinueForestProjectionRejectV1,
};
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::loop_structural_facts::{
    LoopRootSourceBindingRejectV1, LoopSourceForestBindingRejectV1,
};
use crate::mir::resolved_semantics::{
    BodyEffectKindV1, CallableSemanticSourceLedgerView, ResolvedAssignmentFormV1,
    ResolvedAssignmentTargetV1, ResolvedBinaryOperatorV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, ResolvedLoopPlacementV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
};

/// One take outcome for a resolver-issued loop site.
///
/// `Armed` projections are consumed exactly once, paired with the optional
/// call-free coverage proof issued beside them.  `Unarmed` sites were
/// cataloged by the resolver but deliberately left unarmed by the forest
/// projection (for example an unsupported ancestor); the caller keeps the
/// ordinary GenericLoop boundary for them.  `BridgeAbsent` covers callables
/// whose inventory never armed a loop bridge at all.  A site that is neither
/// armed nor unarmed remains a contract violation, not a fallback.
#[derive(Debug)]
pub(in crate::mir::builder) enum CallableLoopSourceBridgeTakeV1 {
    BridgeAbsent,
    Armed {
        projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
        call_free: Option<VerifiedCallableLoopCallFreeCoverageV1>,
    },
    Unarmed,
}

#[derive(Debug)]
pub(super) struct CallableLoopSourceBridgeV1 {
    projections: BTreeMap<SourceStmtSiteV1, VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    source_items: BTreeMap<SourceStmtSiteV1, Box<[CallableLoopSourceItemBindingV1]>>,
    call_free: BTreeMap<SourceStmtSiteV1, VerifiedCallableLoopCallFreeCoverageV1>,
    unarmed: BTreeSet<SourceStmtSiteV1>,
}

impl CallableLoopSourceBridgeV1 {
    /// Build one owned inventory from the exact resolver input.  Declared
    /// callables are the only owners admitted by the forest projection; other
    /// source roots remain explicitly unarmed.
    pub(super) fn from_input(
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<Option<Self>, String> {
        if input.function().source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction {
            return Ok(None);
        }

        let loop_sites = root_loop_sites(input.function().loop_sites().cloned().collect());
        let mut projections = BTreeMap::new();
        let mut source_items = BTreeMap::new();
        let mut call_free = BTreeMap::new();
        let mut unarmed = BTreeSet::new();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .map_err(|error| {
                format!("[freeze:contract][callable-loop/source-bridge/ledger] {error:?}")
            })?;
        for site in &loop_sites {
            let located = input.source().exact_stmt(site).map_err(|error| {
                format!("[freeze:contract][callable-loop/source-bridge/locate] {error:?}")
            })?;
            let projection =
                match issue_loop_cond_break_continue_source_forest_projection_v1(input, &located) {
                    Ok(projection) => projection,
                    Err(error) if projection_is_unarmed(&error) => {
                        if !unarmed.insert(site.clone()) {
                            return Err(
                                "[freeze:contract][callable-loop/source-bridge/duplicate-site]"
                                    .to_owned(),
                            );
                        }
                        continue;
                    }
                    Err(error) => {
                        return Err(format!(
                            "[freeze:contract][callable-loop/source-bridge/projection] {error:?}"
                        ));
                    }
                };
            if projections.insert(site.clone(), projection).is_some() {
                return Err(
                    "[freeze:contract][callable-loop/source-bridge/duplicate-site]".to_owned(),
                );
            }
            let items = ledger
                .method_calls()
                .filter(|(call_site, _)| is_under_root(call_site.node(), site.node()))
                .map(|(_, call)| {
                    CallableLoopSourceItemBindingV1::from_resolved(input.owner(), call).map_err(
                        |error| {
                            format!("[freeze:contract][callable-loop/source-bridge/item] {error:?}")
                        },
                    )
                })
                .collect::<Result<Vec<_>, _>>()?
                .into_boxed_slice();
            if items.is_empty() {
                // A call-free armed loop is admissible only behind the
                // complete bounded-grammar proof; an unproven empty
                // inventory still reaches the route token's
                // SourceItemsMissing terminal with no coverage.
                if let Some(coverage) = issue_call_free_coverage(input, &ledger, site) {
                    call_free.insert(site.clone(), coverage);
                }
            }
            if source_items.insert(site.clone(), items).is_some() {
                return Err(
                    "[freeze:contract][callable-loop/source-bridge/duplicate-items]".to_owned(),
                );
            }
        }

        Ok((!projections.is_empty()).then_some(Self {
            projections,
            source_items,
            call_free,
            unarmed,
        }))
    }

    pub(super) fn take_for(
        &mut self,
        site: &SourceStmtSiteV1,
    ) -> Result<CallableLoopSourceBridgeTakeV1, String> {
        if let Some(projection) = self.projections.remove(site) {
            return Ok(CallableLoopSourceBridgeTakeV1::Armed {
                projection,
                call_free: self.call_free.remove(site),
            });
        }
        if self.unarmed.contains(site) {
            return Ok(CallableLoopSourceBridgeTakeV1::Unarmed);
        }
        Err(format!(
            "[freeze:contract][callable-loop/source-bridge/missing-site] site={site:?}"
        ))
    }

    pub(super) fn source_items_for(
        &self,
        site: &SourceStmtSiteV1,
    ) -> Option<Box<[CallableLoopSourceItemBindingV1]>> {
        self.source_items.get(site).cloned()
    }

    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.projections.len()
    }
}

fn is_under_root(site: &SourceNodeSiteV1, root: &SourceNodeSiteV1) -> bool {
    site.segments().starts_with(root.segments())
}

fn root_loop_sites(loop_sites: Vec<SourceStmtSiteV1>) -> Vec<SourceStmtSiteV1> {
    loop_sites
        .iter()
        .filter(|site| {
            !loop_sites.iter().any(|ancestor| {
                ancestor != *site
                    && site
                        .node()
                        .segments()
                        .starts_with(ancestor.node().segments())
            })
        })
        .cloned()
        .collect()
}

fn projection_is_unarmed(error: &LoopCondBreakContinueForestProjectionRejectV1) -> bool {
    matches!(
        error,
        LoopCondBreakContinueForestProjectionRejectV1::ForestBinding(
            LoopSourceForestBindingRejectV1::Source {
                reason: LoopRootSourceBindingRejectV1::UnsupportedAncestor { .. },
                ..
            }
        )
    )
}

/// Issue the bounded call-free coverage proof for one armed loop whose call
/// item inventory is empty.
///
/// The proof reads resolver-owned rows only. Every expression site under the
/// loop must resolve to an integer literal, a local lexical read, an
/// `Add`/`Multiply`/`Less` binary, or a local `BindingRebind` target; every
/// direct `LoopBody(_)` statement must be a plain binding rebind; no call,
/// exit, or non-`Write` effect row may reach under the loop. `Write` effects
/// are admitted only on the proven rebind targets. Anything outside the
/// grammar returns `None` — an unproven loop keeps the route token's
/// `SourceItemsMissing` terminal rather than widening the grammar by
/// inference.
fn issue_call_free_coverage(
    input: ResolvedFunctionLoweringInputV1<'_>,
    ledger: &CallableSemanticSourceLedgerView<'_>,
    site: &SourceStmtSiteV1,
) -> Option<VerifiedCallableLoopCallFreeCoverageV1> {
    let body_shape = input.body_shape()?;
    let loop_segments = site.node().segments();
    let under_loop = |node: &SourceNodeSiteV1| node.segments().starts_with(loop_segments);

    // Exact condition identity — the resolver's LoopCondition child of this
    // loop site — cross-checked through the placement authority and bounded
    // to the accepted `<` comparison.
    let condition_site = SourcePathV1::from_node(site.node())
        .child(SourcePathSegmentV1::LoopCondition)
        .expr();
    if !matches!(
        ledger.resolved_loop_placement(site, &condition_site),
        Ok(Some(ResolvedLoopPlacementV1::Condition))
    ) {
        return None;
    }
    if !matches!(
        ledger.binary_source(&condition_site),
        Some(binary) if binary.operator() == ResolvedBinaryOperatorV1::Less
    ) {
        return None;
    }

    // Flat NoExitBody — every statement nested under the loop is either the
    // loop statement itself or a direct `LoopBody(_)` plain binding rebind.
    // Deeper statement paths, returns, declarations, nested control, or
    // non-local targets all fall outside the accepted card.
    let mut covered_statements = Vec::new();
    let mut write_targets = BTreeSet::new();
    let mut covered_expressions: BTreeSet<SourceExprSiteV1> =
        [condition_site.clone()].into_iter().collect();
    for statement in body_shape.statements() {
        let stmt_segments = statement.site().node().segments();
        if !stmt_segments.starts_with(loop_segments) {
            continue;
        }
        match &stmt_segments[loop_segments.len()..] {
            [] => continue,
            [SourcePathSegmentV1::LoopBody(_)] => {}
            _ => return None,
        }
        let assignment = body_shape
            .assignment_sources()
            .iter()
            .find(|source| source.statement_site() == statement.site())?;
        if assignment.form() != ResolvedAssignmentFormV1::Plain {
            return None;
        }
        if !matches!(
            ledger.assignment_target(assignment.target_site()),
            Some(ResolvedAssignmentTargetV1::BindingRebind(_))
        ) {
            return None;
        }
        covered_statements.push(statement.site().clone());
        write_targets.insert(assignment.target_site().clone());
        covered_expressions.insert(assignment.target_site().clone());
        covered_expressions.insert(assignment.value_site().clone());
    }

    // Every remaining expression site under the loop must sit inside the
    // bounded scalar grammar; call sites, unary/conditional/construction
    // shapes, upvar captures, and opaque rows all classify as uncovered.
    for expr_site in ledger.source_site_inventory().expression_sites() {
        if !under_loop(expr_site.node())
            || expr_site == &condition_site
            || write_targets.contains(expr_site)
        {
            continue;
        }
        let scalar = matches!(
            ledger.literal_source(expr_site),
            Some(ResolvedLiteralSourceV1::Integer(_))
                | Some(ResolvedLiteralSourceV1::TypedInteger { .. })
        ) || matches!(
            ledger.variable_ref(expr_site),
            Some(ResolvedLexicalRefV1::Local(_))
        ) || matches!(
            ledger.binary_source(expr_site),
            Some(binary)
                if matches!(
                    binary.operator(),
                    ResolvedBinaryOperatorV1::Add
                        | ResolvedBinaryOperatorV1::Multiply
                        | ResolvedBinaryOperatorV1::Less
                )
        );
        if !scalar {
            return None;
        }
        covered_expressions.insert(expr_site.clone());
    }

    // No control transfer may originate under the covered loop.
    for (exit_site, _) in ledger.resolved_exits() {
        if under_loop(exit_site.node()) {
            return None;
        }
    }

    // Effects under the loop are bounded to the proven rebind writes — a
    // call/await/allocation/throw row contradicts the call-free claim even
    // when the item inventory is empty.
    for effect in body_shape.effects() {
        if !under_loop(effect.site.node()) {
            continue;
        }
        match effect.kind {
            BodyEffectKindV1::Write if write_targets.contains(&effect.site) => {}
            _ => return None,
        }
    }

    Some(VerifiedCallableLoopCallFreeCoverageV1::issue(
        input.owner(),
        site.clone(),
        condition_site,
        covered_statements.into_boxed_slice(),
        covered_expressions.into_iter().collect(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{CallableLoopSourceBridgeTakeV1, CallableLoopSourceBridgeV1};
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1};
    use crate::parser::NyashParser;

    #[test]
    fn root_inventory_drops_nested_loop_roots() {
        let root = SourceStmtSiteV1::from_node(SourcePathV1::root_body(1).node());
        let child = SourceStmtSiteV1::from_node(
            SourcePathV1::root_body(1)
                .child(SourcePathSegmentV1::LoopBodyRoot)
                .child(SourcePathSegmentV1::LoopBody(0))
                .node(),
        );

        assert_eq!(
            super::root_loop_sites(vec![child, root.clone()]),
            vec![root]
        );
    }

    #[test]
    fn owned_inventory_takes_exact_loop_projection_once() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(
            crate::mir::compiler::loop_cond_function_for_test(),
        )
        .expect("resolved loop-cond fixture");
        let input = unit.root_function_input().expect("root input");
        let site = input
            .function()
            .loop_sites()
            .next()
            .expect("loop site")
            .clone();
        let mut bridge = CallableLoopSourceBridgeV1::from_input(input)
            .expect("source bridge inventory")
            .expect("declared function loop inventory");

        assert_eq!(bridge.len(), 1);
        let CallableLoopSourceBridgeTakeV1::Armed { projection, .. } =
            bridge.take_for(&site).expect("exact projection")
        else {
            panic!("armed loop site must take the armed projection")
        };
        assert_eq!(projection.member_sites().len(), 1);
        assert!(bridge.take_for(&site).is_err(), "take is one-shot");
        assert_eq!(bridge.len(), 0);
    }

    #[test]
    fn nested_scope_loop_does_not_abort_callable_source_bridge() {
        let program = NyashParser::parse_from_string(
            r#"
static function nested_scope_loop(x: i64): i64 {
    fastmem Contract {
        loop(x < 2) {
            x = x + 1
        }
    }
    return x
}
"#,
        )
        .expect("nested scope loop fixture parses");
        let function = match program {
            crate::ast::ASTNode::Program { statements, .. } => statements
                .into_iter()
                .find(|node| matches!(node, crate::ast::ASTNode::FunctionDeclaration { .. }))
                .expect("nested scope loop function"),
            _ => panic!("fixture is a program"),
        };
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(function)
            .expect("nested scope loop resolves");
        let input = unit.root_function_input().expect("root input");

        assert!(CallableLoopSourceBridgeV1::from_input(input)
            .expect("unsupported nested scope loop must remain unarmed")
            .is_none());
    }

    fn mixed_loop_function() -> crate::ast::ASTNode {
        let program = NyashParser::parse_from_string(
            r#"
static function mixed_loop(x: i64): i64 {
    loop(x < 10) {
        x = x + 1
    }
    fastmem Contract {
        loop(x < 2) {
            x = x + 1
        }
    }
    return x
}
"#,
        )
        .expect("mixed loop fixture parses");
        let crate::ast::ASTNode::Program { statements, .. } = program else {
            panic!("fixture is a program")
        };
        statements
            .into_iter()
            .find(|node| matches!(node, crate::ast::ASTNode::FunctionDeclaration { .. }))
            .expect("mixed loop function")
    }

    #[test]
    fn unarmed_nested_loop_take_is_a_typed_disposition() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(mixed_loop_function())
            .expect("mixed loop fixture resolves");
        let input = unit.root_function_input().expect("root input");
        let sites: Vec<_> = input.function().loop_sites().cloned().collect();
        assert_eq!(sites.len(), 2, "fixture has one armed and one unarmed loop");
        let armed_site = sites
            .iter()
            .find(|site| site.node().segments().len() == 1)
            .expect("root loop site")
            .clone();
        let unarmed_site = sites
            .iter()
            .find(|site| site.node().segments().len() == 2)
            .expect("fastmem-nested loop site")
            .clone();
        let mut bridge = CallableLoopSourceBridgeV1::from_input(input)
            .expect("mixed loop bridge")
            .expect("armed root loop keeps the bridge armed");

        assert!(
            matches!(
                bridge.take_for(&unarmed_site),
                Ok(CallableLoopSourceBridgeTakeV1::Unarmed)
            ),
            "unsupported-ancestor loop must take the unarmed disposition"
        );
        assert!(
            matches!(
                bridge.take_for(&unarmed_site),
                Ok(CallableLoopSourceBridgeTakeV1::Unarmed)
            ),
            "unarmed disposition is not a consumed projection"
        );
        assert!(matches!(
            bridge.take_for(&armed_site),
            Ok(CallableLoopSourceBridgeTakeV1::Armed { .. })
        ));
        assert!(
            bridge.take_for(&armed_site).is_err(),
            "armed take remains one-shot"
        );
        let foreign_site = SourceStmtSiteV1::from_node(SourcePathV1::root_body(9).node());
        assert!(
            bridge.take_for(&foreign_site).is_err(),
            "uncataloged site remains a contract violation"
        );
    }

    #[test]
    fn callable_state_reports_unarmed_disposition_for_nested_loop() {
        use super::super::CallableSemanticLoweringState;
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(mixed_loop_function())
            .expect("mixed loop fixture resolves");
        let input = unit.root_function_input().expect("root input");
        let sites: Vec<_> = input.function().loop_sites().cloned().collect();
        let unarmed_site = sites
            .iter()
            .find(|site| site.node().segments().len() == 2)
            .expect("fastmem-nested loop site")
            .clone();
        let mut state = CallableSemanticLoweringState::from_exact_source(input)
            .expect("mixed loop callable state");

        assert!(matches!(
            state.take_source_loop_bridge(unarmed_site.node()),
            Ok(CallableLoopSourceBridgeTakeV1::Unarmed)
        ));
    }
}
