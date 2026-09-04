//! Synchronous C projection of the published MIR backend view.
//!
//! These rows own only physical transport.  The parent view remains the sole
//! source of route and definition meaning, and public re-exports preserve the
//! historical `crate::mir::function` paths.

use std::ffi::CString;
use std::os::raw::c_char;

use super::{
    PublishedMirBackendView, PublishedMirBackendViewErrorV1,
};
use crate::mir::ArrayElementWriteKind;

/// Physical row kinds carried across the typed C frame.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedCallKindV1 {
    StaticMethod = 1,
    BuiltinPrint = 2,
    FreeFunction = 3,
    ArrayLiteralAppend = 4,
    ArrayPush = 5,
    ArraySet = 6,
    ArrayInsert = 7,
}

pub(crate) const PUBLISHED_ROW_DST_PRESENT_V1: u32 = 1;
pub(crate) const PUBLISHED_ROW_INDEX_PRESENT_V1: u32 = 2;

/// Borrow-independent C transport row.  The C consumer receives only this
/// temporary frame; it cannot recover owner/method/arity from a symbol.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedStaticMethodCallCRowV1 {
    pub(crate) function_name: *const c_char,
    pub(crate) block_id: u32,
    pub(crate) instruction_index: u32,
    pub(crate) target_symbol: *const c_char,
    pub(crate) arity: u32,
    pub(crate) kind: u32,
    pub(crate) site_id: u32,
    pub(crate) receiver: u32,
    pub(crate) index: u32,
    pub(crate) value: u32,
    pub(crate) dst: u32,
    pub(crate) flags: u32,
}

/// Owned strings keep C row pointers valid for exactly one synchronous
/// backend call.  This is physical transport, not a second semantic owner.
#[derive(Debug)]
pub(crate) struct PublishedStaticMethodCFrameV1 {
    function_names: Vec<CString>,
    target_symbols: Vec<CString>,
    rows: Vec<PublishedStaticMethodCallCRowV1>,
}

impl PublishedStaticMethodCFrameV1 {
    pub(crate) fn from_view(
        view: &PublishedMirBackendView<'_>,
    ) -> Result<Self, PublishedMirBackendViewErrorV1> {
        let total = view.static_method_calls.len()
            + view.free_function_calls.len()
            + view.builtin_print_calls.len()
            + view.array_element_writes.len();
        let mut function_names = Vec::with_capacity(total);
        let mut target_symbols =
            Vec::with_capacity(view.static_method_calls.len() + view.free_function_calls.len());
        let mut rows = Vec::with_capacity(total);
        for (kind, function_name, block_id, instruction_index, key) in view
            .static_method_calls
            .iter()
            .map(|call| {
                (
                    PublishedCallKindV1::StaticMethod,
                    call.function_name,
                    call.block_id,
                    call.instruction_index,
                    call.key,
                )
            })
            .chain(view.free_function_calls.iter().map(|call| {
                (
                    PublishedCallKindV1::FreeFunction,
                    call.function_name,
                    call.block_id,
                    call.instruction_index,
                    call.key,
                )
            }))
        {
            let symbol = key.mir_symbol_projection();
            let function_name = CString::new(function_name).map_err(|_| {
                PublishedMirBackendViewErrorV1::DefinitionSymbolMismatch {
                    key: key.clone(),
                    symbol: function_name.to_owned(),
                }
            })?;
            let target_symbol = CString::new(symbol.clone()).map_err(|_| {
                PublishedMirBackendViewErrorV1::DefinitionSymbolMismatch {
                    key: key.clone(),
                    symbol,
                }
            })?;
            function_names.push(function_name);
            target_symbols.push(target_symbol);
            let function_name_ptr = function_names
                .last()
                .expect("just-pushed function name")
                .as_ptr();
            let target_symbol_ptr = target_symbols
                .last()
                .expect("just-pushed target symbol")
                .as_ptr();
            rows.push(PublishedStaticMethodCallCRowV1 {
                function_name: function_name_ptr,
                block_id,
                instruction_index,
                target_symbol: target_symbol_ptr,
                arity: key.arity(),
                kind: kind as u32,
                site_id: 0,
                receiver: 0,
                index: 0,
                value: 0,
                dst: 0,
                flags: 0,
            });
        }
        for call in &view.builtin_print_calls {
            let function_name = CString::new(call.function_name).map_err(|_| {
                PublishedMirBackendViewErrorV1::BuiltinPrintArityMismatch {
                    function: call.function_name.to_owned(),
                    expected: 1,
                    actual: call.args.len(),
                }
            })?;
            function_names.push(function_name);
            let function_name_ptr = function_names
                .last()
                .expect("just-pushed builtin function name")
                .as_ptr();
            rows.push(PublishedStaticMethodCallCRowV1 {
                function_name: function_name_ptr,
                block_id: call.block_id,
                instruction_index: call.instruction_index,
                target_symbol: std::ptr::null(),
                arity: 1,
                kind: PublishedCallKindV1::BuiltinPrint as u32,
                site_id: 0,
                receiver: 0,
                index: 0,
                value: 0,
                dst: 0,
                flags: 0,
            });
        }
        for write in &view.array_element_writes {
            let function_name = CString::new(write.function_name()).map_err(|_| {
                PublishedMirBackendViewErrorV1::ArrayElementWriteShapeMismatch {
                    function: write.function_name().to_owned(),
                    kind: write.kind(),
                }
            })?;
            let mut flags = 0;
            let index = write.index().map_or(0, |value| {
                flags |= PUBLISHED_ROW_INDEX_PRESENT_V1;
                value.as_u32()
            });
            let dst = write.dst().map_or(0, |value| {
                flags |= PUBLISHED_ROW_DST_PRESENT_V1;
                value.as_u32()
            });
            function_names.push(function_name);
            let function_name_ptr = function_names
                .last()
                .expect("just-pushed array-write function name")
                .as_ptr();
            rows.push(PublishedStaticMethodCallCRowV1 {
                function_name: function_name_ptr,
                block_id: write.block_id(),
                instruction_index: write.instruction_index(),
                target_symbol: std::ptr::null(),
                arity: 0,
                kind: array_write_kind(write.kind()) as u32,
                site_id: write.site_id(),
                receiver: write.receiver().as_u32(),
                index,
                value: write.value().as_u32(),
                dst,
                flags,
            });
        }
        Ok(Self {
            function_names,
            target_symbols,
            rows,
        })
    }

    pub(crate) fn as_ptr(&self) -> *const PublishedStaticMethodCallCRowV1 {
        self.rows.as_ptr()
    }

    pub(crate) const fn len(&self) -> usize {
        self.rows.len()
    }

    pub(crate) fn as_slice(&self) -> &[PublishedStaticMethodCallCRowV1] {
        &self.rows
    }

    #[cfg(test)]
    fn row(&self, index: usize) -> PublishedStaticMethodCallCRowV1 {
        self.rows[index]
    }
}

fn array_write_kind(kind: ArrayElementWriteKind) -> PublishedCallKindV1 {
    match kind {
        ArrayElementWriteKind::LiteralAppend => PublishedCallKindV1::ArrayLiteralAppend,
        ArrayElementWriteKind::Push => PublishedCallKindV1::ArrayPush,
        ArrayElementWriteKind::Set => PublishedCallKindV1::ArraySet,
        ArrayElementWriteKind::Insert => PublishedCallKindV1::ArrayInsert,
    }
}
