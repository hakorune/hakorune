use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
use crate::mir::builder::program_root_lowering::ProgramDeferredStaticBoxLifecycleV1;
use crate::mir::builder::recursive_child_lowering::{
    RawBoxMethodChildPortV1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::{ConstValue, MirBuilder, MirInstruction, MirType, ValueId};
use crate::parser::NyashParser;

#[derive(Default)]
struct RecordingOrdinaryPortV1 {
    methods: Vec<(String, String, usize)>,
    static_methods: Vec<String>,
    static_context_active: Vec<bool>,
    fail_static_method: Option<String>,
    record_only_static: bool,
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
        _canonical_key: super::CanonicalSameModuleCallableKeyV1,
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
    let mut port = RecordingOrdinaryPortV1::default();
    let result = builder
        .lower_program_root_with_callable_port_v1(statements, &root, &mut port)
        .expect("shared root kernel");
    let module = builder.finalize_module(result).expect("module");

    assert_eq!(port.methods, vec![("Worker".into(), "run".into(), 1)]);
    assert!(module.functions.contains_key("Worker.run/1"));
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
