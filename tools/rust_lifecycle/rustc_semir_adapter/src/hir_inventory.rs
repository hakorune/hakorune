use rustc_driver::{run_compiler, Callbacks, Compilation};
use rustc_hir::ItemKind;
use rustc_interface::interface;
use rustc_middle::ty::{self, TyCtxt};
use rustc_span::def_id::LocalDefId;
use std::path::{Path, PathBuf};

const CONTRACT: &str = "rustc-semir-adapter-hir-item-provenance-inventory-v0";
const JSON_CONTRACT: &str = "rustc-semir-adapter-hir-inventory-contract-v0";

#[derive(Default)]
struct HirInventoryCallbacks {
    output: Vec<String>,
}

impl Callbacks for HirInventoryCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let source_map = compiler.sess.source_map();
        self.output.push(format!("output_contract={CONTRACT}"));
        self.output
            .push("hir_item_provenance_inventory_green=1".to_string());
        self.output.push(format!(
            "crate_name={}",
            tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE)
        ));
        self.output.push("crate_identity_reported=1".to_string());
        self.output.push("module_identity_reported=1".to_string());
        self.output.push("module_0_path=crate".to_string());

        let mut item_count = 0usize;
        for item_id in tcx.hir_free_items() {
            let item = tcx.hir_item(item_id);
            let def_id = item.owner_id.def_id;
            let span = source_map.lookup_char_pos(item.span.lo());
            item_count += 1;
            self.output.push(format!(
                "item_{item_count}_path={}",
                tcx.def_path_str(def_id)
            ));
            self.output.push(format!(
                "item_{item_count}_kind={}",
                item_kind_name(&item.kind)
            ));
            self.output.push(format!(
                "item_{item_count}_source={}:{}:{}",
                span.file.name.prefer_local_unconditionally(),
                span.line,
                span.col_display + 1
            ));
        }

        self.output.push(format!("item_count={item_count}"));
        self.output.push("item_identity_reported=1".to_string());
        self.output.push("source_provenance_reported=1".to_string());
        self.output
            .push("RustLifecycleAdapterFacts_generated=0".to_string());
        self.output.push("hako_plan_emitted=0".to_string());
        self.output.push("hako_source_emitted=0".to_string());
        self.output.push("backend_behavior_changed=0".to_string());
        self.output.push("summary=ok".to_string());
        Compilation::Stop
    }
}

fn item_kind_name(kind: &ItemKind<'_>) -> &'static str {
    match kind {
        ItemKind::ExternCrate(..) => "ExternCrate",
        ItemKind::Use(..) => "Use",
        ItemKind::Static(..) => "Static",
        ItemKind::Const(..) => "Const",
        ItemKind::Fn { .. } => "Fn",
        ItemKind::Macro(..) => "Macro",
        ItemKind::Mod(..) => "Mod",
        ItemKind::ForeignMod { .. } => "ForeignMod",
        ItemKind::GlobalAsm { .. } => "GlobalAsm",
        ItemKind::TyAlias(..) => "TyAlias",
        ItemKind::Enum(..) => "Enum",
        ItemKind::Struct(..) => "Struct",
        ItemKind::Union(..) => "Union",
        ItemKind::Trait { .. } => "Trait",
        ItemKind::TraitAlias(..) => "TraitAlias",
        ItemKind::Impl { .. } => "Impl",
    }
}

#[derive(Clone)]
struct SourceLoc {
    path_kind: &'static str,
    path: String,
    line: usize,
    column: usize,
}

#[derive(Clone)]
struct ModuleRecord {
    module_id: String,
    parent_module_id: Option<String>,
    name: String,
    source: SourceLoc,
}

#[derive(Clone)]
struct DefinitionRecord {
    inventory_id: String,
    semantic_id: Option<String>,
    identity_kind: &'static str,
    definition_kind: &'static str,
    kind: &'static str,
    namespace: Option<&'static str>,
    module_id: String,
    name: Option<String>,
    path_segments: Vec<String>,
    visibility_kind: String,
    visibility_scope_module_id: Option<String>,
    source: SourceLoc,
}

#[derive(Default)]
struct HirContractCallbacks {
    crate_root: PathBuf,
    output: Option<String>,
}

impl Callbacks for HirContractCallbacks {
    fn after_analysis<'tcx>(
        &mut self,
        compiler: &interface::Compiler,
        tcx: TyCtxt<'tcx>,
    ) -> Compilation {
        let source_map = compiler.sess.source_map();
        let crate_name = tcx.crate_name(rustc_hir::def_id::LOCAL_CRATE).to_string();
        let mut modules = vec![ModuleRecord {
            module_id: "crate".to_string(),
            parent_module_id: None,
            name: "crate".to_string(),
            source: SourceLoc {
                path_kind: "crate_relative",
                path: root_source_path(&self.crate_root),
                line: 1,
                column: 1,
            },
        }];
        let mut definitions = Vec::new();
        let mut anonymous_owner_count = 0usize;

        for item_id in tcx.hir_free_items() {
            let item = tcx.hir_item(item_id);
            let def_id = item.owner_id.def_id;
            let def_path = tcx.def_path_str(def_id);
            let path_segments = path_segments_from_def_path(&def_path);
            let source = source_loc(source_map, item.span, &self.crate_root);

            if matches!(item.kind, ItemKind::Mod(..)) {
                let module_id = module_id_from_item_path(&def_path);
                modules.push(ModuleRecord {
                    parent_module_id: parent_module_id(&module_id),
                    name: path_segments.last().cloned().unwrap_or(module_id.clone()),
                    module_id,
                    source: source.clone(),
                });
                continue;
            }

            let kind = item_kind_name(&item.kind);
            let namespace = namespace_for_item(&item.kind);
            let (semantic_id, identity_kind, name) = if let Some(namespace) = namespace {
                (
                    Some(format!(
                        "{namespace}:{}",
                        canonical_path_from_def_path(&def_path)
                    )),
                    "named_path",
                    path_segments.last().cloned(),
                )
            } else {
                anonymous_owner_count += 1;
                (None, "anonymous_owner", path_segments.last().cloned())
            };
            let visibility = visibility_record(tcx, def_id);
            definitions.push(DefinitionRecord {
                inventory_id: format!("definition-{:06}", definitions.len() + 1),
                semantic_id,
                identity_kind,
                definition_kind: "item",
                kind,
                namespace,
                module_id: module_id_for_definition_path(&def_path),
                name,
                path_segments,
                visibility_kind: visibility.0,
                visibility_scope_module_id: visibility.1,
                source,
            });
        }

        modules.sort_by(|a, b| {
            if a.module_id == "crate" {
                std::cmp::Ordering::Less
            } else if b.module_id == "crate" {
                std::cmp::Ordering::Greater
            } else {
                a.module_id.cmp(&b.module_id)
            }
        });
        modules.dedup_by(|a, b| a.module_id == b.module_id);
        definitions.sort_by(|a, b| {
            (
                &a.source.path,
                a.source.line,
                a.source.column,
                a.semantic_id.as_deref().unwrap_or(""),
            )
                .cmp(&(
                    &b.source.path,
                    b.source.line,
                    b.source.column,
                    b.semantic_id.as_deref().unwrap_or(""),
                ))
        });
        for (index, definition) in definitions.iter_mut().enumerate() {
            definition.inventory_id = format!("definition-{:06}", index + 1);
        }

        self.output = Some(render_json_contract(
            &crate_name,
            &modules,
            &definitions,
            anonymous_owner_count,
        ));
        Compilation::Stop
    }
}

fn namespace_for_item(kind: &ItemKind<'_>) -> Option<&'static str> {
    match kind {
        ItemKind::Static(..) | ItemKind::Const(..) | ItemKind::Fn { .. } => Some("value"),
        ItemKind::Macro(..) => Some("macro"),
        ItemKind::TyAlias(..)
        | ItemKind::Enum(..)
        | ItemKind::Struct(..)
        | ItemKind::Union(..)
        | ItemKind::Trait { .. }
        | ItemKind::TraitAlias(..) => Some("type"),
        ItemKind::Impl { .. } => None,
        ItemKind::ExternCrate(..)
        | ItemKind::Use(..)
        | ItemKind::Mod(..)
        | ItemKind::ForeignMod { .. }
        | ItemKind::GlobalAsm { .. } => None,
    }
}

fn visibility_record(tcx: TyCtxt<'_>, def_id: LocalDefId) -> (String, Option<String>) {
    match tcx.local_visibility(def_id) {
        ty::Visibility::Public => ("public".to_string(), None),
        ty::Visibility::Restricted(scope) => {
            if scope.is_top_level_module() {
                ("crate".to_string(), Some("crate".to_string()))
            } else if scope == tcx.parent_module_from_def_id(def_id).to_local_def_id() {
                (
                    "private".to_string(),
                    Some(module_id_from_local_def_id(tcx, scope)),
                )
            } else {
                (
                    "restricted".to_string(),
                    Some(module_id_from_local_def_id(tcx, scope)),
                )
            }
        }
    }
}

fn module_id_from_local_def_id(tcx: TyCtxt<'_>, def_id: LocalDefId) -> String {
    let path = tcx.def_path_str(def_id);
    if path.is_empty() {
        "crate".to_string()
    } else {
        canonical_path_from_def_path(&path)
    }
}

fn source_loc(
    source_map: &rustc_span::source_map::SourceMap,
    span: rustc_span::Span,
    crate_root: &Path,
) -> SourceLoc {
    let pos = source_map.lookup_char_pos(span.lo());
    let raw_path = pos.file.name.prefer_local_unconditionally().to_string();
    let (path_kind, path) = normalize_source_path(&raw_path, crate_root);
    SourceLoc {
        path_kind,
        path,
        line: pos.line,
        column: pos.col_display + 1,
    }
}

fn root_source_path(crate_root: &Path) -> String {
    crate_root
        .file_name()
        .map(|name| name.to_string_lossy().replace('\\', "/"))
        .unwrap_or_else(|| "src/lib.rs".to_string())
}

fn normalize_source_path(raw_path: &str, crate_root: &Path) -> (&'static str, String) {
    let path = Path::new(raw_path);
    if path.is_absolute() {
        if let Ok(stripped) = path.strip_prefix(crate_root.parent().unwrap_or(crate_root)) {
            return (
                "crate_relative",
                stripped.to_string_lossy().replace('\\', "/"),
            );
        }
        return (
            "external",
            path.file_name()
                .map(|name| name.to_string_lossy().replace('\\', "/"))
                .unwrap_or_else(|| "external.rs".to_string()),
        );
    }
    ("crate_relative", raw_path.replace('\\', "/"))
}

fn canonical_path_from_def_path(def_path: &str) -> String {
    if def_path.is_empty() {
        "crate".to_string()
    } else {
        format!("crate::{def_path}")
    }
}

fn path_segments_from_def_path(def_path: &str) -> Vec<String> {
    let mut segments = vec!["crate".to_string()];
    if !def_path.is_empty() {
        segments.extend(def_path.split("::").map(str::to_string));
    }
    segments
}

fn module_id_from_item_path(def_path: &str) -> String {
    canonical_path_from_def_path(def_path)
}

fn module_id_for_definition_path(def_path: &str) -> String {
    let mut segments: Vec<&str> = def_path.split("::").filter(|s| !s.is_empty()).collect();
    if segments.len() <= 1 {
        return "crate".to_string();
    }
    segments.pop();
    format!("crate::{}", segments.join("::"))
}

fn parent_module_id(module_id: &str) -> Option<String> {
    module_id
        .rsplit_once("::")
        .map(|(parent, _)| parent.to_string())
}

fn render_json_contract(
    crate_name: &str,
    modules: &[ModuleRecord],
    definitions: &[DefinitionRecord],
    anonymous_owner_count: usize,
) -> String {
    let mut out = String::new();
    out.push_str("{\n");
    json_field(&mut out, 1, "output_contract", Some(JSON_CONTRACT), true);
    out.push_str("  \"schema_version\": 0,\n");
    json_field(&mut out, 1, "kind", Some("RustcSemirHirInventory"), true);
    json_field(
        &mut out,
        1,
        "id_policy",
        Some("canonical-rust-path-v0"),
        true,
    );
    json_field(
        &mut out,
        1,
        "ordering_policy",
        Some("module-id-and-source-order-v0"),
        true,
    );
    out.push_str("  \"crate\": {\n");
    json_field(&mut out, 2, "name", Some(crate_name), true);
    json_field(&mut out, 2, "edition", Some("2021"), true);
    json_field(&mut out, 2, "root_module_id", Some("crate"), true);
    json_field(
        &mut out,
        2,
        "root_source_path",
        modules.first().map(|m| m.source.path.as_str()),
        false,
    );
    out.push_str("  },\n");
    render_modules(&mut out, modules);
    render_definitions(&mut out, definitions);
    out.push_str("  \"coverage\": {\n");
    out.push_str(&format!("    \"module_count\": {},\n", modules.len()));
    out.push_str(&format!(
        "    \"definition_count\": {},\n",
        definitions.len()
    ));
    out.push_str("    \"semantic_id_missing_count\": 0,\n");
    out.push_str(&format!(
        "    \"anonymous_owner_count\": {},\n",
        anonymous_owner_count
    ));
    out.push_str("    \"absolute_source_paths\": 0\n");
    out.push_str("  },\n");
    out.push_str("  \"claims\": {\n");
    out.push_str("    \"THIR_extracted\": 0,\n");
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

fn render_modules(out: &mut String, modules: &[ModuleRecord]) {
    out.push_str("  \"modules\": [\n");
    for (index, module) in modules.iter().enumerate() {
        out.push_str("    {\n");
        json_field(out, 3, "module_id", Some(&module.module_id), true);
        json_optional_field(
            out,
            3,
            "parent_module_id",
            module.parent_module_id.as_deref(),
            true,
        );
        json_field(out, 3, "name", Some(&module.name), true);
        json_array_field(
            out,
            3,
            "path_segments",
            &path_segments_from_module_id(&module.module_id),
            true,
        );
        render_source(out, 3, &module.source, false);
        out.push_str("    }");
        if index + 1 != modules.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ],\n");
}

fn render_definitions(out: &mut String, definitions: &[DefinitionRecord]) {
    out.push_str("  \"definitions\": [\n");
    for (index, definition) in definitions.iter().enumerate() {
        out.push_str("    {\n");
        json_field(out, 3, "inventory_id", Some(&definition.inventory_id), true);
        json_optional_field(
            out,
            3,
            "semantic_id",
            definition.semantic_id.as_deref(),
            true,
        );
        json_field(
            out,
            3,
            "identity_kind",
            Some(definition.identity_kind),
            true,
        );
        json_field(
            out,
            3,
            "definition_kind",
            Some(definition.definition_kind),
            true,
        );
        json_field(out, 3, "kind", Some(definition.kind), true);
        json_optional_field(out, 3, "namespace", definition.namespace, true);
        json_field(out, 3, "module_id", Some(&definition.module_id), true);
        json_optional_field(out, 3, "name", definition.name.as_deref(), true);
        json_array_field(out, 3, "path_segments", &definition.path_segments, true);
        out.push_str("      \"declared_visibility\": {\n");
        json_field(out, 4, "kind", Some(&definition.visibility_kind), true);
        json_optional_field(
            out,
            4,
            "scope_module_id",
            definition.visibility_scope_module_id.as_deref(),
            false,
        );
        out.push_str("      },\n");
        render_source(out, 3, &definition.source, false);
        out.push_str("    }");
        if index + 1 != definitions.len() {
            out.push(',');
        }
        out.push('\n');
    }
    out.push_str("  ],\n");
}

fn render_source(out: &mut String, indent: usize, source: &SourceLoc, trailing_comma: bool) {
    let pad = "  ".repeat(indent);
    out.push_str(&format!("{pad}\"source\": {{\n"));
    json_field(out, indent + 1, "path_kind", Some(source.path_kind), true);
    json_field(out, indent + 1, "path", Some(&source.path), true);
    out.push_str(&format!(
        "{}\"start\": {{\"line\": {}, \"column\": {}}},\n",
        "  ".repeat(indent + 1),
        source.line,
        source.column
    ));
    out.push_str(&format!(
        "{}\"end\": {{\"line\": {}, \"column\": {}}}\n",
        "  ".repeat(indent + 1),
        source.line,
        source.column
    ));
    out.push_str(&format!("{pad}}}"));
    if trailing_comma {
        out.push(',');
    }
    out.push('\n');
}

fn path_segments_from_module_id(module_id: &str) -> Vec<String> {
    module_id.split("::").map(str::to_string).collect()
}

fn json_field(out: &mut String, indent: usize, key: &str, value: Option<&str>, comma: bool) {
    json_optional_field(out, indent, key, value, comma);
}

fn json_optional_field(
    out: &mut String,
    indent: usize,
    key: &str,
    value: Option<&str>,
    comma: bool,
) {
    let pad = "  ".repeat(indent);
    out.push_str(&format!("{pad}\"{}\": ", escape_json(key)));
    match value {
        Some(value) => out.push_str(&format!("\"{}\"", escape_json(value))),
        None => out.push_str("null"),
    }
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn json_array_field(out: &mut String, indent: usize, key: &str, values: &[String], comma: bool) {
    let pad = "  ".repeat(indent);
    out.push_str(&format!("{pad}\"{}\": [", escape_json(key)));
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("\"{}\"", escape_json(value)));
    }
    out.push(']');
    if comma {
        out.push(',');
    }
    out.push('\n');
}

fn escape_json(input: &str) -> String {
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

pub fn run_key_value(inputs: &[String]) {
    if inputs.len() != 1 {
        eprintln!("usage: rustc-semir-adapter --hir-item-provenance-inventory <rust-source>");
        std::process::exit(2);
    }

    let mut callbacks = HirInventoryCallbacks::default();
    let rustc_args = vec![
        "rustc-semir-adapter".to_string(),
        "--crate-type=lib".to_string(),
        "--edition=2021".to_string(),
        inputs[0].clone(),
    ];
    run_compiler(&rustc_args, &mut callbacks);

    for line in callbacks.output {
        println!("{line}");
    }
}

pub fn run_contract(inputs: &[String]) {
    if inputs.is_empty() {
        eprintln!(
            "usage: rustc-semir-adapter --hir-inventory-contract <rust-source> [rustc-arg...]"
        );
        std::process::exit(2);
    }

    let input_path = PathBuf::from(&inputs[0]);
    let crate_root = input_path.canonicalize().unwrap_or(input_path.clone());
    let mut callbacks = HirContractCallbacks {
        crate_root,
        output: None,
    };
    let mut rustc_args = vec![
        "rustc-semir-adapter".to_string(),
        "--crate-type=lib".to_string(),
        "--edition=2021".to_string(),
        inputs[0].clone(),
    ];
    rustc_args.extend(inputs.iter().skip(1).cloned());
    run_compiler(&rustc_args, &mut callbacks);

    if let Some(output) = callbacks.output {
        print!("{output}");
    } else {
        eprintln!("rustc-semir-adapter: HIR inventory contract was not produced");
        std::process::exit(1);
    }
}
