//! One-shot resolver-owned Loop forest capability for a callable lowering scope.
//!
//! The capability owns projections made at the package scope.  It deliberately
//! does not retain `ResolvedFunctionLoweringInputV1` or a second resolver
//! ledger, so the lowering state can lend an exact forest row to a child
//! without introducing a third lifetime on `RawInvocationChildPortV1`.

use std::collections::BTreeMap;

use crate::mir::builder::normal_callable_loop_source_route::CallableLoopSourceItemBindingV1;
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
    SemanticOwnerSourceKindV1, SourceNodeSiteV1, SourceStmtSiteV1,
};

#[derive(Debug)]
pub(super) struct CallableLoopSourceBridgeV1 {
    projections: BTreeMap<SourceStmtSiteV1, VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
    source_items: BTreeMap<SourceStmtSiteV1, Box<[CallableLoopSourceItemBindingV1]>>,
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
                    Err(error) if projection_is_unarmed(&error) => continue,
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
            if source_items.insert(site.clone(), items).is_some() {
                return Err(
                    "[freeze:contract][callable-loop/source-bridge/duplicate-items]".to_owned(),
                );
            }
        }

        Ok((!projections.is_empty()).then_some(Self {
            projections,
            source_items,
        }))
    }

    pub(super) fn take_for(
        &mut self,
        site: &SourceStmtSiteV1,
    ) -> Result<VerifiedLoopCondBreakContinueSourceForestProjectionV1, String> {
        self.projections.remove(site).ok_or_else(|| {
            format!("[freeze:contract][callable-loop/source-bridge/missing-site] site={site:?}")
        })
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

#[cfg(test)]
mod tests {
    use super::CallableLoopSourceBridgeV1;
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
        let projection = bridge.take_for(&site).expect("exact projection");
        assert_eq!(projection.member_sites().len(), 1);
        assert!(bridge.take_for(&site).is_err(), "take is one-shot");
        assert_eq!(bridge.len(), 0);
    }

    #[test]
    fn nested_scope_loop_does_not_abort_callable_source_bridge() {
        let program = NyashParser::parse_from_string(
            r#"
static function nested_scope_loop(x: i64): i64 {
    if x == 1 {
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
}
