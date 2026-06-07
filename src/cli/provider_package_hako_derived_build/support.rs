use sha2::{Digest, Sha256};
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::Command;

pub(super) fn bool_i32(value: bool) -> i32 {
    if value {
        1
    } else {
        0
    }
}

pub(super) fn build_hako_derived_wrapper(
    source_path: &Path,
    artifact_path: &Path,
) -> Result<(), String> {
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

pub(super) fn hako_derived_wrapper_source(
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
    include_str!("../provider_package_hako_derived_build_templates/host_malloc_wrapper.c")
}

fn hako_derived_native_slot_wrapper_template() -> &'static str {
    include_str!("../provider_package_hako_derived_build_templates/native_slot_wrapper.c")
}

pub(super) fn semantic_codegen_output(mode: &str) -> &str {
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

pub(super) fn default_artifact_name(platform: &str) -> &'static str {
    match platform {
        "windows" | "win32" => "hakorune_provider.dll",
        "macos" | "darwin" => "libhakorune_provider.dylib",
        _ => "libhakorune_provider.so",
    }
}

pub(super) fn required_string(value: Option<&str>, name: &str) -> Result<String, String> {
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

pub(super) fn required_path(value: Option<&str>, name: &str) -> Result<PathBuf, String> {
    Ok(PathBuf::from(required_string(value, name)?))
}

pub(super) fn require_hako_source(path: &Path) -> Result<(), String> {
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

pub(super) fn require_shared_library_name(path: &Path) -> Result<(), String> {
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("so" | "dll" | "dylib") => Ok(()),
        _ => Err(
            "[provider-package-hako-build/invalid-binary-extension] expected .so|.dll|.dylib"
                .to_string(),
        ),
    }
}

pub(super) fn sha256_file(path: &Path) -> Result<String, String> {
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

pub(super) fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}
