/*!
 * Sum placement layout pilot.
 *
 * This chooses the LLVM-side local aggregate payload lane for selected local sum
 * routes. It is still inspection-only metadata; lowering consumes it in later
 * slices.
 */

use super::{
    sum_placement_selection::{SumPlacementPath, SumPlacementSelection},
    thin_entry::ThinEntrySurface,
    BasicBlockId, MirFunction, MirInstruction, MirModule, MirType, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SumLocalAggregateLayout {
    TagOnly,
    TagI64Payload,
    TagF64Payload,
    TagHandlePayload,
}

impl std::fmt::Display for SumLocalAggregateLayout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::TagOnly => f.write_str("tag_only"),
            Self::TagI64Payload => f.write_str("tag_i64_payload"),
            Self::TagF64Payload => f.write_str("tag_f64_payload"),
            Self::TagHandlePayload => f.write_str("tag_handle_payload"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumPlacementLayout {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub value: Option<ValueId>,
    pub surface: ThinEntrySurface,
    pub subject: String,
    pub source_sum: Option<ValueId>,
    pub layout: SumLocalAggregateLayout,
    pub reason: String,
}

impl SumPlacementLayout {
    pub fn summary(&self) -> String {
        let value_suffix = self
            .value
            .map(|value| format!(" value=%{}", value.as_u32()))
            .unwrap_or_default();
        let source_suffix = self
            .source_sum
            .map(|value| format!(" source_sum=%{}", value.as_u32()))
            .unwrap_or_default();
        format!(
            "bb{}#{} {} {} layout={}{}{} reason={}",
            self.block.as_u32(),
            self.instruction_index,
            self.surface,
            self.subject,
            self.layout,
            value_suffix,
            source_suffix,
            self.reason
        )
    }
}

pub fn refresh_module_sum_placement_layouts(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_sum_placement_layouts(function);
    }
}

pub fn refresh_function_sum_placement_layouts(function: &mut MirFunction) {
    let selections = function.metadata.sum_placement_selections.clone();
    let mut layouts = Vec::new();
    for selection in selections
        .iter()
        .filter(|selection| selection.selected_path == SumPlacementPath::LocalAggregate)
    {
        let Some(layout) = bind_layout(function, selection) else {
            continue;
        };
        layouts.push(layout);
    }
    layouts.sort_by_key(|layout| (layout.block.as_u32(), layout.instruction_index));
    function.metadata.sum_placement_layouts = layouts;
}

fn bind_layout(
    function: &MirFunction,
    selection: &SumPlacementSelection,
) -> Option<SumPlacementLayout> {
    let block = function.blocks.get(&selection.block)?;
    let inst = block.instructions.get(selection.instruction_index)?;
    let (layout, reason) = match (selection.surface, inst) {
        (
            ThinEntrySurface::VariantMake,
            MirInstruction::VariantMake {
                payload_type,
                payload,
                ..
            },
        ) => {
            let layout = layout_from_payload_type(payload_type.as_ref(), payload.is_some());
            let reason = reason_for_layout(layout, selection.surface, payload_type.as_ref());
            (layout, reason)
        }
        (ThinEntrySurface::VariantProject, MirInstruction::VariantProject { payload_type, .. }) => {
            let layout = layout_from_payload_type(payload_type.as_ref(), true);
            let reason = reason_for_layout(layout, selection.surface, payload_type.as_ref());
            (layout, reason)
        }
        _ => return None,
    };
    Some(SumPlacementLayout {
        block: selection.block,
        instruction_index: selection.instruction_index,
        value: selection.value,
        surface: selection.surface,
        subject: selection.subject.clone(),
        source_sum: selection.source_sum,
        layout,
        reason,
    })
}

fn layout_from_payload_type(
    payload_type: Option<&MirType>,
    has_payload: bool,
) -> SumLocalAggregateLayout {
    if !has_payload {
        return SumLocalAggregateLayout::TagOnly;
    }
    match payload_type {
        Some(MirType::Integer) | Some(MirType::Bool) => SumLocalAggregateLayout::TagI64Payload,
        Some(MirType::Float) => SumLocalAggregateLayout::TagF64Payload,
        Some(MirType::Void) => SumLocalAggregateLayout::TagOnly,
        Some(MirType::String)
        | Some(MirType::Box(_))
        | Some(MirType::Array(_))
        | Some(MirType::Future(_))
        | Some(MirType::WeakRef)
        | Some(MirType::Unknown)
        | None => SumLocalAggregateLayout::TagHandlePayload,
    }
}

fn reason_for_layout(
    layout: SumLocalAggregateLayout,
    surface: ThinEntrySurface,
    payload_type: Option<&MirType>,
) -> String {
    let payload_text = payload_type
        .map(|ty| format!("{:?}", ty))
        .unwrap_or_else(|| "None".to_string());
    match layout {
        SumLocalAggregateLayout::TagOnly => format!(
            "{} keeps only the tag lane because this selected sum route has no payload (payload_type={})",
            surface, payload_text
        ),
        SumLocalAggregateLayout::TagI64Payload => format!(
            "{} uses a tag+i64 payload lane; bool rides the same canonical i64 lane in this local aggregate layout (payload_type={})",
            surface, payload_text
        ),
        SumLocalAggregateLayout::TagF64Payload => format!(
            "{} uses a tag+f64 payload lane for the selected local aggregate layout (payload_type={})",
            surface, payload_text
        ),
        SumLocalAggregateLayout::TagHandlePayload => format!(
            "{} uses a tag+handle payload lane so non-scalar or not-yet-proved payloads can stay on the selected local aggregate route without changing canonical MIR (payload_type={})",
            surface, payload_text
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        EffectMask, FunctionSignature, SumPlacementSelection, ThinEntrySurface, ValueId,
    };

    #[test]
    fn refresh_function_chooses_i64_and_float_layouts_for_selected_sum_routes() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        let block = function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block exists");
        block.add_instruction(MirInstruction::VariantMake {
            dst: ValueId::new(1),
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(ValueId::new(10)),
            payload_type: Some(MirType::Integer),
        });
        block.add_instruction(MirInstruction::VariantProject {
            dst: ValueId::new(2),
            value: ValueId::new(1),
            enum_name: "FloatOption".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload_type: Some(MirType::Float),
        });
        function.metadata.sum_placement_selections = vec![
            SumPlacementSelection {
                block: BasicBlockId::new(0),
                instruction_index: 0,
                value: Some(ValueId::new(1)),
                surface: ThinEntrySurface::VariantMake,
                subject: "Option::Some".to_string(),
                source_sum: None,
                manifest_row: "variant_make.local_aggregate",
                selected_path: SumPlacementPath::LocalAggregate,
                reason: "selected".to_string(),
            },
            SumPlacementSelection {
                block: BasicBlockId::new(0),
                instruction_index: 1,
                value: Some(ValueId::new(2)),
                surface: ThinEntrySurface::VariantProject,
                subject: "FloatOption::Some".to_string(),
                source_sum: Some(ValueId::new(1)),
                manifest_row: "variant_project.local_aggregate",
                selected_path: SumPlacementPath::LocalAggregate,
                reason: "selected".to_string(),
            },
        ];

        refresh_function_sum_placement_layouts(&mut function);

        assert!(function
            .metadata
            .sum_placement_layouts
            .iter()
            .any(|layout| {
                layout.surface == ThinEntrySurface::VariantMake
                    && layout.layout == SumLocalAggregateLayout::TagI64Payload
            }));
        assert!(function
            .metadata
            .sum_placement_layouts
            .iter()
            .any(|layout| {
                layout.surface == ThinEntrySurface::VariantProject
                    && layout.layout == SumLocalAggregateLayout::TagF64Payload
                    && layout.source_sum == Some(ValueId::new(1))
            }));
    }

    #[test]
    fn refresh_function_chooses_handle_layout_for_unknown_payloads() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .get_block_mut(BasicBlockId::new(0))
            .expect("entry block exists")
            .add_instruction(MirInstruction::VariantMake {
                dst: ValueId::new(3),
                enum_name: "Option".to_string(),
                variant: "Some".to_string(),
                tag: 1,
                payload: Some(ValueId::new(11)),
                payload_type: None,
            });
        function.metadata.sum_placement_selections = vec![SumPlacementSelection {
            block: BasicBlockId::new(0),
            instruction_index: 0,
            value: Some(ValueId::new(3)),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            manifest_row: "variant_make.local_aggregate",
            selected_path: SumPlacementPath::LocalAggregate,
            reason: "selected".to_string(),
        }];

        refresh_function_sum_placement_layouts(&mut function);

        assert_eq!(function.metadata.sum_placement_layouts.len(), 1);
        assert_eq!(
            function.metadata.sum_placement_layouts[0].layout,
            SumLocalAggregateLayout::TagHandlePayload
        );
    }
}
