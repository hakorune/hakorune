/*!
 * VM source preflight helper.
 *
 * Purpose:
 * - Keep `vm.rs` focused on route execution instead of source read / source prepare.
 * - Own the source-read + source_hint + safety-gate preflight for VM lanes.
 */

use crate::runner::NyashRunner;
use std::collections::HashMap;

pub(crate) struct PreparedVmSource {
    pub(crate) code_final: String,
    pub(crate) using_imports: HashMap<String, String>,
}

pub(crate) fn prepare_vm_source(runner: &NyashRunner, filename: &str) -> Option<PreparedVmSource> {
    let code = match std::fs::read_to_string(filename) {
        Ok(content) => content,
        Err(e) => {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            ring0
                .log
                .error(&format!("❌ Error reading file {}: {}", filename, e));
            return None;
        }
    };

    let prepared = match crate::runner::modes::common_util::source_hint::prepare_source_with_imports(
        runner, filename, &code,
    ) {
        Ok(prepared) => prepared,
        Err(e) => {
            let ring0 = crate::runtime::ring0::get_global_ring0();
            let msg = if e.starts_with("[freeze:contract][module_registry]") {
                e
            } else {
                format!("❌ {}", e)
            };
            ring0.log.error(&msg);
            return None;
        }
    };

    crate::runner::modes::common_util::safety_gate::enforce_vm_source_safety_or_exit(
        &prepared.code,
        "vm",
    );

    Some(PreparedVmSource {
        code_final: prepared.code,
        using_imports: prepared.imports,
    })
}
