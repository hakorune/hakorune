//! The parent side of a child source Loop: where the parent's own source walk
//! retained it, and the parent-visible boundary of its verified product.
use super::*;

/// A child Loop retained by this Loop's own source walk at `site`.
pub(in crate::mir::loop_recipe_contract) fn source_child_loop_v2<'a>(source: &'a Summary, site: &SourceStmtSiteV1)
    -> Option<&'a Rc<VerifiedSourceLoopRecipeProductV2>> {
    fn find<'a>(body: &'a Body, site: &SourceStmtSiteV1) -> Option<&'a Rc<VerifiedSourceLoopRecipeProductV2>> {
        for statement in &body.statements {
            match statement {
                Statement::Loop { site: original, product } if original == site => return Some(product),
                Statement::If { then_body, else_body, .. } => {
                    if let Some(row) = find(then_body, site).or_else(|| else_body.as_ref().and_then(|body| find(body, site))) {
                        return Some(row);
                    }
                }
                _ => {}
            }
        }
        None
    }
    find(source.control.as_ref().ok()?, site)
}

/// The parent-visible boundary of a closed child Loop: every child binding the
/// parent can see at the child entry (inputs) and every child carrier (After
/// results), with their child recipe classes. A child with a Return or any
/// operation that is not NonFaulting is not composed yet.
pub(in crate::mir::loop_recipe_contract) fn child_loop_boundary_v2(child: &VerifiedSourceLoopRecipeProductV2)
    -> Result<(Vec<(BindingRefV1, LoopValueClassV2)>, Vec<(BindingRefV1, LoopValueClassV2)>), String> {
    let recipe = child.recipe.as_recipe();
    if child.schedule().returns_v2().next().is_some() || recipe.items.iter().any(|row| matches!(&row.item,
        LoopRecipeItemV2::Operation { operation } if operation.execution_class_v2() != LoopOperationExecutionClassV2::NonFaulting)) {
        return Err(reject("child-loop-effects"));
    }
    let class = |binding: BindingRefV1| child.bindings.get(&binding)
        .and_then(|key| recipe.bindings.get(key.raw() as usize)).map(|row| row.class)
        .ok_or_else(|| reject("child-loop-binding"));
    let inputs = child.source.source.bindings().bindings().filter(|(_, role)| *role != Role::IterationLocal)
        .map(|(binding, _)| Ok((binding, class(binding)?))).collect::<Result<Vec<_>, String>>()?;
    let outputs = child.source.headers.keys().map(|binding| Ok((*binding, class(*binding)?)))
        .collect::<Result<Vec<_>, String>>()?;
    Ok((inputs, outputs))
}
