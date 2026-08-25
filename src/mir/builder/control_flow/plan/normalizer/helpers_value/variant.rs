use super::MirBuilder;
use crate::mir::MirType;

pub(super) fn enum_payload_mir_type(raw: &str) -> Option<MirType> {
    if raw.is_empty()
        || raw
            .chars()
            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
    {
        return None;
    }
    Some(MirBuilder::parse_type_name_to_mir(raw))
}

pub(super) fn runtime_variant_box_name(enum_name: &str) -> String {
    format!("__hako_sum_{}", enum_name)
}
