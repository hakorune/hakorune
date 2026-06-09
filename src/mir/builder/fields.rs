// Field access and assignment lowering
use super::ValueId;
use crate::ast::ASTNode;
use crate::mir::instruction::FastMemRegionId;

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

        self.build_field_access_from_value(object_value, field)
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

    pub(super) fn build_field_access_from_value(
        &mut self,
        object_value: ValueId,
        field: String,
    ) -> Result<ValueId, String> {
        let region = self.current_fastmem_region();
        let receiver_box_name = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned();
        self.record_field_access_site(
            region,
            object_value,
            receiver_box_name,
            field.clone(),
            None,
            "load",
            if region.is_some() {
                "verified_layout_field"
            } else {
                "none"
            },
            if region.is_some() {
                "forbidden"
            } else {
                "allow_dynamic"
            },
        )?;
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
                let dst = field_val;
                super::weak_field_validator::WeakFieldValidatorBox::annotate_read_result(
                    &mut self.type_ctx,
                    dst,
                );

                let _ = self.emit_barrier_read(dst);
                return Ok(dst);
            }
        }

        Ok(field_val)
    }

    pub(super) fn build_field_assignment_from_value(
        &mut self,
        object_value: ValueId,
        field: String,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        let mut value_result = self.build_expression(value)?;
        value_result = self.local_arg(value_result);
        self.build_field_assignment_from_value_id(
            self.current_fastmem_region(),
            object_value,
            field,
            value_result,
        )
    }

    pub(super) fn build_field_assignment_from_value_id(
        &mut self,
        region: Option<FastMemRegionId>,
        object_value: ValueId,
        field: String,
        value_result: ValueId,
    ) -> Result<ValueId, String> {
        let region = region.or_else(|| self.current_fastmem_region());
        let receiver_box_name = self
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned();
        self.record_field_access_site(
            region,
            object_value,
            receiver_box_name,
            field.clone(),
            None,
            "store",
            "none",
            "allow_dynamic",
        )?;
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

#[cfg(test)]
mod tests {
    use super::super::MirBuilder;
    use crate::ast::{ASTNode, LiteralValue, Span};

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn field(object: ASTNode, name: &str) -> ASTNode {
        ASTNode::FieldAccess {
            object: Box::new(object),
            field: name.to_string(),
            span: span(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: span(),
        }
    }

    fn local(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(value))],
            declared_type_names: Vec::new(),
            span: span(),
        }
    }

    #[test]
    fn ordinary_field_access_records_site_metadata() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("ordinary_field_access/0".to_string());
        let body = vec![
            local("obj", int_lit(1)),
            local("loaded", field(var("obj"), "used")),
            assign(field(var("obj"), "used"), int_lit(2)),
        ];

        super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
        let function = builder.scope_ctx.current_function.as_ref().unwrap();

        assert_eq!(function.metadata.fastmem_field_access_sites.len(), 2);
        assert!(function
            .metadata
            .fastmem_field_access_sites
            .iter()
            .all(|site| site.region.is_none()));
        assert_eq!(
            function.metadata.fastmem_field_access_sites[0].required_route,
            "none"
        );
        assert_eq!(
            function.metadata.fastmem_field_access_sites[0].fallback_policy,
            "allow_dynamic"
        );
        assert_eq!(
            function.metadata.fastmem_field_access_sites[0].access_kind,
            "load"
        );
        assert_eq!(
            function.metadata.fastmem_field_access_sites[1].access_kind,
            "store"
        );
    }
}
