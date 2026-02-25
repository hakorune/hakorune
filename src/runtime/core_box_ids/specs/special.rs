use super::CoreMethodSpec;
use crate::runtime::core_box_ids::{CoreBoxId, CoreMethodId};

pub(super) const SPECS: &[CoreMethodSpec] = &[
    // ResultBox methods (QMark 対応)
    CoreMethodSpec {
        id: CoreMethodId::ResultIsOk,
        box_id: CoreBoxId::Result,
        name: "isOk",
        arity: 0,
        return_type_name: "BoolBox",
        is_pure: true,
        allowed_in_condition: true,
        allowed_in_init: true,
        vtable_slot: None,
    },
    CoreMethodSpec {
        id: CoreMethodId::ResultGetValue,
        box_id: CoreBoxId::Result,
        name: "getValue",
        arity: 0,
        return_type_name: "Unknown",
        is_pure: true,
        allowed_in_condition: false,
        allowed_in_init: true,
        vtable_slot: None,
    },
];
