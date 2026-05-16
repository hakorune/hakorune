//! RecordValueScalarizationBox - C205b builder-local record construction/read.
//!
//! Records are identity-free aggregate values. This first lowering row keeps
//! them inside the MIR builder and replaces direct field reads with constructor
//! operands. It deliberately does not emit `NewBox`, typed-object plans,
//! backend hooks, or ArrayBox packed-storage use.

use crate::ast::ASTNode;
use crate::mir::builder::compilation_context::RecordLocalFieldValue;
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

impl MirBuilder {
    pub(in crate::mir::builder) fn is_record_constructor_class(&self, class: &str) -> bool {
        self.comp_ctx.is_record_decl(class)
    }

    pub(in crate::mir::builder) fn build_record_constructor_value(
        &mut self,
        class: String,
        arguments: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        let Some(decl) = self.comp_ctx.record_decls.get(&class).cloned() else {
            return Err(format!(
                "[record-construction/unknown-record] record={}",
                class
            ));
        };
        if !decl.type_parameters.is_empty() {
            return Err(format!(
                "[record-construction/generic-unsupported] record={}",
                class
            ));
        }
        if arguments.len() != decl.fields.len() {
            return Err(format!(
                "[record-construction/arity-mismatch] record={} expected={} actual={}",
                class,
                decl.fields.len(),
                arguments.len()
            ));
        }

        let mut field_values = Vec::with_capacity(decl.fields.len());
        for (field, argument) in decl.fields.iter().zip(arguments.into_iter()) {
            field_values.push(self.build_record_local_field_value(
                field.name.clone(),
                field.declared_type_name.clone(),
                argument,
            )?);
        }

        self.register_record_local_fields(class, field_values)
    }

    pub(in crate::mir::builder) fn build_record_literal_value(
        &mut self,
        record_type_name: String,
        fields: Vec<(String, ASTNode)>,
    ) -> Result<ValueId, String> {
        let Some(decl) = self.comp_ctx.record_decls.get(&record_type_name).cloned() else {
            return Err(format!(
                "[record-literal/unknown-record] record={}",
                record_type_name
            ));
        };
        if !decl.type_parameters.is_empty() {
            return Err(format!(
                "[record-literal/generic-unsupported] record={}",
                record_type_name
            ));
        }

        let mut by_name = BTreeMap::new();
        for (field_name, expr) in fields {
            if by_name.insert(field_name.clone(), expr).is_some() {
                return Err(format!(
                    "[record-literal/duplicate-field] record={} field={}",
                    record_type_name, field_name
                ));
            }
        }

        let mut field_values = Vec::with_capacity(decl.fields.len());
        for field in &decl.fields {
            let Some(expr) = by_name.remove(&field.name) else {
                return Err(format!(
                    "[record-literal/missing-field] record={} field={}",
                    record_type_name, field.name
                ));
            };
            field_values.push(self.build_record_local_field_value(
                field.name.clone(),
                field.declared_type_name.clone(),
                expr,
            )?);
        }

        if let Some((field_name, _)) = by_name.into_iter().next() {
            return Err(format!(
                "[record-literal/unknown-field] record={} field={}",
                record_type_name, field_name
            ));
        }

        self.register_record_local_fields(record_type_name, field_values)
    }

    pub(in crate::mir::builder) fn fail_if_record_value_escape_by_name(
        &self,
        name: &str,
        value: ValueId,
    ) -> Result<(), String> {
        if let Some(record) = self.comp_ctx.record_local_value(value) {
            return Err(format!(
                "[record-value/escape] name={} record={} supported_use=field-read",
                name, record.record_name
            ));
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn try_lower_record_field_read_from_ast(
        &mut self,
        object: &ASTNode,
        field: &str,
    ) -> Result<Option<ValueId>, String> {
        match object {
            ASTNode::Variable { name, .. } => {
                let Some(value) = self.variable_ctx.variable_map.get(name).copied() else {
                    return Ok(None);
                };
                self.lower_record_field_read_from_value(value, field)
            }
            ASTNode::New {
                class, arguments, ..
            } if self.is_record_constructor_class(class) => {
                let value =
                    self.build_record_constructor_value(class.clone(), arguments.clone())?;
                self.lower_record_field_read_from_value(value, field)
            }
            ASTNode::RecordLiteral {
                record_type_name,
                fields,
                ..
            } => {
                let value =
                    self.build_record_literal_value(record_type_name.clone(), fields.clone())?;
                self.lower_record_field_read_from_value(value, field)
            }
            _ => Ok(None),
        }
    }

    pub(in crate::mir::builder) fn fail_if_record_field_assignment_target(
        &self,
        object: &ASTNode,
        field: &str,
    ) -> Result<(), String> {
        match object {
            ASTNode::Variable { name, .. } => {
                if let Some(value) = self.variable_ctx.variable_map.get(name).copied() {
                    if let Some(record) = self.comp_ctx.record_local_value(value) {
                        return Err(format!(
                            "[record-field-set/unsupported] name={} record={} field={}",
                            name, record.record_name, field
                        ));
                    }
                }
            }
            ASTNode::New { class, .. } if self.is_record_constructor_class(class) => {
                return Err(format!(
                    "[record-field-set/unsupported] record={} field={}",
                    class, field
                ));
            }
            ASTNode::RecordLiteral {
                record_type_name, ..
            } => {
                return Err(format!(
                    "[record-field-set/unsupported] record={} field={}",
                    record_type_name, field
                ));
            }
            _ => {}
        }
        Ok(())
    }

    fn lower_record_field_read_from_value(
        &mut self,
        value: ValueId,
        field: &str,
    ) -> Result<Option<ValueId>, String> {
        let Some(record) = self.comp_ctx.record_local_value(value).cloned() else {
            return Ok(None);
        };
        let Some(field_value) = record
            .fields
            .iter()
            .find(|candidate| candidate.name == field)
        else {
            return Err(format!(
                "[record-field-read/unknown-field] record={} field={}",
                record.record_name, field
            ));
        };
        if let Some(declared_type) = field_value.declared_type_name.as_deref() {
            let ty = Self::parse_type_name_to_mir(declared_type);
            self.type_ctx.value_types.insert(field_value.value, ty);
        }
        Ok(Some(field_value.value))
    }

    fn build_record_local_field_value(
        &mut self,
        name: String,
        declared_type_name: Option<String>,
        expr: ASTNode,
    ) -> Result<RecordLocalFieldValue, String> {
        let value = self.build_expression(expr)?;
        Ok(RecordLocalFieldValue {
            name,
            declared_type_name,
            value,
        })
    }

    fn register_record_local_fields(
        &mut self,
        record_name: String,
        field_values: Vec<RecordLocalFieldValue>,
    ) -> Result<ValueId, String> {
        let placeholder = crate::mir::builder::emission::constant::emit_void(self)?;
        self.comp_ctx
            .register_record_local_value(placeholder, record_name, field_values);
        Ok(placeholder)
    }
}
