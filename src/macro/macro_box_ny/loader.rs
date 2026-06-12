use super::analysis::caps_allow_macro_source;
use super::child::{register_decl_box, register_top_level_static};
use nyash_rust::ASTNode;

/// Load MacroBoxes written in Nyash.
/// Preferred env: NYASH_MACRO_PATHS=comma,separated,paths
/// Backward compat: NYASH_MACRO_BOX_NY=1 + NYASH_MACRO_BOX_NY_PATHS
pub fn init_from_env() {
    let paths = crate::config::env::macro_paths()
        .filter(|s| !s.trim().is_empty())
        .or_else(|| {
            if crate::config::env::macro_box_ny() {
                if let Some(s) = crate::config::env::macro_box_ny_paths() {
                    if !s.trim().is_empty() {
                        crate::macro_log!("[macro][compat] NYASH_MACRO_BOX_NY*_ vars are deprecated; use NYASH_MACRO_PATHS");
                        return Some(s);
                    }
                }
            }
            None
        });

    if crate::config::env::macro_toplevel_allow().is_some() {
        crate::macro_log!("[macro][compat] NYASH_MACRO_TOPLEVEL_ALLOW is deprecated; default is OFF. Prefer CLI --macro-top-level-allow if needed");
    }
    if crate::config::env::macro_box_child_runner().is_some() {
        crate::macro_log!("[macro][compat] NYASH_MACRO_BOX_CHILD_RUNNER is deprecated; runner mode is managed automatically");
    }

    let Some(paths) = paths else {
        return;
    };
    for p in paths.split(',').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        if let Err(e) = try_load_one(p) {
            let noisy =
                crate::config::env::macro_trace() || crate::config::env::macro_cli_verbose();
            if noisy {
                crate::macro_log!("[macro][box_ny] failed to load '{}': {}", p, e);
            }
        }
    }
}

fn try_load_one(path: &str) -> Result<(), String> {
    let src = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let prev_sugar = crate::config::env::macro_syntax_sugar_level();
    std::env::set_var("NYASH_SYNTAX_SUGAR_LEVEL", "basic");
    let ast_res = nyash_rust::parser::NyashParser::parse_from_string(&src);
    if let Some(v) = prev_sugar {
        std::env::set_var("NYASH_SYNTAX_SUGAR_LEVEL", v);
    } else {
        std::env::remove_var("NYASH_SYNTAX_SUGAR_LEVEL");
    }
    let ast = ast_res.map_err(|e| format!("parse error: {:?}", e))?;
    if let ASTNode::Program { statements, .. } = ast {
        if let Err(msg) = caps_allow_macro_source(&ASTNode::Program {
            statements: statements.clone(),
            span: nyash_rust::ast::Span::unknown(),
        }) {
            crate::macro_log!("[macro][box_ny][caps] {} (in '{}')", msg, path);
            if strict_enabled() {
                return Err(msg);
            }
            return Ok(());
        }
        for st in &statements {
            if let ASTNode::BoxDeclaration {
                name: box_name,
                methods,
                ..
            } = st
            {
                if let Some(ASTNode::FunctionDeclaration { name: mname, .. }) =
                    methods.get("expand")
                {
                    if mname == "expand" {
                        let _ = mname; // keep the shape explicit for readability
                        return register_decl_box(path, box_name.as_str(), methods);
                    }
                }
            }
        }
        for st in &statements {
            if let ASTNode::FunctionDeclaration {
                is_static: true,
                name,
                ..
            } = st
            {
                if let Some((box_name, method)) = name.split_once('.') {
                    if method == "expand" {
                        register_top_level_static(path, box_name);
                        return Ok(());
                    }
                }
            }
        }
    }
    Err("no Box with static expand(ast) found".into())
}

fn strict_enabled() -> bool {
    match crate::config::env::macro_strict() {
        Some(v) => v,
        None => true,
    }
}
