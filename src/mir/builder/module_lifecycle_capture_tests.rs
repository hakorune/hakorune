use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, ParamDecl};
use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
use crate::mir::builder::recursive_child_lowering::{
    RawBoxMethodChildPortV1, RawLegacyChildLoweringPortV1, RecursiveChildLoweringPortV1,
};
use crate::mir::{
    BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirBuilder, MirFunction,
    MirInstruction, MirType, ValueId,
};
use crate::parser::NyashParser;

#[derive(Default)]
struct RecordingOrdinaryPortV1 {
    methods: Vec<(String, String, usize)>,
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
    let mut port = RecordingOrdinaryPortV1::default();
    let result = builder
        .lower_root_after_callable_catalog_install_with_callable_port_v1(
            root.clone(),
            &root,
            &mut port,
        )
        .expect("shared root kernel");
    let module = builder.finalize_module(result).expect("module");

    assert_eq!(port.methods, vec![("Worker".into(), "run".into(), 1)]);
    assert!(module.functions.contains_key("Worker.run/1"));
}

#[test]
fn ordinary_root_commits_one_complete_callable_batch() {
    let source = r#"
        function first() { return 1 }
        box Worker {
            birth(value) { return value }
            run(value) { return value }
        }
        static box Helpers {
            step(value) { return value }
        }
    "#;
    let root = NyashParser::parse_from_string(source).expect("collector source");
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    let result = builder
        .lower_root(root)
        .expect("collector-backed production root");

    let module = builder.current_module.as_ref().expect("unfinalized module");
    for symbol in [
        "first/0",
        "Worker.birth/1",
        "Worker.run/1",
        "Helpers.step/1",
    ] {
        assert!(module.functions.contains_key(symbol), "missing {symbol}");
    }

    builder.finalize_module(result).expect("final module");
}

#[test]
fn callable_batch_collision_publishes_no_collected_prefix() {
    let root = NyashParser::parse_from_string(
        r#"
            function first() { return 1 }
            function second() { return 2 }
        "#,
    )
    .expect("collision source");
    let mut builder = MirBuilder::new();
    builder.prepare_module().expect("module shell");
    let signature = FunctionSignature {
        name: "second/0".into(),
        params: Vec::new(),
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    builder
        .current_module
        .as_mut()
        .expect("module")
        .add_function(MirFunction::new(signature, BasicBlockId::new(99)));

    let error = builder.lower_root(root).unwrap_err();
    assert!(error.contains("[mir/callable-collector/atomic-commit]"));
    let module = builder.current_module.as_ref().expect("retained module");
    assert!(module.functions.contains_key("second/0"));
    assert!(!module.functions.contains_key("first/0"));
    assert_eq!(module.functions.len(), 1);
}
