use crate::mir::definitions::call_unified::Callee;
use serde_json::json;

/// Helper: Create JSON v1 root with schema information
/// Includes version, capabilities, metadata for advanced MIR features
pub(crate) fn create_json_v1_root(functions: serde_json::Value) -> serde_json::Value {
    json!({
        "schema_version": "1.0",
        "capabilities": [
            "unified_call",      // Phase 15.5: Unified MirCall support
            "phi",              // SSA Phi functions
            "effects",          // Effect tracking for optimization
            "callee_typing"     // Type-safe call target resolution
        ],
        "metadata": {
            "generator": "nyash-rust",
            "phase": "15.5",
            "build_time": "Phase 15.5 Development",
            "features": ["mir_call_unification", "json_v1_schema"]
        },
        "functions": functions
    })
}

/// Helper: Emit unified mir_call JSON (v1 format)
/// Supports all 6 Callee types in a single unified JSON structure
pub(crate) fn emit_unified_mir_call(
    dst: Option<u32>,
    callee: &Callee,
    args: &[u32],
    effects: &[&str],
) -> serde_json::Value {
    let mut call_obj = json!({
        "op": "mir_call",
        "dst": dst,
        "mir_call": {
            "args": args,
            "effects": effects,
            "flags": {}
        }
    });

    // Generate Callee-specific mir_call structure
    match callee {
        Callee::Global(name) => {
            call_obj["mir_call"]["callee"] = json!({
                "type": "Global",
                "name": name
            });
        }
        Callee::Method {
            box_name,
            method,
            receiver,
            certainty,
            ..
        } => {
            call_obj["mir_call"]["callee"] = json!({
                "type": "Method",
                "box_name": box_name,
                "name": method,
                "receiver": receiver.map(|v| v.as_u32()),
                "certainty": match certainty { crate::mir::definitions::call_unified::TypeCertainty::Known => "Known", crate::mir::definitions::call_unified::TypeCertainty::Union => "Union" }
            });
        }
        Callee::Constructor { box_type } => {
            call_obj["mir_call"]["callee"] = json!({
                "type": "Constructor",
                "name": box_type
            });
        }
        Callee::Closure {
            params,
            captures,
            me_capture,
        } => {
            let captures_json: Vec<_> = captures
                .iter()
                .map(|(name, vid)| json!([name, vid.as_u32()]))
                .collect();
            call_obj["mir_call"]["callee"] = json!({
                "type": "Closure",
                "params": params,
                "captures": captures_json,
                "me_capture": me_capture.map(|v| v.as_u32())
            });
        }
        Callee::Value(vid) => {
            call_obj["mir_call"]["callee"] = json!({
                "type": "Value",
                "value": vid.as_u32()
            });
        }
        Callee::Extern(name) => {
            call_obj["mir_call"]["callee"] = json!({
                "type": "Extern",
                "name": name
            });
        }
    }

    call_obj
}

/// Helper: detect residual numeric-core boxcalls that should have been lowered by AotPrepNumericCoreBox.
/// Currently we only check for `boxcall` with `method:"mul_naive"` which should become
/// `call("NyNumericMatI64.mul_naive", ...)` when NYASH_AOT_NUMERIC_CORE=1 is effective.
#[allow(dead_code)]
pub(crate) fn has_numeric_core_boxcall(root: &serde_json::Value) -> bool {
    let funs = match root.get("functions") {
        Some(v) => v.as_array().cloned().unwrap_or_default(),
        None => return false,
    };
    for f in funs {
        let blocks = match f.get("blocks").and_then(|b| b.as_array()) {
            Some(b) => b,
            None => continue,
        };
        for b in blocks {
            let insts = match b.get("instructions").and_then(|i| i.as_array()) {
                Some(i) => i,
                None => continue,
            };
            for inst in insts {
                let op = inst.get("op").and_then(|v| v.as_str());
                let method = inst.get("method").and_then(|v| v.as_str());
                if op == Some("boxcall") && method == Some("mul_naive") {
                    return true;
                }
            }
        }
    }
    false
}

/// Helper: enforce numeric_core invariants when NYASH_AOT_NUMERIC_CORE=1 is set.
/// - Default: emit a warning if mul_naive boxcalls are still present.
/// - Strict: if NYASH_AOT_NUMERIC_CORE_STRICT=1, return Err to fail fast.
#[allow(dead_code)]
pub(crate) fn check_numeric_core_invariants(root: &serde_json::Value) -> Result<(), String> {
    let numeric_on = matches!(
        std::env::var("NYASH_AOT_NUMERIC_CORE").ok().as_deref(),
        Some("1")
    );
    if !numeric_on {
        return Ok(());
    }

    if !has_numeric_core_boxcall(root) {
        return Ok(());
    }

    let strict = matches!(
        std::env::var("NYASH_AOT_NUMERIC_CORE_STRICT")
            .ok()
            .as_deref(),
        Some("1")
    );

    eprintln!(
        "[mir_json/numeric_core] NYASH_AOT_NUMERIC_CORE=1 but MIR JSON still contains boxcall(\"mul_naive\"). \
AotPrepNumericCoreBox may not have run or did not match; inspect AotPrep logs or run tools/hakorune_emit_mir.sh with HAKO_SELFHOST_TRACE=1."
    );

    if strict {
        return Err(
            "NYASH_AOT_NUMERIC_CORE_STRICT=1: numeric_core invariants violated (mul_naive boxcall remains)"
                .to_string(),
        );
    }
    Ok(())
}
