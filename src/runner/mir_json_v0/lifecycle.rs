use crate::mir::{MirInstruction, ValueId};
use serde_json::Value;

use super::helpers::{parse_value_id_array, require_u64};

/// Parse value transport and lifecycle opcodes without owning route policy.
///
/// The caller retains block insertion and ValueId high-water accounting. This
/// box only preserves the existing JSON-v0 field validation and instruction
/// construction behavior.
pub(super) fn parse_value_transport_or_lifecycle(
    op: &str,
    instruction: &Value,
) -> Result<MirInstruction, String> {
    match op {
        "copy" => {
            let dst = require_u64(instruction, "dst", "copy dst")? as u32;
            let src = require_u64(instruction, "src", "copy src")? as u32;
            Ok(MirInstruction::Copy {
                dst: ValueId::new(dst),
                src: ValueId::new(src),
            })
        }
        "keepalive" => {
            let values = parse_value_id_array(instruction, "values", "keepalive value")?;
            Ok(MirInstruction::KeepAlive { values })
        }
        "release_strong" => {
            let values = parse_value_id_array(instruction, "values", "release_strong value")?;
            Ok(MirInstruction::ReleaseStrong { values })
        }
        _ => Err(format!("unsupported value transport/lifecycle op '{}'", op)),
    }
}
