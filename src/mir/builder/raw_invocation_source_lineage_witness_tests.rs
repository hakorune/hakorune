use super::*;
use crate::ast::{ASTNode, LiteralValue, Span};

fn static_method_call() -> ASTNode {
    ASTNode::MethodCall {
        object: Box::new(ASTNode::Variable {
            name: "Api".to_owned(),
            span: Span::unknown(),
        }),
        method: "run".to_owned(),
        arguments: vec![ASTNode::Literal {
            value: LiteralValue::Integer(1),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    }
}

#[test]
fn unlocated_source_loss_retains_the_root_lineage_witness() {
    let root = RawInvocationRootLineageV1::Cataloged(
        crate::mir::builder::CanonicalSameModuleCallableKeyV1::test_static_box_method(
            "Api", "run", 1,
        ),
    );
    let (_, context) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::root(Vec::<ASTNode>::new(), root.clone()),
    );
    let (_, child) = RawInvocationSourceContextV1::from_transport(
        context.body_statement(static_method_call(), 2),
    );

    let RawInvocationSourceContextV1::UnlocatedCompatibility {
        reason: RawUnlocatedPortalV1::CallObject,
        expected_lineage: Some(actual_root),
    } = &child
    else {
        panic!("source loss must retain a lineage witness");
    };
    assert_eq!(actual_root, &root);

    let argument = child.child_call_argument(0);
    assert!(matches!(
        argument,
        RawInvocationSourceContextV1::UnlocatedCompatibility {
            expected_lineage: Some(_), ..
        }
    ));
}

#[test]
fn compatibility_unlocated_context_has_no_lineage_witness() {
    let (_, context) = RawInvocationSourceContextV1::from_transport(
        RawInvocationSourceTransportV1::unlocated(
            static_method_call(),
            RawUnlocatedPortalV1::CallObject,
        ),
    );

    assert!(matches!(
        context,
        RawInvocationSourceContextV1::UnlocatedCompatibility {
            reason: RawUnlocatedPortalV1::CallObject,
            expected_lineage: None,
        }
    ));
}
