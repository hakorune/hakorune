#![cfg(feature = "cranelift-jit")]

// Cranelift-based IR builder moved out of builder.rs for readability and maintainability

use cranelift_codegen::ir::InstBuilder;
use cranelift_module::Module;

// TLS utilities and runtime shims live next to this builder under the same module
use super::rt_shims::{
    nyash_host_stub0, nyash_jit_block_enter, nyash_jit_dbg_i64, nyash_plugin_invoke3_f64,
    nyash_plugin_invoke3_i64, nyash_plugin_invoke_name_call_i64,
    nyash_plugin_invoke_name_getattr_i64,
};
use super::tls::clif_tls;

// Handle-based extern thunks used by lowering
use super::super::extern_thunks::{
    nyash_any_is_empty_h, nyash_any_length_h, nyash_array_get_h, nyash_array_last_h,
    nyash_array_len_h, nyash_array_push_h, nyash_array_set_h, nyash_box_birth_h,
    nyash_box_birth_i64, nyash_console_birth_h, nyash_gc_barrier_write, nyash_handle_of,
    nyash_integer_birth_h, nyash_map_get_h, nyash_map_get_hh, nyash_map_has_h, nyash_map_set_h,
    nyash_map_size_h, nyash_math_abs_f64, nyash_math_cos_f64, nyash_math_max_f64,
    nyash_math_min_f64, nyash_math_sin_f64, nyash_rt_checkpoint, nyash_string_birth_h,
    nyash_string_charcode_at_h, nyash_string_concat_hh, nyash_string_eq_hh, nyash_string_from_ptr,
    nyash_string_len_h, nyash_string_lt_hh,
};

use crate::jit::r#extern::r#async::nyash_future_await_h;
use crate::jit::r#extern::result::{nyash_result_err_h, nyash_result_ok_h};
pub struct CraneliftBuilder {
    pub module: cranelift_jit::JITModule,
    pub ctx: cranelift_codegen::Context,
    pub fbc: cranelift_frontend::FunctionBuilderContext,
    pub stats: (usize, usize, usize, usize, usize), // (consts, binops, cmps, branches, rets)
    // Build-state (minimal stack machine for Core-1)
    current_name: Option<String>,
    value_stack: Vec<cranelift_codegen::ir::Value>,
    entry_block: Option<cranelift_codegen::ir::Block>,
    // Phase 10.7: basic block wiring state
    blocks: Vec<cranelift_codegen::ir::Block>,
    current_block_index: Option<usize>,
    block_param_counts: std::collections::HashMap<usize, usize>,
    // Local stack slots for minimal Load/Store lowering (i64 only)
    local_slots: std::collections::HashMap<usize, cranelift_codegen::ir::StackSlot>,
    // Finalized function pointer (if any)
    compiled_closure: Option<
        std::sync::Arc<
            dyn Fn(&[crate::jit::abi::JitValue]) -> crate::jit::abi::JitValue + Send + Sync,
        >,
    >,
    // Desired simple ABI (Phase 10_c minimal): i64 params count and i64 return
    desired_argc: usize,
    desired_has_ret: bool,
    desired_ret_is_f64: bool,
    typed_sig_prepared: bool,
    // Return-type hint: function returns boolean (footing only; ABI remains i64 for now)
    ret_hint_is_b1: bool,
    // Single-exit epilogue (jit-direct stability): ret block + i64 slot
    ret_block: Option<cranelift_codegen::ir::Block>,
    ret_slot: Option<cranelift_codegen::ir::StackSlot>,
    // Blocks requested before begin_function (to avoid TLS usage early)
    pending_blocks: usize,
    // Whether current block needs a terminator before switching away
    cur_needs_term: bool,
    // Track blocks sealed to avoid resealing
    sealed_blocks: std::collections::HashSet<usize>,
}

mod calls;
mod flow;
mod lifecycle;
mod ops;

impl CraneliftBuilder {
    fn materialize_succ_params(
        &mut self,
        fb: &mut cranelift_frontend::FunctionBuilder<'static>,
        succ_index: usize,
    ) -> bool {
        use cranelift_codegen::ir::types;
        if succ_index >= self.blocks.len() {
            return false;
        }
        let b = self.blocks[succ_index];
        let has_inst = fb.func.layout.first_inst(b).is_some();
        if !has_inst {
            let desired = self
                .block_param_counts
                .get(&succ_index)
                .copied()
                .unwrap_or(0);
            let current = fb.func.dfg.block_params(b).len();
            if desired > current {
                for _ in current..desired {
                    let _ = fb.append_block_param(b, types::I64);
                }
            }
        }
        has_inst
    }
    fn entry_param(&mut self, index: usize) -> Option<cranelift_codegen::ir::Value> {
        if let Some(b) = self.entry_block {
            return Self::with_fb(|fb| fb.func.dfg.block_params(b).get(index).copied());
        }
        None
    }
    fn with_fb<R>(f: impl FnOnce(&mut cranelift_frontend::FunctionBuilder<'static>) -> R) -> R {
        clif_tls::FB.with(|cell| {
            let mut opt = cell.borrow_mut();
            let tls = opt.as_mut().expect("FunctionBuilder TLS not initialized");
            tls.with(f)
        })
    }
    pub fn new() -> Self {
        let mut builder = cranelift_jit::JITBuilder::new(cranelift_module::default_libcall_names())
            .expect("JITBuilder");
        // Hostcall symbols
        builder.symbol("nyash.host.stub0", nyash_host_stub0 as *const u8);
        builder.symbol("nyash.jit.dbg_i64", nyash_jit_dbg_i64 as *const u8);
        builder.symbol("nyash.jit.block_enter", nyash_jit_block_enter as *const u8);
        // Async/Result
        builder.symbol(
            crate::jit::r#extern::r#async::SYM_FUTURE_AWAIT_H,
            nyash_future_await_h as *const u8,
        );
        builder.symbol(
            crate::jit::r#extern::result::SYM_RESULT_OK_H,
            nyash_result_ok_h as *const u8,
        );
        builder.symbol(
            crate::jit::r#extern::result::SYM_RESULT_ERR_H,
            nyash_result_err_h as *const u8,
        );
        // Math
        builder.symbol("nyash.math.sin_f64", nyash_math_sin_f64 as *const u8);
        builder.symbol("nyash.math.cos_f64", nyash_math_cos_f64 as *const u8);
        builder.symbol("nyash.math.abs_f64", nyash_math_abs_f64 as *const u8);
        builder.symbol("nyash.math.min_f64", nyash_math_min_f64 as *const u8);
        builder.symbol("nyash.math.max_f64", nyash_math_max_f64 as *const u8);
        // Handle-based collection/string/runtime
        {
            use crate::jit::r#extern::{birth as b, collections as c, handles as h, runtime as r};
            builder.symbol(c::SYM_ARRAY_LEN_H, nyash_array_len_h as *const u8);
            builder.symbol(c::SYM_ARRAY_GET_H, nyash_array_get_h as *const u8);
            builder.symbol(c::SYM_ARRAY_SET_H, nyash_array_set_h as *const u8);
            builder.symbol(c::SYM_ARRAY_PUSH_H, nyash_array_push_h as *const u8);
            builder.symbol(c::SYM_ARRAY_LAST_H, nyash_array_last_h as *const u8);
            builder.symbol(c::SYM_MAP_SIZE_H, nyash_map_size_h as *const u8);
            builder.symbol(c::SYM_MAP_GET_H, nyash_map_get_h as *const u8);
            builder.symbol(c::SYM_MAP_GET_HH, nyash_map_get_hh as *const u8);
            builder.symbol(c::SYM_MAP_SET_H, nyash_map_set_h as *const u8);
            builder.symbol(c::SYM_MAP_HAS_H, nyash_map_has_h as *const u8);
            builder.symbol(c::SYM_ANY_LEN_H, nyash_any_length_h as *const u8);
            builder.symbol(c::SYM_STRING_LEN_H, nyash_string_len_h as *const u8);
            builder.symbol(c::SYM_ANY_IS_EMPTY_H, nyash_any_is_empty_h as *const u8);
            builder.symbol(
                c::SYM_STRING_CHARCODE_AT_H,
                nyash_string_charcode_at_h as *const u8,
            );
            builder.symbol(c::SYM_STRING_BIRTH_H, nyash_string_birth_h as *const u8);
            builder.symbol(c::SYM_INTEGER_BIRTH_H, nyash_integer_birth_h as *const u8);
            builder.symbol("nyash.console.birth_h", nyash_console_birth_h as *const u8);
            builder.symbol(c::SYM_STRING_CONCAT_HH, nyash_string_concat_hh as *const u8);
            builder.symbol(c::SYM_STRING_EQ_HH, nyash_string_eq_hh as *const u8);
            builder.symbol(c::SYM_STRING_LT_HH, nyash_string_lt_hh as *const u8);
            builder.symbol(b::SYM_BOX_BIRTH_H, nyash_box_birth_h as *const u8);
            builder.symbol("nyash.box.birth_i64", nyash_box_birth_i64 as *const u8);
            builder.symbol(
                crate::jit::r#extern::birth::SYM_INSTANCE_BIRTH_NAME_U64X2,
                super::super::extern_thunks::nyash_instance_birth_name_u64x2 as *const u8,
            );
            builder.symbol(h::SYM_HANDLE_OF, nyash_handle_of as *const u8);
            builder.symbol(r::SYM_RT_CHECKPOINT, nyash_rt_checkpoint as *const u8);
            builder.symbol(r::SYM_GC_BARRIER_WRITE, nyash_gc_barrier_write as *const u8);
        }
        // Plugin invoke shims
        builder.symbol(
            "nyash_plugin_invoke3_i64",
            nyash_plugin_invoke3_i64 as *const u8,
        );
        builder.symbol(
            "nyash_plugin_invoke3_f64",
            nyash_plugin_invoke3_f64 as *const u8,
        );
        builder.symbol(
            "nyash_plugin_invoke_name_getattr_i64",
            nyash_plugin_invoke_name_getattr_i64 as *const u8,
        );
        builder.symbol(
            "nyash_plugin_invoke_name_call_i64",
            nyash_plugin_invoke_name_call_i64 as *const u8,
        );
        builder.symbol(
            crate::jit::r#extern::collections::SYM_STRING_FROM_U64X2,
            super::super::extern_thunks::nyash_string_from_u64x2 as *const u8,
        );

        // Host-bridge (by-slot) imports (opt-in)
        if std::env::var("NYASH_JIT_HOST_BRIDGE").ok().as_deref() == Some("1") {
            use crate::jit::r#extern::host_bridge as hb;
            // Instance.getField/setField (recv_h, name_i[, val_i])
            // Use arity-stable import symbols to avoid signature collisions
            builder.symbol(
                hb::SYM_HOST_INSTANCE_FIELD3,
                super::super::extern_thunks::nyash_host_instance_field3 as *const u8,
            );
            // String.len (recv_h)
            builder.symbol(
                hb::SYM_HOST_STRING_LEN,
                super::super::extern_thunks::nyash_host_string_len as *const u8,
            );
            // Console.* (value)
            builder.symbol(
                hb::SYM_HOST_CONSOLE_LOG,
                super::super::extern_thunks::nyash_host_console_log_i64 as *const u8,
            );
            builder.symbol(
                hb::SYM_HOST_CONSOLE_WARN,
                super::super::extern_thunks::nyash_host_console_warn_i64 as *const u8,
            );
            builder.symbol(
                hb::SYM_HOST_CONSOLE_ERROR,
                super::super::extern_thunks::nyash_host_console_error_i64 as *const u8,
            );
        }

        let module = cranelift_jit::JITModule::new(builder);
        let ctx = cranelift_codegen::Context::new();
        let fbc = cranelift_frontend::FunctionBuilderContext::new();
        CraneliftBuilder {
            module,
            ctx,
            fbc,
            stats: (0, 0, 0, 0, 0),
            current_name: None,
            value_stack: Vec::new(),
            entry_block: None,
            blocks: Vec::new(),
            current_block_index: None,
            block_param_counts: std::collections::HashMap::new(),
            local_slots: std::collections::HashMap::new(),
            compiled_closure: None,
            desired_argc: 0,
            desired_has_ret: true,
            desired_ret_is_f64: false,
            typed_sig_prepared: false,
            ret_hint_is_b1: false,
            ret_block: None,
            ret_slot: None,
            pending_blocks: 0,
            cur_needs_term: false,
            sealed_blocks: std::collections::HashSet::new(),
        }
    }
    pub fn take_compiled_closure(
        &mut self,
    ) -> Option<
        std::sync::Arc<
            dyn Fn(&[crate::jit::abi::JitValue]) -> crate::jit::abi::JitValue + Send + Sync,
        >,
    > {
        self.compiled_closure.take()
    }
}
