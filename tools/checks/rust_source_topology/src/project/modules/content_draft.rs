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

use super::{
    DeclaredModuleContentGateV1, ModuleContentCandidateIdV1, ModuleContentDefiningSurfaceV1,
};
use super::super::inner_cfg_surface::collect_inner_topology_surface_from_parsed_file_v1;

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
        }
    }
}

impl std::error::Error for ModuleContentDraftErrorV1 {}

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
        crate::project::CfgDecisionStateV1::Unknown => {
            Err(ModuleContentDraftErrorV1::UnknownCfg {
                source_path_workspace_relative: path,
            })
        }
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

    fn classify(source: &str) -> Result<ClassifiedModuleContentDraftV1, ModuleContentDraftErrorV1> {
        let draft = parse_module_content_draft_v1(
            ModuleContentCandidateIdV1::Root,
            source_surface(),
            "src/lib.rs",
            source,
        )?;
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
