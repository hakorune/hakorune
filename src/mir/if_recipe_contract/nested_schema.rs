//! Portable depth-one nested-If recipe contract.
//!
//! This is deliberately a separate shell from [`super::schema`].  The first
//! If recipe is a fixed four-block, leaf-operation contract; adding a child
//! node to that schema would silently widen its authority.  This schema owns
//! only one outer node, one inner child in the outer `then` branch, and the
//! portable value/binding identities needed to describe their joins.

use serde::{Deserialize, Serialize};

use super::schema::IfRecipeSourceOwnerV1;

pub(crate) const NESTED_IF_RECIPE_SCHEMA_VERSION_V1: u16 = 1;

macro_rules! nested_key {
    ($name:ident) => {
        #[derive(
            Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
        )]
        #[serde(transparent)]
        pub(crate) struct $name(u32);

        impl $name {
            pub(crate) const fn new(raw: u32) -> Self {
                Self(raw)
            }

            pub(crate) const fn raw(self) -> u32 {
                self.0
            }
        }
    };
}

nested_key!(NestedIfNodeKeyV1);
nested_key!(NestedIfBindingKeyV1);
nested_key!(NestedIfValueKeyV1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NestedIfRecipeProfileV1 {
    ResolvedTrivialExplicitElseDepthOne,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfRecipeArtifactV1 {
    pub(crate) schema_version: u16,
    pub(crate) provenance: NestedIfRecipeProvenanceV1,
    pub(crate) source_binding: NestedIfRecipeSourceBindingV1,
    pub(crate) recipe: NestedIfRecipeV1,
}

impl NestedIfRecipeArtifactV1 {
    pub(crate) fn new(
        provenance: NestedIfRecipeProvenanceV1,
        source_binding: NestedIfRecipeSourceBindingV1,
        recipe: NestedIfRecipeV1,
    ) -> Self {
        Self {
            schema_version: NESTED_IF_RECIPE_SCHEMA_VERSION_V1,
            provenance,
            source_binding,
            recipe,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfRecipeProvenanceV1 {
    pub(crate) profile: NestedIfRecipeProfileV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfSourcePathV1 {
    pub(crate) steps: Vec<NestedIfSourcePathStepV1>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NestedIfSourcePathStepV1 {
    BodyItem(u32),
    IfCondition,
    IfThenItem(u32),
    IfElseItem(u32),
    Value,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NestedIfSourceClaimRoleV1 {
    OuterIfNode,
    OuterCondition,
    InnerIfNode,
    InnerCondition,
    InnerThenAssignment,
    InnerElseAssignment,
    OuterElseAssignment,
    ContinuationRead,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfSourceClaimV1 {
    pub(crate) role: NestedIfSourceClaimRoleV1,
    pub(crate) path: NestedIfSourcePathV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfRecipeSourceBindingV1 {
    pub(crate) owner: IfRecipeSourceOwnerV1,
    pub(crate) claims: Vec<NestedIfSourceClaimV1>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NestedIfValueClassV1 {
    I64,
    Bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NestedIfBinaryOpV1 {
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
    Equal,
    NotEqual,
    Less,
    Greater,
    LessEqual,
    GreaterEqual,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(crate) enum NestedIfExprKindV1 {
    ReadBinding {
        binding: NestedIfBindingKeyV1,
    },
    ConstI64 {
        value: i64,
    },
    ConstBool {
        value: bool,
    },
    Binary {
        op: NestedIfBinaryOpV1,
        left: NestedIfValueKeyV1,
        right: NestedIfValueKeyV1,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfExprV1 {
    pub(crate) key: NestedIfValueKeyV1,
    pub(crate) class: NestedIfValueClassV1,
    pub(crate) kind: NestedIfExprKindV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfAssignmentV1 {
    pub(crate) binding: NestedIfBindingKeyV1,
    pub(crate) value: NestedIfValueKeyV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfBindingV1 {
    pub(crate) key: NestedIfBindingKeyV1,
    pub(crate) class: NestedIfValueClassV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfJoinRowV1 {
    pub(crate) binding: NestedIfBindingKeyV1,
    pub(crate) class: NestedIfValueClassV1,
    pub(crate) entry_value: NestedIfValueKeyV1,
    pub(crate) then_value: NestedIfValueKeyV1,
    pub(crate) else_value: NestedIfValueKeyV1,
    pub(crate) merge_value: NestedIfValueKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfNodeV1 {
    pub(crate) key: NestedIfNodeKeyV1,
    pub(crate) condition: NestedIfValueKeyV1,
    pub(crate) then_child: Option<NestedIfNodeKeyV1>,
    pub(crate) then_assignments: Vec<NestedIfAssignmentV1>,
    pub(crate) else_assignments: Vec<NestedIfAssignmentV1>,
    pub(crate) join: NestedIfJoinRowV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfContinuationV1 {
    pub(crate) binding: NestedIfBindingKeyV1,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct NestedIfRecipeV1 {
    pub(crate) nodes: Vec<NestedIfNodeV1>,
    pub(crate) expressions: Vec<NestedIfExprV1>,
    pub(crate) bindings: Vec<NestedIfBindingV1>,
    pub(crate) entry_value: NestedIfValueKeyV1,
    pub(crate) outer_merge_value: NestedIfValueKeyV1,
    pub(crate) continuation: NestedIfContinuationV1,
}
