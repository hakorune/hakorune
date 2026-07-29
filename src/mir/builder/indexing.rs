use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::instruction::MemOpKind;

use super::{EffectMask, MirInstruction, MirType, ValueId};

mod static_load_type;

use static_load_type::PreparedStaticU16LoadTypeV1;

/// One source-only Index-read route selected before child lowering or Builder
/// effects. The route vocabulary stays private so callers can only consume the
/// complete preparation through the Index owner.
pub(in crate::mir::builder) struct PreparedRawIndexReadV1 {
    route: PreparedRawIndexReadRouteV1,
}

enum PreparedRawIndexReadRouteV1 {
    Static {
        plan: crate::mir::function::StaticDataPlan,
        result_type: PreparedStaticU16LoadTypeV1,
        index: ASTNode,
    },
    Dynamic {
        target: ASTNode,
        index: ASTNode,
        target_label: Option<String>,
    },
}

/// Source-only Index-assignment snapshot prepared before child descent.
pub(in crate::mir::builder) struct PreparedRawIndexAssignmentV1 {
    target: ASTNode,
    index: ASTNode,
    value: ASTNode,
    target_label: Option<String>,
}

impl PreparedRawIndexAssignmentV1 {
    pub(in crate::mir::builder) fn prepare(
        target: ASTNode,
        index: ASTNode,
        value: ASTNode,
    ) -> Self {
        let target_label = match &target {
            ASTNode::Variable { name, .. } => Some(name.clone()),
            _ => None,
        };
        Self {
            target,
            index,
            value,
            target_label,
        }
    }
}

pub(in crate::mir::builder) fn lower_prepared_raw_index_assignment_with_port_v1<Port>(
    builder: &mut super::MirBuilder,
    port: &mut Port,
    prepared: PreparedRawIndexAssignmentV1,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    let PreparedRawIndexAssignmentV1 {
        target,
        index,
        value,
        target_label,
    } = prepared;
    let target_val = drive_legacy_expression_v1(builder, port, target)?;
    let index_val = drive_legacy_expression_v1(builder, port, index)?;
    let value_val = drive_legacy_expression_v1(builder, port, value)?;
    builder.build_index_access_from_values(
        None,
        target_val,
        index_val,
        target_label,
        "store",
        Some(value_val),
    )
}

impl PreparedRawIndexReadV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &super::MirBuilder,
        target: ASTNode,
        index: ASTNode,
    ) -> Result<Self, String> {
        let (static_plan, target_label) = match &target {
            ASTNode::Variable { name, .. } => {
                let static_plan = builder.current_module.as_ref().and_then(|module| {
                    crate::mir::static_data_plan::find_static_data_plan(
                        &module.metadata.static_data_plans,
                        name,
                    )
                    .cloned()
                });
                let target_label = static_plan.is_none().then(|| name.clone());
                (static_plan, target_label)
            }
            _ => (None, None),
        };

        let route = match static_plan {
            Some(plan) => {
                let result_type = PreparedStaticU16LoadTypeV1::prepare(&plan, None)
                    .map_err(|error| error.to_string())?;
                PreparedRawIndexReadRouteV1::Static {
                    plan,
                    result_type,
                    index,
                }
            }
            None => PreparedRawIndexReadRouteV1::Dynamic {
                target,
                index,
                target_label,
            },
        };
        Ok(Self { route })
    }
}

impl super::MirBuilder {
    pub(super) fn infer_index_target_class(&self, target_val: ValueId) -> Option<String> {
        if let Some(cls) = self
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&target_val)
        {
            return Some(cls.clone());
        }
        self.function_state
            .type_ctx
            .value_types
            .get(&target_val)
            .and_then(|ty| match ty {
                MirType::Box(name) => Some(name.clone()),
                MirType::String => Some("StringBox".to_string()),
                MirType::Integer => Some("Integer".to_string()),
                MirType::Float => Some("Float".to_string()),
                _ => None,
            })
    }

    fn format_index_target_kind(class_hint: Option<&String>) -> String {
        class_hint
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("unknown")
            .to_string()
    }

    pub(super) fn build_index_access_from_values(
        &mut self,
        region: Option<FastMemRegionId>,
        target_val: ValueId,
        index_val: ValueId,
        target_label: Option<String>,
        access_kind: &'static str,
        store_value: Option<ValueId>,
    ) -> Result<ValueId, String> {
        let region = region.or_else(|| self.current_fastmem_region());
        let table_id = target_label
            .clone()
            .or_else(|| region.map(|_| format!("value_{}", target_val.0)));
        let class_hint = self.infer_index_target_class(target_val);
        let required_route = if region.is_some() {
            "verified_table_index"
        } else {
            "none"
        };
        let fallback_policy = if region.is_some() {
            "forbidden"
        } else {
            "allow_dynamic"
        };
        self.record_index_access_site(
            region,
            target_val,
            index_val,
            table_id.clone(),
            None,
            access_kind,
            required_route,
            fallback_policy,
        )?;

        match class_hint.as_deref() {
            Some("ArrayBox") => {
                if let Some(value_id) = store_value {
                    self.emit_array_element_write(
                        None,
                        crate::mir::ArrayElementWriteKind::Set,
                        if access_kind == "compound_store" {
                            crate::mir::ArrayWriteProducerKind::CompoundIndexAssignment
                        } else {
                            crate::mir::ArrayWriteProducerKind::IndexAssignment
                        },
                        target_val,
                        Some(index_val),
                        value_id,
                    )?;
                    return Ok(value_id);
                }
                let dst = if store_value.is_some() {
                    None
                } else {
                    Some(self.next_value_id())
                };
                let value_id = match store_value {
                    Some(value_id) => value_id,
                    None => dst.expect("dst must exist for load"),
                };
                self.emit_box_or_plugin_call(
                    dst,
                    target_val,
                    if store_value.is_some() {
                        "set".to_string()
                    } else {
                        "get".to_string()
                    },
                    None,
                    if store_value.is_some() {
                        vec![index_val, value_id]
                    } else {
                        vec![index_val]
                    },
                    if store_value.is_some() {
                        EffectMask::MUT
                    } else {
                        EffectMask::READ
                    },
                )?;
                Ok(value_id)
            }
            Some("MapBox") => {
                let dst = if store_value.is_some() {
                    None
                } else {
                    Some(self.next_value_id())
                };
                let value_id = match store_value {
                    Some(value_id) => value_id,
                    None => dst.expect("dst must exist for load"),
                };
                self.emit_box_or_plugin_call(
                    dst,
                    target_val,
                    if store_value.is_some() {
                        "set".to_string()
                    } else {
                        "get".to_string()
                    },
                    None,
                    if store_value.is_some() {
                        vec![index_val, value_id]
                    } else {
                        vec![index_val]
                    },
                    if store_value.is_some() {
                        EffectMask::MUT
                    } else {
                        EffectMask::READ
                    },
                )?;
                Ok(value_id)
            }
            _ if region.is_some() => {
                let slot = self.emit_fastmem_value_memop_with_access(
                    region.expect("region required"),
                    MemOpKind::TableIndex,
                    vec![target_val, index_val],
                    Some(crate::mir::instruction::MemOpAccess::table(
                        table_id.unwrap_or_else(|| format!("value_{}", target_val.0)),
                    )),
                )?;
                if let Some(value_id) = store_value {
                    self.emit_fastmem_memop(
                        region.expect("region required"),
                        crate::mir::instruction::MemOpKind::FieldStore,
                        None,
                        vec![slot, value_id],
                        None,
                    )?;
                    Ok(value_id)
                } else {
                    Ok(slot)
                }
            }
            _ => Err(format!(
                "index operator is only supported for Array/Map (found {})",
                Self::format_index_target_kind(class_hint.as_ref())
            )),
        }
    }

    /// Consume one already selected Index-read route. Static-data validation
    /// has completed before this terminal opens either child descent.
    pub(in crate::mir::builder) fn lower_prepared_raw_index_read_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        prepared: PreparedRawIndexReadV1,
    ) -> Result<ValueId, String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        match prepared.route {
            PreparedRawIndexReadRouteV1::Static {
                plan,
                result_type: prepared,
                index,
            } => {
                let index_val = drive_legacy_expression_v1(self, port, index)?;
                let dst = self.next_value_id();
                self.emit_instruction(MirInstruction::StaticDataLoad {
                    dst,
                    source_name: plan.source_name,
                    symbol: plan.symbol,
                    element: plan.element,
                    len: plan.values.len() as u32,
                    align: plan.align,
                    index: index_val,
                })?;
                prepared.commit(dst, &mut self.function_state.type_ctx);
                Ok(dst)
            }
            PreparedRawIndexReadRouteV1::Dynamic {
                target,
                index,
                target_label,
            } => {
                let target_val = drive_legacy_expression_v1(self, port, target)?;
                let index_val = drive_legacy_expression_v1(self, port, index)?;
                self.build_index_access_from_values(
                    None,
                    target_val,
                    index_val,
                    target_label,
                    "load",
                    None,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::MirBuilder;
    use super::MirType;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::recursive_child_lowering::{
        RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
    };
    use crate::mir::function::StaticDataPlan;
    use crate::mir::{MirInstruction, MirModule, ValueId};

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

    fn index(target: ASTNode, idx: ASTNode) -> ASTNode {
        ASTNode::Index {
            target: Box::new(target),
            index: Box::new(idx),
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

    fn u16_static_plan() -> StaticDataPlan {
        StaticDataPlan {
            source_name: "SIZE_CLASS".to_string(),
            symbol: ".hako.static.SIZE_CLASS".to_string(),
            element: "u16".to_string(),
            align: 2,
            linkage: "private".to_string(),
            unnamed_addr: true,
            values: vec![8, 16, 24, 32],
        }
    }

    fn install_static_plan(builder: &mut MirBuilder, plan: StaticDataPlan) {
        let mut module = MirModule::new("static-load-indexing-test".to_string());
        module.metadata.static_data_plans.push(plan);
        builder.current_module = Some(module);
    }

    fn has_static_load(builder: &MirBuilder) -> bool {
        builder
            .function_state
            .current_function
            .as_ref()
            .into_iter()
            .flat_map(|function| function.blocks.values())
            .flat_map(|block| block.instructions.iter())
            .any(|instruction| matches!(instruction, MirInstruction::StaticDataLoad { .. }))
    }

    fn lower_index_read_with_port<Port>(
        builder: &mut MirBuilder,
        port: &mut Port,
        target: ASTNode,
        index: ASTNode,
    ) -> Result<ValueId, String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        let prepared = super::PreparedRawIndexReadV1::prepare(builder, target, index)?;
        builder.lower_prepared_raw_index_read_with_port_v1(port, prepared)
    }

    fn lower_index_read(
        builder: &mut MirBuilder,
        target: ASTNode,
        index: ASTNode,
    ) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        lower_index_read_with_port(builder, &mut port, target, index)
    }

    struct RecordingIndexPortV1 {
        events: Vec<&'static str>,
        target_value: ValueId,
        index_value: ValueId,
        value_value: ValueId,
    }

    impl RecursiveChildLoweringPortV1 for RecordingIndexPortV1 {
        type BodyInput = ();
        type StatementInput = ();
        type ExpressionInput = ASTNode;

        fn lower_body(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::BodyInput,
        ) -> Result<ValueId, String> {
            Err("body descent is outside Index".to_owned())
        }

        fn lower_statement(
            &mut self,
            _builder: &mut MirBuilder,
            _input: Self::StatementInput,
        ) -> Result<ValueId, String> {
            Err("statement descent is outside Index".to_owned())
        }

        fn lower_expression(
            &mut self,
            _builder: &mut MirBuilder,
            input: Self::ExpressionInput,
        ) -> Result<ValueId, String> {
            match input {
                ASTNode::Variable { name, .. } if name == "target" => {
                    self.events.push("target");
                    Ok(self.target_value)
                }
                ASTNode::Variable { name, .. } if name == "index" => {
                    self.events.push("index");
                    Ok(self.index_value)
                }
                ASTNode::Variable { name, .. } if name == "value" => {
                    self.events.push("value");
                    Ok(self.value_value)
                }
                ASTNode::Variable { name, .. } if name == "SIZE_CLASS" => {
                    self.events.push("static-target");
                    Ok(self.target_value)
                }
                other => Err(format!("unexpected Index child: {other:?}")),
            }
        }
    }

    #[test]
    fn generic_index_lowers_target_then_index_once_through_the_same_port() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("generic_index_port_order/0".to_owned());
        let target_value = builder.alloc_value_for_test();
        let index_value = builder.alloc_value_for_test();
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(target_value, MirType::Box("ArrayBox".to_owned()));
        let mut port = RecordingIndexPortV1 {
            events: Vec::new(),
            target_value,
            index_value,
            value_value: builder.alloc_value_for_test(),
        };

        lower_index_read_with_port(&mut builder, &mut port, var("target"), var("index"))
            .expect("generic Index");

        assert_eq!(port.events, ["target", "index"]);
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("test function")
                .metadata
                .fastmem_index_access_sites
                .len(),
            1
        );
    }

    #[test]
    fn static_index_skips_target_and_lowers_index_once_through_the_port() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("static_index_port_order/0".to_owned());
        install_static_plan(&mut builder, u16_static_plan());
        let mut port = RecordingIndexPortV1 {
            events: Vec::new(),
            target_value: builder.alloc_value_for_test(),
            index_value: builder.alloc_value_for_test(),
            value_value: builder.alloc_value_for_test(),
        };

        lower_index_read_with_port(&mut builder, &mut port, var("SIZE_CLASS"), var("index"))
            .expect("static Index");

        assert_eq!(port.events, ["index"]);
        assert!(has_static_load(&builder));
    }

    #[test]
    fn index_assignment_prepares_label_and_lowers_children_once_in_order() {
        let prepared =
            super::PreparedRawIndexAssignmentV1::prepare(var("target"), var("index"), var("value"));
        assert_eq!(prepared.target_label.as_deref(), Some("target"));
        let non_variable =
            super::PreparedRawIndexAssignmentV1::prepare(int_lit(1), var("index"), var("value"));
        assert_eq!(non_variable.target_label, None);

        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("index_assignment_order/0".to_owned());
        let target_value = builder.alloc_value_for_test();
        let index_value = builder.alloc_value_for_test();
        let value_value = builder.alloc_value_for_test();
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(target_value, MirType::Box("ArrayBox".to_owned()));
        let mut port = RecordingIndexPortV1 {
            events: Vec::new(),
            target_value,
            index_value,
            value_value,
        };

        let result = super::lower_prepared_raw_index_assignment_with_port_v1(
            &mut builder,
            &mut port,
            prepared,
        )
        .expect("prepared Index assignment");

        assert_eq!(result, value_value);
        assert_eq!(port.events, ["target", "index", "value"]);
        let site = &builder
            .function_state
            .current_function
            .as_ref()
            .expect("test function")
            .metadata
            .fastmem_index_access_sites[0];
        assert_eq!(site.table_id.as_deref(), Some("target"));
        assert_eq!(site.access_kind, "store");
    }

    #[test]
    fn ordinary_index_access_records_site_metadata() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("ordinary_index_access/0".to_string());
        let page_table_id = builder.alloc_value_for_test();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("page_table".to_string(), page_table_id);
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(page_table_id, MirType::Box("ArrayBox".to_string()));

        let body = vec![
            local("key", int_lit(3)),
            local("loaded", index(var("page_table"), var("key"))),
            assign(index(var("page_table"), var("key")), int_lit(42)),
        ];

        super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
        let function = builder.function_state.current_function.as_ref().unwrap();

        assert_eq!(function.metadata.fastmem_index_access_sites.len(), 2);
        assert!(function
            .metadata
            .fastmem_index_access_sites
            .iter()
            .all(|site| site.region.is_none()));
        assert_eq!(
            function.metadata.fastmem_index_access_sites[0].required_route,
            "none"
        );
        assert_eq!(
            function.metadata.fastmem_index_access_sites[0].fallback_policy,
            "allow_dynamic"
        );
        assert_eq!(
            function.metadata.fastmem_index_access_sites[0].access_kind,
            "load"
        );
        assert_eq!(
            function.metadata.fastmem_index_access_sites[1].access_kind,
            "store"
        );
    }

    #[test]
    fn static_u16_load_publishes_transient_integer_before_finalization() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("static_u16_load_before_finalization/0".to_string());
        install_static_plan(&mut builder, u16_static_plan());

        let dst = lower_index_read(&mut builder, var("SIZE_CLASS"), int_lit(2))
            .expect("sealed u16 static load");

        assert_eq!(
            builder.function_state.type_ctx.value_types.get(&dst),
            Some(&MirType::Integer)
        );
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .and_then(|function| function.metadata.value_types.get(&dst)),
            None,
            "STATICLOAD0-I0: metadata is finalized only after the function session closes"
        );
        assert!(has_static_load(&builder));
        assert!(
            !builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .contains_key(&dst),
            "StaticDataLoad must not publish an origin fact"
        );
        let finalized = builder
            .finalize_function_draft(false)
            .expect("finalize static load test function");
        assert_eq!(
            finalized.metadata.value_types.get(&dst),
            Some(&MirType::Integer),
            "normal finalization must snapshot the transient StaticDataLoad fact"
        );
    }

    #[test]
    fn unsupported_static_element_rejects_before_index_or_load_allocation() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("static_load_unsupported_element/0".to_string());
        let mut plan = u16_static_plan();
        plan.element = "u8".to_string();
        install_static_plan(&mut builder, plan);
        let next_before = builder
            .function_state
            .current_function
            .as_ref()
            .expect("test function")
            .next_value_id;

        let error = lower_index_read(&mut builder, var("SIZE_CLASS"), int_lit(0))
            .expect_err("unsupported static element must reject");

        assert!(
            error.contains("[static-const/load-unsupported-element]"),
            "{error}"
        );
        assert_eq!(
            builder
                .function_state
                .current_function
                .as_ref()
                .expect("test function")
                .next_value_id,
            next_before
        );
        assert!(!has_static_load(&builder));
    }

    #[test]
    fn failed_static_load_emission_publishes_no_load_type_or_origin() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("static_load_emission_failure/0".to_string());
        install_static_plan(&mut builder, u16_static_plan());
        let index = builder.alloc_value_for_test();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("index".to_string(), index);
        let load_dst = builder
            .function_state
            .current_function
            .as_ref()
            .expect("test function")
            .next_value_id;
        builder.function_state.current_block = None;

        let error = lower_index_read(&mut builder, var("SIZE_CLASS"), var("index"))
            .expect_err("missing block must reject StaticDataLoad emission");

        assert_eq!(error, "No current basic block");
        let dst = super::ValueId::new(load_dst);
        assert!(
            !builder
                .function_state
                .type_ctx
                .value_types
                .contains_key(&dst),
            "failed load must not publish a transient type"
        );
        assert!(
            !builder
                .function_state
                .current_function
                .as_ref()
                .expect("test function")
                .metadata
                .value_types
                .contains_key(&dst),
            "failed load must not publish metadata"
        );
        assert!(
            !builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .contains_key(&dst),
            "failed load must not publish an origin fact"
        );
        assert!(!has_static_load(&builder));
    }
}
