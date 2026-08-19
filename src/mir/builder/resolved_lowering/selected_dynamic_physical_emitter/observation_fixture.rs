//! Test-only real-source carrier for selected Dynamic lowering inspection.

use crate::mir::{
    BasicBlockId, ConstValue, EffectMask, FunctionSignature, MirFunction, MirInstruction,
    MirModule, MirType, ValueId,
};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::Path;

pub(super) const SOURCE_PATH: &str = "lang/src/compiler/parser/scan/parser_scan_loop_box.hako";
pub(super) const SOURCE: &str =
    include_str!("../../../../../lang/src/compiler/parser/scan/parser_scan_loop_box.hako");
pub(super) const FUNCTION: &str = "ParserScanLoopBox.skip_while/4";

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

fn launch_scaffold() -> MirFunction {
    let mut function = MirFunction::new(
        FunctionSignature {
            name: "main".to_owned(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    );
    let value = ValueId::new(0);
    let entry = function
        .get_block_mut(BasicBlockId::new(0))
        .expect("launch scaffold entry");
    entry.add_instruction(MirInstruction::Const {
        dst: value,
        value: ConstValue::Integer(0),
    });
    entry.set_terminator(MirInstruction::Return { value: Some(value) });
    function
}

pub(super) fn publish(output_dir: &Path, helper: MirFunction) -> Result<(), String> {
    if helper.signature.name != FUNCTION {
        return Err("selected Dynamic observation helper mismatch".to_owned());
    }
    let mut module = MirModule::new("selected-dynamic-inspection".to_owned());
    module
        .try_add_function(launch_scaffold())
        .map_err(|error| error.to_string())?;
    module
        .try_add_function(helper)
        .map_err(|error| error.to_string())?;
    let encoded =
        crate::runner::mir_json_emit::emit_mir_json_string_for_unpublished_candidate(&module)?;
    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;
    let source = output_dir.join("source.full.hako");
    let mir = output_dir.join("real.json");
    let manifest = output_dir.join("producer.json");
    let temporary = output_dir.join("producer.json.tmp");
    std::fs::write(&source, SOURCE.as_bytes()).map_err(|error| error.to_string())?;
    std::fs::write(&mir, encoded.as_bytes()).map_err(|error| error.to_string())?;
    let value = json!({
        "output_contract": "hako-inspect-selected-dynamic-producer-v1",
        "source_kind": "source_backed_fixture",
        "launch_kind": "route_admission_scaffold_non_authority",
        "source_path": SOURCE_PATH,
        "source_file": "source.full.hako",
        "source_sha256": sha256(SOURCE.as_bytes()),
        "mir_json_file": "real.json",
        "mir_json_sha256": sha256(encoded.as_bytes()),
        "mir_function": FUNCTION,
        "summary": "ok",
    });
    let mut bytes = serde_json::to_vec_pretty(&value).map_err(|error| error.to_string())?;
    bytes.push(b'\n');
    std::fs::write(&temporary, bytes).map_err(|error| error.to_string())?;
    std::fs::rename(temporary, manifest).map_err(|error| error.to_string())
}
