use super::super::{
    CallableResultCatalogErrorV1, CallableResultUnavailableReasonV1,
    VerifiedCallableResultDispositionV1,
};
use super::support::{disposition, key, normalized, seal};

fn exact(requirements: &[u32]) -> VerifiedCallableResultDispositionV1 {
    VerifiedCallableResultDispositionV1::ExactI64 {
        required_i64_arguments: requirements.into(),
    }
}

fn exact_box(name: &str) -> VerifiedCallableResultDispositionV1 {
    VerifiedCallableResultDispositionV1::ExactNominalBox {
        box_name: name.to_owned(),
    }
}

#[test]
fn literals_parameters_locals_arithmetic_and_declared_seed_are_exact() {
    let source = r#"
        static box ScalarProofV1 {
            literal() { return 1 }
            parameter(value) { return value }
            arithmetic(value) {
                local copy = value
                copy = copy + 1
                return copy * 2
            }
            branch(value) {
                if value == 0 { return 1 } else { return value }
            }
            declared(): i64 { return "body is not the ABI authority" }
        }
    "#;

    assert_eq!(
        disposition(source, "ScalarProofV1", "literal", 0),
        exact(&[])
    );
    assert_eq!(
        disposition(source, "ScalarProofV1", "parameter", 1),
        exact(&[0])
    );
    assert_eq!(
        disposition(source, "ScalarProofV1", "arithmetic", 1),
        exact(&[0])
    );
    assert_eq!(
        disposition(source, "ScalarProofV1", "branch", 1),
        exact(&[0])
    );
    assert_eq!(
        disposition(source, "ScalarProofV1", "declared", 0),
        exact(&[])
    );
}

#[test]
fn constructed_nominal_box_result_is_exact_without_name_or_mir_inference() {
    let source = r#"
        box ProductV1 { birth() {} }
        static box ProductFactoryV1 {
            make() { return new ProductV1() }
            local_copy() {
                local product = new ProductV1()
                return product
            }
            branch(flag) {
                if flag == 0 { return new ProductV1() }
                return new ProductV1()
            }
        }
    "#;

    assert_eq!(
        disposition(source, "ProductFactoryV1", "make", 0),
        exact_box("ProductV1")
    );
    assert_eq!(
        disposition(source, "ProductFactoryV1", "local_copy", 0),
        exact_box("ProductV1")
    );
    assert_eq!(
        disposition(source, "ProductFactoryV1", "branch", 1),
        exact_box("ProductV1")
    );
}

#[test]
fn mixed_nominal_box_results_fail_fast_instead_of_selecting_a_type() {
    let source = r#"
        box LeftV1 { birth() {} }
        box RightV1 { birth() {} }
        static box ProductFactoryV1 {
            mixed_boxes(flag) {
                if flag == 0 { return new LeftV1() }
                return new RightV1()
            }
            mixed_scalar(flag) {
                if flag == 0 { return new LeftV1() }
                return 1
            }
            mixed_scalar_reversed(flag) {
                if flag == 0 { return 1 }
                return new LeftV1()
            }
        }
    "#;
    let conflict = VerifiedCallableResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::ConflictingReturnRepresentations,
    );

    assert_eq!(
        disposition(source, "ProductFactoryV1", "mixed_boxes", 1),
        conflict
    );
    assert_eq!(
        disposition(source, "ProductFactoryV1", "mixed_scalar", 1),
        conflict
    );
    assert_eq!(
        disposition(source, "ProductFactoryV1", "mixed_scalar_reversed", 1),
        conflict
    );
}

#[test]
fn invalid_result_surfaces_close_to_explicit_unavailable_rows() {
    let source = r#"
        static box NegativeProofV1 {
            no_value() { return }
            missing() { local value = 1 }
            text() { return "not i64" }
            declared_text(): String { return 1 }
            declared_text_ok(): String { return "abc" }
        }
    "#;

    assert_eq!(
        disposition(source, "NegativeProofV1", "no_value", 0),
        VerifiedCallableResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::NoValueReturn
        )
    );
    assert_eq!(
        disposition(source, "NegativeProofV1", "missing", 0),
        VerifiedCallableResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::MissingReturn
        )
    );
    assert_eq!(
        disposition(source, "NegativeProofV1", "text", 0),
        VerifiedCallableResultDispositionV1::ExactString
    );
    assert_eq!(
        disposition(source, "NegativeProofV1", "declared_text", 0),
        VerifiedCallableResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::DeclaredResultDisagrees
        )
    );
    assert_eq!(
        disposition(source, "NegativeProofV1", "declared_text_ok", 0),
        VerifiedCallableResultDispositionV1::ExactString
    );
}

#[test]
fn declared_box_result_must_be_the_box_the_body_returns() {
    let source = r#"
        box Leaf { birth() { } }
        box Other { birth() { } }
        static box ObjectProofV1 {
            same(): Leaf { return new Leaf() }
            other(): Leaf { return new Other() }
            scalar(): Leaf { return 1 }
        }
    "#;

    assert_eq!(
        disposition(source, "ObjectProofV1", "same", 0),
        VerifiedCallableResultDispositionV1::ExactNominalBox { box_name: "Leaf".into() }
    );
    for method in ["other", "scalar"] {
        assert_eq!(
            disposition(source, "ObjectProofV1", method, 0),
            VerifiedCallableResultDispositionV1::Unavailable(
                CallableResultUnavailableReasonV1::DeclaredResultDisagrees
            )
        );
    }
}

#[test]
fn heterogeneous_untyped_returns_do_not_select_an_arbitrary_representation() {
    let source = r#"
        static box HeterogeneousReturnV1 {
            choose(flag) {
                if flag == 0 { return 1 }
                return "not i64"
            }
        }
    "#;

    assert_eq!(
        disposition(source, "HeterogeneousReturnV1", "choose", 1),
        VerifiedCallableResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::ConflictingReturnRepresentations,
        ),
    );
}

#[test]
fn loop_invariant_ignores_condition_only_unknown_values() {
    let source = r#"
        static box LoopProofV1 {
            skip_ws(src, i) {
                if src == null { return i }
                local s = "" + src
                local n = s.length()
                local j = i
                loop(j < n) {
                    j = j + 1
                }
                return j
            }
        }
    "#;

    assert_eq!(
        disposition(source, "LoopProofV1", "skip_ws", 2),
        exact(&[1])
    );
}

#[test]
fn local_rows_are_exact_but_every_call_result_waits_for_target_authority() {
    let source = r#"
        static box ProviderV1 {
            step(value) { return value + 1 }
            pair(left, right) { return left + right }
            same(value) { return me.step(value) }
        }
        static box CollisionV1 {
            str(value) { return value }
        }
        static box ConsumerV1 {
            qualified(value) { return ProviderV1.step(value) }
            fixed(value) { return ProviderV1.step(41) }
            union(left, right) { return ProviderV1.pair(right, left) }
            bare(value) { return step(value) }
            builtin_collision(value) { return str(value) }
        }
    "#;
    let unavailable = VerifiedCallableResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable,
    );

    assert_eq!(disposition(source, "ProviderV1", "step", 1), exact(&[0]));
    assert_eq!(disposition(source, "ProviderV1", "pair", 2), exact(&[0, 1]));
    assert_eq!(disposition(source, "CollisionV1", "str", 1), exact(&[0]));
    for (owner, name, arity) in [
        ("ProviderV1", "same", 1),
        ("ConsumerV1", "qualified", 1),
        ("ConsumerV1", "fixed", 1),
        ("ConsumerV1", "union", 2),
        ("ConsumerV1", "bare", 1),
        ("ConsumerV1", "builtin_collision", 1),
    ] {
        assert_eq!(disposition(source, owner, name, arity), unavailable);
    }
}

#[test]
fn forward_backward_declaration_order_has_identical_normalized_rows() {
    let provider = r#"
        static box ProviderV1 { step(value) { return value + 1 } }
    "#;
    let consumer = r#"
        static box ConsumerV1 { wrap(value) { return ProviderV1.step(value) } }
    "#;
    let forward = format!("{consumer}\n{provider}");
    let backward = format!("{provider}\n{consumer}");

    assert_eq!(normalized(&forward), normalized(&backward));
    assert_eq!(
        disposition(&forward, "ConsumerV1", "wrap", 1),
        VerifiedCallableResultDispositionV1::Unavailable(
            CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable
        )
    );
}

#[test]
fn direct_and_mutual_calls_stop_at_the_same_target_authority_boundary() {
    let source = r#"
        static box DirectV1 { again(value) { return me.again(value) } }
        static box LeftV1 { call(value) { return RightV1.call(value) } }
        static box RightV1 { call(value) { return LeftV1.call(value) } }
    "#;
    let expected = VerifiedCallableResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable,
    );

    assert_eq!(disposition(source, "DirectV1", "again", 1), expected);
    assert_eq!(disposition(source, "LeftV1", "call", 1), expected);
    assert_eq!(disposition(source, "RightV1", "call", 1), expected);
}

#[test]
fn bare_qualified_and_shadowed_calls_never_guess_a_target() {
    let source = r#"
        static box LeftProviderV1 { step(value) { return value } }
        static box RightProviderV1 { step(value) { return value } }
        static box ConsumerV1 {
            ambiguous(value) { return step(value) }
            shadowed(value) {
                local LeftProviderV1 = 1
                return LeftProviderV1.step(value)
            }
            qualified(value) { return LeftProviderV1.step(value) }
        }
    "#;
    let unavailable = VerifiedCallableResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable,
    );

    assert_eq!(
        disposition(source, "ConsumerV1", "ambiguous", 1),
        unavailable
    );
    assert_eq!(
        disposition(source, "ConsumerV1", "shadowed", 1),
        unavailable
    );
    assert_eq!(
        disposition(source, "ConsumerV1", "qualified", 1),
        unavailable
    );
}

#[test]
fn requirement_ordinals_are_checked_against_the_canonical_arity() {
    let source = r#"static box ProviderV1 { step(value) { return value } }"#;
    let (declarations, _) = seal(source);
    let step = key(&declarations, "ProviderV1", "step", 1);

    assert_eq!(
        VerifiedCallableResultDispositionV1::exact_i64(&step, [1]).unwrap_err(),
        CallableResultCatalogErrorV1::RequiredArgumentOrdinalOutOfRange {
            key: step,
            ordinal: 1,
            arity: 1,
        }
    );
}

#[test]
fn grouped_assignment_anywhere_in_the_body_closes_the_first_proof_grammar() {
    let source = r#"
        static box ProviderV1 {
            constant(unused) { return 1 }
            second(unused, value) { return value }
        }
        static box ConsumerV1 {
            non_required_argument(value) {
                ProviderV1.constant((value = "text"))
                return value
            }
            earlier_argument(value) {
                ProviderV1.second((value = "text"), value)
                return value
            }
            unresolved_argument(value) {
                missing((value = "text"))
                return value
            }
            condition(value) {
                if ((value = "text") == "text") { }
                return value
            }
            loop_condition(value) {
                loop((value = "text") == "other") { break }
                return value
            }
        }
    "#;
    let unavailable = VerifiedCallableResultDispositionV1::Unavailable(
        CallableResultUnavailableReasonV1::UnsupportedExpressionKind,
    );

    for name in [
        "non_required_argument",
        "earlier_argument",
        "unresolved_argument",
        "condition",
        "loop_condition",
    ] {
        assert_eq!(disposition(source, "ConsumerV1", name, 1), unavailable);
    }
}

#[test]
fn instance_methods_never_receive_result_rows() {
    let source = r#"
        box OrdinaryV1 { run(value) { return value } }
        static box StaticV1 { run(value) { return value } }
    "#;
    let (declarations, results) = seal(source);

    assert_eq!(declarations.len(), 2);
    assert_eq!(results.len(), 1);
    assert!(!results.is_empty());
    assert_eq!(
        results.rows().next().map(|(key, _)| key.owner()),
        Some("StaticV1")
    );
}

#[test]
fn actual_string_helpers_keeps_skip_ws_exact_and_records_to_i64_design_boundary() {
    let source = include_str!(concat!(
        "../../../../lang/src/shared/common/",
        "string_helpers.hako"
    ));

    assert_eq!(
        disposition(source, "StringHelpers", "skip_ws", 2),
        exact(&[1])
    );
    assert!(matches!(
        disposition(source, "StringHelpers", "to_i64", 1),
        VerifiedCallableResultDispositionV1::Unavailable(_)
    ));
}

#[test]
fn actual_parser_wrapper_waits_for_canonical_call_target_projection() {
    let helper = include_str!(concat!(
        "../../../../lang/src/shared/common/",
        "string_helpers.hako"
    ));
    let wrapper = include_str!(concat!(
        "../../../../lang/src/compiler/parser/scan/",
        "parser_string_utils_box.hako"
    ));
    let provider_first = format!("{helper}\n{wrapper}");
    let wrapper_first = format!("{wrapper}\n{helper}");

    for source in [&provider_first, &wrapper_first] {
        assert_eq!(
            disposition(source, "StringHelpers", "skip_ws", 2),
            exact(&[1])
        );
        assert_eq!(
            disposition(source, "ParserStringUtilsBox", "skip_ws", 2),
            VerifiedCallableResultDispositionV1::Unavailable(
                CallableResultUnavailableReasonV1::StaticCallTargetAuthorityUnavailable
            )
        );
    }
}

/// R1/R2: a declared `Bool` is proved against its body and must come out `ExactBool`; a declared
/// `i64` keeps its early answer. The declaration is never read back from the body.
#[test]
fn declared_bool_result_must_be_proved_exact_bool_by_its_body() {
    let source = r#"static box DeclaredBoolV1 {
        literal(): Bool { return true }
        equals(x): Bool { return x == " " }
        arithmetic(x): Bool { return x + 1 }
        integer(): Bool { return 1 }
        text(): Bool { return "x" }
        mixed(x): Bool { if x == 0 { return true } return 1 }
        declared_i64(): i64 { return 1 }
    }"#;
    for (method, arity) in [("literal", 0), ("equals", 1)] {
        assert_eq!(disposition(source, "DeclaredBoolV1", method, arity), VerifiedCallableResultDispositionV1::ExactBool { required_i64_arguments: Box::new([]) }, "{method}");
    }
    for (method, arity) in [("arithmetic", 1), ("integer", 0), ("text", 0)] {
        assert_eq!(
            disposition(source, "DeclaredBoolV1", method, arity),
            VerifiedCallableResultDispositionV1::Unavailable(CallableResultUnavailableReasonV1::DeclaredResultDisagrees),
            "{method}"
        );
    }
    // A body that already conflicts with itself keeps its own named reason.
    assert_eq!(
        disposition(source, "DeclaredBoolV1", "mixed", 1),
        VerifiedCallableResultDispositionV1::Unavailable(CallableResultUnavailableReasonV1::ConflictingReturnRepresentations)
    );
    assert!(matches!(
        disposition(source, "DeclaredBoolV1", "declared_i64", 0),
        VerifiedCallableResultDispositionV1::ExactI64 { .. }
    ));
}
