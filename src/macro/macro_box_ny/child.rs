use nyash_rust::ASTNode;

pub(crate) fn derive_box_name(default: &str, name_fn: Option<&ASTNode>) -> &'static str {
    // If name() { return "X" } pattern is detected, use it; else box name
    if let Some(ASTNode::FunctionDeclaration { body, .. }) = name_fn {
        if body.len() == 1 {
            if let ASTNode::Return { value: Some(v), .. } = &body[0] {
                if let ASTNode::Literal {
                    value: nyash_rust::ast::LiteralValue::String(s),
                    ..
                } = &**v
                {
                    let owned = s.clone();
                    return Box::leak(owned.into_boxed_str());
                }
            }
        }
    }
    Box::leak(default.to_string().into_boxed_str())
}

fn expand_is_identity(body: &[ASTNode], params: &[String]) -> bool {
    if body.len() != 1 {
        return false;
    }
    if let ASTNode::Return { value: Some(v), .. } = &body[0] {
        if let ASTNode::Variable { name, .. } = &**v {
            return params.get(0).map(|p| p == name).unwrap_or(false);
        }
    }
    false
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

pub(crate) struct NyIdentityMacroBox {
    pub nm: &'static str,
}

impl super::super::macro_box::MacroBox for NyIdentityMacroBox {
    fn name(&self) -> &'static str {
        self.nm
    }
    fn expand(&self, ast: &ASTNode) -> ASTNode {
        if crate::config::env::macro_box_ny_identity_roundtrip() {
            let j = crate::r#macro::ast_json::ast_to_json(ast);
            if let Some(a2) = crate::r#macro::ast_json::json_to_ast(&j) {
                return a2;
            }
        }
        ast.clone()
    }
}

pub(crate) struct NyChildMacroBox {
    pub nm: &'static str,
    pub file: &'static str,
}

pub(crate) fn register_decl_box(
    path: &str,
    box_name: &str,
    methods: &std::collections::HashMap<String, ASTNode>,
) -> Result<(), String> {
    if let Some(ASTNode::FunctionDeclaration {
        name: mname,
        body: exp_body,
        params,
        ..
    }) = methods.get("expand")
    {
        if mname == "expand" {
            let reg_name = derive_box_name(box_name, methods.get("name"));
            let use_child = crate::config::env::macro_box_child();
            if use_child {
                let nm = reg_name;
                let file_static: &'static str = Box::leak(path.to_string().into_boxed_str());
                crate::r#macro::macro_box::register(Box::leak(Box::new(NyChildMacroBox {
                    nm,
                    file: file_static,
                })));
                crate::macro_log!(
                    "[macro][box_ny] registered child-proxy MacroBox '{}' for {}",
                    nm,
                    path
                );
            } else {
                let mut mapped = false;
                match reg_name {
                    "UppercasePrintMacro" => {
                        crate::r#macro::macro_box::register(
                            &crate::r#macro::macro_box::UppercasePrintMacro,
                        );
                        crate::macro_log!(
                            "[macro][box_ny] registered built-in '{}' from {}",
                            reg_name,
                            path
                        );
                        mapped = true;
                    }
                    _ => {}
                }
                if !mapped {
                    if expand_is_identity(exp_body, params) {
                        let nm = reg_name;
                        crate::r#macro::macro_box::register(Box::leak(Box::new(
                            NyIdentityMacroBox { nm },
                        )));
                        crate::macro_log!("[macro][box_ny] registered Ny MacroBox '{}' (identity by body) from {}", nm, path);
                    } else if expand_indicates_uppercase(exp_body, params) {
                        crate::r#macro::macro_box::register(
                            &crate::r#macro::macro_box::UppercasePrintMacro,
                        );
                        crate::macro_log!("[macro][box_ny] registered built-in 'UppercasePrintMacro' by body pattern from {}", path);
                    } else {
                        let nm = reg_name;
                        crate::r#macro::macro_box::register(Box::leak(Box::new(
                            NyIdentityMacroBox { nm },
                        )));
                        crate::macro_log!("[macro][box_ny] registered Ny MacroBox '{}' (identity: unknown body) from {}", nm, path);
                    }
                }
            }
            return Ok(());
        }
    }
    Err("no Box with static expand(ast) found".into())
}

pub(crate) fn register_top_level_static(path: &str, name: &str) {
    let nm: &'static str = Box::leak(name.to_string().into_boxed_str());
    let file_static: &'static str = Box::leak(path.to_string().into_boxed_str());
    let use_child = crate::config::env::macro_box_child();
    let allow_top = crate::config::env::macro_toplevel_allow().unwrap_or(false);
    if use_child && allow_top {
        crate::r#macro::macro_box::register(Box::leak(Box::new(NyChildMacroBox {
            nm,
            file: file_static,
        })));
        crate::macro_log!(
            "[macro][box_ny] registered child-proxy MacroBox '{}' (top-level static) for {}",
            nm,
            path
        );
    } else {
        crate::r#macro::macro_box::register(Box::leak(Box::new(NyIdentityMacroBox { nm })));
        crate::macro_log!(
            "[macro][box_ny] registered identity MacroBox '{}' (top-level static) for {}",
            nm,
            path
        );
    }
}

impl super::super::macro_box::MacroBox for NyChildMacroBox {
    fn name(&self) -> &'static str {
        self.nm
    }
    fn expand(&self, ast: &ASTNode) -> ASTNode {
        // Parent-side proxy: prefer runner script (PyVM) when enabled; otherwise fallback to internal child mode.
        let exe = match std::env::current_exe() {
            Ok(p) => p,
            Err(e) => {
                crate::macro_log!("[macro-proxy] current_exe failed: {}", e);
                return ast.clone();
            }
        };
        // Prefer Nyash runner route by default for self-hosting; legacy env can force internal child with 0.
        let use_runner = crate::config::env::macro_box_child_runner().unwrap_or(false);
        if crate::config::env::macro_box_child_runner().is_some() {
            crate::macro_log!(
                "[macro][compat] NYASH_MACRO_BOX_CHILD_RUNNER is deprecated; prefer defaults"
            );
        }
        let mut cmd = std::process::Command::new(exe.clone());
        // Build MacroCtx JSON once (caps only, MVP)
        let mctx = crate::r#macro::ctx::MacroCtx::from_env();
        let ctx_json = format!(
            "{{\"caps\":{{\"io\":{},\"net\":{},\"env\":{}}}}}",
            mctx.caps.io, mctx.caps.net, mctx.caps.env
        );
        if use_runner {
            // Synthesize a tiny runner that inlines the macro file and calls MacroBoxSpec.expand
            use std::io::Write as _;
            let tmp_dir = std::path::Path::new("tmp");
            let _ = std::fs::create_dir_all(tmp_dir);
            let ts = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis();
            let tmp_path = tmp_dir.join(format!("macro_expand_runner_{}.hako", ts));
            let mut f = match std::fs::File::create(&tmp_path) {
                Ok(x) => x,
                Err(e) => {
                    crate::macro_log!("[macro-proxy] create tmp runner failed: {}", e);
                    return ast.clone();
                }
            };
            let macro_src = std::fs::read_to_string(self.file)
                .unwrap_or_else(|_| String::from("// failed to read macro file\n"));
            let script = format!(
                "{}\n\nfunction main(args) {{\n    if args.length() == 0 {{\n        print(\"{{}}\")\n        return 0\n    }}\n    local j, r, ctx\n    j = args.get(0)\n    if args.length() > 1 {{ ctx = args.get(1) }} else {{ ctx = \"{{}}\" }}\n    r = MacroBoxSpec.expand(j, ctx)\n    print(r)\n    return 0\n}}\n",
                macro_src
            );
            if let Err(e) = f.write_all(script.as_bytes()) {
                crate::macro_log!("[macro-proxy] write tmp runner failed: {}", e);
                return ast.clone();
            }
            // Deprecated compat runner route: keep the VM-backed script path explicit until
            // the macro child runner compatibility lane is retired.
            cmd.arg("--backend").arg("vm").arg(tmp_path);
            // Append script args after '--'
            let j = crate::r#macro::ast_json::ast_to_json(ast).to_string();
            cmd.arg("--").arg(j);
            // Provide MacroCtx as JSON (runner takes it as script arg)
            cmd.arg(ctx_json.clone());
            cmd.stdin(std::process::Stdio::null());
        } else {
            // Internal child mode: --macro-expand-child <macro file> with stdin JSON
            cmd.arg("--macro-expand-child")
                .arg(self.file)
                .stdin(std::process::Stdio::piped());
            // Provide MacroCtx via env for internal child
            cmd.env("NYASH_MACRO_CTX_JSON", ctx_json.clone());
        }
        cmd.stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped());
        // Sandbox env (PoC): keep runtime deterministic and plugin-free.
        cmd.env("NYASH_DISABLE_PLUGINS", "1");
        cmd.env("NYASH_SYNTAX_SUGAR_LEVEL", "basic");
        // Mark sandbox mode explicitly for PyVM capability hooks
        cmd.env("NYASH_MACRO_SANDBOX", "1");
        // Disable macro system inside child to avoid recursive registration/expansion
        cmd.env("NYASH_MACRO_ENABLE", "0");
        cmd.env_remove("NYASH_MACRO_PATHS");
        cmd.env_remove("NYASH_MACRO_BOX_NY");
        cmd.env_remove("NYASH_MACRO_BOX_NY_PATHS");
        cmd.env_remove("NYASH_MACRO_BOX_CHILD");
        cmd.env_remove("NYASH_MACRO_BOX_CHILD_RUNNER");
        // Timeout
        let timeout_ms = crate::config::env::ny_compiler_timeout_ms();
        // Spawn
        let mut child = match cmd.spawn() {
            Ok(c) => c,
            Err(e) => {
                crate::macro_log!("[macro-proxy] spawn failed: {}", e);
                if strict_enabled() {
                    std::process::exit(2);
                }
                return ast.clone();
            }
        };
        // Write stdin only in internal child mode
        if !use_runner {
            if let Some(mut sin) = child.stdin.take() {
                let j = crate::r#macro::ast_json::ast_to_json(ast).to_string();
                use std::io::Write;
                let _ = sin.write_all(j.as_bytes());
            }
        }
        // Wait with timeout
        use std::time::{Duration, Instant};
        let start = Instant::now();
        let mut out = String::new();
        loop {
            match child.try_wait() {
                Ok(Some(_status)) => {
                    if let Some(mut so) = child.stdout.take() {
                        use std::io::Read;
                        let _ = so.read_to_string(&mut out);
                    }
                    break;
                }
                Ok(None) => {
                    if start.elapsed() >= Duration::from_millis(timeout_ms) {
                        let _ = child.kill();
                        let _ = child.wait();
                        crate::macro_log!("[macro-proxy] timeout {} ms", timeout_ms);
                        if strict_enabled() {
                            std::process::exit(124);
                        }
                        return ast.clone();
                    }
                    std::thread::sleep(Duration::from_millis(5));
                }
                Err(e) => {
                    crate::macro_log!("[macro-proxy] wait error: {}", e);
                    if strict_enabled() {
                        std::process::exit(2);
                    }
                    return ast.clone();
                }
            }
        }
        // capture stderr for diagnostics and continue
        // Capture stderr for diagnostics
        let mut err = String::new();
        if let Some(mut se) = child.stderr.take() {
            use std::io::Read;
            let _ = se.read_to_string(&mut err);
        }
        // Parse output JSON
        match serde_json::from_str::<serde_json::Value>(&out) {
            Ok(v) => match crate::r#macro::ast_json::json_to_ast(&v) {
                Some(a) => a,
                None => {
                    crate::macro_log!(
                        "[macro-proxy] child JSON did not map to AST. stderr=\n{}",
                        err
                    );
                    if strict_enabled() {
                        std::process::exit(2);
                    }
                    ast.clone()
                }
            },
            Err(e) => {
                crate::macro_log!("[macro-proxy] invalid JSON from child: {}\n-- child stderr --\n{}\n-- end stderr --", e, err);
                if strict_enabled() {
                    std::process::exit(2);
                }
                ast.clone()
            }
        }
    }
}

fn strict_enabled() -> bool {
    match crate::config::env::macro_strict() {
        Some(v) => v,
        None => true,
    }
}
