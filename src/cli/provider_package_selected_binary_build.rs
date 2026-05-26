use super::diagnostic_output::finish_result;
use super::CliConfig;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

const SCHEMA_VERSION: &str = "hakorune-provider-package-v1";
const ABI_VERSION: &str = "hakorune-provider-abi-v1";
const DESCRIPTOR_EXPORT: &str = "hakorune_provider_descriptor_v1";
const OUTPUT_CONTRACT: &str = "hakorune-provider-package-selected-binary-build-v0";
const BUILD_MODE: &str = "selected-fixture";
const PACKAGE_MODE: &str = "selected-binary-build-package";

pub fn maybe_run_provider_package_selected_binary_build(config: &CliConfig) -> Option<i32> {
    if !config.provider_package_selected_binary_build_fixture {
        return None;
    }
    finish_result(run_provider_package_selected_binary_build(config))
}

fn run_provider_package_selected_binary_build(config: &CliConfig) -> Result<(String, i32), String> {
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
            "[provider-package-build/invalid-profile] expected speed|diagnostic".to_string(),
        );
    }
    if provider_kind != "allocator" {
        return Err(
            "[provider-package-build/unsupported-kind] selected fixture is allocator-only"
                .to_string(),
        );
    }

    let artifact_name = match config.provider_package_artifact_name.as_deref() {
        Some(name) => {
            if name.contains('/') || name.contains('\\') {
                return Err(
                    "[provider-package-build/invalid-artifact-name] expected single file name"
                        .to_string(),
                );
            }
            name.to_string()
        }
        None => default_artifact_name(&platform).to_string(),
    };
    require_shared_library_name(Path::new(&artifact_name))?;

    fs::create_dir_all(&out_dir)
        .map_err(|error| format!("[provider-package-build/create-dir-failed] {error}"))?;
    let manifest_path = out_dir.join("hakorune_provider.json");
    let sha_path = out_dir.join("hakorune_provider.sha256");
    let artifact_path = out_dir.join(&artifact_name);
    if !config.provider_package_force
        && (manifest_path.exists() || sha_path.exists() || artifact_path.exists())
    {
        return Err(
            "[provider-package-build/output-exists] pass --provider-package-force".to_string(),
        );
    }

    let build_dir = out_dir.join(".hakorune_provider_build");
    fs::create_dir_all(&build_dir)
        .map_err(|error| format!("[provider-package-build/create-build-dir-failed] {error}"))?;
    let source_path = build_dir.join("selected_fixture_provider.c");
    fs::write(&source_path, selected_fixture_source())
        .map_err(|error| format!("[provider-package-build/write-source-failed] {error}"))?;

    build_selected_fixture(&source_path, &artifact_path)?;

    let artifact_sha = sha256_file(&artifact_path)?;
    let artifact_size = artifact_path
        .metadata()
        .map_err(|error| format!("[provider-package-build/stat-failed] {error}"))?
        .len();
    let activation = json!({
        "provider_call_allowed": config.provider_package_provider_call_allowed,
        "allocator_replacement_allowed": false,
        "hook_allowed": false,
        "global_allocator_allowed": false
    });
    let required_exports = vec![DESCRIPTOR_EXPORT.to_string()];
    let capabilities = vec![
        "descriptor".to_string(),
        "explicit_allocator_api".to_string(),
    ];
    let contract = json!({
        "abi_version": ABI_VERSION,
        "provider_kind": provider_kind,
        "capabilities": capabilities,
        "required_exports": required_exports,
        "descriptor_schema_version": "hakorune-provider-descriptor-v1",
        "api_table_schema_version": "hakorune-provider-api-v1",
        "activation": activation,
        "memory_ownership_policy": "provider_alloc_provider_free"
    });
    let contract_hash = sha256_bytes(
        serde_json::to_string(&contract)
            .map_err(|error| format!("[provider-package-build/contract-serialize-failed] {error}"))?
            .as_bytes(),
    );
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
            "producer": "hakorune-selected-fixture-provider-c",
            "hako_shared_library_generation": false
        },
        "artifact": {
            "path": artifact_name,
            "sha256": artifact_sha,
            "size_bytes": artifact_size
        },
        "contract_hash": contract_hash,
        "required_exports": required_exports,
        "capabilities": capabilities,
        "activation": activation
    });
    let manifest_text = serde_json::to_string_pretty(&manifest)
        .map_err(|error| format!("[provider-package-build/manifest-serialize-failed] {error}"))?
        + "\n";
    fs::write(&manifest_path, manifest_text)
        .map_err(|error| format!("[provider-package-build/write-manifest-failed] {error}"))?;
    fs::write(&sha_path, format!("{artifact_sha}  {artifact_name}\n"))
        .map_err(|error| format!("[provider-package-build/write-sha-failed] {error}"))?;

    let output = format!(
        "output_contract={OUTPUT_CONTRACT}\n\
         package_mode={PACKAGE_MODE}\n\
         build_mode={BUILD_MODE}\n\
         build_command_executed=1\n\
         hako_shared_library_generation=0\n\
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

fn build_selected_fixture(source_path: &Path, artifact_path: &Path) -> Result<(), String> {
    let output = Command::new("cc")
        .arg("-shared")
        .arg("-fPIC")
        .arg("-o")
        .arg(artifact_path)
        .arg(source_path)
        .output()
        .map_err(|error| format!("[provider-package-build/compiler-start-failed] {error}"))?;
    if !output.status.success() {
        return Err(format!(
            "[provider-package-build/compiler-failed] status={} stderr={}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        ));
    }
    Ok(())
}

fn default_artifact_name(platform: &str) -> &'static str {
    match platform {
        "windows" | "win32" => "hakorune_provider.dll",
        "macos" | "darwin" => "libhakorune_provider.dylib",
        _ => "libhakorune_provider.so",
    }
}

fn selected_fixture_source() -> &'static str {
    r#"#include <stdint.h>
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

static int hako_ping(void) { return 0; }
static void* hako_alloc(size_t size, size_t align) {
  (void)align;
  return malloc(size);
}
static void hako_free(void* ptr) { free(ptr); }
static int hako_owns(void* ptr) { return ptr != NULL; }

static const HakoProviderDescriptorV1 DESCRIPTOR = {
  0x484B5250u, 1, 0, sizeof(HakoProviderDescriptorV1),
  "org.hakorune.provider.selected.fixture", "allocator", "0.1.0",
  3u, 1u,
  "0000000000000000000000000000000000000000000000000000000000000000",
  "1111111111111111111111111111111111111111111111111111111111111111",
  sizeof(HakoProviderApiV1), 0
};

__attribute__((visibility("default")))
const HakoProviderDescriptorV1* hakorune_provider_descriptor_v1(void) {
  return &DESCRIPTOR;
}

__attribute__((visibility("default")))
int hakorune_provider_get_api_v1(const void* host, HakoProviderApiV1* out) {
  (void)host;
  if (!out) return -1;
  out->magic = 0x484B5241u;
  out->abi_major = 1;
  out->abi_minor = 0;
  out->api_table_size = sizeof(HakoProviderApiV1);
  out->ping = hako_ping;
  out->alloc = hako_alloc;
  out->free = hako_free;
  out->owns = hako_owns;
  return 0;
}
"#
}

fn required_string(value: Option<&str>, name: &str) -> Result<String, String> {
    let Some(value) = value else {
        return Err(format!("[provider-package-build/missing-arg] --{name}"));
    };
    if value.is_empty() {
        return Err(format!("[provider-package-build/empty-arg] --{name}"));
    }
    Ok(value.to_string())
}

fn required_path(value: Option<&str>, name: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(required_string(value, name)?))
}

fn require_shared_library_name(path: &Path) -> Result<(), String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("so" | "dll" | "dylib") => Ok(()),
        _ => Err(
            "[provider-package-build/invalid-binary-extension] expected .so|.dll|.dylib"
                .to_string(),
        ),
    }
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = fs::File::open(path)
        .map_err(|error| format!("[provider-package-build/read-failed] {error}"))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|error| format!("[provider-package-build/read-failed] {error}"))?;
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
