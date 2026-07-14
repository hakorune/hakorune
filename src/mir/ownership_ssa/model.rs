use crate::mir::{BasicBlockId, ValueId};
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct VerifiedOwnershipSsaV1 {
    owner: OwnershipFunctionOwnerV1,
    kinds: BTreeMap<ValueId, MirOwnershipKindV1>,
    dispositions: BTreeMap<ValueId, Box<[OwnershipDispositionV1]>>,
}

impl VerifiedOwnershipSsaV1 {
    pub(super) fn new(
        owner: OwnershipFunctionOwnerV1,
        kinds: BTreeMap<ValueId, MirOwnershipKindV1>,
        dispositions: BTreeMap<ValueId, Box<[OwnershipDispositionV1]>>,
    ) -> Self {
        Self {
            owner,
            kinds,
            dispositions,
        }
    }

    pub(crate) const fn owner(&self) -> OwnershipFunctionOwnerV1 {
        self.owner
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
}
