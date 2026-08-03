//! Builder-free fixed-shell If recipe schema.

use serde::{Deserialize, Serialize};

use super::ids::{IfBindingKeyV1, IfBlockKeyV1, IfItemKeyV1, IfValueKeyV1};

pub(crate) const IF_RECIPE_SCHEMA_VERSION_V1: u16 = 1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfRecipeArtifactV1 {
    pub(crate) schema_version: u16,
    pub(crate) provenance: IfRecipeProvenanceV1,
    pub(crate) source_binding: IfRecipeSourceBindingV1,
    pub(crate) recipe: IfRecipeV1,
}

impl IfRecipeArtifactV1 {
    pub(crate) fn new(
        provenance: IfRecipeProvenanceV1,
        source_binding: IfRecipeSourceBindingV1,
        recipe: IfRecipeV1,
    ) -> Self {
        Self {
            schema_version: IF_RECIPE_SCHEMA_VERSION_V1,
            provenance,
            source_binding,
            recipe,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfRecipeProvenanceV1 {
    pub(crate) profile: IfRecipeProfileV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IfRecipeProfileV1 {
    ResolvedTrivialExplicitElse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum IfRecipeSourceOwnerV1 {
    FunctionBody {
        compilation_unit_ordinal: u32,
        function_ordinal: u32,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum IfSourcePathStepV1 {
    BodyItem { index: u32 },
    IfCondition,
    IfThenItem { index: u32 },
    IfElseItem { index: u32 },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfSourcePathV1 {
    pub(crate) steps: Vec<IfSourcePathStepV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IfSourceClaimRoleV1 {
    IfNode,
    Condition,
    ThenAssignment,
    ElseAssignment,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfSourceClaimV1 {
    pub(crate) role: IfSourceClaimRoleV1,
    pub(crate) path: IfSourcePathV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfRecipeSourceBindingV1 {
    pub(crate) owner: IfRecipeSourceOwnerV1,
    pub(crate) claims: Vec<IfSourceClaimV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IfBlockRoleV1 {
    Condition,
    Then,
    Else,
    Continuation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IfElseDispositionV1 {
    Explicit,
    ImplicitFallthrough,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IfValueClassV1 {
    I64,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IfBindingRoleV1 {
    Input,
    MergeTarget,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfRecipeBindingV1 {
    pub(crate) key: IfBindingKeyV1,
    pub(crate) role: IfBindingRoleV1,
    pub(crate) class: IfValueClassV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfRecipeValueV1 {
    pub(crate) key: IfValueKeyV1,
    pub(crate) class: IfValueClassV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IfBinaryOpV1 {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    BitAnd,
    BitOr,
    BitXor,
    Shl,
    Shr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum IfCompareOpV1 {
    Less,
    LessEqual,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum IfOperationV1 {
    ReadBinding {
        binding: IfBindingKeyV1,
        result: IfValueKeyV1,
    },
    ConstI64 {
        result: IfValueKeyV1,
        value: i64,
    },
    ConstBool {
        result: IfValueKeyV1,
        value: bool,
    },
    BinaryI64 {
        op: IfBinaryOpV1,
        left: IfValueKeyV1,
        right: IfValueKeyV1,
        result: IfValueKeyV1,
    },
    CompareI64 {
        op: IfCompareOpV1,
        left: IfValueKeyV1,
        right: IfValueKeyV1,
        result: IfValueKeyV1,
    },
    WriteBinding {
        binding: IfBindingKeyV1,
        value: IfValueKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfRecipeBlockV1 {
    pub(crate) key: IfBlockKeyV1,
    pub(crate) role: IfBlockRoleV1,
    pub(crate) items: Vec<IfRecipeItemRowV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfRecipeItemRowV1 {
    pub(crate) key: IfItemKeyV1,
    pub(crate) operation: IfOperationV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfJoinRowV1 {
    pub(crate) binding: IfBindingKeyV1,
    pub(crate) class: IfValueClassV1,
    pub(crate) entry_value: IfValueKeyV1,
    pub(crate) then_value: IfValueKeyV1,
    pub(crate) else_value: IfValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfContinuationV1 {
    pub(crate) required_read: IfBindingKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct IfRecipeV1 {
    pub(crate) condition_block: IfRecipeBlockV1,
    pub(crate) then_block: IfRecipeBlockV1,
    pub(crate) else_block: Option<IfRecipeBlockV1>,
    pub(crate) continuation_block: IfRecipeBlockV1,
    pub(crate) else_disposition: IfElseDispositionV1,
    pub(crate) condition: IfValueKeyV1,
    pub(crate) inputs: Vec<IfValueKeyV1>,
    pub(crate) bindings: Vec<IfRecipeBindingV1>,
    pub(crate) values: Vec<IfRecipeValueV1>,
    pub(crate) joins: Vec<IfJoinRowV1>,
    pub(crate) continuation: IfContinuationV1,
}
