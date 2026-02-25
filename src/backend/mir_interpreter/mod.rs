/*!
 * Minimal MIR Interpreter
 *
 * Executes a subset of MIR instructions for fast iteration without LLVM/JIT.
 * Supported: Const, BinOp(Add/Sub/Mul/Div/Mod), Compare, Load/Store, Branch, Jump, Return,
 * Print/Debug (best-effort), Barrier/Safepoint (no-op).
 */

use rustc_hash::{FxHashMap, FxHashSet};
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};
use std::rc::Rc;

use crate::box_trait::NyashBox;

pub(super) use crate::backend::abi_util::{eq_vm, to_bool_vm};
pub(super) use crate::backend::vm::{VMError, VMValue};
pub(super) use crate::mir::{
    BasicBlockId, BinaryOp, Callee, CompareOp, ConstValue, MirFunction, MirInstruction, MirModule,
    TypeOpKind, ValueId, WeakRefOp,
};

mod exec;
mod handlers;
mod helpers;
mod method_router;
pub mod static_box_registry;
mod utils;

pub use static_box_registry::StaticBoxRegistry;

pub struct MirInterpreter {
    pub(super) regs: FxHashMap<ValueId, VMValue>,
    // VM fast-path dense register slots (bench/profile opt-in).
    pub(super) reg_fast_slots: Vec<Option<VMValue>>,
    // VM fast-path: integer-only copy alias map (dst -> canonical src).
    // This avoids repeated Integer clone/insert churn for Copy-heavy MIR.
    pub(super) reg_copy_aliases: FxHashMap<ValueId, ValueId>,
    // VM fast-path scalar caches (ValueId-indexed, lazily grown).
    pub(super) reg_i64_cache: Vec<Option<i64>>,
    pub(super) reg_bool_cache: Vec<Option<bool>>,
    pub(super) mem: FxHashMap<ValueId, VMValue>,
    // Object field storage keyed by stable object identity (Arc ptr addr fallback)
    pub(super) obj_fields: FxHashMap<u64, FxHashMap<String, VMValue>>,
    pub(super) functions: BTreeMap<String, MirFunction>,
    pub(super) cur_fn: Option<String>,
    // User-defined Box field declarations (compiler-provided; used for NewBox)
    pub(super) user_box_field_decls: FxHashMap<String, Vec<String>>,
    // Trace context (dev-only; enabled with NYASH_VM_TRACE=1)
    pub(super) last_block: Option<BasicBlockId>,
    pub(super) last_inst: Option<MirInstruction>,
    pub(super) last_inst_index: Option<usize>,
    // 🎯 Phase 173-B: Unified static box management via StaticBoxRegistry
    pub(super) static_box_registry: StaticBoxRegistry,
    // Lightweight dev counters (opt-in print via NYASH_VM_STATS=1)
    pub(super) inst_count: u64,
    pub(super) branch_count: u64,
    pub(super) compare_count: u64,
    // Cached runtime mode flags used by VM hot paths.
    pub(super) vm_fast_enabled: bool,
    pub(super) vm_fast_regfile_enabled: bool,
    pub(super) string_cp_mode: bool,
    pub(super) vm_tolerate_void_enabled: bool,
    pub(super) vm_null_missing_box_enabled: bool,
    pub(super) vm_box_trace_enabled: bool,
    pub(super) vm_provider_trace_enabled: bool,
    pub(super) vm_trace_enabled: bool,
    pub(super) joinir_debug_enabled: bool,
    pub(super) vm_trace_phi_enabled: bool,
    pub(super) vm_phi_tolerate_undefined_enabled: bool,
    pub(super) vm_phi_strict_enabled: bool,
    pub(super) vm_error_loc_enabled: bool,
    pub(super) vm_stats_enabled: bool,
    pub(super) vm_capture_last_inst_enabled: bool,
    // Cached operator-box availability flags (updated when function table is replaced).
    pub(super) operator_box_caps: OperatorBoxCaps,
    // Cached operator-box adopt toggles (avoid per-op env var reads on hot paths).
    pub(super) operator_box_add_adopt_enabled: bool,
    pub(super) operator_box_compare_adopt_enabled: bool,
    /// Call stack depth (exec_function_inner nesting). Used as a safety valve
    /// to prevent Rust stack overflow on accidental infinite recursion in MIR.
    pub(super) call_depth: usize,
    /// Call stack (dev-only; used for depth overflow diagnostics).
    pub(super) call_stack: Vec<String>,
    /// Recent step trace for budget diagnostics (debug-only).
    pub(super) recent_steps: VecDeque<StepTrace>,
    /// Phase 288.1: REPL session reference (REPL mode only)
    /// Enables variable persistence across REPL lines via __repl.get/set bridge
    pub(super) repl_session: Option<Rc<RefCell<crate::runner::repl::ReplSessionBox>>>,
    /// Dev-only preflight checks already performed for a function name.
    /// Avoids repeating verifier work on hot call paths.
    pub(super) preflight_checked_fns: FxHashSet<String>,
}

const VM_RECENT_STEP_LIMIT: usize = 32;

#[derive(Debug, Clone, Copy, Default)]
pub(super) struct OperatorBoxCaps {
    pub(super) add_apply: bool,
    pub(super) compare_apply: bool,
}

impl OperatorBoxCaps {
    fn from_functions(functions: &BTreeMap<String, MirFunction>) -> Self {
        Self {
            add_apply: functions.contains_key("AddOperator.apply/2"),
            compare_apply: functions.contains_key("CompareOperator.apply/3"),
        }
    }
}

#[derive(Debug, Clone)]
pub(super) struct StepTrace {
    pub(super) bb: BasicBlockId,
    pub(super) inst_idx: Option<usize>,
    pub(super) inst: Option<String>,
}

impl MirInterpreter {
    pub fn new() -> Self {
        Self {
            regs: FxHashMap::default(),
            reg_fast_slots: Vec::new(),
            reg_copy_aliases: FxHashMap::default(),
            reg_i64_cache: Vec::new(),
            reg_bool_cache: Vec::new(),
            mem: FxHashMap::default(),
            obj_fields: FxHashMap::default(),
            functions: BTreeMap::new(),
            cur_fn: None,
            user_box_field_decls: FxHashMap::default(),
            last_block: None,
            last_inst: None,
            last_inst_index: None,
            static_box_registry: StaticBoxRegistry::new(),
            inst_count: 0,
            branch_count: 0,
            compare_count: 0,
            vm_fast_enabled: crate::config::env::vm_fast_enabled(),
            vm_fast_regfile_enabled: crate::config::env::vm_fast_regfile_enabled(),
            string_cp_mode: crate::config::env::string_codepoint_mode(),
            vm_tolerate_void_enabled: crate::config::env::vm_tolerate_void_enabled(),
            vm_null_missing_box_enabled: crate::config::env::null_missing_box_enabled(),
            vm_box_trace_enabled: crate::config::env::vm_box_trace_enabled(),
            vm_provider_trace_enabled: crate::config::env::dev_provider_trace(),
            vm_trace_enabled: crate::config::env::vm_trace_enabled(),
            joinir_debug_enabled: crate::config::env::joinir_dev::debug_enabled(),
            vm_trace_phi_enabled: crate::config::env::vm_trace_phi_enabled(),
            vm_phi_tolerate_undefined_enabled:
                crate::config::env::vm_phi_tolerate_undefined_enabled(),
            vm_phi_strict_enabled: crate::config::env::vm_phi_strict_enabled(),
            vm_error_loc_enabled: crate::config::env::vm_error_loc_enabled(),
            vm_stats_enabled: crate::config::env::vm_stats_enabled(),
            vm_capture_last_inst_enabled: crate::config::env::vm_capture_last_inst_enabled(),
            operator_box_caps: OperatorBoxCaps::default(),
            operator_box_add_adopt_enabled: crate::config::env::operator_box_add_adopt(),
            operator_box_compare_adopt_enabled: crate::config::env::operator_box_compare_adopt(),
            call_depth: 0,
            call_stack: Vec::new(),
            recent_steps: VecDeque::new(),
            repl_session: None,
            preflight_checked_fns: FxHashSet::default(),
        }
    }

    fn refresh_operator_box_flags(&mut self) {
        self.operator_box_caps = OperatorBoxCaps::from_functions(&self.functions);
    }

    /// Return (inst_count, branch_count, compare_count)
    pub fn stats_counters(&self) -> (u64, u64, u64) {
        (self.inst_count, self.branch_count, self.compare_count)
    }

    fn is_strong_root_value(v: &VMValue) -> bool {
        matches!(v, VMValue::BoxRef(_) | VMValue::Future(_))
    }

    /// Count strong temp roots currently held by VM registers.
    ///
    /// This is used by runtime observability to report `temps` category.
    pub fn strong_temp_root_count(&self) -> usize {
        let mut count = self
            .reg_fast_slots
            .iter()
            .filter_map(|v| v.as_ref())
            .filter(|v| Self::is_strong_root_value(v))
            .count();

        count += self
            .regs
            .iter()
            .filter(|(id, v)| {
                let idx = id.as_u32() as usize;
                let shadowed_by_slot = idx < self.reg_fast_slots.len() && self.reg_fast_slots[idx].is_some();
                !shadowed_by_slot && Self::is_strong_root_value(v)
            })
            .count();
        count
    }

    /// Count strong roots held by object fields.
    ///
    /// This is used by runtime observability to report `heap_fields` category.
    pub fn strong_heap_field_root_count(&self) -> usize {
        self.obj_fields
            .values()
            .flat_map(|fields| fields.values())
            .filter(|v| Self::is_strong_root_value(v))
            .count()
    }

    /// Register static box declarations (called from vm.rs during setup)
    /// Now delegated to StaticBoxRegistry
    pub fn register_static_box_decl(
        &mut self,
        name: String,
        decl: crate::core::model::BoxDeclaration,
    ) {
        self.static_box_registry.register_declaration(name, decl);
    }

    /// Check if a static box is already registered (Phase 173-B)
    /// Now delegated to StaticBoxRegistry
    pub fn has_static_box_decl(&self, name: &str) -> bool {
        self.static_box_registry.exists(name)
    }

    /// Initialize registry from MIR function names (auto-detect using imports)
    /// Called from vm.rs after module is loaded
    pub fn detect_static_boxes_from_functions(&mut self) {
        self.static_box_registry
            .detect_from_mir_functions(self.functions.keys());
    }

    /// Phase 288.1: Set REPL session for variable persistence
    /// Enables __repl.get/set ExternCall handlers to access session state
    pub fn set_repl_session(&mut self, session: Rc<RefCell<crate::runner::repl::ReplSessionBox>>) {
        self.repl_session = Some(session);
    }

    /// Execute a BoxCall with VM's complete semantics (Phase 27-shortterm S-5.2-improved)
    ///
    /// This wrapper allows external modules (e.g., JoinIR Runner) to invoke BoxCall
    /// with the VM's complete semantics including:
    /// - Void guards (e.g., Void.length() → 0)
    /// - PluginBox support (FileBox, NetBox, etc.)
    /// - InstanceBox policy checks
    /// - object_fields handling
    /// - Method re-routing (toString→str)
    ///
    /// # Implementation Notes
    /// - Uses 1_000_000 register range for scratch registers (避ける ID 衝突)
    /// - Properly cleans up temporary registers after use
    /// - Delegates to `handle_box_call` for complete VM semantics
    ///
    /// # Arguments
    /// - `receiver`: The box instance (VMValue::BoxRef or primitive)
    /// - `method`: Method name to invoke
    /// - `args`: Method arguments as VMValue
    ///
    /// # Returns
    /// Result value as VMValue (may be Void, Int, String, BoxRef, etc.)
    pub fn execute_box_call(
        &mut self,
        receiver: VMValue,
        method: &str,
        args: Vec<VMValue>,
    ) -> Result<VMValue, VMError> {
        // Allocate temporary register IDs in the 1_000_000 range (not 1000!)
        // This avoids conflicts with user code and future extensions
        let base = ValueId(1_000_000);
        let recv_id = base;

        // Place receiver in register
        self.write_reg(recv_id, receiver);

        // Place arguments in consecutive registers
        let arg_ids: Vec<ValueId> = args
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                let id = ValueId(base.0 + 1 + i as u32);
                self.write_reg(id, v);
                id
            })
            .collect();

        // Allocate destination register
        let dst_id = ValueId(base.0 + 1000);

        // Invoke handle_box_call for complete VM semantics
        self.handle_box_call(Some(dst_id), recv_id, method, &arg_ids)?;

        // Read result (may be Void if method returns nothing)
        let result = self.take_reg(dst_id).unwrap_or(VMValue::Void);

        // Cleanup temporary registers (important to avoid stale values!)
        let _ = self.take_reg(recv_id);
        for id in arg_ids {
            let _ = self.take_reg(id);
        }

        Ok(result)
    }

    /// Ensure static box singleton instance exists, create if not
    /// Returns mutable reference to the singleton instance
    /// Now delegated to StaticBoxRegistry
    fn ensure_static_box_instance(
        &mut self,
        box_name: &str,
    ) -> Result<&mut crate::instance_v2::InstanceBox, VMError> {
        self.static_box_registry
            .get_or_create_instance(box_name)
            .map_err(|e| VMError::InvalidInstruction(e))
    }

    /// Check if a function name represents a static box method
    /// Format: "BoxName.method/Arity"
    /// Now uses StaticBoxRegistry naming utilities
    #[allow(dead_code)]
    fn is_static_box_method(&self, func_name: &str) -> Option<String> {
        static_box_registry::naming::extract_box_name(func_name)
            .filter(|name| self.static_box_registry.exists(name))
    }

    /// Execute module entry (main) and return boxed result
    pub fn execute_module(&mut self, module: &MirModule) -> Result<Box<dyn NyashBox>, VMError> {
        crate::runtime::leak_tracker::reset_observed_roots();

        // Snapshot functions for call resolution
        self.functions = module.functions.clone();
        self.refresh_operator_box_flags();
        self.user_box_field_decls = module.metadata.user_box_decls.clone().into_iter().collect();

        // 🎯 Phase 173-B: Auto-detect static boxes from MIR function names
        // This handles using-imported static boxes that aren't in AST
        self.detect_static_boxes_from_functions();

        // Determine entry function with sensible fallbacks (arity-aware, Strangler-safe).
        //
        // Priority (SSOT-ish for VM backend):
        //   1) NYASH_ENTRY env (exact; if arity-less, try auto-match)
        //   2) Main.main/0
        //   3) Main.main
        //   4) main (top-level; only if NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1)
        let mut candidates: Vec<String> = Vec::new();
        if let Ok(e) = std::env::var("NYASH_ENTRY") {
            let entry = e.trim();
            if !entry.is_empty() {
                candidates.push(entry.to_string());
                if !entry.contains('/') {
                    candidates.push(format!("{}/0", entry));
                    candidates.push(format!("{}/1", entry));
                }
            }
        }
        candidates.push(crate::mir::naming::encode_static_method("Main", "main", 0));
        candidates.push("Main.main".to_string());
        if crate::config::env::entry_allow_toplevel_main() {
            candidates.push("main".to_string());
        }

        let mut chosen: Option<&nyash_rust::mir::MirFunction> = None;
        let mut chosen_name: Option<String> = None;
        for c in &candidates {
            if let Some(f) = module.functions.get(c) {
                chosen = Some(f);
                chosen_name = Some(c.clone());
                break;
            }
        }

        let func = match chosen {
            Some(f) => f,
            None => {
                let mut names: Vec<&String> = module.functions.keys().collect();
                names.sort();
                let avail = names
                    .into_iter()
                    .take(12)
                    .cloned()
                    .collect::<Vec<_>>()
                    .join(", ");
                let tried = candidates.join(", ");
                let msg = format!(
                    "entry function not found. searched: [{}]. available: [{}]. hint: define 'static box Main {{ method main(args){{ ... }} }}' or set NYASH_ENTRY=Name",
                    tried, avail
                );
                return Err(VMError::InvalidInstruction(msg));
            }
        };

        if std::env::var("NYASH_EMIT_MIR_TRACE").ok().as_deref() == Some("1") {
            if let Some(name) = chosen_name {
                crate::runtime::get_global_ring0()
                    .log
                    .debug(&format!("[vm/entry] main={}", name));
            }
        }

        // Prepare arguments if the entry takes parameters (pass script args as ArrayBox)
        let ret = if func.signature.params.len() == 0 {
            self.execute_function(func)?
        } else {
            // Build argv from (priority) HEX JSON, normal JSON, or NYASH_ARGV
            // 1) NYASH_SCRIPT_ARGS_HEX_JSON: JSON array of hex-encoded UTF-8 strings
            // 2) NYASH_SCRIPT_ARGS_JSON: JSON array of strings
            // 3) NYASH_ARGV: JSON array (legacy)
            let mut argv_list: Vec<String> = Vec::new();
            if let Ok(s) = std::env::var("NYASH_SCRIPT_ARGS_HEX_JSON") {
                if let Ok(v) = serde_json::from_str::<Vec<String>>(&s) {
                    let mut out = Vec::with_capacity(v.len());
                    for hx in v.into_iter() {
                        match hex_decode_to_string(&hx) {
                            Ok(ss) => out.push(ss),
                            Err(_) => out.push(String::new()),
                        }
                    }
                    argv_list = out;
                }
            } else if let Ok(s) = std::env::var("NYASH_SCRIPT_ARGS_JSON") {
                if let Ok(v) = serde_json::from_str::<Vec<String>>(&s) {
                    argv_list = v;
                }
            } else if let Ok(s) = std::env::var("NYASH_ARGV") {
                if let Ok(v) = serde_json::from_str::<Vec<String>>(&s) {
                    argv_list = v;
                }
            }
            // Construct ArrayBox of StringBox
            let array = crate::boxes::array::ArrayBox::new();
            for a in argv_list.iter() {
                let sb = crate::boxes::basic::StringBox::new(a);
                let _ = array.push(Box::new(sb));
            }
            let boxed: Box<dyn crate::box_trait::NyashBox> = Box::new(array);
            let arg0 = super::vm_types::VMValue::from_nyash_box(boxed);
            // Fill remaining params with Void
            let mut vm_args: Vec<super::vm_types::VMValue> = Vec::new();
            vm_args.push(arg0);
            for _ in 1..func.signature.params.len() {
                vm_args.push(super::vm_types::VMValue::Void);
            }
            self.exec_function_inner(func, Some(&vm_args))?
        };
        Ok(ret.to_nyash_box())
    }

    /// Execute a specific function with explicit arguments (bypasses entry discovery).
    pub fn execute_function_with_args(
        &mut self,
        module: &MirModule,
        func_name: &str,
        args: &[VMValue],
    ) -> Result<VMValue, VMError> {
        // Snapshot functions for call resolution
        self.functions = module.functions.clone();
        self.refresh_operator_box_flags();

        let func = self
            .functions
            .get(func_name)
            .ok_or_else(|| {
                VMError::InvalidInstruction(format!("function not found: {}", func_name))
            })?
            .clone();

        self.exec_function_inner(&func, Some(args))
    }

    fn execute_function(&mut self, func: &MirFunction) -> Result<VMValue, VMError> {
        self.exec_function_inner(func, None)
    }
}

fn hex_decode_to_string(hex: &str) -> Result<String, ()> {
    let mut bytes: Vec<u8> = Vec::with_capacity(hex.len() / 2);
    let mut it = hex.as_bytes().iter().cloned();
    while let (Some(h), Some(l)) = (it.next(), it.next()) {
        let hi = from_hex(h).ok_or(())?;
        let lo = from_hex(l).ok_or(())?;
        bytes.push((hi << 4) | lo);
    }
    match String::from_utf8(bytes) {
        Ok(s) => Ok(s),
        Err(e) => Ok(String::from_utf8_lossy(e.as_bytes()).into_owned()),
    }
}

fn from_hex(b: u8) -> Option<u8> {
    match b {
        b'0'..=b'9' => Some(b - b'0'),
        b'a'..=b'f' => Some(b - b'a' + 10),
        b'A'..=b'F' => Some(b - b'A' + 10),
        _ => None,
    }
}
