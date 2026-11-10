use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum MirOwnershipKindV1 {
    None,
    Borrowed,
    Owned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum FunctionResultOwnershipV1 {
    None,
    Owned,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum OwnershipFunctionOwnerV1 {
    Legacy(u64),
    Source(crate::mir::resolved_semantics::FunctionOwnerIdV1),
}

impl OwnershipFunctionOwnerV1 {
    pub(crate) const fn new(raw: u64) -> Self {
        Self::Legacy(raw)
    }

    pub(crate) const fn source(owner: crate::mir::resolved_semantics::FunctionOwnerIdV1) -> Self {
        Self::Source(owner)
    }

    pub(crate) const fn is_valid(self) -> bool {
        !matches!(self, Self::Legacy(u64::MAX))
    }

    pub(super) const fn is_source(self) -> bool { matches!(self, Self::Source(_)) }

    pub(crate) const fn legacy_id(self) -> Option<u64> {
        match self { Self::Legacy(value) => Some(value), Self::Source(_) => None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OwnershipFunctionAbiV1 {
    owner: OwnershipFunctionOwnerV1,
    parameter_kinds: Box<[MirOwnershipKindV1]>,
    result: FunctionResultOwnershipV1,
    /// Exact finished New-local copies selected by the source handoff.
    aliases: BTreeMap<ValueId, ValueId>,
    initially_published: BTreeSet<ValueId>,
    /// Source-certified entry-owned parameters, already completely constructed.
    owned_parameters: BTreeMap<ValueId, hakorune_mir_defs::CanonicalObjectIdV1>,
    /// Ordinary take transfers at entry, on both outcomes (unlike Birth).
    call_takes: BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, BTreeMap<u32, hakorune_mir_defs::CanonicalObjectIdV1>>,
    /// Exact complete End contracts retained from this function's source owner.
    shared_descriptors: Vec<crate::mir::instruction::SharedDescriptorV1>,
    /// Source-admitted nullable owned formals. Values remain solely in Birth's
    /// ordered arguments; only its Normal edge transfers their existing tokens.
    birth_takes: BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, BTreeSet<u32>>,
}

impl OwnershipFunctionAbiV1 {
    pub(crate) fn new(
        owner: OwnershipFunctionOwnerV1,
        parameter_kinds: Vec<MirOwnershipKindV1>,
        result: FunctionResultOwnershipV1,
    ) -> Self {
        Self {
            owner,
            parameter_kinds: parameter_kinds.into_boxed_slice(),
            result,
            aliases: BTreeMap::new(),
            initially_published: BTreeSet::new(),
            owned_parameters: BTreeMap::new(),
            call_takes: BTreeMap::new(),
            shared_descriptors: Vec::new(),
            birth_takes: BTreeMap::new(),
        }
    }

    pub(crate) const fn owner(&self) -> OwnershipFunctionOwnerV1 {
        self.owner
    }

    pub(crate) fn parameter_kinds(&self) -> &[MirOwnershipKindV1] {
        &self.parameter_kinds
    }

    pub(crate) const fn result(&self) -> FunctionResultOwnershipV1 {
        self.result
    }

    pub(crate) fn with_source_aliases(mut self, aliases: BTreeMap<ValueId, ValueId>) -> Self {
        self.aliases = aliases;
        self
    }

    pub(super) fn aliases(&self) -> &BTreeMap<ValueId, ValueId> { &self.aliases }
    pub(crate) fn with_initially_published(mut self, values: BTreeSet<ValueId>) -> Self {
        self.initially_published = values;
        self
    }
    pub(super) fn initially_published(&self) -> &BTreeSet<ValueId> { &self.initially_published }
    pub(crate) fn with_source_owned_parameters(mut self, parameters: BTreeMap<ValueId, hakorune_mir_defs::CanonicalObjectIdV1>) -> Self {
        self.owned_parameters = parameters;
        self
    }
    pub(super) fn owned_parameters(&self) -> &BTreeMap<ValueId, hakorune_mir_defs::CanonicalObjectIdV1> { &self.owned_parameters }
    pub(crate) fn with_source_call_takes(mut self, takes: BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, BTreeMap<u32, hakorune_mir_defs::CanonicalObjectIdV1>>) -> Self {
        self.call_takes = takes;
        self
    }
    pub(super) fn call_takes(&self, callee: &crate::mir::Callee) -> Option<(u32, &BTreeMap<u32, hakorune_mir_defs::CanonicalObjectIdV1>)> {
        if !self.owner.is_source() { return None; }
        self.call_takes.iter().find_map(|(key, takes)| {
            let matches = match callee {
                crate::mir::Callee::SameModuleInstance { key: actual, .. } => actual == key,
                crate::mir::Callee::Global(actual) => key.canonical_global_target_v1().as_ref() == Ok(actual),
                _ => false,
            };
            matches.then_some((key.arity(), takes))
        })
    }


    pub(crate) fn with_source_shared_descriptors(mut self, rows: Vec<crate::mir::instruction::SharedDescriptorV1>) -> Self {
        self.shared_descriptors = rows;
        self
    }
    pub(crate) fn with_source_birth_takes(mut self,
        takes: BTreeMap<hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, BTreeSet<u32>>) -> Self {
        self.birth_takes = takes;
        self
    }
    pub(super) fn birth_takes(&self, key: &hakorune_mir_defs::CanonicalSameModuleCallableKeyV1, ordinal: usize) -> bool {
        self.owner.is_source() && self.birth_takes.get(key).is_some_and(|rows| rows.contains(&(ordinal as u32)))
    }
    pub(super) fn admits_shared_descriptor(&self, row: &crate::mir::instruction::SharedDescriptorV1) -> bool {
        self.shared_descriptors.contains(row)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum OwnershipDispositionV1 {
    Destroy {
        block: BasicBlockId,
    },
    Return {
        block: BasicBlockId,
    },
    PhiEdge {
        predecessor: BasicBlockId,
        successor: BasicBlockId,
        destination: ValueId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum OwnershipOperationKindV1 {
    CopyOwned { dst: ValueId, src: ValueId },
    DestroyOwned { value: ValueId },
    Shared { operation: crate::mir::instruction::SharedInvokeOperation, normal: BasicBlockId, fault: BasicBlockId },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OwnershipOperationV1 {
    block: BasicBlockId,
    instruction_index: usize,
    kind: OwnershipOperationKindV1,
}

impl OwnershipOperationV1 {
    pub(crate) const fn block(&self) -> BasicBlockId {
        self.block
    }

    pub(crate) const fn instruction_index(&self) -> usize {
        self.instruction_index
    }

    pub(crate) const fn kind(&self) -> &OwnershipOperationKindV1 {
        &self.kind
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedOwnershipSsaV1 {
    abi: OwnershipFunctionAbiV1,
    kinds: BTreeMap<ValueId, MirOwnershipKindV1>,
    dispositions: BTreeMap<ValueId, Box<[OwnershipDispositionV1]>>,
    operations: Box<[OwnershipOperationV1]>,
}

impl VerifiedOwnershipSsaV1 {
    pub(super) fn new(
        abi: OwnershipFunctionAbiV1,
        kinds: BTreeMap<ValueId, MirOwnershipKindV1>,
        dispositions: BTreeMap<ValueId, Box<[OwnershipDispositionV1]>>,
        operations: Box<[OwnershipOperationV1]>,
    ) -> Self {
        Self {
            abi,
            kinds,
            dispositions,
            operations,
        }
    }

    pub(crate) const fn owner(&self) -> OwnershipFunctionOwnerV1 {
        self.abi.owner()
    }

    pub(crate) fn kind(&self, value: ValueId) -> Option<MirOwnershipKindV1> {
        self.kinds.get(&value).copied()
    }

    pub(crate) fn dispositions(&self, value: ValueId) -> &[OwnershipDispositionV1] {
        self.dispositions
            .get(&value)
            .map(Box::as_ref)
            .unwrap_or(&[])
    }

    pub(crate) fn kinds(&self) -> impl Iterator<Item = (ValueId, MirOwnershipKindV1)> + '_ {
        self.kinds.iter().map(|(value, kind)| (*value, *kind))
    }

    pub(crate) fn operations(&self) -> &[OwnershipOperationV1] {
        &self.operations
    }

    pub(crate) fn matches_function(&self, function: &MirFunction) -> bool {
        super::verify::verify(function, &self.abi, None).as_ref() == Ok(self)
    }
}

pub(super) fn collect_ownership_operations(function: &MirFunction) -> Box<[OwnershipOperationV1]> {
    let mut operations = Vec::new();
    for (block_id, block) in &function.blocks {
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let kind = match instruction {
                MirInstruction::CopyOwned { dst, src } => OwnershipOperationKindV1::CopyOwned {
                    dst: *dst,
                    src: *src,
                },
                MirInstruction::DestroyOwned { value } => {
                    OwnershipOperationKindV1::DestroyOwned { value: *value }
                }
                _ => continue,
            };
            operations.push(OwnershipOperationV1 {
                block: *block_id,
                instruction_index,
                kind,
            });
        }
        if let Some(MirInstruction::Invoke {
            operation: crate::mir::instruction::InvokeOperation::Shared(operation),
            normal_landing, fault_landing, ..
        }) = &block.terminator {
            operations.push(OwnershipOperationV1 {
                block: *block_id,
                instruction_index: block.instructions.len(),
                kind: OwnershipOperationKindV1::Shared {
                    operation: operation.clone(), normal: *normal_landing, fault: *fault_landing,
                },
            });
        }
    }
    operations.into_boxed_slice()
}
