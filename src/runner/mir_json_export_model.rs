//! MIR JSON export model boundary.
//!
//! This module is intentionally passive. The main crate still projects
//! `MirModule` into JSON, while a future backend-side crate may own
//! serialization of a JSON-ready export model.

#![allow(dead_code)] // Passive boundary vocabulary; wired by later split rows.

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
}
