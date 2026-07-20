//! CONTENTCFG0's disconnected inner-content candidate vocabulary.
//!
//! A content gate is deliberately not a module instance. It records the
//! defining source surface and its ordered inner CFG stream before later R0/I0
//! decide whether direct items may issue a child module instance.

use serde::Serialize;

use crate::project::{CfgAttributeStreamDecisionV1, CfgAttributeStreamInputRowV1};
use crate::SourceRangeV1;

/// Identifies the one source-content surface considered before instance issue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleContentCandidateIdV1 {
    Root,
    ModuleEdge { edge_id: String },
}

/// Bounded source evidence that defines a candidate's direct item surface.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleContentDefiningSurfaceV1 {
    SourceFile {
        source_path_workspace_relative: String,
        content_digest: String,
    },
    InlineBody {
        parent_source_observation_id: String,
        body_range: SourceRangeV1,
    },
}

/// The complete ordered inner-CFG decision for one candidate surface.
///
/// This product has no traversal consumer through CONTENTCFG0-S0. R0 will
/// construct it from a private parsed-content draft, and I0 alone may use an
/// Included gate to expose direct items.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredModuleContentGateV1 {
    pub candidate_id: ModuleContentCandidateIdV1,
    pub defining_surface: ModuleContentDefiningSurfaceV1,
    pub inner_cfg_sites: Box<[CfgAttributeStreamInputRowV1]>,
    pub cfg_decision: CfgAttributeStreamDecisionV1,
}
