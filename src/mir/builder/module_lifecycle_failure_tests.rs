use super::{parsed_static_box, RecordingOrdinaryPortV1};
use crate::mir::builder::program_root_lowering::ProgramDeferredStaticBoxLifecycleV1;
use crate::mir::MirBuilder;

#[test]
fn deferred_static_box_lifecycle_restores_context_and_stops_after_failure() {
    let mut builder = MirBuilder::new();
    let mut port = RecordingOrdinaryPortV1 {
        fail_static_method: Some("Broken.beta/0".to_owned()),
        record_only_static: true,
        ..RecordingOrdinaryPortV1::default()
    };

    let (name, methods) = parsed_static_box(
        "static box Broken { gamma() { return 3 } beta() { return 2 } alpha() { return 1 } }",
    );
    let error = ProgramDeferredStaticBoxLifecycleV1::new(name, methods)
        .lower_with_port_v1(&mut builder, &mut port)
        .expect_err("selected static method must fail");

    assert_eq!(error, "selected static method failure: Broken.beta/0");
    assert_eq!(port.static_methods, vec!["Broken.alpha/0", "Broken.beta/0"]);
    assert_eq!(port.static_context_active, vec![true, true]);
    assert!(builder.comp_ctx.compilation_context.is_none());
}
