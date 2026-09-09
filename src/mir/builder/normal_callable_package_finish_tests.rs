use super::*;
use crate::mir::normal_callable_semantic_package::issue_normal_callable_semantic_package_v1;
use crate::mir::resolved_semantics::FunctionSemanticResolverSessionV1;

#[test]
fn unopened_dropped_and_failed_ports_cannot_finish_scope() {
    for mode in 0..3 {
        let mut resolver = FunctionSemanticResolverSessionV1::new(1079).unwrap();
        let package =
            issue_normal_callable_semantic_package_v1(&mut resolver, super::tests::source())
                .unwrap();
        let mut context = CompilationContext::new();
        let mut scope = package
            .with_normal_callable_install_once(&mut context, BuilderInstallConsumerV1::new())
            .unwrap()
            .into_lowering_scope();
        if mode != 0 {
            let port = scope.open_lowering_once(&context).unwrap();
            if mode == 1 {
                drop(port);
            } else {
                assert!(port.complete().is_err());
            }
        }
        assert!(matches!(
            scope.finish(),
            Err(NormalCallableSemanticPackageInstallIssueV1::LoweringNotCompleted)
        ));
    }
}
