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

