//! MIR JSON export model boundary.
//!
//! This module is intentionally passive. The main crate still projects
//! `MirModule` into JSON, while a future backend-side crate may own
//! serialization of a JSON-ready export model.

#![allow(dead_code)] // Passive boundary vocabulary; wired by later split rows.

use serde_json::{json, Value};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MirJsonExportSchema {
    LegacyV0,
    V1,
}

impl MirJsonExportSchema {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::LegacyV0 => "legacy_v0",
            Self::V1 => "v1",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum MirJsonExportRootKind {
    FunctionsOnly,
    SchemaVersioned,
}

impl MirJsonExportRootKind {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::FunctionsOnly => "functions_only",
            Self::SchemaVersioned => "schema_versioned",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MirJsonExportModelSummary {
    pub schema: MirJsonExportSchema,
    pub root_kind: MirJsonExportRootKind,
    pub function_count: usize,
    pub root_metadata_entry_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MirJsonFunctionExportSummary {
    pub name: String,
    pub param_count: usize,
    pub block_count: usize,
    pub instruction_count: usize,
    pub metadata_entry_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MirJsonExportSurface {
    pub key: String,
    pub value: Value,
}

impl MirJsonExportSurface {
    pub(crate) fn new(key: impl Into<String>, value: Value) -> Self {
        Self {
            key: key.into(),
            value,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MirJsonExportInstruction {
    pub payload: Value,
}

impl MirJsonExportInstruction {
    pub(crate) fn new(payload: Value) -> Self {
        Self { payload }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MirJsonExportBlock {
    pub id: u32,
    pub instructions: Vec<MirJsonExportInstruction>,
}

impl MirJsonExportBlock {
    pub(crate) fn new(id: u32, instructions: Vec<MirJsonExportInstruction>) -> Self {
        Self { id, instructions }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MirJsonExportFunction {
    pub name: String,
    pub params: Vec<u32>,
    pub blocks: Vec<MirJsonExportBlock>,
    pub metadata: Vec<MirJsonExportSurface>,
    pub attrs: Value,
}

impl MirJsonExportFunction {
    pub(crate) fn new(
        name: impl Into<String>,
        params: Vec<u32>,
        blocks: Vec<MirJsonExportBlock>,
        metadata: Vec<MirJsonExportSurface>,
        attrs: Value,
    ) -> Self {
        Self {
            name: name.into(),
            params,
            blocks,
            metadata,
            attrs,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct MirJsonExportDocument {
    pub schema: MirJsonExportSchema,
    pub root_kind: MirJsonExportRootKind,
    pub root_metadata: Vec<MirJsonExportSurface>,
    pub functions: Vec<MirJsonExportFunction>,
}

impl MirJsonExportDocument {
    pub(crate) fn new(
        schema: MirJsonExportSchema,
        root_kind: MirJsonExportRootKind,
        root_metadata: Vec<MirJsonExportSurface>,
        functions: Vec<MirJsonExportFunction>,
    ) -> Self {
        Self {
            schema,
            root_kind,
            root_metadata,
            functions,
        }
    }
}

pub(crate) fn serialize_document(document: &MirJsonExportDocument) -> Value {
    let functions: Vec<_> = document
        .functions
        .iter()
        .map(|function| {
            json!({
                "name": function.name.clone(),
                "params": function.params.clone(),
                "blocks": function
                    .blocks
                    .iter()
                    .map(|block| {
                        json!({
                            "id": block.id,
                            "instructions": block
                                .instructions
                                .iter()
                                .map(|instruction| instruction.payload.clone())
                                .collect::<Vec<_>>()
                        })
                    })
                    .collect::<Vec<_>>(),
                "metadata": surfaces_to_object(&function.metadata),
                "attrs": function.attrs.clone()
            })
        })
        .collect();

    let mut root = match document.root_kind {
        MirJsonExportRootKind::FunctionsOnly => json!({ "functions": functions }),
        MirJsonExportRootKind::SchemaVersioned => json!({
            "schema_version": 1,
            "kind": "MirModule",
            "functions": functions
        }),
    };

    if let Some(obj) = root.as_object_mut() {
        for surface in &document.root_metadata {
            obj.insert(surface.key.clone(), surface.value.clone());
        }
    }

    root
}

fn surfaces_to_object(surfaces: &[MirJsonExportSurface]) -> Value {
    let mut object = serde_json::Map::new();
    for surface in surfaces {
        object.insert(surface.key.clone(), surface.value.clone());
    }
    Value::Object(object)
}

impl MirJsonFunctionExportSummary {
    pub(crate) fn new(
        name: impl Into<String>,
        param_count: usize,
        block_count: usize,
        instruction_count: usize,
        metadata_entry_count: usize,
    ) -> Self {
        Self {
            name: name.into(),
            param_count,
            block_count,
            instruction_count,
            metadata_entry_count,
        }
    }
}

pub(crate) fn summarize_function(
    name: impl Into<String>,
    param_count: usize,
    block_count: usize,
    instruction_count: usize,
    metadata_entry_count: usize,
) -> MirJsonFunctionExportSummary {
    MirJsonFunctionExportSummary::new(
        name,
        param_count,
        block_count,
        instruction_count,
        metadata_entry_count,
    )
}

impl MirJsonExportModelSummary {
    pub(crate) fn new(
        schema: MirJsonExportSchema,
        root_kind: MirJsonExportRootKind,
        function_count: usize,
        root_metadata_entry_count: usize,
    ) -> Self {
        Self {
            schema,
            root_kind,
            function_count,
            root_metadata_entry_count,
        }
    }

    pub(crate) fn is_schema_versioned(&self) -> bool {
        self.root_kind == MirJsonExportRootKind::SchemaVersioned
    }
}

pub(crate) fn summarize_root(
    schema_v1_enabled: bool,
    function_count: usize,
    root_metadata_entry_count: usize,
) -> MirJsonExportModelSummary {
    let (schema, root_kind) = if schema_v1_enabled {
        (
            MirJsonExportSchema::V1,
            MirJsonExportRootKind::SchemaVersioned,
        )
    } else {
        (
            MirJsonExportSchema::LegacyV0,
            MirJsonExportRootKind::FunctionsOnly,
        )
    };

    MirJsonExportModelSummary::new(schema, root_kind, function_count, root_metadata_entry_count)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn export_schema_names_are_stable() {
        assert_eq!(MirJsonExportSchema::LegacyV0.as_str(), "legacy_v0");
        assert_eq!(MirJsonExportSchema::V1.as_str(), "v1");
        assert_eq!(
            MirJsonExportRootKind::FunctionsOnly.as_str(),
            "functions_only"
        );
        assert_eq!(
            MirJsonExportRootKind::SchemaVersioned.as_str(),
            "schema_versioned"
        );
    }

    #[test]
    fn export_model_summary_is_passive_boundary_vocabulary() {
        let summary = MirJsonExportModelSummary::new(
            MirJsonExportSchema::V1,
            MirJsonExportRootKind::SchemaVersioned,
            3,
            17,
        );

        assert!(summary.is_schema_versioned());
        assert_eq!(summary.function_count, 3);
        assert_eq!(summary.root_metadata_entry_count, 17);
    }

    #[test]
    fn summarize_root_tracks_schema_switch_without_json_projection() {
        let legacy = summarize_root(false, 2, 5);
        assert_eq!(legacy.schema, MirJsonExportSchema::LegacyV0);
        assert_eq!(legacy.root_kind, MirJsonExportRootKind::FunctionsOnly);
        assert!(!legacy.is_schema_versioned());

        let v1 = summarize_root(true, 2, 5);
        assert_eq!(v1.schema, MirJsonExportSchema::V1);
        assert_eq!(v1.root_kind, MirJsonExportRootKind::SchemaVersioned);
        assert!(v1.is_schema_versioned());
        assert_eq!(v1.function_count, 2);
        assert_eq!(v1.root_metadata_entry_count, 5);
    }

    #[test]
    fn function_export_summary_is_passive_boundary_vocabulary() {
        let summary = MirJsonFunctionExportSummary::new("Main.main/0", 0, 2, 7, 4);

        assert_eq!(summary.name, "Main.main/0");
        assert_eq!(summary.param_count, 0);
        assert_eq!(summary.block_count, 2);
        assert_eq!(summary.instruction_count, 7);
        assert_eq!(summary.metadata_entry_count, 4);
    }

    #[test]
    fn summarize_function_keeps_projection_counts_without_json_payload() {
        let summary = summarize_function("Main.helper/1", 1, 3, 11, 6);

        assert_eq!(summary.name, "Main.helper/1");
        assert_eq!(summary.param_count, 1);
        assert_eq!(summary.block_count, 3);
        assert_eq!(summary.instruction_count, 11);
        assert_eq!(summary.metadata_entry_count, 6);
    }

    #[test]
    fn export_document_dto_keeps_json_ready_payloads_without_mir_types() {
        let instruction = MirJsonExportInstruction::new(json!({
            "op": "const",
            "dst": 1,
            "value": {"type": "i64", "value": 42}
        }));
        let block = MirJsonExportBlock::new(0, vec![instruction]);
        let function = MirJsonExportFunction::new(
            "main",
            vec![0],
            vec![block],
            vec![MirJsonExportSurface::new("lowering_plan", json!([]))],
            json!({"runes": []}),
        );
        let document = MirJsonExportDocument::new(
            MirJsonExportSchema::V1,
            MirJsonExportRootKind::SchemaVersioned,
            vec![MirJsonExportSurface::new("cfg", json!({"blocks": []}))],
            vec![function],
        );

        assert_eq!(document.schema, MirJsonExportSchema::V1);
        assert_eq!(document.root_metadata[0].key, "cfg");
        assert_eq!(document.functions[0].name, "main");
        assert_eq!(
            document.functions[0].blocks[0].instructions[0].payload["op"],
            "const"
        );
    }

    #[test]
    fn serialize_document_matches_legacy_root_shape() {
        let document = MirJsonExportDocument::new(
            MirJsonExportSchema::LegacyV0,
            MirJsonExportRootKind::FunctionsOnly,
            vec![MirJsonExportSurface::new("cfg", json!({"blocks": []}))],
            vec![MirJsonExportFunction::new(
                "main",
                vec![0],
                vec![MirJsonExportBlock::new(
                    0,
                    vec![MirJsonExportInstruction::new(json!({"op": "ret"}))],
                )],
                vec![MirJsonExportSurface::new("lowering_plan", json!([]))],
                json!({"runes": []}),
            )],
        );

        let root = serialize_document(&document);

        assert!(root.get("schema_version").is_none());
        assert_eq!(root["cfg"], json!({"blocks": []}));
        assert_eq!(root["functions"][0]["name"], "main");
        assert_eq!(root["functions"][0]["metadata"]["lowering_plan"], json!([]));
    }

    #[test]
    fn serialize_document_matches_v1_root_shape() {
        let document = MirJsonExportDocument::new(
            MirJsonExportSchema::V1,
            MirJsonExportRootKind::SchemaVersioned,
            vec![],
            vec![],
        );

        let root = serialize_document(&document);

        assert_eq!(root["schema_version"], 1);
        assert_eq!(root["kind"], "MirModule");
        assert_eq!(root["functions"], json!([]));
    }
}
