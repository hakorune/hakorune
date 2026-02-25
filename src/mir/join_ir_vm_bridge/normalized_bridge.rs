#![cfg(feature = "normalized_dev")]

use super::bridge::lower_joinir_structured_to_mir_with_meta;
use super::JoinIrVmBridgeError;
use crate::mir::join_ir::frontend::JoinFuncMetaMap;
use crate::mir::join_ir::{
    normalized_pattern1_to_structured, normalized_pattern2_to_structured, JoinIrPhase,
    NormalizedModule,
};
use crate::mir::MirModule;

mod direct;

fn dev_debug_enabled() -> bool {
    crate::mir::join_ir::normalized::dev_env::normalized_dev_logs_enabled()
}

/// Dev logging helper with unified category prefix.
pub(super) fn log_dev(category: &str, message: impl AsRef<str>, important: bool) {
    let debug = dev_debug_enabled();
    let msg = format!("[joinir/normalized-dev/bridge/{}] {}", category, message.as_ref());
    if important {
        crate::runtime::get_global_ring0().log.info(&msg);
    } else if debug {
        crate::runtime::get_global_ring0().log.debug(&msg);
    }
}

pub(super) fn log_debug(category: &str, message: impl AsRef<str>) {
    log_dev(category, message, false);
}

/// Direct Normalized → MIR 変換が未対応のときに使うフォールバック。
fn lower_normalized_via_structured(
    norm: &NormalizedModule,
    meta: &JoinFuncMetaMap,
) -> Result<MirModule, JoinIrVmBridgeError> {
    let structured = if let Some(snapshot) = norm.to_structured() {
        snapshot
    } else if norm.functions.len() <= 2 {
        normalized_pattern1_to_structured(norm)
    } else {
        normalized_pattern2_to_structured(norm)
    };

    log_dev(
        "fallback",
        format!(
            "using structured path (functions={})",
            structured.functions.len()
        ),
        true,
    );

    lower_joinir_structured_to_mir_with_meta(&structured, meta)
}

/// Dev-only Normalized → MIR ブリッジ（Pattern1/2 ミニ + JP mini/atoi mini 専用）
pub(crate) fn lower_normalized_to_mir_minimal(
    norm: &NormalizedModule,
    meta: &JoinFuncMetaMap,
    allow_structured_fallback: bool,
) -> Result<MirModule, JoinIrVmBridgeError> {
    if norm.phase != JoinIrPhase::Normalized {
        return Err(JoinIrVmBridgeError::new(
            "[joinir/bridge/normalized] expected Normalized JoinIR module",
        ));
    }

    if dev_debug_enabled() {
        log_debug(
            "debug",
            format!(
                "lowering normalized module (functions={}, env_layouts={})",
                norm.functions.len(),
                norm.env_layouts.len()
            ),
        );
        for layout in &norm.env_layouts {
            let fields: Vec<String> = layout
                .fields
                .iter()
                .map(|f| format!("{}={:?}", f.name, f.value_id))
                .collect();
            log_debug(
                "debug",
                format!("env_layout {} fields: {}", layout.id, fields.join(", ")),
            );
        }
        for func in norm.functions.values() {
            log_debug(
                "debug",
                format!(
                    "fn {} (id={:?}) env_layout={:?} body_len={}",
                    func.name,
                    func.id,
                    func.env_layout,
                    func.body.len()
                ),
            );
        }
    }

    // direct 対象は Normalized → MIR をそのまま吐く。未対応 shape は Structured 経由にフォールバック。
    match direct::lower_normalized_direct_minimal(norm) {
        Ok(mir) => Ok(mir),
        Err(err) if allow_structured_fallback => {
            log_dev(
                "fallback",
                format!(
                    "direct path failed: {}; falling back to Structured path",
                    err.message
                ),
                true,
            );
            lower_normalized_via_structured(norm, meta)
        }
        Err(err) => Err(JoinIrVmBridgeError::new(format!(
            "[joinir/normalized-bridge] direct path failed and fallback disabled: {}",
            err.message
        ))),
    }
}
