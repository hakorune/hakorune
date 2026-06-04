//! Field/property fact helpers.
//!
//! This module observes already-lowered receiver values and builder registries.
//! It must not lower receiver ASTs or re-run semantic calls; `fields.rs` owns
//! field emission order and `property_reads.rs` owns property getter lowering.

use super::ValueId;

impl super::MirBuilder {
    pub(super) fn declared_field_type_for_value(
        &self,
        object_value: ValueId,
        field: &str,
    ) -> Option<crate::mir::MirType> {
        self.type_ctx
            .value_origin_newbox
            .get(&object_value)
            .and_then(|box_name| self.comp_ctx.declared_field_type_name(box_name, field))
            .map(Self::parse_type_name_to_mir)
    }

    pub(super) fn inferred_field_result_class(
        &self,
        object_value: ValueId,
        field: &str,
    ) -> Option<String> {
        self.comp_ctx
            .field_origin_class
            .get(&(object_value, field.to_string()))
            .cloned()
            .or_else(|| {
                let base_cls = self
                    .type_ctx
                    .value_origin_newbox
                    .get(&object_value)?
                    .clone();
                self.comp_ctx
                    .field_origin_by_box
                    .get(&(base_cls, field.to_string()))
                    .cloned()
            })
    }

    pub(super) fn publish_field_result_origin(
        &mut self,
        field_val: ValueId,
        object_value: ValueId,
        field: &str,
    ) {
        let inferred_class = self.inferred_field_result_class(object_value, field);
        if let Some(class_name) = inferred_class {
            if super::utils::builder_debug_enabled() || crate::config::env::builder_debug_enabled()
            {
                super::utils::builder_debug_log(&format!(
                    "field-origin publish: base=%{} .{} -> {}",
                    object_value.0, field, class_name
                ));
            }
            self.type_ctx
                .value_origin_newbox
                .insert(field_val, class_name);
        }
    }

    pub(super) fn resolve_property_getter_name(
        &self,
        object_value: ValueId,
        field: &str,
    ) -> Option<String> {
        let class_name = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)?
            .clone();
        self.comp_ctx
            .property_getter_method_name(&class_name, field)
    }

    pub(super) fn is_weak_field_on_base(
        &self,
        object_value: ValueId,
        field: &str,
    ) -> Option<String> {
        let class_name = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)?
            .clone();
        self.comp_ctx
            .weak_fields_by_box
            .get(&class_name)
            .and_then(|weak_set| weak_set.contains(field).then_some(class_name))
    }

    pub(super) fn is_weak_field_on_result_class(&self, class_name: &str, field: &str) -> bool {
        self.comp_ctx
            .weak_fields_by_box
            .get(class_name)
            .is_some_and(|weak_set| weak_set.contains(field))
    }
}
