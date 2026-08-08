//! Explicit V2 wire types for the typed Loop vocabulary.
//!
//! V2 is intentionally a separate wire.  The source-coordinate and
//! provenance records are unchanged structural records from V1; the logical
//! value/operation family is not widened in place.

use serde::{Deserialize, Serialize};

use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::schema::{LoopRecipeProvenanceV1, LoopRecipeSourceBindingV1};

pub(crate) const LOOP_RECIPE_SCHEMA_VERSION_V2: u16 = 2;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeArtifactV2 {
    pub(crate) schema_version: u16,
    pub(crate) provenance: LoopRecipeProvenanceV1,
    pub(crate) source_binding: LoopRecipeSourceBindingV1,
    pub(crate) recipe: LoopRecipeV2,
}

impl LoopRecipeArtifactV2 {
    pub(crate) fn new(
        provenance: LoopRecipeProvenanceV1,
        source_binding: LoopRecipeSourceBindingV1,
        recipe: LoopRecipeV2,
    ) -> Self {
        Self {
            schema_version: LOOP_RECIPE_SCHEMA_VERSION_V2,
            provenance,
            source_binding,
            recipe,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeV2 {
    pub(crate) root_loop: LoopNodeKeyV1,
    pub(crate) loops: Vec<LoopNodeV2>,
    pub(crate) blocks: Vec<LoopRecipeBlockV2>,
    pub(crate) items: Vec<LoopRecipeItemRowV2>,
    pub(crate) bindings: Vec<LoopRecipeBindingV2>,
    pub(crate) values: Vec<LoopRecipeValueV2>,
    pub(crate) inputs: Vec<LoopValueKeyV1>,
    pub(crate) carriers: Vec<LoopRecipeCarrierV2>,
    pub(crate) exits: Vec<LoopRecipeExitV2>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopNodeV2 {
    pub(crate) key: LoopNodeKeyV1,
    pub(crate) parent: Option<LoopNodeKeyV1>,
    pub(crate) condition: LoopConditionV2,
    pub(crate) body: LoopBlockKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopConditionV2 {
    Always,
    Predicate {
        block: LoopBlockKeyV1,
        value: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeBlockV2 {
    pub(crate) key: LoopBlockKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) items: Vec<LoopItemKeyV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeItemRowV2 {
    pub(crate) key: LoopItemKeyV1,
    pub(crate) item: LoopRecipeItemV2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopRecipeItemV2 {
    Operation { operation: LoopOperationV2 },
    If {
        condition: LoopValueKeyV1,
        then_block: LoopBlockKeyV1,
        else_block: Option<LoopBlockKeyV1>,
    },
    Loop { loop_key: LoopNodeKeyV1 },
    Exit { exit: LoopExitKeyV1 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopOperationV2 {
    ReadBinding {
        binding: LoopBindingKeyV1,
        result: LoopValueKeyV1,
    },
    ConstI64 {
        result: LoopValueKeyV1,
        value: i64,
    },
    BinaryI64 {
        op: LoopBinaryI64OpV2,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    CompareI64 {
        op: LoopCompareI64OpV2,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    WriteBinding {
        binding: LoopBindingKeyV1,
        value: LoopValueKeyV1,
    },
    CallSlot {
        receiver: Option<LoopValueKeyV1>,
        args: Vec<LoopValueKeyV1>,
        result: Option<LoopValueKeyV1>,
    },
    TextEq {
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopBinaryI64OpV2 {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopCompareI64OpV2 {
    Less,
    LessEqual,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopValueClassV2 {
    I64,
    Bool,
    Unit,
    Text,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeBindingV2 {
    pub(crate) key: LoopBindingKeyV1,
    pub(crate) label: String,
    pub(crate) class: LoopValueClassV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeValueV2 {
    pub(crate) key: LoopValueKeyV1,
    pub(crate) class: LoopValueClassV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeCarrierV2 {
    pub(crate) key: LoopCarrierKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) class: LoopValueClassV2,
    pub(crate) entry_value: LoopValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeExitV2 {
    pub(crate) key: LoopExitKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) kind: LoopExitKindV2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopExitKindV2 {
    Break { target_loop: LoopNodeKeyV1 },
    Continue { target_loop: LoopNodeKeyV1 },
    Return { value: Option<LoopValueKeyV1> },
}
