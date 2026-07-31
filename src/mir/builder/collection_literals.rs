use crate::ast::ASTNode;
use crate::mir::builder::observe::types as type_trace;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::definitions::call_unified::TypeCertainty;
use crate::mir::ssot::method_call::runtime_method_call;
use crate::mir::{ArrayElementWriteKind, ArrayWriteProducerKind};

use super::{EffectMask, MirInstruction, MirType, ValueId};

impl super::MirBuilder {
    pub(super) fn build_array_literal(
        &mut self,
        elements: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_array_literal_with_port_v1(&mut port, elements)
    }

    /// Lower an array literal while retaining the caller's raw child port.
    pub(in crate::mir::builder) fn build_array_literal_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        elements: Vec<ASTNode>,
    ) -> Result<ValueId, String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        self.build_array_literal_with_contract_and_port_v1(port, elements, None)
            .map(|(value, _)| value)
    }

    pub(super) fn build_typed_array_literal(
        &mut self,
        elements: Vec<ASTNode>,
    ) -> Result<(ValueId, String), String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_typed_array_literal_with_port_v1(&mut port, elements)
    }

    /// Lower a typed Local array initializer without replacing the caller's
    /// recursive child port.
    pub(in crate::mir::builder) fn build_typed_array_literal_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        elements: Vec<ASTNode>,
    ) -> Result<(ValueId, String), String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        let (value, contract_id) = self.build_array_literal_with_contract_and_port_v1(
            port,
            elements,
            Some("local-literal"),
        )?;
        Ok((value, contract_id.expect("typed literal emits contract ID")))
    }

    fn build_array_literal_with_contract_and_port_v1<Port>(
        &mut self,
        port: &mut Port,
        elements: Vec<ASTNode>,
        contract_prefix: Option<&str>,
    ) -> Result<(ValueId, Option<String>), String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        let arr_id = self.next_value_id();
        self.emit_instruction(MirInstruction::NewBox {
            dst: arr_id,
            box_type: "ArrayBox".to_string(),
            args: vec![],
        })?;
        self.emit_constructor_birth_marker(arr_id, "ArrayBox")?;
        self.function_state
            .type_ctx
            .value_origin_newbox
            .insert(arr_id, "ArrayBox".to_string());
        self.function_state
            .type_ctx
            .value_types
            .insert(arr_id, MirType::Box("ArrayBox".to_string()));
        self.comp_ctx
            .type_registry
            .record_newbox(arr_id, "ArrayBox".to_string());
        self.comp_ctx
            .type_registry
            .record_type(arr_id, MirType::Box("ArrayBox".to_string()));
        type_trace::origin("newbox:ArrayLiteral", arr_id, "ArrayBox");
        type_trace::ty(
            "newbox:ArrayLiteral",
            arr_id,
            &MirType::Box("ArrayBox".to_string()),
        );
        let contract_id =
            contract_prefix.map(|prefix| format!("typed-array:{prefix}:{}", arr_id.0));
        if let Some(contract_id) = contract_id.as_ref() {
            self.emit_instruction(MirInstruction::ArrayStateContractClaim {
                contract_id: contract_id.clone(),
                array: arr_id,
            })?;
        }

        let mut element_types = Vec::new();
        for element in elements {
            let value = drive_legacy_expression_v1(self, port, element)?;
            let element_type = self
                .function_state
                .type_ctx
                .value_types
                .get(&value)
                .cloned()
                .or_else(|| {
                    self.function_state
                        .type_ctx
                        .value_origin_newbox
                        .get(&value)
                        .map(|box_name| MirType::Box(box_name.clone()))
                });
            self.emit_array_element_write(
                None,
                ArrayElementWriteKind::LiteralAppend,
                ArrayWriteProducerKind::Literal,
                arr_id,
                None,
                value,
            )?;
            element_types.push(element_type);
        }

        crate::mir::builder::types::array_element::record_array_literal_elements(
            self,
            arr_id,
            &element_types,
        );
        Ok((arr_id, contract_id))
    }

    pub(super) fn build_map_literal(
        &mut self,
        entries: Vec<(String, ASTNode)>,
    ) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_map_literal_with_port_v1(&mut port, entries)
    }

    /// Lower a map literal while retaining the caller's raw child port.
    pub(in crate::mir::builder) fn build_map_literal_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        entries: Vec<(String, ASTNode)>,
    ) -> Result<ValueId, String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        let map_id = self.next_value_id();
        self.emit_instruction(MirInstruction::NewBox {
            dst: map_id,
            box_type: "MapBox".to_string(),
            args: vec![],
        })?;
        self.emit_constructor_birth_marker(map_id, "MapBox")?;
        self.function_state
            .type_ctx
            .value_origin_newbox
            .insert(map_id, "MapBox".to_string());
        self.function_state
            .type_ctx
            .value_types
            .insert(map_id, MirType::Box("MapBox".to_string()));
        self.comp_ctx
            .type_registry
            .record_newbox(map_id, "MapBox".to_string());
        self.comp_ctx
            .type_registry
            .record_type(map_id, MirType::Box("MapBox".to_string()));
        type_trace::origin("newbox:MapLiteral", map_id, "MapBox");
        type_trace::ty(
            "newbox:MapLiteral",
            map_id,
            &MirType::Box("MapBox".to_string()),
        );

        for (key, expr) in entries {
            let key_id = crate::mir::builder::emission::constant::emit_string(self, key)?;
            let value_id = drive_legacy_expression_v1(self, port, expr)?;
            self.emit_instruction(runtime_method_call(
                None,
                map_id,
                "MapBox",
                "set",
                vec![key_id, value_id],
                EffectMask::MUT,
                TypeCertainty::Known,
            ))?;
        }
        Ok(map_id)
    }
}

#[cfg(test)]
mod tests {
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::module_draft_collector::ModuleDraftCollectorV1;
    use crate::mir::builder::module_lowering_invocation::ModuleLoweringInvocationV1;
    use crate::mir::builder::raw_invocation_source_transport::{
        RawInvocationSourceTransportV1, RawSourceTransportPortV1,
    };
    use crate::mir::builder::recursive_child_lowering::{
        drive_legacy_expression_v1, drive_raw_legacy_expression_v1, RawInvocationChildPortV1,
        RecursiveChildLoweringPortV1,
    };
    use crate::mir::{Callee, EffectMask, MirBuilder, MirInstruction, MirType};

    fn integer(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: Span::unknown(),
        }
    }

    fn boolean(value: bool) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Bool(value),
            span: Span::unknown(),
        }
    }

    fn array(elements: Vec<ASTNode>) -> ASTNode {
        ASTNode::ArrayLiteral {
            elements,
            span: Span::unknown(),
        }
    }

    fn nested_array() -> ASTNode {
        array(vec![
            array(vec![integer(1), integer(2)]),
            array(vec![integer(3), integer(4)]),
        ])
    }

    fn map(entries: Vec<(&str, ASTNode)>) -> ASTNode {
        ASTNode::MapLiteral {
            entries: entries
                .into_iter()
                .map(|(key, value)| (key.to_owned(), value))
                .collect(),
            span: Span::unknown(),
        }
    }

    fn nested_map() -> ASTNode {
        map(vec![
            ("dup", integer(1)),
            ("dup", integer(2)),
            ("nested", map(vec![("inner", boolean(true))])),
        ])
    }

    fn spanned_instructions(builder: &MirBuilder) -> Vec<(String, Span)> {
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("current function")
            .blocks
            .values()
            .flat_map(|block| block.all_spanned_instructions())
            .map(|instruction| (format!("{:?}", instruction.inst), instruction.span))
            .collect()
    }

    fn array_write_count(builder: &MirBuilder) -> usize {
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("current function")
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter(|instruction| matches!(instruction, MirInstruction::ArrayElementWrite { .. }))
            .count()
    }

    fn map_set_count(builder: &MirBuilder) -> usize {
        builder
            .function_state
            .current_function
            .as_ref()
            .expect("current function")
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .filter(|instruction| {
                matches!(
                    instruction,
                    MirInstruction::Call {
                        callee: Some(Callee::Method {
                            box_name,
                            method,
                            ..
                        }),
                        effects,
                        ..
                    } if box_name == "MapBox"
                        && method == "set"
                        && *effects == EffectMask::MUT
                )
            })
            .count()
    }

    #[test]
    fn selected_array_port_matches_raw_legacy_state_exactly() {
        for (root, expected_type, expected_writes) in [
            (array(Vec::new()), MirType::Box("ArrayBox".to_owned()), 0),
            (
                array(vec![integer(1), integer(2)]),
                MirType::Array(Box::new(MirType::Integer)),
                2,
            ),
            (
                array(vec![integer(1), boolean(true)]),
                MirType::Array(Box::new(MirType::Unknown)),
                2,
            ),
            (
                nested_array(),
                MirType::Array(Box::new(MirType::Array(Box::new(MirType::Integer)))),
                6,
            ),
        ] {
            let mut legacy = MirBuilder::new();
            legacy.enter_function_for_test("array_port_parity/0".to_owned());
            let legacy_value =
                drive_raw_legacy_expression_v1(&mut legacy, root.clone()).expect("legacy Array");

            let mut selected = MirBuilder::new();
            selected.enter_function_for_test("array_port_parity/0".to_owned());
            let selected_value = {
                let mut invocation = ModuleLoweringInvocationV1::with_collector(
                    &mut selected,
                    ModuleDraftCollectorV1::default(),
                );
                invocation.with_module_port(|builder, module_port| {
                    let mut port = RawInvocationChildPortV1::new(module_port);
                    port.with_source_transport_v1(
                        RawInvocationSourceTransportV1::script_root(ASTNode::Program {
                            statements: vec![root],
                            span: Span::unknown(),
                        }),
                        |port, program| {
                            let ASTNode::Program { statements, .. } = program else {
                                unreachable!("selected Array test installs a Program root")
                            };
                            port.lower_body(builder, statements)
                        },
                    )
                })
            }
            .expect("selected Array");

            assert_eq!(selected_value, legacy_value);
            assert_eq!(
                spanned_instructions(&selected),
                spanned_instructions(&legacy)
            );
            assert_eq!(array_write_count(&selected), expected_writes);
            assert_eq!(array_write_count(&selected), array_write_count(&legacy));
            assert_eq!(
                selected
                    .function_state
                    .type_ctx
                    .value_origin_newbox
                    .get(&selected_value),
                legacy
                    .function_state
                    .type_ctx
                    .value_origin_newbox
                    .get(&legacy_value)
            );
            assert_eq!(
                selected.comp_ctx.type_registry.get_origin(selected_value),
                legacy.comp_ctx.type_registry.get_origin(legacy_value)
            );
            assert_eq!(
                selected
                    .function_state
                    .type_ctx
                    .value_types
                    .get(&selected_value),
                Some(&expected_type)
            );
            assert_eq!(
                selected
                    .function_state
                    .type_ctx
                    .value_types
                    .get(&selected_value),
                legacy
                    .function_state
                    .type_ctx
                    .value_types
                    .get(&legacy_value)
            );
        }
    }

    fn assert_selected_map_parity() -> Vec<(String, Span)> {
        let mut legacy = MirBuilder::new();
        legacy.enter_function_for_test("map_port_parity/0".to_owned());
        let legacy_value =
            drive_raw_legacy_expression_v1(&mut legacy, nested_map()).expect("legacy Map");

        let mut selected = MirBuilder::new();
        selected.enter_function_for_test("map_port_parity/0".to_owned());
        let selected_value = {
            let mut invocation = ModuleLoweringInvocationV1::with_collector(
                &mut selected,
                ModuleDraftCollectorV1::default(),
            );
            invocation.with_module_port(|builder, module_port| {
                let mut port = RawInvocationChildPortV1::new(module_port);
                port.with_source_transport_v1(
                    RawInvocationSourceTransportV1::script_root(ASTNode::Program {
                        statements: vec![nested_map()],
                        span: Span::unknown(),
                    }),
                    |port, program| {
                        let ASTNode::Program { statements, .. } = program else {
                            unreachable!("selected Map test installs a Program root")
                        };
                        port.lower_body(builder, statements)
                    },
                )
            })
        }
        .expect("selected Map");

        assert_eq!(selected_value, legacy_value);
        assert_eq!(
            spanned_instructions(&selected),
            spanned_instructions(&legacy)
        );
        assert_eq!(map_set_count(&selected), 4);
        assert_eq!(map_set_count(&selected), map_set_count(&legacy));
        assert_eq!(
            selected
                .function_state
                .type_ctx
                .value_origin_newbox
                .get(&selected_value),
            Some(&"MapBox".to_owned())
        );
        assert_eq!(
            selected
                .function_state
                .type_ctx
                .value_origin_newbox
                .get(&selected_value),
            legacy
                .function_state
                .type_ctx
                .value_origin_newbox
                .get(&legacy_value)
        );
        assert_eq!(
            selected.comp_ctx.type_registry.get_origin(selected_value),
            legacy.comp_ctx.type_registry.get_origin(legacy_value)
        );
        assert_eq!(
            selected
                .function_state
                .type_ctx
                .value_types
                .get(&selected_value),
            Some(&MirType::Box("MapBox".to_owned()))
        );
        assert_eq!(
            selected.function_state.type_ctx.string_literals,
            legacy.function_state.type_ctx.string_literals
        );
        assert_eq!(
            selected
                .function_state
                .type_ctx
                .string_literals
                .values()
                .filter(|key| key.as_str() == "dup")
                .count(),
            2
        );
        assert_eq!(
            selected.function_state.local_ssa_map,
            legacy.function_state.local_ssa_map
        );
        assert!(
            selected.function_state.type_ctx.map_value_types.is_empty()
                && selected
                    .function_state
                    .type_ctx
                    .map_literal_value_types
                    .is_empty()
        );
        assert_eq!(
            selected.core_ctx.peek_next_value(),
            legacy.core_ctx.peek_next_value()
        );
        spanned_instructions(&selected)
    }

    #[test]
    fn selected_map_port_matches_raw_legacy_and_unified_modes_exactly() {
        let off = crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "off", || {
            assert_selected_map_parity()
        });
        let on = crate::test_support::with_env_var("NYASH_MIR_UNIFIED_CALL", "1", || {
            assert_selected_map_parity()
        });
        assert_eq!(off, on);
    }

    #[test]
    fn selected_map_child_failure_stops_before_set_and_later_entry() {
        let root = map(vec![
            ("before", integer(1)),
            (
                "missing",
                ASTNode::Variable {
                    name: "missing".to_owned(),
                    span: Span::unknown(),
                },
            ),
            ("after", integer(2)),
        ]);
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("map_failure/0".to_owned());
        let error = {
            let mut invocation = ModuleLoweringInvocationV1::with_collector(
                &mut builder,
                ModuleDraftCollectorV1::default(),
            );
            invocation.with_module_port(|builder, module_port| {
                let mut port = RawInvocationChildPortV1::new(module_port);
                drive_legacy_expression_v1(builder, &mut port, root)
            })
        }
        .expect_err("missing Map value must fail");

        assert!(error.contains("Undefined variable: missing"), "{error}");
        assert_eq!(map_set_count(&builder), 1);
        let keys = builder
            .function_state
            .type_ctx
            .string_literals
            .values()
            .map(String::as_str)
            .collect::<Vec<_>>();
        assert!(keys.contains(&"before"));
        assert!(keys.contains(&"missing"));
        assert!(!keys.contains(&"after"));
    }
}
