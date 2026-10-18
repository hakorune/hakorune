// Field access and assignment lowering
use super::ValueId;
use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawLegacyChildLoweringPortV1,
};

mod assignment;
mod post_success;
mod store_post_success;
mod checked_read;

pub(in crate::mir::builder) use assignment::{
    lower_prepared_raw_field_assignment_with_port_v1, PreparedRawFieldAssignmentV1,
};

pub(in crate::mir::builder) struct PreparedRawFieldReadV1 {
    route: PreparedRawFieldReadRouteV1,
}

/// Which sealed claim family owns one exact `receiver.field` read —
/// decides the destination type and the ledger record family. `Local`
/// carries the issuer's declared-type classification: `Scalar` reads an
/// `Integer`, `Alias` reads a `Box` handle of the declared class.
pub(in crate::mir::builder) enum PreparedExactFieldReadClaimV1 {
    Terminal,
    CheckedLocal {
        source: std::rc::Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>,
        read: std::rc::Rc<crate::mir::builder::ExactLexicalReadV1>,
        state: std::rc::Rc<std::cell::RefCell<crate::mir::builder::normal_callable_semantic_lowering_state::CallableSemanticLoweringState>>,
    },
    Local(std::rc::Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>),
}

enum PreparedRawFieldReadRouteV1 {
    SourceArray {
        object: ASTNode,
        expected_base: ValueId,
        field: String,
        source: std::rc::Rc<crate::mir::normal_callable_semantic_package::ObjectFieldClassSourceV1>,
    },
    ResultPath {
        object: ASTNode,
        site: crate::mir::resolved_semantics::OwnedExprSiteV1,
        ledger: std::rc::Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    },
    ExactObject {
        object: ASTNode,
        base: ValueId,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        site: crate::mir::resolved_semantics::OwnedExprSiteV1,
        ledger: std::rc::Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
        claim: PreparedExactFieldReadClaimV1,
    },
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
    pub(in crate::mir::builder) fn source_array(object: ASTNode, expected_base: ValueId, field: String,
        source: std::rc::Rc<crate::mir::normal_callable_semantic_package::ObjectFieldClassSourceV1>,
    ) -> Self { Self { route: PreparedRawFieldReadRouteV1::SourceArray { object, expected_base, field, source } } }
    pub(in crate::mir::builder) fn result_path(object: ASTNode,
        site: crate::mir::resolved_semantics::OwnedExprSiteV1,
        ledger: std::rc::Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
    ) -> Self { Self { route: PreparedRawFieldReadRouteV1::ResultPath { object, site, ledger } } }
    pub(in crate::mir::builder) fn exact_object(
        object: ASTNode,
        base: ValueId,
        field: hakorune_mir_defs::CanonicalFieldRefV1,
        site: crate::mir::resolved_semantics::OwnedExprSiteV1,
        ledger: std::rc::Rc<crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1>,
        claim: PreparedExactFieldReadClaimV1,
    ) -> Self {
        Self {
            route: PreparedRawFieldReadRouteV1::ExactObject {
                object,
                base,
                field,
                site,
                ledger,
                claim,
            },
        }
    }
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
        matches!(
            self.route,
            PreparedRawFieldReadRouteV1::Dynamic { .. }
                | PreparedRawFieldReadRouteV1::SourceArray { .. }
                | PreparedRawFieldReadRouteV1::ResultPath { .. }
        ) || matches!(&self.route, PreparedRawFieldReadRouteV1::ExactObject { claim, .. }
            if !matches!(claim, PreparedExactFieldReadClaimV1::CheckedLocal { .. })
        )
    }
}

impl super::MirBuilder {
    /// Sole exact-object field append. Call/field source owners supply the
    /// already-qualified field and representation; this never selects a field.
    pub(in crate::mir::builder) fn append_exact_object_field_read_v1(
        &mut self, base: ValueId, field: hakorune_mir_defs::CanonicalFieldRefV1,
        ty: crate::mir::MirType,
    ) -> Result<(ValueId, (crate::mir::BasicBlockId, crate::mir::MirInstruction)), String> {
        self.append_exact_object_field_read_with_destination_v1(base, field, ty, None)
    }

    fn append_exact_object_field_read_with_destination_v1(
        &mut self, base: ValueId, field: hakorune_mir_defs::CanonicalFieldRefV1,
        ty: crate::mir::MirType, destination: Option<ValueId>,
    ) -> Result<(ValueId, (crate::mir::BasicBlockId, crate::mir::MirInstruction)), String> {
        let block = self.function_state.current_block
            .ok_or("[freeze:contract][ordinary-field-read/no-block]")?;
        let dst = destination.unwrap_or_else(|| self.next_value_id());
        let instruction = crate::mir::MirInstruction::ObjectFieldGet { dst, base, field };
        self.emit_instruction(instruction.clone())?;
        self.function_state.type_ctx.value_types.insert(dst, ty.clone());
        if let Some(function) = self.function_state.current_function.as_mut() {
            function.metadata.value_types.insert(dst, ty);
        }
        Ok((dst, (block, instruction)))
    }

    /// Both raw FieldAccess and a retained expression consume this same
    /// append/type/record transaction, preserving the original affine row.
    pub(in crate::mir::builder) fn complete_exact_field_read_v1(
        &mut self, base: ValueId, field: hakorune_mir_defs::CanonicalFieldRefV1,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        claim: PreparedExactFieldReadClaimV1,
    ) -> Result<(ValueId, (crate::mir::BasicBlockId, crate::mir::MirInstruction)), String> {
        self.complete_exact_field_read_with_destination_v1(base, field, site, ledger, claim, None)
    }

    /// Plan destinations are allocations, not source/type authority. The same
    /// source transaction and sole append owner serve both raw and Plan calls.
    pub(in crate::mir::builder) fn complete_exact_field_read_with_destination_v1(
        &mut self, base: ValueId, field: hakorune_mir_defs::CanonicalFieldRefV1,
        site: &crate::mir::resolved_semantics::OwnedExprSiteV1,
        ledger: &crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1,
        claim: PreparedExactFieldReadClaimV1, destination: Option<ValueId>,
    ) -> Result<(ValueId, (crate::mir::BasicBlockId, crate::mir::MirInstruction)), String> {
        if let Some(dst) = destination {
            let function = self.function_state.current_function.as_ref()
                .ok_or("[freeze:contract][ordinary-field-read/no-function]")?;
            if function.params.contains(&dst) || function.blocks.values().any(|block|
                block.all_instructions().any(|instruction| instruction.dst_value() == Some(dst))) {
                return Err("[freeze:contract][ordinary-field-read/destination-already-defined]".into());
            }
        }
        if let PreparedExactFieldReadClaimV1::CheckedLocal { source, read, state } = &claim {
            return self.complete_source_checked_field_read_v1(base, field, site, ledger, source,
                std::rc::Rc::clone(read), &mut state.borrow_mut(), destination);
        }
        if matches!(&claim, PreparedExactFieldReadClaimV1::Local(source) if source.requires_checked_field_read_v1()) {
            return Err("[freeze:contract][ordinary-field-read/checked-fault-context-missing]".into());
        }
        let ty = match &claim {
            PreparedExactFieldReadClaimV1::Terminal => crate::mir::MirType::Integer,
            PreparedExactFieldReadClaimV1::CheckedLocal { .. } => return Err("[freeze:contract][ordinary-field-read/checked-dispatch]".into()),
            PreparedExactFieldReadClaimV1::Local(source) => match source.result() {
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Scalar => crate::mir::MirType::Integer,
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::Alias(class) => crate::mir::MirType::Box(class.to_string()),
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedArray(_)
                | crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::DeclaredArray(_) => crate::mir::MirType::Box("ArrayBox".into()),
            },
        };
        if let PreparedExactFieldReadClaimV1::Local(source) = &claim {
            let array_field = match source.result() {
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::BorrowedArray(array) =>
                    Some((array.requirement.owner(), array.requirement.field_site(), array.requirement.field_name())),
                crate::mir::resolved_semantics::home_new_prefix::LocalFieldReadResultV1::DeclaredArray(array) =>
                    Some((array.site().owner(), array.site().site(), array.name())),
                _ => None,
            };
            if let Some((owner, source_site, field_name)) = array_field {
                if source.field() != field || source_site != site.site() || owner != site.owner() {
                    return Err("[freeze:contract][ordinary-field-read/array-source-drift]".into());
                }
                let block = self.function_state.current_block.ok_or("[freeze:contract][ordinary-field-read/no-block]")?;
                let dst = self.emit_field_access_from_value_with_destination_v1(base, field_name.to_owned(), destination, Some(ty))?;
                let mut emitted = self.function_state.current_function.as_ref().and_then(|f| f.blocks.get(&block))
                    .into_iter().flat_map(|b| b.all_instructions()).filter(|instruction|
                        matches!(instruction, crate::mir::MirInstruction::FieldGet { dst: actual, .. } if *actual == dst));
                let instruction = emitted.next().cloned().ok_or("[freeze:contract][ordinary-field-read/array-emission-missing]")?;
                if emitted.next().is_some() { return Err("[freeze:contract][ordinary-field-read/array-emission-duplicate]".into()); }
                ledger.record_local_field_read_binding(site, block, instruction.clone(), source)?;
                return Ok((dst, (block, instruction)));
            }
        }
        let (dst, binding) = self.append_exact_object_field_read_with_destination_v1(base, field, ty, destination)?;
        match claim {
            PreparedExactFieldReadClaimV1::CheckedLocal { .. } => return Err("[freeze:contract][ordinary-field-read/checked-dispatch]".into()),
            PreparedExactFieldReadClaimV1::Terminal => ledger.record_terminal_field_read(site, binding.0, dst, base, field)?,
            PreparedExactFieldReadClaimV1::Local(source) => ledger.record_local_field_read(site, binding.0, dst, base, field, &source)?,
        }
        Ok((dst, binding))
    }

    pub(in crate::mir::builder) fn lower_prepared_raw_field_read_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        prepared: PreparedRawFieldReadV1,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        match prepared.route {
            PreparedRawFieldReadRouteV1::SourceArray { object, expected_base, field, source } => {
                if source.array_field().is_none() { return Err("[freeze:contract][ordinary-field-read/array-source-unavailable]".into()); }
                let base = drive_legacy_expression_v1(self, port, object)?;
                if base != expected_base { return Err("[freeze:contract][ordinary-field-read/receiver-value-mismatch]".into()); }
                self.emit_field_access_from_value_with_destination_v1(base,
                    field, None, Some(crate::mir::MirType::Box(source.class().to_owned())))
            }
            PreparedRawFieldReadRouteV1::ResultPath { object, site, ledger } => {
                let base = drive_legacy_expression_v1(self, port, object)?;
                let source = ledger.bind_result_path_receiver_v1(&site, base)?;
                self.complete_exact_field_read_v1(base, source.field(), &site, &ledger,
                    PreparedExactFieldReadClaimV1::Local(source)).map(|(value, _)| value)
            }
            PreparedRawFieldReadRouteV1::ExactObject {
                object,
                base,
                field,
                site,
                ledger,
                claim,
            } => {
                if matches!(&claim, PreparedExactFieldReadClaimV1::CheckedLocal { .. }) {
                    return self.complete_exact_field_read_v1(base, field, &site, &ledger, claim).map(|(value, _)| value);
                }
                let actual = drive_legacy_expression_v1(self, port, object)?;
                if actual != base {
                    return Err(
                        "[freeze:contract][ordinary-field-read/receiver-value-mismatch]".into(),
                    );
                }
                self.complete_exact_field_read_v1(base, field, &site, &ledger, claim)
                    .map(|(value, _)| value)
            }
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
        let declared_type = self.declared_field_type_for_value(object_value, &field);
        self.emit_field_access_from_value_with_destination_v1(object_value, field, None, declared_type)
    }

    fn emit_field_access_from_value_with_destination_v1(
        &mut self, object_value: ValueId, field: String, destination: Option<ValueId>,
        declared_type: Option<crate::mir::MirType>,
    ) -> Result<ValueId, String> {
        let region = self.current_fastmem_region();
        let receiver_box_name = self
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned();
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
            let field_val = destination.unwrap_or_else(|| self.next_value_id());
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
        let field_val = destination.unwrap_or_else(|| self.next_value_id());
        self.emit_instruction(crate::mir::MirInstruction::FieldGet {
            dst: field_val,
            base: object_value,
            field,
            declared_type,
        })?;
        post_success.commit(self, field_val, object_value);

        Ok(field_val)
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
            PreparedRawFieldReadRouteV1::ResultPath { .. } => panic!("compatibility preparation cannot select source path"),
            PreparedRawFieldReadRouteV1::ExactObject { .. } => {
                panic!("compatibility preparation must not issue an exact object field read")
            }
            PreparedRawFieldReadRouteV1::ExistingRecord { .. } => "existing-record",
            PreparedRawFieldReadRouteV1::RecordConstructor { .. } => "record-constructor",
            PreparedRawFieldReadRouteV1::RecordLiteral { .. } => "record-literal",
            PreparedRawFieldReadRouteV1::RecordUpdate { .. } => "record-update",
            PreparedRawFieldReadRouteV1::Dynamic { .. } => "dynamic",
            PreparedRawFieldReadRouteV1::SourceArray { .. } => "source_array",
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

        let prepared = PreparedRawFieldReadV1::prepare(&builder, var("pair"), "value".to_string());
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
