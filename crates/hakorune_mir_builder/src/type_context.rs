/*!
 * TypeContext - Type information management for MirBuilder
 *
 * Phase 136 follow-up: Extract type-related fields from MirBuilder
 * to improve code organization and testability.
 *
 * Consolidates:
 * - value_types: ValueId -> MirType mapping
 * - value_kinds: ValueId -> MirValueKind mapping (Phase 26-A)
 * - value_origin_newbox: ValueId -> Box class name origin tracking
 */

use hakorune_mir_core::{MirType, MirValueKind, ValueId};
use std::collections::{BTreeMap, HashMap};

/// Type information context for MIR builder
///
/// Manages all type-related mappings and origin tracking for ValueIds.
#[derive(Debug, Default)]
pub struct TypeContext {
    /// Optional per-value type annotations (MIR-level): ValueId -> MirType
    /// Phase 25.1: BTreeMap for deterministic iteration
    pub value_types: BTreeMap<ValueId, MirType>,

    /// Phase 26-A: ValueId type kind mapping (type safety enhancement)
    /// ValueId -> MirValueKind mapping
    /// - GUARD bug prevention: Distinguish Parameter vs Local for ValueId(0)
    /// - Default: Unregistered ValueIds are treated as Temporary
    pub value_kinds: HashMap<ValueId, MirValueKind>,

    /// Origin tracking for simple optimizations (e.g., object.method after new)
    /// Maps a ValueId to the class name if it was produced by NewBox of that class
    /// Phase 25.1: BTreeMap for deterministic iteration
    pub value_origin_newbox: BTreeMap<ValueId, String>,

    /// String literal payloads keyed by SSA value for conservative literal-aware
    /// collection typing.
    pub string_literals: BTreeMap<ValueId, String>,

    /// Receiver-local homogeneous MapBox value type facts.
    pub map_value_types: BTreeMap<ValueId, MirType>,

    /// Receiver-local literal-key MapBox value facts.
    pub map_literal_value_types: BTreeMap<(ValueId, String), MirType>,
}

#[derive(Debug, Default)]
pub struct TypeContextSnapshot {
    value_types: BTreeMap<ValueId, MirType>,
    value_kinds: HashMap<ValueId, MirValueKind>,
    value_origin_newbox: BTreeMap<ValueId, String>,
    string_literals: BTreeMap<ValueId, String>,
    map_value_types: BTreeMap<ValueId, MirType>,
    map_literal_value_types: BTreeMap<(ValueId, String), MirType>,
}

#[allow(dead_code)]
impl TypeContext {
    /// Create a new empty TypeContext
    pub fn new() -> Self {
        Self::default()
    }

    /// Get the type annotation for a ValueId, if present
    pub fn get_type(&self, value_id: ValueId) -> Option<&MirType> {
        self.value_types.get(&value_id)
    }

    /// Set the type annotation for a ValueId
    #[track_caller]
    pub fn set_type(&mut self, value_id: ValueId, ty: MirType) {
        self.value_types.insert(value_id, ty);
    }

    /// Get the value kind for a ValueId, defaulting to Temporary if not registered
    pub fn get_kind(&self, value_id: ValueId) -> MirValueKind {
        self.value_kinds
            .get(&value_id)
            .copied()
            .unwrap_or(MirValueKind::Temporary)
    }

    /// Set the value kind for a ValueId
    pub fn set_kind(&mut self, value_id: ValueId, kind: MirValueKind) {
        self.value_kinds.insert(value_id, kind);
    }

    /// Get the origin box class name for a ValueId, if tracked
    pub fn get_origin_box(&self, value_id: ValueId) -> Option<&str> {
        self.value_origin_newbox.get(&value_id).map(|s| s.as_str())
    }

    /// Set the origin box class name for a ValueId
    pub fn set_origin_box(&mut self, value_id: ValueId, class_name: String) {
        self.value_origin_newbox.insert(value_id, class_name);
    }

    /// Clear all origin box mappings (useful for cleanup)
    pub fn clear_origin_boxes(&mut self) {
        self.value_origin_newbox.clear();
    }

    /// Snapshot and clear the type context (function-scope isolation).
    pub fn take_snapshot(&mut self) -> TypeContextSnapshot {
        TypeContextSnapshot {
            value_types: std::mem::take(&mut self.value_types),
            value_kinds: std::mem::take(&mut self.value_kinds),
            value_origin_newbox: std::mem::take(&mut self.value_origin_newbox),
            string_literals: std::mem::take(&mut self.string_literals),
            map_value_types: std::mem::take(&mut self.map_value_types),
            map_literal_value_types: std::mem::take(&mut self.map_literal_value_types),
        }
    }

    /// Restore a previously captured snapshot.
    pub fn restore_snapshot(&mut self, snapshot: TypeContextSnapshot) {
        self.value_types = snapshot.value_types;
        self.value_kinds = snapshot.value_kinds;
        self.value_origin_newbox = snapshot.value_origin_newbox;
        self.string_literals = snapshot.string_literals;
        self.map_value_types = snapshot.map_value_types;
        self.map_literal_value_types = snapshot.map_literal_value_types;
    }
}
