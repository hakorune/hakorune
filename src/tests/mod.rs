pub mod helpers;

#[cfg(feature = "aot-plan-import")]
pub mod aot_plan_import;
pub mod box_tests;
pub mod core13_smoke_array;
pub mod exec_parity;
// Legacy PHI-off flow shape tests (pre-JoinIR). Disable by default.
#[cfg(feature = "legacy-tests")]
pub mod flow;
pub mod functionbox_call_tests;
pub mod host_reverse_slot;
// Legacy PHI-off if/merge shape tests (pre-JoinIR). Disable by default.
#[cfg(feature = "legacy-tests")]
pub mod if_no_phi;
pub mod if_return_exec;
// Legacy StringUtils VM parity smoke (pre-JoinIR). Disable by default.
#[cfg(feature = "legacy-tests")]
pub mod json_lint_stringutils_min_vm; // Phase 21.7++: using StringUtils alias resolution fix
pub mod llvm_bitops_test;
pub mod macro_tests;
pub mod mir;
pub mod mir_direct_route_decode_escapes;
pub mod namingbox_static_method_id; // Phase 21.7++ Phase 1: StaticMethodId structure tests
pub mod nyash_abi_basic;
pub mod parser;
pub mod phase61_if_in_loop_dryrun; // Phase 61-2: If-in-loop JoinIR dry-run tests
pub mod phase67_generic_type_resolver; // Phase 67: P3-C GenericTypeResolver tests
pub mod plugin_hygiene;
pub mod policy_mutdeny;
pub mod refcell_assignment_test;
// Stage1 CLI SSA smoke (pre-JoinIR expectations). Disable by default.
#[cfg(feature = "legacy-tests")]
pub mod stage1_cli_entry_ssa_smoke;
pub mod sugar;
pub mod typebox_tlv_diff;
pub mod vm;
pub mod vtable;

// Phase 34-2: JoinIR Frontend (AST→JoinIR) and related components
pub mod joinir;

// Phase 40-3: array_ext.filter A/B test
pub mod phase40_array_ext_filter_test;

// Phase 256 P1.5: Select instruction minimal test
pub mod phase256_select_minimal_test;
