//! One physical compiled-entry contract issued by an activated lifecycle view.
//!
//! It joins already-retained source/handoff facts to final-MIR parameter values.
//! It neither classifies source parameters nor makes C ABI choices.

use crate::mir::normal_callable_semantic_package::{
    BirthFormalPhysicalDispositionV1, FinalizedRootResultAbiV1,
};
use crate::mir::ValueId;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct CompiledEntryFormalV1 {
    source_ordinal: Option<u32>,
    physical_ordinal: u32,
    value: ValueId,
    kind: CompiledEntryFormalKindV1,
    disposition: Option<BirthFormalPhysicalDispositionV1>,
}

impl CompiledEntryFormalV1 {
    pub(crate) const fn source_ordinal(self) -> Option<u32> {
        self.source_ordinal
    }
    pub(crate) const fn physical_ordinal(self) -> u32 {
        self.physical_ordinal
    }
    pub(crate) const fn value(self) -> ValueId {
        self.value
    }
    pub(crate) const fn kind(self) -> CompiledEntryFormalKindV1 {
        self.kind
    }
    pub(crate) const fn disposition(self) -> Option<BirthFormalPhysicalDispositionV1> {
        self.disposition
    }
}

#[derive(Debug, Clone)]
pub(crate) struct CompiledEntryBirthV1 {
    function_index: u32,
    formals: Box<[CompiledEntryFormalV1]>,
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
pub(crate) struct CompiledEntryContractV1<'module> {
    program: PublishedLifecyclePhysicalProgramV1<'module>,
    root_result: FinalizedRootResultAbiV1,
    births: Box<[CompiledEntryBirthV1]>,
}

impl<'module> CompiledEntryContractV1<'module> {
    pub(crate) fn program(&self) -> &PublishedLifecyclePhysicalProgramV1<'module> {
        &self.program
    }
    pub(crate) const fn root_result(&self) -> FinalizedRootResultAbiV1 {
        self.root_result
    }
    pub(crate) fn births(&self) -> &[CompiledEntryBirthV1] {
        &self.births
    }
}

impl<'module> PublishedMirBackendView<'module> {
    pub(crate) fn issue_lifecycle_compiled_entry_contract(
        &self,
    ) -> Result<CompiledEntryContractV1<'module>, String> {
        let program = self.issue_lifecycle_physical_program()?;
        let (root_result, contract_births) = {
            let [root, births @ ..] = program.functions() else {
                return Err(fault("compiled-entry-root-missing"));
            };
            let PublishedLifecyclePhysicalFunctionRoleV1::RootI64 { result } = root.role() else {
                return Err(fault("compiled-entry-root-role"));
            };
            let mut contract_births = Vec::with_capacity(births.len());
            for (index, function) in births.iter().enumerate() {
                let PublishedLifecyclePhysicalFunctionRoleV1::BirthUnit { abi } = function.role()
                else {
                    return Err(fault("compiled-entry-birth-role"));
                };
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
                    disposition: None,
                });
                for (ordinal, (lane, contract)) in abi
                    .parameters()
                    .iter()
                    .zip(abi.formal_contracts())
                    .enumerate()
                {
                    let ordinal =
                        u32::try_from(ordinal).map_err(|_| fault("compiled-entry-ordinal"))?;
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
                        disposition: Some(contract.disposition()),
                    });
                }
                contract_births.push(CompiledEntryBirthV1 {
                    function_index: u32::try_from(index + 1)
                        .map_err(|_| fault("compiled-entry-index"))?,
                    formals: formals.into_boxed_slice(),
                });
            }
            (*result, contract_births)
        };
        Ok(CompiledEntryContractV1 {
            program,
            root_result,
            births: contract_births.into_boxed_slice(),
        })
    }
}

fn fault(detail: &str) -> String {
    format!("[freeze:contract][published-lifecycle/{detail}]")
}
