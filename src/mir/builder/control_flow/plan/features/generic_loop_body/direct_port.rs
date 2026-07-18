//! Associated-input sequence owner for direct GenericLoopV1 bodies.
//!
//! The caller owns mode and step selection. This module only consumes an
//! already-selected statement sequence in order and stops after a terminal
//! plan. Statement semantics stay injected so Parts can remain their SSOT.

use crate::mir::builder::control_flow::plan::{LoopPlanExpressionPortV1, LoweredRecipe};
use crate::mir::builder::MirBuilder;
use crate::mir::ValueId;
use std::collections::BTreeMap;

use super::body_plans_exit_on_all_paths;

pub(in crate::mir::builder) fn lower_generic_loop_v1_direct_inputs<
    'input,
    P,
    Inputs,
    LowerStatement,
>(
    builder: &mut MirBuilder,
    current_bindings: &mut BTreeMap<String, ValueId>,
    port: &P,
    statements: Inputs,
    mut lower_statement: LowerStatement,
) -> Result<Vec<LoweredRecipe>, String>
where
    P: LoopPlanExpressionPortV1 + 'input,
    Inputs: IntoIterator<Item = P::StmtInput<'input>>,
    LowerStatement: FnMut(
        &mut MirBuilder,
        &mut BTreeMap<String, ValueId>,
        &P,
        P::StmtInput<'input>,
    ) -> Result<Vec<LoweredRecipe>, String>,
{
    let mut plans = Vec::new();
    for statement in statements {
        plans.extend(lower_statement(builder, current_bindings, port, statement)?);
        if body_plans_exit_on_all_paths(&plans) {
            break;
        }
    }
    Ok(plans)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{ASTNode, LiteralValue, Span};
    use crate::mir::builder::control_flow::plan::{
        CoreExitPlan, CorePlan, RawLoopPlanExpressionPortV1,
    };

    #[test]
    fn generic_loop_v1_direct_port_preserves_order_and_stops_after_terminal() {
        let statements = vec![local("first"), returning(), local("unreachable")];
        let port = RawLoopPlanExpressionPortV1::new();
        let mut builder = MirBuilder::new();
        let mut bindings = BTreeMap::new();
        let mut visited = Vec::new();

        let plans = lower_generic_loop_v1_direct_inputs(
            &mut builder,
            &mut bindings,
            &port,
            statements.iter(),
            |_builder, _bindings, port, statement| {
                visited.push(port.stmt_syntax(&statement).node_type().to_string());
                if matches!(port.stmt_syntax(&statement), ASTNode::Return { .. }) {
                    Ok(vec![CorePlan::Exit(CoreExitPlan::Return(None))])
                } else {
                    Ok(Vec::new())
                }
            },
        )
        .expect("direct sequence lowers");

        assert_eq!(visited, ["Local", "Return"]);
        assert!(matches!(
            plans.as_slice(),
            [CorePlan::Exit(CoreExitPlan::Return(None))]
        ));
    }

    #[test]
    fn generic_loop_v1_direct_port_failure_stops_before_later_inputs() {
        let statements = vec![local("first"), local("fail"), local("unreachable")];
        let port = RawLoopPlanExpressionPortV1::new();
        let mut builder = MirBuilder::new();
        let mut bindings = BTreeMap::new();
        let mut visited = Vec::new();

        let error = lower_generic_loop_v1_direct_inputs(
            &mut builder,
            &mut bindings,
            &port,
            statements.iter(),
            |_builder, _bindings, port, statement| {
                let ASTNode::Local { variables, .. } = port.stmt_syntax(&statement) else {
                    unreachable!();
                };
                visited.push(variables[0].clone());
                if variables[0] == "fail" {
                    Err("selected direct statement failed".to_string())
                } else {
                    Ok(Vec::new())
                }
            },
        )
        .expect_err("selected failure propagates");

        assert_eq!(error, "selected direct statement failed");
        assert_eq!(visited, ["first", "fail"]);
    }

    #[test]
    fn generic_loop_v1_direct_port_accepts_empty_prefix() {
        let port = RawLoopPlanExpressionPortV1::new();
        let mut builder = MirBuilder::new();
        let mut bindings = BTreeMap::new();
        let statements: [&ASTNode; 0] = [];

        let plans = lower_generic_loop_v1_direct_inputs(
            &mut builder,
            &mut bindings,
            &port,
            statements,
            |_builder, _bindings, _port, _statement| -> Result<Vec<CorePlan>, String> {
                unreachable!("empty prefix does not invoke statement lowering")
            },
        )
        .expect("empty prefix is a valid direct body");

        assert!(plans.is_empty());
    }

    fn local(name: &str) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }))],
            declared_type_names: Vec::new(),
            span: Span::unknown(),
        }
    }

    fn returning() -> ASTNode {
        ASTNode::Return {
            value: None,
            span: Span::unknown(),
        }
    }
}
