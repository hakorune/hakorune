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
    HakoTlsCacheSlotGetI64,
    HakoTlsCacheSlotSetI64,
    HakoWorkerCurrentIdI64,
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
            Self::ArraySlotLoadI64 => "extern.array.slot_load_i64",
            Self::ArraySlotStoreI64 => "extern.array.slot_store_i64",
            Self::HakoAtomicSlotCasI64 => "extern.hako_atomic.slot_cas_i64",
            Self::HakoAtomicSlotFetchAddI64 => "extern.hako_atomic.slot_fetch_add_i64",
            Self::HakoAtomicSlotLoadI64 => "extern.hako_atomic.slot_load_i64",
            Self::HakoAtomicSlotStoreI64 => "extern.hako_atomic.slot_store_i64",
            Self::HakoAtomicPtrCasOrdered => "extern.hako_atomic.ptr_cas_ordered",
            Self::HakoAtomicPtrLoadOrdered => "extern.hako_atomic.ptr_load_ordered",
            Self::HakoAtomicPtrStoreOrdered => "extern.hako_atomic.ptr_store_ordered",
            Self::HakoMemAlloc => "extern.hako_mem.alloc",
            Self::HakoMemFree => "extern.hako_mem.free",
            Self::HakoOsvmReserveBytesI64 => "extern.hako_osvm.reserve_bytes_i64",
            Self::HakoOsvmCommitBytesI64 => "extern.hako_osvm.commit_bytes_i64",
            Self::HakoOsvmDecommitBytesI64 => "extern.hako_osvm.decommit_bytes_i64",
            Self::HakoTlsCacheSlotGetI64 => "extern.hako_tls.cache_slot_get_i64",
            Self::HakoTlsCacheSlotSetI64 => "extern.hako_tls.cache_slot_set_i64",
            Self::HakoWorkerCurrentIdI64 => "extern.hako_worker.current_id_i64",
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
            Self::ArraySlotLoadI64 => "ArraySlotLoadI64",
            Self::ArraySlotStoreI64 => "ArraySlotStoreI64",
            Self::HakoAtomicSlotCasI64 => "HakoAtomicSlotCasI64",
            Self::HakoAtomicSlotFetchAddI64 => "HakoAtomicSlotFetchAddI64",
            Self::HakoAtomicSlotLoadI64 => "HakoAtomicSlotLoadI64",
            Self::HakoAtomicSlotStoreI64 => "HakoAtomicSlotStoreI64",
            Self::HakoAtomicPtrCasOrdered => "HakoAtomicPtrCasOrdered",
            Self::HakoAtomicPtrLoadOrdered => "HakoAtomicPtrLoadOrdered",
            Self::HakoAtomicPtrStoreOrdered => "HakoAtomicPtrStoreOrdered",
            Self::HakoMemAlloc => "HakoMemAlloc",
            Self::HakoMemFree => "HakoMemFree",
            Self::HakoOsvmReserveBytesI64 => "HakoOsvmReserveBytesI64",
            Self::HakoOsvmCommitBytesI64 => "HakoOsvmCommitBytesI64",
            Self::HakoOsvmDecommitBytesI64 => "HakoOsvmDecommitBytesI64",
            Self::HakoTlsCacheSlotGetI64 => "HakoTlsCacheSlotGetI64",
            Self::HakoTlsCacheSlotSetI64 => "HakoTlsCacheSlotSetI64",
            Self::HakoWorkerCurrentIdI64 => "HakoWorkerCurrentIdI64",
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
            Self::ArraySlotLoadI64 => "nyash.array.slot_load_hi",
            Self::ArraySlotStoreI64 => "nyash.array.slot_store_hii",
            Self::HakoAtomicSlotCasI64 => "hako_atomic_slot_cas_i64",
            Self::HakoAtomicSlotFetchAddI64 => "hako_atomic_slot_fetch_add_i64",
            Self::HakoAtomicSlotLoadI64 => "hako_atomic_slot_load_i64",
            Self::HakoAtomicSlotStoreI64 => "hako_atomic_slot_store_i64",
            Self::HakoAtomicPtrCasOrdered => "hako_atomic_ptr_cas_ordered",
            Self::HakoAtomicPtrLoadOrdered => "hako_atomic_ptr_load_ordered",
            Self::HakoAtomicPtrStoreOrdered => "hako_atomic_ptr_store_ordered",
            Self::HakoMemAlloc => "hako_mem_alloc",
            Self::HakoMemFree => "hako_mem_free",
            Self::HakoOsvmReserveBytesI64 => "hako_osvm_reserve_bytes_i64",
            Self::HakoOsvmCommitBytesI64 => "hako_osvm_commit_bytes_i64",
            Self::HakoOsvmDecommitBytesI64 => "hako_osvm_decommit_bytes_i64",
            Self::HakoTlsCacheSlotGetI64 => "hako_tls_cache_slot_get_i64",
            Self::HakoTlsCacheSlotSetI64 => "hako_tls_cache_slot_set_i64",
            Self::HakoWorkerCurrentIdI64 => "hako_worker_current_id_i64",
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
            Self::ArraySlotLoadI64 => "extern_registry",
            Self::ArraySlotStoreI64 => "extern_registry",
            Self::HakoAtomicSlotCasI64 => "extern_registry",
            Self::HakoAtomicSlotFetchAddI64 => "extern_registry",
            Self::HakoAtomicSlotLoadI64 => "extern_registry",
            Self::HakoAtomicSlotStoreI64 => "extern_registry",
            Self::HakoAtomicPtrCasOrdered => "extern_registry",
            Self::HakoAtomicPtrLoadOrdered => "extern_registry",
            Self::HakoAtomicPtrStoreOrdered => "extern_registry",
            Self::HakoMemAlloc => "extern_registry",
            Self::HakoMemFree => "extern_registry",
            Self::HakoOsvmReserveBytesI64 => "extern_registry",
            Self::HakoOsvmCommitBytesI64 => "extern_registry",
            Self::HakoOsvmDecommitBytesI64 => "extern_registry",
            Self::HakoTlsCacheSlotGetI64 => "extern_registry",
            Self::HakoTlsCacheSlotSetI64 => "extern_registry",
            Self::HakoWorkerCurrentIdI64 => "extern_registry",
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
            Self::ArraySlotLoadI64 => "scalar_i64",
            Self::ArraySlotStoreI64 => "scalar_i64",
            Self::HakoAtomicSlotCasI64 => "scalar_i64",
            Self::HakoAtomicSlotFetchAddI64 => "scalar_i64",
            Self::HakoAtomicSlotLoadI64 => "scalar_i64",
            Self::HakoAtomicSlotStoreI64 => "scalar_i64",
            Self::HakoAtomicPtrCasOrdered => "native_ptr_nullable",
            Self::HakoAtomicPtrLoadOrdered => "native_ptr_nullable",
            Self::HakoAtomicPtrStoreOrdered => "scalar_i64",
            Self::HakoMemAlloc => "native_ptr_nullable",
            Self::HakoMemFree => "void_sentinel_i64_zero",
            Self::HakoOsvmReserveBytesI64 => "native_ptr_nullable",
            Self::HakoOsvmCommitBytesI64 => "scalar_i64",
            Self::HakoOsvmDecommitBytesI64 => "scalar_i64",
            Self::HakoTlsCacheSlotGetI64 => "scalar_i64",
            Self::HakoTlsCacheSlotSetI64 => "scalar_i64",
            Self::HakoWorkerCurrentIdI64 => "scalar_i64",
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
            Self::ArraySlotLoadI64 => "runtime_i64",
            Self::ArraySlotStoreI64 => "runtime_i64",
            Self::HakoAtomicSlotCasI64 => "runtime_i64",
            Self::HakoAtomicSlotFetchAddI64 => "runtime_i64",
            Self::HakoAtomicSlotLoadI64 => "runtime_i64",
            Self::HakoAtomicSlotStoreI64 => "runtime_i64",
            Self::HakoAtomicPtrCasOrdered => "native_ptr_nullable",
            Self::HakoAtomicPtrLoadOrdered => "native_ptr_nullable",
            Self::HakoAtomicPtrStoreOrdered => "native_ptr_nullable",
            Self::HakoMemAlloc => "native_ptr_nullable",
            Self::HakoMemFree => "scalar_i64",
            Self::HakoOsvmReserveBytesI64 => "native_ptr_nullable",
            Self::HakoOsvmCommitBytesI64 => "runtime_i64",
            Self::HakoOsvmDecommitBytesI64 => "runtime_i64",
            Self::HakoTlsCacheSlotGetI64 => "runtime_i64",
            Self::HakoTlsCacheSlotSetI64 => "runtime_i64",
            Self::HakoWorkerCurrentIdI64 => "runtime_i64",
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
            Self::ArraySlotLoadI64 => &["array.slot_load"],
            Self::ArraySlotStoreI64 => &["array.slot_store_i64"],
            Self::HakoAtomicSlotCasI64 => &["hako.atomic.slot_cas"],
            Self::HakoAtomicSlotFetchAddI64 => &["hako.atomic.slot_fetch_add"],
            Self::HakoAtomicSlotLoadI64 => &["hako.atomic.slot_load"],
            Self::HakoAtomicSlotStoreI64 => &["hako.atomic.slot_store"],
            Self::HakoAtomicPtrCasOrdered => &["hako.atomic.ptr_cas"],
            Self::HakoAtomicPtrLoadOrdered => &["hako.atomic.ptr_load"],
            Self::HakoAtomicPtrStoreOrdered => &["hako.atomic.ptr_store"],
            Self::HakoMemAlloc => &["hako.mem.alloc"],
            Self::HakoMemFree => &["hako.mem.free"],
            Self::HakoOsvmReserveBytesI64 => &["hako.osvm.reserve"],
            Self::HakoOsvmCommitBytesI64 => &["hako.osvm.commit"],
            Self::HakoOsvmDecommitBytesI64 => &["hako.osvm.decommit"],
            Self::HakoTlsCacheSlotGetI64 => &["hako.tls.cache_slot_get"],
            Self::HakoTlsCacheSlotSetI64 => &["hako.tls.cache_slot_set"],
            Self::HakoWorkerCurrentIdI64 => &["hako.worker.current_id"],
            Self::HostBridgeExternInvoke => &["hostbridge.extern"],
            Self::Stage1EmitProgramJson => &["stage1.emit_program_json"],
            Self::Stage1EmitMirFromSource => &["stage1.emit_mir_from_source"],
            Self::Stage1EmitMirFromProgramJson => &["stage1.emit_mir_from_program_json"],
        }
    }

    pub fn accepts_void_result(self) -> bool {
        matches!(
            self,
            Self::EnvSet
                | Self::ArraySlotAppendAny
                | Self::ArraySlotStoreI64
                | Self::HakoAtomicSlotCasI64
                | Self::HakoAtomicSlotFetchAddI64
                | Self::HakoAtomicSlotStoreI64
                | Self::HakoAtomicPtrCasOrdered
                | Self::HakoAtomicPtrStoreOrdered
                | Self::HakoMemFree
                | Self::HakoOsvmCommitBytesI64
                | Self::HakoOsvmDecommitBytesI64
                | Self::HakoTlsCacheSlotSetI64
        )
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

    pub fn result_value_opt(&self) -> Option<ValueId> {
        if self.result_value == ValueId::INVALID {
            None
        } else {
            Some(self.result_value)
        }
    }

    pub fn arity(&self) -> usize {
        match self.kind {
            ExternCallRouteKind::EnvGet => 1,
            ExternCallRouteKind::EnvSet => 2,
            ExternCallRouteKind::AnyHandleLive => 1,
            ExternCallRouteKind::ArraySlotAppendAny => 2,
            ExternCallRouteKind::ArraySlotLenI64 => 1,
            ExternCallRouteKind::ArraySlotLoadI64 => 2,
            ExternCallRouteKind::ArraySlotStoreI64 => 3,
            ExternCallRouteKind::HakoAtomicSlotCasI64 => 3,
            ExternCallRouteKind::HakoAtomicSlotFetchAddI64 => 2,
            ExternCallRouteKind::HakoAtomicSlotLoadI64 => 1,
            ExternCallRouteKind::HakoAtomicSlotStoreI64 => 2,
            ExternCallRouteKind::HakoAtomicPtrCasOrdered => 5,
            ExternCallRouteKind::HakoAtomicPtrLoadOrdered => 2,
            ExternCallRouteKind::HakoAtomicPtrStoreOrdered => 3,
            ExternCallRouteKind::HakoMemAlloc => 1,
            ExternCallRouteKind::HakoMemFree => 1,
            ExternCallRouteKind::HakoOsvmReserveBytesI64 => 1,
            ExternCallRouteKind::HakoOsvmCommitBytesI64 => 2,
            ExternCallRouteKind::HakoOsvmDecommitBytesI64 => 2,
            ExternCallRouteKind::HakoTlsCacheSlotGetI64 => 1,
            ExternCallRouteKind::HakoTlsCacheSlotSetI64 => 2,
            ExternCallRouteKind::HakoWorkerCurrentIdI64 => 1,
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
        .or_else(|| name.strip_suffix("/4"))
        .or_else(|| name.strip_suffix("/5"))
        .unwrap_or(name)
}

pub fn classify_extern_call_route(name: &str, argc: usize) -> Option<ExternCallRouteKind> {
    match (normalize_extern_symbol(name), argc) {
        ("env.get", 1) | ("nyash.env.get", 1) => Some(ExternCallRouteKind::EnvGet),
        ("env.set", 2) | ("nyash.env.set", 2) => Some(ExternCallRouteKind::EnvSet),
        ("nyash.any.handle_live_h", 1) => Some(ExternCallRouteKind::AnyHandleLive),
        ("nyash.array.slot_append_hh", 2) => Some(ExternCallRouteKind::ArraySlotAppendAny),
        ("nyash.array.slot_len_h", 1) => Some(ExternCallRouteKind::ArraySlotLenI64),
        ("nyash.array.slot_load_hi", 2) => Some(ExternCallRouteKind::ArraySlotLoadI64),
        ("nyash.array.slot_store_hii", 3) => Some(ExternCallRouteKind::ArraySlotStoreI64),
        ("hako_atomic_slot_cas_i64", 3) => Some(ExternCallRouteKind::HakoAtomicSlotCasI64),
        ("hako_atomic_slot_fetch_add_i64", 2) => {
            Some(ExternCallRouteKind::HakoAtomicSlotFetchAddI64)
        }
        ("hako_atomic_slot_load_i64", 1) => Some(ExternCallRouteKind::HakoAtomicSlotLoadI64),
        ("hako_atomic_slot_store_i64", 2) => Some(ExternCallRouteKind::HakoAtomicSlotStoreI64),
        ("hako_atomic_ptr_cas_ordered", 5) => Some(ExternCallRouteKind::HakoAtomicPtrCasOrdered),
        ("hako_atomic_ptr_load_ordered", 2) => Some(ExternCallRouteKind::HakoAtomicPtrLoadOrdered),
        ("hako_atomic_ptr_store_ordered", 3) => {
            Some(ExternCallRouteKind::HakoAtomicPtrStoreOrdered)
        }
        ("hako_mem_alloc", 1) => Some(ExternCallRouteKind::HakoMemAlloc),
        ("hako_mem_free", 1) => Some(ExternCallRouteKind::HakoMemFree),
        ("hako_osvm_reserve_bytes_i64", 1) => Some(ExternCallRouteKind::HakoOsvmReserveBytesI64),
        ("hako_osvm_commit_bytes_i64", 2) => Some(ExternCallRouteKind::HakoOsvmCommitBytesI64),
        ("hako_osvm_decommit_bytes_i64", 2) => Some(ExternCallRouteKind::HakoOsvmDecommitBytesI64),
        ("hako_tls_cache_slot_get_i64", 1) => Some(ExternCallRouteKind::HakoTlsCacheSlotGetI64),
        ("hako_tls_cache_slot_set_i64", 2) => Some(ExternCallRouteKind::HakoTlsCacheSlotSetI64),
        ("hako_worker_current_id_i64", 1) => Some(ExternCallRouteKind::HakoWorkerCurrentIdI64),
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
                dst,
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
            if dst.is_none() && !kind.accepts_void_result() {
                continue;
            }
            let Some(key_value) = args.first().copied() else {
                continue;
            };
            let value_value = match kind {
                ExternCallRouteKind::EnvGet => None,
                ExternCallRouteKind::EnvSet => args.get(1).copied(),
                ExternCallRouteKind::AnyHandleLive => None,
                ExternCallRouteKind::ArraySlotAppendAny => args.get(1).copied(),
                ExternCallRouteKind::ArraySlotLenI64 => None,
                ExternCallRouteKind::ArraySlotLoadI64 => args.get(1).copied(),
                ExternCallRouteKind::ArraySlotStoreI64 => args.get(2).copied(),
                ExternCallRouteKind::HakoAtomicSlotCasI64 => args.get(2).copied(),
                ExternCallRouteKind::HakoAtomicSlotFetchAddI64 => args.get(1).copied(),
                ExternCallRouteKind::HakoAtomicSlotLoadI64 => None,
                ExternCallRouteKind::HakoAtomicSlotStoreI64 => args.get(1).copied(),
                ExternCallRouteKind::HakoAtomicPtrCasOrdered => args.get(2).copied(),
                ExternCallRouteKind::HakoAtomicPtrLoadOrdered => None,
                ExternCallRouteKind::HakoAtomicPtrStoreOrdered => args.get(1).copied(),
                ExternCallRouteKind::HakoMemAlloc => None,
                ExternCallRouteKind::HakoMemFree => None,
                ExternCallRouteKind::HakoOsvmReserveBytesI64 => None,
                ExternCallRouteKind::HakoOsvmCommitBytesI64 => args.get(1).copied(),
                ExternCallRouteKind::HakoOsvmDecommitBytesI64 => args.get(1).copied(),
                ExternCallRouteKind::HakoTlsCacheSlotGetI64 => None,
                ExternCallRouteKind::HakoTlsCacheSlotSetI64 => args.get(1).copied(),
                ExternCallRouteKind::HakoWorkerCurrentIdI64 => None,
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
                dst.unwrap_or(ValueId::INVALID),
            ));
        }
    }

    function.metadata.extern_call_routes = routes;
}

#[cfg(test)]
mod tests;
