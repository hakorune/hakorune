use super::*;
use crate::instance_v2::InstanceBox;
use std::sync::Arc;

pub(super) const ENUM_TAG_FIELD: &str = "__variant_tag";
pub(super) const ENUM_PAYLOAD_FIELD: &str = "__variant_payload";

pub(super) fn runtime_variant_box_name(enum_name: &str) -> String {
    format!("__NyVariant_{}", enum_name)
}

pub(super) fn store_sum_payload(
    this: &mut MirInterpreter,
    runtime_sum: &Arc<dyn crate::box_trait::NyashBox>,
    payload_value: VMValue,
) {
    this.set_object_field(
        runtime_sum_storage_key(runtime_sum),
        ENUM_PAYLOAD_FIELD.to_string(),
        payload_value,
    );
}

pub(super) fn read_sum_payload(
    this: &MirInterpreter,
    runtime_sum: &Arc<dyn crate::box_trait::NyashBox>,
    instance: &InstanceBox,
) -> Option<VMValue> {
    this.get_object_field(runtime_sum_storage_key(runtime_sum), ENUM_PAYLOAD_FIELD)
        .or_else(|| {
            instance
                .get_field(ENUM_PAYLOAD_FIELD)
                .map(|payload| VMValue::from_nyash_box(payload.share_box()))
        })
}

pub(super) fn read_variant_tag(instance: &InstanceBox) -> Option<u32> {
    match instance.get_field_ng(ENUM_TAG_FIELD) {
        Some(crate::value::NyashValue::Integer(value)) => u32::try_from(value).ok(),
        _ => None,
    }
}

pub(super) fn validate_variant_tag(
    this: &MirInterpreter,
    enum_name: &str,
    tag: u32,
) -> Result<(), VMError> {
    let decl: &crate::mir::MirEnumDecl = this.enum_decls.get(enum_name).ok_or_else(|| {
        this.err_invalid(format!(
            "[freeze:contract][vm/sum:meta] missing enum declaration for {}",
            enum_name
        ))
    })?;
    if usize::try_from(tag)
        .ok()
        .filter(|idx| *idx < decl.variants.len())
        .is_none()
    {
        return Err(this.err_invalid(format!(
            "[freeze:contract][vm/sum:tag] {} tag {} is out of range (variants={})",
            enum_name,
            tag,
            decl.variants.len()
        )));
    }
    Ok(())
}

fn runtime_sum_storage_key(box_ref: &Arc<dyn crate::box_trait::NyashBox>) -> u64 {
    let ptr = Arc::as_ptr(box_ref) as *const ();
    ptr as usize as u64
}
