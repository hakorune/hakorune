use std::collections::{BTreeMap, BTreeSet};

use cfg_expr::{targets::get_builtin_target_by_triple, Expression, Predicate, TargetPredicate};

use super::cargo::CargoDeclaredUnitProcessEvidenceV1;
use super::error::CfgDecisionErrorV1;
use super::model::{CargoTargetKindV1, CfgDecisionStateV1, CfgEvaluationEnvironmentV1};

/// Builds one exact CFG environment from sealed Cargo and rustc evidence.
///
/// This conversion is shared by the legacy module gate and disconnected
/// source-surface proofs. Neither consumer may hand-assemble feature or
/// target facts from a profile label.
pub fn cfg_environment_from_declared_unit_evidence_v1(
    evidence: &CargoDeclaredUnitProcessEvidenceV1,
) -> CfgEvaluationEnvironmentV1 {
    let declared = evidence.declared_unit();
    let probe = evidence.rustc_cfg_probe();
    let known_flags = probe
        .cfg_flags()
        .iter()
        .cloned()
        .map(|flag| (flag, true))
        .collect::<BTreeMap<_, _>>();
    let known_key_values = probe
        .cfg_key_values()
        .iter()
        .map(|(key, values)| (key.clone(), values.iter().cloned().collect()))
        .collect::<BTreeMap<String, BTreeSet<String>>>();
    let target_features = known_key_values
        .get("target_feature")
        .cloned()
        .unwrap_or_default();
    CfgEvaluationEnvironmentV1 {
        profile_id: declared.profile_id().to_string(),
        target_kind: declared.target().semantic_kind(),
        target_triple: probe.target_triple().to_string(),
        activated_features: declared
            .cargo_resolved_root_features()
            .iter()
            .cloned()
            .collect(),
        test_cfg: probe.cfg_flags().iter().any(|flag| flag == "test"),
        debug_assertions: probe
            .cfg_flags()
            .iter()
            .any(|flag| flag == "debug_assertions"),
        target_features,
        target_features_sealed: true,
        target_predicates_sealed: true,
        known_flags,
        known_key_values,
    }
}

/// Validates the sealed build inputs shared by legacy and ordered CFG owners.
pub(super) fn validate_cfg_environment_v1(
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<&'static cfg_expr::targets::TargetInfo, CfgDecisionErrorV1> {
    if environment.profile_id.is_empty() {
        return Err(CfgDecisionErrorV1::EmptyProfileId);
    }
    get_builtin_target_by_triple(&environment.target_triple).ok_or_else(|| {
        CfgDecisionErrorV1::UnsupportedTargetTriple {
            target_triple: environment.target_triple.clone(),
        }
    })
}

/// Evaluates one normalized `cfg(...)` predicate without owning row order.
pub(super) fn decide_cfg_predicate_syntax_v1(
    syntax: &str,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
) -> Result<(CfgDecisionStateV1, Box<[String]>), CfgDecisionErrorV1> {
    let mut unknown = Vec::new();
    let state = eval_cfg_expression(syntax, environment, target, &mut unknown)?;
    unknown.sort();
    unknown.dedup();
    Ok((state, unknown.into_boxed_slice()))
}

fn eval_cfg_expression(
    syntax: &str,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
    unknown: &mut Vec<String>,
) -> Result<CfgDecisionStateV1, CfgDecisionErrorV1> {
    let expression =
        Expression::parse(syntax).map_err(|error| CfgDecisionErrorV1::MalformedCfgExpression {
            syntax: syntax.to_string(),
            detail: error.to_string(),
        })?;
    let value = expression.eval::<_, Option<bool>>(|predicate| {
        eval_predicate(predicate, environment, target, unknown)
    });
    Ok(option_state(value))
}

fn eval_predicate(
    predicate: &Predicate<'_>,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
    unknown: &mut Vec<String>,
) -> Option<bool> {
    match predicate {
        Predicate::Target(target_predicate) if environment.target_predicates_sealed => {
            Some(matches_sealed_target(target_predicate, environment))
        }
        Predicate::Target(target_predicate) => Some(target_predicate.matches(target)),
        Predicate::Test => Some(environment.test_cfg),
        Predicate::DebugAssertions => Some(environment.debug_assertions),
        Predicate::ProcMacro => Some(environment.target_kind == CargoTargetKindV1::ProcMacro),
        Predicate::Feature(feature) => Some(environment.activated_features.contains(*feature)),
        Predicate::TargetFeature(feature) if environment.target_features_sealed => {
            Some(environment.target_features.contains(*feature))
        }
        Predicate::TargetFeature(feature) => {
            unknown.push(format!("target_feature={feature}"));
            None
        }
        Predicate::Flag(flag) => match environment.known_flags.get(*flag) {
            Some(value) => Some(*value),
            None => {
                unknown.push(format!("flag={flag}"));
                None
            }
        },
        Predicate::KeyValue { key, val } => match environment.known_key_values.get(*key) {
            Some(values) => Some(values.contains(*val)),
            None => {
                unknown.push(format!("key_value={key}={val}"));
                None
            }
        },
    }
}

fn matches_sealed_target(
    predicate: &TargetPredicate,
    environment: &CfgEvaluationEnvironmentV1,
) -> bool {
    let has_value = |key: &str, value: &str| {
        environment
            .known_key_values
            .get(key)
            .is_some_and(|values| values.contains(value))
    };
    match predicate {
        TargetPredicate::Abi(value) => has_value("target_abi", value.as_str()),
        TargetPredicate::Arch(value) => has_value("target_arch", value.as_str()),
        TargetPredicate::Endian(value) => has_value(
            "target_endian",
            match value {
                cfg_expr::targets::Endian::big => "big",
                cfg_expr::targets::Endian::little => "little",
            },
        ),
        TargetPredicate::Env(value) => has_value("target_env", value.as_str()),
        TargetPredicate::Family(value) => {
            has_value("target_family", value.as_str())
                || environment.known_flags.get(value.as_str()) == Some(&true)
        }
        TargetPredicate::HasAtomic(value) => has_value("target_has_atomic", &value.to_string()),
        TargetPredicate::Os(value) => has_value("target_os", value.as_str()),
        TargetPredicate::Panic(value) => has_value("panic", value.as_str()),
        TargetPredicate::PointerWidth(value) => {
            has_value("target_pointer_width", &value.to_string())
        }
        TargetPredicate::Vendor(value) => has_value("target_vendor", value.as_str()),
    }
}

fn option_state(value: Option<bool>) -> CfgDecisionStateV1 {
    match value {
        Some(true) => CfgDecisionStateV1::Included,
        Some(false) => CfgDecisionStateV1::Excluded,
        None => CfgDecisionStateV1::Unknown,
    }
}
