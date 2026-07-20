use std::collections::{BTreeMap, BTreeSet};

use quote::ToTokens;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Attribute, Meta, Token};

use crate::project::cargo::CargoDeclaredUnitProcessEvidenceV1;
use crate::project::{
    decide_cfg_rows_v1, CfgDecisionStateV1, CfgDecisionV1, CfgEvaluationEnvironmentV1,
};

use super::declarations::direct_path_literal;
use super::error::ModuleTopologyErrorV1;

pub(super) fn sealed_cfg_environment_v1(
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

pub(super) fn decide_module_cfg_v1(
    rows: &[String],
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<CfgDecisionV1, ModuleTopologyErrorV1> {
    decide_cfg_rows_v1(rows, environment).map_err(Into::into)
}

pub(super) fn select_active_path_v1(
    module: &str,
    attributes: &[Attribute],
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<Option<String>, ModuleTopologyErrorV1> {
    let mut active = Vec::new();
    for attribute in attributes {
        if let Some(path) = direct_path_literal(module, attribute)? {
            active.push(path);
            continue;
        }
        if attribute.path().is_ident("cfg_attr") {
            collect_cfg_attr_paths(module, &attribute.meta, environment, &mut active)?;
        }
    }
    match active.len() {
        0 => Ok(None),
        1 => Ok(active.pop()),
        _ => Err(ModuleTopologyErrorV1::MultipleActivePaths {
            module: module.to_string(),
        }),
    }
}

pub(super) fn validate_active_cfg_attributes_v1(
    module: &str,
    attributes: &[Attribute],
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<(), ModuleTopologyErrorV1> {
    for attribute in attributes {
        if attribute.path().is_ident("cfg_attr") {
            validate_cfg_attr_contents(module, &attribute.meta, environment)?;
        }
    }
    Ok(())
}

fn collect_cfg_attr_paths(
    module: &str,
    meta: &Meta,
    environment: &CfgEvaluationEnvironmentV1,
    active: &mut Vec<String>,
) -> Result<(), ModuleTopologyErrorV1> {
    let Meta::List(list) = meta else {
        return Ok(());
    };
    let nested = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(list.tokens.clone())
        .map_err(|_| ModuleTopologyErrorV1::NonLiteralPath {
            module: module.to_string(),
        })?;
    let mut rows = nested.iter();
    let Some(condition) = rows.next() else {
        return Ok(());
    };
    let condition_syntax = format!("cfg({})", condition.to_token_stream());
    let decision = decide_cfg_rows_v1(&[condition_syntax], environment)?;
    match decision.state {
        CfgDecisionStateV1::Excluded => return Ok(()),
        CfgDecisionStateV1::Unknown if rows.clone().any(meta_contains_path) => {
            return Err(ModuleTopologyErrorV1::UnknownCfg {
                module: module.to_string(),
            })
        }
        CfgDecisionStateV1::Unknown | CfgDecisionStateV1::Included => {}
    }
    if decision.state != CfgDecisionStateV1::Included {
        return Ok(());
    }
    for nested_meta in rows {
        if nested_meta.path().is_ident("path") {
            let attribute: Attribute = syn::parse_quote!(#[#nested_meta]);
            if let Some(path) = direct_path_literal(module, &attribute)? {
                active.push(path);
            }
        } else if nested_meta.path().is_ident("cfg_attr") {
            collect_cfg_attr_paths(module, nested_meta, environment, active)?;
        }
    }
    Ok(())
}

fn validate_cfg_attr_contents(
    module: &str,
    meta: &Meta,
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<(), ModuleTopologyErrorV1> {
    let Meta::List(list) = meta else {
        return Ok(());
    };
    let nested = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(list.tokens.clone())
        .map_err(|_| ModuleTopologyErrorV1::UnsupportedModuleAttribute {
            module: module.to_string(),
            attribute: list.to_token_stream().to_string(),
        })?;
    let mut rows = nested.iter();
    let Some(condition) = rows.next() else {
        return Ok(());
    };
    let decision = decide_cfg_rows_v1(
        &[format!("cfg({})", condition.to_token_stream())],
        environment,
    )?;
    for nested_meta in rows {
        let recognized = [
            "cfg",
            "cfg_attr",
            "path",
            "doc",
            "deprecated",
            "allow",
            "warn",
            "deny",
            "forbid",
            "expect",
            "no_implicit_prelude",
            "macro_use",
        ]
        .iter()
        .any(|name| nested_meta.path().is_ident(name));
        if !recognized {
            return Err(match decision.state {
                CfgDecisionStateV1::Unknown => ModuleTopologyErrorV1::UnknownCfg {
                    module: module.to_string(),
                },
                CfgDecisionStateV1::Included => ModuleTopologyErrorV1::UnsupportedModuleAttribute {
                    module: module.to_string(),
                    attribute: nested_meta.path().to_token_stream().to_string(),
                },
                CfgDecisionStateV1::Excluded => continue,
            });
        }
        if decision.state == CfgDecisionStateV1::Included && nested_meta.path().is_ident("cfg_attr")
        {
            validate_cfg_attr_contents(module, nested_meta, environment)?;
        }
    }
    Ok(())
}

fn meta_contains_path(meta: &Meta) -> bool {
    if meta.path().is_ident("path") {
        return true;
    }
    let Meta::List(list) = meta else {
        return false;
    };
    Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(list.tokens.clone())
        .map(|rows| rows.iter().any(meta_contains_path))
        .unwrap_or(true)
}
