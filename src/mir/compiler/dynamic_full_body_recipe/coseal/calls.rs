//! Exact CallSlot/source/target relation verification.

use crate::mir::core_method_op::CoreMethodOp;
use crate::mir::core_method_result_kind::{
    lookup_core_method_result_row_by_op_v1, CoreMethodResultKindV1,
};
use crate::mir::loop_recipe_contract::{
    LoopItemKeyV1, LoopOperationV2, LoopRecipeItemV2, LoopValueClassV2, LoopValueKeyV1,
    VerifiedLoopRecipeV2,
};
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1, SourceExprSiteV1};
use crate::mir::source_call_target::VerifiedSourceBoundDynamicMemberCallV1;

use super::super::super::dynamic_full_body_source::{
    DynamicFullBodyBindingRoleV1, DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1,
};
use super::super::DynamicFullLoopRetainedSourceV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DynamicFullLoopCallRelationRejectV2 {
    MissingTarget,
    AmbiguousTarget,
    DifferentOwner,
    MissingSourceRole,
    MissingBindingRole,
    TargetSourceMismatch,
    TargetBindingMismatch,
    TargetArgumentMismatch,
    TargetDispatchMismatch,
    RecipeCallSlotMismatch,
    RecipeValueClassMismatch,
    CoreMethodContractMismatch,
    ReusedTarget,
    TargetCountMismatch,
    UnexpectedTarget,
}

#[derive(Debug)]
pub(super) struct DynamicFullLoopCallRelationV2 {
    pub(super) item: LoopItemKeyV1,
    pub(super) call_role: DynamicFullBodySourceRoleV1,
    pub(super) target: VerifiedSourceBoundDynamicMemberCallV1,
    core_method: &'static crate::mir::core_method_result_kind::CoreMethodContractResultRowV1,
}

impl DynamicFullLoopCallRelationV2 {
    pub(super) fn core_method(
        &self,
    ) -> &'static crate::mir::core_method_result_kind::CoreMethodContractResultRowV1 {
        self.core_method
    }
}

#[derive(Debug)]
pub(super) struct VerifiedDynamicFullLoopCallRelationsV2 {
    owner: FunctionOwnerIdV1,
    rows: [DynamicFullLoopCallRelationV2; 2],
}

impl VerifiedDynamicFullLoopCallRelationsV2 {
    #[cfg(test)]
    pub(super) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    #[cfg(test)]
    pub(super) const fn rows(&self) -> &[DynamicFullLoopCallRelationV2; 2] {
        &self.rows
    }

    pub(super) fn item_for(&self, role: DynamicFullBodySourceRoleV1) -> Option<LoopItemKeyV1> {
        self.rows
            .iter()
            .find(|row| row.call_role == role)
            .map(|row| row.item)
    }

    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal) fn target_for_item(
        &self,
        item: LoopItemKeyV1,
    ) -> Option<&VerifiedSourceBoundDynamicMemberCallV1> {
        self.rows
            .iter()
            .find(|row| row.item == item)
            .map(|row| &row.target)
    }

    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal) fn relation_for_item(
        &self,
        item: LoopItemKeyV1,
    ) -> Option<&DynamicFullLoopCallRelationV2> {
        self.rows.iter().find(|row| row.item == item)
    }
}

pub(super) fn verify_dynamic_call_relations_v2(
    source: &DynamicFullLoopRetainedSourceV1,
    recipe: &VerifiedLoopRecipeV2,
    targets: Box<[VerifiedSourceBoundDynamicMemberCallV1]>,
) -> Result<VerifiedDynamicFullLoopCallRelationsV2, DynamicFullLoopCallRelationRejectV2> {
    if targets.len() > 2 {
        return Err(DynamicFullLoopCallRelationRejectV2::TargetCountMismatch);
    }
    let substring_expected = DynamicCallExpectationV2::substring();
    let substring_core = core_method_row(&substring_expected)?;
    let index_of_expected = DynamicCallExpectationV2::index_of();
    let index_of_core = core_method_row(&index_of_expected)?;
    verify_one(source, recipe, &targets, substring_expected, substring_core)?;
    verify_one(source, recipe, &targets, index_of_expected, index_of_core)?;

    let substring_site = expr_site(source, DynamicFullBodySourceRoleV1::SubstringCall)?.clone();
    let index_of_site = expr_site(source, DynamicFullBodySourceRoleV1::IndexOfCall)?.clone();
    let mut substring_target = None;
    let mut index_of_target = None;
    for target in targets.into_vec() {
        if target.owner() != source.owner {
            return Err(DynamicFullLoopCallRelationRejectV2::DifferentOwner);
        }
        if target.call_site() == &substring_site {
            if substring_target.replace(target).is_some() {
                return Err(DynamicFullLoopCallRelationRejectV2::AmbiguousTarget);
            }
        } else if target.call_site() == &index_of_site {
            if index_of_target.replace(target).is_some() {
                return Err(DynamicFullLoopCallRelationRejectV2::AmbiguousTarget);
            }
        } else {
            return Err(DynamicFullLoopCallRelationRejectV2::UnexpectedTarget);
        }
    }
    let substring_target =
        substring_target.ok_or(DynamicFullLoopCallRelationRejectV2::MissingTarget)?;
    let index_of_target =
        index_of_target.ok_or(DynamicFullLoopCallRelationRejectV2::MissingTarget)?;
    if substring_target.call_site() == index_of_target.call_site() {
        return Err(DynamicFullLoopCallRelationRejectV2::ReusedTarget);
    }
    Ok(VerifiedDynamicFullLoopCallRelationsV2 {
        owner: source.owner,
        rows: [
            DynamicFullLoopCallRelationV2 {
                item: LoopItemKeyV1::new(6),
                call_role: DynamicFullBodySourceRoleV1::SubstringCall,
                target: substring_target,
                core_method: substring_core,
            },
            DynamicFullLoopCallRelationV2 {
                item: LoopItemKeyV1::new(7),
                call_role: DynamicFullBodySourceRoleV1::IndexOfCall,
                target: index_of_target,
                core_method: index_of_core,
            },
        ],
    })
}

struct DynamicCallExpectationV2 {
    item: LoopItemKeyV1,
    call: DynamicFullBodySourceRoleV1,
    receiver_site: DynamicFullBodySourceRoleV1,
    receiver_binding: DynamicFullBodyBindingRoleV1,
    arguments: &'static [(u32, DynamicFullBodySourceRoleV1)],
    arity: u32,
    recipe_receiver: LoopValueKeyV1,
    recipe_receiver_class: LoopValueClassV2,
    recipe_arguments: &'static [u32],
    recipe_argument_classes: &'static [LoopValueClassV2],
    recipe_result: LoopValueKeyV1,
    core_method_op: CoreMethodOp,
}

impl DynamicCallExpectationV2 {
    const fn substring() -> Self {
        Self {
            item: LoopItemKeyV1::new(6),
            call: DynamicFullBodySourceRoleV1::SubstringCall,
            receiver_site: DynamicFullBodySourceRoleV1::SubstringReceiverSrc,
            receiver_binding: DynamicFullBodyBindingRoleV1::Src,
            arguments: &[
                (0, DynamicFullBodySourceRoleV1::SubstringStartI),
                (1, DynamicFullBodySourceRoleV1::SubstringEndAdd),
            ],
            arity: 2,
            recipe_receiver: LoopValueKeyV1::new(0),
            recipe_receiver_class: LoopValueClassV2::Dynamic,
            recipe_arguments: &[6, 9],
            recipe_argument_classes: &[LoopValueClassV2::I64, LoopValueClassV2::I64],
            recipe_result: LoopValueKeyV1::new(10),
            core_method_op: CoreMethodOp::StringSubstring,
        }
    }

    const fn index_of() -> Self {
        Self {
            item: LoopItemKeyV1::new(7),
            call: DynamicFullBodySourceRoleV1::IndexOfCall,
            receiver_site: DynamicFullBodySourceRoleV1::IndexOfReceiverPredChars,
            receiver_binding: DynamicFullBodyBindingRoleV1::PredChars,
            arguments: &[(0, DynamicFullBodySourceRoleV1::IndexOfArgumentCh)],
            arity: 1,
            recipe_receiver: LoopValueKeyV1::new(3),
            recipe_receiver_class: LoopValueClassV2::Dynamic,
            recipe_arguments: &[10],
            recipe_argument_classes: &[LoopValueClassV2::Dynamic],
            recipe_result: LoopValueKeyV1::new(11),
            core_method_op: CoreMethodOp::StringIndexOf,
        }
    }
}

fn verify_one<'a>(
    source: &DynamicFullLoopRetainedSourceV1,
    recipe: &VerifiedLoopRecipeV2,
    targets: &'a [VerifiedSourceBoundDynamicMemberCallV1],
    expected: DynamicCallExpectationV2,
    core_row: &'static crate::mir::core_method_result_kind::CoreMethodContractResultRowV1,
) -> Result<&'a VerifiedSourceBoundDynamicMemberCallV1, DynamicFullLoopCallRelationRejectV2> {
    let call_site = expr_site(source, expected.call)?;
    let receiver_site = expr_site(source, expected.receiver_site)?;
    let receiver_binding = binding(source, expected.receiver_binding)?;
    let mut matches = targets
        .iter()
        .filter(|target| target.owner() == source.owner && target.call_site() == call_site);
    let Some(target) = matches.next() else {
        return Err(DynamicFullLoopCallRelationRejectV2::MissingTarget);
    };
    if matches.next().is_some() {
        return Err(DynamicFullLoopCallRelationRejectV2::AmbiguousTarget);
    }

    if target.call_site() != call_site
        || target.result_site() != call_site
        || target.receiver_site() != receiver_site
    {
        return Err(DynamicFullLoopCallRelationRejectV2::TargetSourceMismatch);
    }
    if target.receiver_binding() != receiver_binding || target.dynamic_origin() != receiver_binding
    {
        return Err(DynamicFullLoopCallRelationRejectV2::TargetBindingMismatch);
    }
    if target.dispatch().selector() != core_row.canonical
        || target.dispatch().arity() != expected.arity
    {
        return Err(DynamicFullLoopCallRelationRejectV2::TargetDispatchMismatch);
    }
    if target.arguments().len() != expected.arguments.len()
        || target
            .arguments()
            .iter()
            .zip(expected.arguments.iter())
            .any(|(actual, (ordinal, role))| {
                actual.ordinal() != *ordinal
                    || expr_site(source, *role).map_or(true, |site| actual.site() != site)
            })
    {
        return Err(DynamicFullLoopCallRelationRejectV2::TargetArgumentMismatch);
    }

    verify_recipe_call(recipe, &expected, core_row)?;
    Ok(target)
}

fn verify_recipe_call(
    recipe: &VerifiedLoopRecipeV2,
    expected: &DynamicCallExpectationV2,
    core_row: &'static crate::mir::core_method_result_kind::CoreMethodContractResultRowV1,
) -> Result<(), DynamicFullLoopCallRelationRejectV2> {
    let recipe = recipe.as_recipe();
    let Some(row) = recipe.items.iter().find(|row| row.key == expected.item) else {
        return Err(DynamicFullLoopCallRelationRejectV2::RecipeCallSlotMismatch);
    };
    let LoopRecipeItemV2::Operation {
        operation:
            LoopOperationV2::CallSlot {
                receiver: Some(receiver),
                args,
                result: Some(result),
            },
    } = &row.item
    else {
        return Err(DynamicFullLoopCallRelationRejectV2::RecipeCallSlotMismatch);
    };
    let expected_args = expected
        .recipe_arguments
        .iter()
        .copied()
        .map(LoopValueKeyV1::new)
        .collect::<Vec<_>>();
    if *receiver != expected.recipe_receiver
        || args != &expected_args
        || *result != expected.recipe_result
    {
        return Err(DynamicFullLoopCallRelationRejectV2::RecipeCallSlotMismatch);
    }
    if recipe
        .values
        .iter()
        .find(|row| row.key == *receiver)
        .map(|row| row.class)
        != Some(expected.recipe_receiver_class)
        || args.len() != expected.recipe_argument_classes.len()
        || args
            .iter()
            .zip(expected.recipe_argument_classes)
            .any(|(key, class)| {
                recipe
                    .values
                    .iter()
                    .find(|row| row.key == *key)
                    .map(|row| row.class)
                    != Some(*class)
            })
        || recipe
            .values
            .iter()
            .find(|row| row.key == *result)
            .map(|row| row.class)
            != Some(recipe_value_class(core_row.result_kind)?)
    {
        return Err(DynamicFullLoopCallRelationRejectV2::RecipeValueClassMismatch);
    }
    Ok(())
}

fn core_method_row(
    expected: &DynamicCallExpectationV2,
) -> Result<
    &'static crate::mir::core_method_result_kind::CoreMethodContractResultRowV1,
    DynamicFullLoopCallRelationRejectV2,
> {
    lookup_core_method_result_row_by_op_v1("StringBox", expected.core_method_op, expected.arity)
        .ok_or(DynamicFullLoopCallRelationRejectV2::CoreMethodContractMismatch)
}

fn recipe_value_class(
    kind: CoreMethodResultKindV1,
) -> Result<LoopValueClassV2, DynamicFullLoopCallRelationRejectV2> {
    match kind {
        CoreMethodResultKindV1::StringValue => Ok(LoopValueClassV2::Dynamic),
        CoreMethodResultKindV1::I64Value => Ok(LoopValueClassV2::I64),
        CoreMethodResultKindV1::BoolValue
        | CoreMethodResultKindV1::NoValue
        | CoreMethodResultKindV1::Dynamic => {
            Err(DynamicFullLoopCallRelationRejectV2::CoreMethodContractMismatch)
        }
    }
}

fn expr_site(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodySourceRoleV1,
) -> Result<&SourceExprSiteV1, DynamicFullLoopCallRelationRejectV2> {
    source
        .rows
        .iter()
        .find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Expression(site) => Some(site),
                DynamicFullBodySourceSiteV1::Statement(_) => None,
            })?
        })
        .ok_or(DynamicFullLoopCallRelationRejectV2::MissingSourceRole)
}

fn binding(
    source: &DynamicFullLoopRetainedSourceV1,
    role: DynamicFullBodyBindingRoleV1,
) -> Result<BindingRefV1, DynamicFullLoopCallRelationRejectV2> {
    source
        .bindings
        .iter()
        .find_map(|row| (row.role() == role).then(|| row.binding()))
        .ok_or(DynamicFullLoopCallRelationRejectV2::MissingBindingRole)
}

#[cfg(test)]
mod tests {
    use crate::mir::loop_recipe_contract::{
        LoopOperationV2, LoopRecipeItemV2, LoopRecipeVerifierV2,
    };

    use super::*;

    #[test]
    fn reordered_recipe_arguments_reject_without_selector_reasoning() {
        let mut recipe = super::super::super::mapping::complete_dynamic_loop_recipe_v2();
        let LoopRecipeItemV2::Operation {
            operation: LoopOperationV2::CallSlot { args, .. },
        } = &mut recipe.items[6].item
        else {
            panic!("I6 must be CallSlot")
        };
        args.swap(0, 1);
        let verified = LoopRecipeVerifierV2::verify(recipe).expect("shape remains type-valid");
        let expected = DynamicCallExpectationV2::substring();
        let core_row = core_method_row(&expected).expect("generated substring row");
        assert_eq!(
            verify_recipe_call(&verified, &expected, core_row),
            Err(DynamicFullLoopCallRelationRejectV2::RecipeCallSlotMismatch)
        );
    }

    #[test]
    fn dispatch_selector_and_arity_are_part_of_the_source_target_contract() {
        let fixture = super::super::tests::fixture(true);
        let (source, artifact, _claims) = fixture.candidate.into_parts();
        let mut expectation = DynamicCallExpectationV2::substring();
        expectation.core_method_op = CoreMethodOp::StringIndexOf;
        let core_row = core_method_row(&expectation).expect("generated indexOf row");
        assert_eq!(
            verify_one(
                &source,
                artifact.recipe(),
                &fixture.calls,
                expectation,
                core_row
            ),
            Err(DynamicFullLoopCallRelationRejectV2::TargetDispatchMismatch)
        );
    }

    #[test]
    fn text_scan_contract_rejects_untyped_generated_result_projection() {
        assert_eq!(
            recipe_value_class(CoreMethodResultKindV1::Dynamic),
            Err(DynamicFullLoopCallRelationRejectV2::CoreMethodContractMismatch)
        );
    }
}
