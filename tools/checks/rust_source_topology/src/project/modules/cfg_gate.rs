//! Module-layer consumers of the ordered CFG stream.
//!
//! Predicate evaluation belongs exclusively to CFGSTREAM0. This adapter only
//! projects its final state into topology, validates active non-predicate
//! attribute shapes, and parses already-selected literal path effects.

use quote::ToTokens;
use syn::Meta;

use crate::project::{
    decide_cfg_attribute_stream_v1, CfgAttributeNestedDecisionV1, CfgAttributeNestedDispositionV1,
    CfgAttributeStreamDecisionV1, CfgAttributeStreamInputRowV1, CfgDecisionStateV1,
    CfgEvaluationEnvironmentV1,
};

use super::error::ModuleTopologyErrorV1;

pub(super) fn decide_module_cfg_stream_v1(
    rows: &[CfgAttributeStreamInputRowV1],
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<CfgAttributeStreamDecisionV1, ModuleTopologyErrorV1> {
    decide_cfg_attribute_stream_v1(rows, environment).map_err(Into::into)
}

pub(super) fn select_active_path_v1(
    module: &str,
    decision: &CfgAttributeStreamDecisionV1,
) -> Result<Option<String>, ModuleTopologyErrorV1> {
    if decision.final_state != CfgDecisionStateV1::Included {
        return Ok(None);
    }
    let mut active = Vec::new();
    for effect in decision.active_path_effects.iter() {
        let meta = syn::parse_str::<Meta>(&effect.syntax).map_err(|_| {
            ModuleTopologyErrorV1::NonLiteralPath {
                module: module.to_string(),
            }
        })?;
        active.push(literal_path(module, &meta)?);
    }
    match active.len() {
        0 => Ok(None),
        1 => Ok(active.pop()),
        _ => Err(ModuleTopologyErrorV1::MultipleActivePaths {
            module: module.to_string(),
        }),
    }
}

pub(super) fn validate_selected_cfg_attributes_v1(
    module: &str,
    decision: &CfgAttributeStreamDecisionV1,
) -> Result<(), ModuleTopologyErrorV1> {
    if decision.final_state != CfgDecisionStateV1::Included {
        return Ok(());
    }
    for row in decision.rows.iter() {
        let Some(condition) = row.cfg_attr_condition.as_ref() else {
            continue;
        };
        validate_nested_attributes(module, &row.nested, condition.state)?;
    }
    Ok(())
}

fn validate_nested_attributes(
    module: &str,
    nested: &[CfgAttributeNestedDecisionV1],
    inherited_condition: CfgDecisionStateV1,
) -> Result<(), ModuleTopologyErrorV1> {
    for row in nested {
        match row.disposition {
            CfgAttributeNestedDispositionV1::NotEvaluatedInactiveCfgAttr
            | CfgAttributeNestedDispositionV1::NotReachedAfterExclusion => continue,
            CfgAttributeNestedDispositionV1::Evaluated
            | CfgAttributeNestedDispositionV1::TopologyNeutral => {}
        }
        let meta = syn::parse_str::<Meta>(&row.syntax).map_err(|_| {
            ModuleTopologyErrorV1::UnsupportedModuleAttribute {
                module: module.to_string(),
                attribute: row.syntax.clone(),
            }
        })?;
        if !is_supported_attribute(&meta) {
            return Err(match inherited_condition {
                CfgDecisionStateV1::Unknown => ModuleTopologyErrorV1::UnknownCfg {
                    module: module.to_string(),
                },
                CfgDecisionStateV1::Included | CfgDecisionStateV1::Excluded => {
                    ModuleTopologyErrorV1::UnsupportedModuleAttribute {
                        module: module.to_string(),
                        attribute: meta.path().to_token_stream().to_string(),
                    }
                }
            });
        }
        if let Some(condition) = row.cfg_attr_condition.as_ref() {
            validate_nested_attributes(module, &row.nested, condition.state)?;
        }
    }
    Ok(())
}

fn literal_path(module: &str, meta: &Meta) -> Result<String, ModuleTopologyErrorV1> {
    if !meta.path().is_ident("path") {
        return Err(ModuleTopologyErrorV1::NonLiteralPath {
            module: module.to_string(),
        });
    }
    let Meta::NameValue(name_value) = meta else {
        return Err(ModuleTopologyErrorV1::NonLiteralPath {
            module: module.to_string(),
        });
    };
    let syn::Expr::Lit(expression) = &name_value.value else {
        return Err(ModuleTopologyErrorV1::NonLiteralPath {
            module: module.to_string(),
        });
    };
    let syn::Lit::Str(value) = &expression.lit else {
        return Err(ModuleTopologyErrorV1::NonLiteralPath {
            module: module.to_string(),
        });
    };
    Ok(value.value())
}

fn is_supported_attribute(meta: &Meta) -> bool {
    [
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
    .any(|name| meta.path().is_ident(name))
}
