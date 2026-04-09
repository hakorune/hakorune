use super::*;
use crate::box_trait::SharedNyashBox;
use crate::instance_v2::InstanceBox;
use crate::mir::{MirEnumDecl, MirEnumVariantDecl, MirType};
use std::collections::HashMap;
use std::sync::Arc;

const SUM_TAG_FIELD: &str = "__sum_tag";
const SUM_PAYLOAD_FIELD: &str = "__sum_payload";

fn runtime_sum_box_name(enum_name: &str) -> String {
    format!("__NySum_{}", enum_name)
}

impl MirInterpreter {
    pub(super) fn handle_sum_make(
        &mut self,
        dst: ValueId,
        enum_name: &str,
        variant: &str,
        tag: u32,
        payload: Option<ValueId>,
        _payload_type: Option<&MirType>,
    ) -> Result<(), VMError> {
        let (expected_tag, variant_decl) = self.resolve_sum_variant(enum_name, variant)?;
        if expected_tag != tag {
            return Err(self.err_invalid(format!(
                "[freeze:contract][vm/sum:make] {}::{} expected tag {} but got {}",
                enum_name, variant, expected_tag, tag
            )));
        }
        let expects_payload = variant_decl.payload_type_name.is_some();
        if expects_payload != payload.is_some() {
            return Err(self.err_invalid(format!(
                "[freeze:contract][vm/sum:make] {}::{} payload mismatch (expects_payload={}, got_payload={})",
                enum_name,
                variant,
                expects_payload,
                payload.is_some()
            )));
        }

        let mut instance = InstanceBox::from_declaration(
            runtime_sum_box_name(enum_name),
            vec![SUM_TAG_FIELD.to_string(), SUM_PAYLOAD_FIELD.to_string()],
            HashMap::new(),
        );
        instance
            .set_field_ng(
                SUM_TAG_FIELD.to_string(),
                crate::value::NyashValue::Integer(tag as i64),
            )
            .map_err(|e| self.err_invalid(format!("[freeze:contract][vm/sum:make] {}", e)))?;
        if let Some(payload_id) = payload {
            let payload_value = self.reg_load(payload_id)?;
            let payload_box = vm_value_to_shared_box(payload_value);
            instance.set_field_dynamic_legacy(SUM_PAYLOAD_FIELD.to_string(), payload_box);
        }
        self.write_reg(dst, VMValue::BoxRef(Arc::new(instance)));
        Ok(())
    }

    pub(super) fn handle_sum_tag(
        &mut self,
        dst: ValueId,
        value: ValueId,
        enum_name: &str,
    ) -> Result<(), VMError> {
        let box_ref = self.load_sum_instance(value, enum_name)?;
        let instance = box_ref
            .as_any()
            .downcast_ref::<InstanceBox>()
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[freeze:contract][vm/sum:value] {} value is not an InstanceBox",
                    enum_name
                ))
            })?;
        let tag = read_sum_tag(instance).ok_or_else(|| {
            self.err_invalid(format!(
                "[freeze:contract][vm/sum:tag] {} missing {}",
                enum_name, SUM_TAG_FIELD
            ))
        })?;
        validate_sum_tag(self, enum_name, tag)?;
        self.write_reg(dst, VMValue::Integer(i64::from(tag)));
        Ok(())
    }

    pub(super) fn handle_sum_project(
        &mut self,
        dst: ValueId,
        value: ValueId,
        enum_name: &str,
        variant: &str,
        tag: u32,
        _payload_type: Option<&MirType>,
    ) -> Result<(), VMError> {
        let (expected_tag, variant_decl) = self.resolve_sum_variant(enum_name, variant)?;
        if expected_tag != tag {
            return Err(self.err_invalid(format!(
                "[freeze:contract][vm/sum:project] {}::{} expected tag {} but got instruction tag {}",
                enum_name, variant, expected_tag, tag
            )));
        }
        if variant_decl.payload_type_name.is_none() {
            return Err(self.err_invalid(format!(
                "[freeze:contract][vm/sum:project] {}::{} has no payload",
                enum_name, variant
            )));
        }

        let box_ref = self.load_sum_instance(value, enum_name)?;
        let instance = box_ref
            .as_any()
            .downcast_ref::<InstanceBox>()
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[freeze:contract][vm/sum:value] {} value is not an InstanceBox",
                    enum_name
                ))
            })?;
        let actual_tag = read_sum_tag(instance).ok_or_else(|| {
            self.err_invalid(format!(
                "[freeze:contract][vm/sum:project] {} missing {}",
                enum_name, SUM_TAG_FIELD
            ))
        })?;
        validate_sum_tag(self, enum_name, actual_tag)?;
        if actual_tag != tag {
            return Err(self.err_invalid(format!(
                "[freeze:contract][vm/sum:project] {}::{} tag mismatch actual={} expected={}",
                enum_name, variant, actual_tag, tag
            )));
        }
        let payload = instance.get_field(SUM_PAYLOAD_FIELD).ok_or_else(|| {
            self.err_invalid(format!(
                "[freeze:contract][vm/sum:project] {}::{} missing {}",
                enum_name, variant, SUM_PAYLOAD_FIELD
            ))
        })?;
        self.write_reg(dst, VMValue::from_nyash_box(payload.share_box()));
        Ok(())
    }

    fn resolve_sum_variant(
        &self,
        enum_name: &str,
        variant: &str,
    ) -> Result<(u32, &MirEnumVariantDecl), VMError> {
        let decl = self.enum_decls.get(enum_name).ok_or_else(|| {
            self.err_invalid(format!(
                "[freeze:contract][vm/sum:meta] missing enum declaration for {}",
                enum_name
            ))
        })?;
        decl.variants
            .iter()
            .enumerate()
            .find(|(_, item)| item.name == variant)
            .map(|(idx, item)| (idx as u32, item))
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[freeze:contract][vm/sum:meta] missing variant {}::{}",
                    enum_name, variant
                ))
            })
    }

    fn load_sum_instance(
        &self,
        value: ValueId,
        enum_name: &str,
    ) -> Result<Arc<dyn crate::box_trait::NyashBox>, VMError> {
        let box_ref = match self.reg_load(value)? {
            VMValue::BoxRef(box_ref) => box_ref,
            other => {
                return Err(self.err_invalid(format!(
                    "[freeze:contract][vm/sum:value] expected {} runtime box, got {:?}",
                    enum_name, other
                )))
            }
        };
        let instance = box_ref
            .as_any()
            .downcast_ref::<InstanceBox>()
            .ok_or_else(|| {
                self.err_invalid(format!(
                    "[freeze:contract][vm/sum:value] {} value is not an InstanceBox",
                    enum_name
                ))
            })?;
        let expected_box_name = runtime_sum_box_name(enum_name);
        if instance.class_name != expected_box_name {
            return Err(self.err_invalid(format!(
                "[freeze:contract][vm/sum:value] {} expected runtime box {} but got {}",
                enum_name, expected_box_name, instance.class_name
            )));
        }
        Ok(box_ref)
    }
}

fn validate_sum_tag(this: &MirInterpreter, enum_name: &str, tag: u32) -> Result<(), VMError> {
    let decl: &MirEnumDecl = this.enum_decls.get(enum_name).ok_or_else(|| {
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

fn read_sum_tag(instance: &InstanceBox) -> Option<u32> {
    match instance.get_field_ng(SUM_TAG_FIELD) {
        Some(crate::value::NyashValue::Integer(value)) => u32::try_from(value).ok(),
        _ => None,
    }
}

fn vm_value_to_shared_box(value: VMValue) -> SharedNyashBox {
    match value {
        VMValue::BoxRef(shared) => shared,
        other => Arc::from(other.to_nyash_box()),
    }
}
