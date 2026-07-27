use crate::mir::builder::callable_declaration_catalog::VerifiedSameModuleCallableDeclarationCatalogV1;
use crate::mir::builder::module_lifecycle::{
    InstanceMethodCapturePortV1, InstanceMethodCaptureRequestV1,
};
use crate::mir::MirBuilder;
use crate::parser::NyashParser;

#[derive(Default)]
struct RecordingOrdinaryPortV1 {
    methods: Vec<(String, String, usize)>,
}

impl InstanceMethodCapturePortV1 for RecordingOrdinaryPortV1 {
    fn lower_instance_method(
        &mut self,
        builder: &mut MirBuilder,
        request: InstanceMethodCaptureRequestV1,
    ) -> Result<(), String> {
        self.methods.push((
            request.owner.clone(),
            request.method.clone(),
            request.params.len(),
        ));
        builder.lower_method_as_function(
            request.function_name,
            request.owner,
            request.params,
            request.param_decls,
            request.return_type_name,
            request.body,
            request.uses,
            request.attrs,
        )
    }
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
        .lower_root_after_callable_catalog_install_with_instance_port_v1(
            root.clone(),
            &root,
            &mut port,
        )
        .expect("shared root kernel");
    let module = builder.finalize_module(result).expect("module");

    assert_eq!(port.methods, vec![("Worker".into(), "run".into(), 1)]);
    assert!(module.functions.contains_key("Worker.run/1"));
}
