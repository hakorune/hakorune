/*!
 * Folded agg_local scalarization owner seam.
 *
 * This module combines the landed sum-placement, thin-entry, and storage-class
 * pilot scaffolds into one generic read-only route inventory. It does not
 * lower or rewrite MIR; it only owns the folded view that later code and JSON
 * consumers can read.
 */

use super::{
    storage_class::StorageClass,
    sum_placement_layout::SumLocalAggregateLayout,
    thin_entry::{ThinEntryPreferredEntry, ThinEntrySurface, ThinEntryValueClass},
    thin_entry_selection::ThinEntrySelection,
    BasicBlockId, MirFunction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AggLocalScalarizationKind {
    SumLocalLayout(SumLocalAggregateLayout),
    UserBoxLocalBody(ThinEntryValueClass),
    TypedSlotStorage(StorageClass),
}

impl std::fmt::Display for AggLocalScalarizationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SumLocalLayout(layout) => write!(f, "sum_local_layout({layout})"),
            Self::UserBoxLocalBody(value_class) => {
                write!(f, "user_box_local_body({value_class})")
            }
            Self::TypedSlotStorage(storage_class) => {
                write!(f, "typed_slot_storage({storage_class})")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AggLocalScalarizationRoute {
    pub block: Option<BasicBlockId>,
    pub instruction_index: Option<usize>,
    pub value: Option<ValueId>,
    pub subject: String,
    pub kind: AggLocalScalarizationKind,
    pub reason: String,
}

impl AggLocalScalarizationRoute {
    pub fn summary(&self) -> String {
        let block_suffix = self
            .block
            .map(|block| format!(" bb{}", block.as_u32()))
            .unwrap_or_else(|| " module".to_string());
        let instruction_suffix = self
            .instruction_index
            .map(|index| format!("#{index}"))
            .unwrap_or_default();
        let value_suffix = self
            .value
            .map(|value| format!(" value=%{}", value.as_u32()))
            .unwrap_or_default();
        format!(
            "{}{} {} kind={}{} reason={}",
            block_suffix, instruction_suffix, self.subject, self.kind, value_suffix, self.reason
        )
    }
}

pub fn refresh_module_agg_local_scalarization_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_agg_local_scalarization_routes(function);
    }
}

pub fn refresh_function_agg_local_scalarization_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();

    collect_sum_layout_routes(function, &mut routes);
    collect_user_box_local_body_routes(function, &mut routes);
    collect_typed_slot_routes(function, &mut routes);

    routes.sort_by_key(route_sort_key);
    function.metadata.agg_local_scalarization_routes = routes;
}

fn collect_sum_layout_routes(function: &MirFunction, routes: &mut Vec<AggLocalScalarizationRoute>) {
    routes.extend(
        function
            .metadata
            .sum_placement_layouts
            .iter()
            .map(|layout| AggLocalScalarizationRoute {
                block: Some(layout.block),
                instruction_index: Some(layout.instruction_index),
                value: layout.value,
                subject: layout.subject.clone(),
                kind: AggLocalScalarizationKind::SumLocalLayout(layout.layout),
                reason: layout.reason.clone(),
            }),
    );
}

fn collect_user_box_local_body_routes(
    function: &MirFunction,
    routes: &mut Vec<AggLocalScalarizationRoute>,
) {
    routes.extend(
        function
            .metadata
            .thin_entry_selections
            .iter()
            .filter(|selection| is_inline_scalar_user_box_local_body(selection))
            .map(|selection| AggLocalScalarizationRoute {
                block: Some(selection.block),
                instruction_index: Some(selection.instruction_index),
                value: selection.value,
                subject: selection.subject.clone(),
                kind: AggLocalScalarizationKind::UserBoxLocalBody(selection.value_class),
                reason: selection.reason.clone(),
            }),
    );
}

fn collect_typed_slot_routes(function: &MirFunction, routes: &mut Vec<AggLocalScalarizationRoute>) {
    routes.extend(function.metadata.value_storage_classes.iter().map(|(value, storage_class)| {
        AggLocalScalarizationRoute {
            block: None,
            instruction_index: None,
            value: Some(*value),
            subject: format!("value%{}", value.as_u32()),
            kind: AggLocalScalarizationKind::TypedSlotStorage(*storage_class),
            reason: format!(
                "value%{} already has storage class {} and can stay visible in the folded agg_local inventory",
                value.as_u32(),
                storage_class
            ),
        }
    }));
}

fn is_inline_scalar_user_box_local_body(selection: &ThinEntrySelection) -> bool {
    matches!(
        selection.surface,
        ThinEntrySurface::UserBoxFieldGet | ThinEntrySurface::UserBoxFieldSet
    ) && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
        && matches!(
            selection.value_class,
            ThinEntryValueClass::InlineI64
                | ThinEntryValueClass::InlineBool
                | ThinEntryValueClass::InlineF64
        )
}

fn route_sort_key(route: &AggLocalScalarizationRoute) -> (u8, u32, u32, u32, String) {
    let kind_rank = match route.kind {
        AggLocalScalarizationKind::SumLocalLayout(_) => 0,
        AggLocalScalarizationKind::UserBoxLocalBody(_) => 1,
        AggLocalScalarizationKind::TypedSlotStorage(_) => 2,
    };
    let block_rank = route.block.map(|block| block.as_u32()).unwrap_or(u32::MAX);
    let instruction_rank = route
        .instruction_index
        .map(|index| index as u32)
        .unwrap_or(u32::MAX);
    let value_rank = route.value.map(|value| value.as_u32()).unwrap_or(u32::MAX);
    (
        kind_rank,
        block_rank,
        instruction_rank,
        value_rank,
        route.subject.clone(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::thin_entry::{
        ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryPreferredEntry, ThinEntrySurface,
        ThinEntryValueClass,
    };
    use crate::mir::thin_entry_selection::{ThinEntrySelection, ThinEntrySelectionState};
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirType, SumLocalAggregateLayout, ValueId,
    };

    #[test]
    fn refresh_function_collects_folded_agg_local_routes() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.metadata.sum_placement_layouts = vec![crate::mir::SumPlacementLayout {
            block: BasicBlockId::new(0),
            instruction_index: 0,
            value: Some(ValueId::new(1)),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            layout: SumLocalAggregateLayout::TagI64Payload,
            reason: "selected local aggregate uses tag+i64 payload lane".to_string(),
        }];
        function
            .metadata
            .thin_entry_selections
            .push(ThinEntrySelection {
                block: BasicBlockId::new(0),
                instruction_index: 1,
                value: Some(ValueId::new(2)),
                surface: ThinEntrySurface::UserBoxFieldGet,
                subject: "Point.x".to_string(),
                manifest_row: "user_box_field_get.inline_scalar",
                selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                state: ThinEntrySelectionState::AlreadySatisfied,
                current_carrier: ThinEntryCurrentCarrier::BackendTyped,
                value_class: ThinEntryValueClass::InlineI64,
                demand: ThinEntryDemand::InlineScalar,
                reason: "typed field read stays on thin internal scalar lane".to_string(),
            });
        function
            .metadata
            .value_storage_classes
            .insert(ValueId::new(3), StorageClass::InlineBool);

        refresh_function_agg_local_scalarization_routes(&mut function);

        assert_eq!(function.metadata.agg_local_scalarization_routes.len(), 3);
        assert!(matches!(
            function.metadata.agg_local_scalarization_routes[0].kind,
            AggLocalScalarizationKind::SumLocalLayout(SumLocalAggregateLayout::TagI64Payload)
        ));
        assert!(matches!(
            function.metadata.agg_local_scalarization_routes[1].kind,
            AggLocalScalarizationKind::UserBoxLocalBody(ThinEntryValueClass::InlineI64)
        ));
        assert!(matches!(
            function.metadata.agg_local_scalarization_routes[2].kind,
            AggLocalScalarizationKind::TypedSlotStorage(StorageClass::InlineBool)
        ));
    }
}
