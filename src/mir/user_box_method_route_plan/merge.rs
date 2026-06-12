use super::{
    BoxOriginInference, FieldBoxOriginKey, FieldBoxOriginMap, ParamBoxOriginKey, ParamBoxOriginMap,
};

pub(super) fn merge_param_box_origin(
    origins: &mut ParamBoxOriginMap,
    key: ParamBoxOriginKey,
    box_name: String,
) -> bool {
    match origins.get(&key) {
        Some(BoxOriginInference::Known(existing)) if existing == &box_name => false,
        Some(BoxOriginInference::Conflict) => false,
        Some(BoxOriginInference::Known(_)) => {
            origins.insert(key, BoxOriginInference::Conflict);
            true
        }
        None => {
            origins.insert(key, BoxOriginInference::Known(box_name));
            true
        }
    }
}

pub(super) fn merge_field_box_origin(
    origins: &mut FieldBoxOriginMap,
    key: FieldBoxOriginKey,
    box_name: String,
) -> bool {
    match origins.get(&key) {
        Some(BoxOriginInference::Known(existing)) if existing == &box_name => false,
        Some(BoxOriginInference::Conflict) => false,
        Some(BoxOriginInference::Known(_)) => {
            origins.insert(key, BoxOriginInference::Conflict);
            true
        }
        None => {
            origins.insert(key, BoxOriginInference::Known(box_name));
            true
        }
    }
}
