use crate::mir::function::TypedObjectFieldStorage;

use std::collections::BTreeMap;

pub(super) type FieldKey = (String, String);
pub(super) type ParamKey = (String, usize);
pub(super) type FieldBoxOriginMap = BTreeMap<FieldKey, BoxOriginInference>;
pub(super) type ParamBoxOriginMap = BTreeMap<ParamKey, BoxOriginInference>;
pub(super) type CollectionElementStorageMap = BTreeMap<FieldKey, FieldStorageInference>;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FieldStorageInference {
    Known(TypedObjectFieldStorage),
    Conflict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum BoxOriginInference {
    Known(String),
    Conflict,
}
