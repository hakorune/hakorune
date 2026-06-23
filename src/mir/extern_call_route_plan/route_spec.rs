use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternCallRouteKind {
    EnvGet,
    EnvSet,
    EnvNowMs,
    StringConcat,
    StringConcat3,
    StringInsert,
    StringSubstring,
    StringSubstringConcat,
    StringSubstringConcat3,
    StringSubstringLen,
    AnyHandleLive,
    ArraySlotAppendAny,
    ArraySlotLenI64,
    ArraySlotLoadI64,
    ArraySlotStoreI64,
    HakoAtomicSlotCasI64,
    HakoAtomicSlotFetchAddI64,
    HakoAtomicSlotLoadI64,
    HakoAtomicSlotStoreI64,
    HakoAtomicPtrCasOrdered,
    HakoAtomicPtrLoadOrdered,
    HakoAtomicPtrStoreOrdered,
    HakoMemAlloc,
    HakoMemFree,
    HakoOsvmReserveBytesI64,
    HakoOsvmCommitBytesI64,
    HakoOsvmDecommitBytesI64,
    HakoOsvmUnreserveBytesI64,
    HakoTlsCacheSlotGetI64,
    HakoTlsCacheSlotSetI64,
    HakoWorkerCurrentIdI64,
    HostBridgeExternInvoke,
    Stage1EmitProgramJson,
    Stage1EmitMirFromSource,
    Stage1EmitMirFromProgramJson,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExternCallRouteSpec {
    kind: ExternCallRouteKind,
    route_id: &'static str,
    core_op: &'static str,
    symbol: &'static str,
    aliases: &'static [&'static str],
    arity: usize,
    value_arg_index: Option<usize>,
    proof: &'static str,
    return_shape: &'static str,
    value_demand: &'static str,
    effect_tags: &'static [&'static str],
    accepts_void_result: bool,
}

impl ExternCallRouteSpec {
    pub fn kind(&self) -> ExternCallRouteKind {
        self.kind
    }
    pub fn route_id(&self) -> &'static str {
        self.route_id
    }
    pub fn core_op(&self) -> &'static str {
        self.core_op
    }
    pub fn symbol(&self) -> &'static str {
        self.symbol
    }
    pub fn aliases(&self) -> &'static [&'static str] {
        self.aliases
    }
    pub fn arity(&self) -> usize {
        self.arity
    }
    pub fn value_arg_index(&self) -> Option<usize> {
        self.value_arg_index
    }
    pub fn proof(&self) -> &'static str {
        self.proof
    }
    pub fn return_shape(&self) -> &'static str {
        self.return_shape
    }
    pub fn value_demand(&self) -> &'static str {
        self.value_demand
    }
    pub fn effect_tags(&self) -> &'static [&'static str] {
        self.effect_tags
    }
    pub fn accepts_void_result(&self) -> bool {
        self.accepts_void_result
    }
    pub fn accepts_symbol(&self, symbol: &str) -> bool {
        self.symbol == symbol || self.aliases.contains(&symbol)
    }
}

const EXTERN_REGISTRY_PROOF: &str = "extern_registry";

static EXTERN_CALL_ROUTE_SPECS: &[ExternCallRouteSpec] = &[
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::EnvGet,
        route_id: "extern.env.get",
        core_op: "EnvGet",
        symbol: "nyash.env.get",
        aliases: &["env.get"],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle_or_null",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["read.env"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::EnvSet,
        route_id: "extern.env.set",
        core_op: "EnvSet",
        symbol: "nyash.env.set",
        aliases: &["env.set"],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["write.env"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::EnvNowMs,
        route_id: "extern.env.now_ms",
        core_op: "EnvNowMs",
        symbol: "nyash.env.now_ms",
        aliases: &["env.now_ms"],
        arity: 0,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["read.time"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::StringConcat,
        route_id: "extern.string.concat_hh",
        core_op: "StringConcat",
        symbol: "nyash.string.concat_hh",
        aliases: &[],
        arity: 2,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["string.concat"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::StringConcat3,
        route_id: "extern.string.concat3_hhh",
        core_op: "StringConcat3",
        symbol: "nyash.string.concat3_hhh",
        aliases: &[],
        arity: 3,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["string.concat"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::StringInsert,
        route_id: "extern.string.insert_hsi",
        core_op: "StringInsert",
        symbol: "nyash.string.insert_hsi",
        aliases: &[],
        arity: 3,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["string.insert"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::StringSubstring,
        route_id: "extern.string.substring_hii",
        core_op: "StringSubstring",
        symbol: "nyash.string.substring_hii",
        aliases: &[],
        arity: 3,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["string.substring"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::StringSubstringConcat,
        route_id: "extern.string.substring_concat_hhii",
        core_op: "StringSubstringConcat",
        symbol: "nyash.string.substring_concat_hhii",
        aliases: &[],
        arity: 4,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["string.substring"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::StringSubstringConcat3,
        route_id: "extern.string.substring_concat3_hhhii",
        core_op: "StringSubstringConcat3",
        symbol: "nyash.string.substring_concat3_hhhii",
        aliases: &[],
        arity: 5,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["string.substring"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::StringSubstringLen,
        route_id: "extern.string.substring_len_hii",
        core_op: "StringSubstringLen",
        symbol: "nyash.string.substring_len_hii",
        aliases: &[],
        arity: 3,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["string.substring_len"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::AnyHandleLive,
        route_id: "extern.any.handle_live",
        core_op: "AnyHandleLive",
        symbol: "nyash.any.handle_live_h",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["read.any.handle_live"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::ArraySlotAppendAny,
        route_id: "extern.array.slot_append_any",
        core_op: "ArraySlotAppendAny",
        symbol: "nyash.array.slot_append_hh",
        aliases: &[],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["array.slot_append"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::ArraySlotLenI64,
        route_id: "extern.array.slot_len_i64",
        core_op: "ArraySlotLenI64",
        symbol: "nyash.array.slot_len_h",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["array.slot_len"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::ArraySlotLoadI64,
        route_id: "extern.array.slot_load_i64",
        core_op: "ArraySlotLoadI64",
        symbol: "nyash.array.slot_load_hi",
        aliases: &[],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["array.slot_load"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::ArraySlotStoreI64,
        route_id: "extern.array.slot_store_i64",
        core_op: "ArraySlotStoreI64",
        symbol: "nyash.array.slot_store_hii",
        aliases: &[],
        arity: 3,
        value_arg_index: Some(2),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["array.slot_store_i64"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoAtomicSlotCasI64,
        route_id: "extern.hako_atomic.slot_cas_i64",
        core_op: "HakoAtomicSlotCasI64",
        symbol: "hako_atomic_slot_cas_i64",
        aliases: &[],
        arity: 3,
        value_arg_index: Some(2),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.atomic.slot_cas"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoAtomicSlotFetchAddI64,
        route_id: "extern.hako_atomic.slot_fetch_add_i64",
        core_op: "HakoAtomicSlotFetchAddI64",
        symbol: "hako_atomic_slot_fetch_add_i64",
        aliases: &[],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.atomic.slot_fetch_add"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoAtomicSlotLoadI64,
        route_id: "extern.hako_atomic.slot_load_i64",
        core_op: "HakoAtomicSlotLoadI64",
        symbol: "hako_atomic_slot_load_i64",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.atomic.slot_load"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoAtomicSlotStoreI64,
        route_id: "extern.hako_atomic.slot_store_i64",
        core_op: "HakoAtomicSlotStoreI64",
        symbol: "hako_atomic_slot_store_i64",
        aliases: &[],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.atomic.slot_store"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoAtomicPtrCasOrdered,
        route_id: "extern.hako_atomic.ptr_cas_ordered",
        core_op: "HakoAtomicPtrCasOrdered",
        symbol: "hako_atomic_ptr_cas_ordered",
        aliases: &[],
        arity: 5,
        value_arg_index: Some(2),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "native_ptr_nullable",
        value_demand: "native_ptr_nullable",
        effect_tags: &["hako.atomic.ptr_cas"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoAtomicPtrLoadOrdered,
        route_id: "extern.hako_atomic.ptr_load_ordered",
        core_op: "HakoAtomicPtrLoadOrdered",
        symbol: "hako_atomic_ptr_load_ordered",
        aliases: &[],
        arity: 2,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "native_ptr_nullable",
        value_demand: "native_ptr_nullable",
        effect_tags: &["hako.atomic.ptr_load"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoAtomicPtrStoreOrdered,
        route_id: "extern.hako_atomic.ptr_store_ordered",
        core_op: "HakoAtomicPtrStoreOrdered",
        symbol: "hako_atomic_ptr_store_ordered",
        aliases: &[],
        arity: 3,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "native_ptr_nullable",
        effect_tags: &["hako.atomic.ptr_store"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoMemAlloc,
        route_id: "extern.hako_mem.alloc",
        core_op: "HakoMemAlloc",
        symbol: "hako_mem_alloc",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "native_ptr_nullable",
        value_demand: "native_ptr_nullable",
        effect_tags: &["hako.mem.alloc"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoMemFree,
        route_id: "extern.hako_mem.free",
        core_op: "HakoMemFree",
        symbol: "hako_mem_free",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "void_sentinel_i64_zero",
        value_demand: "scalar_i64",
        effect_tags: &["hako.mem.free"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoOsvmReserveBytesI64,
        route_id: "extern.hako_osvm.reserve_bytes_i64",
        core_op: "HakoOsvmReserveBytesI64",
        symbol: "hako_osvm_reserve_bytes_i64",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "native_ptr_nullable",
        value_demand: "native_ptr_nullable",
        effect_tags: &["hako.osvm.reserve"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoOsvmCommitBytesI64,
        route_id: "extern.hako_osvm.commit_bytes_i64",
        core_op: "HakoOsvmCommitBytesI64",
        symbol: "hako_osvm_commit_bytes_i64",
        aliases: &[],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.osvm.commit"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoOsvmDecommitBytesI64,
        route_id: "extern.hako_osvm.decommit_bytes_i64",
        core_op: "HakoOsvmDecommitBytesI64",
        symbol: "hako_osvm_decommit_bytes_i64",
        aliases: &[],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.osvm.decommit"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoOsvmUnreserveBytesI64,
        route_id: "extern.hako_osvm.unreserve_bytes_i64",
        core_op: "HakoOsvmUnreserveBytesI64",
        symbol: "hako_osvm_unreserve_bytes_i64",
        aliases: &[],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.osvm.unreserve"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoTlsCacheSlotGetI64,
        route_id: "extern.hako_tls.cache_slot_get_i64",
        core_op: "HakoTlsCacheSlotGetI64",
        symbol: "hako_tls_cache_slot_get_i64",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.tls.cache_slot_get"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoTlsCacheSlotSetI64,
        route_id: "extern.hako_tls.cache_slot_set_i64",
        core_op: "HakoTlsCacheSlotSetI64",
        symbol: "hako_tls_cache_slot_set_i64",
        aliases: &[],
        arity: 2,
        value_arg_index: Some(1),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.tls.cache_slot_set"],
        accepts_void_result: true,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HakoWorkerCurrentIdI64,
        route_id: "extern.hako_worker.current_id_i64",
        core_op: "HakoWorkerCurrentIdI64",
        symbol: "hako_worker_current_id_i64",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "scalar_i64",
        value_demand: "runtime_i64",
        effect_tags: &["hako.worker.current_id"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::HostBridgeExternInvoke,
        route_id: "extern.hostbridge.extern_invoke",
        core_op: "HostBridgeExternInvoke",
        symbol: "nyash.hostbridge.extern_invoke",
        aliases: &["hostbridge.extern_invoke"],
        arity: 3,
        value_arg_index: Some(2),
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle_or_null",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["hostbridge.extern"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::Stage1EmitProgramJson,
        route_id: "extern.stage1.emit_program_json_v0",
        core_op: "Stage1EmitProgramJson",
        symbol: "nyash.stage1.emit_program_json_v0_h",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["stage1.emit_program_json"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::Stage1EmitMirFromSource,
        route_id: "extern.stage1.emit_mir_from_source_v0",
        core_op: "Stage1EmitMirFromSource",
        symbol: "nyash.stage1.emit_mir_from_source_v0_h",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["stage1.emit_mir_from_source"],
        accepts_void_result: false,
    },
    ExternCallRouteSpec {
        kind: ExternCallRouteKind::Stage1EmitMirFromProgramJson,
        route_id: "extern.stage1.emit_mir_from_program_json_v0",
        core_op: "Stage1EmitMirFromProgramJson",
        symbol: "nyash.stage1.emit_mir_from_program_json_v0_h",
        aliases: &[],
        arity: 1,
        value_arg_index: None,
        proof: EXTERN_REGISTRY_PROOF,
        return_shape: "string_handle",
        value_demand: "runtime_i64_or_handle",
        effect_tags: &["stage1.emit_mir_from_program_json"],
        accepts_void_result: false,
    },
];

pub fn extern_call_route_specs() -> &'static [ExternCallRouteSpec] {
    EXTERN_CALL_ROUTE_SPECS
}

impl ExternCallRouteKind {
    pub fn spec(self) -> &'static ExternCallRouteSpec {
        EXTERN_CALL_ROUTE_SPECS
            .iter()
            .find(|spec| spec.kind == self)
            .expect("extern call route kind must have a spec")
    }

    pub fn route_id(self) -> &'static str {
        self.spec().route_id()
    }
    pub fn core_op(self) -> &'static str {
        self.spec().core_op()
    }
    pub fn symbol(self) -> &'static str {
        self.spec().symbol()
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
        self.spec().proof()
    }
    pub fn return_shape(self) -> &'static str {
        self.spec().return_shape()
    }
    pub fn value_demand(self) -> &'static str {
        self.spec().value_demand()
    }
    pub fn effect_tags(self) -> &'static [&'static str] {
        self.spec().effect_tags()
    }
    pub fn arity(self) -> usize {
        self.spec().arity()
    }
    pub fn value_arg_index(self) -> Option<usize> {
        self.spec().value_arg_index()
    }
    pub fn accepts_void_result(self) -> bool {
        self.spec().accepts_void_result()
    }
}

pub fn normalize_extern_symbol(name: &str) -> &str {
    name.strip_suffix("/0")
        .or_else(|| name.strip_suffix("/1"))
        .or_else(|| name.strip_suffix("/2"))
        .or_else(|| name.strip_suffix("/3"))
        .or_else(|| name.strip_suffix("/4"))
        .or_else(|| name.strip_suffix("/5"))
        .unwrap_or(name)
}

pub fn classify_extern_call_route(name: &str, argc: usize) -> Option<ExternCallRouteKind> {
    let normalized = normalize_extern_symbol(name);
    EXTERN_CALL_ROUTE_SPECS
        .iter()
        .find(|spec| spec.arity() == argc && spec.accepts_symbol(normalized))
        .map(ExternCallRouteSpec::kind)
}

pub fn is_hostbridge_extern_invoke_symbol(name: &str, argc: usize) -> bool {
    classify_extern_call_route(name, argc) == Some(ExternCallRouteKind::HostBridgeExternInvoke)
}
