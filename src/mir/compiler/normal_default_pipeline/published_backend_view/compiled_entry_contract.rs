//! One physical compiled-entry contract issued by an activated lifecycle view.
//!
//! It joins already-retained source/handoff facts to final-MIR parameter values.
//! It neither classifies source parameters nor makes C ABI choices.

use crate::mir::instruction::InvokeCallResultKind;
use crate::mir::instruction::InvokeOperation;
use crate::mir::normal_callable_semantic_package::{
    BirthFormalContractV1, BirthFormalPhysicalDispositionV1, FinalizedBirthActualsV1,
    FinalizedRootResultAbiV1,
};
use crate::mir::{Callee, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[path = "compiled_entry_contract/birth_calls.rs"]
mod birth_calls;

use super::{
    physical_program::{
        PublishedLifecyclePhysicalFunctionRoleV1, PublishedLifecyclePhysicalProgramV1,
    },
    PublishedMirBackendView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompiledEntryFormalKindV1 {
    Receiver,
    Parameter,
}

/// Backend-facing result category. Source terminal provenance stops here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompiledEntryRootResultV1 {
    I64,
    Unit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompiledEntryFormalV1 {
    source_ordinal: Option<u32>,
    physical_ordinal: u32,
    value: ValueId,
    kind: CompiledEntryFormalKindV1,
    contract: Option<BirthFormalContractV1>,
}

impl CompiledEntryFormalV1 {
    pub(crate) const fn source_ordinal(&self) -> Option<u32> {
        self.source_ordinal
    }
    pub(crate) const fn physical_ordinal(&self) -> u32 {
        self.physical_ordinal
    }
    pub(crate) const fn value(&self) -> ValueId {
        self.value
    }
    pub(crate) const fn kind(&self) -> CompiledEntryFormalKindV1 {
        self.kind
    }
    pub(crate) fn contract(&self) -> Option<&BirthFormalContractV1> {
        self.contract.as_ref()
    }
    pub(crate) fn disposition(&self) -> Option<BirthFormalPhysicalDispositionV1> {
        self.contract
            .as_ref()
            .map(BirthFormalContractV1::disposition)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledEntryBirthV1 {
    function_index: u32,
    formals: Box<[CompiledEntryFormalV1]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CompiledEntryBirthCallV1 {
    caller_function_index: u32,
    function_index: u32,
    actual: FinalizedBirthActualsV1,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CompiledEntryOrdinaryCallV1 {
    function_index: u32,
    call: crate::mir::definitions::MirCall,
}

impl CompiledEntryOrdinaryCallV1 {
    pub(crate) const fn function_index(&self) -> u32 {
        self.function_index
    }
    pub(crate) fn call(&self) -> &crate::mir::definitions::MirCall {
        &self.call
    }
}

impl CompiledEntryBirthCallV1 {
    pub(crate) const fn caller_function_index(&self) -> u32 {
        self.caller_function_index
    }
    pub(crate) const fn function_index(&self) -> u32 {
        self.function_index
    }
    pub(crate) fn receiver(&self) -> ValueId {
        self.actual.receiver()
    }
    pub(crate) fn actual(&self) -> &FinalizedBirthActualsV1 {
        &self.actual
    }
    pub(crate) fn arguments(&self) -> impl ExactSizeIterator<Item = ValueId> + '_ {
        self.actual
            .arguments()
            .iter()
            .map(|argument| argument.value())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompiledEntryCleanupKindV1 {
    HomeRelease,
    ReclaimUnpublished,
    FaultFrameEnter,
    ReturnFault,
    ArrayResidenceRelease,
    MapEnd,
    MapEndOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CompiledEntryCleanupCoordinateV1 {
    function_index: u32,
    block_id: u32,
    instruction_index: u32,
    kind: CompiledEntryCleanupKindV1,
}

impl CompiledEntryCleanupCoordinateV1 {
    pub(crate) const fn function_index(self) -> u32 {
        self.function_index
    }
    pub(crate) const fn block_id(self) -> u32 {
        self.block_id
    }
    pub(crate) const fn instruction_index(self) -> u32 {
        self.instruction_index
    }
    pub(crate) const fn kind(self) -> CompiledEntryCleanupKindV1 {
        self.kind
    }
}

impl CompiledEntryBirthV1 {
    pub(crate) const fn function_index(&self) -> u32 {
        self.function_index
    }
    pub(crate) fn formals(&self) -> &[CompiledEntryFormalV1] {
        &self.formals
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledEntryArrayClaimV1<'module> {
    pub(crate) array: ValueId,
    pub(crate) contract_id: &'module str,
    pub(crate) spec: crate::typed_array_contract_spec::ArrayElementContractSpec,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CompiledEntryArrayValueKindV1 {
    I64,
    Bool,
    F64,
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledEntryArrayWriteV1 {
    pub(crate) site: crate::mir::ArrayWriteSiteId,
    pub(crate) array: ValueId,
    pub(crate) value: ValueId,
    pub(crate) kind: CompiledEntryArrayValueKindV1,
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledEntryContractV1<'module> {
    program: PublishedLifecyclePhysicalProgramV1<'module>,
    root_result: CompiledEntryRootResultV1,
    births: Box<[CompiledEntryBirthV1]>,
    birth_calls: Box<[CompiledEntryBirthCallV1]>,
    ordinary_calls: Box<[CompiledEntryOrdinaryCallV1]>,
    cleanup: Box<[CompiledEntryCleanupCoordinateV1]>,
    array_claims: Box<[CompiledEntryArrayClaimV1<'module>]>,
    array_writes: Box<[CompiledEntryArrayWriteV1]>,
}

impl<'module> CompiledEntryContractV1<'module> {
    pub(crate) fn array_claims(&self) -> &[CompiledEntryArrayClaimV1<'module>] {
        &self.array_claims
    }
    pub(crate) fn array_writes(&self) -> &[CompiledEntryArrayWriteV1] {
        &self.array_writes
    }

    pub(crate) fn program(&self) -> &PublishedLifecyclePhysicalProgramV1<'module> {
        &self.program
    }
    pub(crate) const fn root_result(&self) -> CompiledEntryRootResultV1 {
        self.root_result
    }
    pub(crate) fn births(&self) -> &[CompiledEntryBirthV1] {
        &self.births
    }
    pub(crate) fn birth_calls(&self) -> &[CompiledEntryBirthCallV1] {
        &self.birth_calls
    }
    pub(crate) fn ordinary_calls(&self) -> &[CompiledEntryOrdinaryCallV1] {
        &self.ordinary_calls
    }
    pub(crate) fn cleanup(&self) -> &[CompiledEntryCleanupCoordinateV1] {
        &self.cleanup
    }
}

impl<'module> PublishedMirBackendView<'module> {
    pub(crate) fn issue_lifecycle_compiled_entry_contract(
        &self,
    ) -> Result<CompiledEntryContractV1<'module>, String> {
        let program = self.issue_lifecycle_physical_program()?;
        let (root_result, ordinary_calls, contract_births, birth_calls, cleanup) = {
            let [root, tail @ ..] = program.functions() else {
                return Err(fault("compiled-entry-root-missing"));
            };
            let PublishedLifecyclePhysicalFunctionRoleV1::Root { result } = root.role() else {
                return Err(fault("compiled-entry-root-role"));
            };
            let root_ordinary_calls = root
                .blocks()
                .iter()
                .flat_map(|block| {
                    block
                        .instructions()
                        .iter()
                        .copied()
                        .chain(std::iter::once(block.terminator()))
                })
                .filter_map(|row| {
                    let MirInstruction::Invoke {
                        operation:
                            InvokeOperation::Call {
                                call,
                                result: InvokeCallResultKind::I64,
                            },
                        ..
                    } = row.instruction()
                    else {
                        return None;
                    };
                    Some(call.clone())
                })
                .collect::<Vec<_>>();
            let mut contract_births = Vec::with_capacity(tail.len());
            let mut birth_functions = Vec::with_capacity(tail.len());
            let mut ordinary_function_indices = BTreeMap::new();
            for (index, function) in tail.iter().enumerate() {
                let physical_index =
                    u32::try_from(index + 1).map_err(|_| fault("compiled-entry-index"))?;
                match function.role() {
                    PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } => {
                        if function.params().len() != abi.abi().physical_arity()
                            || abi.formal_contracts().len() != abi.parameters().len()
                        {
                            return Err(fault("compiled-entry-birth-arity"));
                        }
                        let receiver = abi.receiver();
                        if receiver.source_ordinal().is_some() || receiver.physical_lane() != 0 {
                            return Err(fault("compiled-entry-receiver-lane"));
                        }
                        let mut formals = Vec::with_capacity(function.params().len());
                        formals.push(CompiledEntryFormalV1 {
                            source_ordinal: None,
                            physical_ordinal: receiver.physical_lane(),
                            value: function.params()[0],
                            kind: CompiledEntryFormalKindV1::Receiver,
                            contract: None,
                        });
                        for (ordinal, (lane, contract)) in abi
                            .parameters()
                            .iter()
                            .zip(abi.formal_contracts())
                            .enumerate()
                        {
                            let ordinal = u32::try_from(ordinal)
                                .map_err(|_| fault("compiled-entry-ordinal"))?;
                            if lane.source_ordinal() != Some(ordinal)
                                || lane.physical_lane() != ordinal + 1
                                || contract.ordinal() != ordinal
                                || contract.binding() != lane.binding()
                            {
                                return Err(fault("compiled-entry-formal-drift"));
                            }
                            formals.push(CompiledEntryFormalV1 {
                                source_ordinal: Some(ordinal),
                                physical_ordinal: lane.physical_lane(),
                                value: function.params()[lane.physical_lane() as usize],
                                kind: CompiledEntryFormalKindV1::Parameter,
                                contract: Some(contract.clone()),
                            });
                        }
                        contract_births.push(CompiledEntryBirthV1 {
                            function_index: physical_index,
                            formals: formals.into_boxed_slice(),
                        });
                        birth_functions.push((physical_index, function));
                    }
                    PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { .. } => {
                        let PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { key } =
                            function.role()
                        else {
                            unreachable!()
                        };
                        if ordinary_function_indices
                            .insert(key.clone(), physical_index)
                            .is_some()
                        {
                            return Err(fault("compiled-entry-ordinary-duplicate"));
                        }
                    }
                    PublishedLifecyclePhysicalFunctionRoleV1::Root { .. } => {
                        return Err(fault("compiled-entry-nested-root"));
                    }
                }
            }
            let mut referenced_ordinary_keys = BTreeSet::new();
            let mut ordinary_calls = Vec::with_capacity(root_ordinary_calls.len());
            for call in root_ordinary_calls {
                let key = super::physical_program::ordinary_callable_key(&call.callee)?;
                let function_index = *ordinary_function_indices
                    .get(&key)
                    .ok_or_else(|| fault("compiled-entry-ordinary-missing"))?;
                let function = program
                    .functions()
                    .get(function_index as usize)
                    .ok_or_else(|| fault("compiled-entry-ordinary-index"))?;
                let receiver = super::physical_program::ordinary_call_receiver(&call.callee)?;
                let expected_arity = call.args.len() + usize::from(receiver.is_some());
                if function.params().len() != expected_arity
                    || !matches!(
                        function.role(),
                        PublishedLifecyclePhysicalFunctionRoleV1::OrdinaryI64 { key: target }
                            if target == &key
                    )
                {
                    return Err(fault("compiled-entry-ordinary-arity"));
                }
                referenced_ordinary_keys.insert(key);
                ordinary_calls.push(CompiledEntryOrdinaryCallV1 {
                    function_index,
                    call,
                });
            }
            if referenced_ordinary_keys.len() != ordinary_function_indices.len() {
                return Err(fault("compiled-entry-ordinary-unissued"));
            }
            let birth_calls = if program.is_native_array() {
                Vec::new()
            } else {
                let actuals = program
                    .handoff()
                    .birth_actuals()
                    .ok_or_else(|| fault("compiled-entry-actual-source-missing"))?;
                issue_birth_calls_for_program(&program, &birth_functions, actuals)?
            };
            let cleanup = issue_cleanup_coordinates(program.functions())?;
            (
                *result,
                ordinary_calls,
                contract_births,
                birth_calls,
                cleanup,
            )
        };
        let mut array_claims = Vec::new();
        let mut array_writes = Vec::new();
        if let Some(script) = program.handoff().script_array() {
            for (array, contract_id, spec) in script.claims() {
                array_claims.push(CompiledEntryArrayClaimV1 {
                    array,
                    contract_id,
                    spec,
                });
            }
            for (site, array, value, definition) in script.writes() {
                use crate::mir::ConstValue;
                let MirInstruction::Const {
                    dst,
                    value: constant,
                } = definition
                else {
                    return Err(fault("compiled-entry-array-definition"));
                };
                if *dst != value {
                    return Err(fault("compiled-entry-array-value"));
                }
                let kind = match constant {
                    ConstValue::Integer(_) => CompiledEntryArrayValueKindV1::I64,
                    ConstValue::Bool(_) => CompiledEntryArrayValueKindV1::Bool,
                    ConstValue::Float(_) => CompiledEntryArrayValueKindV1::F64,
                    _ => return Err(fault("compiled-entry-array-representation")),
                };
                array_writes.push(CompiledEntryArrayWriteV1 {
                    site,
                    array,
                    value,
                    kind,
                });
            }
        }
        Ok(CompiledEntryContractV1 {
            program,
            root_result,
            ordinary_calls: ordinary_calls.into_boxed_slice(),
            births: contract_births.into_boxed_slice(),
            birth_calls: birth_calls.into_boxed_slice(),
            cleanup: cleanup.into_boxed_slice(),
            array_claims: array_claims.into_boxed_slice(),
            array_writes: array_writes.into_boxed_slice(),
        })
    }
}

fn issue_birth_calls_for_owner(
    root: &super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    births: &[(
        u32,
        &super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    )],
    actuals: &[FinalizedBirthActualsV1],
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> Result<Vec<CompiledEntryBirthCallV1>, String> {
    issue_birth_calls_for_owner_at(0, root, births, actuals, owner)
}

fn issue_birth_calls_for_owner_at(
    caller_function_index: u32,
    root: &super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    births: &[(
        u32,
        &super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    )],
    actuals: &[FinalizedBirthActualsV1],
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> Result<Vec<CompiledEntryBirthCallV1>, String> {
    for (i, actual) in actuals.iter().enumerate() {
        if actuals[..i].iter().any(|previous| {
            previous.site() == actual.site() || previous.destination() == actual.destination()
        }) || actual.site().owner() != owner
            || actual.destination().owner() != owner
            || actual
                .arguments()
                .iter()
                .enumerate()
                .any(|(ordinal, argument)| {
                    argument.source().ordinal() as usize != ordinal
                        || argument.source().owner() != actual.destination().owner()
                        || argument.source().new_site() != actual.site()
                })
        {
            return Err(fault("compiled-entry-actual-membership"));
        }
    }
    let mut consumed = vec![false; actuals.len()];
    let mut referenced = vec![false; births.len()];
    let mut calls = Vec::new();
    for block in root.blocks() {
        for row in block
            .instructions()
            .iter()
            .copied()
            .chain(std::iter::once(block.terminator()))
        {
            let MirInstruction::Invoke {
                operation:
                    InvokeOperation::Call {
                        call,
                        result: InvokeCallResultKind::Unit,
                    },
                ..
            } = row.instruction()
            else {
                continue;
            };
            let Callee::BirthConstructor { key, receiver } = &call.callee else {
                continue;
            };
            let index = births
                .iter()
                .position(|(_, birth)| {
                    matches!(birth.role(),
                PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } if abi.target() == key)
                })
                .ok_or_else(|| fault("compiled-entry-call-target"))?;
            let matching = actuals
                .iter()
                .enumerate()
                .filter(|(_, actual)| {
                    actual.target() == key
                        && actual.receiver() == *receiver
                        && actual
                            .arguments()
                            .iter()
                            .map(|argument| argument.value())
                            .eq(call.args.iter().copied())
                })
                .map(|(i, _)| i)
                .collect::<Vec<_>>();
            let [actual_index] = matching.as_slice() else {
                return Err(fault("compiled-entry-call-actual-mismatch"));
            };
            if std::mem::replace(&mut consumed[*actual_index], true) {
                return Err(fault("compiled-entry-call-duplicate"));
            }
            if call.args.len() != births[index].1.params().len().saturating_sub(1) {
                return Err(fault("compiled-entry-call-arity"));
            }
            referenced[index] = true;
            calls.push(CompiledEntryBirthCallV1 {
                caller_function_index,
                function_index: births[index].0,
                actual: actuals[*actual_index].clone(),
            });
        }
    }
    if consumed.contains(&false) || referenced.contains(&false) {
        return Err(fault("compiled-entry-call-missing"));
    }
    Ok(calls)
}

fn issue_birth_calls_for_program(
    program: &super::physical_program::PublishedLifecyclePhysicalProgramV1<'_>,
    births: &[(
        u32,
        &super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    )],
    actuals: &[FinalizedBirthActualsV1],
) -> Result<Vec<CompiledEntryBirthCallV1>, String> {
    birth_calls::issue_birth_calls_for_program(program, births, actuals)
}

fn issue_birth_calls(
    root: &super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>,
    births: &[super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>],
    actuals: &[FinalizedBirthActualsV1],
    result: FinalizedRootResultAbiV1,
) -> Result<Vec<CompiledEntryBirthCallV1>, String> {
    let owner = match result {
        FinalizedRootResultAbiV1::CallReturn { owner }
        | FinalizedRootResultAbiV1::I64AddReturn { owner }
        | FinalizedRootResultAbiV1::UnitReturn { owner }
        | FinalizedRootResultAbiV1::IntegerLiteralReturn { owner }
        | FinalizedRootResultAbiV1::I64FieldReturn { owner } => owner,
    };
    let indexed = births
        .iter()
        .enumerate()
        .map(|(index, birth)| {
            Ok((
                u32::try_from(index + 1).map_err(|_| fault("compiled-entry-call-index"))?,
                birth,
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    issue_birth_calls_for_owner(root, &indexed, actuals, owner)
}

fn issue_cleanup_coordinates(
    functions: &[super::physical_program::PublishedLifecyclePhysicalFunctionV1<'_>],
) -> Result<Vec<CompiledEntryCleanupCoordinateV1>, String> {
    let mut rows = Vec::new();
    for (function_index, function) in functions.iter().enumerate() {
        for block in function.blocks() {
            for row in block
                .instructions()
                .iter()
                .copied()
                .chain(std::iter::once(block.terminator()))
            {
                let kind = match row.instruction() {
                    MirInstruction::Invoke {
                        operation: InvokeOperation::HomeRelease { .. },
                        ..
                    } => CompiledEntryCleanupKindV1::HomeRelease,
                    MirInstruction::Invoke {
                        operation: InvokeOperation::ReclaimUnpublished { .. },
                        ..
                    } => CompiledEntryCleanupKindV1::ReclaimUnpublished,
                    MirInstruction::Invoke {
                        operation:
                            InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::End {
                                ..
                            }),
                        ..
                    } => CompiledEntryCleanupKindV1::MapEnd,
                    MirInstruction::Invoke {
                        operation:
                            InvokeOperation::Map(
                                crate::mir::instruction::MapInvokeOperation::EndOutcome { .. },
                            ),
                        ..
                    } => CompiledEntryCleanupKindV1::MapEndOutcome,
                    MirInstruction::FaultFrameEnter { .. } => {
                        CompiledEntryCleanupKindV1::FaultFrameEnter
                    }
                    MirInstruction::ReturnFault { .. } => CompiledEntryCleanupKindV1::ReturnFault,
                    MirInstruction::ArrayResidenceRelease { .. } => {
                        CompiledEntryCleanupKindV1::ArrayResidenceRelease
                    }
                    _ => continue,
                };
                rows.push(CompiledEntryCleanupCoordinateV1 {
                    function_index: u32::try_from(function_index)
                        .map_err(|_| fault("compiled-entry-cleanup-index"))?,
                    block_id: block.id().0,
                    instruction_index: row.index(),
                    kind,
                });
            }
        }
    }
    if rows.is_empty() {
        return Err(fault("compiled-entry-cleanup-missing"));
    }
    Ok(rows)
}

pub(super) fn root_result_category(result: FinalizedRootResultAbiV1) -> CompiledEntryRootResultV1 {
    match result {
        FinalizedRootResultAbiV1::CallReturn { .. }
        | FinalizedRootResultAbiV1::I64AddReturn { .. }
        | FinalizedRootResultAbiV1::IntegerLiteralReturn { .. }
        | FinalizedRootResultAbiV1::I64FieldReturn { .. } => CompiledEntryRootResultV1::I64,
        FinalizedRootResultAbiV1::UnitReturn { .. } => CompiledEntryRootResultV1::Unit,
    }
}

fn fault(detail: &str) -> String {
    format!("[freeze:contract][published-lifecycle/{detail}]")
}

#[cfg(test)]
#[path = "compiled_entry_contract_tests.rs"]
mod tests;
