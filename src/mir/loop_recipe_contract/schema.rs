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
    pub(super) schema_version: u16,
    pub(super) provenance: LoopRecipeProvenanceV1,
    pub(super) source_binding: LoopRecipeSourceBindingV1,
    pub(super) recipe: LoopRecipeV1,
}

impl LoopRecipeArtifactV1 {
    /// Assembles the portable wire without exposing source/provenance fields to
    /// semantic or physical consumers.
    pub(crate) fn new(
        provenance: LoopRecipeProvenanceV1,
        source_binding: LoopRecipeSourceBindingV1,
        recipe: LoopRecipeV1,
    ) -> Self {
        Self {
            schema_version: LOOP_RECIPE_SCHEMA_VERSION_V1,
            provenance,
            source_binding,
            recipe,
        }
    }

    pub(crate) fn recipe(&self) -> &LoopRecipeV1 {
        &self.recipe
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeProvenanceV1 {
    pub(crate) producer_route: LoopRouteId,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeSourceBindingV1 {
    pub(crate) owner: LoopRecipeSourceOwnerV1,
    pub(crate) loops: Vec<LoopNodeSourceBindingV1>,
}

impl LoopRecipeSourceBindingV1 {
    pub(crate) fn new(owner: LoopRecipeSourceOwnerV1, loops: Vec<LoopNodeSourceBindingV1>) -> Self {
        Self { owner, loops }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
/// Declared-function identity claimed by the wire artifact.
///
/// These ordinals are structural coordinates, not proof that the declaration
/// exists or owns the claimed source paths.
pub(crate) enum LoopRecipeSourceOwnerV1 {
    FunctionBody {
        compilation_unit_ordinal: u32,
        function_ordinal: u32,
    },
}

impl LoopRecipeSourceOwnerV1 {
    pub(crate) const fn function_body(
        compilation_unit_ordinal: u32,
        function_ordinal: u32,
    ) -> Self {
        Self::FunctionBody {
            compilation_unit_ordinal,
            function_ordinal,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopNodeSourceBindingV1 {
    pub(crate) loop_key: LoopNodeKeyV1,
    pub(crate) path: LoopSourcePathV1,
}

impl LoopNodeSourceBindingV1 {
    pub(crate) fn new(loop_key: LoopNodeKeyV1, path: LoopSourcePathV1) -> Self {
        Self { loop_key, path }
    }
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
    pub(crate) condition: LoopConditionV1,
    pub(crate) body: LoopBlockKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopSourcePathV1 {
    pub(crate) steps: Vec<LoopSourcePathStepV1>,
}

impl LoopSourcePathV1 {
    pub(crate) fn new(steps: Vec<LoopSourcePathStepV1>) -> Self {
        Self { steps }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum LoopSourcePathStepV1 {
    BodyItem { index: u32 },
    ScopeBodyItem { index: u32 },
    LoopBodyItem { index: u32 },
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
