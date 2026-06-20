use rustc_driver::{run_compiler, Callbacks, Compilation};
use rustc_interface::interface;
use rustc_middle::thir::{ExprKind, StmtKind};
use rustc_middle::ty::TyCtxt;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

const CONTRACT: &str = "rustc-semir-adapter-binding-context-thir-body-inventory-v0";

#[derive(Default)]
struct ThirInventoryCallbacks {
    crate_root: PathBuf,
    bodies: Vec<BodyInventory>,
}

struct BodyInventory {
    hir_owner_reference: String,
    rustc_owner_path: String,
    module_id: String,
    source_path: String,
    source_line: usize,
    source_column: usize,
    body_type: String,
    root_expr_kind: String,
    expr_count: usize,
    stmt_count: usize,
    block_count: usize,
    expr_kind_counts: BTreeMap<String, usize>,
    stmt_kind_counts: BTreeMap<String, usize>,
    type_samples: Vec<String>,
}

impl Callbacks for ThirInventoryCallbacks {
    fn after_expansion<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        rustc_hir_analysis::check_crate(tcx);
        tcx.dcx().abort_if_errors();
        let source_map = compiler.sess.source_map();
        for def_id in tcx.hir_body_owners() {
            let owner_path = tcx.def_path_str(def_id);
            if !owner_path.starts_with("binding_context::") {
                continue;
            }
            let Ok((thir, root_expr)) = tcx.thir_body(def_id) else {
                continue;
            };
            let thir = thir.borrow();
            let source = source_loc(source_map, tcx.def_span(def_id), &self.crate_root);
            let mut expr_kind_counts = BTreeMap::new();
            let mut stmt_kind_counts = BTreeMap::new();
            let mut type_samples = Vec::new();

            for expr in thir.exprs.iter() {
                *expr_kind_counts
                    .entry(expr_kind_name(&expr.kind).to_string())
                    .or_default() += 1;
                let ty = expr.ty.to_string();
                if type_samples.len() < 8 && !type_samples.iter().any(|sample| sample == &ty) {
                    type_samples.push(ty);
                }
            }
            for stmt in thir.stmts.iter() {
                *stmt_kind_counts
                    .entry(stmt_kind_name(&stmt.kind).to_string())
                    .or_default() += 1;
            }

            self.bodies.push(BodyInventory {
                hir_owner_reference: format!("body:{}", canonical_path_from_def_path(&owner_path)),
                rustc_owner_path: owner_path.clone(),
                module_id: module_id_for_body_path(&owner_path),
                source_path: source.0,
                source_line: source.1,
                source_column: source.2,
                body_type: variant_name(&thir.body_type),
                root_expr_kind: expr_kind_name(&thir[root_expr].kind).to_string(),
                expr_count: thir.exprs.len(),
                stmt_count: thir.stmts.len(),
                block_count: thir.blocks.len(),
                expr_kind_counts,
                stmt_kind_counts,
                type_samples,
            });
        }
        self.bodies
            .sort_by(|a, b| a.hir_owner_reference.cmp(&b.hir_owner_reference));
        Compilation::Stop
    }
}

fn expr_kind_name(kind: &ExprKind<'_>) -> &'static str {
    match kind {
        ExprKind::Scope { .. } => "Scope",
        ExprKind::If { .. } => "If",
        ExprKind::Call { .. } => "Call",
        ExprKind::ByUse { .. } => "ByUse",
        ExprKind::Deref { .. } => "Deref",
        ExprKind::Binary { .. } => "Binary",
        ExprKind::LogicalOp { .. } => "LogicalOp",
        ExprKind::Unary { .. } => "Unary",
        ExprKind::Cast { .. } => "Cast",
        ExprKind::Use { .. } => "Use",
        ExprKind::NeverToAny { .. } => "NeverToAny",
        ExprKind::PointerCoercion { .. } => "PointerCoercion",
        ExprKind::Loop { .. } => "Loop",
        ExprKind::LoopMatch { .. } => "LoopMatch",
        ExprKind::Let { .. } => "Let",
        ExprKind::Match { .. } => "Match",
        ExprKind::Block { .. } => "Block",
        ExprKind::Assign { .. } => "Assign",
        ExprKind::AssignOp { .. } => "AssignOp",
        ExprKind::Field { .. } => "Field",
        ExprKind::Index { .. } => "Index",
        ExprKind::VarRef { .. } => "VarRef",
        ExprKind::UpvarRef { .. } => "UpvarRef",
        ExprKind::Borrow { .. } => "Borrow",
        ExprKind::RawBorrow { .. } => "RawBorrow",
        ExprKind::Break { .. } => "Break",
        ExprKind::Continue { .. } => "Continue",
        ExprKind::ConstContinue { .. } => "ConstContinue",
        ExprKind::Return { .. } => "Return",
        ExprKind::Become { .. } => "Become",
        ExprKind::ConstBlock { .. } => "ConstBlock",
        ExprKind::Repeat { .. } => "Repeat",
        ExprKind::Array { .. } => "Array",
        ExprKind::Tuple { .. } => "Tuple",
        ExprKind::Adt(_) => "Adt",
        ExprKind::PlaceTypeAscription { .. } => "PlaceTypeAscription",
        ExprKind::ValueTypeAscription { .. } => "ValueTypeAscription",
        ExprKind::PlaceUnwrapUnsafeBinder { .. } => "PlaceUnwrapUnsafeBinder",
        ExprKind::ValueUnwrapUnsafeBinder { .. } => "ValueUnwrapUnsafeBinder",
        ExprKind::WrapUnsafeBinder { .. } => "WrapUnsafeBinder",
        ExprKind::Closure(_) => "Closure",
        ExprKind::Literal { .. } => "Literal",
        ExprKind::NonHirLiteral { .. } => "NonHirLiteral",
        ExprKind::ZstLiteral { .. } => "ZstLiteral",
        ExprKind::NamedConst { .. } => "NamedConst",
        ExprKind::ConstParam { .. } => "ConstParam",
        ExprKind::StaticRef { .. } => "StaticRef",
        ExprKind::InlineAsm(_) => "InlineAsm",
        ExprKind::ThreadLocalRef(_) => "ThreadLocalRef",
        ExprKind::Yield { .. } => "Yield",
        ExprKind::Reborrow { .. } => "Reborrow",
    }
}

fn stmt_kind_name(kind: &StmtKind<'_>) -> &'static str {
    match kind {
        StmtKind::Expr { .. } => "Expr",
        StmtKind::Let { .. } => "Let",
    }
}

fn source_loc(
    source_map: &rustc_span::source_map::SourceMap,
    span: rustc_span::Span,
    crate_root: &Path,
) -> (String, usize, usize) {
    let pos = source_map.lookup_char_pos(span.lo());
    let raw_path = pos.file.name.prefer_local_unconditionally().to_string();
    (
        normalize_source_path(&raw_path, crate_root),
        pos.line,
        pos.col_display + 1,
    )
}

fn normalize_source_path(raw_path: &str, crate_root: &Path) -> String {
    let path = Path::new(raw_path);
    if path.is_absolute() {
        if let Ok(stripped) = path.strip_prefix(crate_root.parent().unwrap_or(crate_root)) {
            return stripped.to_string_lossy().replace('\\', "/");
        }
        return path
            .file_name()
            .map(|name| name.to_string_lossy().replace('\\', "/"))
            .unwrap_or_else(|| "external.rs".to_string());
    }
    raw_path.replace('\\', "/")
}

fn canonical_path_from_def_path(def_path: &str) -> String {
    if def_path.is_empty() {
        "crate".to_string()
    } else {
        format!("crate::{def_path}")
    }
}

fn module_id_for_body_path(def_path: &str) -> String {
    let mut segments: Vec<&str> = def_path.split("::").filter(|s| !s.is_empty()).collect();
    while matches!(segments.last(), Some(segment) if segment.starts_with("{impl#")) {
        segments.pop();
    }
    if segments.len() >= 2
        && segments[segments.len() - 2]
            .chars()
            .next()
            .is_some_and(char::is_uppercase)
    {
        segments.pop();
        segments.pop();
    } else if matches!(segments.last(), Some(segment) if !segment.starts_with("{impl#")) {
        segments.pop();
    }
    if segments.is_empty() {
        "crate".to_string()
    } else {
        format!("crate::{}", segments.join("::"))
    }
}

fn variant_name<T: std::fmt::Debug>(value: &T) -> String {
    let debug = format!("{value:?}");
    debug
        .split(|ch: char| ch == ' ' || ch == '{' || ch == '(')
        .next()
        .unwrap_or("Unknown")
        .to_string()
}

fn render_json(bodies: &[BodyInventory]) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    field(&mut out, 1, "output_contract", CONTRACT, true);
    out.push_str("  \"schema_version\": 0,\n");
    field(
        &mut out,
        1,
        "kind",
        "RustcSemirBindingContextThirBodyInventory",
        true,
    );
    out.push_str("  \"family\": \"BindingContext\",\n");
    out.push_str("  \"bodies\": [\n");
    for (index, body) in bodies.iter().enumerate() {
        render_body(&mut out, body);
        if index + 1 != bodies.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ],\n");
    out.push_str("  \"coverage\": {\n");
    out.push_str(&format!(
        "    \"selected_definition_count\": {},\n",
        bodies.len()
    ));
    out.push_str("    \"hir_owner_reference_used\": 1,\n");
    out.push_str("    \"binding_context_family_selected\": 1\n");
    out.push_str("  },\n");
    out.push_str("  \"claims\": {\n");
    out.push_str("    \"MIR_or_borrowck_extracted\": 0,\n");
    out.push_str("    \"drop_elaboration_extracted\": 0,\n");
    out.push_str("    \"RustLifecycleAdapterFacts_generated\": 0,\n");
    out.push_str("    \"hako_plan_emitted\": 0,\n");
    out.push_str("    \"hako_source_emitted\": 0,\n");
    out.push_str("    \"backend_behavior_changed\": 0\n");
    out.push_str("  }\n");
    out.push_str("}\n");
    out
}

fn render_body(out: &mut String, body: &BodyInventory) {
    out.push_str("    {\n");
    field(
        out,
        3,
        "hir_owner_reference",
        &body.hir_owner_reference,
        true,
    );
    field(out, 3, "rustc_owner_path", &body.rustc_owner_path, true);
    field(out, 3, "module_id", &body.module_id, true);
    out.push_str("      \"source\": {\n");
    field(out, 4, "path", &body.source_path, true);
    out.push_str(&format!(
        "        \"start\": {{\"line\": {}, \"column\": {}}}\n",
        body.source_line, body.source_column
    ));
    out.push_str("      },\n");
    field(out, 3, "body_type", &body.body_type, true);
    field(out, 3, "root_expr_kind", &body.root_expr_kind, true);
    out.push_str(&format!("      \"expr_count\": {},\n", body.expr_count));
    out.push_str(&format!("      \"stmt_count\": {},\n", body.stmt_count));
    out.push_str(&format!("      \"block_count\": {},\n", body.block_count));
    counts(out, 3, "expr_kind_counts", &body.expr_kind_counts, true);
    counts(out, 3, "stmt_kind_counts", &body.stmt_kind_counts, true);
    array(out, 3, "type_samples", &body.type_samples, false);
    out.push_str("    }");
}

fn field(out: &mut String, indent: usize, key: &str, value: &str, comma: bool) {
    out.push_str(&format!(
        "{}\"{}\": \"{}\"{}",
        "  ".repeat(indent),
        escape(key),
        escape(value),
        if comma { "," } else { "" }
    ));
    out.push('\n');
}

fn counts(
    out: &mut String,
    indent: usize,
    key: &str,
    values: &BTreeMap<String, usize>,
    comma: bool,
) {
    out.push_str(&format!("{}\"{}\": {{", "  ".repeat(indent), escape(key)));
    if !values.is_empty() {
        out.push('\n');
    }
    for (index, (name, count)) in values.iter().enumerate() {
        out.push_str(&format!(
            "{}\"{}\": {}{}",
            "  ".repeat(indent + 1),
            escape(name),
            count,
            if index + 1 != values.len() { "," } else { "" }
        ));
        out.push('\n');
    }
    out.push_str(&format!(
        "{}}}{}",
        "  ".repeat(indent),
        if comma { "," } else { "" }
    ));
    out.push('\n');
}

fn array(out: &mut String, indent: usize, key: &str, values: &[String], comma: bool) {
    out.push_str(&format!("{}\"{}\": [", "  ".repeat(indent), escape(key)));
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("\"{}\"", escape(value)));
    }
    out.push(']');
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn escape(input: &str) -> String {
    let mut escaped = String::new();
    for ch in input.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if ch.is_control() => escaped.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => escaped.push(ch),
        }
    }
    escaped
}

pub fn run_binding_context(inputs: &[String]) {
    if inputs.is_empty() {
        eprintln!(
            "usage: rustc-semir-adapter --binding-context-thir-body-inventory <rust-source> [rustc-arg...]"
        );
        std::process::exit(2);
    }
    let input_path = PathBuf::from(&inputs[0]);
    let crate_root = input_path.canonicalize().unwrap_or(input_path.clone());
    let mut callbacks = ThirInventoryCallbacks {
        crate_root,
        bodies: Vec::new(),
    };
    let mut rustc_args = vec![
        "rustc-semir-adapter".to_string(),
        "--crate-type=lib".to_string(),
        "--edition=2021".to_string(),
        inputs[0].clone(),
    ];
    rustc_args.extend(inputs.iter().skip(1).cloned());
    run_compiler(&rustc_args, &mut callbacks);

    if callbacks.bodies.is_empty() {
        eprintln!("rustc-semir-adapter: no BindingContext THIR bodies selected");
        std::process::exit(1);
    }
    print!("{}", render_json(&callbacks.bodies));
}
