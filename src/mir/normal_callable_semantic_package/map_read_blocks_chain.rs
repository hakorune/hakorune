//! Source issuer for the bounded `blocks[0].get("kind")` read chain.
//!
//! This stays beside the main Map-read Fact issuer because the parent module
//! is already near its source-size boundary.  It only issues the same
//! `MapLookup -> ArrayIndex -> MapLookup(TextView)` rows used by the landed
//! `functions[0].get("name")` chain; no new physical authority is introduced.

use super::{
    literal_integer, literal_string, unique_call, MapReadFactIssueV1, MapReadFactV1,
    MapReadOperandV1, MapReadOperationV1, MapReadResultClassV1,
};
use crate::mir::normal_callable_semantic_package::OrdinaryNewClaimLedgerV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    ArrayElementSource, MapEntryStoreClassV1, MapHomeEntry, MapHomeFlow, MapValueSource,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, OwnedExprSiteV1, ResolvedLexicalRefV1, ResolvedMethodCallReceiverSourceV1,
};
use std::collections::BTreeSet;

pub(super) fn has_blocks_kind_candidate(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    formal_binding: BindingRefV1,
) -> bool {
    let Some(first) = function.method_calls().find_map(|(_, call)| {
        (call.receiver()
            == ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                formal_binding,
            ))
            && call.selector() == "get"
            && call.arity() == 1
            && literal_string(function, call.arguments()[0].site()) == Some("blocks"))
        .then_some(call)
    }) else {
        return false;
    };
    let Some(second) = function.method_calls().find_map(|(_, call)| {
        (call.receiver_site() == first.site() && call.selector() == "get" && call.arity() == 1)
            .then_some(call)
    }) else {
        return false;
    };
    function.method_calls().any(|(_, call)| {
        call.receiver_site() == second.site() && call.selector() == "get" && call.arity() == 1
    })
}

pub(super) fn issue_blocks_kind_chain(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    formal_binding: BindingRefV1,
    call_site: OwnedExprSiteV1,
    argument_ordinal: u32,
    actual_map: &OwnedExprSiteV1,
    actual_flow: &MapHomeFlow,
    ledger: &OrdinaryNewClaimLedgerV1,
) -> Result<Vec<MapReadFactV1>, MapReadFactIssueV1> {
    let first = unique_call(
        function,
        |call| {
            call.receiver()
                == ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                    formal_binding,
                ))
                && call.selector() == "get"
                && call.arity() == 1
                && literal_string(function, call.arguments()[0].site()) == Some("blocks")
        },
        MapReadFactIssueV1::BlocksEntryMissing {
            site: actual_map.clone(),
        },
        MapReadFactIssueV1::FirstLookupDuplicate {
            owner: actual_map.owner(),
        },
    )?;
    let second = unique_call(
        function,
        |call| {
            call.receiver_site() == first.site() && call.selector() == "get" && call.arity() == 1
        },
        MapReadFactIssueV1::ArrayIndexMissing {
            owner: actual_map.owner(),
        },
        MapReadFactIssueV1::ArrayIndexDuplicate {
            owner: actual_map.owner(),
        },
    )?;
    if literal_integer(function, second.arguments()[0].site()) != Some(0) {
        return Err(MapReadFactIssueV1::ArrayIndexOperandMismatch {
            owner: actual_map.owner(),
        });
    }
    let third = unique_call(
        function,
        |call| {
            call.receiver_site() == second.site() && call.selector() == "get" && call.arity() == 1
        },
        MapReadFactIssueV1::KindEntryMissing {
            site: actual_map.clone(),
        },
        MapReadFactIssueV1::KindEntryDuplicate {
            site: actual_map.clone(),
        },
    )?;
    if literal_string(function, third.arguments()[0].site()) != Some("kind") {
        return Err(MapReadFactIssueV1::KindEntryOperandMismatch {
            site: actual_map.clone(),
        });
    }

    let blocks = actual_flow
        .entries()
        .iter()
        .find(|entry| entry.key() == "blocks")
        .ok_or_else(|| MapReadFactIssueV1::BlocksEntryMissing {
            site: actual_map.clone(),
        })?;
    if blocks.array_elements().is_none() {
        return Err(MapReadFactIssueV1::BlocksEntryNotArray {
            site: actual_map.clone(),
        });
    }
    if blocks.store_class() != MapEntryStoreClassV1::BorrowedArray {
        return Err(MapReadFactIssueV1::BlocksEntryUnsupported {
            site: actual_map.clone(),
        });
    }
    let elements = blocks
        .array_elements()
        .expect("BorrowedArray has sealed elements");
    let element = elements
        .first()
        .ok_or_else(|| MapReadFactIssueV1::BlocksElementMissing {
            site: actual_map.clone(),
        })?;
    let Some(MapValueSource::MapLocal(binding)) = element.value_source() else {
        return Err(MapReadFactIssueV1::BlocksElementNotMapLocal {
            site: OwnedExprSiteV1::new(actual_map.owner(), element.site().clone()),
        });
    };
    let child = ledger
        .map_flow_for_local_binding(*binding)
        .ok()
        .flatten()
        .ok_or_else(|| MapReadFactIssueV1::BlocksChildMapUnavailable {
            site: OwnedExprSiteV1::new(actual_map.owner(), element.site().clone()),
        })?;
    let kind = child
        .entries()
        .iter()
        .find(|entry| entry.key() == "kind")
        .ok_or_else(|| MapReadFactIssueV1::KindEntryMissing {
            site: child.site().clone(),
        })?;
    if !matches!(kind.value_source(), Some(MapValueSource::String(_))) {
        return Err(MapReadFactIssueV1::KindValueNotText {
            site: child.site().clone(),
        });
    }

    let mut containment = vec![actual_map.clone()];
    containment.push(OwnedExprSiteV1::new(
        actual_map.owner(),
        blocks.site().clone(),
    ));
    containment.push(OwnedExprSiteV1::new(
        actual_map.owner(),
        element.site().clone(),
    ));
    if child.site() != actual_map {
        containment.push(child.site().clone());
    }
    let owner = first.owner();
    let borrow_roots = borrow_roots(actual_flow, blocks, child);
    let first_site = OwnedExprSiteV1::new(owner, first.site().clone());
    let second_site = OwnedExprSiteV1::new(owner, second.site().clone());
    let third_site = OwnedExprSiteV1::new(owner, third.site().clone());
    let first_receiver = OwnedExprSiteV1::new(owner, first.receiver_site().clone());
    let second_receiver = OwnedExprSiteV1::new(owner, second.receiver_site().clone());
    let third_receiver = OwnedExprSiteV1::new(owner, third.receiver_site().clone());
    let first_operand = OwnedExprSiteV1::new(owner, first.arguments()[0].site().clone());
    let second_operand = OwnedExprSiteV1::new(owner, second.arguments()[0].site().clone());
    let third_operand = OwnedExprSiteV1::new(owner, third.arguments()[0].site().clone());
    Ok(vec![
        MapReadFactV1 {
            owner,
            call_site: call_site.clone(),
            argument_ordinal,
            receiver_binding: formal_binding,
            site: first_site,
            receiver_site: first_receiver,
            operand_site: first_operand,
            operation: MapReadOperationV1::MapLookup,
            operand: MapReadOperandV1::Key("blocks".into()),
            result: MapReadResultClassV1::ArrayView,
            containment: containment.clone().into_boxed_slice(),
            borrow_roots: borrow_roots.clone().into_boxed_slice(),
        },
        MapReadFactV1 {
            owner,
            call_site: call_site.clone(),
            argument_ordinal,
            receiver_binding: formal_binding,
            site: second_site,
            receiver_site: second_receiver,
            operand_site: second_operand,
            operation: MapReadOperationV1::ArrayIndex,
            operand: MapReadOperandV1::Index(0),
            result: MapReadResultClassV1::MapView,
            containment: containment.clone().into_boxed_slice(),
            borrow_roots: borrow_roots.clone().into_boxed_slice(),
        },
        MapReadFactV1 {
            owner,
            call_site,
            argument_ordinal,
            receiver_binding: formal_binding,
            site: third_site,
            receiver_site: third_receiver,
            operand_site: third_operand,
            operation: MapReadOperationV1::MapLookup,
            operand: MapReadOperandV1::Key("kind".into()),
            result: MapReadResultClassV1::TextView,
            containment: containment.into_boxed_slice(),
            borrow_roots: borrow_roots.into_boxed_slice(),
        },
    ])
}

pub(super) fn borrow_roots(
    actual: &MapHomeFlow,
    entry: &MapHomeEntry,
    child: &MapHomeFlow,
) -> Vec<BindingRefV1> {
    let mut roots = BTreeSet::new();
    roots.extend(actual.allocation_fault());
    roots.extend(child.allocation_fault());
    if let Some(elements) = entry.array_elements() {
        collect_element_roots(elements, &mut roots);
    }
    roots.into_iter().collect()
}

pub(super) fn borrow_roots_for_entry(
    actual: &MapHomeFlow,
    entry: &MapHomeEntry,
) -> Vec<BindingRefV1> {
    let mut roots = BTreeSet::new();
    roots.extend(actual.allocation_fault());
    if let Some(elements) = entry.array_elements() {
        collect_element_roots(elements, &mut roots);
    }
    roots.into_iter().collect()
}

fn collect_element_roots(elements: &[ArrayElementSource], roots: &mut BTreeSet<BindingRefV1>) {
    for element in elements {
        if let Some(source) = element.value_source() {
            if let Some((_, root)) = source.borrowed_root() {
                roots.insert(root);
            }
        }
        if let Some(nested) = element.nested_elements() {
            collect_element_roots(nested, roots);
        }
    }
}
