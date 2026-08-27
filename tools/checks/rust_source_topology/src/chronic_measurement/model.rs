use serde::{Deserialize, Serialize};

use crate::model::SourceRangeV1;

pub const CHRONIC_MEASUREMENT_SCHEMA_V1: &str = "chronic-measurement-v1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChronicMeasurementReportV1 {
    pub schema: &'static str,
    pub schema_version: u32,
    pub scanner_version: String,
    pub scope_id: String,
    pub scope_manifest_hash: String,
    pub source_scope_hash: String,
    pub evidence_hash: String,
    pub summary: ChronicSummaryV1,
    pub files: Vec<ChronicFileObservationV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Default)]
pub struct ChronicSummaryV1 {
    pub panic_count: usize,
    pub unwrap_count: usize,
    pub expect_count: usize,
    pub todo_count: usize,
    pub dead_code_allowance_count: usize,
    pub dead_code_allowance_line_count: usize,
    pub unclassified_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChronicFileObservationV1 {
    pub path: String,
    pub source_digest: String,
    pub compile_domain: String,
    pub role: String,
    pub observations: Vec<ChronicObservationV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "row_kind", rename_all = "snake_case")]
pub enum ChronicObservationV1 {
    CallSite {
        metric: ChronicMetricV1,
        source_range: SourceRangeV1,
        item_key: String,
        direct_cfg_syntax: Vec<String>,
        inherited_cfg_syntax: Vec<String>,
    },
    DeadCodeAllowance {
        source_range: SourceRangeV1,
        target_range: SourceRangeV1,
        item_key: String,
        attribute_kind: ChronicAllowanceKindV1,
        raw_condition: Option<String>,
        direct_cfg_syntax: Vec<String>,
        inherited_cfg_syntax: Vec<String>,
    },
    ModuleEdge {
        edge_kind: ChronicModuleEdgeKindV1,
        source_range: SourceRangeV1,
        item_key: String,
        syntax: String,
    },
    OpaqueMacro {
        source_range: SourceRangeV1,
        item_key: String,
        syntax_name: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronicMetricV1 {
    Panic,
    Unwrap,
    Expect,
    Todo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronicAllowanceKindV1 {
    OuterAllow,
    InnerAllow,
    CfgAttrAllow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChronicModuleEdgeKindV1 {
    InlineModule,
    ExternalModule,
    PathAttributedExternalModule,
    IncludeMacro,
}
