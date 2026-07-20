//! Source-order CFG / cfg_attr stream decisions.
//!
//! CFGSTREAM0-I0 makes this the sole ordered decision owner for module and
//! include declarations. Consumers receive completed stream evidence; they do
//! not re-evaluate predicates or reconstruct cfg_attr path effects.

use std::fmt;

use proc_macro2::{TokenStream, TokenTree};
use quote::ToTokens;
use syn::parse::Parser;
use syn::punctuated::Punctuated;
use syn::{Meta, Token};

use super::cfg_eval::{decide_cfg_predicate_syntax_v1, validate_cfg_environment_v1};
use super::error::CfgDecisionErrorV1;
use super::model::{
    CfgAttributeActivePathEffectV1, CfgAttributeConditionDecisionV1, CfgAttributeNestedDecisionV1,
    CfgAttributeNestedDispositionV1, CfgAttributeStreamDecisionV1, CfgAttributeStreamInputRowV1,
    CfgAttributeStreamRowDecisionV1, CfgAttributeStreamRowDispositionV1, CfgDecisionStateV1,
    CfgEvaluationEnvironmentV1,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CfgAttributeStreamErrorV1 {
    Environment {
        source: CfgDecisionErrorV1,
    },
    InvalidSourceRange {
        source_ordinal: u32,
        byte_start: usize,
        byte_end: usize,
    },
    DuplicateSourceOrdinal {
        source_ordinal: u32,
    },
    NonMonotonicSourceOrdinal {
        previous_ordinal: u32,
        source_ordinal: u32,
    },
    Row {
        source_ordinal: u32,
        byte_start: usize,
        byte_end: usize,
        syntax: String,
        source: CfgDecisionErrorV1,
    },
}

impl fmt::Display for CfgAttributeStreamErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Environment { source } => {
                write!(formatter, "[rust-source-topology/cfg-stream/environment] source={source}")
            }
            Self::InvalidSourceRange {
                source_ordinal,
                byte_start,
                byte_end,
            } => write!(
                formatter,
                "[rust-source-topology/cfg-stream/invalid-source-range] ordinal={source_ordinal} byte_start={byte_start} byte_end={byte_end}"
            ),
            Self::DuplicateSourceOrdinal { source_ordinal } => write!(
                formatter,
                "[rust-source-topology/cfg-stream/duplicate-source-ordinal] ordinal={source_ordinal}"
            ),
            Self::NonMonotonicSourceOrdinal {
                previous_ordinal,
                source_ordinal,
            } => write!(
                formatter,
                "[rust-source-topology/cfg-stream/non-monotonic-source-ordinal] previous={previous_ordinal} actual={source_ordinal}"
            ),
            Self::Row {
                source_ordinal,
                byte_start,
                byte_end,
                syntax,
                source,
            } => write!(
                formatter,
                "[rust-source-topology/cfg-stream/row-error] ordinal={source_ordinal} byte_start={byte_start} byte_end={byte_end} syntax={syntax:?} source={source}"
            ),
        }
    }
}

impl std::error::Error for CfgAttributeStreamErrorV1 {}

/// Decides one ordered attribute stream without evaluating past a terminal row.
///
/// An `Excluded` row preserves all later source rows as explicit non-reached
/// evidence. An `Unknown` row is terminal but has no invented disposition for
/// later rows: the caller must reject the incomplete topology rather than let a
/// later exclusion erase the unknown fact.
pub fn decide_cfg_attribute_stream_v1(
    rows: &[CfgAttributeStreamInputRowV1],
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<CfgAttributeStreamDecisionV1, CfgAttributeStreamErrorV1> {
    let target = validate_cfg_environment_v1(environment)
        .map_err(|source| CfgAttributeStreamErrorV1::Environment { source })?;
    validate_stream_rows(rows)?;

    let mut decisions = Vec::with_capacity(rows.len());
    let mut active_path_effects = Vec::new();
    for (index, input) in rows.iter().enumerate() {
        let evaluated = evaluate_outer_row(input, environment, &target)?;
        let state = evaluated
            .decision
            .state
            .expect("evaluated row must carry a state");
        active_path_effects.extend(evaluated.active_path_syntaxes.into_vec().into_iter().map(
            |effect| CfgAttributeActivePathEffectV1 {
                outer_source_ordinal: input.source_ordinal,
                outer_source_range: input.source_range,
                nested_index_path: effect.nested_index_path,
                syntax: effect.syntax,
            },
        ));
        decisions.push(evaluated.decision);

        match state {
            CfgDecisionStateV1::Included => {}
            CfgDecisionStateV1::Unknown => {
                return Ok(finish_decision(
                    environment,
                    CfgDecisionStateV1::Unknown,
                    Some(input.source_ordinal),
                    decisions,
                    active_path_effects,
                ));
            }
            CfgDecisionStateV1::Excluded => {
                for later in &rows[index + 1..] {
                    decisions.push(CfgAttributeStreamRowDecisionV1 {
                        input: later.clone(),
                        disposition: CfgAttributeStreamRowDispositionV1::NotReachedAfterExclusion,
                        state: None,
                        unknown_predicates: Box::new([]),
                        cfg_attr_condition: None,
                        nested: Box::new([]),
                    });
                }
                return Ok(finish_decision(
                    environment,
                    CfgDecisionStateV1::Excluded,
                    Some(input.source_ordinal),
                    decisions,
                    active_path_effects,
                ));
            }
        }
    }

    Ok(finish_decision(
        environment,
        CfgDecisionStateV1::Included,
        None,
        decisions,
        active_path_effects,
    ))
}

fn finish_decision(
    environment: &CfgEvaluationEnvironmentV1,
    final_state: CfgDecisionStateV1,
    decisive_row_ordinal: Option<u32>,
    rows: Vec<CfgAttributeStreamRowDecisionV1>,
    active_path_effects: Vec<CfgAttributeActivePathEffectV1>,
) -> CfgAttributeStreamDecisionV1 {
    CfgAttributeStreamDecisionV1 {
        profile_id: environment.profile_id.clone(),
        final_state,
        decisive_row_ordinal,
        rows: rows.into_boxed_slice(),
        active_path_effects: if final_state == CfgDecisionStateV1::Included {
            active_path_effects.into_boxed_slice()
        } else {
            Box::new([])
        },
    }
}

fn validate_stream_rows(
    rows: &[CfgAttributeStreamInputRowV1],
) -> Result<(), CfgAttributeStreamErrorV1> {
    let mut previous_ordinal = None;
    for input in rows {
        validate_range(input)?;
        if let Some(previous_ordinal) = previous_ordinal {
            if input.source_ordinal == previous_ordinal {
                return Err(CfgAttributeStreamErrorV1::DuplicateSourceOrdinal {
                    source_ordinal: input.source_ordinal,
                });
            }
            if input.source_ordinal < previous_ordinal {
                return Err(CfgAttributeStreamErrorV1::NonMonotonicSourceOrdinal {
                    previous_ordinal,
                    source_ordinal: input.source_ordinal,
                });
            }
        }
        previous_ordinal = Some(input.source_ordinal);
    }
    Ok(())
}

fn validate_range(input: &CfgAttributeStreamInputRowV1) -> Result<(), CfgAttributeStreamErrorV1> {
    if input.source_range.byte_start > input.source_range.byte_end {
        return Err(CfgAttributeStreamErrorV1::InvalidSourceRange {
            source_ordinal: input.source_ordinal,
            byte_start: input.source_range.byte_start,
            byte_end: input.source_range.byte_end,
        });
    }
    Ok(())
}

fn evaluate_outer_row(
    input: &CfgAttributeStreamInputRowV1,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
) -> Result<EvaluatedOuterRowV1, CfgAttributeStreamErrorV1> {
    let meta = parse_meta(input, &input.syntax)?;
    let evaluated = evaluate_meta(&meta, &input.syntax, environment, target)
        .map_err(|source| row_error(input, source))?;
    Ok(EvaluatedOuterRowV1 {
        decision: CfgAttributeStreamRowDecisionV1 {
            input: input.clone(),
            disposition: evaluated.disposition,
            state: Some(evaluated.state),
            unknown_predicates: evaluated.unknown_predicates,
            cfg_attr_condition: evaluated.cfg_attr_condition,
            nested: evaluated.nested,
        },
        active_path_syntaxes: evaluated.active_path_syntaxes,
    })
}

struct EvaluatedOuterRowV1 {
    decision: CfgAttributeStreamRowDecisionV1,
    active_path_syntaxes: Box<[ActivePathSyntaxV1]>,
}

struct ActivePathSyntaxV1 {
    syntax: String,
    nested_index_path: Box<[u32]>,
}

impl ActivePathSyntaxV1 {
    fn direct(syntax: &str) -> Self {
        Self {
            syntax: syntax.to_string(),
            nested_index_path: Box::new([]),
        }
    }

    fn nested_below(mut self, index: usize) -> Self {
        let index = u32::try_from(index).expect("nested cfg_attr index exceeds u32");
        let mut nested_index_path = Vec::with_capacity(self.nested_index_path.len() + 1);
        nested_index_path.push(index);
        nested_index_path.extend(self.nested_index_path);
        self.nested_index_path = nested_index_path.into_boxed_slice();
        self
    }
}

struct EvaluatedAttributeV1 {
    disposition: CfgAttributeStreamRowDispositionV1,
    state: CfgDecisionStateV1,
    unknown_predicates: Box<[String]>,
    cfg_attr_condition: Option<CfgAttributeConditionDecisionV1>,
    nested: Box<[CfgAttributeNestedDecisionV1]>,
    active_path_syntaxes: Box<[ActivePathSyntaxV1]>,
}

struct EvaluatedNestedStreamV1 {
    state: CfgDecisionStateV1,
    unknown_predicates: Box<[String]>,
    nested: Box<[CfgAttributeNestedDecisionV1]>,
    active_path_syntaxes: Box<[ActivePathSyntaxV1]>,
}

fn evaluate_meta(
    meta: &Meta,
    syntax: &str,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
) -> Result<EvaluatedAttributeV1, CfgDecisionErrorV1> {
    if meta.path().is_ident("cfg") {
        let Meta::List(list) = meta else {
            return Err(CfgDecisionErrorV1::MalformedCfgExpression {
                syntax: syntax.to_string(),
                detail: "cfg must be a list".to_string(),
            });
        };
        let normalized = format!("cfg({})", list.tokens);
        let (state, unknown_predicates) =
            decide_cfg_predicate_syntax_v1(&normalized, environment, target)?;
        return Ok(EvaluatedAttributeV1 {
            disposition: CfgAttributeStreamRowDispositionV1::Evaluated,
            state,
            unknown_predicates,
            cfg_attr_condition: None,
            nested: Box::new([]),
            active_path_syntaxes: Box::new([]),
        });
    }
    if meta.path().is_ident("cfg_attr") {
        return evaluate_cfg_attr(meta, syntax, environment, target);
    }
    Ok(EvaluatedAttributeV1 {
        disposition: CfgAttributeStreamRowDispositionV1::TopologyNeutral,
        state: CfgDecisionStateV1::Included,
        unknown_predicates: Box::new([]),
        cfg_attr_condition: None,
        nested: Box::new([]),
        active_path_syntaxes: if meta.path().is_ident("path") {
            vec![ActivePathSyntaxV1::direct(syntax)].into_boxed_slice()
        } else {
            Box::new([])
        },
    })
}

fn evaluate_cfg_attr(
    meta: &Meta,
    syntax: &str,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
) -> Result<EvaluatedAttributeV1, CfgDecisionErrorV1> {
    let Meta::List(list) = meta else {
        return Err(CfgDecisionErrorV1::MalformedCfgAttr {
            syntax: syntax.to_string(),
            detail: "cfg_attr must be a list".to_string(),
        });
    };
    let (condition_tokens, nested_tokens) = split_cfg_attr_tokens(list, syntax)?;
    let condition_syntax = format!("cfg({condition_tokens})");
    let (condition_state, condition_unknown) =
        decide_cfg_predicate_syntax_v1(&condition_syntax, environment, target)?;
    let condition = CfgAttributeConditionDecisionV1 {
        syntax: condition_syntax,
        state: condition_state,
        unknown_predicates: condition_unknown.clone(),
    };

    if condition_state == CfgDecisionStateV1::Excluded {
        let nested = if nested_tokens.is_empty() {
            Box::new([])
        } else {
            vec![CfgAttributeNestedDecisionV1 {
                syntax: nested_tokens.to_string(),
                disposition: CfgAttributeNestedDispositionV1::NotEvaluatedInactiveCfgAttr,
                state: None,
                unknown_predicates: Box::new([]),
                cfg_attr_condition: None,
                nested: Box::new([]),
            }]
            .into_boxed_slice()
        };
        return Ok(EvaluatedAttributeV1 {
            disposition: CfgAttributeStreamRowDispositionV1::Evaluated,
            state: CfgDecisionStateV1::Included,
            unknown_predicates: Box::new([]),
            cfg_attr_condition: Some(condition),
            nested,
            active_path_syntaxes: Box::new([]),
        });
    }

    let nested_metas = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse2(nested_tokens)
        .map_err(|error| CfgDecisionErrorV1::MalformedCfgAttr {
            syntax: syntax.to_string(),
            detail: error.to_string(),
        })?;
    let nested = evaluate_active_nested_stream(nested_metas, environment, target)?;
    let mut unknown = condition_unknown.into_vec();
    unknown.extend(nested.unknown_predicates.iter().cloned());
    if condition_state == CfgDecisionStateV1::Unknown {
        unknown.extend(
            nested
                .active_path_syntaxes
                .iter()
                .map(|effect| format!("cfg_attr:path:{}", effect.syntax)),
        );
    }
    unknown.sort();
    unknown.dedup();
    let state = match condition_state {
        CfgDecisionStateV1::Included => nested.state,
        CfgDecisionStateV1::Unknown if !nested.active_path_syntaxes.is_empty() => {
            CfgDecisionStateV1::Unknown
        }
        CfgDecisionStateV1::Unknown => implication_state(condition_state, nested.state),
        CfgDecisionStateV1::Excluded => CfgDecisionStateV1::Included,
    };
    Ok(EvaluatedAttributeV1 {
        disposition: CfgAttributeStreamRowDispositionV1::Evaluated,
        state,
        unknown_predicates: unknown.into_boxed_slice(),
        cfg_attr_condition: Some(condition),
        nested: nested.nested,
        active_path_syntaxes: nested.active_path_syntaxes,
    })
}

fn split_cfg_attr_tokens(
    list: &syn::MetaList,
    syntax: &str,
) -> Result<(TokenStream, TokenStream), CfgDecisionErrorV1> {
    let mut condition = TokenStream::new();
    let mut nested = TokenStream::new();
    let mut found_separator = false;
    for token in list.tokens.clone() {
        if !found_separator
            && matches!(token, TokenTree::Punct(ref punct) if punct.as_char() == ',')
        {
            found_separator = true;
            continue;
        }
        if found_separator {
            nested.extend([token]);
        } else {
            condition.extend([token]);
        }
    }
    if condition.is_empty() {
        return Err(CfgDecisionErrorV1::MalformedCfgAttr {
            syntax: syntax.to_string(),
            detail: "cfg_attr requires a condition".to_string(),
        });
    }
    Ok((condition, nested))
}

fn evaluate_active_nested_stream(
    nested_metas: Punctuated<Meta, Token![,]>,
    environment: &CfgEvaluationEnvironmentV1,
    target: &cfg_expr::targets::TargetInfo,
) -> Result<EvaluatedNestedStreamV1, CfgDecisionErrorV1> {
    let mut unknown = Vec::new();
    let mut nested = Vec::new();
    let mut active_path_syntaxes = Vec::new();
    let mut metas = nested_metas.into_iter().enumerate();
    while let Some((index, meta)) = metas.next() {
        let syntax = meta.to_token_stream().to_string();
        let child = evaluate_meta(&meta, &syntax, environment, target)?;
        let child_state = child.state;
        unknown.extend(child.unknown_predicates.iter().cloned());
        active_path_syntaxes.extend(
            child
                .active_path_syntaxes
                .into_vec()
                .into_iter()
                .map(|effect| effect.nested_below(index)),
        );
        nested.push(CfgAttributeNestedDecisionV1 {
            syntax,
            disposition: nested_disposition(child.disposition),
            state: Some(child_state),
            unknown_predicates: child.unknown_predicates,
            cfg_attr_condition: child.cfg_attr_condition,
            nested: child.nested,
        });
        match child_state {
            CfgDecisionStateV1::Included => {}
            CfgDecisionStateV1::Unknown => {
                return Ok(EvaluatedNestedStreamV1 {
                    state: CfgDecisionStateV1::Unknown,
                    unknown_predicates: normalize_unknown(unknown),
                    nested: nested.into_boxed_slice(),
                    active_path_syntaxes: active_path_syntaxes.into_boxed_slice(),
                });
            }
            CfgDecisionStateV1::Excluded => {
                for (_, later) in metas {
                    nested.push(CfgAttributeNestedDecisionV1 {
                        syntax: later.to_token_stream().to_string(),
                        disposition: CfgAttributeNestedDispositionV1::NotReachedAfterExclusion,
                        state: None,
                        unknown_predicates: Box::new([]),
                        cfg_attr_condition: None,
                        nested: Box::new([]),
                    });
                }
                return Ok(EvaluatedNestedStreamV1 {
                    state: CfgDecisionStateV1::Excluded,
                    unknown_predicates: normalize_unknown(unknown),
                    nested: nested.into_boxed_slice(),
                    active_path_syntaxes: active_path_syntaxes.into_boxed_slice(),
                });
            }
        }
    }
    Ok(EvaluatedNestedStreamV1 {
        state: CfgDecisionStateV1::Included,
        unknown_predicates: normalize_unknown(unknown),
        nested: nested.into_boxed_slice(),
        active_path_syntaxes: active_path_syntaxes.into_boxed_slice(),
    })
}

fn normalize_unknown(mut unknown: Vec<String>) -> Box<[String]> {
    unknown.sort();
    unknown.dedup();
    unknown.into_boxed_slice()
}

fn parse_meta(
    input: &CfgAttributeStreamInputRowV1,
    syntax: &str,
) -> Result<Meta, CfgAttributeStreamErrorV1> {
    syn::parse_str::<Meta>(syntax).map_err(|error| {
        row_error(
            input,
            CfgDecisionErrorV1::MalformedAttribute {
                syntax: syntax.to_string(),
                detail: error.to_string(),
            },
        )
    })
}

fn row_error(
    input: &CfgAttributeStreamInputRowV1,
    source: CfgDecisionErrorV1,
) -> CfgAttributeStreamErrorV1 {
    CfgAttributeStreamErrorV1::Row {
        source_ordinal: input.source_ordinal,
        byte_start: input.source_range.byte_start,
        byte_end: input.source_range.byte_end,
        syntax: input.syntax.clone(),
        source,
    }
}

fn nested_disposition(
    disposition: CfgAttributeStreamRowDispositionV1,
) -> CfgAttributeNestedDispositionV1 {
    match disposition {
        CfgAttributeStreamRowDispositionV1::Evaluated => CfgAttributeNestedDispositionV1::Evaluated,
        CfgAttributeStreamRowDispositionV1::TopologyNeutral => {
            CfgAttributeNestedDispositionV1::TopologyNeutral
        }
        CfgAttributeStreamRowDispositionV1::NotReachedAfterExclusion => {
            unreachable!("nested cfg_attr evaluation never emits outer non-reached rows")
        }
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
