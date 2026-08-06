// Field access and assignment lowering
use super::weak_field_write_route::{prepare_field_write_route_v1, PreparedFieldWriteRouteV1};
use super::ValueId;
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawLegacyChildLoweringPortV1,
};
use crate::mir::instruction::FastMemRegionId;

mod post_success;
mod store_post_success;

use store_post_success::PreparedOrdinaryFieldStoreAccessSiteV1;

pub(in crate::mir::builder) struct PreparedRawFieldReadV1 {
    route: PreparedRawFieldReadRouteV1,
}

/// Ordinary Field-assignment source and record-target admission prepared once.
pub(in crate::mir::builder) struct PreparedRawFieldAssignmentV1 {
    object: ASTNode,
    field: String,
    value: ASTNode,
}

impl PreparedRawFieldAssignmentV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &super::MirBuilder,
        object: ASTNode,
        field: String,
        value: ASTNode,
    ) -> Result<Self, String> {
        builder.fail_if_record_field_assignment_target(&object, &field)?;
        Ok(Self {
            object,
            field,
            value,
        })
    }
}

pub(in crate::mir::builder) fn lower_prepared_raw_field_assignment_with_port_v1<Port>(
    builder: &mut super::MirBuilder,
    port: &mut Port,
    prepared: PreparedRawFieldAssignmentV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let PreparedRawFieldAssignmentV1 {
        object,
        field,
        value,
    } = prepared;
    let object_value = drive_legacy_expression_v1(builder, port, object)?;
    let object_value = builder.local_field_base(object_value);
    builder.build_field_assignment_from_value_with_port_v1(port, object_value, field, value)
}

enum PreparedRawFieldReadRouteV1 {
    ExistingRecord {
        value: ValueId,
        field: String,
    },
    RecordConstructor {
        class: String,
        arguments: Vec<ASTNode>,
        field: String,
    },
    RecordLiteral {
        record_type_name: String,
        fields: Vec<(String, ASTNode)>,
        field: String,
    },
    RecordUpdate {
        base: ASTNode,
        updates: Vec<(String, ASTNode)>,
        field: String,
    },
    Dynamic {
        object: ASTNode,
        field: String,
    },
}

impl PreparedRawFieldReadV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &super::MirBuilder,
        object: ASTNode,
        field: String,
    ) -> Self {
        if let ASTNode::Variable { name, .. } = &object {
            let record_value = builder
                .function_state
                .variable_ctx
                .variable_map
                .get(name)
                .copied()
                .filter(|value| {
                    builder
                        .function_state
                        .compilation
                        .record_local_value(*value)
                        .is_some()
                });
            if let Some(value) = record_value {
                return Self {
                    route: PreparedRawFieldReadRouteV1::ExistingRecord { value, field },
                };
            }
        }
        let route = match object {
            ASTNode::New {
                class, arguments, ..
            } if builder.is_record_constructor_class(&class) => {
                PreparedRawFieldReadRouteV1::RecordConstructor {
                    class,
                    arguments,
                    field,
                }
            }
            ASTNode::RecordLiteral {
                record_type_name,
                fields,
                ..
            } => PreparedRawFieldReadRouteV1::RecordLiteral {
                record_type_name,
                fields,
                field,
            },
            ASTNode::RecordUpdate { base, updates, .. } => {
                PreparedRawFieldReadRouteV1::RecordUpdate {
                    base: *base,
                    updates,
                    field,
                }
            }
            object => PreparedRawFieldReadRouteV1::Dynamic { object, field },
        };
        Self { route }
    }

    pub(in crate::mir::builder) fn requires_receiver_source_v1(&self) -> bool {
        matches!(self.route, PreparedRawFieldReadRouteV1::Dynamic { .. })
    }
}

impl super::MirBuilder {
    pub(in crate::mir::builder) fn lower_prepared_raw_field_read_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        prepared: PreparedRawFieldReadV1,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        match prepared.route {
            PreparedRawFieldReadRouteV1::ExistingRecord { value, field } => {
                self.lower_prepared_record_field_read_from_value(value, &field)
            }
            PreparedRawFieldReadRouteV1::RecordConstructor {
                class,
                arguments,
                field,
            } => {
                let value =
                    self.build_record_constructor_value_with_port_v1(port, class, arguments)?;
                self.lower_prepared_record_field_read_from_value(value, &field)
            }
            PreparedRawFieldReadRouteV1::RecordLiteral {
                record_type_name,
                fields,
                field,
            } => {
                let value =
                    self.build_record_literal_value_with_port_v1(port, record_type_name, fields)?;
                self.lower_prepared_record_field_read_from_value(value, &field)
            }
            PreparedRawFieldReadRouteV1::RecordUpdate {
                base,
                updates,
                field,
            } => {
                let value = self.build_record_update_value_with_port_v1(port, base, updates)?;
                self.lower_prepared_record_field_read_from_value(value, &field)
            }
            PreparedRawFieldReadRouteV1::Dynamic { object, field } => {
                let object_value = drive_legacy_expression_v1(self, port, object)?;
                let object_value = self.local_field_base(object_value);
                if let Some(property_value) =
                    self.try_lower_property_read_with_port_v1(port, object_value, &field)?
                {
                    return Ok(property_value);
                }
                self.build_field_access_from_value(object_value, field)
            }
        }
    }

    pub(super) fn build_box_field_initializers(
        &mut self,
        object_value: ValueId,
        class: &str,
        field_initializers: Vec<(String, ASTNode)>,
    ) -> Result<(), String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_box_field_initializers_with_port_v1(
            &mut port,
            object_value,
            class,
            field_initializers,
        )
    }

    /// Lower box field initializers without dropping the raw child port.
    pub(in crate::mir::builder) fn build_box_field_initializers_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        object_value: ValueId,
        class: &str,
        field_initializers: Vec<(String, ASTNode)>,
    ) -> Result<(), String>
    where
        Port: RawAstChildLoweringPortV1,
    {
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
            self.build_field_assignment_from_value_with_port_v1(port, object_value, field, value)?;
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
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned();
        let declared_type = self.declared_field_type_for_value(object_value, &field);
        if let Some(region) = region {
            let field_result_origin = self.inferred_field_result_class(object_value, &field);
            let lifecycle =
                super::fastmem::field_load::PreparedFastMemFieldLoadLifecycleV1::prepare(
                    declared_type.as_ref(),
                    receiver_box_name.as_deref(),
                    &field,
                    field_result_origin.as_deref(),
                );
            lifecycle.reserve_site(self, region, object_value)?;
            let field_val = self.next_value_id();
            lifecycle.reserve_declared_type(self, field_val);
            self.emit_fastmem_memop(
                region,
                crate::mir::instruction::MemOpKind::FieldLoad,
                Some(field_val),
                vec![object_value],
                Some(crate::mir::instruction::MemOpAccess::field(field.clone())),
            )?;
            lifecycle.complete_after_success(self, field_val);
            return Ok(field_val);
        }

        let field_result_origin = self.inferred_field_result_class(object_value, &field);
        let post_success = post_success::PreparedOrdinaryFieldGetPostSuccessV1::prepare(
            declared_type.as_ref(),
            receiver_box_name.as_deref(),
            &field,
            field_result_origin.as_deref(),
        )
        .map_err(|error| error.to_string())?;
        let field_val = self.next_value_id();
        self.emit_instruction(crate::mir::MirInstruction::FieldGet {
            dst: field_val,
            base: object_value,
            field,
            declared_type,
        })?;
        post_success.commit(self, field_val, object_value);

        Ok(field_val)
    }

    pub(super) fn build_field_assignment_from_value(
        &mut self,
        object_value: ValueId,
        field: String,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_field_assignment_from_value_with_port_v1(&mut port, object_value, field, value)
    }

    fn build_field_assignment_from_value_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        object_value: ValueId,
        field: String,
        value: ASTNode,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        let mut value_result = drive_legacy_expression_v1(self, port, value)?;
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
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned();
        let declared_type = self.declared_field_type_for_value(object_value, &field);

        let route = prepare_field_write_route_v1(
            region,
            object_value,
            &field,
            value_result,
            receiver_box_name.as_deref(),
            receiver_box_name
                .as_ref()
                .and_then(|owner| self.comp_ctx.user_box_field_decls.get(owner))
                .map(Vec::as_slice),
        );
        let contract_identity = self.declared_field_contract_identity(object_value, &field);
        let is_typed_array = contract_identity
            .as_ref()
            .and_then(|(_, _, declared)| declared.as_deref())
            .map(crate::typed_array_contract_spec::parse_annotation)
            .transpose()?
            .flatten()
            .is_some();
        let contract_identity = is_typed_array.then_some(contract_identity).flatten();
        let has_contract_identity = contract_identity.is_some();
        let mut is_known_weak = false;
        let ordinary_receipt = match route {
            PreparedFieldWriteRouteV1::KnownWeak(prepared) => {
                is_known_weak = true;
                self.record_field_access_site(
                    region,
                    object_value,
                    receiver_box_name.clone(),
                    field.clone(),
                    None,
                    "store",
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
                self.emit_prepared_known_weak_field_write(prepared)?;
                None
            }
            PreparedFieldWriteRouteV1::Ordinary(prepared) => {
                if region.is_none() && contract_identity.is_none() {
                    Some(PreparedOrdinaryFieldStoreAccessSiteV1::prepare(
                        self.metadata_ctx.current_span(),
                        prepared.base,
                        receiver_box_name.as_deref(),
                        &prepared.field,
                    ))
                } else {
                    self.record_field_access_site(
                        region,
                        object_value,
                        receiver_box_name.clone(),
                        field.clone(),
                        None,
                        "store",
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
                    None
                }
            }
        };

        if let Some((box_name, field_index, declared_name)) = contract_identity.clone() {
            let function = self
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[type/typed_array_contract_carrier_missing] function=<none>".to_string()
                })?;
            let contract_id = crate::mir::type_contracts::typed_array::register_instruction_source(
                function,
                crate::mir::function::TypedArrayContractBoundary::BoxFieldWrite,
                crate::mir::function::TypedArrayContractSourceIdentity::BoxField {
                    box_name,
                    field_index,
                },
                value_result,
                declared_name.as_deref(),
                &format!(
                    "box-field:{}:{}:{}",
                    object_value.as_u32(),
                    field,
                    value_result.as_u32()
                ),
            )?;
            if let Some(contract_id) = contract_id {
                self.emit_instruction(crate::mir::MirInstruction::ArrayStateContractClaim {
                    contract_id,
                    array: value_result,
                })?;
            }
        }

        if is_known_weak {
            // WeakFieldWrite owns validation, publication, and bookkeeping.
        } else if ordinary_receipt.is_none() && region.is_none() && !has_contract_identity {
            // The prepared ordinary receipt is committed below after FieldSet.
        } else if has_contract_identity {
            // The typed-array contract lane owns its existing FieldSet timing.
        } else if let Some(region) = region {
            self.emit_fastmem_memop(
                region,
                crate::mir::instruction::MemOpKind::FieldStore,
                None,
                vec![object_value, value_result],
                Some(crate::mir::instruction::MemOpAccess::field(field.clone())),
            )?;
        } else {
            self.emit_instruction(crate::mir::MirInstruction::FieldSet {
                base: object_value,
                field: field.clone(),
                value: value_result,
                declared_type,
            })?;
        }

        if let Some(receipt) = ordinary_receipt {
            receipt.commit(self)?;
        }

        // Record origin class for this field value if known
        if let Some(val_cls) = self
            .function_state
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
                .function_state
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
    use super::{
        super::MirBuilder, PreparedRawFieldAssignmentV1, PreparedRawFieldReadRouteV1,
        PreparedRawFieldReadV1,
    };
    use crate::ast::{ASTNode, FieldDecl, LiteralValue, Span};

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

    fn prepared_route(builder: &MirBuilder, object: ASTNode) -> &'static str {
        match PreparedRawFieldReadV1::prepare(builder, object, "value".to_string()).route {
            PreparedRawFieldReadRouteV1::ExistingRecord { .. } => "existing-record",
            PreparedRawFieldReadRouteV1::RecordConstructor { .. } => "record-constructor",
            PreparedRawFieldReadRouteV1::RecordLiteral { .. } => "record-literal",
            PreparedRawFieldReadRouteV1::RecordUpdate { .. } => "record-update",
            PreparedRawFieldReadRouteV1::Dynamic { .. } => "dynamic",
        }
    }

    fn builder_with_pair_record(function: &str) -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder.comp_ctx.register_record_decl(
            "Pair".to_string(),
            Vec::new(),
            &[FieldDecl {
                name: "value".to_string(),
                declared_type_name: None,
                is_weak: false,
                default_value: None,
            }],
        );
        builder.enter_function_for_test(function.to_string());
        let existing = builder.alloc_value_for_test();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("pair".to_string(), existing);
        builder
            .function_state
            .compilation
            .register_record_local_value(existing, "Pair".to_string(), Vec::new());
        builder
    }

    #[test]
    fn field_read_source_route_is_total_and_disjoint() {
        let builder = builder_with_pair_record("field_read_route/0");

        assert_eq!(prepared_route(&builder, var("pair")), "existing-record");
        assert_eq!(
            prepared_route(
                &builder,
                ASTNode::New {
                    class: "Pair".to_string(),
                    arguments: vec![int_lit(1)],
                    field_initializers: Vec::new(),
                    type_arguments: Vec::new(),
                    span: span(),
                },
            ),
            "record-constructor"
        );
        assert_eq!(
            prepared_route(
                &builder,
                ASTNode::RecordLiteral {
                    record_type_name: "Pair".to_string(),
                    fields: vec![("value".to_string(), int_lit(1))],
                    span: span(),
                },
            ),
            "record-literal"
        );
        assert_eq!(
            prepared_route(
                &builder,
                ASTNode::RecordUpdate {
                    base: Box::new(var("pair")),
                    updates: vec![("value".to_string(), int_lit(2))],
                    span: span(),
                },
            ),
            "record-update"
        );
        assert_eq!(prepared_route(&builder, var("ordinary")), "dynamic");
    }

    #[test]
    fn dynamic_field_read_requires_receiver_source() {
        let builder = builder_with_pair_record("field_read_receiver/0");
        let prepared = PreparedRawFieldReadV1::prepare(
            &builder,
            field(var("value"), "field"),
            "field".to_string(),
        );
        assert!(prepared.requires_receiver_source_v1());

        let prepared = PreparedRawFieldReadV1::prepare(
            &builder,
            var("pair"),
            "value".to_string(),
        );
        assert!(!prepared.requires_receiver_source_v1());
    }

    #[test]
    fn field_assignment_prepares_record_rejection_before_child_descent() {
        let builder = builder_with_pair_record("field_assignment_prepare/0");

        let rejected = vec![
            (
                var("pair"),
                "[record-field-set/unsupported] name=pair record=Pair field=value",
            ),
            (
                ASTNode::New {
                    class: "Pair".to_string(),
                    arguments: Vec::new(),
                    field_initializers: Vec::new(),
                    type_arguments: Vec::new(),
                    span: span(),
                },
                "[record-field-set/unsupported] record=Pair field=value",
            ),
            (
                ASTNode::RecordLiteral {
                    record_type_name: "Pair".to_string(),
                    fields: Vec::new(),
                    span: span(),
                },
                "[record-field-set/unsupported] record=Pair field=value",
            ),
            (
                ASTNode::RecordUpdate {
                    base: Box::new(var("ordinary")),
                    updates: Vec::new(),
                    span: span(),
                },
                "[record-field-set/unsupported] record-update field=value",
            ),
        ];
        for (object, expected) in rejected {
            let error = match PreparedRawFieldAssignmentV1::prepare(
                &builder,
                object,
                "value".to_string(),
                int_lit(1),
            ) {
                Ok(_) => panic!("record Field assignment must reject during preparation"),
                Err(error) => error,
            };
            assert_eq!(error, expected);
        }
        PreparedRawFieldAssignmentV1::prepare(
            &builder,
            var("ordinary"),
            "value".to_string(),
            int_lit(1),
        )
        .expect("ordinary Field assignment must prepare");
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
        let function = builder.function_state.current_function.as_ref().unwrap();

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

    #[test]
    fn ordinary_fieldset_failure_leaves_no_access_site_after_receipt_cutover() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("ordinary_fieldset_timing_failure/0".to_string());
        let base = builder.alloc_value_for_test();
        let value = builder.alloc_value_for_test();
        builder.function_state.current_block = None;

        let error = builder
            .build_field_assignment_from_value_id(None, base, "slot".to_string(), value)
            .unwrap_err();

        assert_eq!(error, "No current basic block");
        let function = builder.function_state.current_function.as_ref().unwrap();
        assert_eq!(
            function.metadata.fastmem_field_access_sites.len(),
            0,
            "ordinary receipt must not publish a site before FieldSet succeeds"
        );
        assert!(function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .all(|instruction| !matches!(
                instruction,
                crate::mir::MirInstruction::FieldSet { .. }
            )));
    }
}
