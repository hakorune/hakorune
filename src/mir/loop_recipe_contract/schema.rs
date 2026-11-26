//! Builder-free recursive Loop recipe schema.

use serde::{Deserialize, Serialize};

use super::ids::{
    LoopBindingKeyV1, LoopNodeKeyV1, LoopValueKeyV1,
};
use super::producer_id::LoopRecipeProducerIdV1;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeProvenanceV1 {
    pub(crate) producer_id: LoopRecipeProducerIdV1,
}

impl LoopRecipeProvenanceV1 {
    pub(crate) const fn new(producer_id: LoopRecipeProducerIdV1) -> Self {
        Self { producer_id }
    }
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
    IfThenItem { index: u32 },
    IfElseItem { index: u32 },
}

/// Root input whose binding is readable but never carried or written.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct LoopRecipeReadOnlyInputV1<Class> {
    pub(crate) owner_loop: LoopNodeKeyV1,
    pub(crate) binding: LoopBindingKeyV1,
    pub(crate) class: Class,
    pub(crate) entry_value: LoopValueKeyV1,
}
