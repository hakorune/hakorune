//! Borrowed physical row references; the published view owns their collection.
use super::*;

/// A single selected static-method call borrowed from the published module.
/// The key and operands are never reconstructed from a physical symbol.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedStaticMethodCallRef<'module> {
    pub(super) function_name: &'module str,
    pub(super) block_id: u32,
    pub(super) instruction_index: u32,
    pub(super) key: &'module CanonicalSameModuleCallableKeyV1,
    pub(super) args: &'module [ValueId],
}

/// A same-module free-function call borrowed from the published module.
/// `key` is the source-issued identity retained through Atomic Publish; the
/// physical symbol is projected only when the temporary C frame is built.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedFreeFunctionCallRef<'module> {
    pub(super) function_name: &'module str,
    pub(super) block_id: u32,
    pub(super) instruction_index: u32,
    pub(super) key: &'module CanonicalSameModuleCallableKeyV1,
    pub(super) args: &'module [ValueId],
}

impl<'module> PublishedFreeFunctionCallRef<'module> {
    pub(crate) fn function_name(self) -> &'module str {
        self.function_name
    }

    pub(crate) fn key(self) -> &'module CanonicalSameModuleCallableKeyV1 {
        self.key
    }

    pub(crate) const fn block_id(self) -> u32 {
        self.block_id
    }

    pub(crate) const fn instruction_index(self) -> u32 {
        self.instruction_index
    }

    pub(crate) fn args(self) -> &'module [ValueId] {
        self.args
    }
}

impl<'module> PublishedStaticMethodCallRef<'module> {
    pub(crate) fn function_name(self) -> &'module str {
        self.function_name
    }

    pub(crate) fn key(self) -> &'module CanonicalSameModuleCallableKeyV1 {
        self.key
    }

    pub(crate) const fn block_id(self) -> u32 {
        self.block_id
    }

    pub(crate) const fn instruction_index(self) -> u32 {
        self.instruction_index
    }

    pub(crate) fn args(self) -> &'module [ValueId] {
        self.args
    }
}

/// A reserved builtin print call borrowed from the published module.  Unlike
/// same-module methods it has no definition-table key: its finite builtin
/// identity is already carried by the canonical global target.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedBuiltinPrintCallRef<'module> {
    pub(super) function_name: &'module str,
    pub(super) block_id: u32,
    pub(super) instruction_index: u32,
    pub(super) args: &'module [ValueId],
}

impl<'module> PublishedBuiltinPrintCallRef<'module> {
    pub(crate) fn function_name(self) -> &'module str {
        self.function_name
    }

    pub(crate) const fn block_id(self) -> u32 {
        self.block_id
    }

    pub(crate) const fn instruction_index(self) -> u32 {
        self.instruction_index
    }

    pub(crate) fn args(self) -> &'module [ValueId] {
        self.args
    }
}

/// A canonical ArrayElementWrite borrowed from the atomically-published MIR.
/// The operation kind, receiver, index, and value are already decided by the
/// ArrayElementWrite owner; the backend only projects these operands.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedArrayElementWriteRef<'module> {
    pub(super) function_name: &'module str,
    pub(super) block_id: u32,
    pub(super) instruction_index: u32,
    pub(super) site_id: u32,
    pub(super) kind: ArrayElementWriteKind,
    pub(super) dst: Option<ValueId>,
    pub(super) receiver: ValueId,
    pub(super) index: Option<ValueId>,
    pub(super) value: ValueId,
}

impl<'module> PublishedArrayElementWriteRef<'module> {
    pub(crate) fn function_name(self) -> &'module str {
        self.function_name
    }

    pub(crate) const fn block_id(self) -> u32 {
        self.block_id
    }

    pub(crate) const fn instruction_index(self) -> u32 {
        self.instruction_index
    }

    pub(crate) const fn site_id(self) -> u32 {
        self.site_id
    }

    pub(crate) const fn kind(self) -> ArrayElementWriteKind {
        self.kind
    }

    pub(crate) const fn dst(self) -> Option<ValueId> {
        self.dst
    }

    pub(crate) const fn receiver(self) -> ValueId {
        self.receiver
    }

    pub(crate) const fn index(self) -> Option<ValueId> {
        self.index
    }

    pub(crate) const fn value(self) -> ValueId {
        self.value
    }
}

#[derive(Debug, Clone, Copy)]
pub(super) struct PublishedIntrinsicArrayRef<'module> {
    pub(super) function_name: &'module str,
    pub(super) block_id: u32,
    pub(super) instruction_index: u32,
    pub(super) dst: ValueId,
}
