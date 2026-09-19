//! Resolver-sealed source Facts for the first nested Map read chain.
//!
//! This module joins only products that already carry source authority: the
//! exact direct-call row, the callee's resolver method-call rows, and the
//! caller's completed MapHomeFlow.  It emits no Recipe key, MIR type, pointer,
//! or physical operation.  The physical read owner remains a later boundary.

use super::direct_call_loan::DirectCallDispositionLoansV1;
use super::model::OwnedCallableParameterContractDeclarationV1;
use super::ordinary_new_coseal::OrdinaryNewClaimLedgerV1;
use crate::mir::callable_parameter_contract::CallableParameterContractKindV1;
use crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1;
use crate::mir::resolved_semantics::home_new_prefix::{
    MapEntryStoreClassV1, MapHomeFlow, MapValueSource, TerminalCallArgumentV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, ResolvedLexicalRefV1,
    ResolvedLiteralSourceV1, ResolvedMethodCallReceiverSourceV1, SourceExprSiteV1,
};
#[path = "map_read_blocks_chain.rs"]
mod blocks_chain;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MapReadOperationV1 {
    MapLookup,
    ArrayIndex,
    ArrayLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MapReadResultClassV1 {
    I64,
    ArrayView,
    MapView,
    TextView,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MapReadOperandV1 {
    Key(Box<str>),
    Index(u32),
}

/// One source read tied to one caller edge.  The containment rows are the
/// exact source-owned map/array path used to prove the read; no downstream
/// stage may recover it from a name or a runtime handle.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MapReadFactV1 {
    owner: FunctionOwnerIdV1,
    call_site: OwnedExprSiteV1,
    argument_ordinal: u32,
    receiver_binding: BindingRefV1,
    site: OwnedExprSiteV1,
    receiver_site: OwnedExprSiteV1,
    operand_site: OwnedExprSiteV1,
    operation: MapReadOperationV1,
    operand: MapReadOperandV1,
    result: MapReadResultClassV1,
    containment: Box<[OwnedExprSiteV1]>,
    borrow_roots: Box<[BindingRefV1]>,
}

impl MapReadFactV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn call_site(&self) -> &OwnedExprSiteV1 {
        &self.call_site
    }

    pub(crate) fn argument_ordinal(&self) -> u32 {
        self.argument_ordinal
    }

    pub(crate) fn receiver_binding(&self) -> BindingRefV1 {
        self.receiver_binding
    }

    pub(crate) fn site(&self) -> &OwnedExprSiteV1 {
        &self.site
    }

    pub(crate) fn receiver_site(&self) -> &OwnedExprSiteV1 {
        &self.receiver_site
    }

    pub(crate) fn operand_site(&self) -> &OwnedExprSiteV1 {
        &self.operand_site
    }

    pub(crate) fn operation(&self) -> MapReadOperationV1 {
        self.operation
    }

    pub(crate) fn operand(&self) -> &MapReadOperandV1 {
        &self.operand
    }

    pub(crate) fn result(&self) -> MapReadResultClassV1 {
        self.result
    }

    pub(crate) fn containment(&self) -> &[OwnedExprSiteV1] {
        &self.containment
    }

    pub(crate) fn borrow_roots(&self) -> &[BindingRefV1] {
        &self.borrow_roots
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct MapReadFactsV1 {
    rows: Box<[MapReadFactV1]>,
}

impl MapReadFactsV1 {
    pub(crate) fn rows(&self) -> &[MapReadFactV1] {
        &self.rows
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    pub(crate) fn first_site(&self) -> Option<&OwnedExprSiteV1> {
        self.rows.first().map(MapReadFactV1::site)
    }

    pub(crate) fn has_edge_read(
        &self,
        owner: FunctionOwnerIdV1,
        call_site: &OwnedExprSiteV1,
        ordinal: u32,
        binding: BindingRefV1,
    ) -> bool {
        self.rows.iter().any(|row| {
            row.owner == owner
                && row.call_site == *call_site
                && row.argument_ordinal == ordinal
                && row.receiver_binding == binding
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum MapReadFactIssueV1 {
    CallerTerminalMissing {
        owner: FunctionOwnerIdV1,
    },
    CallerArgumentMissing {
        call: OwnedExprSiteV1,
        ordinal: u32,
    },
    ActualMapUnavailable {
        site: OwnedExprSiteV1,
    },
    FormalMissing {
        owner: FunctionOwnerIdV1,
        ordinal: u32,
    },
    FormalKindMismatch {
        owner: FunctionOwnerIdV1,
        ordinal: u32,
    },
    FirstLookupMissing {
        owner: FunctionOwnerIdV1,
    },
    FirstLookupDuplicate {
        owner: FunctionOwnerIdV1,
    },
    FirstLookupOperandMismatch {
        owner: FunctionOwnerIdV1,
    },
    ArrayIndexMissing {
        owner: FunctionOwnerIdV1,
    },
    ArrayIndexDuplicate {
        owner: FunctionOwnerIdV1,
    },
    ArrayIndexOperandMismatch {
        owner: FunctionOwnerIdV1,
    },
    ArrayLengthMissing {
        owner: FunctionOwnerIdV1,
    },
    ArrayLengthDuplicate {
        owner: FunctionOwnerIdV1,
    },
    ArrayLengthOperandMismatch {
        owner: FunctionOwnerIdV1,
    },
    TextLookupMissing {
        owner: FunctionOwnerIdV1,
    },
    TextLookupDuplicate {
        owner: FunctionOwnerIdV1,
    },
    TextLookupOperandMismatch {
        owner: FunctionOwnerIdV1,
    },
    FunctionsEntryMissing {
        site: OwnedExprSiteV1,
    },
    FunctionsEntryNotArray {
        site: OwnedExprSiteV1,
    },
    ParamsEntryMissing {
        site: OwnedExprSiteV1,
    },
    ParamsEntryNotArray {
        site: OwnedExprSiteV1,
    },
    ParamsEntryUnsupported {
        site: OwnedExprSiteV1,
    },
    BlocksEntryMissing {
        site: OwnedExprSiteV1,
    },
    BlocksEntryNotArray {
        site: OwnedExprSiteV1,
    },
    BlocksEntryUnsupported {
        site: OwnedExprSiteV1,
    },
    BlocksElementMissing {
        site: OwnedExprSiteV1,
    },
    BlocksElementNotMapLocal {
        site: OwnedExprSiteV1,
    },
    BlocksChildMapUnavailable {
        site: OwnedExprSiteV1,
    },
    KindEntryMissing {
        site: OwnedExprSiteV1,
    },
    KindEntryDuplicate {
        site: OwnedExprSiteV1,
    },
    KindEntryOperandMismatch {
        site: OwnedExprSiteV1,
    },
    KindValueNotText {
        site: OwnedExprSiteV1,
    },
    BlocksLengthMissing {
        site: OwnedExprSiteV1,
    },
    BlocksLengthDuplicate {
        site: OwnedExprSiteV1,
    },
    BlocksLengthOperandMismatch {
        site: OwnedExprSiteV1,
    },
    ArrayElementMissing {
        site: OwnedExprSiteV1,
    },
    ArrayElementNotMap {
        site: OwnedExprSiteV1,
    },
    ChildMapUnavailable {
        site: OwnedExprSiteV1,
    },
    NameEntryMissing {
        site: OwnedExprSiteV1,
    },
    NameValueNotText {
        site: OwnedExprSiteV1,
    },
    DuplicateReadSite {
        site: OwnedExprSiteV1,
    },
}

pub(super) fn issue_map_read_facts_v1(
    batch: &VerifiedResolvedCallableSemanticBatchV1,
    loans: Option<&DirectCallDispositionLoansV1>,
    parameters: &[OwnedCallableParameterContractDeclarationV1],
    ledger: &OrdinaryNewClaimLedgerV1,
) -> Result<MapReadFactsV1, MapReadFactIssueV1> {
    let mut rows = Vec::new();
    let Some(loans) = loans else {
        return Ok(MapReadFactsV1::default());
    };
    for loan in loans.iter() {
        for (call_site, ordinal, callee_owner) in loan.map_formal_edges() {
            let Some((caller, terminal)) = ledger.call_source_completion_for_owner(loan.owner())
            else {
                continue;
            };
            if terminal.call_site() != call_site.site() {
                continue;
            }
            let Some(argument) = terminal.arguments().get(ordinal as usize) else {
                return Err(MapReadFactIssueV1::CallerArgumentMissing {
                    call: call_site,
                    ordinal,
                });
            };
            let TerminalCallArgumentV1::Map(actual_map) = argument else {
                continue;
            };
            let actual_flow = ledger.map_flow(actual_map).map_err(|_| {
                MapReadFactIssueV1::ActualMapUnavailable {
                    site: actual_map.clone(),
                }
            })?;
            let formal = parameters
                .iter()
                .filter(|row| row.owner == callee_owner)
                .flat_map(|row| row.parameters.iter())
                .find(|row| row.ordinal == ordinal)
                .ok_or(MapReadFactIssueV1::FormalMissing {
                    owner: callee_owner,
                    ordinal,
                })?;
            if formal.kind != CallableParameterContractKindV1::Map {
                return Err(MapReadFactIssueV1::FormalKindMismatch {
                    owner: callee_owner,
                    ordinal,
                });
            }
            let declaration = batch
                .declarations()
                .find(|row| row.owner() == callee_owner)
                .ok_or(MapReadFactIssueV1::FormalMissing {
                    owner: callee_owner,
                    ordinal,
                })?;
            let has_first_candidate = batch
                .with_lowering_input(declaration.batch_slot(), |input| {
                    has_first_lookup_candidate(input.function(), formal.binding)
                })
                .unwrap_or(false);
            let has_params_length_candidate = batch
                .with_lowering_input(declaration.batch_slot(), |input| {
                    has_array_length_candidate(input.function(), formal.binding)
                })
                .unwrap_or(false);
            let has_blocks_kind_candidate = batch
                .with_lowering_input(declaration.batch_slot(), |input| {
                    blocks_chain::has_blocks_kind_candidate(input.function(), formal.binding)
                })
                .unwrap_or(false);
            let has_blocks_length_candidate = batch
                .with_lowering_input(declaration.batch_slot(), |input| {
                    blocks_chain::has_blocks_length_candidate(input.function(), formal.binding)
                })
                .unwrap_or(false);
            if !has_first_candidate
                && !has_params_length_candidate
                && !has_blocks_kind_candidate
                && !has_blocks_length_candidate
            {
                continue;
            }
            let issued = batch
                .with_lowering_input(declaration.batch_slot(), |input| {
                    let mut rows = Vec::new();
                    if has_first_candidate {
                        rows.extend(issue_chain(
                            input.function(),
                            formal.binding,
                            call_site.clone(),
                            ordinal,
                            actual_map,
                            actual_flow,
                            ledger,
                        )?);
                    }
                    if has_params_length_candidate {
                        rows.extend(issue_array_length_chain(
                            input.function(),
                            formal.binding,
                            call_site.clone(),
                            ordinal,
                            actual_map,
                            actual_flow,
                        )?);
                    }
                    if has_blocks_kind_candidate {
                        rows.extend(blocks_chain::issue_blocks_kind_chain(
                            input.function(),
                            formal.binding,
                            call_site.clone(),
                            ordinal,
                            actual_map,
                            actual_flow,
                            ledger,
                        )?);
                    }
                    if has_blocks_length_candidate {
                        rows.extend(blocks_chain::issue_blocks_length_chain(
                            input.function(),
                            formal.binding,
                            call_site,
                            ordinal,
                            actual_map,
                            actual_flow,
                        )?);
                    }
                    Ok(rows)
                })
                .map_err(|_| MapReadFactIssueV1::FormalMissing {
                    owner: callee_owner,
                    ordinal,
                })??;
            rows.extend(issued);
            let _ = caller;
        }
    }
    rows.sort_by(|left, right| left.site.cmp(&right.site));
    for pair in rows.windows(2) {
        if pair[0].site == pair[1].site {
            return Err(MapReadFactIssueV1::DuplicateReadSite {
                site: pair[0].site.clone(),
            });
        }
    }
    Ok(MapReadFactsV1 {
        rows: rows.into_boxed_slice(),
    })
}

fn has_first_lookup_candidate(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    formal_binding: BindingRefV1,
) -> bool {
    function.method_calls().any(|(_, call)| {
        call.receiver()
            == ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                formal_binding,
            ))
            && call.selector() == "get"
            && call.arity() == 1
            && literal_string(function, call.arguments()[0].site()) == Some("functions")
    })
}

fn has_array_length_candidate(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    formal_binding: BindingRefV1,
) -> bool {
    function.method_calls().any(|(_, call)| {
        call.receiver()
            == ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(
                formal_binding,
            ))
            && call.selector() == "get"
            && call.arity() == 1
            && literal_string(function, call.arguments()[0].site()) == Some("params")
    }) && function
        .method_calls()
        .any(|(_, call)| call.selector() == "length" && call.arity() == 0)
}

fn issue_array_length_chain(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    formal_binding: BindingRefV1,
    call_site: OwnedExprSiteV1,
    argument_ordinal: u32,
    actual_map: &OwnedExprSiteV1,
    actual_flow: &MapHomeFlow,
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
                && literal_string(function, call.arguments()[0].site()) == Some("params")
        },
        MapReadFactIssueV1::FirstLookupMissing {
            owner: actual_map.owner(),
        },
        MapReadFactIssueV1::FirstLookupDuplicate {
            owner: actual_map.owner(),
        },
    )?;
    let second = unique_call(
        function,
        |call| {
            call.receiver_site() == first.site() && call.selector() == "length" && call.arity() == 0
        },
        MapReadFactIssueV1::ArrayLengthMissing {
            owner: actual_map.owner(),
        },
        MapReadFactIssueV1::ArrayLengthDuplicate {
            owner: actual_map.owner(),
        },
    )?;
    if !second.arguments().is_empty() {
        return Err(MapReadFactIssueV1::ArrayLengthOperandMismatch {
            owner: actual_map.owner(),
        });
    }
    let params = actual_flow
        .entries()
        .iter()
        .find(|entry| entry.key() == "params")
        .ok_or_else(|| MapReadFactIssueV1::ParamsEntryMissing {
            site: actual_map.clone(),
        })?;
    if params.array_elements().is_none() {
        return Err(MapReadFactIssueV1::ParamsEntryNotArray {
            site: actual_map.clone(),
        });
    }
    if !matches!(
        params.store_class(),
        MapEntryStoreClassV1::EmptyArray | MapEntryStoreClassV1::BorrowedArray
    ) {
        return Err(MapReadFactIssueV1::ParamsEntryUnsupported {
            site: actual_map.clone(),
        });
    }
    let owner = first.owner();
    let mut containment = vec![actual_map.clone()];
    containment.push(OwnedExprSiteV1::new(
        actual_map.owner(),
        params.site().clone(),
    ));
    let first_site = OwnedExprSiteV1::new(owner, first.site().clone());
    let second_site = OwnedExprSiteV1::new(owner, second.site().clone());
    let first_receiver = OwnedExprSiteV1::new(owner, first.receiver_site().clone());
    let second_receiver = OwnedExprSiteV1::new(owner, second.receiver_site().clone());
    let first_operand = OwnedExprSiteV1::new(owner, first.arguments()[0].site().clone());
    let borrow_roots = blocks_chain::borrow_roots_for_entry(actual_flow, params);
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
            operand: MapReadOperandV1::Key("params".into()),
            result: MapReadResultClassV1::ArrayView,
            containment: containment.clone().into_boxed_slice(),
            borrow_roots: borrow_roots.clone().into_boxed_slice(),
        },
        MapReadFactV1 {
            owner,
            call_site,
            argument_ordinal,
            receiver_binding: formal_binding,
            site: second_site,
            receiver_site: second_receiver.clone(),
            operand_site: second_receiver,
            operation: MapReadOperationV1::ArrayLength,
            operand: MapReadOperandV1::Key("params".into()),
            result: MapReadResultClassV1::I64,
            containment: containment.into_boxed_slice(),
            borrow_roots: borrow_roots.into_boxed_slice(),
        },
    ])
}

fn issue_chain(
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
                && literal_string(function, call.arguments()[0].site()) == Some("functions")
        },
        MapReadFactIssueV1::FirstLookupMissing {
            owner: actual_map.owner(),
        },
        MapReadFactIssueV1::FirstLookupDuplicate {
            owner: actual_map.owner(),
        },
    )?;
    if first.arguments().len() != 1
        || literal_string(function, first.arguments()[0].site()) != Some("functions")
    {
        return Err(MapReadFactIssueV1::FirstLookupOperandMismatch {
            owner: actual_map.owner(),
        });
    }
    let second = unique_call(
        function,
        |call| {
            call.receiver_site() == first.site()
                && call.selector() == "get"
                && call.arity() == 1
                && literal_integer(function, call.arguments()[0].site()) == Some(0)
        },
        MapReadFactIssueV1::ArrayIndexMissing {
            owner: actual_map.owner(),
        },
        MapReadFactIssueV1::ArrayIndexDuplicate {
            owner: actual_map.owner(),
        },
    )?;
    if second.arguments().len() != 1
        || literal_integer(function, second.arguments()[0].site()) != Some(0)
    {
        return Err(MapReadFactIssueV1::ArrayIndexOperandMismatch {
            owner: actual_map.owner(),
        });
    }
    let third = unique_call(
        function,
        |call| {
            call.receiver_site() == second.site()
                && call.selector() == "get"
                && call.arity() == 1
                && literal_string(function, call.arguments()[0].site()) == Some("name")
        },
        MapReadFactIssueV1::TextLookupMissing {
            owner: actual_map.owner(),
        },
        MapReadFactIssueV1::TextLookupDuplicate {
            owner: actual_map.owner(),
        },
    )?;
    if third.arguments().len() != 1
        || literal_string(function, third.arguments()[0].site()) != Some("name")
    {
        return Err(MapReadFactIssueV1::TextLookupOperandMismatch {
            owner: actual_map.owner(),
        });
    }

    let functions = actual_flow
        .entries()
        .iter()
        .find(|entry| entry.key() == "functions")
        .ok_or_else(|| MapReadFactIssueV1::FunctionsEntryMissing {
            site: actual_map.clone(),
        })?;
    let elements =
        functions
            .array_elements()
            .ok_or_else(|| MapReadFactIssueV1::FunctionsEntryNotArray {
                site: actual_map.clone(),
            })?;
    let element = elements
        .first()
        .ok_or_else(|| MapReadFactIssueV1::ArrayElementMissing {
            site: actual_map.clone(),
        })?;
    let child = blocks_chain::child_map_for_element(ledger, element).ok_or_else(|| {
        MapReadFactIssueV1::ArrayElementNotMap {
            site: OwnedExprSiteV1::new(actual_map.owner(), element.site().clone()),
        }
    })?;
    let name = child
        .entries()
        .iter()
        .find(|entry| entry.key() == "name")
        .ok_or_else(|| MapReadFactIssueV1::NameEntryMissing {
            site: child.site().clone(),
        })?;
    if !matches!(name.value_source(), Some(MapValueSource::String(_))) {
        return Err(MapReadFactIssueV1::NameValueNotText {
            site: child.site().clone(),
        });
    }

    let mut containment = vec![actual_map.clone()];
    containment.push(OwnedExprSiteV1::new(
        actual_map.owner(),
        functions.site().clone(),
    ));
    containment.push(OwnedExprSiteV1::new(
        actual_map.owner(),
        element.site().clone(),
    ));
    if child.site() != actual_map {
        containment.push(child.site().clone());
    }
    let owner = first.owner();
    let borrow_roots = blocks_chain::borrow_roots(actual_flow, functions, child);
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
            operand: MapReadOperandV1::Key("functions".into()),
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
            operand: MapReadOperandV1::Key("name".into()),
            result: MapReadResultClassV1::TextView,
            containment: containment.into_boxed_slice(),
            borrow_roots: borrow_roots.into_boxed_slice(),
        },
    ])
}

fn unique_call<'a>(
    function: &'a crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    predicate: impl Fn(&crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1) -> bool,
    missing: MapReadFactIssueV1,
    duplicate: MapReadFactIssueV1,
) -> Result<
    &'a crate::mir::resolved_semantics::VerifiedResolvedMethodCallSourceV1,
    MapReadFactIssueV1,
> {
    let mut rows = function
        .method_calls()
        .filter_map(|(_, call)| predicate(call).then_some(call));
    let Some(first) = rows.next() else {
        return Err(missing);
    };
    if rows.next().is_some() {
        return Err(duplicate);
    }
    Ok(first)
}

fn literal_string<'a>(
    function: &'a crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Option<&'a str> {
    match function.expression_source().literal(site) {
        Some(ResolvedLiteralSourceV1::String(value)) => Some(value),
        _ => None,
    }
}

fn literal_integer(
    function: &crate::mir::resolved_semantics::VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Option<i64> {
    match function.expression_source().literal(site) {
        Some(ResolvedLiteralSourceV1::Integer(value)) => Some(*value),
        Some(ResolvedLiteralSourceV1::TypedInteger { value, .. }) => Some(*value),
        _ => None,
    }
}
