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
use crate::mir::compiler::loop_cond_break_continue_projection::issue_loop_cond_break_continue_source_forest_projection_v1;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{
    BodyEffectKindV1, CallableSemanticSourceLedgerView, ResolvedAssignmentFormV1,
    ResolvedAssignmentTargetV1, ResolvedBinaryOperatorV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, ResolvedLoopPlacementV1, SemanticOwnerSourceKindV1, SourceExprSiteV1,
    SourceNodeSiteV1, SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1,
};

/// One take outcome for a resolver-issued loop site.
///
/// `Armed` projections are consumed exactly once, paired with the optional
/// call-free coverage proof issued beside them. `BridgeAbsent` covers
/// owners without a bridge. A missing site on an existing bridge remains a
/// contract violation.
#[derive(Debug)]
pub(in crate::mir::builder) enum CallableLoopSourceBridgeTakeV1 {
    BridgeAbsent,
    Armed {
        projection: VerifiedLoopCondBreakContinueSourceForestProjectionV1,
        call_free: Option<VerifiedCallableLoopCallFreeCoverageV1>,
    },
}

#[derive(Debug)]
pub(super) struct CallableLoopSourceBridgeV1 {
    projections: BTreeMap<SourceStmtSiteV1, VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    source_items: BTreeMap<SourceStmtSiteV1, Box<[CallableLoopSourceItemBindingV1]>>,
    call_free: BTreeMap<SourceStmtSiteV1, VerifiedCallableLoopCallFreeCoverageV1>,
}

impl CallableLoopSourceBridgeV1 {
    /// Build one owned inventory from the exact resolver input.  Declared
    /// callables are the only owners admitted by the forest projection; other
    /// source roots have no bridge.
    pub(super) fn from_input(
        input: ResolvedFunctionLoweringInputV1<'_>,
    ) -> Result<Option<Self>, String> {
        if input.function().source_kind() != SemanticOwnerSourceKindV1::DeclaredFunction {
            return Ok(None);
        }

        // Every Loop statement, root or child, enters through the same raw
        // Loop entry and takes its own projection here.
        let loop_sites: Vec<SourceStmtSiteV1> = input.function().loop_sites().cloned().collect();
        let mut projections = BTreeMap::new();
        let mut source_items = BTreeMap::new();
        let mut call_free = BTreeMap::new();
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
                issue_loop_cond_break_continue_source_forest_projection_v1(input, &located)
                    .map_err(|error| {
                        format!("[freeze:contract][callable-loop/source-bridge/projection] {error:?}")
                    })?;
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
        assert_eq!(projection.members().len(), 1);
        assert!(bridge.take_for(&site).is_err(), "take is one-shot");
        assert_eq!(bridge.len(), 0);
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
    fn nested_and_root_loop_sites_both_take_the_resolved_projection_once() {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(mixed_loop_function())
            .expect("mixed loop fixture resolves");
        let input = unit.root_function_input().expect("root input");
        let sites: Vec<_> = input.function().loop_sites().cloned().collect();
        assert_eq!(sites.len(), 2, "fixture has one root loop and one nested loop");
        let armed_site = sites
            .iter()
            .find(|site| site.node().segments().len() == 1)
            .expect("root loop site")
            .clone();
        let nested_site = sites
            .iter()
            .find(|site| site.node().segments().len() == 2)
            .expect("fastmem-nested loop site")
            .clone();
        let mut bridge = CallableLoopSourceBridgeV1::from_input(input)
            .expect("mixed loop bridge")
            .expect("declared function loop inventory");

        let CallableLoopSourceBridgeTakeV1::Armed { projection, .. } =
            bridge.take_for(&nested_site).expect("nested loop projection")
        else {
            panic!("the nested loop must use its resolved source site")
        };
        assert_eq!(projection.members()[0].source().site(), &nested_site);
        assert!(bridge.take_for(&nested_site).is_err(), "nested take is one-shot");
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

}
