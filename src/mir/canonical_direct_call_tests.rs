use crate::ast::{ASTNode, DeclarationAttrs, ParamDecl, Span};
use crate::mir::compiler::VerifiedResolvedCallableProgramV1;
use crate::mir::resolved_semantics::VerifiedCallableHeaderV1;

use super::*;
use crate::mir::canonical_direct_call_contract::VerifiedDirectCallEffectV1;

fn header() -> VerifiedCallableHeaderV1 {
    let tree = ASTNode::FunctionDeclaration {
        name: "countdown".to_string(),
        params: vec!["n".to_string()],
        param_decls: vec![ParamDecl {
            name: "n".to_string(),
            declared_type_name: Some("i64".to_string()),
        }],
        return_type_name: Some("i64".to_string()),
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    };
    let source = VerifiedResolvedCallableProgramV1::resolve(ASTNode::Program {
        statements: vec![tree],
        span: Span::unknown(),
    })
    .unwrap();
    source
        .module()
        .source()
        .catalog()
        .index()
        .resolve_free_static_source_call("countdown", 1)
        .unwrap()
        .clone()
}

#[test]
fn materializes_exact_callee_without_legacy_resolution() {
    let header = header();
    let expected_owner = header.callable().owner();
    let emission = VerifiedCanonicalDirectCallEmissionV1::conservative_from_header(&header);
    assert_eq!(emission.target().callable().owner(), expected_owner);
    assert_eq!(emission.target().signature().arity(), 1);
    assert_eq!(
        emission.effect(),
        VerifiedDirectCallEffectV1::ConservativeBarrier
    );

    let expected_effects =
        materialize_direct_call_effect_v1(VerifiedDirectCallEffectV1::ConservativeBarrier);
    let instruction = emission
        .materialize(ValueId::new(9), vec![ValueId::new(3)])
        .unwrap();
    let MirInstruction::Call {
        dst,
        func,
        callee,
        args,
        effects,
    } = instruction
    else {
        unreachable!()
    };
    assert_eq!(dst, Some(ValueId::new(9)));
    assert_eq!(func, ValueId::INVALID);
    assert_eq!(callee, Some(Callee::Global("countdown/1".to_string())));
    assert_eq!(args, vec![ValueId::new(3)]);
    assert_eq!(effects, expected_effects);
    assert!(effects.contains(Effect::Barrier));
    assert!(!effects.contains(Effect::Pure));
    assert!(!effects.is_read_only());
    assert!(!effects.is_parallel_safe());
    assert!(!effects.is_moveable());
}

#[test]
fn rejects_argument_cardinality_before_materialization() {
    let emission = VerifiedCanonicalDirectCallEmissionV1::conservative_from_header(&header());
    assert_eq!(
        emission.materialize(ValueId::new(9), Vec::new()),
        Err(DirectCallEmissionErrorV1::ArgumentCardinality {
            expected: 1,
            actual: 0,
        })
    );
}
