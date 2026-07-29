use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl};
use crate::mir::builder::callable_declaration_catalog::{
    SameModuleCallableNamespaceV1, VerifiedSameModuleCallableDeclarationCatalogV1,
};
use crate::mir::builder::instance_box_constructor_batch::PreparedInstanceBoxConstructorBatchV1;
use crate::mir::builder::instance_box_declaration_lifecycle::PreparedInstanceBoxDeclarationLifecycleV1;
use crate::mir::builder::instance_box_method_batch::PreparedInstanceBoxMethodBatchV1;
use crate::mir::builder::main_expansion::VerifiedRawRootExpansionV1;
use crate::mir::builder::module_compat_policy::CallableMainCompatibilityPolicyV1;
use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
use crate::mir::builder::nonmain_static_box_method_batch::PreparedNonMainStaticBoxMethodBatchV1;
use crate::mir::builder::program_root_lowering::ProgramDeferredStaticBoxLifecycleV1;
use crate::mir::builder::recursive_child_lowering::{
    RawBoxMethodChildPortV1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::{ConstValue, MirBuilder, MirInstruction, MirType, ValueId};
use crate::parser::NyashParser;

#[derive(Default)]
struct RecordingOrdinaryPortV1 {
    methods: Vec<(String, String, usize)>,
    instance_keys: Vec<(SameModuleCallableNamespaceV1, String)>,
    static_methods: Vec<String>,
    static_context_active: Vec<bool>,
    instance_methods: Vec<String>,
    fail_static_method: Option<String>,
    fail_instance_method: Option<String>,
    record_only_static: bool,
    record_only_instance: bool,
    body_calls: usize,
}

impl RawBoxMethodChildPortV1 for RecordingOrdinaryPortV1 {
    fn lower_static_main_box(
        &mut self,
        builder: &mut MirBuilder,
        box_name: String,
        methods: std::collections::HashMap<String, ASTNode>,
    ) -> Result<ValueId, String> {
        RawLegacyChildLoweringPortV1.lower_static_main_box(builder, box_name, methods)
    }

    fn lower_static_box_method(
        &mut self,
        builder: &mut MirBuilder,
        function_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.static_methods.push(function_name.clone());
        self.static_context_active
            .push(builder.comp_ctx.compilation_context.is_some());
        if self.fail_static_method.as_deref() == Some(function_name.as_str()) {
            return Err(format!("selected static method failure: {function_name}"));
        }
        if self.record_only_static {
            return Ok(());
        }
        RawLegacyChildLoweringPortV1.lower_static_box_method(
            builder,
            function_name,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }

    fn lower_instance_box_method(
        &mut self,
        builder: &mut MirBuilder,
        function_name: String,
        owner: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.instance_methods.push(function_name.clone());
        if self.fail_instance_method.as_deref() == Some(function_name.as_str()) {
            return Err(format!("selected instance method failure: {function_name}"));
        }
        if self.record_only_instance {
            return Ok(());
        }
        RawLegacyChildLoweringPortV1.lower_instance_box_method(
            builder,
            function_name,
            owner,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }
}

impl RecursiveChildLoweringPortV1 for RecordingOrdinaryPortV1 {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        builder: &mut MirBuilder,
        body: Self::BodyInput,
    ) -> Result<ValueId, String> {
        self.body_calls += 1;
        RawLegacyChildLoweringPortV1.lower_body(builder, body)
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        statement: Self::StatementInput,
    ) -> Result<ValueId, String> {
        RawLegacyChildLoweringPortV1.lower_statement(builder, statement)
    }

    fn lower_expression(
        &mut self,
        builder: &mut MirBuilder,
        expression: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        RawLegacyChildLoweringPortV1.lower_expression(builder, expression)
    }
}

impl RootCallableCapturePortV1 for RecordingOrdinaryPortV1 {
    fn lower_root_instance_method(
        &mut self,
        builder: &mut MirBuilder,
        canonical_key: super::CanonicalSameModuleCallableKeyV1,
        owner: String,
        method: String,
        function_name: String,
        params: Vec<String>,
        param_decls: Vec<ParamDecl>,
        return_type_name: Option<String>,
        body: Vec<ASTNode>,
        uses: Vec<String>,
        attrs: DeclarationAttrs,
    ) -> Result<(), String> {
        self.instance_keys.push((
            canonical_key.namespace(),
            canonical_key.mir_symbol_projection(),
        ));
        self.methods.push((owner.clone(), method, params.len()));
        self.lower_instance_box_method(
            builder,
            function_name,
            owner,
            params,
            param_decls,
            return_type_name,
            body,
            uses,
            attrs,
        )
    }
}

fn parsed_static_box(source: &str) -> (String, std::collections::HashMap<String, ASTNode>) {
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string(source).expect("deferred static Box source")
    else {
        panic!("parser must return Program");
    };
    let ASTNode::BoxDeclaration { name, methods, .. } = statements.remove(0) else {
        panic!("fixture must contain one static Box");
    };
    (name, methods)
}

fn parsed_instance_box(source: &str) -> (String, std::collections::HashMap<String, ASTNode>) {
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string(source).expect("instance Box source")
    else {
        panic!("parser must return Program");
    };
    let ASTNode::BoxDeclaration {
        name, constructors, ..
    } = statements.remove(0)
    else {
        panic!("fixture must contain one instance Box");
    };
    (name, constructors)
}

fn parsed_instance_methods(source: &str) -> (String, std::collections::HashMap<String, ASTNode>) {
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string(source).expect("instance method source")
    else {
        panic!("parser must return Program");
    };
    let ASTNode::BoxDeclaration { name, methods, .. } = statements.remove(0) else {
        panic!("fixture must contain one instance Box");
    };
    (name, methods)
}

fn parsed_instance_declaration(source: &str) -> ASTNode {
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string(source).expect("instance declaration source")
    else {
        panic!("parser must return Program");
    };
    statements.remove(0)
}

#[test]
fn mirbuilder_minimal_literal_integer_path_smoke() {
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    let literal = builder
        .build_literal(LiteralValue::Integer(0))
        .expect("literal integer");
    let module = builder.finalize_module(literal).expect("final module");
    let main = module
        .get_function("main")
        .expect("minimal literal path should create main");

    assert_eq!(main.signature.return_type, MirType::Integer);
    assert!(module.get_function("condition_fn").is_some());
    assert!(main.blocks.values().any(|block| {
        block.instructions.iter().any(|instruction| {
            matches!(
                instruction,
                MirInstruction::Const {
                    dst,
                    value: ConstValue::Integer(0),
                } if *dst == literal
            )
        }) && matches!(
            &block.terminator,
            Some(MirInstruction::Return { value: Some(value) }) if *value == literal
        )
    }));
}

#[test]
fn shared_root_kernel_lends_each_instance_method_to_one_stack_port() {
    let source = "box Worker { run(value) { return value } }";
    let root = NyashParser::parse_from_string(source).expect("capture seam source");
    let catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&root).expect("catalog");
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .expect("catalog install");
    let ASTNode::Program { statements, .. } = root.clone() else {
        panic!("capture seam must parse a Program");
    };
    let expansion =
        VerifiedRawRootExpansionV1::from_program(&root).expect("verified Script expansion");
    let mut port = RecordingOrdinaryPortV1::default();
    let result = builder
        .lower_program_root_with_callable_port_v1(statements, &root, &expansion, &mut port)
        .expect("shared root kernel");
    let module = builder.finalize_module(result).expect("module");

    assert_eq!(port.methods, vec![("Worker".into(), "run".into(), 1)]);
    assert!(module.functions.contains_key("Worker.run/1"));
}

#[test]
fn instance_method_batch_preserves_route_specific_admission_and_order() {
    let source = "box Page { omega(value) { return value } alpha() { return 1 } }";
    let root = NyashParser::parse_from_string(source).expect("catalog source");
    let catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&root).expect("catalog");
    let (owner, mut methods) = parsed_instance_methods(source);
    let (_, mut static_methods) = parsed_static_box("static box Static { helper() { return 0 } }");
    methods.insert(
        "static_helper".to_owned(),
        static_methods.remove("helper").expect("static helper"),
    );
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string("42").expect("non-function method fixture")
    else {
        panic!("parser must return Program");
    };
    methods.insert("ignored".to_owned(), statements.remove(0));

    let mut raw_builder = MirBuilder::new();
    let mut raw_port = RecordingOrdinaryPortV1 {
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };
    PreparedInstanceBoxMethodBatchV1::prepare(&owner, &methods)
        .lower_raw_with_port_v1(&mut raw_builder, &mut raw_port)
        .expect("raw method batch needs no catalog");

    let mut root_builder = MirBuilder::new();
    root_builder.prepare_module().expect("root module shell");
    root_builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .expect("catalog install");
    let mut root_port = RecordingOrdinaryPortV1 {
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };
    PreparedInstanceBoxMethodBatchV1::prepare(&owner, &methods)
        .lower_root_with_port_v1(&mut root_builder, &mut root_port)
        .expect("root method batch uses exact catalog");

    let expected = vec!["Page.alpha/0", "Page.omega/1"];
    assert_eq!(raw_port.instance_methods, expected);
    assert_eq!(root_port.instance_methods, expected);
    assert_eq!(
        root_port.methods,
        vec![
            ("Page".into(), "alpha".into(), 0),
            ("Page".into(), "omega".into(), 1),
        ]
    );
    assert_eq!(
        root_port.instance_keys,
        expected
            .into_iter()
            .map(|symbol| (
                SameModuleCallableNamespaceV1::InstanceBoxMethod,
                symbol.into()
            ))
            .collect::<Vec<_>>()
    );
}

#[test]
fn instance_method_batch_preserves_prefix_on_route_failure() {
    let source = "box Page { gamma() { return 3 } beta() { return 2 } alpha() { return 1 } }";
    let (owner, methods) = parsed_instance_methods(source);
    let prefix_root = NyashParser::parse_from_string("box Page { alpha() { return 1 } }")
        .expect("prefix catalog source");
    let prefix_catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&prefix_root).expect("catalog");
    let mut root_builder = MirBuilder::new();
    root_builder.prepare_module().expect("root module shell");
    root_builder
        .comp_ctx
        .install_callable_declaration_catalog(prefix_catalog)
        .expect("catalog install");
    let mut root_port = RecordingOrdinaryPortV1 {
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };
    let error = PreparedInstanceBoxMethodBatchV1::prepare(&owner, &methods)
        .lower_root_with_port_v1(&mut root_builder, &mut root_port)
        .expect_err("missing beta catalog row must stop the batch");
    assert!(error.contains("missing exact declaration for Page.beta/0"));
    assert_eq!(root_port.instance_methods, vec!["Page.alpha/0"]);

    let mut raw_builder = MirBuilder::new();
    let mut raw_port = RecordingOrdinaryPortV1 {
        fail_instance_method: Some("Page.beta/0".to_owned()),
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };
    let error = PreparedInstanceBoxMethodBatchV1::prepare(&owner, &methods)
        .lower_raw_with_port_v1(&mut raw_builder, &mut raw_port)
        .expect_err("raw beta failure must stop the batch");
    assert_eq!(error, "selected instance method failure: Page.beta/0");
    assert_eq!(
        raw_port.instance_methods,
        vec!["Page.alpha/0", "Page.beta/0"]
    );
}

#[test]
fn instance_box_declaration_lifecycle_preserves_prefix_and_route_terminals() {
    let source = "box Page {
        value: IntegerBox
        weak parent
        birth() { return 0 }
        omega() { return 2 }
        alpha() { return 1 }
    }";
    let root = NyashParser::parse_from_string(source).expect("catalog source");
    let catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&root).expect("catalog");

    let mut raw_builder = MirBuilder::new();
    raw_builder.prepare_module().expect("raw module shell");
    let mut raw_port = RecordingOrdinaryPortV1 {
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };
    let ASTNode::BoxDeclaration {
        name,
        methods,
        fields,
        field_decls,
        constructors,
        init_fields,
        weak_fields,
        ..
    } = parsed_instance_declaration(source)
    else {
        panic!("fixture must contain one Box");
    };
    PreparedInstanceBoxDeclarationLifecycleV1::prepare(
        &name,
        &methods,
        &fields,
        &field_decls,
        &constructors,
        &init_fields,
        &weak_fields,
    )
    .lower_raw_with_port_v1(&mut raw_builder, &mut raw_port)
    .expect("raw declaration lifecycle");

    let mut root_builder = MirBuilder::new();
    root_builder.prepare_module().expect("root module shell");
    root_builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .expect("catalog install");
    let mut root_port = RecordingOrdinaryPortV1 {
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };
    PreparedInstanceBoxDeclarationLifecycleV1::prepare(
        &name,
        &methods,
        &fields,
        &field_decls,
        &constructors,
        &init_fields,
        &weak_fields,
    )
    .lower_root_with_port_v1(&mut root_builder, &mut root_port)
    .expect("root declaration lifecycle");

    let expected = vec!["Page.birth/0", "Page.alpha/0", "Page.omega/0"];
    assert_eq!(raw_port.instance_methods, expected);
    assert_eq!(root_port.instance_methods, expected);
    assert!(raw_port.instance_keys.is_empty());
    assert_eq!(
        root_port.instance_keys,
        vec![
            (
                SameModuleCallableNamespaceV1::InstanceBoxMethod,
                "Page.alpha/0".into()
            ),
            (
                SameModuleCallableNamespaceV1::InstanceBoxMethod,
                "Page.omega/0".into()
            ),
        ]
    );
    for builder in [&raw_builder, &root_builder] {
        assert_eq!(
            builder.comp_ctx.user_defined_boxes.get("Page"),
            Some(&vec!["value".to_owned(), "parent".to_owned()])
        );
        assert!(builder
            .comp_ctx
            .weak_fields_by_box
            .get("Page")
            .is_some_and(|fields| fields.contains("parent")));
    }
}

#[test]
fn instance_box_declaration_lifecycle_stops_after_exact_dirty_prefix() {
    let source = "box Page {
        value: IntegerBox
        birth() { return 0 }
        alpha() { return 1 }
    }";
    let ASTNode::BoxDeclaration {
        name,
        methods,
        fields,
        field_decls,
        constructors,
        init_fields,
        weak_fields,
        ..
    } = parsed_instance_declaration(source)
    else {
        panic!("fixture must contain one Box");
    };
    let mut metadata_failure = MirBuilder::new();
    let mut untouched_port = RecordingOrdinaryPortV1 {
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };
    PreparedInstanceBoxDeclarationLifecycleV1::prepare(
        &name,
        &methods,
        &fields,
        &field_decls,
        &constructors,
        &init_fields,
        &weak_fields,
    )
    .lower_raw_with_port_v1(&mut metadata_failure, &mut untouched_port)
    .expect_err("metadata emission without module shell must fail");
    assert!(metadata_failure
        .comp_ctx
        .user_defined_boxes
        .contains_key("Page"));
    assert!(untouched_port.instance_methods.is_empty());

    let mut constructor_failure = MirBuilder::new();
    constructor_failure
        .prepare_module()
        .expect("constructor module shell");
    let mut prefix_port = RecordingOrdinaryPortV1 {
        fail_instance_method: Some("Page.birth/0".to_owned()),
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };
    let error = PreparedInstanceBoxDeclarationLifecycleV1::prepare(
        &name,
        &methods,
        &fields,
        &field_decls,
        &constructors,
        &init_fields,
        &weak_fields,
    )
    .lower_raw_with_port_v1(&mut constructor_failure, &mut prefix_port)
    .expect_err("constructor failure must stop methods");
    assert_eq!(error, "selected instance method failure: Page.birth/0");
    assert_eq!(prefix_port.instance_methods, vec!["Page.birth/0"]);
    assert!(constructor_failure
        .comp_ctx
        .user_defined_boxes
        .contains_key("Page"));
}

#[test]
fn verified_main_expansion_lowers_helpers_in_order_before_body() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let root = NyashParser::parse_from_string(
        "static box Main { zeta(value) { return value } alpha() { return 1 } main() { return 0 } }",
    )
    .expect("verified Main source");
    let catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&root).expect("catalog");
    let expansion =
        VerifiedRawRootExpansionV1::from_program(&root).expect("verified App expansion");
    let ASTNode::Program { statements, .. } = root.clone() else {
        panic!("verified Main source must parse a Program");
    };
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .expect("catalog install");
    let mut port = RecordingOrdinaryPortV1 {
        record_only_static: true,
        ..RecordingOrdinaryPortV1::default()
    };

    builder
        .lower_program_root_with_callable_port_v1(statements, &root, &expansion, &mut port)
        .expect("verified Main lowering");

    assert_eq!(port.static_methods, vec!["Main.alpha/0", "Main.zeta/1"]);
    assert_eq!(port.body_calls, 1);
}

#[test]
fn verified_and_compatibility_main_share_required_callable_order() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let source =
        "static box Main { zeta(value) { return value } alpha() { return 1 } main() { return 0 } }";
    let root = NyashParser::parse_from_string(source).expect("verified Main source");
    let catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&root).expect("catalog");
    let expansion =
        VerifiedRawRootExpansionV1::from_program(&root).expect("verified App expansion");
    let ASTNode::Program { statements, .. } = root.clone() else {
        panic!("verified Main source must parse a Program");
    };
    let mut selected_builder = MirBuilder::new();
    selected_builder.prepare_module().expect("module shell");
    selected_builder.comp_ctx.callable_main_compatibility_policy =
        CallableMainCompatibilityPolicyV1::Required;
    selected_builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .expect("catalog install");
    let mut selected_port = RecordingOrdinaryPortV1 {
        record_only_static: true,
        ..RecordingOrdinaryPortV1::default()
    };
    selected_builder
        .lower_program_root_with_callable_port_v1(statements, &root, &expansion, &mut selected_port)
        .expect("verified Main lowering");

    let (box_name, methods) = parsed_static_box(source);
    let mut compatibility_builder = MirBuilder::new();
    compatibility_builder
        .prepare_module()
        .expect("compatibility module shell");
    compatibility_builder
        .comp_ctx
        .callable_main_compatibility_policy = CallableMainCompatibilityPolicyV1::Required;
    let mut compatibility_port = RecordingOrdinaryPortV1 {
        record_only_static: true,
        ..RecordingOrdinaryPortV1::default()
    };
    compatibility_builder
        .build_static_main_box_with_port_v1(&mut compatibility_port, box_name, methods)
        .expect("explicit compatibility Main lowering");

    let expected = vec!["Main.alpha/0", "Main.zeta/1", "Main.main/0"];
    assert_eq!(selected_port.static_methods, expected);
    assert_eq!(compatibility_port.static_methods, expected);
    assert_eq!(selected_port.body_calls, 1);
    assert_eq!(compatibility_port.body_calls, 1);
}

#[test]
fn verified_main_helper_failure_stops_later_helpers_and_body() {
    let root = NyashParser::parse_from_string(
        "static box Main { zeta() { return 2 } alpha() { return 1 } main() { return 0 } }",
    )
    .expect("verified Main failure source");
    let catalog =
        VerifiedSameModuleCallableDeclarationCatalogV1::seal_root(&root).expect("catalog");
    let expansion =
        VerifiedRawRootExpansionV1::from_program(&root).expect("verified App expansion");
    let ASTNode::Program { statements, .. } = root.clone() else {
        panic!("verified Main source must parse a Program");
    };
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    builder
        .comp_ctx
        .install_callable_declaration_catalog(catalog)
        .expect("catalog install");
    let mut port = RecordingOrdinaryPortV1 {
        fail_static_method: Some("Main.alpha/0".to_owned()),
        record_only_static: true,
        ..RecordingOrdinaryPortV1::default()
    };

    let error = builder
        .lower_program_root_with_callable_port_v1(statements, &root, &expansion, &mut port)
        .expect_err("first verified helper must fail");

    assert_eq!(
        error,
        "[callable-main/lowering] selected static method failure: Main.alpha/0"
    );
    assert_eq!(port.static_methods, vec!["Main.alpha/0"]);
    assert_eq!(port.body_calls, 0);
}

#[test]
fn deferred_static_box_lifecycle_lowers_sorted_methods_and_clears_on_success() {
    let (name, methods) =
        parsed_static_box("static box Helpers { beta() { return 2 } alpha() { return 1 } }");
    let mut builder = MirBuilder::new();
    let mut port = RecordingOrdinaryPortV1 {
        record_only_static: true,
        ..RecordingOrdinaryPortV1::default()
    };

    ProgramDeferredStaticBoxLifecycleV1::new(name, methods)
        .lower_with_port_v1(&mut builder, &mut port)
        .expect("deferred static Box lifecycle");

    assert_eq!(
        port.static_methods,
        vec!["Helpers.alpha/0", "Helpers.beta/0"]
    );
    assert_eq!(port.static_context_active, vec![true, true]);
    assert!(builder.comp_ctx.compilation_context.is_none());
}

#[test]
fn nonmain_static_method_batch_sorts_projects_and_keeps_ordinary_main() {
    let (name, mut methods) = parsed_static_box(
        "static box Helpers { omega() { return 3 } main() { return 2 } alpha(value) { return value } }",
    );
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string("42").expect("non-function method-map fixture")
    else {
        panic!("parser must return Program");
    };
    methods.insert("ignored".to_owned(), statements.remove(0));
    let mut builder = MirBuilder::new();
    let mut port = RecordingOrdinaryPortV1 {
        record_only_static: true,
        ..RecordingOrdinaryPortV1::default()
    };

    PreparedNonMainStaticBoxMethodBatchV1::prepare(name, methods)
        .lower_with_port_v1(&mut builder, &mut port)
        .expect("prepared static method batch");

    assert_eq!(
        port.static_methods,
        vec!["Helpers.alpha/1", "Helpers.main/0", "Helpers.omega/0"]
    );
}

#[test]
fn instance_constructor_batch_sorts_projects_and_skips_non_function_rows() {
    let (name, mut constructors) = parsed_instance_box(
        "box Page { birth(value, extra) { return value } birth() { return 0 } }",
    );
    let ASTNode::Program { mut statements, .. } =
        NyashParser::parse_from_string("42").expect("non-function constructor-map fixture")
    else {
        panic!("parser must return Program");
    };
    constructors.insert("ignored/0".to_owned(), statements.remove(0));
    let mut builder = MirBuilder::new();
    let mut port = RecordingOrdinaryPortV1 {
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };

    PreparedInstanceBoxConstructorBatchV1::prepare(&name, &constructors)
        .lower_with_port_v1(&mut builder, &mut port)
        .expect("prepared constructor batch");

    assert_eq!(port.instance_methods, vec!["Page.birth/0", "Page.birth/2"]);
}

#[test]
fn instance_constructor_batch_stops_after_first_failure() {
    let (name, constructors) = parsed_instance_box(
        "box Page { birth(value, extra) { return value } birth(value) { return value } birth() { return 0 } }",
    );
    let mut builder = MirBuilder::new();
    let mut port = RecordingOrdinaryPortV1 {
        fail_instance_method: Some("Page.birth/1".to_owned()),
        record_only_instance: true,
        ..RecordingOrdinaryPortV1::default()
    };

    let error = PreparedInstanceBoxConstructorBatchV1::prepare(&name, &constructors)
        .lower_with_port_v1(&mut builder, &mut port)
        .expect_err("selected constructor must fail");

    assert_eq!(error, "selected instance method failure: Page.birth/1");
    assert_eq!(port.instance_methods, vec!["Page.birth/0", "Page.birth/1"]);
}

#[test]
fn deferred_static_box_lifecycle_keeps_dirty_candidate_and_stops_after_failure() {
    let (name, methods) = parsed_static_box(
        "static box Broken { gamma() { return 3 } beta() { return 2 } alpha() { return 1 } }",
    );
    let mut builder = MirBuilder::new();
    let mut port = RecordingOrdinaryPortV1 {
        fail_static_method: Some("Broken.beta/0".to_owned()),
        record_only_static: true,
        ..RecordingOrdinaryPortV1::default()
    };

    let error = ProgramDeferredStaticBoxLifecycleV1::new(name, methods)
        .lower_with_port_v1(&mut builder, &mut port)
        .expect_err("selected static method must fail");

    assert_eq!(error, "selected static method failure: Broken.beta/0");
    assert_eq!(port.static_methods, vec!["Broken.alpha/0", "Broken.beta/0"]);
    assert_eq!(port.static_context_active, vec![true, true]);
    assert!(builder.comp_ctx.compilation_context.is_some());
}
