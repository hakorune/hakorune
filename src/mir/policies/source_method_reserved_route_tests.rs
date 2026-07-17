use crate::ast::{ASTNode, LiteralValue, Span};

use super::source_method_reserved_route::{
    classify_source_method_reserved_route_v1, MirDebugMethodV1, ReplIntrinsicMethodV1,
    SourceMethodReservedRouteContextV1 as Context, SourceMethodReservedRouteDecisionV1 as Decision,
    SourceMethodReservedRouteFailureV1 as Failure,
};

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn string(value: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(value.into()),
        span: Span::unknown(),
    }
}

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

#[test]
fn fastmem_is_context_bound_and_precedes_other_routes() {
    assert_eq!(
        classify_source_method_reserved_route_v1(
            Context::FastMemBody,
            &variable("mem"),
            "unknown",
            &[]
        ),
        Decision::FastMem
    );
    assert_eq!(
        classify_source_method_reserved_route_v1(Context::Ordinary, &variable("mem"), "addr", &[]),
        Decision::Ordinary
    );
}

#[test]
fn mir_debug_admission_returns_verified_payload() {
    assert_eq!(
        classify_source_method_reserved_route_v1(
            Context::Ordinary,
            &variable("__mir__"),
            "log",
            &[string("value"), integer(1)]
        ),
        Decision::MirDebug {
            method: MirDebugMethodV1::Log,
            label: "value".into(),
        }
    );
    assert_eq!(
        classify_source_method_reserved_route_v1(
            Context::Ordinary,
            &variable("__mir__"),
            "mark",
            &[string("point")]
        ),
        Decision::MirDebug {
            method: MirDebugMethodV1::Mark,
            label: "point".into(),
        }
    );
}

#[test]
fn mir_debug_preserves_fail_and_fallthrough_boundaries() {
    assert_eq!(
        classify_source_method_reserved_route_v1(
            Context::Ordinary,
            &variable("__mir__"),
            "log",
            &[]
        ),
        Decision::ReservedFail(Failure::MirDebugLabelRequired)
    );
    assert_eq!(
        classify_source_method_reserved_route_v1(
            Context::Ordinary,
            &variable("__mir__"),
            "log",
            &[integer(1)]
        ),
        Decision::Ordinary
    );
    assert_eq!(
        classify_source_method_reserved_route_v1(
            Context::Ordinary,
            &variable("__mir__"),
            "other",
            &[string("label")]
        ),
        Decision::Ordinary
    );
}

#[test]
fn repl_intrinsics_and_failure_are_exact() {
    for (spelling, expected) in [
        ("get", ReplIntrinsicMethodV1::Get),
        ("set", ReplIntrinsicMethodV1::Set),
    ] {
        assert_eq!(
            classify_source_method_reserved_route_v1(
                Context::Ordinary,
                &variable("__repl"),
                spelling,
                &[]
            ),
            Decision::ReplIntrinsic { method: expected }
        );
    }
    assert_eq!(
        classify_source_method_reserved_route_v1(
            Context::Ordinary,
            &variable("__repl"),
            "other",
            &[]
        ),
        Decision::ReservedFail(Failure::UnsupportedReplMethod)
    );
}

#[test]
fn non_variable_and_ordinary_receivers_fall_through() {
    assert_eq!(
        classify_source_method_reserved_route_v1(Context::FastMemBody, &integer(1), "addr", &[]),
        Decision::Ordinary
    );
    assert_eq!(
        classify_source_method_reserved_route_v1(
            Context::Ordinary,
            &variable("Helpers"),
            "run",
            &[]
        ),
        Decision::Ordinary
    );
}
