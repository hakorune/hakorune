use super::super::scope_manager::ScopeManager;
use super::ExprLoweringError;
use crate::ast::ASTNode;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;

pub(crate) fn build_condition_env_from_scope<S: ScopeManager>(
    scope: &S,
    ast: &ASTNode,
) -> Result<ConditionEnv, ExprLoweringError> {
    let mut env = ConditionEnv::new();
    let mut vars = Vec::new();
    collect_vars(ast, &mut vars);

    for var in vars {
        if let Some(id) = scope.lookup(&var) {
            env.insert(var.clone(), id);
        } else {
            return Err(ExprLoweringError::VariableNotFound(var));
        }
    }

    Ok(env)
}

fn collect_vars(ast: &ASTNode, vars: &mut Vec<String>) {
    match ast {
        ASTNode::Variable { name, .. } => vars.push(name.clone()),
        ASTNode::BinaryOp { left, right, .. } => {
            collect_vars(left, vars);
            collect_vars(right, vars);
        }
        ASTNode::UnaryOp { operand, .. } => collect_vars(operand, vars),
        ASTNode::MethodCall {
            object, arguments, ..
        } => {
            collect_vars(object, vars);
            for arg in arguments {
                collect_vars(arg, vars);
            }
        }
        _ => {}
    }
}
