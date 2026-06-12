/*!
 * MIR-owned route plans for generic array/text read-side observers.
 *
 * This module owns the legality/provenance/consumer contract for routes such
 * as `array.get(i).indexOf(needle)`. Backends may consume this metadata to
 * select helper calls, but helper symbols and raw MIR window matching stay out
 * of the MIR contract.
 */

use super::{
    array_text_observer_region_contract::ArrayTextObserverExecutorContract, BasicBlockId, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverKind {
    IndexOf,
}

impl std::fmt::Display for ArrayTextObserverKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextObserverKind {
    fn as_str(self) -> &'static str {
        match self {
            Self::IndexOf => "indexof",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverConsumerShape {
    DirectScalar,
    FoundPredicate,
}

impl std::fmt::Display for ArrayTextObserverConsumerShape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextObserverConsumerShape {
    fn as_str(self) -> &'static str {
        match self {
            Self::DirectScalar => "direct_scalar",
            Self::FoundPredicate => "found_predicate",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverProofRegion {
    ArrayGetReceiverIndexOf,
}

impl std::fmt::Display for ArrayTextObserverProofRegion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextObserverProofRegion {
    fn as_str(self) -> &'static str {
        match self {
            Self::ArrayGetReceiverIndexOf => "array_get_receiver_indexof",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverPublicationBoundary {
    None,
}

impl std::fmt::Display for ArrayTextObserverPublicationBoundary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextObserverPublicationBoundary {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArrayTextObserverResultRepr {
    ScalarI64,
}

impl std::fmt::Display for ArrayTextObserverResultRepr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ArrayTextObserverResultRepr {
    fn as_str(self) -> &'static str {
        match self {
            Self::ScalarI64 => "scalar_i64",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayTextObserverArgRepr {
    Value,
    ConstUtf8 { text: String, byte_len: usize },
}

impl ArrayTextObserverArgRepr {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Value => "value",
            Self::ConstUtf8 { .. } => "const_utf8",
        }
    }

    fn is_const_utf8(&self) -> bool {
        matches!(self, Self::ConstUtf8 { .. })
    }

    fn text(&self) -> Option<&str> {
        match self {
            Self::ConstUtf8 { text, .. } => Some(text.as_str()),
            Self::Value => None,
        }
    }

    fn byte_len(&self) -> Option<usize> {
        match self {
            Self::ConstUtf8 { byte_len, .. } => Some(*byte_len),
            Self::Value => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayTextObserverRoute {
    block: BasicBlockId,
    observer_instruction_index: usize,
    get_block: BasicBlockId,
    get_instruction_index: usize,
    array_value: ValueId,
    index_value: ValueId,
    source_value: ValueId,
    observer_kind: ArrayTextObserverKind,
    observer_arg0: ValueId,
    observer_arg0_repr: ArrayTextObserverArgRepr,
    observer_arg0_keep_live: bool,
    result_value: ValueId,
    consumer_shape: ArrayTextObserverConsumerShape,
    proof_region: ArrayTextObserverProofRegion,
    publication_boundary: ArrayTextObserverPublicationBoundary,
    result_repr: ArrayTextObserverResultRepr,
    keep_get_live: bool,
    selected_route: &'static str,
    selected_bridge_symbol: &'static str,
    fallback_route: &'static str,
    fallback_policy: &'static str,
    executor_contract: Option<ArrayTextObserverExecutorContract>,
}

impl ArrayTextObserverRoute {
    pub fn block(&self) -> BasicBlockId {
        self.block
    }

    pub fn observer_instruction_index(&self) -> usize {
        self.observer_instruction_index
    }

    pub fn get_block(&self) -> BasicBlockId {
        self.get_block
    }

    pub fn get_instruction_index(&self) -> usize {
        self.get_instruction_index
    }

    pub fn array_value(&self) -> ValueId {
        self.array_value
    }

    pub fn index_value(&self) -> ValueId {
        self.index_value
    }

    pub fn source_value(&self) -> ValueId {
        self.source_value
    }

    pub fn observer_kind(&self) -> &'static str {
        self.observer_kind.as_str()
    }

    pub fn observer_arg0(&self) -> ValueId {
        self.observer_arg0
    }

    pub fn observer_arg0_repr_kind(&self) -> &'static str {
        self.observer_arg0_repr.kind()
    }

    pub fn observer_arg0_text(&self) -> Option<&str> {
        self.observer_arg0_repr.text()
    }

    pub fn observer_arg0_byte_len(&self) -> Option<usize> {
        self.observer_arg0_repr.byte_len()
    }

    pub fn observer_arg0_keep_live(&self) -> bool {
        self.observer_arg0_keep_live
    }

    pub fn result_value(&self) -> ValueId {
        self.result_value
    }

    pub fn consumer_shape(&self) -> &'static str {
        self.consumer_shape.as_str()
    }

    pub(crate) fn has_found_predicate_consumer(&self) -> bool {
        self.consumer_shape == ArrayTextObserverConsumerShape::FoundPredicate
    }

    pub(crate) fn observer_arg0_is_const_utf8(&self) -> bool {
        matches!(
            self.observer_arg0_repr,
            ArrayTextObserverArgRepr::ConstUtf8 { .. }
        )
    }

    pub fn proof_region(&self) -> &'static str {
        self.proof_region.as_str()
    }

    pub fn publication_boundary(&self) -> &'static str {
        self.publication_boundary.as_str()
    }

    pub fn result_repr(&self) -> &'static str {
        self.result_repr.as_str()
    }

    pub fn keep_get_live(&self) -> bool {
        self.keep_get_live
    }

    pub fn selected_route(&self) -> &'static str {
        self.selected_route
    }

    pub fn selected_bridge_symbol(&self) -> &'static str {
        self.selected_bridge_symbol
    }

    pub fn fallback_route(&self) -> &'static str {
        self.fallback_route
    }

    pub fn fallback_policy(&self) -> &'static str {
        self.fallback_policy
    }

    pub fn executor_contract(&self) -> Option<&ArrayTextObserverExecutorContract> {
        self.executor_contract.as_ref()
    }
}

mod routes;

pub use routes::{
    refresh_function_array_text_observer_routes, refresh_module_array_text_observer_routes,
};

#[cfg(test)]
mod tests;
