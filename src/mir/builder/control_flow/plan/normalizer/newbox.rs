use crate::mir::builder::MirBuilder;
use crate::mir::MirType;
use crate::mir::ValueId;

pub(super) fn record_newbox_metadata(builder: &mut MirBuilder, value_id: ValueId, class: &str) {
    let class_name = class.to_string();
    builder
        .type_ctx
        .value_types
        .insert(value_id, MirType::Box(class_name.clone()));
    builder
        .type_ctx
        .value_origin_newbox
        .insert(value_id, class_name.clone());
    builder
        .comp_ctx
        .type_registry
        .record_newbox(value_id, class_name.clone());
    builder
        .comp_ctx
        .type_registry
        .record_type(value_id, MirType::Box(class_name));
}
