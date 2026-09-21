//! Source/Facts and Recipe co-seal for the composite LoopBreak topology.
//!
//! This owner consumes the resolver-issued composite body-role product. It
//! does not select a physical route or manufacture a target from syntax.

use std::collections::BTreeMap;

use crate::mir::builder::control_flow::facts::canon::cond_block_view::CondBlockView;
use crate::mir::builder::control_flow::plan::recipe_tree::verify_source_recipe_block;
use crate::mir::builder::control_flow::plan::recipe_tree::{
    BlockContractKind, BuiltRecipeTree, ExitKind, IfContractKind, IfMode, LoopKindV0,
    LoopV0Features, RecipeBlock, RecipeBodies, RecipeItem,
};
use crate::mir::builder::control_flow::recipes::refs::StmtRef;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::compiler::located::{LocatedBodyV1, LocatedStmtV1};
use crate::mir::compiler::loop_break_composite_body_role::CompositeLoopBodyRoleV1;
use crate::mir::compiler::loop_break_composite_source_projection::{
    issue_loop_break_composite_source_projection_v1, LoopBreakCompositeSourceProjectionRejectV1,
    VerifiedLoopBreakCompositeSourceProjectionV1,
};
use crate::mir::resolved_semantics::{
    RegionId, ResolvedControlTransferV1, VerifiedResolvedLoopSourceV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(in crate::mir) enum CompositeLoopBreakSourceFactsIssueV1 {
    Projection(LoopBreakCompositeSourceProjectionRejectV1),
    Recipe(Box<str>),
    NotComposite,
}

/// One source-bound composite candidate. The Recipe and source projection are
/// issued together so package retention cannot drop their relation.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedCallableLoopBreakCompositeSourceCandidateV1 {
    projection: VerifiedLoopBreakCompositeSourceProjectionV1,
    recipe: BuiltRecipeTree,
}

impl VerifiedCallableLoopBreakCompositeSourceCandidateV1 {
    pub(in crate::mir) fn projection(&self) -> &VerifiedLoopBreakCompositeSourceProjectionV1 {
        &self.projection
    }

    pub(in crate::mir) fn recipe_is_nonempty(&self) -> bool {
        !self.recipe.root.items.is_empty()
    }

    pub(in crate::mir::builder) fn into_parts(
        self,
    ) -> (
        VerifiedLoopBreakCompositeSourceProjectionV1,
        BuiltRecipeTree,
    ) {
        (self.projection, self.recipe)
    }
}

pub(in crate::mir) fn issue_composite_source_candidate_v1(
    input: ResolvedFunctionLoweringInputV1<'_>,
    loop_stmt: &LocatedStmtV1<'_>,
    resolved_source: VerifiedResolvedLoopSourceV1,
) -> Result<VerifiedCallableLoopBreakCompositeSourceCandidateV1, CompositeLoopBreakSourceFactsIssueV1>
{
    let projection =
        issue_loop_break_composite_source_projection_v1(input, loop_stmt, resolved_source)
            .map_err(CompositeLoopBreakSourceFactsIssueV1::Projection)?;
    if projection.forest().member_sites().len() < 2 {
        return Err(CompositeLoopBreakSourceFactsIssueV1::NotComposite);
    }
    let recipe = build_composite_source_recipe(input, &projection)
        .map_err(CompositeLoopBreakSourceFactsIssueV1::Recipe)?;
    Ok(VerifiedCallableLoopBreakCompositeSourceCandidateV1 { projection, recipe })
}

fn build_composite_source_recipe(
    input: ResolvedFunctionLoweringInputV1<'_>,
    projection: &VerifiedLoopBreakCompositeSourceProjectionV1,
) -> Result<BuiltRecipeTree, Box<str>> {
    if projection.owner() != input.owner() {
        return Err("[freeze:contract][composite-loopbreak/recipe-owner]".into());
    }
    let root_stmt = input
        .source()
        .exact_stmt(projection.loop_site())
        .map_err(|_| "[freeze:contract][composite-loopbreak/root-stmt]".to_owned())?;
    let root_body = input
        .source()
        .child_body_from_stmt(
            &root_stmt,
            crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
        )
        .map_err(|_| "[freeze:contract][composite-loopbreak/root-body]".to_owned())?;
    let Some((evidence, body)) = projection.body_roles().root_for_recipe().as_loop() else {
        return Err("[freeze:contract][composite-loopbreak/root-role]".into());
    };
    if evidence.site() != projection.loop_site() || body.len() != root_body.statements().len() {
        return Err("[freeze:contract][composite-loopbreak/root-coverage]".into());
    }

    let mut arena = RecipeBodies::new();
    let root_body_id = arena.register(crate::mir::builder::control_flow::recipes::RecipeBody::new(
        vec![root_stmt.node().clone()],
    ));
    let mut region_to_member = BTreeMap::new();
    let mut parents = Vec::with_capacity(projection.forest().member_sites().len());
    for (index, site) in projection.forest().member_sites().iter().enumerate() {
        let region = input
            .function()
            .loop_region_bundle(site)
            .map_err(|_| "[freeze:contract][composite-loopbreak/loop-region]".to_owned())?
            .loop_pair()
            .region();
        region_to_member.insert(region, index as u32);
        let parent = projection
            .forest()
            .forest_binding()
            .members()
            .get(index)
            .ok_or_else(|| "[freeze:contract][composite-loopbreak/forest-parent]".to_owned())?
            .parent_index();
        parents.push(parent);
    }
    let body_block = build_body_block(
        input,
        &root_body,
        body,
        &mut arena,
        &region_to_member,
        &parents,
        evidence.member_index(),
    )?;
    let condition = input
        .source()
        .child_expr_from_stmt(
            &root_stmt,
            crate::mir::resolved_semantics::ExprChildRoleV1::LoopCondition,
        )
        .map_err(|_| "[freeze:contract][composite-loopbreak/root-condition]".to_owned())?;
    if condition.site() != projection.loop_condition_site() {
        return Err("[freeze:contract][composite-loopbreak/root-condition-site]".into());
    }
    let root = RecipeBlock::new(
        root_body_id,
        vec![RecipeItem::LoopV0 {
            loop_stmt: StmtRef::new(0),
            kind: LoopKindV0::WhileLike,
            cond_view: CondBlockView::from_expr(condition.node()),
            body_block: Box::new(body_block),
            body_contract: BlockContractKind::ExitAllowed,
            features: LoopV0Features::default(),
        }],
    );
    verify_source_recipe_block(&arena, &root, "composite-loopbreak-source")
        .map_err(|error| error.into_boxed_str())?;
    Ok(BuiltRecipeTree { arena, root })
}

fn build_body_block(
    input: ResolvedFunctionLoweringInputV1<'_>,
    body: &LocatedBodyV1<'_>,
    roles: &[CompositeLoopBodyRoleV1],
    arena: &mut RecipeBodies,
    region_to_member: &BTreeMap<RegionId, u32>,
    parents: &[Option<u32>],
    current_member: u32,
) -> Result<RecipeBlock, Box<str>> {
    if roles.len() != body.statements().len() {
        return Err("[freeze:contract][composite-loopbreak/body-coverage]".into());
    }
    let body_id = arena.register(crate::mir::builder::control_flow::recipes::RecipeBody::new(
        body.statements().to_vec(),
    ));
    let mut items = Vec::with_capacity(roles.len());
    for (index, role) in roles.iter().enumerate() {
        let stmt = input
            .source()
            .body_stmt(body, index)
            .map_err(|_| "[freeze:contract][composite-loopbreak/body-stmt]".to_owned())?;
        let stmt_ref = StmtRef::new(index);
        if let Some(site) = role.as_statement() {
            if site != stmt.site() {
                return Err("[freeze:contract][composite-loopbreak/statement-site]".into());
            }
            items.push(RecipeItem::Stmt(stmt_ref));
            continue;
        }
        if let Some((site, record)) = role.as_exit() {
            if site != stmt.site() {
                return Err("[freeze:contract][composite-loopbreak/exit-site]".into());
            }
            items.push(RecipeItem::Exit {
                kind: exit_kind(record.transfer(), current_member, parents, region_to_member)?,
                stmt: stmt_ref,
            });
            continue;
        }
        if let Some((site, condition_site, then_roles, else_roles)) = role.as_if() {
            if site != stmt.site() {
                return Err("[freeze:contract][composite-loopbreak/if-site]".into());
            }
            let then_source = input
                .source()
                .child_body_from_stmt(
                    &stmt,
                    crate::mir::resolved_semantics::BodyChildRoleV1::IfThen,
                )
                .map_err(|_| "[freeze:contract][composite-loopbreak/if-then]".to_owned())?;
            let then_block = build_body_block(
                input,
                &then_source,
                then_roles,
                arena,
                region_to_member,
                parents,
                current_member,
            )?;
            let else_block = if let Some(else_roles) = else_roles {
                let else_source = input
                    .source()
                    .child_body_from_stmt(
                        &stmt,
                        crate::mir::resolved_semantics::BodyChildRoleV1::IfElse,
                    )
                    .map_err(|_| "[freeze:contract][composite-loopbreak/if-else]".to_owned())?;
                Some(Box::new(build_body_block(
                    input,
                    &else_source,
                    else_roles,
                    arena,
                    region_to_member,
                    parents,
                    current_member,
                )?))
            } else {
                None
            };
            let condition = input
                .source()
                .child_expr_from_stmt(
                    &stmt,
                    crate::mir::resolved_semantics::ExprChildRoleV1::IfCondition,
                )
                .map_err(|_| "[freeze:contract][composite-loopbreak/if-condition]".to_owned())?;
            if condition.site() != condition_site {
                return Err("[freeze:contract][composite-loopbreak/if-condition-site]".into());
            }
            let contract = if else_block.is_none() && then_roles.iter().all(is_exit_role) {
                IfContractKind::ExitOnly {
                    mode: IfMode::ExitIf,
                }
            } else {
                IfContractKind::ExitAllowed {
                    mode: IfMode::ExitIf,
                }
            };
            items.push(RecipeItem::IfV2 {
                if_stmt: stmt_ref,
                cond_view: CondBlockView::from_expr(condition.node()),
                contract,
                then_block: Box::new(then_block),
                else_block,
            });
            continue;
        }
        if let Some((evidence, loop_roles)) = role.as_loop() {
            if evidence.site() != stmt.site() {
                return Err("[freeze:contract][composite-loopbreak/loop-site]".into());
            }
            let nested_source = input
                .source()
                .child_body_from_stmt(
                    &stmt,
                    crate::mir::resolved_semantics::BodyChildRoleV1::LoopBody,
                )
                .map_err(|_| "[freeze:contract][composite-loopbreak/nested-body]".to_owned())?;
            let nested_block = build_body_block(
                input,
                &nested_source,
                loop_roles,
                arena,
                region_to_member,
                parents,
                evidence.member_index(),
            )?;
            let condition = input
                .source()
                .child_expr_from_stmt(
                    &stmt,
                    crate::mir::resolved_semantics::ExprChildRoleV1::LoopCondition,
                )
                .map_err(|_| {
                    "[freeze:contract][composite-loopbreak/nested-condition]".to_owned()
                })?;
            items.push(RecipeItem::LoopV0 {
                loop_stmt: stmt_ref,
                kind: LoopKindV0::WhileLike,
                cond_view: CondBlockView::from_expr(condition.node()),
                body_block: Box::new(nested_block),
                body_contract: BlockContractKind::ExitAllowed,
                features: LoopV0Features::default(),
            });
            continue;
        }
        return Err("[freeze:contract][composite-loopbreak/unknown-role]".into());
    }
    Ok(RecipeBlock::new(body_id, items))
}

fn is_exit_role(role: &CompositeLoopBodyRoleV1) -> bool {
    role.as_exit().is_some()
}

fn exit_kind(
    transfer: ResolvedControlTransferV1,
    current_member: u32,
    parents: &[Option<u32>],
    region_to_member: &BTreeMap<RegionId, u32>,
) -> Result<ExitKind, Box<str>> {
    match transfer {
        ResolvedControlTransferV1::Return { .. } => Ok(ExitKind::Return),
        ResolvedControlTransferV1::Break { target_loop }
        | ResolvedControlTransferV1::Continue { target_loop } => {
            let target = region_to_member
                .get(&target_loop)
                .copied()
                .ok_or_else(|| "[freeze:contract][composite-loopbreak/exit-target]".to_owned())?;
            let mut cursor = current_member;
            let mut depth = 1u32;
            while cursor != target {
                let parent = parents
                    .get(cursor as usize)
                    .and_then(|parent| *parent)
                    .ok_or_else(|| {
                        "[freeze:contract][composite-loopbreak/exit-depth]".to_owned()
                    })?;
                cursor = parent;
                depth += 1;
            }
            match transfer {
                ResolvedControlTransferV1::Break { .. } => Ok(ExitKind::Break { depth }),
                ResolvedControlTransferV1::Continue { .. } => Ok(ExitKind::Continue { depth }),
                ResolvedControlTransferV1::Return { .. } => unreachable!(),
            }
        }
    }
}
