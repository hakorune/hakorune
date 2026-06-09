use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::process::Command;

pub(super) fn emit_mir_json(hako_source: &Path, mir_json_path: &Path) -> Result<(), String> {
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

pub(super) fn extract_hako_provider_ping_literal(mir_json_path: &Path) -> Result<i64, String> {
    extract_hako_provider_literal(
        mir_json_path,
        "HakoProvider.ping/0",
        "ping",
        "missing-hako-provider-ping",
    )
}

pub(super) fn extract_hako_provider_owns_allocated_literal(
    mir_json_path: &Path,
) -> Result<i64, String> {
    extract_hako_provider_literal(
        mir_json_path,
        "HakoProvider.ownsAllocated/0",
        "owns",
        "missing-hako-provider-owns-allocated",
    )
}

pub(super) fn validate_hako_provider_object_lifecycle_entrypoint(
    mir_json_path: &Path,
    provider_entrypoint: &str,
    alloc_entrypoint: &str,
    release_entrypoint: &str,
) -> Result<(), String> {
    let text = fs::read_to_string(mir_json_path)
        .map_err(|error| format!("[provider-package-hako-build/read-mir-json-failed] {error}"))?;
    let data: serde_json::Value = serde_json::from_str(&text)
        .map_err(|error| format!("[provider-package-hako-build/parse-mir-json-failed] {error}"))?;
    let functions = data
        .get("functions")
        .and_then(serde_json::Value::as_array)
        .ok_or_else(|| "[provider-package-hako-build/mir-json-missing-functions]".to_string())?;
    require_mir_function(functions, provider_entrypoint)?;
    require_mir_function(
        functions,
        "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1",
    )?;
    require_mir_function(
        functions,
        "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
    )?;
    for required in [alloc_entrypoint, release_entrypoint] {
        require_mir_function(functions, required)?;
    }

    let provider_fn = find_mir_function(functions, provider_entrypoint)?;
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
    require_mir_method_call(alloc_fn, "HakoAllocPageModel", "acquireFreshSmall")?;

    let release_fn = find_mir_function(
        functions,
        "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2",
    )?;
    require_mir_method_call(release_fn, "HakoAllocPageModel", "releaseLocalKnownLive")?;
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
