use super::diagnostic_output::finish_result;
use super::CliConfig;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA_VERSION: &str = "hakorune-provider-package-v1";
const ABI_VERSION: &str = "hakorune-provider-abi-v1";
const DESCRIPTOR_EXPORT: &str = "hakorune_provider_descriptor_v1";
const OUTPUT_CONTRACT: &str = "hakorune-provider-package-hako-derived-build-v0";
const BUILD_MODE: &str = "hako-derived-selected-fixture";
const PACKAGE_MODE: &str = "hako-derived-provider-package";
const OBJECT_LIFECYCLE_MODE: &str = "object-lifecycle-small-alloc-release-v0";

pub fn maybe_run_provider_package_hako_derived_build(config: &CliConfig) -> Option<i32> {
    config
        .provider_package_hako_derived_build_fixture
        .as_ref()?;
    finish_result(run_provider_package_hako_derived_build(config))
}

fn run_provider_package_hako_derived_build(config: &CliConfig) -> Result<(String, i32), String> {
    let hako_source = required_path(
        config
            .provider_package_hako_derived_build_fixture
            .as_deref(),
        "provider-package-hako-derived-build-fixture",
    )?;
    require_hako_source(&hako_source)?;

    let out_dir = required_path(
        config.provider_package_out_dir.as_deref(),
        "provider-package-out-dir",
    )?;
    let package_id = required_string(config.provider_package_id.as_deref(), "provider-package-id")?;
    let provider_name = required_string(
        config.provider_package_name.as_deref(),
        "provider-package-name",
    )?;
    let target_triple = required_string(
        config.provider_package_target_triple.as_deref(),
        "provider-package-target-triple",
    )?;
    let platform = required_string(
        config.provider_package_platform.as_deref(),
        "provider-package-platform",
    )?;

    let provider_kind = config
        .provider_package_kind
        .as_deref()
        .unwrap_or("allocator");
    let provider_version = config
        .provider_package_version
        .as_deref()
        .unwrap_or("0.1.0");
    let profile = config
        .provider_package_profile
        .as_deref()
        .unwrap_or("speed");
    if profile != "speed" && profile != "diagnostic" {
        return Err(
            "[provider-package-hako-build/invalid-profile] expected speed|diagnostic".to_string(),
        );
    }
    if provider_kind != "allocator" {
        return Err(
            "[provider-package-hako-build/unsupported-kind] hako-derived fixture is allocator-only"
                .to_string(),
        );
    }
    let semantic_codegen = config
        .provider_package_hako_semantic_codegen
        .as_deref()
        .unwrap_or("none");
    if semantic_codegen != "none"
        && semantic_codegen != "ping-literal-v0"
        && semantic_codegen != "alloc-free-owns-literal-v0"
        && semantic_codegen != OBJECT_LIFECYCLE_MODE
    {
        return Err("[provider-package-hako-build/unsupported-semantic-codegen] expected none|ping-literal-v0|alloc-free-owns-literal-v0|object-lifecycle-small-alloc-release-v0".to_string());
    }

    let artifact_name = match config.provider_package_artifact_name.as_deref() {
        Some(name) => {
            if name.contains('/') || name.contains('\\') {
                return Err(
                    "[provider-package-hako-build/invalid-artifact-name] expected single file name"
                        .to_string(),
                );
            }
            name.to_string()
        }
        None => default_artifact_name(&platform).to_string(),
    };
    require_shared_library_name(Path::new(&artifact_name))?;

    fs::create_dir_all(&out_dir)
        .map_err(|error| format!("[provider-package-hako-build/create-dir-failed] {error}"))?;
    let manifest_path = out_dir.join("hakorune_provider.json");
    let sha_path = out_dir.join("hakorune_provider.sha256");
    let artifact_path = out_dir.join(&artifact_name);
    if !config.provider_package_force
        && (manifest_path.exists() || sha_path.exists() || artifact_path.exists())
    {
        return Err(
            "[provider-package-hako-build/output-exists] pass --provider-package-force".to_string(),
        );
    }

    let build_dir = out_dir.join(".hakorune_provider_build");
    fs::create_dir_all(&build_dir).map_err(|error| {
        format!("[provider-package-hako-build/create-build-dir-failed] {error}")
    })?;
    let mir_json_path = build_dir.join("hako_derived_fixture.mir.json");
    emit_mir_json(&hako_source, &mir_json_path)?;

    let hako_source_hash = sha256_file(&hako_source)?;
    let hako_mir_json_hash = sha256_file(&mir_json_path)?;
    let semantic_ping_value = if semantic_codegen == "ping-literal-v0"
        || semantic_codegen == "alloc-free-owns-literal-v0"
        || semantic_codegen == OBJECT_LIFECYCLE_MODE
    {
        Some(extract_hako_provider_ping_literal(&mir_json_path)?)
    } else {
        None
    };
    let semantic_owns_value = if semantic_codegen == "alloc-free-owns-literal-v0"
        || semantic_codegen == OBJECT_LIFECYCLE_MODE
    {
        let value = extract_hako_provider_owns_allocated_literal(&mir_json_path)?;
        if value != 0 && value != 1 {
            return Err(
                "[provider-package-hako-build/owns-literal-out-of-range] expected 0|1".to_string(),
            );
        }
        Some(value)
    } else {
        None
    };
    let semantic_object_lifecycle_verified = if semantic_codegen == OBJECT_LIFECYCLE_MODE {
        validate_hako_provider_object_lifecycle_entrypoint(&mir_json_path)?;
        true
    } else {
        false
    };
    let activation = json!({
        "provider_call_allowed": config.provider_package_provider_call_allowed,
        "allocator_replacement_allowed": false,
        "hook_allowed": false,
        "global_allocator_allowed": false
    });
    let contract = json!({
        "abi_version": ABI_VERSION,
        "provider_kind": provider_kind,
        "capabilities": [
            "descriptor",
            "explicit_allocator_api",
        ],
        "required_exports": [DESCRIPTOR_EXPORT],
        "descriptor_schema_version": "hakorune-provider-descriptor-v1",
        "api_table_schema_version": "hakorune-provider-api-v1",
        "activation": activation,
        "memory_ownership_policy": "provider_alloc_provider_free",
        "hako_source_hash": hako_source_hash,
        "hako_mir_json_hash": hako_mir_json_hash,
        "hako_semantic_provider_codegen": semantic_codegen,
        "hako_provider_ping_value": semantic_ping_value,
        "hako_provider_owns_value": semantic_owns_value,
        "hako_provider_object_lifecycle_entrypoint_verified": semantic_object_lifecycle_verified,
    });
    let contract_hash = sha256_bytes(
        serde_json::to_string(&contract)
            .map_err(|error| {
                format!("[provider-package-hako-build/contract-serialize-failed] {error}")
            })?
            .as_bytes(),
    );
    let function_table_hash = hako_derived_function_table_hash(
        provider_kind,
        &hako_source_hash,
        &hako_mir_json_hash,
        semantic_codegen,
        semantic_ping_value,
        semantic_owns_value,
        semantic_object_lifecycle_verified,
    )?;
    let source_path = build_dir.join("hako_derived_provider_wrapper.c");
    fs::write(
        &source_path,
        hako_derived_wrapper_source(
            &package_id,
            provider_kind,
            provider_version,
            &contract_hash,
            &function_table_hash,
            semantic_ping_value.unwrap_or(0),
            semantic_owns_value.unwrap_or(1),
        ),
    )
    .map_err(|error| format!("[provider-package-hako-build/write-source-failed] {error}"))?;

    build_hako_derived_wrapper(&source_path, &artifact_path)?;

    let artifact_sha = sha256_file(&artifact_path)?;
    let artifact_size = artifact_path
        .metadata()
        .map_err(|error| format!("[provider-package-hako-build/stat-failed] {error}"))?
        .len();
    let manifest = json!({
        "schema_version": SCHEMA_VERSION,
        "package_id": package_id,
        "provider_kind": provider_kind,
        "provider_name": provider_name,
        "provider_version": provider_version,
        "abi_version": ABI_VERSION,
        "target_triple": target_triple,
        "platform": platform,
        "profile": profile,
        "build": {
            "mode": BUILD_MODE,
            "producer": "hakorune-hako-derived-provider-wrapper-c",
            "hako_source": {
                "path": hako_source.display().to_string(),
                "sha256": hako_source_hash
            },
            "hako_mir_json": {
                "path": mir_json_path.display().to_string(),
                "sha256": hako_mir_json_hash
            },
            "hako_semantic_provider_codegen": semantic_codegen,
            "hako_provider_ping_value": semantic_ping_value,
            "hako_provider_owns_value": semantic_owns_value,
            "hako_provider_object_lifecycle_entrypoint_verified": semantic_object_lifecycle_verified,
            "hako_shared_library_generation": true
        },
        "artifact": {
            "path": artifact_name,
            "sha256": artifact_sha,
            "size_bytes": artifact_size
        },
        "contract_hash": contract_hash,
        "required_exports": [DESCRIPTOR_EXPORT],
        "capabilities": ["descriptor", "explicit_allocator_api"],
        "activation": activation
    });
    let manifest_text = serde_json::to_string_pretty(&manifest).map_err(|error| {
        format!("[provider-package-hako-build/manifest-serialize-failed] {error}")
    })? + "\n";
    fs::write(&manifest_path, manifest_text)
        .map_err(|error| format!("[provider-package-hako-build/write-manifest-failed] {error}"))?;
    fs::write(&sha_path, format!("{artifact_sha}  {artifact_name}\n"))
        .map_err(|error| format!("[provider-package-hako-build/write-sha-failed] {error}"))?;

    let output = format!(
        "output_contract={OUTPUT_CONTRACT}\n\
         package_mode={PACKAGE_MODE}\n\
         build_mode={BUILD_MODE}\n\
         build_command_executed=1\n\
         hako_source_path={}\n\
         hako_source_checked=1\n\
         hako_source_hash={hako_source_hash}\n\
         hako_mir_json_path={}\n\
         hako_mir_json_emitted=1\n\
         hako_mir_json_hash={hako_mir_json_hash}\n\
         hako_semantic_provider_codegen={}\n\
         hako_provider_ping_codegen={}\n\
         hako_provider_ping_value={}\n\
         hako_provider_owns_codegen={}\n\
         hako_provider_owns_value={}\n\
         hako_provider_object_lifecycle_codegen={}\n\
         hako_provider_object_lifecycle_entrypoint_verified={}\n\
         shared_library_artifact_generated=1\n\
         arbitrary_shell_build_executed=0\n\
         package_dir={}\n\
         build_source={}\n\
         manifest_path={}\n\
         sha256_path={}\n\
         schema_version={SCHEMA_VERSION}\n\
         package_id={package_id}\n\
         provider_kind={provider_kind}\n\
         provider_name={provider_name}\n\
         provider_version={provider_version}\n\
         abi_version={ABI_VERSION}\n\
         target_triple={target_triple}\n\
         platform={platform}\n\
         profile={profile}\n\
         artifact_path={artifact_name}\n\
         artifact_sha256={artifact_sha}\n\
         artifact_size_bytes={artifact_size}\n\
         contract_hash={contract_hash}\n\
         required_exports={DESCRIPTOR_EXPORT}\n\
         capabilities=descriptor,explicit_allocator_api\n\
         provider_call_allowed={}\n\
         provider_active=0\n\
         replacement_active=0\n\
         hook_installed=0\n\
         global_allocator=0\n\
         shared_library_load_executed=0\n\
         required_export_resolved=0\n\
         descriptor_read_executed=0\n\
         provider_call_executed=0\n\
         winner_claim=0\n\
         summary=ok\n",
        hako_source.display(),
        mir_json_path.display(),
        semantic_codegen_output(semantic_codegen),
        if semantic_ping_value.is_some() { 1 } else { 0 },
        semantic_ping_value.unwrap_or(0),
        if semantic_owns_value.is_some() { 1 } else { 0 },
        semantic_owns_value.unwrap_or(0),
        if semantic_object_lifecycle_verified {
            1
        } else {
            0
        },
        if semantic_object_lifecycle_verified {
            1
        } else {
            0
        },
        out_dir.display(),
        source_path.display(),
        manifest_path.display(),
        sha_path.display(),
        if config.provider_package_provider_call_allowed {
            1
        } else {
            0
        }
    );
    Ok((output, 0))
}

fn emit_mir_json(hako_source: &Path, mir_json_path: &Path) -> Result<(), String> {
    let exe = std::env::current_exe()
        .map_err(|error| format!("[provider-package-hako-build/current-exe-failed] {error}"))?;
    let output = Command::new(exe)
        .arg("--backend")
        .arg("mir")
        .arg("--emit-mir-json")
        .arg(mir_json_path)
        .arg(hako_source)
        .env("NYASH_FEATURES", "rune")
        .env("NYASH_DISABLE_PLUGINS", "1")
        .output()
        .map_err(|error| format!("[provider-package-hako-build/emit-mir-start-failed] {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "[provider-package-hako-build/emit-mir-failed] status={} stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    if !mir_json_path.is_file() {
        return Err("[provider-package-hako-build/emit-mir-missing-output]".to_string());
    }
    Ok(())
}

fn extract_hako_provider_ping_literal(mir_json_path: &Path) -> Result<i64, String> {
    extract_hako_provider_literal(
        mir_json_path,
        "HakoProvider.ping/0",
        "ping",
        "missing-hako-provider-ping",
    )
}

fn extract_hako_provider_owns_allocated_literal(mir_json_path: &Path) -> Result<i64, String> {
    extract_hako_provider_literal(
        mir_json_path,
        "HakoProvider.ownsAllocated/0",
        "owns",
        "missing-hako-provider-owns-allocated",
    )
}

fn validate_hako_provider_object_lifecycle_entrypoint(mir_json_path: &Path) -> Result<(), String> {
    let text = fs::read_to_string(mir_json_path)
        .map_err(|error| format!("[provider-package-hako-build/read-mir-json-failed] {error}"))?;
    let data: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| format!("[provider-package-hako-build/parse-mir-json-failed] {error}"))?;
    let functions = data
        .get("functions")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "[provider-package-hako-build/mir-json-missing-functions]".to_string())?;
    require_mir_function(
        functions,
        "HakoProvider.objectLifecycleSmallAllocReleaseOk/0",
    )?;
    for required in [
        "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
        "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
        "HakoAllocPageModel.acquire/1",
        "HakoAllocPageModel.releaseLocal/1",
    ] {
        require_mir_function(functions, required)?;
    }

    let provider_fn = find_mir_function(
        functions,
        "HakoProvider.objectLifecycleSmallAllocReleaseOk/0",
    )?;
    require_mir_method_call(
        provider_fn,
        "HakoAllocObjectLifecycleFacade",
        "objectLifecycleSmallAlloc",
    )?;
    require_mir_method_call(
        provider_fn,
        "HakoAllocObjectLifecycleFacade",
        "objectLifecycleReleaseBlock",
    )?;

    let alloc_fn = find_mir_function(
        functions,
        "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    )?;
    require_mir_method_call(alloc_fn, "HakoAllocPageModel", "acquire")?;

    let release_fn = find_mir_function(
        functions,
        "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
    )?;
    require_mir_method_call(release_fn, "HakoAllocPageModel", "releaseLocal")?;
    Ok(())
}

fn require_mir_function<'a>(
    functions: &'a [serde_json::Value],
    name: &str,
) -> Result<&'a serde_json::Value, String> {
    find_mir_function(functions, name)
        .map_err(|_| format!("[provider-package-hako-build/missing-mir-function] {name}"))
}

fn find_mir_function<'a>(
    functions: &'a [serde_json::Value],
    name: &str,
) -> Result<&'a serde_json::Value, String> {
    functions
        .iter()
        .find(|function| function.get("name").and_then(serde_json::Value::as_str) == Some(name))
        .ok_or_else(|| format!("[provider-package-hako-build/missing-mir-function] {name}"))
}

fn require_mir_method_call(
    function: &serde_json::Value,
    box_name: &str,
    method_name: &str,
) -> Result<(), String> {
    let blocks = function
        .get("blocks")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| {
            "[provider-package-hako-build/object-lifecycle-missing-blocks]".to_string()
        })?;
    for block in blocks {
        let Some(instructions) = block
            .get("instructions")
            .and_then(serde_json::Value::as_array)
        else {
            continue;
        };
        for instruction in instructions {
            if instruction.get("op").and_then(serde_json::Value::as_str) != Some("mir_call") {
                continue;
            }
            let Some(callee) = instruction
                .get("mir_call")
                .and_then(|call| call.get("callee"))
            else {
                continue;
            };
            if callee.get("type").and_then(serde_json::Value::as_str) == Some("Method")
                && callee.get("box_name").and_then(serde_json::Value::as_str) == Some(box_name)
                && callee.get("name").and_then(serde_json::Value::as_str) == Some(method_name)
            {
                return Ok(());
            }
        }
    }
    Err(format!(
        "[provider-package-hako-build/missing-object-lifecycle-call] {}.{} in {}",
        box_name,
        method_name,
        function
            .get("name")
            .and_then(serde_json::Value::as_str)
            .unwrap_or("<unknown>")
    ))
}

fn extract_hako_provider_literal(
    mir_json_path: &Path,
    function_name: &str,
    tag: &str,
    missing_error: &str,
) -> Result<i64, String> {
    let text = fs::read_to_string(mir_json_path)
        .map_err(|error| format!("[provider-package-hako-build/read-mir-json-failed] {error}"))?;
    let data: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| format!("[provider-package-hako-build/parse-mir-json-failed] {error}"))?;
    let functions = data
        .get("functions")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "[provider-package-hako-build/mir-json-missing-functions]".to_string())?;
    let function = functions
        .iter()
        .find(|function| {
            function.get("name").and_then(serde_json::Value::as_str) == Some(function_name)
        })
        .ok_or_else(|| format!("[provider-package-hako-build/{missing_error}] {function_name}"))?;

    let mut constants = HashMap::new();
    let mut ret_value = None;
    let blocks = function
        .get("blocks")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| format!("[provider-package-hako-build/{tag}-missing-blocks]"))?;
    for block in blocks {
        let instructions = block
            .get("instructions")
            .and_then(serde_json::Value::as_array)
            .ok_or_else(|| format!("[provider-package-hako-build/{tag}-missing-instructions]"))?;
        for instruction in instructions {
            match instruction.get("op").and_then(serde_json::Value::as_str) {
                Some("const") => {
                    let Some(dst) = instruction.get("dst").and_then(serde_json::Value::as_i64)
                    else {
                        continue;
                    };
                    let value = instruction
                        .get("value")
                        .and_then(|value| value.get("value"))
                        .and_then(serde_json::Value::as_i64);
                    let ty = instruction
                        .get("value")
                        .and_then(|value| value.get("type"))
                        .and_then(serde_json::Value::as_str);
                    if ty == Some("i64") {
                        if let Some(value) = value {
                            constants.insert(dst, value);
                        }
                    }
                }
                Some("ret") => {
                    ret_value = instruction.get("value").and_then(serde_json::Value::as_i64);
                }
                _ => {}
            }
        }
    }
    let ret_value =
        ret_value.ok_or_else(|| format!("[provider-package-hako-build/{tag}-missing-ret]"))?;
    constants
        .get(&ret_value)
        .copied()
        .ok_or_else(|| format!("[provider-package-hako-build/{tag}-ret-not-literal-i64]"))
}

fn build_hako_derived_wrapper(source_path: &Path, artifact_path: &Path) -> Result<(), String> {
    let output = Command::new("cc")
        .arg("-shared")
        .arg("-fPIC")
        .arg("-o")
        .arg(artifact_path)
        .arg(source_path)
        .output()
        .map_err(|error| format!("[provider-package-hako-build/compiler-start-failed] {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "[provider-package-hako-build/compiler-failed] status={} stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

fn hako_derived_function_table_hash(
    provider_kind: &str,
    hako_source_hash: &str,
    hako_mir_json_hash: &str,
    semantic_codegen: &str,
    semantic_ping_value: Option<i64>,
    semantic_owns_value: Option<i64>,
    semantic_object_lifecycle_verified: bool,
) -> Result<String, String> {
    let contract = json!({
        "abi_version": ABI_VERSION,
        "api_table_schema_version": "hakorune-provider-api-v1",
        "entrypoints": ["ping", "alloc", "free", "owns"],
        "provider_kind": provider_kind,
        "hako_source_hash": hako_source_hash,
        "hako_mir_json_hash": hako_mir_json_hash,
        "hako_semantic_provider_codegen": semantic_codegen,
        "hako_provider_ping_value": semantic_ping_value,
        "hako_provider_owns_value": semantic_owns_value,
        "hako_provider_object_lifecycle_entrypoint_verified": semantic_object_lifecycle_verified,
    });
    Ok(sha256_bytes(
        serde_json::to_string(&contract)
            .map_err(|error| {
                format!(
                    "[provider-package-hako-build/function-table-hash-serialize-failed] {error}"
                )
            })?
            .as_bytes(),
    ))
}

fn hako_derived_wrapper_source(
    package_id: &str,
    provider_kind: &str,
    provider_version: &str,
    contract_hash: &str,
    function_table_hash: &str,
    ping_value: i64,
    owns_value: i64,
) -> String {
    let source = r#"#include <stdint.h>
#include <stddef.h>
#include <stdlib.h>

typedef struct HakoProviderDescriptorV1 {
  uint32_t magic;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t descriptor_size;
  const char* provider_id;
  const char* provider_kind;
  const char* provider_version;
  uint64_t capability_bits;
  uint64_t safety_flags;
  const char* contract_hash;
  const char* function_table_hash;
  uint32_t api_table_size;
  uint32_t reserved;
} HakoProviderDescriptorV1;

typedef struct HakoProviderApiV1 {
  uint32_t magic;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t api_table_size;
  int (*ping)(void);
  void* (*alloc)(size_t size, size_t align);
  void (*free)(void* ptr);
  int (*owns)(void* ptr);
} HakoProviderApiV1;

static int hako_ping(void) { return __PING_VALUE__; }
static void* hako_alloc(size_t size, size_t align) {
  (void)align;
  return malloc(size);
}
static void hako_free(void* ptr) { free(ptr); }
static int hako_owns(void* ptr) {
  if (ptr == NULL) {
    return 0;
  }
  return __OWNS_VALUE__;
}

static const HakoProviderApiV1 API = {
  0x484B5241u, 1, 0, sizeof(HakoProviderApiV1),
  hako_ping, hako_alloc, hako_free, hako_owns
};

static const HakoProviderDescriptorV1 DESCRIPTOR = {
  0x484B5250u, 1, 0, sizeof(HakoProviderDescriptorV1),
  "__PACKAGE_ID__", "__PROVIDER_KIND__", "__PROVIDER_VERSION__",
  3u, 1u,
  "__CONTRACT_HASH__",
  "__FUNCTION_TABLE_HASH__",
  sizeof(HakoProviderApiV1), 0
};

__attribute__((visibility("default")))
const HakoProviderDescriptorV1* hakorune_provider_descriptor_v1(void) {
  return &DESCRIPTOR;
}

__attribute__((visibility("default")))
const HakoProviderApiV1* hakorune_provider_get_api_v1(void) {
  return &API;
}
"#;
    source
        .replace("__PACKAGE_ID__", &c_string_fragment(package_id))
        .replace("__PROVIDER_KIND__", &c_string_fragment(provider_kind))
        .replace("__PROVIDER_VERSION__", &c_string_fragment(provider_version))
        .replace("__CONTRACT_HASH__", contract_hash)
        .replace("__FUNCTION_TABLE_HASH__", function_table_hash)
        .replace("__PING_VALUE__", &ping_value.to_string())
        .replace("__OWNS_VALUE__", &owns_value.to_string())
}

fn semantic_codegen_output(mode: &str) -> &str {
    if mode == "none" {
        "0"
    } else {
        mode
    }
}

fn c_string_fragment(value: &str) -> String {
    let mut out = String::new();
    for ch in value.chars() {
        match ch {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            ch if ch.is_ascii_graphic() || ch == ' ' => out.push(ch),
            ch => out.push_str(&format!("\\x{:02x}", ch as u32)),
        }
    }
    out
}

fn default_artifact_name(platform: &str) -> &'static str {
    match platform {
        "windows" | "win32" => "hakorune_provider.dll",
        "macos" | "darwin" => "libhakorune_provider.dylib",
        _ => "libhakorune_provider.so",
    }
}

fn required_string(value: Option<&str>, name: &str) -> Result<String, String> {
    let Some(value) = value else {
        return Err(format!(
            "[provider-package-hako-build/missing-arg] --{name}"
        ));
    };
    if value.is_empty() {
        return Err(format!("[provider-package-hako-build/empty-arg] --{name}"));
    }
    Ok(value.to_string())
}

fn required_path(value: Option<&str>, name: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(required_string(value, name)?))
}

fn require_hako_source(path: &Path) -> Result<(), String> {
    if !path.is_file() {
        return Err(format!(
            "[provider-package-hako-build/source-not-found] {}",
            path.display()
        ));
    }
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("hako") => Ok(()),
        _ => {
            Err("[provider-package-hako-build/invalid-source-extension] expected .hako".to_string())
        }
    }
}

fn require_shared_library_name(path: &Path) -> Result<(), String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("so" | "dll" | "dylib") => Ok(()),
        _ => Err(
            "[provider-package-hako-build/invalid-binary-extension] expected .so|.dll|.dylib"
                .to_string(),
        ),
    }
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(path)
        .map_err(|error| format!("[provider-package-hako-build/read-failed] {error}"))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|error| format!("[provider-package-hako-build/read-failed] {error}"))?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
