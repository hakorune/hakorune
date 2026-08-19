//! Test-only observation carrier for the source-backed S6C real candidate.

use crate::mir::MirFunction;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::path::Path;

pub(super) const S6C_SOURCE_PATH: &str = "apps/tests/scan_with_init_typed_ok_min.hako";
pub(super) const S6C_SOURCE: &str =
    include_str!("../../../../apps/tests/scan_with_init_typed_ok_min.hako");

pub(super) struct PinnedTextRealObservationCandidate {
    source_path: &'static str,
    source_bytes: &'static [u8],
    function: MirFunction,
}

impl PinnedTextRealObservationCandidate {
    pub(super) fn new(function: MirFunction) -> Self {
        Self {
            source_path: S6C_SOURCE_PATH,
            source_bytes: S6C_SOURCE.as_bytes(),
            function,
        }
    }

    pub(super) fn function(&self) -> &MirFunction {
        &self.function
    }

    pub(super) fn into_parts(self) -> (&'static str, &'static [u8], MirFunction) {
        (self.source_path, self.source_bytes, self.function)
    }
}

fn sha256(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(super) fn publish_observation_cohort(
    output_dir: &Path,
    source_path_value: &str,
    source_bytes: &[u8],
    mir_function: &str,
    encoded: &str,
) -> Result<(), String> {
    std::fs::create_dir_all(output_dir).map_err(|error| error.to_string())?;
    let source_path = output_dir.join("source.full.hako");
    let json_path = output_dir.join("real.json");
    let manifest_path = output_dir.join("producer.json");
    let manifest_tmp = output_dir.join("producer.json.tmp");
    let _ = std::fs::remove_file(&manifest_path);
    let _ = std::fs::remove_file(&manifest_tmp);
    std::fs::write(&source_path, source_bytes).map_err(|error| error.to_string())?;
    std::fs::write(&json_path, encoded.as_bytes()).map_err(|error| error.to_string())?;
    let manifest = json!({
        "output_contract": "hako-inspect-s6c-producer-v1",
        "source_kind": "source_backed_fixture",
        "source_path": source_path_value,
        "source_file": "source.full.hako",
        "source_sha256": sha256(source_bytes),
        "mir_json_file": "real.json",
        "mir_json_sha256": sha256(encoded.as_bytes()),
        "mir_function": mir_function,
        "summary": "ok",
    });
    let bytes = serde_json::to_vec_pretty(&manifest).map_err(|error| error.to_string())?;
    std::fs::write(&manifest_tmp, [bytes, b"\n".to_vec()].concat())
        .map_err(|error| error.to_string())?;
    std::fs::rename(manifest_tmp, manifest_path).map_err(|error| error.to_string())
}
