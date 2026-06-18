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
}
