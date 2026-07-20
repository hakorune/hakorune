use std::collections::{BTreeMap, BTreeSet};
use std::process::Command;

use serde::Serialize;

use super::fingerprint::sha256_bytes;
use super::{
    CargoCompileModeV1, CargoProcessEvidenceErrorV1, CargoTargetKindV1,
    ValidatedBuildProfileInputV1,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RustcCfgProbeEvidenceV1 {
    profile_id: String,
    target_triple: String,
    rustc_version_sha256: String,
    probe_arguments: Box<[String]>,
    cfg_output_sha256: String,
    cfg_flags: Box<[String]>,
    cfg_key_values: BTreeMap<String, Box<[String]>>,
}

impl RustcCfgProbeEvidenceV1 {
    pub fn profile_id(&self) -> &str {
        &self.profile_id
    }

    pub fn target_triple(&self) -> &str {
        &self.target_triple
    }

    pub fn rustc_version_sha256(&self) -> &str {
        &self.rustc_version_sha256
    }

    pub fn cfg_flags(&self) -> &[String] {
        &self.cfg_flags
    }

    pub fn cfg_values(&self, key: &str) -> Option<&[String]> {
        self.cfg_key_values.get(key).map(AsRef::as_ref)
    }
}

pub fn collect_rustc_cfg_probe_v1(
    profile: &ValidatedBuildProfileInputV1,
    cargo_resolved_root_features: &[String],
) -> Result<RustcCfgProbeEvidenceV1, CargoProcessEvidenceErrorV1> {
    let features = exact_strings(cargo_resolved_root_features)?;
    let version = run_rustc(&["-vV".to_string()], &profile.profile_id, "version")?;
    let arguments = probe_arguments(profile, &features);
    let output = run_rustc(&arguments, &profile.profile_id, "cfg")?;
    let version_after = run_rustc(&["-vV".to_string()], &profile.profile_id, "version")?;
    if version != version_after {
        return Err(CargoProcessEvidenceErrorV1::RustcExecutableChangedDuringObservation);
    }
    let (flags, key_values, normalized) = parse_cfg_output(&output)?;
    validate_probe_contract(profile, &features, &flags, &key_values)?;
    Ok(RustcCfgProbeEvidenceV1 {
        profile_id: profile.profile_id.clone(),
        target_triple: profile.target_triple.clone(),
        rustc_version_sha256: sha256_bytes(&version),
        probe_arguments: arguments.into_boxed_slice(),
        cfg_output_sha256: sha256_bytes(normalized.as_bytes()),
        cfg_flags: flags.into_iter().collect::<Vec<_>>().into_boxed_slice(),
        cfg_key_values: key_values
            .into_iter()
            .map(|(key, values)| {
                (
                    key,
                    values.into_iter().collect::<Vec<_>>().into_boxed_slice(),
                )
            })
            .collect(),
    })
}

fn probe_arguments(
    profile: &ValidatedBuildProfileInputV1,
    features: &BTreeSet<String>,
) -> Vec<String> {
    let mut arguments = vec![
        "--print".to_string(),
        "cfg".to_string(),
        "--target".to_string(),
        profile.target_triple.clone(),
        "--crate-name".to_string(),
        "rust_source_topology_probe".to_string(),
    ];
    match profile.compile_mode {
        CargoCompileModeV1::UnitTestHarness | CargoCompileModeV1::IntegrationTestTarget => {
            arguments.push("--test".to_string());
        }
        CargoCompileModeV1::Normal => {
            arguments.push("--crate-type".to_string());
            arguments.push(crate_type(profile.target_kind).to_string());
        }
    }
    arguments.extend([
        "-C".to_string(),
        format!(
            "debug-assertions={}",
            if profile.debug_assertions {
                "yes"
            } else {
                "no"
            }
        ),
        "-C".to_string(),
        format!("panic={}", profile.panic_strategy),
    ]);
    for feature in features {
        arguments.push("--cfg".to_string());
        arguments.push(format!("feature=\"{feature}\""));
    }
    arguments
}

fn crate_type(kind: CargoTargetKindV1) -> &'static str {
    match kind {
        CargoTargetKindV1::Library => "rlib",
        CargoTargetKindV1::ProcMacro => "proc-macro",
        CargoTargetKindV1::Binary
        | CargoTargetKindV1::IntegrationTest
        | CargoTargetKindV1::Example
        | CargoTargetKindV1::BuildScript => "bin",
    }
}

fn run_rustc(
    arguments: &[String],
    profile_id: &str,
    operation: &'static str,
) -> Result<Vec<u8>, CargoProcessEvidenceErrorV1> {
    let mut command = Command::new("rustc");
    command.args(arguments);
    for key in [
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "RUSTC_BOOTSTRAP",
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
    ] {
        command.env_remove(key);
    }
    let output = command.output().map_err(|error| match operation {
        "version" => CargoProcessEvidenceErrorV1::RustcVersionFailed {
            detail: error.kind().to_string(),
        },
        _ => CargoProcessEvidenceErrorV1::RustcCfgProbeFailed {
            profile_id: profile_id.to_string(),
            detail: error.kind().to_string(),
        },
    })?;
    if !output.status.success() {
        return Err(match operation {
            "version" => CargoProcessEvidenceErrorV1::RustcVersionFailed {
                detail: format!("status={}", output.status),
            },
            _ => CargoProcessEvidenceErrorV1::RustcCfgProbeFailed {
                profile_id: profile_id.to_string(),
                detail: format!("status={}", output.status),
            },
        });
    }
    Ok(output.stdout)
}

type ParsedCfgV1 = (BTreeSet<String>, BTreeMap<String, BTreeSet<String>>, String);

fn parse_cfg_output(output: &[u8]) -> Result<ParsedCfgV1, CargoProcessEvidenceErrorV1> {
    let text = std::str::from_utf8(output).map_err(|_| {
        CargoProcessEvidenceErrorV1::RustcCfgMalformed {
            line: "non-utf8".to_string(),
        }
    })?;
    let mut flags = BTreeSet::new();
    let mut key_values = BTreeMap::<String, BTreeSet<String>>::new();
    let mut normalized_lines = BTreeSet::new();
    for raw in text.lines() {
        let line = raw.trim();
        if line.is_empty() {
            continue;
        }
        normalized_lines.insert(line.to_string());
        if let Some((key, quoted)) = line.split_once('=') {
            let value = quoted
                .strip_prefix('"')
                .and_then(|text| text.strip_suffix('"'));
            let Some(value) = value else {
                return Err(CargoProcessEvidenceErrorV1::RustcCfgMalformed {
                    line: line.to_string(),
                });
            };
            key_values
                .entry(key.to_string())
                .or_default()
                .insert(value.to_string());
        } else {
            flags.insert(line.to_string());
        }
    }
    let normalized = normalized_lines.into_iter().collect::<Vec<_>>().join("\n") + "\n";
    Ok((flags, key_values, normalized))
}

fn validate_probe_contract(
    profile: &ValidatedBuildProfileInputV1,
    features: &BTreeSet<String>,
    flags: &BTreeSet<String>,
    key_values: &BTreeMap<String, BTreeSet<String>>,
) -> Result<(), CargoProcessEvidenceErrorV1> {
    require_bool_fact(
        profile,
        "debug_assertions",
        flags.contains("debug_assertions"),
        profile.debug_assertions,
    )?;
    require_bool_fact(profile, "test", flags.contains("test"), profile.test_cfg)?;
    let panic = key_values.get("panic").cloned().unwrap_or_default();
    if panic != BTreeSet::from([profile.panic_strategy.clone()]) {
        return Err(CargoProcessEvidenceErrorV1::RustcCfgContractMismatch {
            profile_id: profile.profile_id.clone(),
            fact: "panic",
        });
    }
    let actual_features = key_values.get("feature").cloned().unwrap_or_default();
    if &actual_features != features {
        return Err(CargoProcessEvidenceErrorV1::RustcCfgContractMismatch {
            profile_id: profile.profile_id.clone(),
            fact: "feature",
        });
    }
    Ok(())
}

fn require_bool_fact(
    profile: &ValidatedBuildProfileInputV1,
    fact: &'static str,
    actual: bool,
    expected: bool,
) -> Result<(), CargoProcessEvidenceErrorV1> {
    if actual != expected {
        return Err(CargoProcessEvidenceErrorV1::RustcCfgContractMismatch {
            profile_id: profile.profile_id.clone(),
            fact,
        });
    }
    Ok(())
}

fn exact_strings(values: &[String]) -> Result<BTreeSet<String>, CargoProcessEvidenceErrorV1> {
    let mut exact = BTreeSet::new();
    for value in values {
        if value.contains('"') || value.contains('\\') || !exact.insert(value.clone()) {
            return Err(CargoProcessEvidenceErrorV1::RustcCfgContractMismatch {
                profile_id: "input".to_string(),
                fact: "feature-input",
            });
        }
    }
    Ok(exact)
}
