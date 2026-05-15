/*!
 * External Call Handling
 *
 * Manages env.* methods and external interface calls
 * Provides bridge to host environment functionality
 */

use crate::mir::{Effect, EffectMask, MirType};

/// Table-like spec for env.* methods
/// Returns (iface_name, method_name, effects, returns_value)
pub fn get_env_method_spec(
    iface: &str,
    method: &str,
) -> Option<(String, String, EffectMask, bool)> {
    match (iface, method) {
        // Future/async operations
        ("future", "delay") => Some((
            "env.future".to_string(),
            "delay".to_string(),
            EffectMask::READ.add(Effect::Io),
            true,
        )),
        ("future", "spawn") => Some((
            "env.future".to_string(),
            "spawn".to_string(),
            EffectMask::IO,
            true,
        )),

        // Task management
        ("task", "currentToken") => Some((
            "env.task".to_string(),
            "currentToken".to_string(),
            EffectMask::READ,
            true,
        )),
        ("task", "cancelCurrent") => Some((
            "env.task".to_string(),
            "cancelCurrent".to_string(),
            EffectMask::IO,
            false,
        )),

        // Console I/O
        ("console", "log") => Some((
            "env.console".to_string(),
            "log".to_string(),
            EffectMask::IO,
            false,
        )),
        ("console", "readLine") => Some((
            "env.console".to_string(),
            "readLine".to_string(),
            EffectMask::IO,
            true,
        )),
        ("console", "error") => Some((
            "env.console".to_string(),
            "error".to_string(),
            EffectMask::IO,
            false,
        )),

        // Canvas operations
        ("canvas", m) if matches!(m, "fillRect" | "fillText" | "clear") => Some((
            "env.canvas".to_string(),
            method.to_string(),
            EffectMask::IO,
            false,
        )),

        // Backend/codegen operations
        ("codegen", "emit_object") => Some((
            "env.codegen".to_string(),
            "emit_object".to_string(),
            EffectMask::IO,
            true,
        )),
        ("codegen", "compile_ll_text") => Some((
            "env.codegen".to_string(),
            "compile_ll_text".to_string(),
            EffectMask::IO,
            true,
        )),
        ("codegen", "link_object") => Some((
            "env.codegen".to_string(),
            "link_object".to_string(),
            EffectMask::IO,
            true,
        )),

        // File system
        ("fs", "readFile") => Some((
            "env.fs".to_string(),
            "readFile".to_string(),
            EffectMask::IO,
            true,
        )),
        ("fs", "writeFile") => Some((
            "env.fs".to_string(),
            "writeFile".to_string(),
            EffectMask::IO,
            false,
        )),
        ("fs", "exists") => Some((
            "env.fs".to_string(),
            "exists".to_string(),
            EffectMask::READ,
            true,
        )),

        // Network
        ("net", "fetch") => Some((
            "env.net".to_string(),
            "fetch".to_string(),
            EffectMask::IO,
            true,
        )),
        ("net", "listen") => Some((
            "env.net".to_string(),
            "listen".to_string(),
            EffectMask::IO,
            true,
        )),

        // Process/system
        ("process", "exit") => Some((
            "env.process".to_string(),
            "exit".to_string(),
            EffectMask::IO.add(Effect::Control),
            false,
        )),
        ("process", "argv") => Some((
            "env.process".to_string(),
            "argv".to_string(),
            EffectMask::READ,
            true,
        )),
        ("process", "env") => Some((
            "env.process".to_string(),
            "env".to_string(),
            EffectMask::READ,
            true,
        )),

        // Direct env access
        ("env", "get") => Some(("env".to_string(), "get".to_string(), EffectMask::READ, true)),
        ("env", "set") => Some(("env".to_string(), "set".to_string(), EffectMask::IO, false)),
        // Convenience aliases
        ("env", "error") => Some((
            "env.console".to_string(),
            "error".to_string(),
            EffectMask::IO,
            false,
        )),

        // Unknown
        _ => None,
    }
}

/// Conservative return type hints for env.* methods.
///
/// This is used only by the plan normalizer path to avoid inventing
/// integer-typed placeholders for extern values that are actually stringish
/// or nullable at runtime.
pub fn get_env_method_return_type(iface: &str, method: &str) -> Option<MirType> {
    match (iface, method) {
        ("env", "get") => Some(MirType::Unknown),
        ("env", "set") => Some(MirType::Void),
        ("console", "readLine") => Some(MirType::String),
        ("fs", "readFile") => Some(MirType::String),
        ("fs", "exists") => Some(MirType::Bool),
        ("process", "argv") => Some(MirType::Unknown),
        ("process", "env") => Some(MirType::Unknown),
        ("now_ms", _) => None,
        _ => Some(MirType::Unknown),
    }
}

/// Split the source-level explicit externcall target.
///
/// `externcall "hako_mem_alloc"(...)` is intentionally kept as the exact
/// extern symbol rather than defaulting to an interface prefix. Dotted names
/// still split into `<iface>.<method>` for canonical MIR extern calls.
pub fn split_explicit_extern_name(name: &str) -> (String, String) {
    match name.rsplit_once('.') {
        Some((iface, method)) if !iface.is_empty() && !method.is_empty() => {
            (iface.to_string(), method.to_string())
        }
        _ => ("".to_string(), name.to_string()),
    }
}

/// Return type hints for source-level `externcall "symbol"(...)`.
///
/// This is shared by the direct MirBuilder path and the JoinIR/loop plan
/// normalizer so explicit externcall keeps the same MIR type facts in both
/// lowering lanes.
pub fn explicit_extern_return_type(name: &str) -> MirType {
    match name {
        "hako_mem_alloc"
        | "hako_atomic_ptr_cas_ordered"
        | "hako_atomic_ptr_load_ordered"
        | "hako_atomic_ptr_store_ordered"
        | "hako_atomic_slot_cas_i64"
        | "hako_atomic_slot_fetch_add_i64"
        | "hako_atomic_slot_load_i64"
        | "hako_atomic_slot_store_i64"
        | "hako_mem_realloc"
        | "hako_mem_free"
        | "hako_osvm_page_size_i64"
        | "hako_osvm_commit_bytes_i64"
        | "hako_osvm_decommit_bytes_i64"
        | "hako_osvm_reserve_bytes_i64"
        | "hako_tls_cache_slot_get_i64"
        | "hako_tls_cache_slot_set_i64"
        | "hako_worker_current_id_i64" => MirType::Integer,
        "nyash.box.from_i8_string" => MirType::Box("StringBox".to_string()),
        _ => MirType::Unknown,
    }
}

/// Parse external call name into interface and method
/// E.g., "nyash.builtin.print" -> ("nyash.builtin", "print")
pub fn parse_extern_name(name: &str) -> (String, String) {
    let parts: Vec<&str> = name.rsplitn(2, '.').collect();
    if parts.len() == 2 {
        (parts[1].to_string(), parts[0].to_string())
    } else {
        ("nyash".to_string(), name.to_string())
    }
}

/// Check if a name refers to an environment interface
#[allow(dead_code)]
pub fn is_env_interface(name: &str) -> bool {
    matches!(
        name,
        "env"
            | "env.console"
            | "env.codegen"
            | "env.fs"
            | "env.net"
            | "env.canvas"
            | "env.task"
            | "env.future"
            | "env.process"
    )
}

/// Determine effects for an external call
pub fn compute_extern_effects(iface: &str, method: &str) -> EffectMask {
    match (iface, method) {
        // Pure reads
        (_, m) if m.starts_with("get") || m == "argv" || m == "env" => EffectMask::READ,
        // Control flow changes
        (_, "exit") | (_, "panic") | (_, "throw") => EffectMask::IO.add(Effect::Control),
        // Memory allocation
        (_, m) if m.starts_with("new") || m.starts_with("create") => {
            EffectMask::IO.add(Effect::Alloc)
        }
        // Default to I/O
        _ => EffectMask::IO,
    }
}
