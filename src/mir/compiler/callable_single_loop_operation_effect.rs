//! Callable source-ledger adapter for the neutral Loop operation/effect product.
//!
//! This is caller-zero evidence only. It consumes the callable co-seal once,
//! proves the profile operation views against the sealed Recipe, and issues
//! the neutral item-keyed product without creating a second Core or operation
//! owner.

#[cfg(test)]
use super::lowering_input::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_recipe_contract::{
    LoopBindingEffectAnchorV1, LoopBindingEffectRoleV1, LoopBlockKeyV1,
    LoopOperationEffectRejectV1, LoopOperationSourceEvidenceV1, LoopOperationV1, LoopRecipeItemV1,
    VerifiedLoopContinuationContractV1, VerifiedLoopOperationEffectProductV1,
    VerifiedLoopSemanticContextV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1};

use super::callable_single_loop_recipe_coseal::{
    LoopRecipeOperationViewV1, VerifiedCallablePreludeV1,
    VerifiedCallableSingleLoopRecipeProductV1, VerifiedCallableTailV1,
    VerifiedLoopOperationSourceRelationV1,
};
use crate::mir::loop_recipe_contract::{
    LoopItemKeyV1, LoopNodeKeyV1, VerifiedLoopInitializedLocalInputSourceSetV1,
};

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum CallableOperationEffectAdapterRejectV1 {
    MissingPlacement { item: LoopItemKeyV1 },
    DuplicatePlacement { item: LoopItemKeyV1 },
    OperationMismatch { item: LoopItemKeyV1 },
    MissingSourceEffect { item: LoopItemKeyV1 },
    Product(LoopOperationEffectRejectV1),
}

/// Move-only neutral parts emitted by the callable source/effect adapter.
/// The prepared-ingress assembler is the only production consumer. This is
/// intentionally not a public profile product and exposes no borrowed view.
#[derive(Debug)]
pub(in crate::mir) struct CallableOperationEffectPartsV1 {
    operation_effect: VerifiedLoopOperationEffectProductV1,
    input: VerifiedLoopInitializedLocalInputSourceSetV1,
    context: VerifiedLoopSemanticContextV1,
    continuation: VerifiedLoopContinuationContractV1,
    prelude: VerifiedCallablePreludeV1,
    tail: VerifiedCallableTailV1,
}

impl CallableOperationEffectPartsV1 {
    pub(in crate::mir) fn into_parts(
        self,
    ) -> (
        VerifiedLoopOperationEffectProductV1,
        VerifiedLoopInitializedLocalInputSourceSetV1,
        VerifiedLoopSemanticContextV1,
        VerifiedLoopContinuationContractV1,
        VerifiedCallablePreludeV1,
        VerifiedCallableTailV1,
    ) {
        (
            self.operation_effect,
            self.input,
            self.context,
            self.continuation,
            self.prelude,
            self.tail,
        )
    }
}

#[cfg(test)]
#[derive(Debug)]
pub(crate) struct VerifiedCallableOperationEffectProductV1 {
    operation_effect: VerifiedLoopOperationEffectProductV1,
    input: VerifiedLoopInitializedLocalInputSourceSetV1,
    context: VerifiedLoopSemanticContextV1,
    continuation: VerifiedLoopContinuationContractV1,
    prelude: VerifiedCallablePreludeV1,
    tail: VerifiedCallableTailV1,
}

#[cfg(test)]
pub(crate) fn callable_operation_effect_for_test() -> VerifiedLoopOperationEffectProductV1 {
    tests::issue().into_operation_effect()
}

#[cfg(test)]
pub(crate) fn callable_operation_demand_parts_for_test() -> (
    VerifiedLoopOperationEffectProductV1,
    VerifiedLoopSemanticContextV1,
    VerifiedLoopContinuationContractV1,
) {
    tests::issue().into_operation_demand_parts()
}

#[cfg(test)]
pub(crate) struct CallableOperationFixtureV1 {
    pub(crate) unit: VerifiedResolvedSourceUnitV1,
    pub(crate) product: VerifiedCallableOperationEffectProductV1,
}

#[cfg(test)]
pub(crate) fn callable_operation_fixture_for_test() -> CallableOperationFixtureV1 {
    let unit = tests::unit_for_fixture();
    let product = tests::issue_for_unit(&unit);
    CallableOperationFixtureV1 { unit, product }
}

#[cfg(test)]
impl VerifiedCallableOperationEffectProductV1 {
    pub(crate) fn operation_effect(&self) -> &VerifiedLoopOperationEffectProductV1 {
        &self.operation_effect
    }

    pub(crate) fn input(&self) -> &VerifiedLoopInitializedLocalInputSourceSetV1 {
        &self.input
    }

    pub(crate) fn context(&self) -> &VerifiedLoopSemanticContextV1 {
        &self.context
    }

    pub(crate) fn continuation(&self) -> &VerifiedLoopContinuationContractV1 {
        &self.continuation
    }

    pub(crate) fn prelude(&self) -> &VerifiedCallablePreludeV1 {
        &self.prelude
    }

    pub(crate) fn tail(&self) -> &VerifiedCallableTailV1 {
        &self.tail
    }

    #[cfg(test)]
    pub(crate) fn into_operation_effect(self) -> VerifiedLoopOperationEffectProductV1 {
        self.operation_effect
    }

    #[cfg(test)]
    pub(crate) fn into_operation_demand_parts(
        self,
    ) -> (
        VerifiedLoopOperationEffectProductV1,
        VerifiedLoopSemanticContextV1,
        VerifiedLoopContinuationContractV1,
    ) {
        let Self {
            operation_effect,
            context,
            continuation,
            input: _,
            prelude: _,
            tail: _,
        } = self;
        (operation_effect, context, continuation)
    }

    #[cfg(test)]
    pub(crate) fn into_full_parts(
        self,
    ) -> (
        VerifiedLoopOperationEffectProductV1,
        VerifiedLoopInitializedLocalInputSourceSetV1,
        VerifiedLoopSemanticContextV1,
        VerifiedLoopContinuationContractV1,
        VerifiedCallablePreludeV1,
        VerifiedCallableTailV1,
    ) {
        let Self {
            operation_effect,
            input,
            context,
            continuation,
            prelude,
            tail,
        } = self;
        (
            operation_effect,
            input,
            context,
            continuation,
            prelude,
            tail,
        )
    }
}

pub(in crate::mir) fn issue_callable_operation_effect_parts_v1(
    product: VerifiedCallableSingleLoopRecipeProductV1,
) -> Result<CallableOperationEffectPartsV1, CallableOperationEffectAdapterRejectV1> {
    let (co_seal, prelude, tail) = product.into_parts();
    let (core, input, operations, context, continuation) = co_seal.into_parts();
    let mut evidence = Vec::with_capacity(operations.len());
    for relation in operations.iter() {
        let item = relation.item();
        if !recipe_operation_matches(&core, relation) {
            return Err(CallableOperationEffectAdapterRejectV1::OperationMismatch { item });
        }
        let (block, owner_loop) = placement(&core, item)?;
        let anchor = LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(
            core.owner(),
            relation.site().clone(),
        ));
        let source_binding = source_binding(&core, relation, &anchor)?;
        evidence.push(LoopOperationSourceEvidenceV1::new(
            item,
            anchor,
            context.loop_site().clone(),
            owner_loop,
            block,
            source_binding,
        ));
    }
    let operation_effect = VerifiedLoopOperationEffectProductV1::issue(core, evidence)
        .map_err(CallableOperationEffectAdapterRejectV1::Product)?;
    Ok(CallableOperationEffectPartsV1 {
        operation_effect,
        input,
        context,
        continuation,
        prelude,
        tail,
    })
}

#[cfg(test)]
pub(crate) fn issue_callable_operation_effect_v1(
    product: VerifiedCallableSingleLoopRecipeProductV1,
) -> Result<VerifiedCallableOperationEffectProductV1, CallableOperationEffectAdapterRejectV1> {
    let parts = issue_callable_operation_effect_parts_v1(product)?;
    let (operation_effect, input, context, continuation, prelude, tail) = parts.into_parts();
    Ok(VerifiedCallableOperationEffectProductV1 {
        operation_effect,
        input,
        context,
        continuation,
        prelude,
        tail,
    })
}

fn placement(
    core: &crate::mir::loop_recipe_contract::VerifiedLoopCoreProductV1,
    item: LoopItemKeyV1,
) -> Result<(LoopBlockKeyV1, LoopNodeKeyV1), CallableOperationEffectAdapterRejectV1> {
    let mut found = None;
    for block in &core.recipe().as_recipe().blocks {
        if block.items.contains(&item) {
            if found.is_some() {
                return Err(CallableOperationEffectAdapterRejectV1::DuplicatePlacement { item });
            }
            found = Some((block.key, block.owner_loop));
        }
    }
    found.ok_or(CallableOperationEffectAdapterRejectV1::MissingPlacement { item })
}

fn recipe_operation_matches(
    core: &crate::mir::loop_recipe_contract::VerifiedLoopCoreProductV1,
    relation: &VerifiedLoopOperationSourceRelationV1,
) -> bool {
    let Some(row) = core
        .recipe()
        .as_recipe()
        .items
        .iter()
        .find(|row| row.key == relation.item())
    else {
        return false;
    };
    let LoopRecipeItemV1::Operation { operation } = row.item else {
        return false;
    };
    match (relation.operation(), operation) {
        (
            LoopRecipeOperationViewV1::ReadBinding { binding, result },
            LoopOperationV1::ReadBinding {
                binding: expected,
                result: expected_result,
            },
        ) => binding == expected && result == expected_result,
        (
            LoopRecipeOperationViewV1::ConstI64 { result, value },
            LoopOperationV1::ConstI64 {
                result: expected_result,
                value: expected_value,
            },
        ) => result == expected_result && value == expected_value,
        (
            LoopRecipeOperationViewV1::CompareI64 {
                op,
                left,
                right,
                result,
            },
            LoopOperationV1::CompareI64 {
                op: expected_op,
                left: expected_left,
                right: expected_right,
                result: expected_result,
            },
        ) => {
            compare_op(op, expected_op)
                && left == expected_left
                && right == expected_right
                && result == expected_result
        }
        (
            LoopRecipeOperationViewV1::BinaryI64 {
                op,
                left,
                right,
                result,
            },
            LoopOperationV1::BinaryI64 {
                op: expected_op,
                left: expected_left,
                right: expected_right,
                result: expected_result,
            },
        ) => {
            binary_op(op, expected_op)
                && left == expected_left
                && right == expected_right
                && result == expected_result
        }
        (
            LoopRecipeOperationViewV1::WriteBinding { binding, value },
            LoopOperationV1::WriteBinding {
                binding: expected,
                value: expected_value,
            },
        ) => binding == expected && value == expected_value,
        _ => false,
    }
}

fn source_binding(
    core: &crate::mir::loop_recipe_contract::VerifiedLoopCoreProductV1,
    relation: &VerifiedLoopOperationSourceRelationV1,
    anchor: &LoopBindingEffectAnchorV1,
) -> Result<Option<BindingRefV1>, CallableOperationEffectAdapterRejectV1> {
    let (binding, read) = match relation.operation() {
        LoopRecipeOperationViewV1::ReadBinding { binding, .. } => (binding, true),
        LoopRecipeOperationViewV1::WriteBinding { binding, .. } => (binding, false),
        _ => return Ok(None),
    };
    core.effect_relations()
        .iter()
        .find(|row| {
            row.recipe_binding() == binding
                && row.anchor() == anchor
                && if read {
                    matches!(row.role(), LoopBindingEffectRoleV1::SourceRead { .. })
                } else {
                    matches!(row.role(), LoopBindingEffectRoleV1::SourceWrite { .. })
                }
        })
        .map(|row| Some(row.source_binding()))
        .ok_or(
            CallableOperationEffectAdapterRejectV1::MissingSourceEffect {
                item: relation.item(),
            },
        )
}

fn compare_op(
    source: crate::mir::compiler::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1,
    recipe: crate::mir::loop_recipe_contract::LoopCompareI64OpV1,
) -> bool {
    matches!(
        (source, recipe),
        (
            crate::mir::compiler::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1::Less,
            crate::mir::loop_recipe_contract::LoopCompareI64OpV1::Less
        )
    )
}

fn binary_op(
    source: crate::mir::compiler::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1,
    recipe: crate::mir::loop_recipe_contract::LoopBinaryI64OpV1,
) -> bool {
    matches!(
        (source, recipe),
        (
            crate::mir::compiler::callable_single_loop_source_shapes::SyntaxBinaryOperatorV1::Add,
            crate::mir::loop_recipe_contract::LoopBinaryI64OpV1::Add
        )
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::compiler::callable_single_loop_recipe_coseal::issue_callable_single_loop_recipe_v1;
    use crate::mir::compiler::callable_single_loop_source_map::issue_callable_single_loop_source_map_v1;
    use crate::mir::compiler::callable_single_loop_syntax_facts::issue_callable_single_loop_syntax_facts_v1;
    use crate::mir::compiler::callable_single_loop_syntax_facts::tests::{
        input_loop_and_context, unit,
    };
    use crate::mir::loop_recipe_contract::LoopValueKeyV1;

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    pub(super) fn unit_for_fixture() -> VerifiedResolvedSourceUnitV1 {
        unit(None, integer(1))
    }

    pub(super) fn issue_for_unit(
        unit: &VerifiedResolvedSourceUnitV1,
    ) -> VerifiedCallableOperationEffectProductV1 {
        let (input, loop_stmt, context) = input_loop_and_context(&unit);
        let syntax = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context).unwrap();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).unwrap();
        let recipe = issue_callable_single_loop_recipe_v1(&ledger, map).unwrap();
        issue_callable_operation_effect_v1(recipe).unwrap()
    }

    pub(super) fn issue() -> VerifiedCallableOperationEffectProductV1 {
        let unit = unit_for_fixture();
        issue_for_unit(&unit)
    }

    #[test]
    fn callable_adapter_issues_neutral_product_once() {
        let product = issue();
        assert_eq!(product.operation_effect().evidence().len(), 7);
        assert_eq!(
            product.operation_effect().core().owner(),
            product.context().owner()
        );
        assert_eq!(product.prelude().binding(), product.tail().binding());
        assert_eq!(product.input().rows().len(), 1);
        assert_eq!(product.input().rows()[0].recipe_value().raw(), 0);
        assert_eq!(product.continuation().loop_key().raw(), 0);
    }

    #[test]
    fn semantic_program_consumes_the_complete_callable_parent_once() {
        let unit = unit_for_fixture();
        let (input, loop_stmt, context) = input_loop_and_context(&unit);
        let syntax = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context).unwrap();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).unwrap();
        let recipe = issue_callable_single_loop_recipe_v1(&ledger, map).unwrap();

        let program = crate::mir::compiler::callable_semantic_program::
            issue_callable_semantic_program_v1(recipe)
            .expect("Callable parent should co-seal once");
        let (operation_effect, input, context, continuation, prelude, tail) =
            program.into_prepared_parts();

        assert_eq!(operation_effect.evidence().len(), 7);
        assert_eq!(input.owner(), context.owner());
        assert_eq!(continuation.loop_key().raw(), 0);
        assert_eq!(prelude.owner(), tail.owner());
    }

    #[test]
    fn callable_adapter_rejects_recipe_operation_mismatch() {
        let unit = unit(None, integer(1));
        let (input, loop_stmt, context) = input_loop_and_context(&unit);
        let syntax = issue_callable_single_loop_syntax_facts_v1(input, loop_stmt, context).unwrap();
        let ledger = input
            .forest()
            .callable_source_ledger(input.owner())
            .unwrap();
        let map = issue_callable_single_loop_source_map_v1(&ledger, syntax).unwrap();
        let product = issue_callable_single_loop_recipe_v1(&ledger, map).unwrap();
        let relation = product
            .co_seal()
            .operations()
            .iter()
            .find(|row| row.item().raw() == 1)
            .expect("condition bound operation");
        let wrong = VerifiedLoopOperationSourceRelationV1::new_for_test(
            relation.role(),
            relation.item(),
            relation.site().clone(),
            LoopRecipeOperationViewV1::ConstI64 {
                result: LoopValueKeyV1::new(2),
                value: 99,
            },
        );
        assert!(!recipe_operation_matches(product.co_seal().core(), &wrong));
    }
}
