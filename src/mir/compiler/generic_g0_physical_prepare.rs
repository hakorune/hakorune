//! Builder-free Generic G0 ingress for the caller-zero parity row.
//!
//! The resolver-issued input remains the only source/entry capability.  The
//! neutral S4 product remains the only Recipe/effect/continuation authority;
//! this module merely proves that the two exact views may be prepared
//! together.  No AST, resolver, Builder, physical ID, fallback, or selector
//! is owned here.

#![cfg(test)]

use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_recipe_contract::{
    produce_generic_g0_recipe_v1, LoopBindingEffectAnchorV1, LoopBindingKeyV1,
    LoopOperationPhysicalDemandRejectV1, LoopValueClassV1, PreparedLoopOperationProgramV1,
    VerifiedGenericG0TailCapabilityV1, VerifiedGenericRecipeProductG0,
    VerifiedLoopOperationPhysicalDemandV1,
};
use crate::mir::resolved_semantics::{
    BindingKindV1, BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, SourceBindingSiteV1,
};

use super::function_input::ResolvedFunctionLoweringInputV1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0IngressRejectReasonV1 {
    MissingInput,
    OwnerMismatch,
    OriginMismatch,
    SourceKindMismatch,
    LoopSiteMissing,
    LoopFrameMismatch,
    ScopeRegionMismatch,
    EntryCountMismatch,
    EntryBindingMissing,
    EntryBindingOwnerMismatch,
    EntryBindingKindMismatch,
    EntryBindingOriginMismatch,
    EntryBindingIndexMismatch,
    EntryBindingClassMismatch,
    EntryBindingAbiMismatch,
    SourceAnchorOwnerMismatch,
    SourceExpressionMissing,
    SourceLoopMissing,
    SourceForestMismatch,
    TailOwnerMismatch,
    TailFrameMismatch,
    TailBindingOwnerMismatch,
    TailStatementMissing,
    TailValueMissing,
    TailAbiMismatch,
    OperationDemand(LoopOperationPhysicalDemandRejectV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericG0IngressRejectV1 {
    NoSafeSlice(GenericG0IngressRejectReasonV1),
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericG0EntryBindingV1 {
    recipe_value: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    binding: BindingRefV1,
    parameter_index: u32,
    abi: ExactTrivialReturnAbiV1,
}

impl VerifiedGenericG0EntryBindingV1 {
    pub(crate) const fn recipe_value(&self) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.recipe_value
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) const fn parameter_index(&self) -> u32 {
        self.parameter_index
    }

    pub(crate) const fn abi(&self) -> ExactTrivialReturnAbiV1 {
        self.abi
    }
}

/// Exact resolver input branded with the two G0 parameter-entry relations.
/// This is a transport receipt, not a second semantic owner.
#[derive(Debug)]
pub(crate) struct VerifiedGenericG0FunctionLoweringInputV1<'a> {
    input: ResolvedFunctionLoweringInputV1<'a>,
    entries: Box<[VerifiedGenericG0EntryBindingV1]>,
}

impl<'a> VerifiedGenericG0FunctionLoweringInputV1<'a> {
    pub(crate) const fn input(&self) -> ResolvedFunctionLoweringInputV1<'a> {
        self.input
    }

    pub(crate) fn entries(&self) -> &[VerifiedGenericG0EntryBindingV1] {
        &self.entries
    }

    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.input.owner()
    }
}

/// One prepared G0 ingress.  It contains the full common program and keeps
/// the profile-specific G0 tail separate from the neutral continuation.
#[derive(Debug)]
pub(crate) struct PreparedGenericG0LoopIngressV1<'a> {
    input: VerifiedGenericG0FunctionLoweringInputV1<'a>,
    program: PreparedLoopOperationProgramV1,
    tail: VerifiedGenericG0TailCapabilityV1,
    target: crate::mir::numeric_substrate::NumericTarget,
}

impl<'a> PreparedGenericG0LoopIngressV1<'a> {
    pub(crate) fn input(&self) -> &VerifiedGenericG0FunctionLoweringInputV1<'a> {
        &self.input
    }

    pub(crate) fn program(&self) -> &PreparedLoopOperationProgramV1 {
        &self.program
    }

    pub(crate) fn tail(&self) -> &VerifiedGenericG0TailCapabilityV1 {
        &self.tail
    }

    pub(crate) const fn target(&self) -> crate::mir::numeric_substrate::NumericTarget {
        self.target
    }

    /// Move the already-prepared G0 capabilities into the physical canary.
    ///
    /// The split is deliberately one-shot: the canary receives the exact
    /// resolver input, full common operation program, and profile-specific
    /// tail without rebuilding any source view or selecting an operation.
    #[cfg(test)]
    pub(crate) fn into_parts(
        self,
    ) -> (
        VerifiedGenericG0FunctionLoweringInputV1<'a>,
        PreparedLoopOperationProgramV1,
        VerifiedGenericG0TailCapabilityV1,
        crate::mir::numeric_substrate::NumericTarget,
    ) {
        (self.input, self.program, self.tail, self.target)
    }
}

pub(crate) fn issue_generic_g0_loop_ingress_v1<'a>(
    input: Option<ResolvedFunctionLoweringInputV1<'a>>,
    product: VerifiedGenericRecipeProductG0,
) -> Result<PreparedGenericG0LoopIngressV1<'a>, GenericG0IngressRejectV1> {
    let input = input.ok_or(no_safe_slice(GenericG0IngressRejectReasonV1::MissingInput))?;
    validate_input_and_product(&input, &product)?;

    let branded = issue_input_capability(&input, &product)?;
    let (operation_effect, context, continuation, tail, target) =
        product.into_physical_parts_for_test();
    let demand =
        VerifiedLoopOperationPhysicalDemandV1::issue(context, operation_effect, continuation)
            .map_err(|reason| {
                no_safe_slice(GenericG0IngressRejectReasonV1::OperationDemand(reason))
            })?;
    let program = demand
        .prepare_all()
        .map_err(|reason| no_safe_slice(GenericG0IngressRejectReasonV1::OperationDemand(reason)))?;
    Ok(PreparedGenericG0LoopIngressV1 {
        input: branded,
        program,
        tail,
        target,
    })
}

fn issue_input_capability<'a>(
    input: &ResolvedFunctionLoweringInputV1<'a>,
    product: &VerifiedGenericRecipeProductG0,
) -> Result<VerifiedGenericG0FunctionLoweringInputV1<'a>, GenericG0IngressRejectV1> {
    let relations = product.core().binding_relations();
    if relations.len() != 2 {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::EntryCountMismatch,
        ));
    }

    let mut entries = Vec::with_capacity(2);
    for (index, expected_recipe_binding) in [0u32, 1u32].into_iter().enumerate() {
        let recipe_binding = LoopBindingKeyV1::new(expected_recipe_binding);
        let Some(relation) = relations
            .iter()
            .find(|relation| relation.recipe_binding() == recipe_binding)
        else {
            return Err(no_safe_slice(
                GenericG0IngressRejectReasonV1::EntryBindingMissing,
            ));
        };
        let binding = relation.source_binding();
        if binding.owner() != input.owner() {
            return Err(no_safe_slice(
                GenericG0IngressRejectReasonV1::EntryBindingOwnerMismatch,
            ));
        }
        if relation.class() != LoopValueClassV1::I64 {
            return Err(no_safe_slice(
                GenericG0IngressRejectReasonV1::EntryBindingClassMismatch,
            ));
        }
        let record = input
            .function()
            .binding(binding)
            .ok_or_else(|| no_safe_slice(GenericG0IngressRejectReasonV1::EntryBindingMissing))?;
        let BindingKindV1::Parameter {
            index: parameter_index,
        } = record.kind()
        else {
            return Err(no_safe_slice(
                GenericG0IngressRejectReasonV1::EntryBindingKindMismatch,
            ));
        };
        if parameter_index != index as u32 {
            return Err(no_safe_slice(
                GenericG0IngressRejectReasonV1::EntryBindingIndexMismatch,
            ));
        }
        if !matches!(
            record.origin(),
            BindingOriginV1::Source(SourceBindingSiteV1::Parameter { index: origin_index })
                if *origin_index == parameter_index
        ) {
            return Err(no_safe_slice(
                GenericG0IngressRejectReasonV1::EntryBindingOriginMismatch,
            ));
        }
        entries.push(VerifiedGenericG0EntryBindingV1 {
            recipe_value: crate::mir::loop_recipe_contract::LoopValueKeyV1::new(index as u32),
            binding,
            parameter_index,
            abi: ExactTrivialReturnAbiV1::I64,
        });
    }
    Ok(VerifiedGenericG0FunctionLoweringInputV1 {
        input: *input,
        entries: entries.into_boxed_slice(),
    })
}

fn validate_input_and_product(
    input: &ResolvedFunctionLoweringInputV1<'_>,
    product: &VerifiedGenericRecipeProductG0,
) -> Result<(), GenericG0IngressRejectV1> {
    let context = product.context();
    if context.owner() != input.owner() || product.core().owner() != input.owner() {
        return Err(no_safe_slice(GenericG0IngressRejectReasonV1::OwnerMismatch));
    }
    if context.origin() != input.function().function_origin() {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::OriginMismatch,
        ));
    }
    if context.source_kind() != input.function().source_kind() {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::SourceKindMismatch,
        ));
    }
    if !input
        .function()
        .source_site_inventory()
        .contains_statement(context.loop_site())
    {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::LoopSiteMissing,
        ));
    }
    let (_, scope_region) = input
        .function()
        .resolved_loop_source_context(context.loop_site())
        .map_err(|_| no_safe_slice(GenericG0IngressRejectReasonV1::LoopSiteMissing))?;
    if scope_region != context.scope_region() {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::ScopeRegionMismatch,
        ));
    }
    let source = input
        .function()
        .resolved_loop_source(context.loop_site())
        .map_err(|_| no_safe_slice(GenericG0IngressRejectReasonV1::LoopSiteMissing))?;
    if !source.frame_key().matches(context.frame()) {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::LoopFrameMismatch,
        ));
    }
    // The S4 source-bound Core already consumed the resolver forest. Reissuing
    // that forest here would create a second caller-zero source boundary; the
    // ingress only checks that each carried anchor stays under the exact root
    // site and retains the sealed product as the forest authority.
    for effect in product.core().effect_relations() {
        match effect.anchor() {
            LoopBindingEffectAnchorV1::Expr(site) => {
                if site.owner() != input.owner() {
                    return Err(no_safe_slice(
                        GenericG0IngressRejectReasonV1::SourceAnchorOwnerMismatch,
                    ));
                }
                if !input
                    .function()
                    .source_site_inventory()
                    .contains_expression(site.site())
                {
                    return Err(no_safe_slice(
                        GenericG0IngressRejectReasonV1::SourceExpressionMissing,
                    ));
                }
            }
            LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                owner, source_loop, ..
            } => {
                if *owner != input.owner() {
                    return Err(no_safe_slice(
                        GenericG0IngressRejectReasonV1::SourceAnchorOwnerMismatch,
                    ));
                }
                if !input
                    .function()
                    .source_site_inventory()
                    .contains_statement(source_loop)
                {
                    return Err(no_safe_slice(
                        GenericG0IngressRejectReasonV1::SourceLoopMissing,
                    ));
                }
                if !source_loop
                    .node()
                    .segments()
                    .starts_with(context.loop_site().node().segments())
                {
                    return Err(no_safe_slice(
                        GenericG0IngressRejectReasonV1::SourceForestMismatch,
                    ));
                }
            }
        }
    }

    let after = product.after();
    if after.owner() != input.owner() {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::TailOwnerMismatch,
        ));
    }
    if !after.frame().matches(context.frame()) {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::TailFrameMismatch,
        ));
    }
    if after.post_loop_read().binding().owner() != input.owner() {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::TailBindingOwnerMismatch,
        ));
    }
    if !input
        .function()
        .source_site_inventory()
        .contains_statement(after.post_loop_read().statement())
    {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::TailStatementMissing,
        ));
    }
    if !input
        .function()
        .source_site_inventory()
        .contains_expression(after.post_loop_read().value())
    {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::TailValueMissing,
        ));
    }
    if after.return_abi() != ExactTrivialReturnAbiV1::I64 {
        return Err(no_safe_slice(
            GenericG0IngressRejectReasonV1::TailAbiMismatch,
        ));
    }
    Ok(())
}

fn no_safe_slice(reason: GenericG0IngressRejectReasonV1) -> GenericG0IngressRejectV1 {
    GenericG0IngressRejectV1::NoSafeSlice(reason)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::loop_recipe_contract::issue_generic_g0_recipe_demand_v1;
    use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;

    fn product_for_test() -> VerifiedGenericRecipeProductG0 {
        let (_, selection) = generic_source_unit_and_selection_for_test();
        let demand = issue_generic_g0_recipe_demand_v1(selection).expect("G0 demand");
        produce_generic_g0_recipe_v1(demand).expect("G0 product")
    }

    #[test]
    fn exact_input_prepares_all_fifteen_rows_and_preserves_g0_tail() {
        let (unit, selection) = generic_source_unit_and_selection_for_test();
        let input = unit.root_function_input().expect("root input");
        let product = produce_generic_g0_recipe_v1(
            issue_generic_g0_recipe_demand_v1(selection).expect("G0 demand"),
        )
        .expect("G0 product");
        let prepared =
            issue_generic_g0_loop_ingress_v1(Some(input), product).expect("exact G0 ingress");
        assert_eq!(prepared.input().entries().len(), 2);
        assert_eq!(prepared.program().coverage().operation_count(), 15);
        let items = prepared
            .program()
            .schedule()
            .iter()
            .map(|row| row.item().raw())
            .collect::<Vec<_>>();
        assert_eq!(
            items,
            vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
        );
        assert_eq!(prepared.tail().return_abi(), ExactTrivialReturnAbiV1::I64);
        assert_eq!(prepared.tail().owner(), prepared.input().owner());
    }

    #[test]
    fn missing_input_is_a_typed_no_safe_slice() {
        let error = issue_generic_g0_loop_ingress_v1(None, product_for_test()).unwrap_err();
        assert_eq!(
            error,
            GenericG0IngressRejectV1::NoSafeSlice(GenericG0IngressRejectReasonV1::MissingInput)
        );
    }

    #[test]
    fn foreign_resolver_input_is_rejected_before_prepare_all() {
        let (unit_a, selection_a) = generic_source_unit_and_selection_for_test();
        let (unit_b, _) = generic_source_unit_and_selection_for_test();
        let input_b = unit_b.root_function_input().expect("foreign input");
        let product_a = produce_generic_g0_recipe_v1(
            issue_generic_g0_recipe_demand_v1(selection_a).expect("G0 demand"),
        )
        .expect("G0 product");
        let error = issue_generic_g0_loop_ingress_v1(Some(input_b), product_a).unwrap_err();
        assert_eq!(
            error,
            GenericG0IngressRejectV1::NoSafeSlice(GenericG0IngressRejectReasonV1::OwnerMismatch)
        );
        drop(unit_a);
    }

    #[test]
    fn split_tail_is_not_relabelled_as_neutral_after() {
        let product = product_for_test();
        let (operation_effect, context, continuation, tail, _) =
            product.into_physical_parts_for_test();
        assert_eq!(continuation.after().loop_key().raw(), 0);
        assert_eq!(continuation.after().binding().raw(), 1);
        assert_eq!(tail.return_abi(), ExactTrivialReturnAbiV1::I64);
        assert_eq!(tail.owner(), operation_effect.core().owner());
        assert_eq!(tail.owner(), context.owner());
    }
}
