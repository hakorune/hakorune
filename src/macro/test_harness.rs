use nyash_rust::ASTNode;

pub fn maybe_inject_test_harness(ast: &ASTNode) -> ASTNode {
    if !crate::config::env::test_run() {
        return ast.clone();
    }
    // Test call plan
    #[derive(Clone)]
    struct TestPlan {
        setup: Option<nyash_rust::ASTNode>,
        call: nyash_rust::ASTNode,
    }

    // Collect tests (top-level and Box)
    let mut tests: Vec<TestPlan> = Vec::new();
    let mut _main_params_len: usize = 0;

    // Optional JSON args:
    // - Simple: { "test_name": [1, "s", true], "Box.method": [ ... ] }
    // - Detailed per-test: { "Box.method": { "args": [...], "instance": {"ctor":"new|birth","args":[...] } } }
    // - Typed values inside args supported via objects (see json_to_ast below)
    #[derive(Clone, Default)]
    struct InstanceSpec {
        ctor: String,
        args: Vec<nyash_rust::ASTNode>,
        type_args: Vec<String>,
    }
    #[derive(Clone, Default)]
    struct TestArgSpec {
        args: Vec<nyash_rust::ASTNode>,
        instance: Option<InstanceSpec>,
    }

    fn json_err(msg: &str) {
        crate::macro_log!("[macro][test][args] {}", msg);
    }

    fn json_to_ast(v: &serde_json::Value) -> Result<nyash_rust::ASTNode, String> {
        use nyash_rust::ast::{ASTNode as A, LiteralValue, Span};
        match v {
            serde_json::Value::String(st) => Ok(A::Literal {
                value: LiteralValue::String(st.clone()),
                span: Span::unknown(),
            }),
            serde_json::Value::Bool(b) => Ok(A::Literal {
                value: LiteralValue::Bool(*b),
                span: Span::unknown(),
            }),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(A::Literal {
                        value: LiteralValue::Integer(i),
                        span: Span::unknown(),
                    })
                } else if let Some(f) = n.as_f64() {
                    Ok(A::Literal {
                        value: LiteralValue::Float(f),
                        span: Span::unknown(),
                    })
                } else {
                    Err("unsupported number literal".into())
                }
            }
            serde_json::Value::Null => Ok(A::Literal {
                value: LiteralValue::Null,
                span: Span::unknown(),
            }),
            serde_json::Value::Array(elems) => {
                // Treat nested arrays as ArrayLiteral by default
                let mut out = Vec::with_capacity(elems.len());
                for x in elems {
                    out.push(json_to_ast(x)?);
                }
                Ok(A::ArrayLiteral {
                    elements: out,
                    span: Span::unknown(),
                })
            }
            serde_json::Value::Object(obj) => {
                // Typed shorthands accepted: {i:1}|{int:1}, {f:1.2}|{float:1.2}, {s:"x"}|{string:"x"}, {b:true}|{bool:true}
                if let Some(v) = obj.get("i").or_else(|| obj.get("int")) {
                    return json_to_ast(v);
                }
                if let Some(v) = obj.get("f").or_else(|| obj.get("float")) {
                    return json_to_ast(v);
                }
                if let Some(v) = obj.get("s").or_else(|| obj.get("string")) {
                    return json_to_ast(v);
                }
                if let Some(v) = obj.get("b").or_else(|| obj.get("bool")) {
                    return json_to_ast(v);
                }
                if let Some(map) = obj.get("map") {
                    if let Some(mo) = map.as_object() {
                        let mut ents: Vec<(String, nyash_rust::ASTNode)> =
                            Vec::with_capacity(mo.len());
                        for (k, vv) in mo {
                            ents.push((k.clone(), json_to_ast(vv)?));
                        }
                        return Ok(A::MapLiteral {
                            entries: ents,
                            span: Span::unknown(),
                        });
                    } else {
                        return Err("map must be an object".into());
                    }
                }
                if let Some(arr) = obj.get("array") {
                    if let Some(va) = arr.as_array() {
                        let mut out = Vec::with_capacity(va.len());
                        for x in va {
                            out.push(json_to_ast(x)?);
                        }
                        return Ok(A::ArrayLiteral {
                            elements: out,
                            span: Span::unknown(),
                        });
                    } else {
                        return Err("array must be an array".into());
                    }
                }
                if let Some(name) = obj.get("var").and_then(|v| v.as_str()) {
                    return Ok(A::Variable {
                        name: name.to_string(),
                        span: Span::unknown(),
                    });
                }
                if let Some(name) = obj.get("call").and_then(|v| v.as_str()) {
                    let mut args: Vec<A> = Vec::new();
                    if let Some(va) = obj.get("args").and_then(|v| v.as_array()) {
                        for x in va {
                            args.push(json_to_ast(x)?);
                        }
                    }
                    return Ok(A::FunctionCall {
                        name: name.to_string(),
                        arguments: args,
                        span: Span::unknown(),
                    });
                }
                if let Some(method) = obj.get("method").and_then(|v| v.as_str()) {
                    let objv = obj
                        .get("object")
                        .ok_or_else(|| "method requires 'object'".to_string())?;
                    let object = json_to_ast(objv)?;
                    let mut args: Vec<A> = Vec::new();
                    if let Some(va) = obj.get("args").and_then(|v| v.as_array()) {
                        for x in va {
                            args.push(json_to_ast(x)?);
                        }
                    }
                    return Ok(A::MethodCall {
                        object: Box::new(object),
                        method: method.to_string(),
                        arguments: args,
                        span: Span::unknown(),
                    });
                }
                if let Some(bx) = obj.get("box").and_then(|v| v.as_str()) {
                    let mut args: Vec<A> = Vec::new();
                    if let Some(va) = obj.get("args").and_then(|v| v.as_array()) {
                        for x in va {
                            args.push(json_to_ast(x)?);
                        }
                    }
                    let type_args: Vec<String> = obj
                        .get("type_args")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    let ctor = obj.get("ctor").and_then(|v| v.as_str()).unwrap_or("new");
                    if ctor == "new" {
                        return Ok(A::New {
                            class: bx.to_string(),
                            arguments: args,
                            type_arguments: type_args,
                            span: Span::unknown(),
                        });
                    } else if ctor == "birth" {
                        return Ok(A::MethodCall {
                            object: Box::new(A::Variable {
                                name: bx.to_string(),
                                span: Span::unknown(),
                            }),
                            method: "birth".into(),
                            arguments: args,
                            span: Span::unknown(),
                        });
                    } else {
                        return Err(format!(
                            "unknown ctor '{}', expected 'new' or 'birth'",
                            ctor
                        ));
                    }
                }
                Err("unknown object mapping for AST".into())
            }
        }
    }

    fn parse_test_arg_spec(v: &serde_json::Value) -> Option<TestArgSpec> {
        match v {
            serde_json::Value::Array(arr) => {
                let mut out: Vec<nyash_rust::ASTNode> = Vec::new();
                for a in arr {
                    match json_to_ast(a) {
                        Ok(n) => out.push(n),
                        Err(e) => {
                            json_err(&format!("args element error: {}", e));
                            return None;
                        }
                    }
                }
                Some(TestArgSpec {
                    args: out,
                    instance: None,
                })
            }
            serde_json::Value::Object(obj) => {
                let mut spec = TestArgSpec::default();
                if let Some(a) = obj.get("args").and_then(|v| v.as_array()) {
                    let mut out: Vec<nyash_rust::ASTNode> = Vec::new();
                    for x in a {
                        match json_to_ast(x) {
                            Ok(n) => out.push(n),
                            Err(e) => {
                                json_err(&format!("args element error: {}", e));
                                return None;
                            }
                        }
                    }
                    spec.args = out;
                }
                if let Some(inst) = obj.get("instance").and_then(|v| v.as_object()) {
                    let ctor = inst
                        .get("ctor")
                        .and_then(|v| v.as_str())
                        .unwrap_or("new")
                        .to_string();
                    let type_args: Vec<String> = inst
                        .get("type_args")
                        .and_then(|v| v.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|x| x.as_str().map(|s| s.to_string()))
                                .collect()
                        })
                        .unwrap_or_default();
                    let mut args: Vec<nyash_rust::ASTNode> = Vec::new();
                    if let Some(va) = inst.get("args").and_then(|v| v.as_array()) {
                        for x in va {
                            match json_to_ast(x) {
                                Ok(n) => args.push(n),
                                Err(e) => {
                                    json_err(&format!("instance.args element error: {}", e));
                                    return None;
                                }
                            }
                        }
                    }
                    spec.instance = Some(InstanceSpec {
                        ctor,
                        args,
                        type_args,
                    });
                }
                Some(spec)
            }
            _ => {
                json_err("test value must be array or object");
                None
            }
        }
    }

    let args_map: Option<std::collections::HashMap<String, TestArgSpec>> = (|| {
        if let Some(s) = crate::config::env::test_args_json() {
            if s.trim().is_empty() {
                return None;
            }
            if let Ok(v) = serde_json::from_str::<serde_json::Value>(&s) {
                let mut map = std::collections::HashMap::new();
                if let Some(obj) = v.as_object() {
                    for (k, vv) in obj {
                        if let Some(spec) = parse_test_arg_spec(vv) {
                            map.insert(k.clone(), spec);
                        }
                    }
                    return Some(map);
                }
            }
        }
        None
    })();
    if let nyash_rust::ASTNode::Program { statements, .. } = ast {
        for st in statements {
            match st {
                nyash_rust::ASTNode::FunctionDeclaration { name, params, .. } => {
                    if name == "main" {
                        _main_params_len = params.len();
                    }
                    if name.starts_with("test_") {
                        let label = name.clone();
                        // select args: JSON map > defaults > skip
                        let mut maybe_args: Option<Vec<nyash_rust::ASTNode>> = None;
                        if let Some(m) = &args_map {
                            if let Some(v) = m.get(&label) {
                                maybe_args = Some(v.args.clone());
                            }
                        }
                        let args = if let Some(a) = maybe_args {
                            a
                        } else if !params.is_empty() && crate::config::env::test_args_defaults() {
                            let mut a: Vec<nyash_rust::ASTNode> = Vec::new();
                            for _ in params {
                                a.push(nyash_rust::ASTNode::Literal {
                                    value: nyash_rust::ast::LiteralValue::Integer(0),
                                    span: nyash_rust::ast::Span::unknown(),
                                });
                            }
                            a
                        } else if params.is_empty() {
                            Vec::new()
                        } else {
                            crate::macro_log!("[macro][test][args] missing args for {} (need {}), skipping (set NYASH_TEST_ARGS_DEFAULTS=1 for zero defaults)", label, params.len());
                            continue;
                        };
                        tests.push(TestPlan {
                            setup: None,
                            call: nyash_rust::ASTNode::FunctionCall {
                                name: name.clone(),
                                arguments: args,
                                span: nyash_rust::ast::Span::unknown(),
                            },
                        });
                    }
                }
                _ => {}
            }
        }
    }
    // Collect Box tests: static and instance (no-arg only for instance)
    if let nyash_rust::ASTNode::Program { statements, .. } = ast {
        for st in statements {
            if let nyash_rust::ASTNode::BoxDeclaration {
                name: box_name,
                methods,
                ..
            } = st
            {
                for (mname, mnode) in methods {
                    if !mname.starts_with("test_") {
                        continue;
                    }
                    if let nyash_rust::ASTNode::FunctionDeclaration {
                        is_static, params, ..
                    } = mnode
                    {
                        if *is_static {
                            // Static: BoxName.test_*()
                            let mut args: Vec<nyash_rust::ASTNode> = Vec::new();
                            if let Some(m) = &args_map {
                                if let Some(v) = m.get(&format!("{}.{}", box_name, mname)) {
                                    args = v.args.clone();
                                }
                            }
                            if args.is_empty() && !params.is_empty() {
                                if crate::config::env::env_flag("NYASH_TEST_ARGS_DEFAULTS").unwrap_or(false) {
                                    for _ in params {
                                        args.push(nyash_rust::ASTNode::Literal {
                                            value: nyash_rust::ast::LiteralValue::Integer(0),
                                            span: nyash_rust::ast::Span::unknown(),
                                        });
                                    }
                                } else {
                                    crate::macro_log!("[macro][test][args] missing args for {}.{} (need {}), skipping", box_name, mname, params.len());
                                    continue;
                                }
                            }
                            let call = nyash_rust::ASTNode::MethodCall {
                                object: Box::new(nyash_rust::ASTNode::Variable {
                                    name: box_name.clone(),
                                    span: nyash_rust::ast::Span::unknown(),
                                }),
                                method: mname.clone(),
                                arguments: args,
                                span: nyash_rust::ast::Span::unknown(),
                            };
                            tests.push(TestPlan { setup: None, call });
                        } else {
                            // Instance: try new BoxName() then .test_*()
                            let inst_var = format!("__t_{}", box_name.to_lowercase());
                            // Instance override via JSON
                            let mut inst_ctor: Option<InstanceSpec> = None;
                            if let Some(m) = &args_map {
                                if let Some(v) = m.get(&format!("{}.{}", box_name, mname)) {
                                    inst_ctor = v.instance.clone();
                                }
                            }
                            let inst_init: nyash_rust::ASTNode = if let Some(spec) = inst_ctor {
                                match spec.ctor.as_str() {
                                    "new" => nyash_rust::ASTNode::New {
                                        class: box_name.clone(),
                                        arguments: spec.args,
                                        type_arguments: spec.type_args,
                                        span: nyash_rust::ast::Span::unknown(),
                                    },
                                    "birth" => nyash_rust::ASTNode::MethodCall {
                                        object: Box::new(nyash_rust::ASTNode::Variable {
                                            name: box_name.clone(),
                                            span: nyash_rust::ast::Span::unknown(),
                                        }),
                                        method: "birth".into(),
                                        arguments: spec.args,
                                        span: nyash_rust::ast::Span::unknown(),
                                    },
                                    other => {
                                        crate::macro_log!("[macro][test][args] unknown ctor '{}' for {}.{}, using new()", other, box_name, mname);
                                        nyash_rust::ASTNode::New {
                                            class: box_name.clone(),
                                            arguments: vec![],
                                            type_arguments: vec![],
                                            span: nyash_rust::ast::Span::unknown(),
                                        }
                                    }
                                }
                            } else {
                                nyash_rust::ASTNode::New {
                                    class: box_name.clone(),
                                    arguments: vec![],
                                    type_arguments: vec![],
                                    span: nyash_rust::ast::Span::unknown(),
                                }
                            };
                            let setup = nyash_rust::ASTNode::Local {
                                variables: vec![inst_var.clone()],
                                initial_values: vec![Some(Box::new(inst_init))],
                                span: nyash_rust::ast::Span::unknown(),
                            };
                            let mut args: Vec<nyash_rust::ASTNode> = Vec::new();
                            if let Some(m) = &args_map {
                                if let Some(v) = m.get(&format!("{}.{}", box_name, mname)) {
                                    args = v.args.clone();
                                }
                            }
                            if args.is_empty() && !params.is_empty() {
                                if crate::config::env::env_flag("NYASH_TEST_ARGS_DEFAULTS").unwrap_or(false) {
                                    for _ in params {
                                        args.push(nyash_rust::ASTNode::Literal {
                                            value: nyash_rust::ast::LiteralValue::Integer(0),
                                            span: nyash_rust::ast::Span::unknown(),
                                        });
                                    }
                                } else {
                                    crate::macro_log!("[macro][test][args] missing args for {}.{} (need {}), skipping", box_name, mname, params.len());
                                    continue;
                                }
                            }
                            let call = nyash_rust::ASTNode::MethodCall {
                                object: Box::new(nyash_rust::ASTNode::Variable {
                                    name: inst_var.clone(),
                                    span: nyash_rust::ast::Span::unknown(),
                                }),
                                method: mname.clone(),
                                arguments: args,
                                span: nyash_rust::ast::Span::unknown(),
                            };
                            tests.push(TestPlan {
                                setup: Some(setup),
                                call,
                            });
                        }
                    }
                }
            }
        }
    }
    if tests.is_empty() {
        return ast.clone();
    }
    if let nyash_rust::ASTNode::Program { statements, .. } = ast {
        let mut out = statements.clone();
        let mut extra: Vec<nyash_rust::ASTNode> = Vec::new();
        for t in tests {
            if let Some(setup) = t.setup {
                extra.push(setup);
            }
            extra.push(t.call);
        }
        out.extend(extra);
        return nyash_rust::ASTNode::Program {
            statements: out,
            span: nyash_rust::ast::Span::unknown(),
        };
    }
    ast.clone()
}
