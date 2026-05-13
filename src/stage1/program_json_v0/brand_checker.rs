use crate::ast::{ASTNode, ParamDecl};
use std::collections::BTreeMap;

#[derive(Clone, Debug)]
struct FunctionSig {
    params: Vec<Option<String>>,
}

#[derive(Clone, Debug)]
struct BrandEnv {
    vars: BTreeMap<String, Option<String>>,
}

impl BrandEnv {
    fn from_params(
        params: &[String],
        param_decls: &[ParamDecl],
        brands: &BTreeMap<String, String>,
    ) -> Self {
        let mut vars = BTreeMap::new();
        for decl in ParamDecl::with_name_fallback(param_decls, params).iter() {
            vars.insert(
                decl.name.clone(),
                brand_type(decl.declared_type_name.as_deref(), brands),
            );
        }
        Self { vars }
    }

    fn bind(&mut self, name: &str, brand: Option<String>) {
        self.vars.insert(name.to_string(), brand);
    }

    fn get(&self, name: &str) -> Option<String> {
        self.vars.get(name).cloned().flatten()
    }
}

pub(super) fn check_brand_mismatches(
    ast: &ASTNode,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    if brands.is_empty() {
        return Ok(());
    }

    let sigs = collect_signatures(ast, brands);
    let ASTNode::Program { statements, .. } = ast else {
        return Ok(());
    };

    for statement in statements {
        match statement {
            ASTNode::FunctionDeclaration {
                params,
                param_decls,
                body,
                name,
                ..
            } => {
                let mut env = BrandEnv::from_params(params, param_decls, brands);
                check_body(body, None, name, &mut env, &sigs, brands)?;
            }
            ASTNode::BoxDeclaration { name, methods, .. } => {
                for method in methods.values() {
                    if let ASTNode::FunctionDeclaration {
                        params,
                        param_decls,
                        body,
                        name: method_name,
                        ..
                    } = method
                    {
                        let mut env = BrandEnv::from_params(params, param_decls, brands);
                        check_body(body, Some(name), method_name, &mut env, &sigs, brands)?;
                    }
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn collect_signatures(
    ast: &ASTNode,
    brands: &BTreeMap<String, String>,
) -> BTreeMap<String, FunctionSig> {
    let mut sigs = BTreeMap::new();
    let ASTNode::Program { statements, .. } = ast else {
        return sigs;
    };

    for statement in statements {
        match statement {
            ASTNode::FunctionDeclaration {
                name,
                params,
                param_decls,
                ..
            } => {
                sigs.insert(name.clone(), function_sig(params, param_decls, brands));
            }
            ASTNode::BoxDeclaration { name, methods, .. } => {
                for method in methods.values() {
                    if let ASTNode::FunctionDeclaration {
                        name: method_name,
                        params,
                        param_decls,
                        ..
                    } = method
                    {
                        sigs.insert(
                            format!("{}.{}", name, method_name),
                            function_sig(params, param_decls, brands),
                        );
                    }
                }
            }
            _ => {}
        }
    }
    sigs
}

fn function_sig(
    params: &[String],
    param_decls: &[ParamDecl],
    brands: &BTreeMap<String, String>,
) -> FunctionSig {
    FunctionSig {
        params: ParamDecl::with_name_fallback(param_decls, params)
            .iter()
            .map(|decl| brand_type(decl.declared_type_name.as_deref(), brands))
            .collect(),
    }
}

fn brand_type(type_name: Option<&str>, brands: &BTreeMap<String, String>) -> Option<String> {
    let type_name = type_name?;
    brands
        .contains_key(type_name)
        .then(|| type_name.to_string())
}

fn check_body(
    body: &[ASTNode],
    current_box: Option<&String>,
    current_fn: &str,
    env: &mut BrandEnv,
    sigs: &BTreeMap<String, FunctionSig>,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    for statement in body {
        check_statement(statement, current_box, current_fn, env, sigs, brands)?;
    }
    Ok(())
}

fn check_statement(
    statement: &ASTNode,
    current_box: Option<&String>,
    current_fn: &str,
    env: &mut BrandEnv,
    sigs: &BTreeMap<String, FunctionSig>,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    match statement {
        ASTNode::Local {
            variables,
            initial_values,
            ..
        } => {
            for (index, name) in variables.iter().enumerate() {
                let brand = initial_values
                    .get(index)
                    .and_then(|value| value.as_deref())
                    .map(|expr| check_expr(expr, current_box, current_fn, env, sigs, brands))
                    .transpose()?
                    .flatten();
                env.bind(name, brand);
            }
        }
        ASTNode::Assignment { value, .. }
        | ASTNode::Print {
            expression: value, ..
        }
        | ASTNode::Throw {
            expression: value, ..
        } => {
            check_expr(value, current_box, current_fn, env, sigs, brands)?;
        }
        ASTNode::Return { value, .. } => {
            if let Some(value) = value {
                check_expr(value, current_box, current_fn, env, sigs, brands)?;
            }
        }
        ASTNode::If {
            condition,
            then_body,
            else_body,
            ..
        } => {
            check_expr(condition, current_box, current_fn, env, sigs, brands)?;
            let mut then_env = env.clone();
            check_body(
                then_body,
                current_box,
                current_fn,
                &mut then_env,
                sigs,
                brands,
            )?;
            if let Some(else_body) = else_body {
                let mut else_env = env.clone();
                check_body(
                    else_body,
                    current_box,
                    current_fn,
                    &mut else_env,
                    sigs,
                    brands,
                )?;
            }
        }
        ASTNode::Loop {
            condition, body, ..
        }
        | ASTNode::While {
            condition, body, ..
        } => {
            check_expr(condition, current_box, current_fn, env, sigs, brands)?;
            let mut loop_env = env.clone();
            check_body(body, current_box, current_fn, &mut loop_env, sigs, brands)?;
        }
        ASTNode::ForRange {
            start, end, body, ..
        } => {
            check_expr(start, current_box, current_fn, env, sigs, brands)?;
            check_expr(end, current_box, current_fn, env, sigs, brands)?;
            let mut loop_env = env.clone();
            check_body(body, current_box, current_fn, &mut loop_env, sigs, brands)?;
        }
        ASTNode::ScopeBox { body, .. } => {
            let mut scope_env = env.clone();
            check_body(body, current_box, current_fn, &mut scope_env, sigs, brands)?;
        }
        ASTNode::FunctionCall { .. } | ASTNode::MethodCall { .. } | ASTNode::Call { .. } => {
            check_expr(statement, current_box, current_fn, env, sigs, brands)?;
        }
        _ => {}
    }
    Ok(())
}

fn check_expr(
    expr: &ASTNode,
    current_box: Option<&String>,
    current_fn: &str,
    env: &BrandEnv,
    sigs: &BTreeMap<String, FunctionSig>,
    brands: &BTreeMap<String, String>,
) -> Result<Option<String>, String> {
    match expr {
        ASTNode::Variable { name, .. } => Ok(env.get(name)),
        ASTNode::FunctionCall {
            name, arguments, ..
        } => {
            for argument in arguments {
                check_expr(argument, current_box, current_fn, env, sigs, brands)?;
            }
            if brands.contains_key(name) {
                return Ok(Some(name.clone()));
            }
            if let Some(sig) = sigs.get(name) {
                check_call_args(
                    name,
                    arguments,
                    sig,
                    current_box,
                    current_fn,
                    env,
                    sigs,
                    brands,
                )?;
            }
            Ok(None)
        }
        ASTNode::MethodCall {
            object,
            method,
            arguments,
            ..
        } => {
            for argument in arguments {
                check_expr(argument, current_box, current_fn, env, sigs, brands)?;
            }
            if let Some(static_receiver) = static_path_from_expr(object) {
                if brands.contains_key(&static_receiver) && method == "unwrap" {
                    return Ok(None);
                }
                let key = format!("{}.{}", static_receiver, method);
                if let Some(sig) = sigs.get(&key) {
                    check_call_args(
                        &key,
                        arguments,
                        sig,
                        current_box,
                        current_fn,
                        env,
                        sigs,
                        brands,
                    )?;
                }
                return Ok(None);
            }
            if matches!(object.as_ref(), ASTNode::Me { .. }) {
                if let Some(box_name) = current_box {
                    let key = format!("{}.{}", box_name, method);
                    if let Some(sig) = sigs.get(&key) {
                        check_call_args(
                            &key,
                            arguments,
                            sig,
                            current_box,
                            current_fn,
                            env,
                            sigs,
                            brands,
                        )?;
                    }
                }
            } else {
                check_expr(object, current_box, current_fn, env, sigs, brands)?;
            }
            Ok(None)
        }
        ASTNode::Call {
            callee, arguments, ..
        } => {
            for argument in arguments {
                check_expr(argument, current_box, current_fn, env, sigs, brands)?;
            }
            if let Some(key) = static_path_from_expr(callee) {
                if let Some(sig) = sigs.get(&key) {
                    check_call_args(
                        &key,
                        arguments,
                        sig,
                        current_box,
                        current_fn,
                        env,
                        sigs,
                        brands,
                    )?;
                }
            }
            Ok(None)
        }
        ASTNode::BlockExpr {
            prelude_stmts,
            tail_expr,
            ..
        } => {
            let mut block_env = env.clone();
            check_body(
                prelude_stmts,
                current_box,
                current_fn,
                &mut block_env,
                sigs,
                brands,
            )?;
            check_expr(tail_expr, current_box, current_fn, &block_env, sigs, brands)
        }
        ASTNode::BinaryOp { left, right, .. } => {
            check_expr(left, current_box, current_fn, env, sigs, brands)?;
            check_expr(right, current_box, current_fn, env, sigs, brands)?;
            Ok(None)
        }
        ASTNode::UnaryOp { operand, .. } => {
            check_expr(operand, current_box, current_fn, env, sigs, brands)?;
            Ok(None)
        }
        ASTNode::FieldAccess { object, .. } => {
            check_expr(object, current_box, current_fn, env, sigs, brands)?;
            Ok(None)
        }
        ASTNode::Index { target, index, .. } => {
            check_expr(target, current_box, current_fn, env, sigs, brands)?;
            check_expr(index, current_box, current_fn, env, sigs, brands)?;
            Ok(None)
        }
        ASTNode::New { arguments, .. } | ASTNode::FromCall { arguments, .. } => {
            for argument in arguments {
                check_expr(argument, current_box, current_fn, env, sigs, brands)?;
            }
            Ok(None)
        }
        _ => Ok(None),
    }
}

fn check_call_args(
    call_name: &str,
    arguments: &[ASTNode],
    sig: &FunctionSig,
    current_box: Option<&String>,
    current_fn: &str,
    env: &BrandEnv,
    sigs: &BTreeMap<String, FunctionSig>,
    brands: &BTreeMap<String, String>,
) -> Result<(), String> {
    for (index, expected) in sig.params.iter().enumerate() {
        let Some(expected_brand) = expected else {
            continue;
        };
        let actual = arguments
            .get(index)
            .map(|arg| check_expr(arg, current_box, current_fn, env, sigs, brands))
            .transpose()?
            .flatten();
        if actual.as_deref() != Some(expected_brand.as_str()) {
            let actual = actual.unwrap_or_else(|| "unbranded".to_string());
            return Err(format!(
                "[brand/mismatch] {} arg {} expected {}, got {} while checking {}",
                call_name,
                index + 1,
                expected_brand,
                actual,
                current_fn
            ));
        }
    }
    Ok(())
}

fn static_path_from_expr(expression: &ASTNode) -> Option<String> {
    match expression {
        ASTNode::Variable { name, .. } if looks_like_static_symbol(name) => Some(name.clone()),
        ASTNode::FieldAccess { object, field, .. } => {
            let base = static_path_from_expr(object)?;
            Some(format!("{}.{}", base, field))
        }
        _ => None,
    }
}

fn looks_like_static_symbol(name: &str) -> bool {
    name.chars()
        .next()
        .map(|ch| ch.is_ascii_uppercase())
        .unwrap_or(false)
}
