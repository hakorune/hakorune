//! Borrow-only view of the canonical callable relation after module publish.
//!
//! This module deliberately does not resolve names, inspect source, or repair
//! legacy call operands.  It validates the relation already published by
//! `MirModule` and exposes only references for a backend consumer.

use hakorune_mir_defs::{
    CanonicalBuiltinGlobalV1, CanonicalGlobalTargetV1, CanonicalSameModuleCallableKeyV1,
    CanonicalSameModuleGlobalTargetV1, SameModuleCallableNamespaceV1,
};

use crate::mir::{ArrayElementWriteKind, Callee, MirFunction, MirInstruction, MirModule, ValueId};

#[path = "../../function/published_backend_view_c_transport.rs"]
mod c_transport;
mod compiled_entry_contract;
mod lifecycle;
#[path = "../../function/published_backend_view_lifecycle_c_transport.rs"]
mod lifecycle_c_transport;
#[path = "../../function/published_backend_view_lifecycle_schema.rs"]
mod lifecycle_schema;
mod physical_abi;
mod physical_program;
mod physical_program_json;

use lifecycle::PublishedLifecycleInstructionRef;

pub(crate) use c_transport::{
    PublishedCallKindV1, PublishedStaticMethodCFrameV1, PublishedStaticMethodCallCRowV1,
};
pub(crate) use compiled_entry_contract::{CompiledEntryFormalKindV1, CompiledEntryRootResultV1};
pub(crate) use lifecycle_c_transport::{
    PublishedLifecycleBodySiteCRowV1, PublishedLifecycleCFrameHeaderV2, PublishedLifecycleCFrameV2,
    PublishedObjectStorageProfileV1,
};
pub(crate) use physical_program::PublishedLifecyclePhysicalFunctionRoleV1;

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

/// A canonical ArrayElementWrite borrowed from the atomically-published MIR.
/// The operation kind, receiver, index, and value are already decided by the
/// ArrayElementWrite owner; the backend only projects these operands.
#[derive(Debug, Clone, Copy)]
pub(crate) struct PublishedArrayElementWriteRef<'module> {
    function_name: &'module str,
    block_id: u32,
    instruction_index: u32,
    site_id: u32,
    kind: ArrayElementWriteKind,
    dst: Option<ValueId>,
    receiver: ValueId,
    index: Option<ValueId>,
    value: ValueId,
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
    retained_birth_keys: Option<Box<[CanonicalSameModuleCallableKeyV1]>>,
    retained_birth_abi:
        Option<Box<[crate::mir::normal_callable_semantic_package::BirthAbiHandoffV1]>>,
    retained_root_source:
        Option<crate::mir::normal_callable_semantic_package::FinalizedRootSourceHandoffV1>,
    retained_root_result:
        Option<crate::mir::normal_callable_semantic_package::FinalizedRootResultAbiV1>,
    route: PublishedStaticMethodRouteV1,
    static_method_calls: Vec<PublishedStaticMethodCallRef<'module>>,
    free_function_calls: Vec<PublishedFreeFunctionCallRef<'module>>,
    builtin_print_calls: Vec<PublishedBuiltinPrintCallRef<'module>>,
    array_element_writes: Vec<PublishedArrayElementWriteRef<'module>>,
    lifecycle_instructions: Vec<PublishedLifecycleInstructionRef<'module>>,
    return_instructions: Vec<PublishedLifecycleInstructionRef<'module>>,
    has_non_lifecycle_unsupported: bool,
    lifecycle_storage_profile: Option<PublishedObjectStorageProfileV1>,
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
        let mut lifecycle_instructions = Vec::new();
        let mut return_instructions = Vec::new();
        let mut has_non_lifecycle_unsupported = false;
        for (function_name, function) in &module.functions {
            let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
            block_ids.sort();
            for block_id in block_ids {
                let block = function
                    .blocks
                    .get(&block_id)
                    .expect("sorted MIR block id must remain present");
                for (instruction_index, instruction) in block.all_instructions().enumerate() {
                    if let Some(row) = PublishedLifecycleInstructionRef::return_instruction(
                        function_name,
                        block_id.as_u32(),
                        instruction_index as u32,
                        instruction,
                    ) {
                        return_instructions.push(row);
                        continue;
                    }
                    if let Some(row) = PublishedLifecycleInstructionRef::from_instruction(
                        function_name,
                        block_id.as_u32(),
                        instruction_index as u32,
                        instruction,
                    ) {
                        lifecycle_instructions.push(row);
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

        if has_non_lifecycle_unsupported || !lifecycle_instructions.is_empty() {
            return Ok(Self {
                module,
                retained_root: None,
                retained_birth_keys: None,
                retained_birth_abi: None,
                retained_root_source: None,
                retained_root_result: None,
                route: PublishedStaticMethodRouteV1::UnsupportedBeforeObject,
                static_method_calls,
                free_function_calls,
                builtin_print_calls,
                array_element_writes,
                lifecycle_instructions,
                return_instructions,
                has_non_lifecycle_unsupported,
                lifecycle_storage_profile: None,
            });
        }
        if static_method_calls.is_empty()
            && free_function_calls.is_empty()
            && builtin_print_calls.is_empty()
            && array_element_writes.is_empty()
        {
            return Ok(Self {
                module,
                retained_root: None,
                retained_birth_keys: None,
                retained_birth_abi: None,
                retained_root_source: None,
                retained_root_result: None,
                route: PublishedStaticMethodRouteV1::ExplicitCompatibility,
                static_method_calls,
                free_function_calls,
                builtin_print_calls,
                array_element_writes,
                lifecycle_instructions,
                return_instructions,
                has_non_lifecycle_unsupported,
                lifecycle_storage_profile: None,
            });
        }
        Ok(Self {
            module,
            retained_root: None,
            retained_birth_keys: None,
            retained_birth_abi: None,
            retained_root_source: None,
            retained_root_result: None,
            route: PublishedStaticMethodRouteV1::CanonicalTyped,
            static_method_calls,
            free_function_calls,
            builtin_print_calls,
            array_element_writes,
            lifecycle_instructions,
            return_instructions,
            has_non_lifecycle_unsupported,
            lifecycle_storage_profile: None,
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
