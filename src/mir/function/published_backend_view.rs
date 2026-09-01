//! Borrow-only view of the canonical callable relation after module publish.
//!
//! This module deliberately does not resolve names, inspect source, or repair
//! legacy call operands.  It validates the relation already published by
//! `MirModule` and exposes only references for a backend consumer.

use std::ffi::CString;
use std::os::raw::c_char;

use hakorune_mir_defs::{
    CanonicalGlobalTargetV1, CanonicalSameModuleCallableKeyV1,
    CanonicalSameModuleGlobalTargetV1, SameModuleCallableNamespaceV1,
};

use crate::mir::{Callee, MirFunction, MirInstruction, MirModule, ValueId};

/// The only route decisions a backend may observe for the selected family.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum PublishedStaticMethodRouteV1 {
    /// At least one selected call exists.  Other call families may remain on
    /// their explicit compatibility routes until their own cohort is cut over.
    CanonicalTyped,
    /// No selected StaticBoxMethod call exists; an explicit compatibility
    /// caller may consume the module without pretending it is canonical.
    ExplicitCompatibility,
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

/// Read-only projection of an already atomically-published module.
///
/// No AST, resolver, registry, JSON, fallback state, or independently-owned
/// semantic table is stored here.  A selected call has one typed route; calls
/// from other families remain explicit compatibility data until their cohort
/// is selected.  They never alter the selected call's target relation.
#[derive(Debug)]
pub(crate) struct PublishedMirBackendView<'module> {
    module: &'module MirModule,
    route: PublishedStaticMethodRouteV1,
    static_method_calls: Vec<PublishedStaticMethodCallRef<'module>>,
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
        let mut function_names = Vec::with_capacity(view.static_method_calls.len());
        let mut target_symbols = Vec::with_capacity(view.static_method_calls.len());
        let mut rows = Vec::with_capacity(view.static_method_calls.len());
        for call in &view.static_method_calls {
            let symbol = call.key.mir_symbol_projection();
            let function_name = CString::new(call.function_name).map_err(|_| {
                PublishedMirBackendViewErrorV1::DefinitionSymbolMismatch {
                    key: call.key.clone(),
                    symbol: call.function_name.to_owned(),
                }
            })?;
            let target_symbol = CString::new(symbol.clone()).map_err(|_| {
                PublishedMirBackendViewErrorV1::DefinitionSymbolMismatch {
                    key: call.key.clone(),
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
                block_id: call.block_id,
                instruction_index: call.instruction_index,
                target_symbol: target_symbol_ptr,
                arity: call.key.arity(),
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
                                let published_key = validate_static_call(
                                    module,
                                    function_name,
                                    key,
                                    *func,
                                    args,
                                )?;
                                static_method_calls.push(PublishedStaticMethodCallRef {
                                    function_name: function_name.as_str(),
                                    block_id: block_id.as_u32(),
                                    instruction_index: instruction_index as u32,
                                    key: published_key,
                                    args: args.as_slice(),
                                });
                            }
                        }
                        Some(_) | None => {}
                    }
                }
            }
        }

        if static_method_calls.is_empty() {
            return Ok(Self {
                module,
                route: PublishedStaticMethodRouteV1::ExplicitCompatibility,
                static_method_calls,
            });
        }
        Ok(Self {
            module,
            route: PublishedStaticMethodRouteV1::CanonicalTyped,
            static_method_calls,
        })
    }

    pub(crate) const fn route(&self) -> PublishedStaticMethodRouteV1 {
        self.route
    }

    pub(crate) fn static_method_calls(&self) -> &[PublishedStaticMethodCallRef<'module>] {
        &self.static_method_calls
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

fn validate_definition_table(
    module: &MirModule,
) -> Result<(), PublishedMirBackendViewErrorV1> {
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
        return Err(PublishedMirBackendViewErrorV1::StaticCallDefinitionMissing {
            function: function_name.to_owned(),
            key,
        });
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
        return Err(PublishedMirBackendViewErrorV1::StaticCallDefinitionMissing {
            function: function_name.to_owned(),
            key,
        });
    };
    if function.signature.return_type != crate::mir::MirType::Integer {
        return Err(PublishedMirBackendViewErrorV1::StaticMethodRequiresIntegerReturn {
            function: function_name.to_owned(),
            key,
        });
    }
    debug_assert_eq!(symbol, &key.mir_symbol_projection());
    Ok(published_key)
}

fn expected_physical_arity(key: &CanonicalSameModuleCallableKeyV1) -> usize {
    match key.namespace() {
        SameModuleCallableNamespaceV1::StaticBoxMethod => key.arity() as usize,
        SameModuleCallableNamespaceV1::InstanceBoxMethod => key.arity() as usize + 1,
    }
}

fn static_method_key(target: &CanonicalGlobalTargetV1) -> Option<CanonicalSameModuleCallableKeyV1> {
    let CanonicalGlobalTargetV1::SameModule(
        CanonicalSameModuleGlobalTargetV1::StaticBoxMethod {
            owner,
            method,
            arity,
        },
    ) = target
    else {
        return None;
    };
    Some(CanonicalSameModuleCallableKeyV1::static_box_method(
        owner, method, *arity,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType};

    fn static_key() -> CanonicalSameModuleCallableKeyV1 {
        CanonicalSameModuleCallableKeyV1::test_static_box_method("MathBox", "sum", 2)
    }

    fn static_function(key: &CanonicalSameModuleCallableKeyV1, func: ValueId) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: key.mir_symbol_projection(),
                params: vec![MirType::Integer; key.arity() as usize],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let target = key
            .canonical_global_target_v1()
            .expect("static key must project to global target");
        function
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("entry block")
            .add_instruction(MirInstruction::Call {
                dst: Some(ValueId::new(10)),
                func,
                callee: Some(Callee::Global(target)),
                args: vec![ValueId::new(1), ValueId::new(2)],
                effects: EffectMask::PURE,
            });
        function
    }

    #[test]
    fn published_static_method_is_typed_and_definition_backed() {
        let key = static_key();
        let mut module = MirModule::new("typed".to_owned());
        module
            .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
            .expect("publish relation");

        let view = PublishedMirBackendView::try_new(&module).expect("typed view");
        assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
        assert_eq!(view.static_method_calls().len(), 1);
        assert_eq!(view.static_method_calls()[0].key(), &key);
        assert!(view.definition(&key).is_some());
    }

    #[test]
    fn c_frame_keeps_exact_site_and_one_way_symbol_projection() {
        let key = static_key();
        let mut module = MirModule::new("typed-c".to_owned());
        module
            .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
            .expect("publish relation");

        let view = PublishedMirBackendView::try_new(&module).expect("typed view");
        let frame = PublishedStaticMethodCFrameV1::from_view(&view).expect("C frame");
        assert_eq!(frame.len(), 1);
        let row = frame.row(0);
        assert_eq!(row.block_id, 0);
        assert_eq!(row.instruction_index, 0);
        assert_eq!(row.arity, 2);
        assert!(!row.function_name.is_null());
        assert!(!row.target_symbol.is_null());
        assert!(!frame.as_ptr().is_null());
    }

    #[test]
    fn selected_static_method_keeps_other_families_on_compatibility_routes() {
        let key = static_key();
        let mut module = MirModule::new("mixed".to_owned());
        module
            .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::INVALID))
            .expect("publish relation");

        let mut legacy = MirFunction::new(
            FunctionSignature {
                name: "legacy/0".to_owned(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        legacy
            .blocks
            .get_mut(&BasicBlockId::new(0))
            .expect("legacy entry block")
            .add_instruction(MirInstruction::Call {
                dst: None,
                func: ValueId::new(1),
                callee: None,
                args: Vec::new(),
                effects: EffectMask::PURE,
            });
        module.add_function(legacy);

        let view = PublishedMirBackendView::try_new(&module).expect("mixed typed view");
        assert_eq!(view.route(), PublishedStaticMethodRouteV1::CanonicalTyped);
        assert_eq!(view.static_method_calls().len(), 1);
    }

    #[test]
    fn selected_static_method_rejects_legacy_function_carrier() {
        let key = static_key();
        let mut module = MirModule::new("legacy-carrier".to_owned());
        module
            .add_cataloged_box_method(key.clone(), static_function(&key, ValueId::new(9)))
            .expect("publish relation");

        let error = PublishedMirBackendView::try_new(&module).unwrap_err();
        assert!(matches!(
            error,
            PublishedMirBackendViewErrorV1::StaticCallUsesLegacyFunctionCarrier { .. }
        ));
    }

    #[test]
    fn module_without_selected_static_method_is_explicit_compatibility() {
        let mut module = MirModule::new("compat".to_owned());
        module.add_function(MirFunction::new(
            FunctionSignature {
                name: "legacy/0".to_owned(),
                params: Vec::new(),
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        ));

        let view = PublishedMirBackendView::try_new(&module).expect("compatibility view");
        assert_eq!(
            view.route(),
            PublishedStaticMethodRouteV1::ExplicitCompatibility
        );
        assert!(view.static_method_calls().is_empty());
    }
}
