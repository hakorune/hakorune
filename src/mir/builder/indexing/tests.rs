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

    let result =
        super::lower_prepared_raw_index_assignment_with_port_v1(&mut builder, &mut port, prepared)
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
