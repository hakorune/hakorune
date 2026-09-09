//! Borrowed module and finalized handoff after canonical callable publication.
//!
//! This module deliberately does not resolve names, inspect source, or repair
//! legacy call operands.  It validates the relation already published by
//! `MirModule` and exposes references for a backend consumer. Physical row
//! vectors remain local to this view; finalized semantic products and selected
//! lifecycle profile stay owned by the normal finalization invocation.

use hakorune_mir_defs::{
    CanonicalBuiltinGlobalV1, CanonicalGlobalTargetV1, CanonicalSameModuleCallableKeyV1,
    CanonicalSameModuleGlobalTargetV1, SameModuleCallableNamespaceV1,
};

use crate::mir::{ArrayElementWriteKind, Callee, MirFunction, MirInstruction, MirModule, ValueId};

mod row_refs;
pub(crate) use row_refs::{
    PublishedArrayElementWriteRef, PublishedBuiltinPrintCallRef, PublishedFreeFunctionCallRef,
    PublishedStaticMethodCallRef,
};

mod c_transport;
mod c_transport_v2;
mod map_body_index;
mod map_original_demand;
mod map_named_allocations;
mod map_value_domains;
mod map_projection;
mod compiled_entry_contract;
mod lifecycle;
mod physical_abi;
mod physical_program;
mod physical_program_json;

pub(super) use lifecycle::is_lifecycle_instruction;
pub(crate) use lifecycle::PublishedObjectStorageProfileV1;

pub(crate) use c_transport::{
    PublishedCallKindV1, PublishedStaticMethodCFrameV1, PublishedStaticMethodCallCRowV1,
};
pub(crate) use compiled_entry_contract::{
    CompiledEntryCleanupKindV1, CompiledEntryContractV1, CompiledEntryFormalKindV1,
    CompiledEntryRootResultV1,
};
pub(crate) use physical_abi::{PublishedLifecyclePhysicalAbiInputV1, PublishedLifecycleRuntimeRequirementsV1};
pub(crate) use physical_program::PublishedLifecyclePhysicalFunctionRoleV1;
pub(crate) use physical_program::ordinary_callable_key;
pub(crate) use physical_program::ordinary_call_receiver;
pub(crate) use physical_program_json::emit_lifecycle_physical_abi_json;

/// The only route decisions a backend may observe for the selected published
/// call family (static method, builtin print, or free function).  An instance
/// call is explicit but has no selected-C consumer yet, so it is a terminal
/// physical admission state rather than an implicit compatibility fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedStaticMethodRouteV1 {
    /// At least one selected call exists.  Other call families may remain on
    /// their explicit compatibility routes until their own cohort is cut over.
    CanonicalTyped,
    /// No selected typed call exists; an explicit compatibility caller may
    /// consume the module without pretending it is canonical.
    ExplicitCompatibility,
    /// A canonical call family has no lossless selected-C consumer. The whole
    /// module must stop before JSON/C/object work; it must not be silently
    /// reclassified as explicit compatibility.
    UnsupportedBeforeObject,
}

/// Publication/view failures are physical admission failures.  They never
/// trigger a second resolver or a compatibility retry for a selected module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum PublishedMirBackendViewErrorV1 {
    RetainedRootMissing,
    IntrinsicArrayShapeMismatch {
        function: String,
    },
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
    ArrayElementWriteShapeMismatch {
        function: String,
        kind: ArrayElementWriteKind,
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

/// Read-only projection of an already atomically-published module.
///
/// No AST, resolver, registry, JSON, fallback state, or independently-owned
/// semantic table is stored here. A selected call has one typed route; calls
/// from other families remain explicit compatibility data until their cohort
/// is selected. They never alter the selected call's target relation.
#[derive(Debug)]
pub(crate) struct PublishedMirBackendView<'module> {
    module: &'module MirModule,
    retained_root: Option<&'module MirFunction>,
    retained_handoff:
        Option<&'module crate::mir::finalized_root_handoff::FinalizedRootHandoffV1>,
    route: PublishedStaticMethodRouteV1,
    static_method_calls: Vec<PublishedStaticMethodCallRef<'module>>,
    free_function_calls: Vec<PublishedFreeFunctionCallRef<'module>>,
    builtin_print_calls: Vec<PublishedBuiltinPrintCallRef<'module>>,
    array_element_writes: Vec<PublishedArrayElementWriteRef<'module>>,
    intrinsic_arrays: Vec<row_refs::PublishedIntrinsicArrayRef<'module>>,
    has_lifecycle_instructions: bool,
    pub(super) has_non_lifecycle_unsupported: bool,
    pub(super) lifecycle_storage_profile: Option<&'module PublishedObjectStorageProfileV1>,
}

impl<'module> PublishedMirBackendView<'module> {
    pub(crate) fn try_new(
        module: &'module MirModule,
    ) -> Result<Self, PublishedMirBackendViewErrorV1> {
        validate_definition_table(module)?;

        let mut static_method_calls = Vec::new();
        let mut free_function_calls = Vec::new();
        let mut builtin_print_calls = Vec::new();
        let mut array_element_writes = Vec::new();
        let mut intrinsic_arrays = Vec::new();
        let mut has_lifecycle_instructions = false;
        let mut has_non_lifecycle_unsupported = false;
        let mut has_intrinsic_maps = false;
        for (function_name, function) in &module.functions {
            let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
            block_ids.sort();
            for block_id in block_ids {
                let block = function
                    .blocks
                    .get(&block_id)
                    .expect("sorted MIR block id must remain present");
                for (instruction_index, instruction) in block.all_instructions().enumerate() {
                    if matches!(instruction, MirInstruction::Return { .. }) {
                        continue;
                    }
                    if let MirInstruction::NewBox {
                        dst,
                        target: crate::mir::ConstructionTarget::IntrinsicArray,
                        args,
                    } = instruction
                    {
                        if !args.is_empty()
                            || *dst == ValueId::INVALID
                            || function_name.contains('\0')
                        {
                            return Err(
                                PublishedMirBackendViewErrorV1::IntrinsicArrayShapeMismatch {
                                    function: function_name.clone(),
                                },
                            );
                        }
                        intrinsic_arrays.push(row_refs::PublishedIntrinsicArrayRef {
                            function_name,
                            block_id: block_id.as_u32(),
                            instruction_index: u32::try_from(instruction_index).map_err(|_| {
                                PublishedMirBackendViewErrorV1::IntrinsicArrayShapeMismatch {
                                    function: function_name.clone(),
                                }
                            })?,
                            dst: *dst,
                        });
                        continue;
                    }
                    // V2 frame and C validate the exact Map physical input.
                    if matches!(instruction,
                        MirInstruction::MapLiteralEntryWrite { .. }
                        | MirInstruction::NewBox {
                            target: crate::mir::ConstructionTarget::IntrinsicMap, ..
                        })
                    {
                        has_intrinsic_maps = true;
                        continue;
                    }
                    if is_lifecycle_instruction(instruction) {
                        has_lifecycle_instructions = true;
                        continue;
                    }
                    if let MirInstruction::ArrayElementWrite {
                        site_id,
                        dst,
                        kind,
                        receiver,
                        index,
                        value,
                        ..
                    } = instruction
                    {
                        crate::mir::array_element_write::validate_shape(*kind, *index).map_err(
                            |_| PublishedMirBackendViewErrorV1::ArrayElementWriteShapeMismatch {
                                function: function_name.clone(),
                                kind: *kind,
                            },
                        )?;
                        array_element_writes.push(PublishedArrayElementWriteRef {
                            function_name: function_name.as_str(),
                            block_id: block_id.as_u32(),
                            instruction_index: instruction_index as u32,
                            site_id: site_id.0,
                            kind: *kind,
                            dst: *dst,
                            receiver: *receiver,
                            index: *index,
                            value: *value,
                        });
                        continue;
                    }

                    let (dst, func, callee, args, canonical_call) = match instruction {
                        MirInstruction::Call(call) => (
                            call.dst,
                            ValueId::INVALID,
                            Some(&call.callee),
                            call.args.as_slice(),
                            true,
                        ),
                        MirInstruction::LegacyCallV0 {
                            dst,
                            func,
                            callee,
                            args,
                            ..
                        } => (*dst, *func, callee.as_ref(), args.as_slice(), false),
                        _ => continue,
                    };

                    match callee {
                        Some(Callee::Global(target)) => {
                            if let Some(key) = static_method_key(target) {
                                let published_key =
                                    validate_static_call(module, function_name, key, func, args)?;
                                static_method_calls.push(PublishedStaticMethodCallRef {
                                    function_name: function_name.as_str(),
                                    block_id: block_id.as_u32(),
                                    instruction_index: instruction_index as u32,
                                    key: published_key,
                                    args,
                                });
                            } else if let Some(key) = free_function_key(target) {
                                let published_key = validate_free_function_call(
                                    module,
                                    function_name,
                                    key,
                                    func,
                                    args,
                                )?;
                                free_function_calls.push(PublishedFreeFunctionCallRef {
                                    function_name: function_name.as_str(),
                                    block_id: block_id.as_u32(),
                                    instruction_index: instruction_index as u32,
                                    key: published_key,
                                    args,
                                });
                            } else if is_builtin_print_target(target) {
                                validate_builtin_print_call(function_name, dst, func, args)?;
                                builtin_print_calls.push(PublishedBuiltinPrintCallRef {
                                    function_name: function_name.as_str(),
                                    block_id: block_id.as_u32(),
                                    instruction_index: instruction_index as u32,
                                    args,
                                });
                            }
                        }
                        Some(
                            Callee::SameModuleInstance { .. } | Callee::BirthConstructor { .. },
                        ) => {
                            has_non_lifecycle_unsupported = true;
                        }
                        Some(Callee::Value(_)) if canonical_call => {
                            has_non_lifecycle_unsupported = true;
                        }
                        Some(_) | None => {}
                    }
                }
            }
        }

        // Intrinsic Map rows are consumed by the same final lifecycle program
        // and C V4 walker as Invoke/Field rows.  Keep them on that route even
        // when a source shape contains no other lifecycle instruction; sending
        // the module through generic V2 would re-enter the broad backend gate
        // before the selected physical Map consumer can validate it.
        has_lifecycle_instructions |= has_intrinsic_maps;

        if has_non_lifecycle_unsupported || has_lifecycle_instructions {
            return Ok(Self {
                module,
                retained_root: None,
                retained_handoff: None,
                route: PublishedStaticMethodRouteV1::UnsupportedBeforeObject,
                static_method_calls,
                free_function_calls,
                builtin_print_calls,
                array_element_writes,
                intrinsic_arrays,
                has_lifecycle_instructions,
                has_non_lifecycle_unsupported,
                lifecycle_storage_profile: None,
            });
        }
        if static_method_calls.is_empty()
            && free_function_calls.is_empty()
            && builtin_print_calls.is_empty()
            && array_element_writes.is_empty()
            && intrinsic_arrays.is_empty()
            && !has_intrinsic_maps
        {
            return Ok(Self {
                module,
                retained_root: None,
                retained_handoff: None,
                route: PublishedStaticMethodRouteV1::ExplicitCompatibility,
                static_method_calls,
                free_function_calls,
                builtin_print_calls,
                array_element_writes,
                intrinsic_arrays,
                has_lifecycle_instructions,
                has_non_lifecycle_unsupported,
                lifecycle_storage_profile: None,
            });
        }
        Ok(Self {
            module,
            retained_root: None,
            retained_handoff: None,
            route: PublishedStaticMethodRouteV1::CanonicalTyped,
            static_method_calls,
            free_function_calls,
            builtin_print_calls,
            array_element_writes,
            intrinsic_arrays,
            has_lifecycle_instructions,
            has_non_lifecycle_unsupported,
            lifecycle_storage_profile: None,
        })
    }

    pub(crate) const fn route(&self) -> PublishedStaticMethodRouteV1 {
        if self.lifecycle_storage_profile.is_some() {
            PublishedStaticMethodRouteV1::CanonicalTyped
        } else {
            self.route
        }
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

    pub(crate) fn array_element_writes(&self) -> &[PublishedArrayElementWriteRef<'module>] {
        &self.array_element_writes
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
        SameModuleCallableNamespaceV1::InstanceBoxMethod
        | SameModuleCallableNamespaceV1::BirthConstructor => key.arity() as usize + 1,
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
mod script_physical_input_tests;

pub(crate) use c_transport_v2::{FrameHeader as PublishedStaticFrameHeaderV2, PublishedStaticMethodCFrameV2};
pub(crate) use map_named_allocations::NamedAllocationConsumer;
