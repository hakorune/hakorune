//! Builder-free recursive Loop recipe schema.

use serde::{Deserialize, Serialize};

use super::ids::{
    LoopBindingKeyV1, LoopBlockKeyV1, LoopCarrierKeyV1, LoopExitKeyV1, LoopItemKeyV1,
    LoopNodeKeyV1, LoopValueKeyV1,
};
use super::route_id::LoopRouteId;

pub(crate) const LOOP_RECIPE_SCHEMA_VERSION_V1: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeArtifactV1 {
    pub(crate) schema_version: u16,
    pub(crate) provenance: LoopRecipeProvenanceV1,
    pub(crate) recipe: LoopRecipeV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeProvenanceV1 {
    pub(crate) producer_route: LoopRouteId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeV1 {
    pub(crate) root_loop: LoopNodeKeyV1,
    pub(crate) loops: Vec<LoopNodeV1>,
    pub(crate) blocks: Vec<LoopRecipeBlockV1>,
    pub(crate) items: Vec<LoopRecipeItemRowV1>,
    pub(crate) bindings: Vec<LoopRecipeBindingV1>,
    pub(crate) values: Vec<LoopRecipeValueV1>,
    pub(crate) inputs: Vec<LoopValueKeyV1>,
    pub(crate) carriers: Vec<LoopRecipeCarrierV1>,
    pub(crate) exits: Vec<LoopRecipeExitV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopNodeV1 {
    pub(crate) key: LoopNodeKeyV1,
    pub(crate) parent: Option<LoopNodeKeyV1>,
    pub(crate) source: LoopSourcePathV1,
    pub(crate) condition: LoopConditionV1,
    pub(crate) body: LoopBlockKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopSourcePathV1 {
    pub(crate) steps: Vec<LoopSourcePathStepV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopSourcePathStepV1 {
    FunctionBody,
    Body { index: u32 },
    LoopBodyRoot,
    LoopBody { index: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopConditionV1 {
    Always,
    Predicate {
        block: LoopBlockKeyV1,
        value: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeBlockV1 {
    pub(crate) key: LoopBlockKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) items: Vec<LoopItemKeyV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeItemRowV1 {
    pub(crate) key: LoopItemKeyV1,
    pub(crate) item: LoopRecipeItemV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopRecipeItemV1 {
    Operation {
        operation: LoopOperationV1,
    },
    If {
        condition: LoopValueKeyV1,
        then_block: LoopBlockKeyV1,
        else_block: Option<LoopBlockKeyV1>,
    },
    Loop {
        loop_key: LoopNodeKeyV1,
    },
    Exit {
        exit: LoopExitKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopOperationV1 {
    ReadBinding {
        binding: LoopBindingKeyV1,
        result: LoopValueKeyV1,
    },
    ConstI64 {
        result: LoopValueKeyV1,
        value: i64,
    },
    BinaryI64 {
        op: LoopBinaryI64OpV1,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    CompareI64 {
        op: LoopCompareI64OpV1,
        left: LoopValueKeyV1,
        right: LoopValueKeyV1,
        result: LoopValueKeyV1,
    },
    WriteBinding {
        binding: LoopBindingKeyV1,
        value: LoopValueKeyV1,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopBinaryI64OpV1 {
    Add,
    Sub,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopCompareI64OpV1 {
    Less,
    LessEqual,
    Equal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopValueClassV1 {
    I64,
    Bool,
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeBindingV1 {
    pub(crate) key: LoopBindingKeyV1,
    pub(crate) label: String,
    pub(crate) class: LoopValueClassV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeValueV1 {
    pub(crate) key: LoopValueKeyV1,
    pub(crate) class: LoopValueClassV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeCarrierV1 {
    pub(crate) key: LoopCarrierKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) class: LoopValueClassV1,
    pub(crate) entry_value: LoopValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeExitV1 {
    pub(crate) key: LoopExitKeyV1,
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) kind: LoopExitKindV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopExitKindV1 {
    Break { target_loop: LoopNodeKeyV1 },
    Continue { target_loop: LoopNodeKeyV1 },
    Return { value: Option<LoopValueKeyV1> },
}
