use super::*;
use serde_json::Value as JsonValue;

mod hostbridge;
mod codegen;
mod lane;
mod loader_cold;
mod runtime_direct;
#[cfg(test)]
mod tests;

impl MirInterpreter {
    #[inline]
    fn should_trace_call_extern(target: &str, method: &str) -> bool {
        if let Ok(flt) = std::env::var("HAKO_CALL_TRACE_FILTER") {
            let key = format!("{}.{}", target, method);
            for pat in flt.split(',') {
                let p = pat.trim();
                if p.is_empty() {
                    continue;
                }
                if p == method || p == key {
                    return true;
                }
            }
            return false;
        }
        true
    }

    fn patch_mir_json_version(s: &str) -> String {
        match serde_json::from_str::<JsonValue>(s) {
            Ok(mut v) => {
                if let JsonValue::Object(ref mut m) = v {
                    if !m.contains_key("version") {
                        m.insert("version".to_string(), JsonValue::from(0));
                        if let Ok(out) = serde_json::to_string(&v) {
                            return out;
                        }
                    }
                }
                s.to_string()
            }
            Err(_) => s.to_string(),
        }
    }

    fn emit_mirbuilder_program_json(&mut self, program_json: &str) -> Result<VMValue, VMError> {
        match crate::runtime::mirbuilder_emit::emit_program_json_to_mir_json_with_env_imports(
            program_json,
        ) {
            Ok(out) => Ok(VMValue::String(Self::patch_mir_json_version(&out))),
            Err(e) => Err(self.err_with_context("env.mirbuilder.emit", &e.to_string())),
        }
    }

    /// Central extern dispatcher used by both execute_extern_function (calls.rs)
    /// and handle_extern_call (externals.rs). Returns a VMValue; callers are
    /// responsible for writing it to registers when needed.
    pub(super) fn extern_provider_dispatch(
        &mut self,
        extern_name: &str,
        args: &[ValueId],
    ) -> Option<Result<VMValue, VMError>> {
        // Unified call trace (optional)
        if std::env::var("HAKO_CALL_TRACE").ok().as_deref() == Some("1") {
            // Split iface.method for filtering
            if let Some((iface, method)) = extern_name.rsplit_once('.') {
                if Self::should_trace_call_extern(iface, method) {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[call:{}.{}]", iface, method));
                }
            } else {
                // Fallback: no dot in extern name (e.g., 'print')
                if Self::should_trace_call_extern("", extern_name) {
                    crate::runtime::get_global_ring0()
                        .log
                        .debug(&format!("[call:{}]", extern_name));
                }
            }
        }
        match lane::classify_extern_provider_lane(extern_name) {
            Some(lane::ExternProviderLane::RuntimeDirect) => {
                Some(self.dispatch_runtime_direct_extern(extern_name, args))
            }
            Some(lane::ExternProviderLane::LoaderCold) => {
                Some(self.dispatch_loader_cold_extern(extern_name, args))
            }
            None => None,
        }
    }
}
