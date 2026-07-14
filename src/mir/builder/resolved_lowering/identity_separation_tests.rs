use crate::ast::{ASTNode, DeclarationAttrs, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{BindingKindV1, SourceBindingSiteV1};
use crate::mir::ValueId;

use super::identity::ResolvedIdentityStateV1;

fn parameter_only_fixture() -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "identity_separation_fixture".into(),
        params: vec!["arg".into()],
        param_decls: Vec::new(),
        return_type_name: None,
        body: Vec::new(),
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn published_parameter_state() -> (VerifiedResolvedSourceUnitV1, ValueId) {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(parameter_only_fixture()).unwrap();
    (unit, ValueId::new(7))
}

#[test]
fn duplicate_scope_success_input_preserves_pre_split_behavior() {
    let (unit, value) = published_parameter_state();
    let input = unit.root_function_input().unwrap();
    let mut state = ResolvedIdentityStateV1::new(input.function());
    let binding = state
        .publish_declaration(
            &SourceBindingSiteV1::Parameter { index: 0 },
            BindingKindV1::Parameter { index: 0 },
            "arg",
            value,
        )
        .unwrap();

    state.retire_scope_success(&[binding, binding]).unwrap();
    assert!(state.current_value(binding).is_err());
    state.finish().unwrap();
}

#[test]
fn scope_error_cleanup_is_value_first_and_idempotent() {
    let (unit, value) = published_parameter_state();
    let input = unit.root_function_input().unwrap();
    let mut state = ResolvedIdentityStateV1::new(input.function());
    let binding = state
        .publish_declaration(
            &SourceBindingSiteV1::Parameter { index: 0 },
            BindingKindV1::Parameter { index: 0 },
            "arg",
            value,
        )
        .unwrap();

    state.retire_scope_error(&[binding, binding]);
    state.retire_scope_error(&[binding]);
    assert!(state.current_value(binding).is_err());
    state.finish().unwrap();
}
