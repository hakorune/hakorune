use super::raw_structured_child_scope::RawStructuredChildScopePortV1;
use super::recursive_child_lowering_port::{
    RecursiveChildLoweringPortV1, ScriptDirectStaticClaimIngressV1,
};
use crate::ast::ASTNode;
use crate::mir::{MirBuilder, ValueId};

struct DefaultPort;

impl RecursiveChildLoweringPortV1 for DefaultPort {
    type BodyInput = Vec<ASTNode>;
    type StatementInput = ASTNode;
    type ExpressionInput = ASTNode;

    fn lower_body(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::BodyInput,
    ) -> Result<ValueId, String> {
        unreachable!("test port does not lower bodies")
    }

    fn lower_statement(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::StatementInput,
    ) -> Result<ValueId, String> {
        unreachable!("test port does not lower statements")
    }

    fn lower_expression(
        &mut self,
        _builder: &mut MirBuilder,
        _input: Self::ExpressionInput,
    ) -> Result<ValueId, String> {
        unreachable!("test port does not lower expressions")
    }
}

#[test]
fn default_claim_ingress_is_non_consuming_and_unavailable() {
    let mut port = DefaultPort;
    let first = RecursiveChildLoweringPortV1::script_direct_static_claim_ingress_v1(
        &mut port, "Helper", "value", 0,
    )
    .expect("default ingress is infallible");
    let second = RecursiveChildLoweringPortV1::script_direct_static_claim_ingress_v1(
        &mut port, "Helper", "value", 0,
    )
    .expect("default ingress remains infallible");
    assert_eq!(first, ScriptDirectStaticClaimIngressV1::Unavailable);
    assert_eq!(second, ScriptDirectStaticClaimIngressV1::Unavailable);
}

#[test]
fn structured_scope_delegates_the_non_consuming_hook() {
    let mut child = DefaultPort;
    let mut scope = RawStructuredChildScopePortV1::new(&mut child, vec![], vec![]);
    let result = RecursiveChildLoweringPortV1::script_direct_static_claim_ingress_v1(
        &mut scope, "Helper", "value", 0,
    )
    .expect("structured delegation is infallible for the default child");
    assert_eq!(result, ScriptDirectStaticClaimIngressV1::Unavailable);
}
