use super::super::tls::clif_tls;
use super::super::{BinOpKind, CmpKind, IRBuilder, ParamKind};
use super::CraneliftBuilder;

impl IRBuilder for CraneliftBuilder {
    fn prepare_signature_typed(&mut self, params: &[ParamKind], ret_is_f64: bool) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        fn abi_param_for_kind(
            k: ParamKind,
            cfg: &crate::jit::config::JitConfig,
        ) -> cranelift_codegen::ir::AbiParam {
            use cranelift_codegen::ir::types;
            match k {
                ParamKind::I64 => cranelift_codegen::ir::AbiParam::new(types::I64),
                ParamKind::F64 => cranelift_codegen::ir::AbiParam::new(types::F64),
                ParamKind::B1 => {
                    let _ = cfg.native_bool_abi;
                    #[cfg(feature = "jit-b1-abi")]
                    {
                        if crate::jit::config::probe_capabilities().supports_b1_sig
                            && cfg.native_bool_abi
                        {
                            return cranelift_codegen::ir::AbiParam::new(types::B1);
                        }
                    }
                    cranelift_codegen::ir::AbiParam::new(types::I64)
                }
            }
        }
        self.desired_argc = params.len();
        self.desired_has_ret = true;
        self.desired_ret_is_f64 = ret_is_f64;
        let call_conv = self.module.isa().default_call_conv();
        let mut sig = Signature::new(call_conv);
        let cfg_now = crate::jit::config::current();
        for &k in params {
            sig.params.push(abi_param_for_kind(k, &cfg_now));
        }
        if self.desired_has_ret {
            if self.desired_ret_is_f64 {
                sig.returns.push(AbiParam::new(types::F64));
            } else {
                let mut used_b1 = false;
                #[cfg(feature = "jit-b1-abi")]
                {
                    let cfg_now = crate::jit::config::current();
                    if crate::jit::config::probe_capabilities().supports_b1_sig
                        && cfg_now.native_bool_abi
                        && self.ret_hint_is_b1
                    {
                        sig.returns.push(AbiParam::new(types::B1));
                        used_b1 = true;
                    }
                }
                if !used_b1 {
                    sig.returns.push(AbiParam::new(types::I64));
                }
            }
        }
        self.ctx.func.signature = sig;
        self.typed_sig_prepared = true;
    }
    fn emit_param_i64(&mut self, index: usize) {
        if let Some(v) = self.entry_param(index) {
            self.value_stack.push(v);
        }
    }
    fn prepare_signature_i64(&mut self, argc: usize, _has_ret: bool) {
        self.desired_argc = argc;
        // JIT-direct stability: always materialize an i64 return slot (VMValue Integer/Bool/Float can be coerced)
        self.desired_has_ret = true;
        // i64-only signature: return type must be i64 regardless of host f64 capability
        self.desired_ret_is_f64 = false;
    }
    fn begin_function(&mut self, name: &str) {
        use cranelift_codegen::ir::{types, AbiParam, Signature};
        self.current_name = Some(name.to_string());
        self.value_stack.clear();
        clif_tls::FB.with(|cell| {
            let mut tls = clif_tls::TlsCtx::new();
            let call_conv = self.module.isa().default_call_conv();
            let mut sig = Signature::new(call_conv);
            if std::env::var("NYASH_JIT_TRACE_SIG").ok().as_deref() == Some("1") {
                eprintln!(
                    "[SIG] begin desired: argc={} has_ret={} ret_is_f64={} typed_prepared={}",
                    self.desired_argc,
                    self.desired_has_ret,
                    self.desired_ret_is_f64,
                    self.typed_sig_prepared
                );
            }
            if !self.typed_sig_prepared {
                for _ in 0..self.desired_argc {
                    sig.params.push(AbiParam::new(types::I64));
                }
                if self.desired_has_ret {
                    if self.desired_ret_is_f64 {
                        sig.returns.push(AbiParam::new(types::F64));
                    } else {
                        sig.returns.push(AbiParam::new(types::I64));
                    }
                }
            } else {
                for _ in 0..self.desired_argc {
                    sig.params.push(AbiParam::new(types::I64));
                }
                if self.desired_has_ret {
                    let mut used_b1 = false;
                    #[cfg(feature = "jit-b1-abi")]
                    {
                        let cfg_now = crate::jit::config::current();
                        if crate::jit::config::probe_capabilities().supports_b1_sig
                            && cfg_now.native_bool_abi
                            && self.ret_hint_is_b1
                        {
                            sig.returns.push(AbiParam::new(types::B1));
                            used_b1 = true;
                        }
                    }
                    if !used_b1 {
                        if self.desired_ret_is_f64 {
                            sig.returns.push(AbiParam::new(types::F64));
                        } else {
                            sig.returns.push(AbiParam::new(types::I64));
                        }
                    }
                }
            }
            tls.ctx.func.signature = sig;
            tls.ctx.func.name = cranelift_codegen::ir::UserFuncName::user(0, 0);
            unsafe {
                tls.create();
            }
            tls.with(|fb| {
                if self.blocks.is_empty() {
                    let block = fb.create_block();
                    self.blocks.push(block);
                }
                if self.pending_blocks > self.blocks.len() {
                    let to_create = self.pending_blocks - self.blocks.len();
                    for _ in 0..to_create {
                        self.blocks.push(fb.create_block());
                    }
                }
                let entry = self.blocks[0];
                fb.append_block_params_for_function_params(entry);
                fb.switch_to_block(entry);
                self.entry_block = Some(entry);
                self.current_block_index = Some(0);
                self.cur_needs_term = true;
                // Force a dbg call at function entry to verify import linking works at runtime
                {
                    use cranelift_codegen::ir::{AbiParam, Signature};
                    let mut sig = Signature::new(self.module.isa().default_call_conv());
                    sig.params.push(AbiParam::new(types::I64));
                    sig.params.push(AbiParam::new(types::I64));
                    sig.returns.push(AbiParam::new(types::I64));
                    let fid = self
                        .module
                        .declare_function(
                            "nyash.jit.dbg_i64",
                            cranelift_module::Linkage::Import,
                            &sig,
                        )
                        .expect("declare dbg_i64 at entry");
                    let fref = self.module.declare_func_in_func(fid, fb.func);
                    let ttag = fb.ins().iconst(types::I64, 900);
                    let tval = fb.ins().iconst(types::I64, 123);
                    let _ = fb.ins().call(fref, &[ttag, tval]);
                }
                let rb = fb.create_block();
                self.ret_block = Some(rb);
                fb.append_block_param(rb, types::I64);
                self.blocks.push(rb);
                self.ret_slot = None;
            });
            cell.replace(Some(tls));
        });
    }
    fn end_function(&mut self) {
        use cranelift_module::Linkage;
        if self.entry_block.is_none() {
            return;
        }
        let mut ctx_opt: Option<cranelift_codegen::Context> = None;
        clif_tls::FB.with(|cell| {
            if let Some(mut tls) = cell.take() {
                tls.with(|fb| {
                    use cranelift_codegen::ir::types;
                    if let Some(rb) = self.ret_block {
                        if let Some(cur) = self.current_block_index {
                            if self.cur_needs_term && self.blocks[cur] != rb {
                                fb.ins().jump(rb, &[]);
                                self.cur_needs_term = false;
                            }
                        }
                        fb.switch_to_block(rb);
                        if fb.func.signature.returns.is_empty() {
                            fb.ins().return_(&[]);
                        } else {
                            // Prefer the persisted return slot if available; fallback to block param 0
                            let mut v = if let Some(ss) = self.ret_slot {
                                fb.ins().stack_load(types::I64, ss, 0)
                            } else {
                                let params = fb.func.dfg.block_params(rb).to_vec();
                                params
                                    .get(0)
                                    .copied()
                                    .unwrap_or_else(|| fb.ins().iconst(types::I64, 0))
                            };
                            // Unconditional runtime debug call to observe return value just before final return (feed result back)
                            {
                                use cranelift_codegen::ir::{AbiParam, Signature};
                                let mut sig = Signature::new(self.module.isa().default_call_conv());
                                sig.params.push(AbiParam::new(types::I64));
                                sig.params.push(AbiParam::new(types::I64));
                                sig.returns.push(AbiParam::new(types::I64));
                                let fid = self
                                    .module
                                    .declare_function("nyash.jit.dbg_i64", Linkage::Import, &sig)
                                    .expect("declare dbg_i64");
                                let fref = self.module.declare_func_in_func(fid, fb.func);
                                let tag = fb.ins().iconst(types::I64, 210);
                                let call_inst = fb.ins().call(fref, &[tag, v]);
                                if let Some(rv) = fb.inst_results(call_inst).get(0).copied() {
                                    v = rv;
                                }
                            }
                            let ret_ty = fb
                                .func
                                .signature
                                .returns
                                .get(0)
                                .map(|p| p.value_type)
                                .unwrap_or(types::I64);
                            if ret_ty == types::F64 {
                                v = fb.ins().fcvt_from_sint(types::F64, v);
                            }
                            fb.ins().return_(&[v]);
                        }
                    }
                    // Seal all blocks to satisfy CLIF verifier
                    for &b in &self.blocks {
                        fb.seal_block(b);
                    }
                });
                ctx_opt = Some(tls.take_context());
            }
        });
        if let Some(mut ctx) = ctx_opt.take() {
            let func_name = self.current_name.as_deref().unwrap_or("jit_func");
            let func_id = self
                .module
                .declare_function(func_name, Linkage::Local, &ctx.func.signature)
                .expect("declare function");
            if std::env::var("NYASH_JIT_TRACE_SIG").ok().as_deref() == Some("1") {
                eprintln!(
                    "[SIG] end returns={} params={}",
                    ctx.func.signature.returns.len(),
                    ctx.func.signature.params.len()
                );
            }
            if std::env::var("NYASH_JIT_DUMP_CLIF").ok().as_deref() == Some("1") {
                eprintln!("[CLIF] {}\n{}", func_name, ctx.func.display());
            }
            self.module
                .define_function(func_id, &mut ctx)
                .expect("define function");
            self.module.clear_context(&mut ctx);
            let _ = self.module.finalize_definitions();
            let code = self.module.get_finalized_function(func_id);
            // Build a callable closure capturing the code pointer
            let argc = self.desired_argc;
            let has_ret = self.desired_has_ret;
            let ret_is_f64 = self.desired_has_ret && self.desired_ret_is_f64;
            let code_usize = code as usize;
            unsafe {
                let closure = std::sync::Arc::new(
                    move |args: &[crate::jit::abi::JitValue]| -> crate::jit::abi::JitValue {
                        let mut a: [i64; 6] = [0; 6];
                        let take = core::cmp::min(core::cmp::min(argc, 6), args.len());
                        for i in 0..take {
                            a[i] = match args[i] {
                                crate::jit::abi::JitValue::I64(v) => v,
                                crate::jit::abi::JitValue::Bool(b) => {
                                    if b {
                                        1
                                    } else {
                                        0
                                    }
                                }
                                crate::jit::abi::JitValue::F64(f) => f as i64,
                                crate::jit::abi::JitValue::Handle(h) => h as i64,
                            };
                        }
                        let ret_i64 = if has_ret {
                            match argc {
                                0 => {
                                    let f: extern "C" fn() -> i64 = std::mem::transmute(code_usize);
                                    f()
                                }
                                1 => {
                                    let f: extern "C" fn(i64) -> i64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0])
                                }
                                2 => {
                                    let f: extern "C" fn(i64, i64) -> i64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1])
                                }
                                3 => {
                                    let f: extern "C" fn(i64, i64, i64) -> i64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1], a[2])
                                }
                                4 => {
                                    let f: extern "C" fn(i64, i64, i64, i64) -> i64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1], a[2], a[3])
                                }
                                5 => {
                                    let f: extern "C" fn(i64, i64, i64, i64, i64) -> i64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1], a[2], a[3], a[4])
                                }
                                _ => {
                                    let f: extern "C" fn(i64, i64, i64, i64, i64, i64) -> i64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1], a[2], a[3], a[4], a[5])
                                }
                            }
                        } else {
                            0
                        };
                        if has_ret && ret_is_f64 {
                            let ret_f64 = match argc {
                                0 => {
                                    let f: extern "C" fn() -> f64 = std::mem::transmute(code_usize);
                                    f()
                                }
                                1 => {
                                    let f: extern "C" fn(i64) -> f64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0])
                                }
                                2 => {
                                    let f: extern "C" fn(i64, i64) -> f64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1])
                                }
                                3 => {
                                    let f: extern "C" fn(i64, i64, i64) -> f64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1], a[2])
                                }
                                4 => {
                                    let f: extern "C" fn(i64, i64, i64, i64) -> f64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1], a[2], a[3])
                                }
                                5 => {
                                    let f: extern "C" fn(i64, i64, i64, i64, i64) -> f64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1], a[2], a[3], a[4])
                                }
                                _ => {
                                    let f: extern "C" fn(i64, i64, i64, i64, i64, i64) -> f64 =
                                        std::mem::transmute(code_usize);
                                    f(a[0], a[1], a[2], a[3], a[4], a[5])
                                }
                            };
                            if std::env::var("NYASH_JIT_TRACE_CALL").ok().as_deref() == Some("1") {
                                eprintln!("[JIT-CALL] ret_f64={}", ret_f64);
                            }
                            return crate::jit::abi::JitValue::F64(ret_f64);
                        }
                        if std::env::var("NYASH_JIT_TRACE_CALL").ok().as_deref() == Some("1") {
                            eprintln!("[JIT-CALL] ret_i64={}", ret_i64);
                        }
                        crate::jit::abi::JitValue::I64(ret_i64)
                    },
                );
                self.compiled_closure = Some(closure);
            }
        }
    }
}
