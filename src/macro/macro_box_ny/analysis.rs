use nyash_rust::ASTNode;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacroBehavior {
    Identity,
    Uppercase,
    ArrayPrependZero,
    MapInsertTag,
    LoopNormalize,
    IfMatchNormalize,
    ForForeachNormalize,
    EnvTagString,
}

pub fn analyze_macro_file(path: &str) -> MacroBehavior {
    let src = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(_) => return MacroBehavior::Identity,
    };
    let ast = match nyash_rust::parser::NyashParser::parse_from_string(&src) {
        Ok(a) => a,
        Err(_) => return MacroBehavior::Identity,
    };
    // Quick heuristics based on literals present in file
    fn ast_has_literal_string(a: &ASTNode, needle: &str) -> bool {
        use nyash_rust::ast::ASTNode as A;
        match a {
            A::Literal {
                value: nyash_rust::ast::LiteralValue::String(s),
                ..
            } => s.contains(needle),
            A::Program { statements, .. } => {
                statements.iter().any(|n| ast_has_literal_string(n, needle))
            }
            A::Print { expression, .. } => ast_has_literal_string(expression, needle),
            A::Return { value, .. } => value
                .as_ref()
                .map(|v| ast_has_literal_string(v, needle))
                .unwrap_or(false),
            A::Assignment { target, value, .. } => {
                ast_has_literal_string(target, needle) || ast_has_literal_string(value, needle)
            }
            A::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                ast_has_literal_string(condition, needle)
                    || then_body.iter().any(|n| ast_has_literal_string(n, needle))
                    || else_body
                        .as_ref()
                        .map(|v| v.iter().any(|n| ast_has_literal_string(n, needle)))
                        .unwrap_or(false)
            }
            A::FunctionDeclaration { body, .. } => {
                body.iter().any(|n| ast_has_literal_string(n, needle))
            }
            A::BinaryOp { left, right, .. } => {
                ast_has_literal_string(left, needle) || ast_has_literal_string(right, needle)
            }
            A::UnaryOp { operand, .. } => ast_has_literal_string(operand, needle),
            A::MethodCall {
                object, arguments, ..
            } => {
                ast_has_literal_string(object, needle)
                    || arguments.iter().any(|n| ast_has_literal_string(n, needle))
            }
            A::FunctionCall { arguments, .. } => {
                arguments.iter().any(|n| ast_has_literal_string(n, needle))
            }
            A::ArrayLiteral { elements, .. } => {
                elements.iter().any(|n| ast_has_literal_string(n, needle))
            }
            A::MapLiteral { entries, .. } => entries
                .iter()
                .any(|(_, v)| ast_has_literal_string(v, needle)),
            _ => false,
        }
    }
    fn ast_has_method(a: &ASTNode, method: &str) -> bool {
        use nyash_rust::ast::ASTNode as A;
        match a {
            A::Program { statements, .. } => statements.iter().any(|n| ast_has_method(n, method)),
            A::Print { expression, .. } => ast_has_method(expression, method),
            A::Return { value, .. } => value
                .as_ref()
                .map(|v| ast_has_method(v, method))
                .unwrap_or(false),
            A::Assignment { target, value, .. } => {
                ast_has_method(target, method) || ast_has_method(value, method)
            }
            A::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                ast_has_method(condition, method)
                    || then_body.iter().any(|n| ast_has_method(n, method))
                    || else_body
                        .as_ref()
                        .map(|v| v.iter().any(|n| ast_has_method(n, method)))
                        .unwrap_or(false)
            }
            A::FunctionDeclaration { body, .. } => body.iter().any(|n| ast_has_method(n, method)),
            A::BinaryOp { left, right, .. } => {
                ast_has_method(left, method) || ast_has_method(right, method)
            }
            A::UnaryOp { operand, .. } => ast_has_method(operand, method),
            A::MethodCall {
                object,
                method: m,
                arguments,
                ..
            } => {
                m == method
                    || ast_has_method(object, method)
                    || arguments.iter().any(|n| ast_has_method(n, method))
            }
            A::FunctionCall { arguments, .. } => {
                arguments.iter().any(|n| ast_has_method(n, method))
            }
            A::ArrayLiteral { elements, .. } => elements.iter().any(|n| ast_has_method(n, method)),
            A::MapLiteral { entries, .. } => entries.iter().any(|(_, v)| ast_has_method(v, method)),
            _ => false,
        }
    }
    // Detect array prepend-zero macro by pattern strings present in macro source
    if ast_has_literal_string(&ast, "\"kind\":\"Array\",\"elements\":[")
        || ast_has_literal_string(&ast, "\"elements\":[")
    {
        return MacroBehavior::ArrayPrependZero;
    }
    // Detect map insert-tag macro by pattern strings
    if ast_has_literal_string(&ast, "\"kind\":\"Map\",\"entries\":[")
        || ast_has_literal_string(&ast, "\"entries\":[")
    {
        return MacroBehavior::MapInsertTag;
    }
    // Detect upper-string macro by pattern or toUpperCase usage
    if ast_has_literal_string(&ast, "\"value\":\"UPPER:") || ast_has_method(&ast, "toUpperCase") {
        return MacroBehavior::Uppercase;
    }
    // Detect env-tag string macro by name literal as fallback
    if ast_has_literal_string(&ast, "EnvTagString") {
        return MacroBehavior::EnvTagString;
    }
    if let ASTNode::Program { statements, .. } = ast {
        for st in statements {
            if let ASTNode::BoxDeclaration {
                name: _, methods, ..
            } = st
            {
                // Detect LoopNormalize/IfMatchNormalize by name() returning a specific string
                if let Some(ASTNode::FunctionDeclaration {
                    name: mname, body, ..
                }) = methods.get("name")
                {
                    if mname == "name" {
                        if body.len() == 1 {
                            if let ASTNode::Return { value: Some(v), .. } = &body[0] {
                                if let ASTNode::Literal {
                                    value: nyash_rust::ast::LiteralValue::String(s),
                                    ..
                                } = &**v
                                {
                                    if s == "LoopNormalize" {
                                        return MacroBehavior::LoopNormalize;
                                    }
                                    if s == "IfMatchNormalize" {
                                        return MacroBehavior::IfMatchNormalize;
                                    }
                                    if s == "ForForeach" {
                                        return MacroBehavior::ForForeachNormalize;
                                    }
                                    if s == "EnvTagString" {
                                        return MacroBehavior::EnvTagString;
                                    }
                                }
                            }
                        }
                    }
                }
                if let Some(ASTNode::FunctionDeclaration {
                    name: mname,
                    body,
                    params,
                    ..
                }) = methods.get("expand")
                {
                    if mname == "expand" {
                        if expand_indicates_uppercase(body, params) {
                            return MacroBehavior::Uppercase;
                        }
                    }
                }
            }
        }
    }
    MacroBehavior::Identity
}

fn expand_indicates_uppercase(body: &[ASTNode], params: &[String]) -> bool {
    if body.len() != 1 {
        return false;
    }
    let p0 = params.get(0).cloned().unwrap_or_else(|| "ast".to_string());
    match &body[0] {
        ASTNode::Return { value: Some(v), .. } => match &**v {
            ASTNode::FunctionCall {
                name, arguments, ..
            } => {
                if (name == "uppercase_print" || name == "upper_print") && arguments.len() == 1 {
                    if let ASTNode::Variable { name: an, .. } = &arguments[0] {
                        return an == &p0;
                    }
                }
                false
            }
            _ => false,
        },
        _ => false,
    }
}

pub(crate) fn caps_allow_macro_source(ast: &ASTNode) -> Result<(), String> {
    let allow_io = crate::config::env::macro_cap_io().unwrap_or(false);
    let allow_net = crate::config::env::macro_cap_net().unwrap_or(false);
    use nyash_rust::ast::ASTNode as A;
    fn scan(n: &A, seen: &mut Vec<String>) {
        match n {
            A::New { class, .. } => seen.push(class.clone()),
            A::Program { statements, .. } => {
                for s in statements {
                    scan(s, seen);
                }
            }
            A::FunctionDeclaration { body, .. } => {
                for s in body {
                    scan(s, seen);
                }
            }
            A::Assignment { target, value, .. } => {
                scan(target, seen);
                scan(value, seen);
            }
            A::Return { value, .. } => {
                if let Some(v) = value {
                    scan(v, seen);
                }
            }
            A::If {
                condition,
                then_body,
                else_body,
                ..
            } => {
                scan(condition, seen);
                for s in then_body {
                    scan(s, seen);
                }
                if let Some(b) = else_body {
                    for s in b {
                        scan(s, seen);
                    }
                }
            }
            A::BinaryOp { left, right, .. } => {
                scan(left, seen);
                scan(right, seen);
            }
            A::UnaryOp { operand, .. } => scan(operand, seen),
            A::MethodCall {
                object, arguments, ..
            } => {
                scan(object, seen);
                for a in arguments {
                    scan(a, seen);
                }
            }
            A::FunctionCall { arguments, .. } => {
                for a in arguments {
                    scan(a, seen);
                }
            }
            A::ArrayLiteral { elements, .. } => {
                for e in elements {
                    scan(e, seen);
                }
            }
            A::MapLiteral { entries, .. } => {
                for (_, v) in entries {
                    scan(v, seen);
                }
            }
            _ => {}
        }
    }
    let mut boxes = Vec::new();
    scan(ast, &mut boxes);
    if !allow_io
        && boxes
            .iter()
            .any(|c| c == "FileBox" || c == "PathBox" || c == "DirBox")
    {
        return Err("macro capability violation: IO (File/Path/Dir) denied".into());
    }
    if !allow_net
        && boxes
            .iter()
            .any(|c| c.contains("HTTP") || c.contains("Http") || c == "SocketBox")
    {
        return Err("macro capability violation: NET (HTTP/Socket) denied".into());
    }
    Ok(())
}
