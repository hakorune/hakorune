/*!
 * MIR-owned route plans for extern call policy.
 *
 * Extern calls are not CoreMethodContract rows. This module keeps the narrow
 * extern-call backend contract in MIR metadata so ny-llvmc can consume an
 * explicit plan instead of classifying raw `env.*` strings in the C shim.
 */

use super::{BasicBlockId, Callee, MirFunction, MirInstruction, MirModule, ValueId};
use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternCallRouteKind {
    EnvGet,
    EnvSet,
    AnyHandleLive,
    ArraySlotAppendAny,
    ArraySlotLenI64,
    HakoMemAlloc,
    HakoMemFree,
    HostBridgeExternInvoke,
    Stage1EmitProgramJson,
    Stage1EmitMirFromSource,
    Stage1EmitMirFromProgramJson,
}

impl ExternCallRouteKind {
    pub fn route_id(self) -> &'static str {
        match self {
            Self::EnvGet => "extern.env.get",
            Self::EnvSet => "extern.env.set",
            Self::AnyHandleLive => "extern.any.handle_live",
            Self::ArraySlotAppendAny => "extern.array.slot_append_any",
            Self::ArraySlotLenI64 => "extern.array.slot_len_i64",
            Self::HakoMemAlloc => "extern.hako_mem.alloc",
            Self::HakoMemFree => "extern.hako_mem.free",
            Self::HostBridgeExternInvoke => "extern.hostbridge.extern_invoke",
            Self::Stage1EmitProgramJson => "extern.stage1.emit_program_json_v0",
            Self::Stage1EmitMirFromSource => "extern.stage1.emit_mir_from_source_v0",
            Self::Stage1EmitMirFromProgramJson => "extern.stage1.emit_mir_from_program_json_v0",
        }
    }

    pub fn core_op(self) -> &'static str {
        match self {
            Self::EnvGet => "EnvGet",
            Self::EnvSet => "EnvSet",
            Self::AnyHandleLive => "AnyHandleLive",
            Self::ArraySlotAppendAny => "ArraySlotAppendAny",
            Self::ArraySlotLenI64 => "ArraySlotLenI64",
            Self::HakoMemAlloc => "HakoMemAlloc",
            Self::HakoMemFree => "HakoMemFree",
            Self::HostBridgeExternInvoke => "HostBridgeExternInvoke",
            Self::Stage1EmitProgramJson => "Stage1EmitProgramJson",
            Self::Stage1EmitMirFromSource => "Stage1EmitMirFromSource",
            Self::Stage1EmitMirFromProgramJson => "Stage1EmitMirFromProgramJson",
        }
    }

    pub fn symbol(self) -> &'static str {
        match self {
            Self::EnvGet => "nyash.env.get",
            Self::EnvSet => "nyash.env.set",
            Self::AnyHandleLive => "nyash.any.handle_live_h",
            Self::ArraySlotAppendAny => "nyash.array.slot_append_hh",
            Self::ArraySlotLenI64 => "nyash.array.slot_len_h",
            Self::HakoMemAlloc => "hako_mem_alloc",
            Self::HakoMemFree => "hako_mem_free",
            Self::HostBridgeExternInvoke => "nyash.hostbridge.extern_invoke",
            Self::Stage1EmitProgramJson => "nyash.stage1.emit_program_json_v0_h",
            Self::Stage1EmitMirFromSource => "nyash.stage1.emit_mir_from_source_v0_h",
            Self::Stage1EmitMirFromProgramJson => "nyash.stage1.emit_mir_from_program_json_v0_h",
        }
    }

    pub fn lowering_tier(self) -> LoweringPlanTier {
        LoweringPlanTier::ColdRuntime
    }

    pub fn tier(self) -> &'static str {
        self.lowering_tier().as_json_name()
    }

    pub fn lowering_emit_kind(self) -> LoweringPlanEmitKind {
        LoweringPlanEmitKind::RuntimeCall
    }

    pub fn emit_kind(self) -> &'static str {
        self.lowering_emit_kind().as_json_name()
    }

    pub fn proof(self) -> &'static str {
        match self {
            Self::EnvGet => "extern_registry",
            Self::EnvSet => "extern_registry",
            Self::AnyHandleLive => "extern_registry",
            Self::ArraySlotAppendAny => "extern_registry",
            Self::ArraySlotLenI64 => "extern_registry",
            Self::HakoMemAlloc => "extern_registry",
            Self::HakoMemFree => "extern_registry",
            Self::HostBridgeExternInvoke => "extern_registry",
            Self::Stage1EmitProgramJson => "extern_registry",
            Self::Stage1EmitMirFromSource => "extern_registry",
            Self::Stage1EmitMirFromProgramJson => "extern_registry",
        }
    }

    pub fn return_shape(self) -> &'static str {
        match self {
            Self::EnvGet => "string_handle_or_null",
            Self::EnvSet => "scalar_i64",
            Self::AnyHandleLive => "scalar_i64",
            Self::ArraySlotAppendAny => "scalar_i64",
            Self::ArraySlotLenI64 => "scalar_i64",
            Self::HakoMemAlloc => "native_ptr_nullable",
            Self::HakoMemFree => "void_sentinel_i64_zero",
            Self::HostBridgeExternInvoke => "string_handle_or_null",
            Self::Stage1EmitProgramJson => "string_handle",
            Self::Stage1EmitMirFromSource => "string_handle",
            Self::Stage1EmitMirFromProgramJson => "string_handle",
        }
    }

    pub fn value_demand(self) -> &'static str {
        match self {
            Self::EnvGet => "runtime_i64_or_handle",
            Self::EnvSet => "runtime_i64",
            Self::AnyHandleLive => "runtime_i64",
            Self::ArraySlotAppendAny => "runtime_i64",
            Self::ArraySlotLenI64 => "runtime_i64",
            Self::HakoMemAlloc => "native_ptr_nullable",
            Self::HakoMemFree => "scalar_i64",
            Self::HostBridgeExternInvoke => "runtime_i64_or_handle",
            Self::Stage1EmitProgramJson => "runtime_i64_or_handle",
            Self::Stage1EmitMirFromSource => "runtime_i64_or_handle",
            Self::Stage1EmitMirFromProgramJson => "runtime_i64_or_handle",
        }
    }

    pub fn effect_tags(self) -> &'static [&'static str] {
        match self {
            Self::EnvGet => &["read.env"],
            Self::EnvSet => &["write.env"],
            Self::AnyHandleLive => &["read.any.handle_live"],
            Self::ArraySlotAppendAny => &["array.slot_append"],
            Self::ArraySlotLenI64 => &["array.slot_len"],
            Self::HakoMemAlloc => &["hako.mem.alloc"],
            Self::HakoMemFree => &["hako.mem.free"],
            Self::HostBridgeExternInvoke => &["hostbridge.extern"],
            Self::Stage1EmitProgramJson => &["stage1.emit_program_json"],
            Self::Stage1EmitMirFromSource => &["stage1.emit_mir_from_source"],
            Self::Stage1EmitMirFromProgramJson => &["stage1.emit_mir_from_program_json"],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternCallRouteSite {
    block: BasicBlockId,
    instruction_index: usize,
}

impl ExternCallRouteSite {
    pub fn new(block: BasicBlockId, instruction_index: usize) -> Self {
        Self {
            block,
            instruction_index,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternCallRoute {
    site: ExternCallRouteSite,
    kind: ExternCallRouteKind,
    source_symbol: String,
    key_value: ValueId,
    value_value: Option<ValueId>,
    result_value: ValueId,
}

impl ExternCallRoute {
    pub fn new(
        site: ExternCallRouteSite,
        kind: ExternCallRouteKind,
        source_symbol: impl Into<String>,
        key_value: ValueId,
        value_value: Option<ValueId>,
        result_value: ValueId,
    ) -> Self {
        Self {
            site,
            kind,
            source_symbol: source_symbol.into(),
            key_value,
            value_value,
            result_value,
        }
    }

    pub fn block(&self) -> BasicBlockId {
        self.site.block
    }

    pub fn instruction_index(&self) -> usize {
        self.site.instruction_index
    }

    pub fn route_id(&self) -> &'static str {
        self.kind.route_id()
    }

    pub fn core_op(&self) -> &'static str {
        self.kind.core_op()
    }

    pub fn symbol(&self) -> &'static str {
        self.kind.symbol()
    }

    pub fn tier(&self) -> &'static str {
        self.kind.tier()
    }

    pub fn lowering_tier(&self) -> LoweringPlanTier {
        self.kind.lowering_tier()
    }

    pub fn emit_kind(&self) -> &'static str {
        self.kind.emit_kind()
    }

    pub fn lowering_emit_kind(&self) -> LoweringPlanEmitKind {
        self.kind.lowering_emit_kind()
    }

    pub fn proof(&self) -> &'static str {
        self.kind.proof()
    }

    pub fn source_symbol(&self) -> &str {
        &self.source_symbol
    }

    pub fn key_value(&self) -> ValueId {
        self.key_value
    }

    pub fn value_value(&self) -> Option<ValueId> {
        self.value_value
    }

    pub fn result_value(&self) -> ValueId {
        self.result_value
    }

    pub fn arity(&self) -> usize {
        match self.kind {
            ExternCallRouteKind::EnvGet => 1,
            ExternCallRouteKind::EnvSet => 2,
            ExternCallRouteKind::AnyHandleLive => 1,
            ExternCallRouteKind::ArraySlotAppendAny => 2,
            ExternCallRouteKind::ArraySlotLenI64 => 1,
            ExternCallRouteKind::HakoMemAlloc => 1,
            ExternCallRouteKind::HakoMemFree => 1,
            ExternCallRouteKind::HostBridgeExternInvoke => 3,
            ExternCallRouteKind::Stage1EmitProgramJson => 1,
            ExternCallRouteKind::Stage1EmitMirFromSource => 1,
            ExternCallRouteKind::Stage1EmitMirFromProgramJson => 1,
        }
    }

    pub fn return_shape(&self) -> &'static str {
        self.kind.return_shape()
    }

    pub fn value_demand(&self) -> &'static str {
        self.kind.value_demand()
    }

    pub fn effect_tags(&self) -> &'static [&'static str] {
        self.kind.effect_tags()
    }
}

pub fn normalize_extern_symbol(name: &str) -> &str {
    name.strip_suffix("/1")
        .or_else(|| name.strip_suffix("/2"))
        .or_else(|| name.strip_suffix("/3"))
        .unwrap_or(name)
}

pub fn classify_extern_call_route(name: &str, argc: usize) -> Option<ExternCallRouteKind> {
    match (normalize_extern_symbol(name), argc) {
        ("env.get", 1) | ("nyash.env.get", 1) => Some(ExternCallRouteKind::EnvGet),
        ("env.set", 2) | ("nyash.env.set", 2) => Some(ExternCallRouteKind::EnvSet),
        ("nyash.any.handle_live_h", 1) => Some(ExternCallRouteKind::AnyHandleLive),
        ("nyash.array.slot_append_hh", 2) => Some(ExternCallRouteKind::ArraySlotAppendAny),
        ("nyash.array.slot_len_h", 1) => Some(ExternCallRouteKind::ArraySlotLenI64),
        ("hako_mem_alloc", 1) => Some(ExternCallRouteKind::HakoMemAlloc),
        ("hako_mem_free", 1) => Some(ExternCallRouteKind::HakoMemFree),
        ("hostbridge.extern_invoke", 3) => Some(ExternCallRouteKind::HostBridgeExternInvoke),
        ("nyash.stage1.emit_program_json_v0_h", 1) => {
            Some(ExternCallRouteKind::Stage1EmitProgramJson)
        }
        ("nyash.stage1.emit_mir_from_source_v0_h", 1) => {
            Some(ExternCallRouteKind::Stage1EmitMirFromSource)
        }
        ("nyash.stage1.emit_mir_from_program_json_v0_h", 1) => {
            Some(ExternCallRouteKind::Stage1EmitMirFromProgramJson)
        }
        _ => None,
    }
}

pub fn is_hostbridge_extern_invoke_symbol(name: &str, argc: usize) -> bool {
    classify_extern_call_route(name, argc) == Some(ExternCallRouteKind::HostBridgeExternInvoke)
}

pub fn refresh_module_extern_call_routes(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_extern_call_routes(function);
    }
}

pub fn refresh_function_extern_call_routes(function: &mut MirFunction) {
    let mut routes = Vec::new();
    let mut block_ids = function.blocks.keys().copied().collect::<Vec<_>>();
    block_ids.sort_by_key(|id| id.as_u32());

    for block_id in block_ids {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, instruction) in block.instructions.iter().enumerate() {
            let MirInstruction::Call {
                dst: Some(dst),
                callee: Some(callee),
                args,
                ..
            } = instruction
            else {
                continue;
            };
            let name = match callee {
                Callee::Extern(name) => name,
                Callee::Global(name) if is_hostbridge_extern_invoke_symbol(name, args.len()) => {
                    name
                }
                _ => continue,
            };
            let Some(kind) = classify_extern_call_route(name, args.len()) else {
                continue;
            };
            let Some(key_value) = args.first().copied() else {
                continue;
            };
            let value_value = match kind {
                ExternCallRouteKind::EnvGet => None,
                ExternCallRouteKind::EnvSet => args.get(1).copied(),
                ExternCallRouteKind::AnyHandleLive => None,
                ExternCallRouteKind::ArraySlotAppendAny => args.get(1).copied(),
                ExternCallRouteKind::ArraySlotLenI64 => None,
                ExternCallRouteKind::HakoMemAlloc => None,
                ExternCallRouteKind::HakoMemFree => None,
                ExternCallRouteKind::HostBridgeExternInvoke => args.get(2).copied(),
                ExternCallRouteKind::Stage1EmitProgramJson => None,
                ExternCallRouteKind::Stage1EmitMirFromSource => None,
                ExternCallRouteKind::Stage1EmitMirFromProgramJson => None,
            };
            routes.push(ExternCallRoute::new(
                ExternCallRouteSite::new(block_id, instruction_index),
                kind,
                name,
                key_value,
                value_value,
                *dst,
            ));
        }
    }

    function.metadata.extern_call_routes = routes;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlock, ConstValue, EffectMask, FunctionSignature, MirType, ValueId};

    fn make_function_with_call(
        callee: &str,
        args: Vec<ValueId>,
        dst: Option<ValueId>,
    ) -> MirFunction {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Integer,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.instructions.push(MirInstruction::Const {
            dst: ValueId::new(1),
            value: ConstValue::String("STAGE1_SOURCE_TEXT".to_string()),
        });
        block.instructions.push(MirInstruction::Call {
            dst,
            func: ValueId::INVALID,
            callee: Some(Callee::Extern(callee.to_string())),
            args,
            effects: EffectMask::PURE,
        });
        function.blocks.insert(BasicBlockId::new(0), block);
        function
    }

    #[test]
    fn refresh_function_extern_call_routes_records_env_get_plan_source() {
        let mut function =
            make_function_with_call("env.get/1", vec![ValueId::new(1)], Some(ValueId::new(2)));

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.env.get");
        assert_eq!(route.core_op(), "EnvGet");
        assert_eq!(route.symbol(), "nyash.env.get");
        assert_eq!(route.lowering_tier(), LoweringPlanTier::ColdRuntime);
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(
            route.lowering_emit_kind(),
            LoweringPlanEmitKind::RuntimeCall
        );
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "env.get/1");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "string_handle_or_null");
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.effect_tags(), &["read.env"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_env_set_plan_source() {
        let mut function = make_function_with_call(
            "env.set/2",
            vec![ValueId::new(1), ValueId::new(2)],
            Some(ValueId::new(3)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.env.set");
        assert_eq!(route.core_op(), "EnvSet");
        assert_eq!(route.symbol(), "nyash.env.set");
        assert_eq!(route.lowering_tier(), LoweringPlanTier::ColdRuntime);
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(
            route.lowering_emit_kind(),
            LoweringPlanEmitKind::RuntimeCall
        );
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "env.set/2");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), Some(ValueId::new(2)));
        assert_eq!(route.result_value(), ValueId::new(3));
        assert_eq!(route.arity(), 2);
        assert_eq!(route.return_shape(), "scalar_i64");
        assert_eq!(route.value_demand(), "runtime_i64");
        assert_eq!(route.effect_tags(), &["write.env"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_any_handle_live_route() {
        let mut function = make_function_with_call(
            "nyash.any.handle_live_h",
            vec![ValueId::new(1)],
            Some(ValueId::new(2)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.any.handle_live");
        assert_eq!(route.core_op(), "AnyHandleLive");
        assert_eq!(route.symbol(), "nyash.any.handle_live_h");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "nyash.any.handle_live_h");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "scalar_i64");
        assert_eq!(route.value_demand(), "runtime_i64");
        assert_eq!(route.effect_tags(), &["read.any.handle_live"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_array_slot_append_route() {
        let mut function = make_function_with_call(
            "nyash.array.slot_append_hh",
            vec![ValueId::new(1), ValueId::new(2)],
            Some(ValueId::new(3)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.array.slot_append_any");
        assert_eq!(route.core_op(), "ArraySlotAppendAny");
        assert_eq!(route.symbol(), "nyash.array.slot_append_hh");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "nyash.array.slot_append_hh");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), Some(ValueId::new(2)));
        assert_eq!(route.result_value(), ValueId::new(3));
        assert_eq!(route.arity(), 2);
        assert_eq!(route.return_shape(), "scalar_i64");
        assert_eq!(route.value_demand(), "runtime_i64");
        assert_eq!(route.effect_tags(), &["array.slot_append"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_array_slot_len_route() {
        let mut function = make_function_with_call(
            "nyash.array.slot_len_h",
            vec![ValueId::new(1)],
            Some(ValueId::new(2)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.array.slot_len_i64");
        assert_eq!(route.core_op(), "ArraySlotLenI64");
        assert_eq!(route.symbol(), "nyash.array.slot_len_h");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "nyash.array.slot_len_h");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "scalar_i64");
        assert_eq!(route.value_demand(), "runtime_i64");
        assert_eq!(route.effect_tags(), &["array.slot_len"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_hako_mem_alloc_route() {
        let mut function = make_function_with_call(
            "hako_mem_alloc",
            vec![ValueId::new(1)],
            Some(ValueId::new(2)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.hako_mem.alloc");
        assert_eq!(route.core_op(), "HakoMemAlloc");
        assert_eq!(route.symbol(), "hako_mem_alloc");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "hako_mem_alloc");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "native_ptr_nullable");
        assert_eq!(route.value_demand(), "native_ptr_nullable");
        assert_eq!(route.effect_tags(), &["hako.mem.alloc"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_hako_mem_free_route() {
        let mut function = make_function_with_call(
            "hako_mem_free/1",
            vec![ValueId::new(1)],
            Some(ValueId::new(2)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.hako_mem.free");
        assert_eq!(route.core_op(), "HakoMemFree");
        assert_eq!(route.symbol(), "hako_mem_free");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "hako_mem_free/1");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "void_sentinel_i64_zero");
        assert_eq!(route.value_demand(), "scalar_i64");
        assert_eq!(route.effect_tags(), &["hako.mem.free"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_hostbridge_extern_invoke_global_source() {
        let mut function = MirFunction::new(
            FunctionSignature {
                name: "main".to_string(),
                params: vec![],
                return_type: MirType::Unknown,
                effects: EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        let mut block = BasicBlock::new(BasicBlockId::new(0));
        block.instructions.push(MirInstruction::Call {
            dst: Some(ValueId::new(10)),
            func: ValueId::INVALID,
            callee: Some(Callee::Global("hostbridge.extern_invoke/3".to_string())),
            args: vec![ValueId::new(1), ValueId::new(2), ValueId::new(3)],
            effects: EffectMask::IO,
        });
        function.blocks.insert(BasicBlockId::new(0), block);

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.hostbridge.extern_invoke");
        assert_eq!(route.core_op(), "HostBridgeExternInvoke");
        assert_eq!(route.symbol(), "nyash.hostbridge.extern_invoke");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "hostbridge.extern_invoke/3");
        assert_eq!(route.key_value(), ValueId::new(1));
        assert_eq!(route.value_value(), Some(ValueId::new(3)));
        assert_eq!(route.result_value(), ValueId::new(10));
        assert_eq!(route.arity(), 3);
        assert_eq!(route.return_shape(), "string_handle_or_null");
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.effect_tags(), &["hostbridge.extern"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_stage1_emit_program_json_extern_route() {
        let mut function = make_function_with_call(
            "nyash.stage1.emit_program_json_v0_h",
            vec![ValueId::new(0)],
            Some(ValueId::new(2)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.stage1.emit_program_json_v0");
        assert_eq!(route.core_op(), "Stage1EmitProgramJson");
        assert_eq!(route.symbol(), "nyash.stage1.emit_program_json_v0_h");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(route.source_symbol(), "nyash.stage1.emit_program_json_v0_h");
        assert_eq!(route.key_value(), ValueId::new(0));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "string_handle");
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.effect_tags(), &["stage1.emit_program_json"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_stage1_emit_mir_from_source_extern_route() {
        let mut function = make_function_with_call(
            "nyash.stage1.emit_mir_from_source_v0_h",
            vec![ValueId::new(0)],
            Some(ValueId::new(2)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(route.route_id(), "extern.stage1.emit_mir_from_source_v0");
        assert_eq!(route.core_op(), "Stage1EmitMirFromSource");
        assert_eq!(route.symbol(), "nyash.stage1.emit_mir_from_source_v0_h");
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(
            route.source_symbol(),
            "nyash.stage1.emit_mir_from_source_v0_h"
        );
        assert_eq!(route.key_value(), ValueId::new(0));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "string_handle");
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.effect_tags(), &["stage1.emit_mir_from_source"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_records_stage1_emit_mir_from_program_json_extern_route()
    {
        let mut function = make_function_with_call(
            "nyash.stage1.emit_mir_from_program_json_v0_h",
            vec![ValueId::new(0)],
            Some(ValueId::new(2)),
        );

        refresh_function_extern_call_routes(&mut function);

        assert_eq!(function.metadata.extern_call_routes.len(), 1);
        let route = &function.metadata.extern_call_routes[0];
        assert_eq!(
            route.route_id(),
            "extern.stage1.emit_mir_from_program_json_v0"
        );
        assert_eq!(route.core_op(), "Stage1EmitMirFromProgramJson");
        assert_eq!(
            route.symbol(),
            "nyash.stage1.emit_mir_from_program_json_v0_h"
        );
        assert_eq!(route.tier(), "ColdRuntime");
        assert_eq!(route.emit_kind(), "runtime_call");
        assert_eq!(route.proof(), "extern_registry");
        assert_eq!(
            route.source_symbol(),
            "nyash.stage1.emit_mir_from_program_json_v0_h"
        );
        assert_eq!(route.key_value(), ValueId::new(0));
        assert_eq!(route.value_value(), None);
        assert_eq!(route.result_value(), ValueId::new(2));
        assert_eq!(route.arity(), 1);
        assert_eq!(route.return_shape(), "string_handle");
        assert_eq!(route.value_demand(), "runtime_i64_or_handle");
        assert_eq!(route.effect_tags(), &["stage1.emit_mir_from_program_json"]);
    }

    #[test]
    fn refresh_function_extern_call_routes_requires_dst_and_matching_arity() {
        let mut missing_dst = make_function_with_call("env.get/1", vec![ValueId::new(1)], None);
        refresh_function_extern_call_routes(&mut missing_dst);
        assert!(missing_dst.metadata.extern_call_routes.is_empty());

        let mut missing_arg = make_function_with_call("env.get/1", vec![], Some(ValueId::new(2)));
        refresh_function_extern_call_routes(&mut missing_arg);
        assert!(missing_arg.metadata.extern_call_routes.is_empty());

        let mut missing_value =
            make_function_with_call("env.set/2", vec![ValueId::new(1)], Some(ValueId::new(2)));
        refresh_function_extern_call_routes(&mut missing_value);
        assert!(missing_value.metadata.extern_call_routes.is_empty());
    }
}
