//! Private parse/classify boundary for one module-content candidate.
//!
//! Parsing always consumes the complete source before the inner CFG stream is
//! classified. Only an Included classification exposes direct items; excluded
//! content cannot accidentally trigger descendant validation in a later I0
//! consumer.

#![allow(dead_code)] // CONTENTCFG0-R0 intentionally has zero production consumers.

use std::fmt;

use crate::project::{
    decide_cfg_attribute_stream_v1, CfgAttributeStreamErrorV1, CfgEvaluationEnvironmentV1,
    InnerTopologyAttributeSurfaceErrorV1,
};
use crate::{PositionV1, SourceRangeV1};

use super::super::inner_cfg_surface::collect_inner_topology_surface_from_parsed_file_v1;
use super::{
    DeclaredModuleContentGateV1, ModuleContentCandidateIdV1, ModuleContentDefiningSurfaceV1,
};

pub(super) struct ParsedModuleContentDraftV1 {
    candidate_id: ModuleContentCandidateIdV1,
    defining_surface: ModuleContentDefiningSurfaceV1,
    inner_surface: crate::project::FileInnerTopologyAttributeSurfaceV1,
    direct_items: Box<[syn::Item]>,
}

pub(super) enum ClassifiedModuleContentDraftV1 {
    Included {
        gate: DeclaredModuleContentGateV1,
        direct_items: Box<[syn::Item]>,
    },
    Excluded {
        gate: DeclaredModuleContentGateV1,
    },
}

#[derive(Debug)]
pub(super) enum ModuleContentDraftErrorV1 {
    Surface(InnerTopologyAttributeSurfaceErrorV1),
    Stream(CfgAttributeStreamErrorV1),
    UnknownCfg {
        source_path_workspace_relative: String,
    },
    ActiveInnerPath {
        source_path_workspace_relative: String,
    },
    InlineBodyRangeInvalid {
        source_path_workspace_relative: String,
        byte_start: usize,
        byte_end: usize,
    },
}

impl fmt::Display for ModuleContentDraftErrorV1 {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Surface(error) => error.fmt(formatter),
            Self::Stream(error) => error.fmt(formatter),
            Self::UnknownCfg {
                source_path_workspace_relative,
            } => write!(
                formatter,
                "[rust-source-topology/content-draft/cfg-unknown] path={source_path_workspace_relative}"
            ),
            Self::ActiveInnerPath {
                source_path_workspace_relative,
            } => write!(
                formatter,
                "[rust-source-topology/content-draft/inner-path-unsupported] path={source_path_workspace_relative}"
            ),
            Self::InlineBodyRangeInvalid {
                source_path_workspace_relative,
                byte_start,
                byte_end,
            } => write!(
                formatter,
                "[rust-source-topology/content-draft/inline-body-range-invalid] path={source_path_workspace_relative} start={byte_start} end={byte_end}"
            ),
        }
    }
}

impl std::error::Error for ModuleContentDraftErrorV1 {}

impl ModuleContentDraftErrorV1 {
    pub(super) fn into_topology_error(self) -> super::ModuleTopologyErrorV1 {
        match self {
            Self::Stream(error) => super::ModuleTopologyErrorV1::CfgStream(error),
            Self::UnknownCfg {
                source_path_workspace_relative,
            } => super::ModuleTopologyErrorV1::ContentCfgUnknown {
                path: source_path_workspace_relative,
            },
            Self::ActiveInnerPath {
                source_path_workspace_relative,
            } => super::ModuleTopologyErrorV1::ActiveContentPath {
                path: source_path_workspace_relative,
            },
            Self::Surface(error) => super::ModuleTopologyErrorV1::ContentDraft {
                detail: error.to_string(),
            },
            error @ Self::InlineBodyRangeInvalid { .. } => {
                super::ModuleTopologyErrorV1::ContentDraft {
                    detail: error.to_string(),
                }
            }
        }
    }
}

pub(super) fn parse_module_content_draft_v1(
    candidate_id: ModuleContentCandidateIdV1,
    defining_surface: ModuleContentDefiningSurfaceV1,
    source_path_workspace_relative: &str,
    source: &str,
) -> Result<ParsedModuleContentDraftV1, ModuleContentDraftErrorV1> {
    let file = syn::parse_file(source).map_err(|error| {
        ModuleContentDraftErrorV1::Surface(InnerTopologyAttributeSurfaceErrorV1::Parse {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            detail: error.to_string(),
        })
    })?;
    let inner_surface = collect_inner_topology_surface_from_parsed_file_v1(
        source_path_workspace_relative,
        source,
        &file,
    )
    .map_err(ModuleContentDraftErrorV1::Surface)?;
    Ok(ParsedModuleContentDraftV1 {
        candidate_id,
        defining_surface,
        inner_surface,
        direct_items: file.items.into_boxed_slice(),
    })
}

/// Parses one inline body and projects its inner-CFG rows back to the parent
/// source coordinate system.  The body-local parser is an implementation
/// detail; `InlineBody` gates never publish body-relative source identity.
pub(super) fn parse_inline_module_content_draft_v1(
    candidate_id: ModuleContentCandidateIdV1,
    parent_source_observation_id: String,
    source_path_workspace_relative: &str,
    parent_source: &str,
    body_range: SourceRangeV1,
) -> Result<ParsedModuleContentDraftV1, ModuleContentDraftErrorV1> {
    let braced_body = parent_source
        .get(body_range.byte_start..body_range.byte_end)
        .ok_or_else(|| ModuleContentDraftErrorV1::InlineBodyRangeInvalid {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            byte_start: body_range.byte_start,
            byte_end: body_range.byte_end,
        })?;
    let body = braced_body
        .strip_prefix('{')
        .and_then(|source| source.strip_suffix('}'))
        .ok_or_else(|| ModuleContentDraftErrorV1::InlineBodyRangeInvalid {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            byte_start: body_range.byte_start,
            byte_end: body_range.byte_end,
        })?;
    let mut draft = parse_module_content_draft_v1(
        candidate_id,
        ModuleContentDefiningSurfaceV1::InlineBody {
            parent_source_observation_id,
            body_range,
        },
        source_path_workspace_relative,
        body,
    )?;
    let content_start = body_range.byte_start.checked_add(1).ok_or_else(|| {
        ModuleContentDraftErrorV1::InlineBodyRangeInvalid {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            byte_start: body_range.byte_start,
            byte_end: body_range.byte_end,
        }
    })?;
    for row in &mut draft.inner_surface.rows {
        row.source_range = translate_body_range_v1(
            parent_source,
            content_start,
            row.source_range,
            source_path_workspace_relative,
            body_range,
        )?;
    }
    Ok(draft)
}

fn translate_body_range_v1(
    parent_source: &str,
    content_start: usize,
    local: SourceRangeV1,
    source_path_workspace_relative: &str,
    body_range: SourceRangeV1,
) -> Result<SourceRangeV1, ModuleContentDraftErrorV1> {
    let byte_start = content_start.checked_add(local.byte_start).ok_or_else(|| {
        ModuleContentDraftErrorV1::InlineBodyRangeInvalid {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            byte_start: body_range.byte_start,
            byte_end: body_range.byte_end,
        }
    })?;
    let byte_end = content_start.checked_add(local.byte_end).ok_or_else(|| {
        ModuleContentDraftErrorV1::InlineBodyRangeInvalid {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            byte_start: body_range.byte_start,
            byte_end: body_range.byte_end,
        }
    })?;
    if byte_start > byte_end || byte_end > parent_source.len() {
        return Err(ModuleContentDraftErrorV1::InlineBodyRangeInvalid {
            source_path_workspace_relative: source_path_workspace_relative.to_string(),
            byte_start: body_range.byte_start,
            byte_end: body_range.byte_end,
        });
    }
    Ok(SourceRangeV1 {
        start: position_at_byte(parent_source, byte_start),
        end: position_at_byte(parent_source, byte_end),
        byte_start,
        byte_end,
    })
}

fn position_at_byte(source: &str, byte_offset: usize) -> PositionV1 {
    let prefix = &source[..byte_offset];
    let line = prefix.bytes().filter(|byte| *byte == b'\n').count() + 1;
    let column = prefix.rfind('\n').map_or(prefix.len(), |newline| {
        prefix.len().saturating_sub(newline + 1)
    });
    PositionV1 { line, column }
}

pub(super) fn classify_module_content_draft_v1(
    draft: ParsedModuleContentDraftV1,
    environment: &CfgEvaluationEnvironmentV1,
) -> Result<ClassifiedModuleContentDraftV1, ModuleContentDraftErrorV1> {
    let decision = decide_cfg_attribute_stream_v1(&draft.inner_surface.rows, environment)
        .map_err(ModuleContentDraftErrorV1::Stream)?;
    let gate = DeclaredModuleContentGateV1 {
        candidate_id: draft.candidate_id,
        defining_surface: draft.defining_surface,
        inner_cfg_sites: draft.inner_surface.rows,
        cfg_decision: decision,
    };
    let path = gate_path(&gate);
    match gate.cfg_decision.final_state {
        crate::project::CfgDecisionStateV1::Included => {
            if !gate.cfg_decision.active_path_effects.is_empty() {
                return Err(ModuleContentDraftErrorV1::ActiveInnerPath {
                    source_path_workspace_relative: path,
                });
            }
            Ok(ClassifiedModuleContentDraftV1::Included {
                gate,
                direct_items: draft.direct_items,
            })
        }
        crate::project::CfgDecisionStateV1::Excluded => {
            Ok(ClassifiedModuleContentDraftV1::Excluded { gate })
        }
        crate::project::CfgDecisionStateV1::Unknown => Err(ModuleContentDraftErrorV1::UnknownCfg {
            source_path_workspace_relative: path,
        }),
    }
}

fn gate_path(gate: &DeclaredModuleContentGateV1) -> String {
    match &gate.defining_surface {
        ModuleContentDefiningSurfaceV1::SourceFile {
            source_path_workspace_relative,
            ..
        } => source_path_workspace_relative.clone(),
        ModuleContentDefiningSurfaceV1::InlineBody {
            parent_source_observation_id,
            ..
        } => parent_source_observation_id.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project::{
        parse_and_verify_profile_schema_v1, CfgDecisionStateV1, CfgEvaluationEnvironmentV1,
    };

    const PROFILES: &str = include_str!("../../../tests/fixtures/profiles_v1.json");

    #[test]
    fn excluded_content_is_parsed_but_never_exposes_direct_items() {
        let classified = classify("#![cfg(any())]\nmod missing;\n").unwrap();
        let ClassifiedModuleContentDraftV1::Excluded { gate } = classified else {
            panic!("excluded content must not expose items");
        };
        assert_eq!(gate.cfg_decision.final_state, CfgDecisionStateV1::Excluded);
    }

    #[test]
    fn included_content_exposes_only_its_direct_items() {
        let classified = classify("#![cfg(all())]\nmod direct { mod descendant; }\n").unwrap();
        let ClassifiedModuleContentDraftV1::Included { gate, direct_items } = classified else {
            panic!("included content must expose direct items");
        };
        assert_eq!(gate.cfg_decision.final_state, CfgDecisionStateV1::Included);
        assert_eq!(direct_items.len(), 1);
    }

    #[test]
    fn parse_failure_precedes_an_inner_false_cfg_decision() {
        let result = parse_module_content_draft_v1(
            ModuleContentCandidateIdV1::Root,
            source_surface(),
            "src/lib.rs",
            "#![cfg(any())]\nthis is not rust {\n",
        );
        assert!(matches!(result, Err(ModuleContentDraftErrorV1::Surface(_))));
    }

    #[test]
    fn active_inner_path_is_a_typed_parked_stop() {
        let result = classify("#![path = \"other.rs\"]\npub fn item() {}\n");
        assert!(matches!(
            result,
            Err(ModuleContentDraftErrorV1::ActiveInnerPath { .. })
        ));
    }

    #[test]
    fn root_external_and_inline_candidates_share_the_same_three_way_gate() {
        let cases = [
            (ModuleContentCandidateIdV1::Root, source_surface(), "root"),
            (
                ModuleContentCandidateIdV1::ModuleEdge {
                    edge_id: "edge:external".to_string(),
                },
                ModuleContentDefiningSurfaceV1::SourceFile {
                    source_path_workspace_relative: "src/external.rs".to_string(),
                    content_digest: "sha256:external".to_string(),
                },
                "external",
            ),
            (
                ModuleContentCandidateIdV1::ModuleEdge {
                    edge_id: "edge:inline".to_string(),
                },
                ModuleContentDefiningSurfaceV1::InlineBody {
                    parent_source_observation_id: "source:parent".to_string(),
                    body_range: crate::SourceRangeV1 {
                        start: crate::PositionV1 { line: 4, column: 0 },
                        end: crate::PositionV1 { line: 6, column: 1 },
                        byte_start: 40,
                        byte_end: 58,
                    },
                },
                "inline",
            ),
        ];
        for (candidate_id, surface, label) in cases {
            let included = classify_with(
                candidate_id.clone(),
                surface.clone(),
                "#![cfg(all())]\npub fn item() {}\n",
            );
            let ClassifiedModuleContentDraftV1::Included { gate, .. } = included.unwrap() else {
                panic!("{label} true must include");
            };
            assert_eq!(gate.candidate_id, candidate_id);
            assert_eq!(gate.defining_surface, surface);

            let excluded = classify_with(
                candidate_id.clone(),
                surface.clone(),
                "#![cfg(any())]\nmod missing;\n",
            );
            let ClassifiedModuleContentDraftV1::Excluded { gate } = excluded.unwrap() else {
                panic!("{label} false must exclude");
            };
            assert_eq!(gate.candidate_id, candidate_id);
            assert_eq!(gate.defining_surface, surface);

            let unknown = classify_with(
                candidate_id,
                surface,
                "#![cfg(content_cfg_unknown)]\npub fn item() {}\n",
            );
            assert!(matches!(
                unknown,
                Err(ModuleContentDraftErrorV1::UnknownCfg { .. })
            ));
        }
    }

    #[test]
    fn excluded_inner_content_short_circuits_later_path_and_malformed_cfg() {
        let classified = classify(
            "#![cfg(any())]\n#![path = concat!(\"not\", \"a-path\")]\n#![cfg(not())]\nmod missing;\n",
        )
        .unwrap();
        let ClassifiedModuleContentDraftV1::Excluded { gate } = classified else {
            panic!("first false cfg must exclude the candidate");
        };
        assert_eq!(gate.cfg_decision.final_state, CfgDecisionStateV1::Excluded);
        assert_eq!(gate.cfg_decision.active_path_effects.len(), 0);
        assert!(gate.cfg_decision.rows.iter().skip(1).all(|row| matches!(
            row.disposition,
            crate::project::CfgAttributeStreamRowDispositionV1::NotReachedAfterExclusion
        )));
    }

    #[test]
    fn active_or_inactive_nested_cfg_attr_keeps_the_shared_stream_law() {
        let inactive = classify("#![cfg_attr(any(), cfg(not()))]\npub fn item() {}\n").unwrap();
        let ClassifiedModuleContentDraftV1::Included { gate, .. } = inactive else {
            panic!("inactive cfg_attr must not parse its malformed nested cfg");
        };
        assert_eq!(gate.cfg_decision.final_state, CfgDecisionStateV1::Included);

        let active = classify("#![cfg_attr(all(), cfg(any()))]\npub fn item() {}\n").unwrap();
        let ClassifiedModuleContentDraftV1::Excluded { gate } = active else {
            panic!("active cfg_attr must apply its nested cfg");
        };
        assert_eq!(gate.cfg_decision.final_state, CfgDecisionStateV1::Excluded);

        let malformed_first = classify("#![cfg(not())]\n#![cfg(any())]\npub fn item() {}\n");
        assert!(matches!(
            malformed_first,
            Err(ModuleContentDraftErrorV1::Stream(_))
        ));
    }

    #[test]
    fn inline_body_rows_keep_parent_source_coordinates() {
        let parent = "mod child { #![cfg(any())]\nmod missing; }\n";
        let open = parent.find('{').unwrap();
        let close = parent.rfind('}').unwrap();
        let draft = parse_inline_module_content_draft_v1(
            ModuleContentCandidateIdV1::ModuleEdge {
                edge_id: "edge:inline".to_string(),
            },
            "source:parent".to_string(),
            "src/lib.rs",
            parent,
            crate::SourceRangeV1 {
                start: crate::PositionV1 {
                    line: 1,
                    column: open,
                },
                end: crate::PositionV1 {
                    line: 1,
                    column: close + 1,
                },
                byte_start: open,
                byte_end: close + 1,
            },
        )
        .unwrap();
        let ClassifiedModuleContentDraftV1::Excluded { gate } =
            classify_module_content_draft_v1(draft, &environment()).unwrap()
        else {
            panic!("inline cfg(any()) must exclude");
        };
        assert_eq!(gate.inner_cfg_sites.len(), 1);
        assert_eq!(
            gate.inner_cfg_sites[0].source_range.byte_start,
            parent.find("cfg(any())").unwrap()
        );
        assert_eq!(gate.inner_cfg_sites[0].syntax, "cfg(any())");
    }

    fn classify(source: &str) -> Result<ClassifiedModuleContentDraftV1, ModuleContentDraftErrorV1> {
        classify_with(ModuleContentCandidateIdV1::Root, source_surface(), source)
    }

    fn classify_with(
        candidate_id: ModuleContentCandidateIdV1,
        surface: ModuleContentDefiningSurfaceV1,
        source: &str,
    ) -> Result<ClassifiedModuleContentDraftV1, ModuleContentDraftErrorV1> {
        let draft = parse_module_content_draft_v1(candidate_id, surface, "src/lib.rs", source)?;
        classify_module_content_draft_v1(draft, &environment())
    }

    fn source_surface() -> ModuleContentDefiningSurfaceV1 {
        ModuleContentDefiningSurfaceV1::SourceFile {
            source_path_workspace_relative: "src/lib.rs".to_string(),
            content_digest: "fnv1a64:content-draft".to_string(),
        }
    }

    fn environment() -> CfgEvaluationEnvironmentV1 {
        let schema = parse_and_verify_profile_schema_v1(PROFILES).unwrap();
        let profile = schema
            .profiles
            .iter()
            .find(|profile| profile.profile_id == "host-default-dev")
            .unwrap();
        CfgEvaluationEnvironmentV1::from_profile_input(profile)
    }
}
