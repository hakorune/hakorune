//! RecordValueScalarizationBox - C205b builder-local record construction/read.
//!
//! Records are identity-free aggregate values. This first lowering row keeps
//! them inside the MIR builder and replaces direct field reads with constructor
//! operands. It deliberately does not emit `NewBox`, typed-object plans,
//! backend hooks, or ArrayBox packed-storage use.

use crate::ast::ASTNode;
use crate::mir::builder::compilation_context::RecordLocalFieldValue;
use crate::mir::builder::MirBuilder;
use crate::mir::function::{RecordDecl, RecordValueBoundaryKind};
use crate::mir::{MirInstruction, UserBoxFieldDecl, ValueId};
use std::collections::{BTreeMap, BTreeSet};

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
                "[type/record_contract_unknown_record] record={}",
                class
            ));
        };
        if !decl.type_parameters.is_empty() {
            return Err(format!(
                "[type/record_contract_generic_unsupported] record={}",
                class
            ));
        }
        if arguments.len() != decl.fields.len() {
            return Err(format!(
                "[type/record_contract_constructor_arity_mismatch] record={} expected={} actual={}",
                class,
                decl.fields.len(),
                arguments.len()
            ));
        }

        let (dst, contract_id, fingerprint) = self.begin_record_value_contract(&decl);
        let mut field_values = Vec::with_capacity(decl.fields.len());
        for (field_index, (field, argument)) in
            decl.fields.iter().zip(arguments.into_iter()).enumerate()
        {
            field_values.push(self.build_checked_record_field_value(
                field_index,
                field,
                argument,
                &contract_id,
                &fingerprint,
            )?);
        }

        self.publish_record_local_fields(
            dst,
            contract_id,
            RecordValueBoundaryKind::Construct,
            class,
            fingerprint,
            None,
            field_values,
        )
    }

    pub(in crate::mir::builder) fn build_record_literal_value(
        &mut self,
        record_type_name: String,
        fields: Vec<(String, ASTNode)>,
    ) -> Result<ValueId, String> {
        let Some(decl) = self.comp_ctx.record_decls.get(&record_type_name).cloned() else {
            return Err(format!(
                "[type/record_contract_unknown_record] record={}",
                record_type_name
            ));
        };
        if !decl.type_parameters.is_empty() {
            return Err(format!(
                "[type/record_contract_generic_unsupported] record={}",
                record_type_name
            ));
        }

        let declared_names = decl
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect::<BTreeSet<_>>();
        let mut seen = BTreeSet::new();
        for (field_name, _) in &fields {
            if !seen.insert(field_name.as_str()) {
                return Err(format!(
                    "[type/record_contract_duplicate_field] record={} field={}",
                    record_type_name, field_name
                ));
            }
            if !declared_names.contains(field_name.as_str()) {
                return Err(format!(
                    "[type/record_contract_unknown_field] record={} field={}",
                    record_type_name, field_name
                ));
            }
        }
        for field in &decl.fields {
            if !seen.contains(field.name.as_str())
                && !decl.default_field_names.contains(&field.name)
            {
                return Err(format!(
                    "[type/record_contract_missing_required_field] record={} field={}",
                    record_type_name, field.name
                ));
            }
        }

        let (dst, contract_id, fingerprint) = self.begin_record_value_contract(&decl);
        let mut by_name = BTreeMap::new();
        // Explicit source expressions retain source order.
        for (field_name, expr) in fields {
            let field_index = decl
                .fields
                .iter()
                .position(|field| field.name == field_name)
                .expect("record field set preflight");
            let field = &decl.fields[field_index];
            let value = self.build_checked_record_field_value(
                field_index,
                field,
                expr,
                &contract_id,
                &fingerprint,
            )?;
            by_name.insert(field_name, value);
        }
        // Missing defaults retain declaration order.
        for (field_index, field) in decl.fields.iter().enumerate() {
            if by_name.contains_key(&field.name) {
                continue;
            }
            let default_expr = self
                .comp_ctx
                .record_field_defaults
                .get(&record_type_name)
                .and_then(|defaults| defaults.get(&field.name))
                .cloned()
                .ok_or_else(|| {
                    format!(
                        "[type/record_contract_source_drift] record={} field={} default=missing",
                        record_type_name, field.name
                    )
                })?;
            let value = self.build_checked_record_field_value(
                field_index,
                field,
                default_expr,
                &contract_id,
                &fingerprint,
            )?;
            by_name.insert(field.name.clone(), value);
        }

        let field_values = decl
            .fields
            .iter()
            .map(|field| {
                by_name
                    .remove(&field.name)
                    .expect("complete record preflight")
            })
            .collect();
        self.publish_record_local_fields(
            dst,
            contract_id,
            RecordValueBoundaryKind::Construct,
            record_type_name,
            fingerprint,
            None,
            field_values,
        )
    }

    pub(in crate::mir::builder) fn build_record_update_value(
        &mut self,
        base: ASTNode,
        updates: Vec<(String, ASTNode)>,
    ) -> Result<ValueId, String> {
        let expected_record_name = self.record_name_for_value_base(&base).ok_or_else(|| {
            "[type/record_contract_update_base_mismatch] expected=record-local-value".to_string()
        })?;
        let decl = self
            .comp_ctx
            .record_decls
            .get(&expected_record_name)
            .cloned()
            .ok_or_else(|| {
                format!(
                    "[type/record_contract_unknown_record] record={}",
                    expected_record_name
                )
            })?;
        let declared_names = decl
            .fields
            .iter()
            .map(|field| field.name.as_str())
            .collect::<BTreeSet<_>>();
        let mut seen = BTreeSet::new();
        for (field_name, _) in &updates {
            if !seen.insert(field_name.as_str()) {
                return Err(format!(
                    "[type/record_contract_duplicate_field] record={} field={}",
                    expected_record_name, field_name
                ));
            }
            if !declared_names.contains(field_name.as_str()) {
                return Err(format!(
                    "[type/record_contract_unknown_field] record={} field={}",
                    expected_record_name, field_name
                ));
            }
        }

        let (dst, contract_id, fingerprint) = self.begin_record_value_contract(&decl);
        let base_value = self.build_record_value_base(base)?;
        let Some(record) = self
            .function_state
            .compilation
            .record_local_value(base_value)
            .cloned()
        else {
            return Err(format!(
                "[type/record_contract_update_base_mismatch] value={}",
                base_value.as_u32()
            ));
        };
        if record.record_name != expected_record_name {
            return Err(format!(
                "[type/record_contract_update_base_mismatch] expected={} actual={}",
                expected_record_name, record.record_name
            ));
        }

        let update_names = updates
            .iter()
            .map(|(name, _)| name.as_str())
            .collect::<BTreeSet<_>>();
        let mut by_name = record
            .fields
            .into_iter()
            .map(|field| (field.name.clone(), field))
            .collect::<BTreeMap<_, _>>();
        // Unchanged final fields are checked before update expression effects.
        for (field_index, field) in decl.fields.iter().enumerate() {
            if !update_names.contains(field.name.as_str()) {
                let value = by_name
                    .get(&field.name)
                    .expect("record schema preflight")
                    .value;
                self.emit_record_field_contract_check(
                    field_index,
                    field,
                    value,
                    &contract_id,
                    &fingerprint,
                )?;
            }
        }
        for (field_name, expr) in updates {
            let field_index = decl
                .fields
                .iter()
                .position(|field| field.name == field_name)
                .expect("update field preflight");
            let value = self.build_checked_record_field_value(
                field_index,
                &decl.fields[field_index],
                expr,
                &contract_id,
                &fingerprint,
            )?;
            by_name.insert(field_name, value);
        }

        let field_values = decl
            .fields
            .iter()
            .map(|field| {
                by_name
                    .remove(&field.name)
                    .expect("record schema preflight")
            })
            .collect();
        self.publish_record_local_fields(
            dst,
            contract_id,
            RecordValueBoundaryKind::WithUpdate,
            expected_record_name,
            fingerprint,
            Some(base_value),
            field_values,
        )
    }

    pub(in crate::mir::builder) fn fail_if_record_value_escape_by_name(
        &self,
        name: &str,
        value: ValueId,
    ) -> Result<(), String> {
        if let Some(record) = self.function_state.compilation.record_local_value(value) {
            return Err(format!(
                "[record-value/escape] name={} record={} supported_use=field-read",
                name, record.record_name
            ));
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn fail_if_record_value_call_arg_by_name(
        &self,
        name: &str,
        value: ValueId,
    ) -> Result<(), String> {
        if let Some(record) = self.function_state.compilation.record_local_value(value) {
            return Err(format!(
                "[record-helper-arg/unsupported] name={} record={} required=helper-argument-scalarization supported_use=field-read",
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
                let Some(value) = self
                    .function_state
                    .variable_ctx
                    .variable_map
                    .get(name)
                    .copied()
                else {
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
            ASTNode::RecordUpdate { base, updates, .. } => {
                let value = self.build_record_update_value(*base.clone(), updates.clone())?;
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
                if let Some(value) = self
                    .function_state
                    .variable_ctx
                    .variable_map
                    .get(name)
                    .copied()
                {
                    if let Some(record) = self.function_state.compilation.record_local_value(value)
                    {
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
            ASTNode::RecordUpdate { .. } => {
                return Err(format!(
                    "[record-field-set/unsupported] record-update field={}",
                    field
                ));
            }
            _ => {}
        }
        Ok(())
    }

    fn build_record_value_base(&mut self, base: ASTNode) -> Result<ValueId, String> {
        match base {
            ASTNode::Variable { name, .. } => self
                .function_state
                .variable_ctx
                .variable_map
                .get(&name)
                .copied()
                .ok_or_else(|| format!("[record-update/base-unresolved] name={}", name)),
            ASTNode::New {
                class, arguments, ..
            } if self.is_record_constructor_class(&class) => {
                self.build_record_constructor_value(class, arguments)
            }
            ASTNode::RecordLiteral {
                record_type_name,
                fields,
                ..
            } => self.build_record_literal_value(record_type_name, fields),
            ASTNode::RecordUpdate { base, updates, .. } => {
                self.build_record_update_value(*base, updates)
            }
            _ => Err("[record-update/base-unsupported] expected=record-local-value".to_string()),
        }
    }

    fn record_name_for_value_base(&self, base: &ASTNode) -> Option<String> {
        match base {
            ASTNode::Variable { name, .. } => self
                .function_state
                .variable_ctx
                .variable_map
                .get(name)
                .and_then(|value| self.function_state.compilation.record_local_value(*value))
                .map(|record| record.record_name.clone()),
            ASTNode::New { class, .. } if self.is_record_constructor_class(class) => {
                Some(class.clone())
            }
            ASTNode::RecordLiteral {
                record_type_name, ..
            } => Some(record_type_name.clone()),
            ASTNode::RecordUpdate { base, .. } => self.record_name_for_value_base(base),
            _ => None,
        }
    }

    fn lower_record_field_read_from_value(
        &mut self,
        value: ValueId,
        field: &str,
    ) -> Result<Option<ValueId>, String> {
        let Some(record) = self
            .function_state
            .compilation
            .record_local_value(value)
            .cloned()
        else {
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
            self.function_state
                .type_ctx
                .value_types
                .insert(field_value.value, ty);
        }
        Ok(Some(field_value.value))
    }

    fn begin_record_value_contract(&mut self, decl: &RecordDecl) -> (ValueId, String, String) {
        let dst = self.next_value_id();
        let contract_id = format!("record-value:{}", dst.as_u32());
        let fingerprint = crate::mir::type_contracts::record_value::record_schema_fingerprint(decl);
        (dst, contract_id, fingerprint)
    }

    fn build_checked_record_field_value(
        &mut self,
        field_index: usize,
        field: &UserBoxFieldDecl,
        expr: ASTNode,
        contract_id: &str,
        fingerprint: &str,
    ) -> Result<RecordLocalFieldValue, String> {
        let value = self.build_expression(expr)?;
        self.emit_record_field_contract_check(field_index, field, value, contract_id, fingerprint)?;
        Ok(RecordLocalFieldValue {
            name: field.name.clone(),
            declared_type_name: field.declared_type_name.clone(),
            value,
        })
    }

    fn emit_record_field_contract_check(
        &mut self,
        field_index: usize,
        field: &UserBoxFieldDecl,
        value: ValueId,
        contract_id: &str,
        fingerprint: &str,
    ) -> Result<(), String> {
        if !crate::mir::type_contracts::record_value::is_active_record_field_type(
            field.declared_type_name.as_deref(),
        ) {
            return Ok(());
        }
        self.emit_instruction(MirInstruction::RecordFieldContractCheck {
            contract_id: contract_id.to_string(),
            schema_fingerprint: fingerprint.to_string(),
            field_index: field_index as u32,
            value,
        })
    }

    #[allow(clippy::too_many_arguments)]
    fn publish_record_local_fields(
        &mut self,
        dst: ValueId,
        contract_id: String,
        boundary: RecordValueBoundaryKind,
        record_name: String,
        fingerprint: String,
        base: Option<ValueId>,
        field_values: Vec<RecordLocalFieldValue>,
    ) -> Result<ValueId, String> {
        for (field_index, field_value) in field_values.iter().enumerate() {
            let function = self
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[type/typed_array_contract_carrier_missing] function=<none>".to_string()
                })?;
            let typed_contract =
                crate::mir::type_contracts::typed_array::register_instruction_source(
                    function,
                    match boundary {
                        RecordValueBoundaryKind::Construct => {
                            crate::mir::function::TypedArrayContractBoundary::RecordConstruct
                        }
                        RecordValueBoundaryKind::WithUpdate => {
                            crate::mir::function::TypedArrayContractBoundary::RecordWithUpdate
                        }
                    },
                    crate::mir::function::TypedArrayContractSourceIdentity::RecordField {
                        schema_fingerprint: fingerprint.clone(),
                        field_index,
                        update: boundary == RecordValueBoundaryKind::WithUpdate,
                    },
                    field_value.value,
                    field_value.declared_type_name.as_deref(),
                    &format!("record:{}:{}", contract_id, field_index),
                )?;
            if let Some(contract_id) = typed_contract {
                self.emit_instruction(MirInstruction::ArrayStateContractClaim {
                    contract_id,
                    array: field_value.value,
                })?;
            }
        }
        self.emit_instruction(MirInstruction::RecordValuePublish {
            dst,
            contract_id,
            boundary,
            diagnostic_record_name: record_name.clone(),
            schema_fingerprint: fingerprint,
            base,
            fields: field_values.iter().map(|field| field.value).collect(),
        })?;
        self.function_state
            .compilation
            .register_record_local_value(dst, record_name, field_values);
        Ok(dst)
    }
}
