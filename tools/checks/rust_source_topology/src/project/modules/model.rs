use serde::Serialize;

use crate::{RustSourceTopologyV1, SourceRangeV1};

use crate::project::CfgAttributeStreamDecisionV1;

pub const DECLARED_MODULE_TOPOLOGY_SCHEMA_V2: &str = "declared-module-topology-v2";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredModuleTopologyV1 {
    pub schema: &'static str,
    pub schema_version: u32,
    pub profile_id: String,
    pub package_key: String,
    pub target_key: String,
    pub root_instance_id: String,
    pub module_instances: Box<[DeclaredModuleInstanceV1]>,
    pub module_edges: Box<[DeclaredModuleEdgeV1]>,
    pub include_edges: Box<[DeclaredIncludeEdgeV1]>,
    pub source_observations: Box<[ModuleSourceObservationV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredModuleInstanceV1 {
    pub instance_id: String,
    pub parent_edge_id: Option<String>,
    pub module_syntax_path: String,
    pub kind: ModuleInstanceKindV1,
    pub source_path_workspace_relative: String,
    pub canonical_source_path_workspace_relative: String,
    pub source_observation_id: String,
    pub inline_body_range: Option<SourceRangeV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleInstanceKindV1 {
    Root,
    Inline,
    OrdinaryFile,
    OrdinaryModFile,
    LiteralPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredModuleEdgeV1 {
    pub edge_id: String,
    pub parent_instance_id: String,
    pub declaration_source_observation_id: String,
    pub declaration_range: SourceRangeV1,
    pub declared_ident_syntax: String,
    pub semantic_segment: String,
    pub kind: ModuleEdgeKindV1,
    pub active_literal_path: Option<String>,
    pub cfg_decision: CfgAttributeStreamDecisionV1,
    pub child_instance_id: Option<String>,
    pub selected_source_path_workspace_relative: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct DeclaredIncludeEdgeV1 {
    pub include_edge_id: String,
    pub owning_module_instance_id: String,
    pub parent_source_observation_id: String,
    pub parent_include_edge_id: Option<String>,
    pub invocation_range: SourceRangeV1,
    pub cfg_decision: CfgAttributeStreamDecisionV1,
    pub literal_path: Option<String>,
    pub selected_source_path_workspace_relative: Option<String>,
    pub child_source_observation_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ModuleEdgeKindV1 {
    Inline,
    Ordinary,
    LiteralPath,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ModuleSourceObservationV1 {
    pub source_observation_id: String,
    pub module_instance_id: String,
    pub parent_include_edge_id: Option<String>,
    pub source_path_workspace_relative: String,
    pub canonical_source_path_workspace_relative: String,
    pub topology: RustSourceTopologyV1,
}
