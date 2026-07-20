use cfg_expr::{targets::get_builtin_target_by_triple, Expression, Predicate, TargetPredicate};
use quote::ToTokens;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Meta, Token};

use super::error::CfgDecisionErrorV1;
use super::model::{
    CargoTargetKindV1, CfgDecisionStateV1, CfgDecisionV1, CfgEvaluationEnvironmentV1,
    CfgRowDecisionV1,
};

pub fn decide_cfg_rows_v1(
    rows: &[String],
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<CfgDecisionV1, CfgDecisionErrorV1> {
    if environment.profile_id.is_empty() {
        return Err(CfgDecisionErrorV1::EmptyProfileId);
    }
    let target = get_builtin_target_by_triple(&environment.target_triple).ok_or_else(|| {
        CfgDecisionErrorV1::UnsupportedTargetTriple {
            target_triple: environment.target_triple.clone(),
        }
    })?;
    let mut decisions = Vec::with_capacity(rows.len());
    let mut combined = CfgDecisionStateV1::Included;
    for syntax in rows {
        let meta = syn::parse_str::<Meta>(syntax).map_err(|error| {
            CfgDecisionErrorV1::MalformedAttribute {
                syntax: syntax.clone(),
                detail: error.to_string(),
            }
        })?;
        let mut unknown = Vec::new();
        let state = eval_meta(&meta, syntax, environment, target, &mut unknown)?;
        unknown.sort();
        unknown.dedup();
        decisions.push(CfgRowDecisionV1 {
            syntax: syntax.clone(),
            state,
            unknown_predicates: unknown.into_boxed_slice(),
        });
        combined = and_state(combined, state);
    }
    Ok(CfgDecisionV1 {
        profile_id: environment.profile_id.clone(),
        state: combined,
        rows: decisions.into_boxed_slice(),
    })
}

fn eval_meta(
    meta: &Meta,
    syntax: &str,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
    unknown: &mut Vec<String>,
) -> Result<CfgDecisionStateV1, CfgDecisionErrorV1> {
    if meta.path().is_ident("cfg") {
        let Meta::List(list) = meta else {
            return Err(CfgDecisionErrorV1::MalformedCfgExpression {
                syntax: syntax.to_string(),
                detail: "cfg must be a list".to_string(),
            });
        };
        let normalized = format!("cfg({})", list.tokens);
        return eval_cfg_expression(&normalized, environment, target, unknown);
    }
    if meta.path().is_ident("cfg_attr") {
        let Meta::List(list) = meta else {
            return Err(CfgDecisionErrorV1::MalformedCfgAttr {
                syntax: syntax.to_string(),
                detail: "cfg_attr must be a list".to_string(),
            });
        };
        return eval_cfg_attr(list, syntax, environment, target, unknown);
    }
    Ok(CfgDecisionStateV1::Included)
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

fn eval_cfg_attr(
    list: &syn::MetaList,
    syntax: &str,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
    unknown: &mut Vec<String>,
) -> Result<CfgDecisionStateV1, CfgDecisionErrorV1> {
    let nested = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(list.tokens.clone())
        .map_err(|error| CfgDecisionErrorV1::MalformedCfgAttr {
            syntax: syntax.to_string(),
            detail: error.to_string(),
        })?;
    let mut rows = nested.iter();
    let Some(condition) = rows.next() else {
        return Err(CfgDecisionErrorV1::MalformedCfgAttr {
            syntax: syntax.to_string(),
            detail: "cfg_attr requires a condition".to_string(),
        });
    };
    let condition_syntax = format!("cfg({})", condition.to_token_stream());
    let condition_state = eval_cfg_expression(&condition_syntax, environment, target, unknown)?;
    let mut result = CfgDecisionStateV1::Included;
    for nested_meta in rows {
        let nested_state = if nested_meta.path().is_ident("path") {
            match condition_state {
                CfgDecisionStateV1::Unknown => {
                    unknown.push(format!("cfg_attr:path:{}", nested_meta.to_token_stream()));
                    CfgDecisionStateV1::Unknown
                }
                CfgDecisionStateV1::Included | CfgDecisionStateV1::Excluded => {
                    CfgDecisionStateV1::Included
                }
            }
        } else if nested_meta.path().is_ident("cfg") || nested_meta.path().is_ident("cfg_attr") {
            let nested_syntax = nested_meta.to_token_stream().to_string();
            let nested_decision =
                eval_meta(nested_meta, &nested_syntax, environment, target, unknown)?;
            implication_state(condition_state, nested_decision)
        } else {
            CfgDecisionStateV1::Included
        };
        result = and_state(result, nested_state);
    }
    Ok(result)
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

fn implication_state(
    condition: CfgDecisionStateV1,
    nested: CfgDecisionStateV1,
) -> CfgDecisionStateV1 {
    match condition {
        CfgDecisionStateV1::Excluded => CfgDecisionStateV1::Included,
        CfgDecisionStateV1::Included => nested,
        CfgDecisionStateV1::Unknown => match nested {
            CfgDecisionStateV1::Included => CfgDecisionStateV1::Included,
            CfgDecisionStateV1::Excluded | CfgDecisionStateV1::Unknown => {
                CfgDecisionStateV1::Unknown
            }
        },
    }
}

fn and_state(left: CfgDecisionStateV1, right: CfgDecisionStateV1) -> CfgDecisionStateV1 {
    match (left, right) {
        (CfgDecisionStateV1::Excluded, _) | (_, CfgDecisionStateV1::Excluded) => {
            CfgDecisionStateV1::Excluded
        }
        (CfgDecisionStateV1::Included, CfgDecisionStateV1::Included) => {
            CfgDecisionStateV1::Included
        }
        _ => CfgDecisionStateV1::Unknown,
    }
}

fn option_state(value: Option<bool>) -> CfgDecisionStateV1 {
    match value {
        Some(true) => CfgDecisionStateV1::Included,
        Some(false) => CfgDecisionStateV1::Excluded,
        None => CfgDecisionStateV1::Unknown,
    }
}
