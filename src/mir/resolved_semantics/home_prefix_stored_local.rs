//! Stored-local equality for branch joins and the observation conversions.
use super::*;

impl OrdinaryObservation {
    pub(in crate::mir::resolved_semantics::home_new_prefix) fn is_trivial(&self) -> bool {
        match self {
            Self::Integer(_) | Self::Bool(_) | Self::TrivialLocal(..) => true,
            Self::Null | Self::Handle(_) | Self::TaggedDynamicLocal(_) | Self::BoundValue(_) => false,
        }
    }

    pub(in crate::mir::resolved_semantics::home_new_prefix) fn into_selected_argument(self) -> Option<SelectedNewArgumentKindV1> {
        match self {
            Self::Integer(value) => Some(SelectedNewArgumentKindV1::Integer(value)),
            Self::Bool(value) => Some(SelectedNewArgumentKindV1::Bool(value)),
            Self::Null => Some(SelectedNewArgumentKindV1::Null),
            Self::TrivialLocal(binding, _) => Some(SelectedNewArgumentKindV1::Local { binding }),
            Self::Handle(root) => Some(SelectedNewArgumentKindV1::Handle { binding: root }),
            Self::TaggedDynamicLocal(binding) => {
                Some(SelectedNewArgumentKindV1::BoundValue { binding })
            }
            Self::BoundValue(binding) => Some(SelectedNewArgumentKindV1::BoundValue { binding }),
        }
    }
}

pub(in crate::mir::resolved_semantics::home_new_prefix) fn stored_local_same(left: &StoredLocal, right: &StoredLocal) -> bool {
    match (left, right) {
        (StoredLocal::StaticText { definition: a }, StoredLocal::StaticText { definition: b }) => {
            a == b
        }
        (
            StoredLocal::Text {
                root: a,
                acquisition: ap,
            },
            StoredLocal::Text {
                root: b,
                acquisition: bp,
            },
        ) => a == b && ap.same_source(bp),
        (StoredLocal::LoopText { product: a }, StoredLocal::LoopText { product: b }) => {
            std::rc::Rc::ptr_eq(a, b)
        }
        (StoredLocal::TextBranch { source: a }, StoredLocal::TextBranch { source: b }) => {
            std::rc::Rc::ptr_eq(a, b)
        }
        (StoredLocal::NamedArrayHolder(a), StoredLocal::NamedArrayHolder(b)) => a == b,
        (
            StoredLocal::BorrowedArray {
                source: a,
                root: ar,
            },
            StoredLocal::BorrowedArray {
                source: b,
                root: br,
            },
        ) => a == b && ar == br,
        (
            StoredLocal::Home {
                acquisition: a,
                construction_source: ac,
                ready: ar,
            },
            StoredLocal::Home {
                acquisition: b,
                construction_source: bc,
                ready: br,
            },
        ) => {
            a == b
                && ar.len() == br.len()
                && ar.iter().all(|(field, frame)| {
                    br.get(field).is_some_and(|other| frame.same_source(other))
                })
                && match (ac, bc) {
                    (None, None) => true,
                    (Some(a), Some(b)) => std::rc::Rc::ptr_eq(a, b),
                    _ => false,
                }
        }
        (
            StoredLocal::ReceivedHandle { acquisition: a },
            StoredLocal::ReceivedHandle { acquisition: b },
        ) => a == b,
        (
            StoredLocal::ReceivedNullable { acquisition: a },
            StoredLocal::ReceivedNullable { acquisition: b },
        ) => a == b,
        (
            StoredLocal::DynamicInvocationResult { frame: a },
            StoredLocal::DynamicInvocationResult { frame: b },
        ) => std::rc::Rc::ptr_eq(a, b),
        (StoredLocal::Map, StoredLocal::Map) => true,
        (StoredLocal::BorrowedNullable(a), StoredLocal::BorrowedNullable(b)) => a == b,
        (StoredLocal::StoredMapGet(a), StoredLocal::StoredMapGet(b)) => std::rc::Rc::ptr_eq(a, b),
        (StoredLocal::ReceivedMixed(a), StoredLocal::ReceivedMixed(b)) => a == b,
        (StoredLocal::BorrowedMap, StoredLocal::BorrowedMap) => true,
        (StoredLocal::Consumed, StoredLocal::Consumed) => true,
        (StoredLocal::MovedOut, StoredLocal::MovedOut) => true,
        (StoredLocal::Shared { acquisition: a, payload_acquisition: ap, construction_source: ac, ready: ar },
         StoredLocal::Shared { acquisition: b, payload_acquisition: bp, construction_source: bc, ready: br }) =>
            a == b && ap == bp && std::rc::Rc::ptr_eq(ac, bc) && ar.len() == br.len()
                && ar.iter().all(|(field, frame)| br.get(field).is_some_and(|other| frame.same_source(other))),
        (StoredLocal::Handle(a), StoredLocal::Handle(b)) => a == b,
        (
            StoredLocal::FieldAlias {
                class: a,
                root: ar,
                borrow: ab,
            },
            StoredLocal::FieldAlias {
                class: b,
                root: br,
                borrow: bb,
            },
        ) => a == b && ar == br && ab == bb,
        (StoredLocal::Trivial(a), StoredLocal::Trivial(b)) => a == b,
        (StoredLocal::BoundValue, StoredLocal::BoundValue) => true,
        (StoredLocal::TaggedDynamic, StoredLocal::TaggedDynamic) => true,
        (StoredLocal::Null, StoredLocal::Null) => true,
        (StoredLocal::Uninitialized, StoredLocal::Uninitialized) => true,
        _ => false,
    }
}

pub(in crate::mir::resolved_semantics::home_new_prefix) fn stored_local_kind(local: Option<&StoredLocal>) -> &'static str {
    match local {
        Some(StoredLocal::Text { .. }) => "Text",
        Some(StoredLocal::TextBranch { .. }) => "TextBranch",
        Some(StoredLocal::LoopText { .. }) => "LoopText",
        Some(StoredLocal::StaticText { .. }) => "StaticText",
        Some(_) => "Other",
        None => "Absent",
    }
}
