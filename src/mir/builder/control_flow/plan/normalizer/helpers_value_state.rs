//! Builder-state helpers shared by port-driven value normalization.

use crate::mir::builder::MirBuilder;
use crate::mir::{MirType, ValueId};
use std::collections::BTreeMap;

impl super::PlanNormalizer {
    pub(in crate::mir::builder) fn declared_field_type_for_base(
        builder: &MirBuilder,
        base: ValueId,
        field: &str,
    ) -> Option<MirType> {
        builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&base)
            .and_then(|box_name| builder.comp_ctx.declared_field_type_name(box_name, field))
            .map(MirBuilder::parse_type_name_to_mir)
    }

    pub(in crate::mir::builder) fn allocate_field_result(
        builder: &mut MirBuilder,
        declared_type: &Option<MirType>,
    ) -> ValueId {
        match declared_type {
            Some(ty) => {
                let value_id = builder.alloc_typed(ty.clone());
                if let MirType::Box(class_name) = ty {
                    builder
                        .function_state
                        .type_ctx
                        .value_origin_newbox
                        .insert(value_id, class_name.clone());
                }
                value_id
            }
            None => {
                let value_id = builder.next_value_id();
                builder
                    .function_state
                    .type_ctx
                    .set_type(value_id, MirType::Unknown);
                value_id
            }
        }
    }

    pub(in crate::mir::builder) fn non_add_arithmetic_result_type(
        builder: &MirBuilder,
        lhs: ValueId,
        rhs: ValueId,
    ) -> MirType {
        let lhs_ty = builder.function_state.type_ctx.get_type(lhs);
        let rhs_ty = builder.function_state.type_ctx.get_type(rhs);
        if matches!(lhs_ty, Some(MirType::Float)) || matches!(rhs_ty, Some(MirType::Float)) {
            MirType::Float
        } else {
            MirType::Integer
        }
    }

    pub(in crate::mir::builder) fn lookup_variable_value(
        builder: &MirBuilder,
        phi_bindings: &BTreeMap<String, ValueId>,
        name: &str,
    ) -> Option<ValueId> {
        let from_map = builder
            .function_state
            .variable_ctx
            .variable_map
            .get(name)
            .copied();
        let from_bindings = phi_bindings.get(name).copied();
        from_bindings.or(from_map)
    }
}
