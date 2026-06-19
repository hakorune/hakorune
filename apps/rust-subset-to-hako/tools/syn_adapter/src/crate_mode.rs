use std::path::{Path, PathBuf};

use serde_json::json;
use syn::Item;

use crate::cli::fail;
use crate::items::file_to_json_for_crate;

struct ModuleArtifact {
    module: String,
    source_path: String,
    artifact_path: String,
    file: syn::File,
}

pub(crate) fn write_crate_bundle(
    crate_root: &Path,
    out_dir: &Path,
    crate_name: &str,
    target_kind: &str,
    target_name: &str,
) {
    if target_kind != "lib" && target_kind != "bin" {
        fail(format!(
            "unsupported target kind for crate manifest v0: {target_kind}"
        ));
    }

    let mut modules = discover_modules(crate_root);
    assign_artifact_paths(&mut modules);

    let modules_dir = out_dir.join("modules");
    std::fs::create_dir_all(&modules_dir).unwrap_or_else(|err| {
        fail(format!(
            "failed to create module output dir {}: {err}",
            modules_dir.display()
        ))
    });

    for module in &modules {
        let artifact = file_to_json_for_crate(&module.file, module.module.clone());
        let text = serde_json::to_string_pretty(&artifact)
            .unwrap_or_else(|err| fail(format!("failed to serialize module JSON: {err}")));
        let path = out_dir.join(&module.artifact_path);
        std::fs::write(&path, format!("{text}\n"))
            .unwrap_or_else(|err| fail(format!("failed to write {}: {err}", path.display())));
    }

    let manifest_modules = modules
        .iter()
        .map(|module| {
            json!({
                "module": module.module,
                "source_path": module.source_path,
                "artifact_path": module.artifact_path,
            })
        })
        .collect::<Vec<_>>();

    let manifest = json!({
        "schema_version": 0,
        "kind": "RustSubsetCrateManifest",
        "crate_name": crate_name,
        "target": {
            "kind": target_kind,
            "name": target_name,
        },
        "root_module": "crate",
        "modules": manifest_modules,
    });
    let text = serde_json::to_string_pretty(&manifest)
        .unwrap_or_else(|err| fail(format!("failed to serialize crate manifest: {err}")));
    let manifest_path = out_dir.join("crate-manifest.json");
    std::fs::write(&manifest_path, format!("{text}\n")).unwrap_or_else(|err| {
        fail(format!(
            "failed to write {}: {err}",
            manifest_path.display()
        ))
    });
}

fn discover_modules(crate_root: &Path) -> Vec<ModuleArtifact> {
    let root_source = PathBuf::from("src/lib.rs");
    let root_file = parse_module(crate_root, &root_source);
    let mut child_sources = external_mod_sources(&root_file);
    child_sources.sort_by(|left, right| left.0.cmp(&right.0));

    let mut modules = vec![ModuleArtifact {
        module: "crate".to_string(),
        source_path: path_to_manifest_string(&root_source),
        artifact_path: String::new(),
        file: root_file,
    }];

    for (name, source_path) in child_sources {
        modules.push(ModuleArtifact {
            module: format!("crate::{name}"),
            source_path: path_to_manifest_string(&source_path),
            artifact_path: String::new(),
            file: parse_module(crate_root, &source_path),
        });
    }

    modules
}

fn external_mod_sources(file: &syn::File) -> Vec<(String, PathBuf)> {
    let mut sources = Vec::new();
    for item in &file.items {
        let Item::Mod(module) = item else {
            continue;
        };
        if module.content.is_some() {
            fail(format!(
                "inline module is out of crate manifest v0 scope: {}",
                module.ident
            ));
        }
        let name = module.ident.to_string();
        sources.push((name.clone(), PathBuf::from(format!("src/{name}.rs"))));
    }
    sources
}

fn parse_module(crate_root: &Path, source_path: &Path) -> syn::File {
    let path = crate_root.join(source_path);
    let source = std::fs::read_to_string(&path)
        .unwrap_or_else(|err| fail(format!("failed to read {}: {err}", path.display())));
    syn::parse_file(&source)
        .unwrap_or_else(|err| fail(format!("failed to parse {}: {err}", path.display())))
}

fn assign_artifact_paths(modules: &mut [ModuleArtifact]) {
    for (index, module) in modules.iter_mut().enumerate() {
        module.artifact_path = format!("modules/{index:04}.json");
    }
}

fn path_to_manifest_string(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}
