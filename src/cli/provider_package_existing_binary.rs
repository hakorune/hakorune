use super::diagnostic_output::finish_result;
use super::CliConfig;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};

const SCHEMA_VERSION: &str = "hakorune-provider-package-v1";
const ABI_VERSION: &str = "hakorune-provider-abi-v1";
const DESCRIPTOR_EXPORT: &str = "hakorune_provider_descriptor_v1";
const OUTPUT_CONTRACT: &str = "hakorune-provider-package-existing-binary-manifest-v0";

pub fn maybe_run_provider_package_existing_binary(config: &CliConfig) -> Option<i32> {
    config.provider_package_existing_binary.as_ref()?;
    finish_result(run_provider_package_existing_binary(config))
}

fn run_provider_package_existing_binary(config: &CliConfig) -> Result<(String, i32), String> {
    let binary = required_path(
        config.provider_package_existing_binary.as_deref(),
        "provider-package-existing-binary",
    )?;
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
        return Err("[provider-package/invalid-profile] expected speed|diagnostic".to_string());
    }
    if !binary.is_file() {
        return Err(format!(
            "[provider-package/missing-binary] path={}",
            binary.display()
        ));
    }
    require_shared_library_name(&binary)?;
    let artifact_name = match config.provider_package_artifact_name.as_deref() {
        Some(name) => {
            if name.contains('/') || name.contains('\\') {
                return Err(
                    "[provider-package/invalid-artifact-name] expected single file name"
                        .to_string(),
                );
            }
            name.to_string()
        }
        None => binary
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| "[provider-package/invalid-binary-name]".to_string())?
            .to_string(),
    };
    require_shared_library_name(Path::new(&artifact_name))?;

    fs::create_dir_all(&out_dir)
        .map_err(|error| format!("[provider-package/create-dir-failed] {error}"))?;
    let manifest_path = out_dir.join("hakorune_provider.json");
    let sha_path = out_dir.join("hakorune_provider.sha256");
    let artifact_path = out_dir.join(&artifact_name);
    if !config.provider_package_force
        && (manifest_path.exists() || sha_path.exists() || artifact_path.exists())
    {
        return Err("[provider-package/output-exists] pass --provider-package-force".to_string());
    }

    fs::copy(&binary, &artifact_path)
        .map_err(|error| format!("[provider-package/copy-failed] {error}"))?;
    let artifact_sha = sha256_file(&artifact_path)?;
    let artifact_size = artifact_path
        .metadata()
        .map_err(|error| format!("[provider-package/stat-failed] {error}"))?
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
            .map_err(|error| format!("[provider-package/contract-serialize-failed] {error}"))?
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
        .map_err(|error| format!("[provider-package/manifest-serialize-failed] {error}"))?
        + "\n";
    fs::write(&manifest_path, manifest_text)
        .map_err(|error| format!("[provider-package/write-manifest-failed] {error}"))?;
    fs::write(&sha_path, format!("{artifact_sha}  {artifact_name}\n"))
        .map_err(|error| format!("[provider-package/write-sha-failed] {error}"))?;

    let output = format!(
        "output_contract={OUTPUT_CONTRACT}\n\
         package_mode=existing-binary-manifest\n\
         package_dir={}\n\
         source_binary={}\n\
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
        binary.display(),
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

fn required_string(value: Option<&str>, name: &str) -> Result<String, String> {
    let Some(value) = value else {
        return Err(format!("[provider-package/missing-arg] --{name}"));
    };
    if value.is_empty() {
        return Err(format!("[provider-package/empty-arg] --{name}"));
    }
    Ok(value.to_string())
}

fn required_path(value: Option<&str>, name: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(required_string(value, name)?))
}

fn require_shared_library_name(path: &Path) -> Result<(), String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("so" | "dll" | "dylib") => Ok(()),
        _ => {
            Err("[provider-package/invalid-binary-extension] expected .so|.dll|.dylib".to_string())
        }
    }
}

fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file =
        fs::File::open(path).map_err(|error| format!("[provider-package/read-failed] {error}"))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0_u8; 1024 * 1024];
    loop {
        let n = file
            .read(&mut buffer)
            .map_err(|error| format!("[provider-package/read-failed] {error}"))?;
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
