use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

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
pub(crate) struct OwnershipFunctionOwnerV1(u64);

impl OwnershipFunctionOwnerV1 {
    pub(crate) const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub(crate) const fn as_u64(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct OwnershipFunctionAbiV1 {
    owner: OwnershipFunctionOwnerV1,
    parameter_kinds: Box<[MirOwnershipKindV1]>,
    result: FunctionResultOwnershipV1,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OwnershipOperationKindV1 {
    CopyOwned { dst: ValueId, src: ValueId },
    DestroyOwned { value: ValueId },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct OwnershipOperationV1 {
    block: BasicBlockId,
    instruction_index: usize,
    kind: OwnershipOperationKindV1,
}

impl OwnershipOperationV1 {
    pub(crate) const fn block(self) -> BasicBlockId {
        self.block
    }

    pub(crate) const fn instruction_index(self) -> usize {
        self.instruction_index
    }

    pub(crate) const fn kind(self) -> OwnershipOperationKindV1 {
        self.kind
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
        super::verify::verify(function, &self.abi).as_ref() == Ok(self)
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
    }
    operations.into_boxed_slice()
}
