use super::ExprLoweringError;
use super::super::scope_manager::ScopeManager;
use crate::ast::ASTNode;
#[cfg(feature = "normalized_dev")]
use crate::mir::builder::MirBuilder;
use crate::mir::join_ir::lowering::condition_env::ConditionEnv;

/// Phase 79-1: Build ConditionEnv with BindingId support (dev-only)
///
/// This is the BindingId-aware version that checks builder.binding_map first.
#[cfg(feature = "normalized_dev")]
pub(crate) fn build_condition_env_from_scope_with_binding<S: ScopeManager>(
    scope: &S,
    ast: &ASTNode,
    builder: &MirBuilder,
) -> Result<ConditionEnv, ExprLoweringError> {
    let mut env = ConditionEnv::new();
    let mut vars = Vec::new();
    collect_vars(ast, &mut vars);

    for var in vars {
        let binding_id = builder.binding_map.get(&var).copied();

        #[cfg(feature = "normalized_dev")]
        if let Some(bid) = binding_id {
            if let Some(value_id) = scope.lookup_with_binding(Some(bid), &var) {
                env.insert(var.clone(), value_id);
                continue;
            }
        }

        // Fallback to name-based lookup
        if let Some(id) = scope.lookup(&var) {
            env.insert(var.clone(), id);
        } else {
            return Err(ExprLoweringError::VariableNotFound(var));
        }
    }

    Ok(env)
}

/// Phase 79-1: Legacy name-only version (for non-dev builds)
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

/// Collect variable names from AST (simple traversal for supported nodes)
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
