//! One-shot resolver-owned Loop forest capability for a callable lowering scope.
//!
//! The capability owns projections made at the package scope.  It deliberately
//! does not retain `ResolvedFunctionLoweringInputV1` or a second resolver
//! ledger, so the lowering state can lend an exact forest row to a child
//! without introducing a third lifetime on `RawInvocationChildPortV1`.

use std::collections::BTreeMap;

use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::loop_cond_break_continue_projection::issue_loop_cond_break_continue_source_forest_projection_v1;
use crate::mir::loop_structural_facts::VerifiedLoopCondBreakContinueSourceForestProjectionV1;
use crate::mir::resolved_semantics::{SemanticOwnerSourceKindV1, SourceStmtSiteV1};

#[derive(Debug)]
pub(super) struct CallableLoopSourceBridgeV1 {
    projections: BTreeMap<SourceStmtSiteV1, VerifiedLoopCondBreakContinueSourceForestProjectionV1>,
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
        for site in &loop_sites {
            let located = input.source().exact_stmt(site).map_err(|error| {
                format!("[freeze:contract][callable-loop/source-bridge/locate] {error:?}")
            })?;
            let projection =
                issue_loop_cond_break_continue_source_forest_projection_v1(input, &located)
                    .map_err(|error| {
                        format!(
                            "[freeze:contract][callable-loop/source-bridge/projection] {error:?}"
                        )
                    })?;
            if projections.insert(site.clone(), projection).is_some() {
                return Err(
                    "[freeze:contract][callable-loop/source-bridge/duplicate-site]".to_owned(),
                );
            }
        }

        Ok((!projections.is_empty()).then_some(Self { projections }))
    }

    pub(super) fn take_for(
        &mut self,
        site: &SourceStmtSiteV1,
    ) -> Result<VerifiedLoopCondBreakContinueSourceForestProjectionV1, String> {
        self.projections.remove(site).ok_or_else(|| {
            format!("[freeze:contract][callable-loop/source-bridge/missing-site] site={site:?}")
        })
    }

    #[cfg(test)]
    pub(super) fn len(&self) -> usize {
        self.projections.len()
    }
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

#[cfg(test)]
mod tests {
    use super::CallableLoopSourceBridgeV1;
    use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
    use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourcePathV1, SourceStmtSiteV1};

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
}
