use super::raw_compatibility_child_terminal::validate_raw_compat_source_context_v1;
use super::raw_invocation_source_transport::{
    RawInvocationRootLineageV1, RawInvocationSourceContextV1, RawInvocationSourceTransportV1,
};
use super::{
    BuilderInvocationConfigV1, CallableMainMaterializationPolicyV1, MirBuilder,
    ModuleBuilderInvocationSessionV1, NormalRuntimeInputSnapshotV1,
    PreparedNormalDefaultProgramRootV1,
};

use crate::mir::resolved_semantics::SourcePathV1;
use crate::parser::NyashParser;

fn session() -> ModuleBuilderInvocationSessionV1 {
    let current = MirBuilder::new();
    let config = BuilderInvocationConfigV1::snapshot_for_raw(&current, None);
    ModuleBuilderInvocationSessionV1::open(&current, config)
}

fn lower_raw_compat(source: &str, policy: CallableMainMaterializationPolicyV1) -> super::MirModule {
    let source = NyashParser::parse_from_string(source).expect("raw compatibility source");
    let source = PreparedNormalDefaultProgramRootV1::seal(source).expect("Program source");
    let completed = session()
        .complete_normal_default_program_root_catalog_lifecycle(
            source,
            policy,
            NormalRuntimeInputSnapshotV1::empty(),
        )
        .expect("raw compatibility terminal");
    let (_, module, _) = completed.into_parts();
    module
}

fn has_function(module: &super::MirModule, name: &str) -> bool {
    module
        .functions
        .iter()
        .any(|(_, function)| function.signature.name == name)
}

#[test]
fn raw_compatibility_root_families_use_the_collector_terminal() {
    let _ = crate::runtime::ring0::ensure_global_ring0_initialized();
    let script_top_level = lower_raw_compat(
        "function helper(value) { return value }",
        CallableMainMaterializationPolicyV1::Omitted,
    );
    assert!(has_function(&script_top_level, "helper/1"));

    let instance_constructor = lower_raw_compat(
        "box Worker { birth(value) { return value } }\nstatic box Main { main() { return 0 } }",
        CallableMainMaterializationPolicyV1::Omitted,
    );
    assert!(has_function(&instance_constructor, "Worker.birth/1"));

    let deferred_static = lower_raw_compat(
        "static box Helpers { run(value) { return value } }\nstatic box Main { main() { return 0 } }",
        CallableMainMaterializationPolicyV1::Omitted,
    );
    assert!(has_function(&deferred_static, "Helpers.run/1"));

    let app_main = lower_raw_compat(
        "static box Main { helper() { return 1 } main() { return 0 } }",
        CallableMainMaterializationPolicyV1::Required,
    );
    assert!(has_function(&app_main, "Main.helper/0"));
    assert!(has_function(&app_main, "Main.main/0"));
}

#[test]
fn raw_compatibility_source_context_is_script_root_only() {
    let (_, script_context) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::script_root(()),
    );
    assert!(validate_raw_compat_source_context_v1(Some(&script_context)).is_ok());

    let (_, unlocated_context) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::unlocated(
            (),
            super::raw_invocation_source_transport::RawUnlocatedPortalV1::CallObject,
        ));
    assert!(validate_raw_compat_source_context_v1(Some(&unlocated_context)).is_err());

    let (_, foreign_context) =
        RawInvocationSourceContextV1::from_transport(RawInvocationSourceTransportV1::root(
            (),
            RawInvocationRootLineageV1::nested_box_method(
                SourcePathV1::function_body().node(),
                "Worker.run/0".to_owned(),
            ),
        ));
    assert!(validate_raw_compat_source_context_v1(Some(&foreign_context)).is_err());
    assert!(validate_raw_compat_source_context_v1(None).is_err());
}
