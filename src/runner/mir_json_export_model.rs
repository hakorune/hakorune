//! Compatibility facade for MIR JSON export DTO serialization.
//!
//! The main crate still owns projection from `MirModule` into JSON-ready DTOs.
//! The DTO model and serializer live in `hakorune-mir-json-emit` so future
//! crate split rows can keep serialization independent from MIR internals.

pub(crate) use hakorune_mir_json_emit::{
    serialize_document, summarize_function, summarize_root, MirJsonExportBlock,
    MirJsonExportDocument, MirJsonExportFunction, MirJsonExportInstruction, MirJsonExportSurface,
};
