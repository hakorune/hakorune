//! Borrow-only view of the canonical callable relation after module publish.
//!
//! This module deliberately does not resolve names, inspect source, or repair
//! legacy call operands.  It validates the relation already published by
//! `MirModule` and exposes only references for a backend consumer.

use std::ffi::CString;
use std::os::raw::c_char;

use hakorune_mir_defs::{
    CanonicalBuiltinGlobalV1, CanonicalGlobalTargetV1, CanonicalSameModuleCallableKeyV1,
    CanonicalSameModuleGlobalTargetV1, SameModuleCallableNamespaceV1,
};

use crate::mir::{Callee, MirFunction, MirInstruction, MirModule, ValueId};

/// The only route decisions a backend may observe for the selected published
/// call family (static method, builtin print, or free function).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedStaticMethodRouteV1 {
    /// At least one selected call exists.  Other call families may remain on
    /// their explicit compatibility routes until their own cohort is cut over.
    CanonicalTyped,
    /// No selected typed call exists; an explicit compatibility caller may
    /// consume the module without pretending it is canonical.
    ExplicitCompatibility,
}

/// Physical row kinds carried across the typed C frame.  This is deliberately
/// a transport discriminator, not a second semantic target authority: the
/// canonical global target was already selected before this view is built.
#[repr(u32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedCallKindV1 {
    StaticMethod = 1,
    BuiltinPrint = 2,
    FreeFunction = 3,
}

/// Publication/view failures are physical admission failures.  They never
/// trigger a second resolver or a compatibility retry for a selected module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PublishedMirBackendViewErrorV1 {
    DefinitionMissing {
        key: CanonicalSameModuleCallableKeyV1,
    },
    DefinitionSymbolMismatch {
        key: CanonicalSameModuleCallableKeyV1,
        symbol: String,
    },
    DefinitionArityMismatch {
        key: CanonicalSameModuleCallableKeyV1,
        expected: usize,
        actual: usize,
    },
    StaticCallDefinitionMissing {
        function: String,
        key: CanonicalSameModuleCallableKeyV1,
    },
    StaticCallUsesLegacyFunctionCarrier {
        function: String,
        key: CanonicalSameModuleCallableKeyV1,
        func: ValueId,
    },
    StaticCallArityMismatch {
        function: String,
        key: CanonicalSameModuleCallableKeyV1,
        expected: usize,
        actual: usize,
    },
    StaticMethodRequiresIntegerReturn {
        function: String,
        key: CanonicalSameModuleCallableKeyV1,
    },
    FreeFunctionCallDefinitionMissing {
        function: String,
        key: CanonicalSameModuleCallableKeyV1,
    },
    FreeFunctionCallUsesLegacyFunctionCarrier {
        function: String,
        key: CanonicalSameModuleCallableKeyV1,
        func: ValueId,
    },
    FreeFunctionCallArityMismatch {
        function: String,
        key: CanonicalSameModuleCallableKeyV1,
        expected: usize,
        actual: usize,
    },
    FreeFunctionRequiresIntegerReturn {
        function: String,
        key: CanonicalSameModuleCallableKeyV1,
    },
    BuiltinPrintUsesLegacyFunctionCarrier {
        function: String,
        func: ValueId,
    },
    BuiltinPrintHasDestination {
        function: String,
        dst: ValueId,
    },
    BuiltinPrintArityMismatch {
        function: String,
        expected: usize,
        actual: usize,
    },
}

impl std::fmt::Display for PublishedMirBackendViewErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][published-mir-backend-view] {self:?}"
        )
    }
}

impl std::error::Error for PublishedMirBackendViewErrorV1 {}

/// A single selected static-method call borrowed from the published module.
/// The key and operands are never reconstructed from a physical symbol.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedStaticMethodCallRef<'module> {
    function_name: &'module str,
    block_id: u32,
    instruction_index: u32,
    key: &'module CanonicalSameModuleCallableKeyV1,
    args: &'module [ValueId],
}

/// A same-module free-function call borrowed from the published module.
/// `key` is the source-issued identity retained through Atomic Publish; the
/// physical symbol is projected only when the temporary C frame is built.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedFreeFunctionCallRef<'module> {
    function_name: &'module str,
    block_id: u32,
    instruction_index: u32,
    key: &'module CanonicalSameModuleCallableKeyV1,
    args: &'module [ValueId],
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
    function_name: &'module str,
    block_id: u32,
    instruction_index: u32,
    args: &'module [ValueId],
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

/// Read-only projection of an already atomically-published module.
///
/// No AST, resolver, registry, JSON, fallback state, or independently-owned
/// semantic table is stored here. A selected call has one typed route; calls
/// from other families remain explicit compatibility data until their cohort
/// is selected. They never alter the selected call's target relation.
#[derive(Debug)]
pub(crate) struct PublishedMirBackendView<'module> {
    module: &'module MirModule,
    route: PublishedStaticMethodRouteV1,
    static_method_calls: Vec<PublishedStaticMethodCallRef<'module>>,
    free_function_calls: Vec<PublishedFreeFunctionCallRef<'module>>,
    builtin_print_calls: Vec<PublishedBuiltinPrintCallRef<'module>>,
}

/// Borrow-independent C transport rows. The C consumer receives only this
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
}

/// Owned strings keep C row pointers valid for exactly one synchronous
/// backend call. This is physical transport, not a second semantic owner.
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
            + view.builtin_print_calls.len();
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

impl<'module> PublishedMirBackendView<'module> {
    pub(crate) fn try_new(
        module: &'module MirModule,
    ) -> Result<Self, PublishedMirBackendViewErrorV1> {
        validate_definition_table(module)?;

        let mut static_method_calls = Vec::new();
        let mut free_function_calls = Vec::new();
        let mut builtin_print_calls = Vec::new();
        for (function_name, function) in &module.functions {
            let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
            block_ids.sort();
            for block_id in block_ids {
                let block = function
                    .blocks
                    .get(&block_id)
                    .expect("sorted MIR block id must remain present");
                for (instruction_index, instruction) in block.all_instructions().enumerate() {
                    let MirInstruction::Call {
                        dst,
                        func,
                        callee,
                        args,
                        ..
                    } = instruction
                    else {
                        continue;
                    };

                    match callee {
                        Some(Callee::Global(target)) => {
                            if let Some(key) = static_method_key(target) {
                                let published_key =
                                    validate_static_call(module, function_name, key, *func, args)?;
                                static_method_calls.push(PublishedStaticMethodCallRef {
                                    function_name: function_name.as_str(),
                                    block_id: block_id.as_u32(),
                                    instruction_index: instruction_index as u32,
                                    key: published_key,
                                    args: args.as_slice(),
                                });
                            } else if let Some(key) = free_function_key(target) {
                                let published_key = validate_free_function_call(
                                    module,
                                    function_name,
                                    key,
                                    *func,
                                    args,
                                )?;
                                free_function_calls.push(PublishedFreeFunctionCallRef {
                                    function_name: function_name.as_str(),
                                    block_id: block_id.as_u32(),
                                    instruction_index: instruction_index as u32,
                                    key: published_key,
                                    args: args.as_slice(),
                                });
                            } else if is_builtin_print_target(target) {
                                validate_builtin_print_call(function_name, *dst, *func, args)?;
                                builtin_print_calls.push(PublishedBuiltinPrintCallRef {
                                    function_name: function_name.as_str(),
                                    block_id: block_id.as_u32(),
                                    instruction_index: instruction_index as u32,
                                    args: args.as_slice(),
                                });
                            }
                        }
                        Some(_) | None => {}
                    }
                }
            }
        }

        if static_method_calls.is_empty()
            && free_function_calls.is_empty()
            && builtin_print_calls.is_empty()
        {
            return Ok(Self {
                module,
                route: PublishedStaticMethodRouteV1::ExplicitCompatibility,
                static_method_calls,
                free_function_calls,
                builtin_print_calls,
            });
        }
        Ok(Self {
            module,
            route: PublishedStaticMethodRouteV1::CanonicalTyped,
            static_method_calls,
            free_function_calls,
            builtin_print_calls,
        })
    }

    pub(crate) const fn route(&self) -> PublishedStaticMethodRouteV1 {
        self.route
    }

    pub(crate) fn static_method_calls(&self) -> &[PublishedStaticMethodCallRef<'module>] {
        &self.static_method_calls
    }

    pub(crate) fn free_function_calls(&self) -> &[PublishedFreeFunctionCallRef<'module>] {
        &self.free_function_calls
    }

    pub(crate) fn builtin_print_calls(&self) -> &[PublishedBuiltinPrintCallRef<'module>] {
        &self.builtin_print_calls
    }

    /// Borrow the one published physical definition for an already-selected
    /// key.  This is a relation lookup, not a name resolver.
    pub(crate) fn definition(
        &self,
        key: &CanonicalSameModuleCallableKeyV1,
    ) -> Option<&'module MirFunction> {
        let symbol = self.module.canonical_callable_definition_symbol(key)?;
        self.module.functions.get(symbol)
    }
}

fn is_builtin_print_target(target: &CanonicalGlobalTargetV1) -> bool {
    matches!(
        target,
        CanonicalGlobalTargetV1::Builtin(CanonicalBuiltinGlobalV1::Print)
    )
}

fn validate_builtin_print_call(
    function_name: &str,
    dst: Option<ValueId>,
    func: ValueId,
    args: &[ValueId],
) -> Result<(), PublishedMirBackendViewErrorV1> {
    if func != ValueId::INVALID {
        return Err(
            PublishedMirBackendViewErrorV1::BuiltinPrintUsesLegacyFunctionCarrier {
                function: function_name.to_owned(),
                func,
            },
        );
    }
    if let Some(dst) = dst {
        return Err(PublishedMirBackendViewErrorV1::BuiltinPrintHasDestination {
            function: function_name.to_owned(),
            dst,
        });
    }
    if args.len() != 1 {
        return Err(PublishedMirBackendViewErrorV1::BuiltinPrintArityMismatch {
            function: function_name.to_owned(),
            expected: 1,
            actual: args.len(),
        });
    }
    Ok(())
}

fn validate_definition_table(module: &MirModule) -> Result<(), PublishedMirBackendViewErrorV1> {
    for (key, symbol) in &module.canonical_callable_definitions {
        let Some(function) = module.functions.get(symbol) else {
            return Err(PublishedMirBackendViewErrorV1::DefinitionMissing { key: key.clone() });
        };
        if symbol != &key.mir_symbol_projection() {
            return Err(PublishedMirBackendViewErrorV1::DefinitionSymbolMismatch {
                key: key.clone(),
                symbol: symbol.clone(),
            });
        }
        let expected = expected_physical_arity(key);
        let actual = function.signature.params.len();
        if actual != expected {
            return Err(PublishedMirBackendViewErrorV1::DefinitionArityMismatch {
                key: key.clone(),
                expected,
                actual,
            });
        }
    }
    Ok(())
}

fn validate_static_call<'module>(
    module: &'module MirModule,
    function_name: &str,
    key: CanonicalSameModuleCallableKeyV1,
    func: ValueId,
    args: &'module [ValueId],
) -> Result<&'module CanonicalSameModuleCallableKeyV1, PublishedMirBackendViewErrorV1> {
    if func != ValueId::INVALID {
        return Err(
            PublishedMirBackendViewErrorV1::StaticCallUsesLegacyFunctionCarrier {
                function: function_name.to_owned(),
                key,
                func,
            },
        );
    }
    let Some((published_key, symbol)) = module
        .canonical_callable_definitions
        .iter()
        .find(|(published_key, _)| *published_key == &key)
    else {
        return Err(
            PublishedMirBackendViewErrorV1::StaticCallDefinitionMissing {
                function: function_name.to_owned(),
                key,
            },
        );
    };
    let expected = key.arity() as usize;
    if args.len() != expected {
        return Err(PublishedMirBackendViewErrorV1::StaticCallArityMismatch {
            function: function_name.to_owned(),
            key,
            expected,
            actual: args.len(),
        });
    }
    let Some(function) = module.functions.get(symbol) else {
        return Err(
            PublishedMirBackendViewErrorV1::StaticCallDefinitionMissing {
                function: function_name.to_owned(),
                key,
            },
        );
    };
    if function.signature.return_type != crate::mir::MirType::Integer {
        return Err(
            PublishedMirBackendViewErrorV1::StaticMethodRequiresIntegerReturn {
                function: function_name.to_owned(),
                key,
            },
        );
    }
    debug_assert_eq!(symbol, &key.mir_symbol_projection());
    Ok(published_key)
}

fn validate_free_function_call<'module>(
    module: &'module MirModule,
    function_name: &str,
    key: CanonicalSameModuleCallableKeyV1,
    func: ValueId,
    args: &'module [ValueId],
) -> Result<&'module CanonicalSameModuleCallableKeyV1, PublishedMirBackendViewErrorV1> {
    if func != ValueId::INVALID {
        return Err(
            PublishedMirBackendViewErrorV1::FreeFunctionCallUsesLegacyFunctionCarrier {
                function: function_name.to_owned(),
                key,
                func,
            },
        );
    }
    let Some((published_key, symbol)) = module
        .canonical_callable_definitions
        .iter()
        .find(|(published_key, _)| *published_key == &key)
    else {
        return Err(
            PublishedMirBackendViewErrorV1::FreeFunctionCallDefinitionMissing {
                function: function_name.to_owned(),
                key,
            },
        );
    };
    let expected = key.arity() as usize;
    if args.len() != expected {
        return Err(
            PublishedMirBackendViewErrorV1::FreeFunctionCallArityMismatch {
                function: function_name.to_owned(),
                key,
                expected,
                actual: args.len(),
            },
        );
    }
    let Some(function) = module.functions.get(symbol) else {
        return Err(
            PublishedMirBackendViewErrorV1::FreeFunctionCallDefinitionMissing {
                function: function_name.to_owned(),
                key,
            },
        );
    };
    if function.signature.return_type != crate::mir::MirType::Integer {
        return Err(
            PublishedMirBackendViewErrorV1::FreeFunctionRequiresIntegerReturn {
                function: function_name.to_owned(),
                key,
            },
        );
    }
    debug_assert_eq!(symbol, &key.mir_symbol_projection());
    Ok(published_key)
}

fn expected_physical_arity(key: &CanonicalSameModuleCallableKeyV1) -> usize {
    match key.namespace() {
        SameModuleCallableNamespaceV1::FreeFunction => key.arity() as usize,
        SameModuleCallableNamespaceV1::StaticBoxMethod => key.arity() as usize,
        SameModuleCallableNamespaceV1::InstanceBoxMethod => key.arity() as usize + 1,
    }
}

fn static_method_key(target: &CanonicalGlobalTargetV1) -> Option<CanonicalSameModuleCallableKeyV1> {
    let CanonicalGlobalTargetV1::SameModule(CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
        owner,
        method,
        arity,
    }) = target
    else {
        return None;
    };
    Some(CanonicalSameModuleCallableKeyV1::static_box_method(
        owner, method, *arity,
    ))
}

fn free_function_key(target: &CanonicalGlobalTargetV1) -> Option<CanonicalSameModuleCallableKeyV1> {
    let CanonicalGlobalTargetV1::SameModule(CanonicalSameModuleGlobalTargetV1::FreeFunction {
        name,
        arity,
    }) = target
    else {
        return None;
    };
    Some(CanonicalSameModuleCallableKeyV1::free_function(
        name, *arity,
    ))
}

#[cfg(test)]
#[path = "published_backend_view_tests.rs"]
mod tests;
