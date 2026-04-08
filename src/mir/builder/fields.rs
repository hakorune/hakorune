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
        let object_clone = object.clone();
        let object_value = self.build_expression(object.clone())?;
        let object_value = self.local_field_base(object_value);
        let declared_type = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .and_then(|box_name| self.comp_ctx.declared_field_type_name(box_name, &field))
            .map(Self::parse_type_name_to_mir);

        // Unified members: if object class is known and has a synthetic getter for `field`,
        // rewrite to method call `__get_<field>()`.
        if let Some(class_name) = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned()
        {
            if let Some(map) = self.comp_ctx.property_getters_by_box.get(&class_name) {
                if let Some(kind) = map.get(&field) {
                    let mname = match kind {
                        super::PropertyKind::Computed => format!("__get_{}", field),
                        super::PropertyKind::Once => format!("__get_once_{}", field),
                        super::PropertyKind::BirthOnce => format!("__get_birth_{}", field),
                    };
                    return self.build_method_call(object_clone, mname, vec![]);
                }
            }
        }

        let field_val = self.next_value_id();
        self.emit_instruction(crate::mir::MirInstruction::FieldGet {
            dst: field_val,
            base: object_value,
            field: field.clone(),
            declared_type,
        })?;

        // Propagate recorded origin class for this field if any (ValueId-scoped)
        if let Some(class_name) = self
            .comp_ctx
            .field_origin_class
            .get(&(object_value, field.clone()))
            .cloned()
        {
            self.type_ctx
                .value_origin_newbox
                .insert(field_val, class_name);
        } else if let Some(base_cls) = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned()
        {
            // Cross-function heuristic: use class-level field origin mapping
            if let Some(fcls) = self
                .comp_ctx
                .field_origin_by_box
                .get(&(base_cls.clone(), field.clone()))
                .cloned()
            {
                if super::utils::builder_debug_enabled()
                    || crate::config::env::builder_debug_enabled()
                {
                    super::utils::builder_debug_log(&format!(
                        "field-origin hit by box-level map: base={} .{} -> {}",
                        base_cls, field, fcls
                    ));
                }
                self.type_ctx.value_origin_newbox.insert(field_val, fcls);
            }
        }

        // If base is a known newbox and field is weak, keep WeakRef (+ optional barrier)
        let mut inferred_class: Option<String> = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned();
        if inferred_class.is_none() {
            if let ASTNode::FieldAccess {
                object: inner_obj,
                field: inner_field,
                ..
            } = object_clone
            {
                if let Ok(base_id) = self.build_expression(*inner_obj.clone()) {
                    if let Some(cls) = self
                        .comp_ctx
                        .field_origin_class
                        .get(&(base_id, inner_field))
                        .cloned()
                    {
                        inferred_class = Some(cls);
                    }
                }
            }
        }
        if let Some(class_name) = inferred_class {
            if let Some(weak_set) = self.comp_ctx.weak_fields_by_box.get(&class_name) {
                if weak_set.contains(&field) {
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
        }

        // Correctness-first: slotify field values so they have block-local defs
        // and participate in PHI merges when reused across branches.
        let pinned = self.pin_to_slot(field_val, "@field")?;
        Ok(pinned)
    }

    /// Build field assignment: object.field = value
    pub(super) fn build_field_assignment(
        &mut self,
        object: ASTNode,
        field: String,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        let object_value = self.build_expression(object)?;
        let object_value = self.local_field_base(object_value);
        let mut value_result = self.build_expression(value)?;
        // LocalSSA: argument in-block (optional safety)
        value_result = self.local_arg(value_result);
        let declared_type = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .and_then(|box_name| self.comp_ctx.declared_field_type_name(box_name, &field))
            .map(Self::parse_type_name_to_mir);

        // Phase 285A1: If field is weak, enforce type contract (3 allowed cases)
        // Delegated to WeakFieldValidatorBox
        if let Some(class_name) = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned()
        {
            if let Some(weak_set) = self.comp_ctx.weak_fields_by_box.get(&class_name) {
                if weak_set.contains(&field) {
                    // Phase 285A1: Strict type check (delegated to validator)
                    let value_type = self.type_ctx.value_types.get(&value_result);
                    super::weak_field_validator::WeakFieldValidatorBox::validate_assignment(
                        value_type,
                        &class_name,
                        &field,
                    )?;
                }
            }
        }

        self.emit_instruction(crate::mir::MirInstruction::FieldSet {
            base: object_value,
            field: field.clone(),
            value: value_result,
            declared_type,
        })?;

        // Write barrier if weak field
        if let Some(class_name) = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned()
        {
            if let Some(weak_set) = self.comp_ctx.weak_fields_by_box.get(&class_name) {
                if weak_set.contains(&field) {
                    let _ = self.emit_barrier_write(value_result);
                }
            }
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
