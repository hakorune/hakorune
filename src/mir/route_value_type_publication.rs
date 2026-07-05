//! Shared MIR value-type publication policy for route metadata.
//!
//! This is deliberately smaller than route selection. It only maps stable
//! route return shapes and helper parameter publication policy to MIR value
//! type facts.

use crate::mir::MirType;

pub(crate) const STRING_HELPERS_TO_I64: &str = "StringHelpers.to_i64/1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HelperParamTypePublicationPolicy {
    PublishFromObservation,
    PolymorphicInputDoNotPublishFromSingleObservation,
}

pub(crate) fn helper_param_type_publication_policy(
    helper_id: &str,
    param_index: usize,
) -> HelperParamTypePublicationPolicy {
    if helper_id == STRING_HELPERS_TO_I64 && param_index == 0 {
        return HelperParamTypePublicationPolicy::PolymorphicInputDoNotPublishFromSingleObservation;
    }
    HelperParamTypePublicationPolicy::PublishFromObservation
}

pub(crate) fn route_return_shape_value_type(return_shape: Option<&str>) -> Option<MirType> {
    match return_shape {
        Some("ScalarI64") | Some("scalar_i64") | Some("void_sentinel_i64_zero") => {
            Some(MirType::Integer)
        }
        Some("string_handle") | Some("string_handle_or_null") => {
            Some(MirType::Box("StringBox".to_string()))
        }
        Some("array_handle") => Some(MirType::Box("ArrayBox".to_string())),
        Some("map_handle") => Some(MirType::Box("MapBox".to_string())),
        Some("object_handle") | Some("mixed_runtime_i64_or_handle") | None => None,
        Some(_) => None,
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
    fn to_i64_param0_is_polymorphic_input_policy() {
        assert_eq!(
            helper_param_type_publication_policy(STRING_HELPERS_TO_I64, 0),
            HelperParamTypePublicationPolicy::PolymorphicInputDoNotPublishFromSingleObservation
        );
        assert_eq!(
            helper_param_type_publication_policy(STRING_HELPERS_TO_I64, 1),
            HelperParamTypePublicationPolicy::PublishFromObservation
        );
    }
}
