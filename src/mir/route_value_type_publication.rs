//! Shared MIR value-type publication policy for route metadata.
//!
//! This is deliberately smaller than route selection. It only maps stable
//! route return shapes and helper parameter publication policy to MIR value
//! type facts.

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirModule, MirType, ValueId};

pub(crate) const BOX_HELPERS_EXPECT_I64: &str = "BoxHelpers.expect_i64/2";
pub(crate) const BOX_HELPERS_IS_ARRAY: &str = "BoxHelpers.is_array/1";
pub(crate) const BOX_HELPERS_IS_MAP: &str = "BoxHelpers.is_map/1";
pub(crate) const BOX_HELPERS_VALUE_I64: &str = "BoxHelpers.value_i64/1";
pub(crate) const MIR_JSON_EMIT_BOX_EXPECT_I64: &str = "MirJsonEmitBox._expect_i64/2";
pub(crate) const MIR_SCHEMA_BOX_EXPECT_I64: &str = "MirSchemaBox._expect_i64/2";
pub(crate) const STRING_HELPERS_INT_TO_STR: &str = "StringHelpers.int_to_str/1";
pub(crate) const STRING_HELPERS_TO_I64: &str = "StringHelpers.to_i64/1";

pub(crate) const POLYMORPHIC_HELPER_PARAM0_INPUTS: &[&str] = &[
    STRING_HELPERS_TO_I64,
    STRING_HELPERS_INT_TO_STR,
    BOX_HELPERS_IS_ARRAY,
    BOX_HELPERS_IS_MAP,
    BOX_HELPERS_VALUE_I64,
    BOX_HELPERS_EXPECT_I64,
    MIR_JSON_EMIT_BOX_EXPECT_I64,
    MIR_SCHEMA_BOX_EXPECT_I64,
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HelperParamTypePublicationPolicy {
    PublishFromObservation,
    PolymorphicInputDoNotPublishFromSingleObservation,
}

pub(crate) fn helper_param_type_publication_policy(
    helper_id: &str,
    param_index: usize,
) -> HelperParamTypePublicationPolicy {
    if param_index == 0 && POLYMORPHIC_HELPER_PARAM0_INPUTS.contains(&helper_id) {
        return HelperParamTypePublicationPolicy::PolymorphicInputDoNotPublishFromSingleObservation;
    }
    HelperParamTypePublicationPolicy::PublishFromObservation
}

pub(crate) fn route_return_shape_value_type(return_shape: Option<&str>) -> Option<MirType> {
    match return_shape {
        Some("ScalarI64")
        | Some("scalar_i64")
        | Some("scalar_i64_or_missing_zero")
        | Some("void_sentinel_i64_zero") => Some(MirType::Integer),
        Some("string_handle") | Some("string_handle_or_null") => {
            Some(MirType::Box("StringBox".to_string()))
        }
        Some("scalar_bool") => Some(MirType::Bool),
        Some("c_void") | Some("no_payload") => None,
        Some("array_handle") => Some(MirType::Box("ArrayBox".to_string())),
        Some("map_handle") => Some(MirType::Box("MapBox".to_string())),
        Some("object_handle") | Some("mixed_runtime_i64_or_handle") | None => None,
        Some(_) => None,
    }
}

/// Publish a snapshot of Copy/PHI facts in scan order before the next pass.
pub(crate) fn propagate_box_value_types(
    module: &mut MirModule,
    refine_placeholder: impl Fn(&MirType, &MirType) -> bool,
) -> bool {
    let mut changed = false;
    for function in module.functions.values_mut() {
        for _ in 0..function.blocks.len().saturating_mul(4).max(4) {
            let mut pending = Vec::<(ValueId, MirType)>::new();
            for block in function.blocks.values() {
                for instruction in &block.instructions {
                    match instruction {
                        MirInstruction::Copy { dst, src } => {
                            if value_has_concrete_box_type(function, *dst) {
                                continue;
                            }
                            if let Some(ty) = concrete_box_value_type(function, *src) {
                                pending.push((*dst, ty));
                            }
                        }
                        MirInstruction::Phi { dst, inputs, .. } => {
                            if value_has_concrete_box_type(function, *dst) {
                                continue;
                            }
                            if let Some(ty) = concrete_phi_box_value_type(function, inputs) {
                                pending.push((*dst, ty));
                            }
                        }
                        _ => {}
                    }
                }
            }
            if pending.is_empty() {
                break;
            }

            let mut pass_changed = false;
            for (value, ty) in pending {
                match function.metadata.value_types.get(&value) {
                    Some(existing) if existing == &ty => {}
                    Some(existing) if refine_placeholder(existing, &ty) => {
                        function.metadata.value_types.insert(value, ty);
                        pass_changed = true;
                    }
                    Some(MirType::Unknown) | None => {
                        function.metadata.value_types.insert(value, ty);
                        pass_changed = true;
                    }
                    Some(_) => {}
                }
            }
            if !pass_changed {
                break;
            }
            changed = true;
        }
    }
    changed
}

fn concrete_phi_box_value_type(
    function: &MirFunction,
    inputs: &[(BasicBlockId, ValueId)],
) -> Option<MirType> {
    let mut inferred = None::<MirType>;
    for (_block, value) in inputs {
        let ty = concrete_box_value_type(function, *value)?;
        inferred = match inferred {
            None => Some(ty),
            Some(existing) if existing == ty => Some(existing),
            Some(_) => return None,
        };
    }
    inferred
}

fn value_has_concrete_box_type(function: &MirFunction, value: ValueId) -> bool {
    concrete_box_value_type(function, value).is_some()
}

fn concrete_box_value_type(function: &MirFunction, value: ValueId) -> Option<MirType> {
    concrete_box_name_from_value_type(function.metadata.value_types.get(&value)).map(MirType::Box)
}

pub(crate) fn concrete_box_name_from_value_type(ty: Option<&MirType>) -> Option<String> {
    match ty {
        Some(MirType::Box(name)) => Some(name.clone()),
        Some(MirType::String) => Some("StringBox".to_string()),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn route_return_shape_publication_contract() {
        assert_eq!(
            route_return_shape_value_type(Some("ScalarI64")),
            Some(MirType::Integer)
        );
        assert_eq!(
            route_return_shape_value_type(Some("scalar_i64")),
            Some(MirType::Integer)
        );
        assert_eq!(
            route_return_shape_value_type(Some("scalar_i64_or_missing_zero")),
            Some(MirType::Integer)
        );
        assert_eq!(
            route_return_shape_value_type(Some("string_handle")),
            Some(MirType::Box("StringBox".to_string()))
        );
        assert_eq!(route_return_shape_value_type(Some("object_handle")), None);
        assert_eq!(
            route_return_shape_value_type(Some("mixed_runtime_i64_or_handle")),
            None
        );
    }

    #[test]
    fn polymorphic_helper_param0_inputs_do_not_publish_from_single_observation() {
        for helper_id in POLYMORPHIC_HELPER_PARAM0_INPUTS {
            assert_eq!(
                helper_param_type_publication_policy(helper_id, 0),
                HelperParamTypePublicationPolicy::PolymorphicInputDoNotPublishFromSingleObservation,
                "helper_id={helper_id}"
            );
        }
        assert_eq!(
            helper_param_type_publication_policy(STRING_HELPERS_TO_I64, 1),
            HelperParamTypePublicationPolicy::PublishFromObservation
        );
    }
}
