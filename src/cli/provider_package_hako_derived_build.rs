use super::diagnostic_output::finish_result;
use super::CliConfig;
use mir_json::{
    emit_mir_json, extract_hako_provider_owns_allocated_literal,
    extract_hako_provider_ping_literal, validate_hako_provider_object_lifecycle_entrypoint,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

mod mir_json;

const SCHEMA_VERSION: &str = "hakorune-provider-package-v1";
const ABI_VERSION: &str = "hakorune-provider-abi-v1";
const DESCRIPTOR_EXPORT: &str = "hakorune_provider_descriptor_v1";
const OUTPUT_CONTRACT: &str = "hakorune-provider-package-hako-derived-build-v0";
const BUILD_MODE: &str = "hako-derived-selected-fixture";
const PACKAGE_MODE: &str = "hako-derived-provider-package";
const OBJECT_LIFECYCLE_MODE: &str = "object-lifecycle-small-alloc-release-v0";
const OBJECT_LIFECYCLE_NATIVE_SLOT_MODE: &str = "object-lifecycle-native-slot-bridge-v0";
const HOST_ALLOC_FREE_ROUTE: &str = "host_malloc_free_wrapper";
const HOST_OBJECT_LIFECYCLE_USAGE: &str = "metadata_verification_only";
const HOST_BACKED_ADAPTER_KIND: &str = "host_backed_adapter";
const NATIVE_SLOT_ALLOC_FREE_ROUTE: &str = "native_static_slot_bridge_from_object_lifecycle_shape";
const NATIVE_SLOT_OBJECT_LIFECYCLE_USAGE: &str = "native_shape_codegen";
const PURE_PROVIDER_KIND: &str = "pure_allocator";

#[derive(Clone, Copy)]
struct AllocFreeRoute {
    route: &'static str,
    allocator_kind: &'static str,
    realloc_claim_enabled: bool,
    usable_size_claim_enabled: bool,
    host_allocator_vtable_init: bool,
    uses_host_malloc: bool,
    uses_hako_object_lifecycle: bool,
    object_lifecycle_usage: &'static str,
}

fn alloc_free_route(semantic_codegen: &str) -> AllocFreeRoute {
    if semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE {
        AllocFreeRoute {
            route: NATIVE_SLOT_ALLOC_FREE_ROUTE,
            allocator_kind: PURE_PROVIDER_KIND,
            realloc_claim_enabled: true,
            usable_size_claim_enabled: true,
            host_allocator_vtable_init: false,
            uses_host_malloc: false,
            uses_hako_object_lifecycle: true,
            object_lifecycle_usage: NATIVE_SLOT_OBJECT_LIFECYCLE_USAGE,
        }
    } else {
        AllocFreeRoute {
            route: HOST_ALLOC_FREE_ROUTE,
            allocator_kind: HOST_BACKED_ADAPTER_KIND,
            realloc_claim_enabled: true,
            usable_size_claim_enabled: true,
            host_allocator_vtable_init: true,
            uses_host_malloc: true,
            uses_hako_object_lifecycle: false,
            object_lifecycle_usage: HOST_OBJECT_LIFECYCLE_USAGE,
        }
    }
}

fn bool_i32(value: bool) -> i32 {
    if value {
        1
    } else {
        0
    }
}

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
        && semantic_codegen != OBJECT_LIFECYCLE_NATIVE_SLOT_MODE
    {
        return Err("[provider-package-hako-build/unsupported-semantic-codegen] expected none|ping-literal-v0|alloc-free-owns-literal-v0|object-lifecycle-small-alloc-release-v0|object-lifecycle-native-slot-bridge-v0".to_string());
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
        || semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE
    {
        Some(extract_hako_provider_ping_literal(&mir_json_path)?)
    } else {
        None
    };
    let semantic_owns_value = if semantic_codegen == "alloc-free-owns-literal-v0"
        || semantic_codegen == OBJECT_LIFECYCLE_MODE
        || semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE
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
    let semantic_object_lifecycle_verified = if semantic_codegen == OBJECT_LIFECYCLE_MODE
        || semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE
    {
        validate_hako_provider_object_lifecycle_entrypoint(&mir_json_path)?;
        true
    } else {
        false
    };
    let alloc_route = alloc_free_route(semantic_codegen);
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
        "hako_provider_alloc_free_route": alloc_route.route,
        "provider_allocator_kind": alloc_route.allocator_kind,
        "provider_abi_claim_ops_v1": true,
        "provider_free_claim_enabled": true,
        "provider_realloc_claim_enabled": alloc_route.realloc_claim_enabled,
        "provider_usable_size_claim_enabled": alloc_route.usable_size_claim_enabled,
        "compat_alloc_free_owns_still_supported": true,
        "compat_owns_free_mainline": false,
        "host_allocator_vtable_init": alloc_route.host_allocator_vtable_init,
        "provider_direct_libc_symbol_dependency": false,
        "ld_preload_reentry_for_host_alloc": false,
        "hako_provider_alloc_free_uses_host_malloc": alloc_route.uses_host_malloc,
        "hako_provider_alloc_free_uses_hako_object_lifecycle": alloc_route.uses_hako_object_lifecycle,
        "hako_provider_object_lifecycle_entrypoint_usage": alloc_route.object_lifecycle_usage,
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
        alloc_route,
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
            semantic_codegen == OBJECT_LIFECYCLE_NATIVE_SLOT_MODE,
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
            "hako_provider_alloc_free_route": alloc_route.route,
            "provider_allocator_kind": alloc_route.allocator_kind,
            "provider_abi_claim_ops_v1": true,
            "provider_free_claim_enabled": true,
            "provider_realloc_claim_enabled": alloc_route.realloc_claim_enabled,
            "provider_usable_size_claim_enabled": alloc_route.usable_size_claim_enabled,
            "compat_alloc_free_owns_still_supported": true,
            "compat_owns_free_mainline": false,
            "host_allocator_vtable_init": alloc_route.host_allocator_vtable_init,
            "provider_direct_libc_symbol_dependency": false,
            "ld_preload_reentry_for_host_alloc": false,
            "hako_provider_alloc_free_uses_host_malloc": alloc_route.uses_host_malloc,
            "hako_provider_alloc_free_uses_hako_object_lifecycle": alloc_route.uses_hako_object_lifecycle,
            "hako_provider_object_lifecycle_entrypoint_usage": alloc_route.object_lifecycle_usage,
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
         hako_provider_alloc_free_route={}\n\
         provider_allocator_kind={}\n\
         provider_abi_claim_ops_v1=1\n\
         provider_free_claim_enabled=1\n\
         provider_realloc_claim_enabled={}\n\
         provider_usable_size_claim_enabled={}\n\
         compat_alloc_free_owns_still_supported=1\n\
         compat_owns_free_mainline=0\n\
         host_allocator_vtable_init={}\n\
         provider_direct_libc_symbol_dependency=0\n\
         ld_preload_reentry_for_host_alloc=0\n\
         hako_provider_alloc_free_uses_host_malloc={}\n\
         hako_provider_alloc_free_uses_hako_object_lifecycle={}\n\
         hako_provider_object_lifecycle_entrypoint_usage={}\n\
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
        alloc_route.route,
        alloc_route.allocator_kind,
        bool_i32(alloc_route.realloc_claim_enabled),
        bool_i32(alloc_route.usable_size_claim_enabled),
        bool_i32(alloc_route.host_allocator_vtable_init),
        bool_i32(alloc_route.uses_host_malloc),
        bool_i32(alloc_route.uses_hako_object_lifecycle),
        alloc_route.object_lifecycle_usage,
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
    alloc_route: AllocFreeRoute,
) -> Result<String, String> {
    let contract = json!({
        "abi_version": ABI_VERSION,
        "api_table_schema_version": "hakorune-provider-api-v1",
        "entrypoints": ["ping", "alloc", "free", "owns", "free_claim", "usable_size_claim", "realloc_claim", "init_host_allocator"],
        "provider_kind": provider_kind,
        "provider_allocator_kind": alloc_route.allocator_kind,
        "provider_abi_claim_ops_v1": true,
        "provider_free_claim_enabled": true,
        "provider_realloc_claim_enabled": alloc_route.realloc_claim_enabled,
        "provider_usable_size_claim_enabled": alloc_route.usable_size_claim_enabled,
        "compat_alloc_free_owns_still_supported": true,
        "compat_owns_free_mainline": false,
        "host_allocator_vtable_init": alloc_route.host_allocator_vtable_init,
        "provider_direct_libc_symbol_dependency": false,
        "ld_preload_reentry_for_host_alloc": false,
        "hako_source_hash": hako_source_hash,
        "hako_mir_json_hash": hako_mir_json_hash,
        "hako_semantic_provider_codegen": semantic_codegen,
        "hako_provider_ping_value": semantic_ping_value,
        "hako_provider_owns_value": semantic_owns_value,
        "hako_provider_object_lifecycle_entrypoint_verified": semantic_object_lifecycle_verified,
        "hako_provider_alloc_free_route": alloc_route.route,
        "hako_provider_alloc_free_uses_host_malloc": alloc_route.uses_host_malloc,
        "hako_provider_alloc_free_uses_hako_object_lifecycle": alloc_route.uses_hako_object_lifecycle,
        "hako_provider_object_lifecycle_entrypoint_usage": alloc_route.object_lifecycle_usage,
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
    use_native_slot_bridge: bool,
) -> String {
    let source = if use_native_slot_bridge {
        hako_derived_native_slot_wrapper_template()
    } else {
        hako_derived_host_malloc_wrapper_template()
    };
    source
        .replace("__PACKAGE_ID__", &c_string_fragment(package_id))
        .replace("__PROVIDER_KIND__", &c_string_fragment(provider_kind))
        .replace("__PROVIDER_VERSION__", &c_string_fragment(provider_version))
        .replace("__CONTRACT_HASH__", contract_hash)
        .replace("__FUNCTION_TABLE_HASH__", function_table_hash)
        .replace("__PING_VALUE__", &ping_value.to_string())
        .replace("__OWNS_VALUE__", &owns_value.to_string())
}

fn hako_derived_host_malloc_wrapper_template() -> &'static str {
    include_str!("provider_package_hako_derived_build_templates/host_malloc_wrapper.c")
}

fn hako_derived_native_slot_wrapper_template() -> &'static str {
    include_str!("provider_package_hako_derived_build_templates/native_slot_wrapper.c")
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
