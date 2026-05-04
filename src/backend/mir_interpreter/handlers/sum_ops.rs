use super::sum_bridge;
use super::*;
use crate::instance_v2::InstanceBox;
use crate::mir::{MirEnumVariantDecl, MirType};
use crate::semantics::option_contract::{nullish_payload_error, requires_non_nullish_payload};
use std::collections::HashMap;
use std::sync::Arc;

impl MirInterpreter {
    pub(super) fn handle_variant_make(
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

        let instance = InstanceBox::from_declaration(
            sum_bridge::runtime_variant_box_name(enum_name),
            vec![
                sum_bridge::ENUM_TAG_FIELD.to_string(),
                sum_bridge::ENUM_PAYLOAD_FIELD.to_string(),
            ],
            HashMap::new(),
        );
        instance
            .set_field_ng(
                sum_bridge::ENUM_TAG_FIELD.to_string(),
                crate::value::NyashValue::Integer(tag as i64),
            )
            .map_err(|e| self.err_invalid(format!("[freeze:contract][vm/sum:make] {}", e)))?;
        let runtime_sum: Arc<dyn crate::box_trait::NyashBox> = Arc::new(instance);
        if let Some(payload_id) = payload {
            let payload_value = self.reg_load(payload_id)?;
            if requires_non_nullish_payload(enum_name, variant)
                && vm_value_is_nullish(&payload_value)
            {
                return Err(self.err_invalid(nullish_payload_error("vm/sum:make")));
            }
            sum_bridge::store_sum_payload(self, &runtime_sum, payload_value);
        }
        self.write_reg(dst, VMValue::BoxRef(runtime_sum));
        Ok(())
    }

    pub(super) fn handle_variant_tag(
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
        let tag = sum_bridge::read_variant_tag(instance).ok_or_else(|| {
            self.err_invalid(format!(
                "[freeze:contract][vm/sum:tag] {} missing {}",
                enum_name,
                sum_bridge::ENUM_TAG_FIELD
            ))
        })?;
        sum_bridge::validate_variant_tag(self, enum_name, tag)?;
        self.write_reg(dst, VMValue::Integer(i64::from(tag)));
        Ok(())
    }

    pub(super) fn handle_variant_project(
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
        let actual_tag = sum_bridge::read_variant_tag(instance).ok_or_else(|| {
            self.err_invalid(format!(
                "[freeze:contract][vm/sum:project] {} missing {}",
                enum_name,
                sum_bridge::ENUM_TAG_FIELD
            ))
        })?;
        sum_bridge::validate_variant_tag(self, enum_name, actual_tag)?;
        if actual_tag != tag {
            return Err(self.err_invalid(format!(
                "[freeze:contract][vm/sum:project] {}::{} tag mismatch actual={} expected={}",
                enum_name, variant, actual_tag, tag
            )));
        }
        let payload = sum_bridge::read_sum_payload(self, &box_ref, instance).ok_or_else(|| {
            self.err_invalid(format!(
                "[freeze:contract][vm/sum:project] {}::{} missing {}",
                enum_name,
                variant,
                sum_bridge::ENUM_PAYLOAD_FIELD
            ))
        })?;
        self.write_reg(dst, payload);
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
        let expected_box_name = sum_bridge::runtime_variant_box_name(enum_name);
        if instance.class_name != expected_box_name {
            return Err(self.err_invalid(format!(
                "[freeze:contract][vm/sum:value] {} expected runtime box {} but got {}",
                enum_name, expected_box_name, instance.class_name
            )));
        }
        Ok(box_ref)
    }
}

fn vm_value_is_nullish(value: &VMValue) -> bool {
    match value {
        VMValue::Void => true,
        VMValue::BoxRef(box_ref) => {
            crate::boxes::null_box::NullBox::check_null(box_ref.as_ref())
                || box_ref
                    .as_any()
                    .downcast_ref::<crate::box_trait::VoidBox>()
                    .is_some()
        }
        _ => false,
    }
}
