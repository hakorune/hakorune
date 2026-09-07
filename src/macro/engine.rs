use nyash_rust::ast::Span;
use nyash_rust::{
    ast::{BoxMethodGeneratedProvenanceV1, BoxMethodInventoryV1},
    ASTNode,
};
use std::time::Instant;

use super::default_derive::{build_equals_method, build_tostring_method, DefaultDeriveSelectionV1};

/// HIR Patch description (MVP placeholder)
#[derive(Clone, Debug, Default)]
pub struct HirPatch {
    // In MVP, we keep it opaque; later will host add/replace nodes
}

pub struct MacroEngine {
    max_passes: usize,
    cycle_window: usize,
    trace: bool,
    measure_ast_bytes: bool,
    default_policy: Option<super::NormalMacroPolicyV1>,
}

impl MacroEngine {
    pub fn new() -> Self {
        let max_passes = crate::config::env::macro_max_passes()
            .map(|v| v as usize)
            .unwrap_or(32);
        let cycle_window = crate::config::env::macro_cycle_window()
            .map(|v| v as usize)
            .unwrap_or(8);
        let trace = crate::config::env::macro_trace();
        let measure_ast_bytes =
            trace || crate::config::env::macro_trace_jsonl().is_some_and(|path| !path.is_empty());
        Self {
            max_passes,
            cycle_window,
            trace,
            measure_ast_bytes,
            default_policy: None,
        }
    }

    /// Expand all macros with depth/cycle guards and return patched AST.
    pub fn expand(&mut self, ast: &ASTNode) -> (ASTNode, Vec<HirPatch>) {
        let patches = Vec::new();
        let mut cur = ast.clone();
        let mut history: std::collections::VecDeque<ASTNode> = std::collections::VecDeque::new();
        for pass in 0..self.max_passes {
            let t0 = Instant::now();
            let before_len = if self.measure_ast_bytes {
                crate::r#macro::ast_json::ast_to_json(&cur)
                    .to_string()
                    .len()
            } else {
                0
            };
            let next0 = self.expand_node(&cur);
            // Apply user MacroBoxes once per pass (if enabled)
            let next = crate::r#macro::macro_box::expand_all_once(&next0);
            let after_len = if self.measure_ast_bytes {
                crate::r#macro::ast_json::ast_to_json(&next)
                    .to_string()
                    .len()
            } else {
                0
            };
            let dt = t0.elapsed();
            if self.trace {
                crate::macro_log!(
                    "[macro][engine] pass={} changed={} bytes:{}=>{} dt={:?}",
                    pass,
                    (next != cur),
                    before_len,
                    after_len,
                    dt
                );
            }
            jsonl_trace(pass, before_len, after_len, next != cur, dt);
            if next == cur {
                return (cur, patches);
            }
            // cycle detection in small window
            if history.iter().any(|h| *h == next) {
                crate::macro_log!(
                    "[macro][engine] cycle detected at pass {} — stopping expansion",
                    pass
                );
                return (cur, patches);
            }
            history.push_back(cur);
            if history.len() > self.cycle_window {
                let _ = history.pop_front();
            }
            cur = next;
        }
        crate::macro_log!(
            "[macro][engine] max passes ({}) exceeded — stopping expansion",
            self.max_passes
        );
        (cur, patches)
    }

    pub(crate) fn with_default_policy(policy: super::NormalMacroPolicyV1) -> Self {
        Self {
            default_policy: Some(policy),
            ..Self::new()
        }
    }

    fn expand_node(&mut self, node: &ASTNode) -> ASTNode {
        match node.clone() {
            ASTNode::Program { statements, span } => {
                if crate::config::env::macro_trace() {
                    crate::macro_log!("[macro][visit] Program: statements={}", statements.len());
                }
                let new_stmts = statements
                    .into_iter()
                    .map(|n| {
                        if crate::config::env::macro_trace() {
                            crate::macro_log!("[macro][visit]  child kind...",);
                        }
                        self.expand_node(&n)
                    })
                    .collect();
                ASTNode::Program {
                    statements: new_stmts,
                    span,
                }
            }
            ASTNode::BoxDeclaration {
                name,
                fields,
                field_decls,
                public_fields,
                private_fields,
                mut methods,
                constructors,
                init_fields,
                weak_fields,
                delegates,
                is_interface,
                is_record,
                transitions,
                extends,
                implements,
                type_parameters,
                is_sync,
                is_static,
                static_init,
                attrs,
                invariants,
                span,
            } => {
                if crate::config::env::macro_trace() {
                    crate::macro_log!(
                        "[macro][visit] BoxDeclaration: {} (fields={})",
                        name,
                        fields.len()
                    );
                }
                // Derive set: default Equals+ToString when macro is enabled
                let (derive_all, derive_set) = match &self.default_policy {
                    Some(policy) => { let (all, set) = policy.settings(); (all, set.to_owned()) }
                    None => (crate::config::env::macro_derive_all(), crate::config::env::macro_derive()
                        .unwrap_or_else(|| "Equals,ToString".to_string())),
                };
                if crate::config::env::macro_trace() {
                    crate::macro_log!(
                        "[macro][derive] box={} derive_all={} set={}",
                        name,
                        derive_all,
                        derive_set
                    );
                }
                // Default derives are instance methods. A static box has no
                // receiver, so it cannot own receiver-based generated methods.
                let selection = match &self.default_policy {
                    Some(policy) => policy.selection(is_static, &methods),
                    None => default_derive_selection(is_static, &methods),
                };
                let want_equals = selection.equals;
                let want_tostring = selection.to_string;
                // Philosophy-2: respect box independence — operate on public interface only
                let field_view: &Vec<String> = &public_fields;
                if want_equals && methods.get_declaration("equals").is_none() {
                    if crate::config::env::macro_trace() {
                        crate::macro_log!(
                            "[macro][derive] equals for {} (public fields: {})",
                            name,
                            field_view.len()
                        );
                    }
                    let m = build_equals_method(&name, field_view);
                    methods
                        .try_push_generated(
                            "equals",
                            m,
                            BoxMethodGeneratedProvenanceV1::MacroOrImport {
                                generator: "macro-derive-equals".into(),
                            },
                            Span::unknown(),
                        )
                        .expect("macro derive preflight must prevent duplicate equals");
                }
                if want_tostring && methods.get_declaration("toString").is_none() {
                    if crate::config::env::macro_trace() {
                        crate::macro_log!(
                            "[macro][derive] toString for {} (public fields: {})",
                            name,
                            field_view.len()
                        );
                    }
                    let m = build_tostring_method(&name, field_view);
                    methods
                        .try_push_generated(
                            "toString",
                            m,
                            BoxMethodGeneratedProvenanceV1::MacroOrImport {
                                generator: "macro-derive-to-string".into(),
                            },
                            Span::unknown(),
                        )
                        .expect("macro derive preflight must prevent duplicate toString");
                }
                ASTNode::BoxDeclaration {
                    name,
                    fields,
                    field_decls,
                    public_fields,
                    private_fields,
                    methods,
                    constructors,
                    init_fields,
                    weak_fields,
                    delegates,
                    is_interface,
                    is_record,
                    transitions,
                    extends,
                    implements,
                    type_parameters,
                    is_sync,
                    is_static,
                    static_init,
                    attrs,
                    invariants,
                    span,
                }
            }
            other => other,
        }
    }
}

fn default_derive_selection(
    is_static: bool,
    methods: &BoxMethodInventoryV1,
) -> DefaultDeriveSelectionV1 {
    let derive_all = crate::config::env::macro_derive_all();
    let derive_set =
        crate::config::env::macro_derive().unwrap_or_else(|| "Equals,ToString".to_string());
    super::default_derive::select(is_static, methods, derive_all, &derive_set)
}

fn jsonl_trace(pass: usize, before: usize, after: usize, changed: bool, dt: std::time::Duration) {
    if let Some(path) = crate::config::env::macro_trace_jsonl() {
        if path.is_empty() {
            return;
        }
        let rec = serde_json::json!({
            "event": "macro_pass",
            "pass": pass,
            "changed": changed,
            "before_bytes": before,
            "after_bytes": after,
            "dt_us": dt.as_micros() as u64,
        })
        .to_string();
        let _ = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .and_then(|mut f| {
                use std::io::Write;
                writeln!(f, "{}", rec)
            });
    }
}
