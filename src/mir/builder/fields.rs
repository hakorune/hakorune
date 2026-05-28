// Field access and assignment lowering
use super::ValueId;
use crate::ast::ASTNode;

impl super::MirBuilder {
    /// Build field access: object.field
    pub(super) fn build_field_access(
        &mut self,
        object: ASTNode,
        field: String,
    ) -> Result<ValueId, String> {
        if let Some(record_field_value) =
            self.try_lower_record_field_read_from_ast(&object, &field)?
        {
            return Ok(record_field_value);
        }

        let object_value = self.build_expression(object)?;
        let object_value = self.local_field_base(object_value);

        if let Some(property_value) = self.try_lower_property_read(object_value, &field)? {
            return Ok(property_value);
        }

        let declared_type = self.declared_field_type_for_value(object_value, &field);

        let field_val = if let Some(ref ty) = declared_type {
            self.alloc_typed(ty.clone())
        } else {
            self.next_value_id()
        };
        self.emit_instruction(crate::mir::MirInstruction::FieldGet {
            dst: field_val,
            base: object_value,
            field: field.clone(),
            declared_type,
        })?;

        self.publish_field_result_origin(field_val, object_value, &field);

        // If the loaded field result has a known box origin and its requested
        // field is weak, keep WeakRef (+ optional barrier). This must only
        // consume already-published origin facts; re-lowering nested field
        // receiver ASTs here would duplicate semantic calls.
        let inferred_class = self.type_ctx.value_origin_newbox.get(&field_val).cloned();
        if let Some(class_name) = inferred_class {
            if self.is_weak_field_on_result_class(&class_name, &field) {
                // Phase 285A1: Read weak field returns WeakRef (no auto-upgrade)
                // Delegated to WeakFieldValidatorBox
                let dst = field_val; // The load result is already our return value

                // Phase 285A1: Annotate result as WeakRef type
                super::weak_field_validator::WeakFieldValidatorBox::annotate_read_result(
                    &mut self.type_ctx,
                    dst,
                );

                let _ = self.emit_barrier_read(dst);
                return Ok(dst); // Return WeakRef directly (no auto-upgrade)
            }
        }

        Ok(field_val)
    }

    /// Build field assignment: object.field = value
    pub(super) fn build_field_assignment(
        &mut self,
        object: ASTNode,
        field: String,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        self.fail_if_record_field_assignment_target(&object, &field)?;
        let object_value = self.build_expression(object)?;
        let object_value = self.local_field_base(object_value);
        self.build_field_assignment_from_value(object_value, field, value)
    }

    pub(super) fn build_box_field_initializers(
        &mut self,
        object_value: ValueId,
        class: &str,
        field_initializers: Vec<(String, ASTNode)>,
    ) -> Result<(), String> {
        let mut seen = std::collections::BTreeSet::new();
        for (field, value) in field_initializers {
            if !seen.insert(field.clone()) {
                return Err(format!(
                    "[box-init/duplicate-field] class={} field={}",
                    class, field
                ));
            }
            if self.comp_ctx.user_defined_boxes.contains_key(class) {
                let declared = self
                    .comp_ctx
                    .user_defined_boxes
                    .get(class)
                    .map(|fields| fields.iter().any(|name| name == &field))
                    .unwrap_or(false);
                if !declared {
                    return Err(format!(
                        "[box-init/unknown-field] class={} field={}",
                        class, field
                    ));
                }
            }
            self.build_field_assignment_from_value(object_value, field, value)?;
        }
        Ok(())
    }

    fn build_field_assignment_from_value(
        &mut self,
        object_value: ValueId,
        field: String,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        let mut value_result = self.build_expression(value)?;
        // LocalSSA: argument in-block (optional safety)
        value_result = self.local_arg(value_result);
        let declared_type = self.declared_field_type_for_value(object_value, &field);

        // Phase 285A1: If field is weak, enforce type contract (3 allowed cases)
        // Delegated to WeakFieldValidatorBox
        if let Some(class_name) = self.is_weak_field_on_base(object_value, &field) {
            // Phase 285A1: Strict type check (delegated to validator)
            let value_type = self.type_ctx.value_types.get(&value_result);
            super::weak_field_validator::WeakFieldValidatorBox::validate_assignment(
                value_type,
                &class_name,
                &field,
            )?;
        }

        self.emit_instruction(crate::mir::MirInstruction::FieldSet {
            base: object_value,
            field: field.clone(),
            value: value_result,
            declared_type,
        })?;

        // Write barrier if weak field
        if self.is_weak_field_on_base(object_value, &field).is_some() {
            let _ = self.emit_barrier_write(value_result);
        }

        // Record origin class for this field value if known
        if let Some(val_cls) = self
            .type_ctx
            .value_origin_newbox
            .get(&value_result)
            .cloned()
        {
            self.comp_ctx
                .field_origin_class
                .insert((object_value, field.clone()), val_cls.clone());
            // Also record class-level mapping if base object class is known
            if let Some(base_cls) = self
                .type_ctx
                .value_origin_newbox
                .get(&object_value)
                .cloned()
            {
                self.comp_ctx
                    .field_origin_by_box
                    .insert((base_cls, field.clone()), val_cls);
            }
        }

        Ok(value_result)
    }
}
