#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="naming-charter-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SSOT="$ROOT_DIR/docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md"
STAGE_TERM_INVENTORY="$ROOT_DIR/docs/development/current/main/design/hakorune-stage-term-existing-name-migration-inventory.md"
CHECK_INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
QUICK_STEPS="$ROOT_DIR/tools/checks/lib/dev_gate_quick_steps.sh"
DOCS_LAYOUT="$ROOT_DIR/docs/development/current/main/DOCS_LAYOUT.md"
CARGO_TOML="$ROOT_DIR/Cargo.toml"
README_MD="$ROOT_DIR/README.md"
CLI_ARGS_RS="$ROOT_DIR/src/cli/args.rs"
CLI_ARGS_TESTS_RS="$ROOT_DIR/src/cli/args/tests.rs"
HACO_WRAPPER="$ROOT_DIR/tools/bin/hako"
MAIN_RS="$ROOT_DIR/src/main.rs"
HAKORUNE_BIN_RS="$ROOT_DIR/src/bin/hakorune.rs"
HAKORUNE_COMPAT_BIN_RS="$ROOT_DIR/src/bin/hakorune_compat.rs"
LANG_README="$ROOT_DIR/lang/README.md"
BUILD_SHARED_RS="$ROOT_DIR/src/runner/build_shared.rs"
BUILD_PRODUCT_RS="$ROOT_DIR/src/runner/build_product.rs"
BUILD_ENGINEERING_RS="$ROOT_DIR/src/runner/build_engineering.rs"
MIR_BUILDER_BUILD_RS="$ROOT_DIR/src/mir/builder/builder_build.rs"
WINDOWS_DIR="$ROOT_DIR/tools/windows"
HAKO_CHECK_SH="$ROOT_DIR/tools/hako-check/hako-check.sh"
BUILD_LLVM_PS="$ROOT_DIR/tools/build_llvm.ps1"
BUILD_AOT_PS="$ROOT_DIR/tools/build_aot.ps1"
USING_UNRESOLVED_SMOKE="$ROOT_DIR/tools/using_unresolved_smoke.sh"
USING_RESOLVE_SMOKE="$ROOT_DIR/tools/using_resolve_smoke.sh"
USING_STRICT_PATH_FAIL_SMOKE="$ROOT_DIR/tools/using_strict_path_fail_smoke.sh"
DEV_SELFHOST_LOOP="$ROOT_DIR/tools/dev_selfhost_loop.sh"
ENGINEERING_PARITY="$ROOT_DIR/tools/engineering/parity.sh"
SELFHOST_EXE_STAGEB="$ROOT_DIR/tools/selfhost_exe_stageb.sh"
HAKORUNE_EMIT_MIR="$ROOT_DIR/tools/hakorune_emit_mir.sh"
STAGEB_PROGRAM_JSON_CAPTURE_CALLER_GUARD="$ROOT_DIR/tools/checks/stageb_program_json_capture_caller_guard.sh"
STAGE1_EMIT_PROGRAM_JSON_RUNTIME_HELPER_GUARD="$ROOT_DIR/tools/checks/stage1_emit_program_json_runtime_helper_guard.sh"
STAGE1_PROGRAM_JSON_COMPAT_CALLER_GUARD="$ROOT_DIR/tools/checks/stage1_program_json_compat_caller_guard.sh"
SELFHOST_STAGEB_PROOF_VM="$ROOT_DIR/tools/selfhost/proof/run_stageb_compiler_vm.sh"
SELFHOST_RUN_ROUTES="$ROOT_DIR/tools/selfhost/lib/selfhost_run_routes.sh"
SELFHOST_BUILD="$ROOT_DIR/tools/selfhost/selfhost_build.sh"
SELFHOST_README="$ROOT_DIR/tools/selfhost/README.md"
SELFHOST_QUICKSTART="$ROOT_DIR/docs/development/selfhosting/quickstart.md"
SELFHOST_MAINLINE_BUILD_STAGE1="$ROOT_DIR/tools/selfhost/mainline/build_stage1.sh"
SELFHOST_STAGE_A_SPAWN_RS="$ROOT_DIR/src/runner/modes/common_util/selfhost/stage_a_spawn.rs"
SELFHOST_STAGE_A_COMPAT_BRIDGE_RS="$ROOT_DIR/src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs"
SELFHOST_STAGE_A_ROUTE_RS="$ROOT_DIR/src/runner/modes/common_util/selfhost/stage_a_route.rs"
SELFHOST_STAGE_A_POLICY_RS="$ROOT_DIR/src/runner/modes/common_util/selfhost/stage_a_policy.rs"
SELFHOST_COMMON_JSON_RS="$ROOT_DIR/src/runner/modes/common_util/selfhost/json.rs"
SELFHOST_STAGE0_CAPTURE_RS="$ROOT_DIR/src/runner/modes/common_util/selfhost/stage0_capture.rs"
SELFHOST_STAGE0_CAPTURE_ROUTE_RS="$ROOT_DIR/src/runner/modes/common_util/selfhost/stage0_capture_route.rs"
RUNNER_SELFHOST_RS="$ROOT_DIR/src/runner/selfhost.rs"
HH_COMPILER_README="$ROOT_DIR/lang/src/compiler/README.md"
STAGE1_BRIDGE_README="$ROOT_DIR/src/runner/stage1_bridge/README.md"
STAGE1_BRIDGE_ENV_RS="$ROOT_DIR/src/runner/stage1_bridge/env.rs"
STAGE1_BRIDGE_MODULES_RS="$ROOT_DIR/src/runner/stage1_bridge/modules.rs"
HH_COMPILER_ENTRY="$ROOT_DIR/lang/src/compiler/entry/compiler.hako"
HH_COMPILER_STAGEB_ENTRY="$ROOT_DIR/lang/src/compiler/entry/compiler_stageb.hako"
HH_STAGEB_ARGS="$ROOT_DIR/lang/src/compiler/entry/stageb_args_box.hako"
HH_STAGEB_BUILD_OPTIONS="$ROOT_DIR/lang/src/compiler/entry/stageb_build_options_box.hako"
HH_STAGEB_COMPILE_ADAPTER="$ROOT_DIR/lang/src/compiler/entry/stageb_compile_adapter_box.hako"
HH_STAGEB_OUTPUT="$ROOT_DIR/lang/src/compiler/entry/stageb_output_box.hako"
HH_BUNDLE_RESOLVER="$ROOT_DIR/lang/src/compiler/entry/bundle_resolver.hako"
HH_STAGEB_BODY_EXTRACTOR="$ROOT_DIR/lang/src/compiler/entry/stageb_body_extractor_box.hako"
HH_STAGEB_KEYWORD_EXPR_STRIP="$ROOT_DIR/lang/src/compiler/entry/stageb_keyword_expr_strip_box.hako"
HH_STAGEB_DRIVER_GUARD="$ROOT_DIR/lang/src/compiler/entry/stageb_driver_guard_box.hako"
HH_STAGEB_TRACE="$ROOT_DIR/lang/src/compiler/entry/stageb_trace_box.hako"
HH_STAGEB_MAIN_DETECTION="$ROOT_DIR/lang/src/compiler/entry/stageb_main_detection_box.hako"
HH_STAGEB_RUNE="$ROOT_DIR/lang/src/compiler/entry/stageb/stageb_rune_box.hako"
HH_STAGEB_USER_BOX_DECL_SCANNER="$ROOT_DIR/lang/src/compiler/entry/stageb/stageb_user_box_decl_scanner_box.hako"
HH_FUNC_SCANNER="$ROOT_DIR/lang/src/compiler/entry/func_scanner.hako"
HH_FUNC_SCANNER_HELPERS="$ROOT_DIR/lang/src/compiler/entry/func_scanner_helpers.hako"
HH_BUILD_README="$ROOT_DIR/lang/src/compiler/build/README.md"
HH_BUILD_BUNDLE_FACADE="$ROOT_DIR/lang/src/compiler/build/build_bundle_facade_box.hako"
HH_MIRBUILDER_README="$ROOT_DIR/lang/src/compiler/mirbuilder/README.md"
HH_PARSER_BOX="$ROOT_DIR/lang/src/compiler/parser/parser_box.hako"
HH_PARSER_EXPR_BOX="$ROOT_DIR/lang/src/compiler/parser/expr/parser_expr_box.hako"
HH_PARSER_NUMBER_SCAN="$ROOT_DIR/lang/src/compiler/parser/scan/parser_number_scan_box.hako"
HH_PARSER_RUNE_CONTRACT="$ROOT_DIR/lang/src/compiler/parser/rune/rune_contract_box.hako"
HH_PARSER_CONTROL_BOX="$ROOT_DIR/lang/src/compiler/parser/stmt/parser_control_box.hako"
HH_PARSER_EXCEPTION_BOX="$ROOT_DIR/lang/src/compiler/parser/stmt/parser_exception_box.hako"
HH_PARSER_STMT_CORE="$ROOT_DIR/lang/src/compiler/parser/stmt/parser_stmt_box/core.hako"
HH_TEST_FUNCSCANNER_SKIP_WS="$ROOT_DIR/lang/src/compiler/tests/funcscanner_skip_ws_min.hako"
HH_TEST_STAGEB_MIN_SAMPLE="$ROOT_DIR/lang/src/compiler/tests/stageb_min_sample.hako"
K2_WIDE_STAGEB_FIELD_TYPE_ANNOTATION_GUARD="$ROOT_DIR/tools/checks/k2_wide_stageb_field_type_annotation_alignment_guard.sh"
K2_WIDE_STAGEB_NUMERIC_LITERAL_SUFFIX_GUARD="$ROOT_DIR/tools/checks/k2_wide_stageb_numeric_literal_suffix_alignment_guard.sh"
NY_PARSER_BRIDGE_SMOKE="$ROOT_DIR/tools/ny_parser_bridge_smoke.sh"
PHI_TRACE_RUN="$ROOT_DIR/tools/debug/phi/phi_trace_run.sh"
TEST_SHLIB="$ROOT_DIR/tools/test/lib/shlib.sh"
EMIT_MIR_ROUTE="$ROOT_DIR/tools/smokes/v2/lib/emit_mir_route.sh"
BRIDGE_CANON_DIR="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/bridge"
HAKO_MIN_BINOP_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/hako_min_binop_vm.sh"
HAKO_MIN_IF_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/hako_min_if_vm.sh"
HAKO_MIN_INDEX_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/index_operator_hako.sh"
HAKO_MIN_COMPILE_RETURN_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/hako_min_compile_return_vm.sh"
HAKO_MAP_ESCAPE_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/hako_map_escape_vm.sh"
STAGEB_HELPERS="$ROOT_DIR/tools/smokes/v2/lib/stageb_helpers.sh"
GATE_C_V1_FILE_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/gate_c_v1_file_vm.sh"
NYVM_WRAPPER_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/nyvm_wrapper_module_json_vm.sh"
FASTMEM_PARSER_PARITY_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/parser/fastmem_parser_parity_smoke.sh"
PARSER_OPT_ANNOTATIONS_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh"
PARSER_TRY_COMPAT_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/parser/parser_try_compat_boundary.sh"
PARSER_MIN_METHODS_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/parser/parser_min_methods_ok.sh"
PARSER_RUNE_DECL_TRACE_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh"
GATE_C_OOB_STRICT_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/core/gate_c_oob_strict_fail_vm.sh"
NY_MIR_BUILDER="$ROOT_DIR/tools/ny_mir_builder.sh"
PHASE29X_L1_CACHE="$ROOT_DIR/tools/cache/phase29x_l1_mir_cache.sh"
PHASE29X_L2_CACHE="$ROOT_DIR/tools/cache/phase29x_l2_object_cache.sh"
SMOKE_PREFLIGHT="$ROOT_DIR/tools/smokes/v2/lib/preflight.sh"
SMOKE_PLUGIN_MANAGER="$ROOT_DIR/tools/smokes/v2/lib/plugin_manager.sh"
SMOKE_TEST_RUNNER="$ROOT_DIR/tools/smokes/v2/lib/test_runner.sh"
SMOKE_AUTO_DETECT_CONF="$ROOT_DIR/tools/smokes/v2/configs/auto_detect.conf"
FASTMEM_SOURCE_SYNTAX_SMOKE="$ROOT_DIR/tools/hako_check/fastmem_source_syntax_smoke.sh"
FASTMEM_TERMINAL_LADDER_SMOKE="$ROOT_DIR/tools/hako_check/fastmem_terminal_ladder_smoke.sh"
FASTMEM_SOURCE_MANIFEST_RUNNER="$ROOT_DIR/tools/hako_check/fastmem_source_manifest_runner.py"
SELFHOST_JSON_V0_TRY_CATCH_CANARY="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_json_v0_try_catch_cleanup_canary_vm.sh"
SELFHOST_STABLE_PATHS_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase120_stable_paths.sh"
SELFHOST_PLANNER_REQUIRED_DEV_GATE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh"
SELFHOST_STAGE1_CONTRACT_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stage1_contract_smoke_vm.sh"
SELFHOST_STAGEB_FUNCSCANNER_BOX_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh"
SELFHOST_STAGEB_FUNCSCANNER_METHOD_BOUNDARY_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh"
SELFHOST_STAGEB_LAMBDA_LITERAL_PAIR_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_lambda_literal_pair_min_vm.sh"
SELFHOST_STAGEB_ROUTE_PARITY_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh"
SELFHOST_STEADY_STATE_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh"
SELFHOST_STAGEB_FUNCSCANNER_TYPED_PARAMS_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29cc_selfhost_stageb_funcscanner_typed_params_implements_min_vm.sh"
APP_BINARY_ONLY_SELFHOST_READINESS_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh"
APP_BINARY_ONLY_RUN_PORTED_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh"
APP_NO_COMPAT_MAINLINE_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh"
APP_PERF_COMPILE_RUN_SPLIT_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/phase21_5_perf_bench_compile_run_split_contract_vm.sh"
APP_SMOKE_LIB_README="$ROOT_DIR/tools/smokes/v2/profiles/integration/apps/lib/README.md"
COLLECTION_MAP_GET_SHARES_MAP_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/collections/map_get_shares_map.sh"
COLLECTION_MAP_GET_SHARES_ARRAY_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/collections/map_get_shares_array.sh"
COLLECTION_STRING_SIZE_ALIAS_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/collections/string_size_alias.sh"
GOLDEN_MACRO_DIR="$ROOT_DIR/tools/test/golden/macro"
GOLDEN_MACRO_RESOLVER="$ROOT_DIR/tools/test/golden/macro/lib/resolve_hakorune.sh"
ENV_RS="$ROOT_DIR/src/config/env.rs"
STAGE1_ENV_RS="$ROOT_DIR/src/config/env/stage1.rs"
PARSER_FLAGS_RS="$ROOT_DIR/src/config/env/parser_flags.rs"
SELFHOST_FLAGS_RS="$ROOT_DIR/src/config/env/selfhost_flags.rs"
ENV_PATHS_RS="$ROOT_DIR/src/config/env/paths.rs"
VERIFICATION_FLAGS_RS="$ROOT_DIR/src/config/env/verification_flags.rs"
ENV_DOC="$ROOT_DIR/docs/reference/environment-variables.md"
STAGE1_BRIDGE_ENV_PARSER_STAGEB_RS="$ROOT_DIR/src/runner/stage1_bridge/env/parser_stageb.rs"
RUST_STAGE1_README="$ROOT_DIR/src/stage1/README.md"
RUST_STAGE1_MOD_RS="$ROOT_DIR/src/stage1/mod.rs"
RUST_STAGE1_PROGRAM_JSON_RS="$ROOT_DIR/src/stage1/program_json_v0.rs"
RUST_STAGE1_PROGRAM_JSON_ROUTING_RS="$ROOT_DIR/src/stage1/program_json_v0/routing.rs"
RUST_STAGE1_PROGRAM_JSON_README="$ROOT_DIR/src/stage1/program_json_v0/README.md"
REFERENCE_EBNF="$ROOT_DIR/docs/reference/language/EBNF.md"
REFERENCE_STATEMENTS="$ROOT_DIR/docs/reference/language/statements.md"
PHASE29CI_STAGEB_BODY_EXTRACT_TEST="$ROOT_DIR/tests/phase29ci_stageb_body_extract.rs"
JSON_V0_BRIDGE_AST_RS="$ROOT_DIR/src/runner/json_v0_bridge/ast.rs"
JSON_V0_BRIDGE_IF_LEGACY_RS="$ROOT_DIR/src/runner/json_v0_bridge/lowering/if_legacy.rs"
JSON_V0_BRIDGE_LAMBDA_LEGACY_RS="$ROOT_DIR/src/runner/json_v0_bridge/lowering/lambda_legacy.rs"
JSON_V0_BRIDGE_LOOP_RS="$ROOT_DIR/src/runner/json_v0_bridge/lowering/loop_.rs"
JSON_V0_BRIDGE_LOOP_RANGE_RS="$ROOT_DIR/src/runner/json_v0_bridge/lowering/loop_range.rs"
JSON_V0_BRIDGE_PROGRAM_RS="$ROOT_DIR/src/runner/json_v0_bridge/lowering/program.rs"
JSON_V0_BRIDGE_BLOCK_EXPR_RS="$ROOT_DIR/src/runner/json_v0_bridge/lowering/expr/block_expr.rs"
PIPELINE_V2_STAGE_COMMENT_FILES=(
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/README.md"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/alias_preflight_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/call_extract_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/compare_extract_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/execution_pipeline_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/flow_entry.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/header_emit_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/method_extract_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/mir_builder_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/new_extract_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/pipeline.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/pipeline_helpers_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/signature_verifier_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/stage1_args_parser_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/stage1_extract_flow.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/stage1_json_scanner_box.hako"
  "$ROOT_DIR/lang/src/compiler/pipeline_v2/stage1_name_args_normalizer_box.hako"
)
JOINIR_LOWERING_STAGE_COMMENT_FILES=(
  "$ROOT_DIR/src/mir/join_ir/lowering/value_id_ranges.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/mod.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/if_lowering_router.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/common/cfg_shape.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/generic_case_a/stage1_using_resolver.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/generic_case_a/mod.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/loop_to_join/case_a_entrypoints.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/stage1_using_resolver/dispatch.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/stage1_using_resolver.rs"
  "$ROOT_DIR/src/mir/join_ir/lowering/loop_view_builder.rs"
)
STAGE1_BRIDGE_PHASE_COMMENT_FILES=(
  "$ROOT_DIR/src/runner/stage1_bridge/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/mod.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/args.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/env.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/modules.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/entry_guard.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/emit_paths.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/plan.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/route_exec.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/route_exec/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/route_exec/direct.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/route_exec/stub.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/direct_route/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/direct_route/mod.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/direct_route/compile.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/direct_route/emit.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/env/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/env/parser_stageb.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/env/runtime_defaults.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/env/stage1_aliases.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/program_json/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/program_json/mod.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/program_json_entry/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_child.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_delegate.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_emit.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_emit/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_emit/parse.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_emit/writeback.rs"
)
STAGE1_BRIDGE_PHASE_COMMENT_REQUIRED_FILES=(
  "$ROOT_DIR/src/runner/stage1_bridge/mod.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/args.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/entry_guard.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/emit_paths.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/plan.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/route_exec.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/route_exec/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/route_exec/direct.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/route_exec/stub.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/direct_route/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/direct_route/mod.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/direct_route/compile.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/direct_route/emit.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/env/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/env/runtime_defaults.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/env/stage1_aliases.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/program_json/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/program_json/mod.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/program_json_entry/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_child.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_delegate.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_emit.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_emit/README.md"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_emit/parse.rs"
  "$ROOT_DIR/src/runner/stage1_bridge/stub_emit/writeback.rs"
)

guard_require_command "$TAG" rg
guard_require_command "$TAG" git

guard_require_unique_values() {
  local label="$1"
  shift
  declare -A seen_values=()
  local value
  for value in "$@"; do
    if [[ -n "${seen_values[$value]:-}" ]]; then
      guard_fail "$TAG" "duplicate $label in guard: $value"
    fi
    seen_values[$value]=1
  done
}

REQUIRED_FILES=(
  "$SSOT"
  "$STAGE_TERM_INVENTORY"
  "$CHECK_INDEX"
  "$QUICK_STEPS"
  "$DOCS_LAYOUT"
  "$CARGO_TOML"
  "$README_MD"
  "$CLI_ARGS_RS"
  "$CLI_ARGS_TESTS_RS"
  "$HACO_WRAPPER"
  "$MAIN_RS"
  "$HAKORUNE_BIN_RS"
  "$HAKORUNE_COMPAT_BIN_RS"
  "$LANG_README"
  "$BUILD_SHARED_RS"
  "$BUILD_PRODUCT_RS"
  "$BUILD_ENGINEERING_RS"
  "$MIR_BUILDER_BUILD_RS"
  "$HAKO_CHECK_SH"
  "$BUILD_LLVM_PS"
  "$BUILD_AOT_PS"
  "$USING_UNRESOLVED_SMOKE"
  "$USING_RESOLVE_SMOKE"
  "$USING_STRICT_PATH_FAIL_SMOKE"
  "$DEV_SELFHOST_LOOP"
  "$ENGINEERING_PARITY"
  "$SELFHOST_EXE_STAGEB"
  "$HAKORUNE_EMIT_MIR"
  "$STAGEB_PROGRAM_JSON_CAPTURE_CALLER_GUARD"
  "$STAGE1_EMIT_PROGRAM_JSON_RUNTIME_HELPER_GUARD"
  "$STAGE1_PROGRAM_JSON_COMPAT_CALLER_GUARD"
  "$SELFHOST_STAGEB_PROOF_VM"
  "$SELFHOST_RUN_ROUTES"
  "$SELFHOST_BUILD"
  "$SELFHOST_README"
  "$SELFHOST_QUICKSTART"
  "$SELFHOST_MAINLINE_BUILD_STAGE1"
  "$SELFHOST_STAGE_A_SPAWN_RS"
  "$SELFHOST_STAGE_A_COMPAT_BRIDGE_RS"
  "$SELFHOST_STAGE_A_ROUTE_RS"
  "$SELFHOST_STAGE_A_POLICY_RS"
  "$SELFHOST_COMMON_JSON_RS"
  "$SELFHOST_STAGE0_CAPTURE_RS"
  "$SELFHOST_STAGE0_CAPTURE_ROUTE_RS"
  "$RUNNER_SELFHOST_RS"
  "$HH_COMPILER_README"
  "$STAGE1_BRIDGE_README"
  "$STAGE1_BRIDGE_ENV_RS"
  "$STAGE1_BRIDGE_MODULES_RS"
  "$HH_COMPILER_ENTRY"
  "$HH_COMPILER_STAGEB_ENTRY"
  "$HH_STAGEB_ARGS"
  "$HH_STAGEB_BUILD_OPTIONS"
  "$HH_STAGEB_COMPILE_ADAPTER"
  "$HH_STAGEB_OUTPUT"
  "$HH_BUNDLE_RESOLVER"
  "$HH_STAGEB_BODY_EXTRACTOR"
  "$HH_STAGEB_KEYWORD_EXPR_STRIP"
  "$HH_STAGEB_DRIVER_GUARD"
  "$HH_STAGEB_TRACE"
  "$HH_STAGEB_MAIN_DETECTION"
  "$HH_STAGEB_RUNE"
  "$HH_STAGEB_USER_BOX_DECL_SCANNER"
  "$HH_FUNC_SCANNER"
  "$HH_FUNC_SCANNER_HELPERS"
  "$HH_BUILD_README"
  "$HH_BUILD_BUNDLE_FACADE"
  "$HH_MIRBUILDER_README"
  "$HH_PARSER_BOX"
  "$HH_PARSER_EXPR_BOX"
  "$HH_PARSER_NUMBER_SCAN"
  "$HH_PARSER_RUNE_CONTRACT"
  "$HH_PARSER_CONTROL_BOX"
  "$HH_PARSER_EXCEPTION_BOX"
  "$HH_PARSER_STMT_CORE"
  "$HH_TEST_FUNCSCANNER_SKIP_WS"
  "$HH_TEST_STAGEB_MIN_SAMPLE"
  "$K2_WIDE_STAGEB_FIELD_TYPE_ANNOTATION_GUARD"
  "$K2_WIDE_STAGEB_NUMERIC_LITERAL_SUFFIX_GUARD"
  "$NY_PARSER_BRIDGE_SMOKE"
  "$PHI_TRACE_RUN"
  "$TEST_SHLIB"
  "$EMIT_MIR_ROUTE"
  "$HAKO_MIN_BINOP_SMOKE"
  "$HAKO_MIN_IF_SMOKE"
  "$HAKO_MIN_INDEX_SMOKE"
  "$HAKO_MIN_COMPILE_RETURN_SMOKE"
  "$HAKO_MAP_ESCAPE_SMOKE"
  "$STAGEB_HELPERS"
  "$GATE_C_V1_FILE_SMOKE"
  "$NYVM_WRAPPER_SMOKE"
  "$FASTMEM_PARSER_PARITY_SMOKE"
  "$PARSER_OPT_ANNOTATIONS_SMOKE"
  "$PARSER_TRY_COMPAT_SMOKE"
  "$PARSER_MIN_METHODS_SMOKE"
  "$PARSER_RUNE_DECL_TRACE_SMOKE"
  "$GATE_C_OOB_STRICT_SMOKE"
  "$NY_MIR_BUILDER"
  "$PHASE29X_L1_CACHE"
  "$PHASE29X_L2_CACHE"
  "$SMOKE_PREFLIGHT"
  "$SMOKE_PLUGIN_MANAGER"
  "$SMOKE_TEST_RUNNER"
  "$SMOKE_AUTO_DETECT_CONF"
  "$FASTMEM_SOURCE_SYNTAX_SMOKE"
  "$FASTMEM_TERMINAL_LADDER_SMOKE"
  "$FASTMEM_SOURCE_MANIFEST_RUNNER"
  "$SELFHOST_JSON_V0_TRY_CATCH_CANARY"
  "$SELFHOST_STABLE_PATHS_SMOKE"
  "$SELFHOST_PLANNER_REQUIRED_DEV_GATE"
  "$SELFHOST_STAGE1_CONTRACT_SMOKE"
  "$SELFHOST_STAGEB_FUNCSCANNER_BOX_SMOKE"
  "$SELFHOST_STAGEB_FUNCSCANNER_METHOD_BOUNDARY_SMOKE"
  "$SELFHOST_STAGEB_LAMBDA_LITERAL_PAIR_SMOKE"
  "$SELFHOST_STAGEB_ROUTE_PARITY_SMOKE"
  "$SELFHOST_STEADY_STATE_SMOKE"
  "$SELFHOST_STAGEB_FUNCSCANNER_TYPED_PARAMS_SMOKE"
  "$APP_BINARY_ONLY_SELFHOST_READINESS_SMOKE"
  "$APP_BINARY_ONLY_RUN_PORTED_SMOKE"
  "$APP_NO_COMPAT_MAINLINE_SMOKE"
  "$APP_PERF_COMPILE_RUN_SPLIT_SMOKE"
  "$APP_SMOKE_LIB_README"
  "$COLLECTION_MAP_GET_SHARES_MAP_SMOKE"
  "$COLLECTION_MAP_GET_SHARES_ARRAY_SMOKE"
  "$COLLECTION_STRING_SIZE_ALIAS_SMOKE"
  "$GOLDEN_MACRO_RESOLVER"
  "$ENV_RS"
  "$STAGE1_ENV_RS"
  "$PARSER_FLAGS_RS"
  "$SELFHOST_FLAGS_RS"
  "$ENV_PATHS_RS"
  "$VERIFICATION_FLAGS_RS"
  "$ENV_DOC"
  "$STAGE1_BRIDGE_ENV_PARSER_STAGEB_RS"
  "$RUST_STAGE1_README"
  "$RUST_STAGE1_MOD_RS"
  "$RUST_STAGE1_PROGRAM_JSON_RS"
  "$RUST_STAGE1_PROGRAM_JSON_ROUTING_RS"
  "$RUST_STAGE1_PROGRAM_JSON_README"
  "$REFERENCE_EBNF"
  "$REFERENCE_STATEMENTS"
  "$PHASE29CI_STAGEB_BODY_EXTRACT_TEST"
  "$JSON_V0_BRIDGE_AST_RS"
  "$JSON_V0_BRIDGE_IF_LEGACY_RS"
  "$JSON_V0_BRIDGE_LAMBDA_LEGACY_RS"
  "$JSON_V0_BRIDGE_LOOP_RS"
  "$JSON_V0_BRIDGE_LOOP_RANGE_RS"
  "$JSON_V0_BRIDGE_PROGRAM_RS"
  "$JSON_V0_BRIDGE_BLOCK_EXPR_RS"
  "${PIPELINE_V2_STAGE_COMMENT_FILES[@]}"
  "${JOINIR_LOWERING_STAGE_COMMENT_FILES[@]}"
  "${STAGE1_BRIDGE_PHASE_COMMENT_REQUIRED_FILES[@]}"
)
guard_require_unique_values "required file" "${REQUIRED_FILES[@]}"
guard_require_files "$TAG" "${REQUIRED_FILES[@]}"

require_fixed() {
  local pattern="$1"
  local file="$2"
  guard_expect_fixed_in_file "$TAG" "$pattern" "$file" "$file missing required naming token: $pattern"
}

require_fixed "RHako" "$SSOT"
require_fixed "HHako" "$SSOT"
require_fixed "Qualify by layer. Naked \"stage\" is forbidden for new names." "$SSOT"
require_fixed "run-pipeline" "$SSOT"
require_fixed "converter" "$SSOT"
require_fixed "adoption-plan" "$SSOT"
require_fixed "NYASH_*" "$SSOT"
require_fixed "HAKORUNE_*" "$SSOT"
SSOT_REQUIRED_TOKENS=(
  "NAMING-CHARTER-STAGE-TERM-DISAMBIGUATION-001"
  "NYASH-TO-HAKORUNE-RENAME-ROADMAP-001"
  "HAKORUNE-USER-FACING-DOCS-CANONICALIZATION-001"
  "HAKORUNE-BINARY-PRIMARY-CUTOVER-INVENTORY-001"
  "HAKORUNE-BINARY-DEFAULT-RUN-CUTOVER-001"
  "HAKORUNE-RUNNER-BUILD-HELPER-BINARY-RESOLUTION-001"
  "HAKORUNE-WINDOWS-BUILD-SCRIPT-CUTOVER-INVENTORY-001"
  "HAKORUNE-HAKO-CHECK-BINARY-RESOLUTION-001"
  "HAKORUNE-ROOT-POWERSHELL-BUILD-SCRIPT-CUTOVER-001"
  "HAKORUNE-DEV-SELFHOST-SMOKE-BINARY-RESOLUTION-001"
  "HAKORUNE-ENGINEERING-PARITY-BINARY-RESOLUTION-001"
  "HAKORUNE-SELFHOST-EXE-STAGEB-BINARY-RESOLUTION-001"
  "HAKORUNE-SELFHOST-EXE-STAGEB-SOURCE-WORDING-001"
  "HAKORUNE-NAMING-GUARD-REQUIRED-FILES-READABILITY-001"
  "HAKORUNE-NAMING-GUARD-REQUIRED-FILES-DUPLICATE-CHECK-001"
  "HAKORUNE-NAMING-GUARD-DUPLICATE-CHECK-HELPER-001"
  "HAKORUNE-NAMING-GUARD-SSOT-TOKEN-LIST-READABILITY-001"
  "HAKORUNE-NAMING-GUARD-DIFF-ALLOWLIST-SSOT-001"
  "HAKORUNE-CORE-EMIT-HELPER-BINARY-RESOLUTION-001"
  "HAKORUNE-SELFHOST-ROUTE-BINARY-DIAGNOSTICS-001"
  "HAKORUNE-SELFHOST-MAINLINE-STAGE1-BINARY-RESOLUTION-001"
  "HAKORUNE-SELFHOST-RUN-DIRECT-MODE-B-DIAGNOSTIC-001"
  "HAKORUNE-PARSER-BRIDGE-SMOKE-BINARY-RESOLUTION-001"
  "HAKORUNE-PHI-TRACE-RUNNER-BINARY-RESOLUTION-001"
  "HAKORUNE-TEST-SHLIB-BINARY-RESOLUTION-001"
  "HAKORUNE-SMOKE-EMIT-MIR-ROUTE-BINARY-ALIAS-001"
  "HAKORUNE-BRIDGE-CANONICALIZE-STABLE-SMOKE-BINARY-RESOLUTION-001"
  "HAKORUNE-WRAPPER-EXECUTABLE-BIT-001"
  "HAKORUNE-MIN-OPTIN-SMOKE-BINARY-RESOLUTION-001"
  "HAKORUNE-QUICK-SMOKE-MODE-A-DIAGNOSTIC-001"
  "HAKORUNE-QUICK-SMOKE-MODE-B-DIAGNOSTIC-001"
  "HAKORUNE-MAP-ESCAPE-OPTIN-SMOKE-BINARY-RESOLUTION-001"
  "HAKORUNE-GATE-C-NYVM-WRAPPER-SMOKE-BINARY-NAMING-001"
  "HAKORUNE-PARSER-INTEGRATION-SMOKE-BINARY-NAMING-001"
  "HAKORUNE-PARSER-TRY-COMPAT-SMOKE-BINARY-NAMING-001"
  "HAKORUNE-PARSER-INTEGRATION-EXTRA-SMOKE-BINARY-NAMING-001"
  "HAKORUNE-GOLDEN-MACRO-BINARY-RESOLVER-001"
  "HAKORUNE-CURRENT-DIAGNOSTIC-BINARY-WORDING-001"
  "HAKORUNE-SMOKE-TEST-RUNNER-BINARY-RESOLUTION-001"
  "HAKORUNE-SMOKE-AUTO-DETECT-BINARY-RESOLUTION-001"
  "HAKORUNE-GATE-C-OOB-STRICT-SMOKE-BINARY-RESOLUTION-001"
  "HAKORUNE-NY-MIR-BUILDER-BINARY-RESOLUTION-001"
  "HAKORUNE-PHASE29X-CACHE-HELPER-BINARY-RESOLUTION-001"
  "HAKORUNE-SMOKE-SHARED-PREFLIGHT-BINARY-RESOLUTION-001"
  "HAKORUNE-SMOKE-PREFLIGHT-STAGE-TERM-DIAGNOSTIC-001"
  "HAKORUNE-HAKO-CHECK-WRAPPER-BINARY-RESOLUTION-001"
  "HAKORUNE-FASTMEM-HAKO-CHECK-SMOKE-BINARY-RESOLUTION-001"
  "HAKORUNE-COLLECTION-QUICK-SMOKE-BINARY-WORDING-001"
  "HAKORUNE-ENV-ALIAS-INVENTORY-001"
  "HAKORUNE-ENV-ALIAS-FIRST-CUT-001"
  "STAGE-TERM-EXISTING-NAME-INVENTORY-001"
  "STAGE-TERM-SYNTAX3-ALIAS-001"
  "STAGE-TERM-SYNTAX3-DIAGNOSTIC-WORDING-001"
  "STAGE-TERM-MODEB-COMPAT-ENV-WORDING-001"
  "STAGE-TERM-MODEB-PROOF-ROUTE-WORDING-001"
  "STAGE-TERM-MODEB-STAGE1-BRIDGE-WORDING-001"
  "STAGE-TERM-MODEA-COMPAT-ROUTE-WORDING-001"
  "STAGE-TERM-MODEB-HHAKO-ENTRY-WORDING-001"
  "STAGE-TERM-MODEB-HHAKO-COMPAT-FIXTURE-WORDING-001"
  "STAGE-TERM-HHAKO-COMPILER-ROUTE-WORDING-001"
  "STAGE-TERM-MODEB-HHAKO-HELPER-COMMENT-WORDING-001"
  "STAGE-TERM-MODEB-CAPTURE-CALLER-GUARD-WORDING-001"
  "STAGE-TERM-PHASE1-PROGRAM-JSON-GUARD-WORDING-001"
  "STAGE-TERM-STAGE0-SHAPE-GATE-LABEL-WORDING-001"
  "STAGE-TERM-MODEB-HHAKO-FUNC-SCANNER-COMMENT-WORDING-001"
  "STAGE-TERM-MODEB-K2-WIDE-GUARD-DIAGNOSTIC-WORDING-001"
  "STAGE-TERM-SELFHOST-SMOKE-COMMENT-WORDING-001"
  "STAGE-TERM-CHECK-SCRIPTS-INDEX-WORDING-001"
  "STAGE-TERM-SYNTAX3-RUST-ENV-COMMENT-WORDING-001"
  "STAGE-TERM-HHAKO-PARSER-BUILD-COMMENT-WORDING-001"
  "STAGE-TERM-JSON-V0-BRIDGE-COMMENT-WORDING-001"
  "STAGE-TERM-HHAKO-BUILD-TEST-COMMENT-WORDING-001"
  "STAGE-TERM-APP-BINARY-ONLY-SMOKE-COMMENT-WORDING-001"
  "STAGE-TERM-APP-SMOKE-PHASE-COMMENT-WORDING-001"
  "STAGE-TERM-STAGE0-CAPTURE-COMMENT-WORDING-001"
  "STAGE-TERM-PIPELINE-V2-COMMENT-WORDING-001"
  "STAGE-TERM-JOINIR-LOWERING-COMMENT-WORDING-001"
  "STAGE-TERM-LANG-README-PHASE-WORDING-001"
  "STAGE-TERM-DOCS-TOOLS-QUICK-ENTRY-WORDING-001"
  "STAGE-TERM-STAGE1-BRIDGE-PHASE-COMMENT-WORDING-001"
  "STAGE-TERM-ENV-REFERENCE-PHASE-WORDING-001"
  "STAGE-TERM-RUST-STAGE1-ENV-HELPER-COMMENT-WORDING-001"
  "STAGE-TERM-RUST-STAGE1-BOUNDARY-COMMENT-WORDING-001"
)
guard_require_unique_values "SSOT required token" "${SSOT_REQUIRED_TOKENS[@]}"
for ssot_token in "${SSOT_REQUIRED_TOKENS[@]}"; do
  require_fixed "$ssot_token" "$SSOT"
done
require_fixed 'prefer `target/release/hakorune` or `$HAKO_BIN`' "$README_MD"
require_fixed '`$NYASH_BIN` remains a compatibility alias' "$README_MD"
require_fixed 'default-run = "hakorune"' "$CARGO_TOML"
require_fixed 'name = "hakorune"' "$CARGO_TOML"
require_fixed 'path = "src/bin/hakorune.rs"' "$CARGO_TOML"
require_fixed 'name = "nyash"' "$CARGO_TOML"
require_fixed 'path = "src/main.rs"' "$CARGO_TOML"
require_fixed 'name = "hakorune-compat"' "$CARGO_TOML"
require_fixed 'path = "src/bin/hakorune_compat.rs"' "$CARGO_TOML"
require_fixed 'cargo check --bin hakorune' "$QUICK_STEPS"
require_fixed 'GlobalCallTarget shape inventory guard' "$QUICK_STEPS"
require_fixed 'mode-B compatibility Program(JSON) capture caller guard' "$QUICK_STEPS"
require_fixed 'phase-1 compatibility emit-program runtime-helper guard' "$QUICK_STEPS"
require_fixed 'phase-1 compatibility Program(JSON) caller guard' "$QUICK_STEPS"
require_fixed 'BIN_HAKORUNE="$ROOT_DIR/target/release/hakorune"' "$HACO_WRAPPER"
require_fixed 'BIN_NYASH="$ROOT_DIR/target/release/nyash"' "$HACO_WRAPPER"
require_fixed 'if [[ -x "$BIN_HAKORUNE" ]]; then' "$HACO_WRAPPER"
if [[ ! -x "$HACO_WRAPPER" ]]; then
  guard_fail "$TAG" "tools/bin/hako must be executable"
fi
require_fixed 'include!("../main.rs");' "$HAKORUNE_BIN_RS"
require_fixed 'include!("../main.rs");' "$HAKORUNE_COMPAT_BIN_RS"
require_fixed "phase-1 core compatibility" "$LANG_README"
require_fixed "future owner-policy boxes" "$LANG_README"
require_fixed "phase-1 bridge/proof reading" "$LANG_README"
require_fixed 'legacy `stage0` is the Rust authority side' "$LANG_README"
require_fixed '`phase-1` / `K2+`' "$LANG_README"
if rg -n 'Stage1 core|Stage1 selfhost|Stage1 コア|stage1 bridge|Stage2\+ line|stage2 owner-policy boxes|単一の stage artifact|stage/selfhost' "$LANG_README"; then
  guard_fail "$TAG" "lang README must use phase-1 / K2+ wording for current user-facing stage terms"
fi
require_fixed "HAKO_ALLOW_NYASH" "$MAIN_RS"
require_fixed "NYASH_ALLOW_NYASH" "$MAIN_RS"
require_fixed "'nyash' binary is deprecated. Please use 'hakorune'." "$MAIN_RS"
require_fixed "hakorune_cli_bin_path" "$BUILD_SHARED_RS"
require_fixed 'join(exe_name("hakorune"))' "$BUILD_SHARED_RS"
require_fixed 'join(exe_name("nyash"))' "$BUILD_SHARED_RS"
require_fixed "hakorune_cli_bin_path" "$BUILD_PRODUCT_RS"
require_fixed "hakorune_cli_bin_path" "$BUILD_ENGINEERING_RS"
if rg -n "nyash_bin_path|nyash\\.exe" "$BUILD_PRODUCT_RS" "$BUILD_ENGINEERING_RS"; then
  guard_fail "$TAG" "runner build product/engineering helpers must use hakorune_cli_bin_path"
fi
if rg -n -F -e "--bin nyash" "$WINDOWS_DIR"; then
  guard_fail "$TAG" "Windows build scripts must not build the legacy nyash binary directly"
fi
require_fixed "Resolve-HakoruneCli" "$ROOT_DIR/tools/windows/build_egui_aot.ps1"
require_fixed "Resolve-HakoruneCli" "$ROOT_DIR/tools/windows/build_app_egui_manual.ps1"
require_fixed 'HAKO_ALIAS_BIN="$ROOT_DIR/tools/bin/hako"' "$HAKO_CHECK_SH"
require_fixed 'HAKORUNE_ALIAS_BIN="$ROOT_DIR/tools/bin/hakorune"' "$HAKO_CHECK_SH"
require_fixed 'HAKORUNE_RELEASE_BIN="$ROOT_DIR/target/release/hakorune"' "$HAKO_CHECK_SH"
require_fixed 'LEGACY_NYASH_BIN="$ROOT_DIR/target/release/nyash"' "$HAKO_CHECK_SH"
require_fixed 'BIN="$LEGACY_NYASH_BIN"' "$HAKO_CHECK_SH"
if rg -n '^\s*BIN="\$ROOT_DIR/target/release/nyash"' "$HAKO_CHECK_SH"; then
  guard_fail "$TAG" "hako-check wrapper must spell legacy nyash as a named compatibility fallback"
fi
require_fixed "Resolve-HakoruneCli" "$BUILD_LLVM_PS"
require_fixed "Resolve-HakoruneCli" "$BUILD_AOT_PS"
require_fixed 'target\release\hakorune.exe' "$BUILD_LLVM_PS"
require_fixed 'target\release\hakorune.exe' "$BUILD_AOT_PS"
if rg -n -F -e '& .\target\release\nyash' "$BUILD_LLVM_PS" "$BUILD_AOT_PS"; then
  guard_fail "$TAG" "root PowerShell build scripts must invoke Resolve-HakoruneCli instead of legacy nyash directly"
fi
for smoke_script in "$USING_UNRESOLVED_SMOKE" "$USING_RESOLVE_SMOKE" "$USING_STRICT_PATH_FAIL_SMOKE" "$DEV_SELFHOST_LOOP"; do
  require_fixed 'HAKORUNE_BIN="$ROOT_DIR/target/release/hakorune"' "$smoke_script"
  require_fixed 'LEGACY_NYASH_BIN="$ROOT_DIR/target/release/nyash"' "$smoke_script"
  if rg -n '^\s*BIN="\$ROOT_DIR/target/release/nyash"' "$smoke_script"; then
    guard_fail "$TAG" "dev/selfhost smoke scripts must resolve Hakorune before legacy nyash"
  fi
done
require_fixed 'HAKORUNE_BIN="$ROOT/target/release/hakorune"' "$ENGINEERING_PARITY"
require_fixed 'LEGACY_NYASH_BIN="$ROOT/target/release/nyash"' "$ENGINEERING_PARITY"
if rg -n '^\s*NYASH_BIN="\$ROOT/target/release/nyash"' "$ENGINEERING_PARITY"; then
  guard_fail "$TAG" "engineering parity helper must resolve Hakorune before legacy nyash"
fi
require_fixed "resolve_hakorune_bin" "$SELFHOST_EXE_STAGEB"
require_fixed 'if [[ "${1:-}" == "--help" || "${1:-}" == "-h" ]]; then' "$SELFHOST_EXE_STAGEB"
require_fixed 'local hakorune_bin="$ROOT_DIR/target/release/hakorune"' "$SELFHOST_EXE_STAGEB"
require_fixed 'local legacy_nyash_bin="$ROOT_DIR/target/release/nyash"' "$SELFHOST_EXE_STAGEB"
require_fixed 'NYASH_BIN="$hakorune_bin"' "$SELFHOST_EXE_STAGEB"
require_fixed 'NYASH_BIN="$legacy_nyash_bin"' "$SELFHOST_EXE_STAGEB"
require_fixed 'Build a native EXE from a Hakorune .hako source' "$SELFHOST_EXE_STAGEB"
if rg -n "resolve_nyash_bin|nyash/hakorune binary not found" "$SELFHOST_EXE_STAGEB"; then
  guard_fail "$TAG" "selfhost EXE Stage-B helper must use Hakorune-first resolver naming"
fi
if rg -n 'Nyash \.hako source' "$SELFHOST_EXE_STAGEB"; then
  guard_fail "$TAG" "selfhost EXE Stage-B helper must describe .hako input as Hakorune source"
fi
for selfhost_route_script in "$SELFHOST_STAGEB_PROOF_VM" "$SELFHOST_RUN_ROUTES" "$SELFHOST_BUILD"; do
  require_fixed 'Hakorune' "$selfhost_route_script"
  if rg -n 'nyash binary not found|no binary found under target/release|no bootstrap binary found under target/release' "$selfhost_route_script"; then
    guard_fail "$TAG" "selfhost route diagnostics must name Hakorune while keeping NYASH_BIN as compatibility override"
  fi
done
require_fixed '--syntax-3' "$SELFHOST_STAGEB_PROOF_VM"
require_fixed 'proof-only mode-B compatibility compiler gate' "$SELFHOST_STAGEB_PROOF_VM"
require_fixed 'mode-B VM compatibility route is proof-only' "$SELFHOST_STAGEB_PROOF_VM"
require_fixed 'mode-B compatibility gate stays explicit proof-only' "$SELFHOST_STAGEB_PROOF_VM"
require_fixed '`--direct`: run mode-B direct/source route' "$SELFHOST_README"
require_fixed 'proof-only mode-B compatibility compiler route' "$SELFHOST_README"
require_fixed 'Explicit mode-B compatibility Program(JSON v0) artifact capture' "$SELFHOST_README"
if rg -n 'Stage-B direct/source route' "$SELFHOST_README"; then
  guard_fail "$TAG" "selfhost README daily --direct description must say mode-B, not Stage-B"
fi
if rg -n 'proof-only Stage-B compiler route|Explicit Stage-B Program\(JSON v0\) artifact capture|consumes Stage-B Program\(JSON v0\)' "$SELFHOST_README"; then
  guard_fail "$TAG" "selfhost README proof route wording must say mode-B compatibility, not Stage-B"
fi
require_fixed 'HAKORUNE_BIN_PATH="$ROOT/target/release/hakorune"' "$SELFHOST_BUILD"
require_fixed 'LEGACY_NYASH_BIN_PATH="$ROOT/target/release/nyash"' "$SELFHOST_BUILD"
require_fixed 'BIN="$HAKORUNE_BIN_PATH"' "$SELFHOST_BUILD"
require_fixed 'BIN="$LEGACY_NYASH_BIN_PATH"' "$SELFHOST_BUILD"
if rg -n '^\s*BIN="\$ROOT/target/release/nyash"' "$SELFHOST_BUILD"; then
  guard_fail "$TAG" "selfhost build helper must use named Hakorune-first binary resolver"
fi
require_fixed 'HAKORUNE_BIN_PATH="$ROOT/target/release/hakorune"' "$HAKORUNE_EMIT_MIR"
require_fixed 'LEGACY_NYASH_BIN_PATH="$ROOT/target/release/nyash"' "$HAKORUNE_EMIT_MIR"
require_fixed 'export NYASH_BIN="$HAKORUNE_BIN_PATH"' "$HAKORUNE_EMIT_MIR"
require_fixed 'export NYASH_BIN="$LEGACY_NYASH_BIN_PATH"' "$HAKORUNE_EMIT_MIR"
if rg -n 'Resolve nyash/hakorune binary|export NYASH_BIN="\$ROOT/target/release/nyash"' "$HAKORUNE_EMIT_MIR"; then
  guard_fail "$TAG" "core emit helper must use named Hakorune-first binary resolver"
fi
require_fixed 'HAKORUNE_BOOTSTRAP_BIN="$ROOT/target/release/hakorune"' "$SELFHOST_MAINLINE_BUILD_STAGE1"
require_fixed 'LEGACY_NYASH_BOOTSTRAP_BIN="$ROOT/target/release/nyash"' "$SELFHOST_MAINLINE_BUILD_STAGE1"
require_fixed 'NYASH_BIN="$HAKORUNE_BOOTSTRAP_BIN"' "$SELFHOST_MAINLINE_BUILD_STAGE1"
require_fixed 'NYASH_BIN="$LEGACY_NYASH_BOOTSTRAP_BIN"' "$SELFHOST_MAINLINE_BUILD_STAGE1"
require_fixed 'Hakorune bootstrap binary not found' "$SELFHOST_MAINLINE_BUILD_STAGE1"
if rg -n '^\s*NYASH_BIN="\$ROOT/target/release/nyash"|no bootstrap binary found under target/release' "$SELFHOST_MAINLINE_BUILD_STAGE1"; then
  guard_fail "$TAG" "selfhost mainline build_stage1 must use named Hakorune-first bootstrap resolver"
fi
require_fixed 'HAKORUNE_BIN="$ROOT_DIR/target/release/hakorune"' "$NY_PARSER_BRIDGE_SMOKE"
require_fixed 'LEGACY_NYASH_BIN="$ROOT_DIR/target/release/nyash"' "$NY_PARSER_BRIDGE_SMOKE"
require_fixed 'mktemp /tmp/hakorune-bridge-smoke.' "$NY_PARSER_BRIDGE_SMOKE"
if rg -n "nyash-bridge-smoke|BIN=\"\\$ROOT_DIR/target/release/nyash\"" "$NY_PARSER_BRIDGE_SMOKE"; then
  guard_fail "$TAG" "parser bridge smoke must use Hakorune-first temp and binary naming"
fi
require_fixed 'HAKORUNE_BIN="${HAKORUNE_BIN:-$ROOT/target/release/hakorune}"' "$PHI_TRACE_RUN"
require_fixed 'LEGACY_NYASH_BIN="$ROOT/target/release/nyash"' "$PHI_TRACE_RUN"
require_fixed '"$BIN" --backend llvm "$APP"' "$PHI_TRACE_RUN"
if rg -n 'target/release/nyash" --backend llvm|nyash exit code' "$PHI_TRACE_RUN"; then
  guard_fail "$TAG" "PHI trace runner must invoke Hakorune-first binary resolver"
fi
require_fixed "resolve_hakorune_bin" "$TEST_SHLIB"
require_fixed 'local primary="${HAKORUNE_BIN:-$ROOT_DIR/target/release/hakorune}"' "$TEST_SHLIB"
require_fixed 'local fallback="$ROOT_DIR/target/release/nyash"' "$TEST_SHLIB"
require_fixed '"$(resolve_hakorune_bin)" --emit-mir-json' "$TEST_SHLIB"
if rg -n 'target/release/nyash" --emit-mir-json' "$TEST_SHLIB"; then
  guard_fail "$TAG" "test shell helper emit_json must invoke Hakorune-first binary resolver"
fi
require_fixed 'if [ -n "${HAKO_BIN:-}" ] && [ -z "${NYASH_BIN:-}" ]; then' "$EMIT_MIR_ROUTE"
require_fixed 'HAKORUNE_BIN_PATH="$ROOT/target/release/hakorune"' "$EMIT_MIR_ROUTE"
require_fixed 'LEGACY_NYASH_BIN_PATH="$ROOT/target/release/nyash"' "$EMIT_MIR_ROUTE"
require_fixed 'NYASH_BIN="$HAKORUNE_BIN_PATH"' "$EMIT_MIR_ROUTE"
require_fixed 'NYASH_BIN="$LEGACY_NYASH_BIN_PATH"' "$EMIT_MIR_ROUTE"
require_fixed 'HAKO_BIN="${HAKO_BIN:-$NYASH_BIN}"' "$EMIT_MIR_ROUTE"
require_fixed '<HAKO_BIN|NYASH_BIN> --backend mir --emit-mir-json' "$EMIT_MIR_ROUTE"
if rg -n "nyash/hakorune binary not found" "$EMIT_MIR_ROUTE"; then
  guard_fail "$TAG" "emit MIR route helper must use Hakorune-first binary wording"
fi
for bridge_script in "$BRIDGE_CANON_DIR/canonicalize_noop_method_on_vm.sh" "$BRIDGE_CANON_DIR/canonicalize_fail_vm.sh"; do
  require_fixed '"$NYASH_BIN" --json-file "$json_path"' "$bridge_script"
  if rg -n 'target/release/nyash" --json-file' "$bridge_script"; then
    guard_fail "$TAG" "bridge canonicalize smoke must use shared Hakorune-first NYASH_BIN resolver"
  fi
done
for hako_min_script in "$HAKO_MIN_BINOP_SMOKE" "$HAKO_MIN_IF_SMOKE" "$HAKO_MIN_INDEX_SMOKE" "$HAKO_MIN_COMPILE_RETURN_SMOKE"; do
  require_fixed '"$HAKO_BIN" --backend vm' "$hako_min_script"
  require_fixed '"$HAKO_BIN" --json-file' "$hako_min_script"
  if rg -n 'target/release/nyash" --(backend vm|json-file)' "$hako_min_script"; then
    guard_fail "$TAG" "Hako minimum opt-in smokes must use HAKO_BIN instead of direct legacy nyash"
  fi
done
require_fixed "mode-A" "$HAKO_MIN_COMPILE_RETURN_SMOKE"
require_fixed '"$HAKO_BIN" --backend vm' "$HAKO_MAP_ESCAPE_SMOKE"
require_fixed '"$HAKO_BIN" --json-file "$json_path"' "$HAKO_MAP_ESCAPE_SMOKE"
require_fixed "mode-A" "$HAKO_MAP_ESCAPE_SMOKE"
if rg -n 'target/release/nyash" --(backend vm|json-file)' "$HAKO_MAP_ESCAPE_SMOKE"; then
  guard_fail "$TAG" "Hako map escape opt-in smoke must use HAKO_BIN instead of direct legacy nyash"
fi
if rg -n 'Stage-A' "$HAKO_MIN_COMPILE_RETURN_SMOKE" "$HAKO_MAP_ESCAPE_SMOKE"; then
  guard_fail "$TAG" "quick smoke diagnostics must say mode-A, not Stage-A"
fi
require_fixed "mode-B" "$STAGEB_HELPERS"
if rg -n 'Stage-B' "$STAGEB_HELPERS"; then
  guard_fail "$TAG" "quick smoke helper diagnostics must say mode-B, not Stage-B"
fi
for gate_smoke in "$GATE_C_V1_FILE_SMOKE" "$NYVM_WRAPPER_SMOKE"; do
  require_fixed 'HAKORUNE_BIN="$ROOT/target/release/hakorune"' "$gate_smoke"
  require_fixed 'LEGACY_NYASH_BIN="$ROOT/target/release/nyash"' "$gate_smoke"
  if rg -n '^\s*BIN="\$ROOT/target/release/nyash"' "$gate_smoke"; then
    guard_fail "$TAG" "Gate-C/NyVM wrapper smokes must spell Hakorune-first binary resolver"
  fi
done
for parser_smoke in "$FASTMEM_PARSER_PARITY_SMOKE" "$PARSER_OPT_ANNOTATIONS_SMOKE"; do
  require_fixed 'HAKORUNE_BIN="${HAKORUNE_BIN:-$NYASH_ROOT/target/release/hakorune}"' "$parser_smoke"
  require_fixed 'LEGACY_NYASH_BIN="$NYASH_ROOT/target/release/nyash"' "$parser_smoke"
  if rg -n '^\s*BIN="\$NYASH_ROOT/target/release/nyash"' "$parser_smoke"; then
    guard_fail "$TAG" "parser integration smokes must spell Hakorune-first binary resolver"
  fi
done
require_fixed 'HAKORUNE_BIN="${HAKORUNE_BIN:-./target/release/hakorune}"' "$PARSER_TRY_COMPAT_SMOKE"
require_fixed 'LEGACY_NYASH_BIN="./target/release/nyash"' "$PARSER_TRY_COMPAT_SMOKE"
if rg -n '^\s*BIN="\./target/release/nyash"' "$PARSER_TRY_COMPAT_SMOKE"; then
  guard_fail "$TAG" "parser try-compat smoke must spell Hakorune-first binary resolver"
fi
require_fixed 'BIN="${NYASH_BIN:-./target/release/hakorune}"' "$PARSER_MIN_METHODS_SMOKE"
require_fixed 'Hakorune binary not found' "$PARSER_MIN_METHODS_SMOKE"
if rg -n 'nyash binary not found|^\s*BIN="\./target/release/nyash"' "$PARSER_MIN_METHODS_SMOKE"; then
  guard_fail "$TAG" "parser min-methods smoke must keep Hakorune default and Hakorune-first error wording"
fi
require_fixed 'HAKORUNE_BIN="${HAKORUNE_BIN:-$NYASH_ROOT/target/release/hakorune}"' "$PARSER_RUNE_DECL_TRACE_SMOKE"
require_fixed 'LEGACY_NYASH_BIN="$NYASH_ROOT/target/release/nyash"' "$PARSER_RUNE_DECL_TRACE_SMOKE"
require_fixed 'Hakorune binary not found' "$PARSER_RUNE_DECL_TRACE_SMOKE"
if rg -n 'nyash/hakorune binary not found|^\s*BIN="\$NYASH_ROOT/target/release/nyash"' "$PARSER_RUNE_DECL_TRACE_SMOKE"; then
  guard_fail "$TAG" "parser rune-decl trace smoke must spell Hakorune-first resolver"
fi
require_fixed 'HAKO_BIN="${HAKO_BIN:-$HAKO_BIN_DEFAULT}"' "$GATE_C_OOB_STRICT_SMOKE"
require_fixed '"$HAKO_BIN" --backend vm' "$GATE_C_OOB_STRICT_SMOKE"
require_fixed '"$HAKO_BIN" --json-file "$json_path"' "$GATE_C_OOB_STRICT_SMOKE"
if rg -n 'target/release/nyash" --(backend vm|json-file)' "$GATE_C_OOB_STRICT_SMOKE"; then
  guard_fail "$TAG" "Gate-C OOB strict smoke must use HAKO_BIN instead of direct legacy nyash"
fi
require_fixed 'HAKORUNE_BIN="./target/release/hakorune"' "$NY_MIR_BUILDER"
require_fixed 'LEGACY_NYASH_BIN="./target/release/nyash"' "$NY_MIR_BUILDER"
require_fixed 'BIN="$HAKORUNE_BIN"' "$NY_MIR_BUILDER"
if rg -n '^\s*BIN="\./target/release/nyash"' "$NY_MIR_BUILDER"; then
  guard_fail "$TAG" "ny MIR builder must spell Hakorune-first binary resolver"
fi
for cache_helper in "$PHASE29X_L1_CACHE" "$PHASE29X_L2_CACHE"; do
  require_fixed 'HAKORUNE_BIN_PATH="$ROOT_DIR/target/release/hakorune"' "$cache_helper"
  require_fixed 'LEGACY_NYASH_BIN_PATH="$ROOT_DIR/target/release/nyash"' "$cache_helper"
  require_fixed 'NYASH_BIN_PATH="$HAKORUNE_BIN_PATH"' "$cache_helper"
  if rg -n '^\s*NYASH_BIN_PATH="\$ROOT_DIR/target/release/nyash"|nyash binary not found' "$cache_helper"; then
    guard_fail "$TAG" "Phase29x cache helpers must spell Hakorune-first resolver and legacy fallback"
  fi
done
for smoke_lib in "$SMOKE_PREFLIGHT" "$SMOKE_PLUGIN_MANAGER"; do
  require_fixed 'HAKORUNE_BIN_PATH="./target/release/hakorune"' "$smoke_lib"
  require_fixed 'LEGACY_NYASH_BIN_PATH="./target/release/nyash"' "$smoke_lib"
  require_fixed 'NYASH_BIN_RESOLVED="${NYASH_BIN:-$HAKORUNE_BIN_PATH}"' "$smoke_lib"
  if rg -n 'NYASH_BIN_RESOLVED="\./target/release/nyash"|hakorune/nyash binary not found' "$smoke_lib"; then
    guard_fail "$TAG" "shared smoke helpers must spell Hakorune-first resolver and legacy fallback"
  fi
done
require_fixed "Hakorune bootstrap CLI" "$SMOKE_PREFLIGHT"
if rg -n "Stage0 CLI" "$SMOKE_PREFLIGHT"; then
  guard_fail "$TAG" "smoke preflight diagnostics must say bootstrap CLI, not Stage0 CLI"
fi
require_fixed 'HAKORUNE_BIN_PATH="${HAKORUNE_BIN:-$NYASH_ROOT/target/release/hakorune}"' "$SMOKE_TEST_RUNNER"
require_fixed 'LEGACY_NYASH_BIN_PATH="$NYASH_ROOT/target/release/nyash"' "$SMOKE_TEST_RUNNER"
require_fixed 'export NYASH_BIN="$HAKORUNE_BIN_PATH"' "$SMOKE_TEST_RUNNER"
require_fixed 'export NYASH_BIN="$LEGACY_NYASH_BIN_PATH"' "$SMOKE_TEST_RUNNER"
if rg -n 'export NYASH_BIN="\$NYASH_ROOT/target/release/nyash"' "$SMOKE_TEST_RUNNER"; then
  guard_fail "$TAG" "smoke test runner must use named Hakorune-first binary resolver"
fi
require_fixed 'HAKORUNE_CLI_BIN="${HAKO_BIN:-./target/release/hakorune}"' "$SMOKE_AUTO_DETECT_CONF"
require_fixed 'LEGACY_NYASH_CLI_BIN="./target/release/nyash"' "$SMOKE_AUTO_DETECT_CONF"
require_fixed 'CLI_BIN="${NYASH_BIN_RESOLVED:-${NYASH_BIN:-$HAKORUNE_CLI_BIN}}"' "$SMOKE_AUTO_DETECT_CONF"
require_fixed 'CLI_BIN="$LEGACY_NYASH_CLI_BIN"' "$SMOKE_AUTO_DETECT_CONF"
if rg -n '^\s*CLI_BIN="\./target/release/nyash"' "$SMOKE_AUTO_DETECT_CONF"; then
  guard_fail "$TAG" "smoke auto-detect config must use named Hakorune-first binary resolver"
fi
for fastmem_smoke in "$FASTMEM_SOURCE_SYNTAX_SMOKE" "$FASTMEM_TERMINAL_LADDER_SMOKE"; do
  require_fixed 'HAKORUNE_BIN="$ROOT/target/release/hakorune"' "$fastmem_smoke"
  require_fixed 'LEGACY_NYASH_BIN="$ROOT/target/release/nyash"' "$fastmem_smoke"
  require_fixed 'BIN="${NYASH_BIN:-$HAKORUNE_BIN}"' "$fastmem_smoke"
  if rg -n '^\s*BIN="\$ROOT/target/release/nyash"|hakorune/nyash binary not found' "$fastmem_smoke"; then
    guard_fail "$TAG" "fastmem hako_check smokes must spell Hakorune-first resolver and legacy fallback"
  fi
done
require_fixed 'Hakorune binary not found' "$FASTMEM_SOURCE_MANIFEST_RUNNER"
require_fixed 'NYASH_BIN remains supported as a compatibility override' "$FASTMEM_SOURCE_MANIFEST_RUNNER"
if rg -n 'hakorune/nyash binary not found|nyash binary not found' "$FASTMEM_SOURCE_MANIFEST_RUNNER"; then
  guard_fail "$TAG" "fastmem source manifest runner must use Hakorune-first diagnostic wording"
fi
for current_diag_script in "$SELFHOST_JSON_V0_TRY_CATCH_CANARY" "$SELFHOST_STAGEB_ROUTE_PARITY_SMOKE"; do
  require_fixed 'Hakorune binary not found' "$current_diag_script"
  if rg -n 'nyash binary not found|hakorune/nyash binary not found' "$current_diag_script"; then
    guard_fail "$TAG" "current selfhost diagnostic scripts must use Hakorune-first binary wording"
  fi
done
require_fixed 'without phase-1 repo dependencies' "$APP_BINARY_ONLY_SELFHOST_READINESS_SMOKE"
require_fixed 'pass1 emit (phase-1 proxy)' "$APP_BINARY_ONLY_SELFHOST_READINESS_SMOKE"
require_fixed 'pass2 emit (phase-2 proxy)' "$APP_BINARY_ONLY_SELFHOST_READINESS_SMOKE"
if rg -n 'without stage1 repo dependencies|pass1 emit \(Stage1 proxy\)|pass2 emit \(Stage2 proxy\)' "$APP_BINARY_ONLY_SELFHOST_READINESS_SMOKE"; then
  guard_fail "$TAG" "binary-only selfhost readiness smoke comments must use phase-1/phase-2 proxy wording"
fi
require_fixed 'phase-1 run route must not depend on repo checkout files' "$APP_BINARY_ONLY_RUN_PORTED_SMOKE"
require_fixed 'Mainline mode-A compatibility runtime probe' "$APP_NO_COMPAT_MAINLINE_SMOKE"
require_fixed 'Phase-1 compatibility diagnostics are covered separately' "$APP_NO_COMPAT_MAINLINE_SMOKE"
require_fixed 'binary-only phase-1 route' "$APP_PERF_COMPILE_RUN_SPLIT_SMOKE"
require_fixed 'binary-only phase-1 probes' "$APP_SMOKE_LIB_README"
if rg -n 'stage1 run route|Mainline stage-a-compat runtime probe|Stage1 bootstrap diagnostics|binary-only stage1 (route|probes)' \
  "$APP_BINARY_ONLY_RUN_PORTED_SMOKE" \
  "$APP_NO_COMPAT_MAINLINE_SMOKE" \
  "$APP_PERF_COMPILE_RUN_SPLIT_SMOKE" \
  "$APP_SMOKE_LIB_README"; then
  guard_fail "$TAG" "app smoke comments must use phase-1 / mode-A compatibility wording"
fi
for collection_smoke in "$COLLECTION_MAP_GET_SHARES_MAP_SMOKE" "$COLLECTION_MAP_GET_SHARES_ARRAY_SMOKE" "$COLLECTION_STRING_SIZE_ALIAS_SMOKE"; do
  require_fixed 'BIN="${NYASH_BIN:-./target/release/hakorune}"' "$collection_smoke"
  require_fixed 'Hakorune binary not found' "$collection_smoke"
  if rg -n 'nyash binary not found|^\s*BIN="\./target/release/nyash"' "$collection_smoke"; then
    guard_fail "$TAG" "collection quick smokes must keep Hakorune default and Hakorune-first error wording"
  fi
done
require_fixed 'resolve_hakorune_golden_bin()' "$GOLDEN_MACRO_RESOLVER"
require_fixed 'local primary="${HAKORUNE_BIN:-$root/target/release/hakorune}"' "$GOLDEN_MACRO_RESOLVER"
require_fixed 'local fallback="$root/target/release/nyash"' "$GOLDEN_MACRO_RESOLVER"
require_fixed 'Hakorune binary not found' "$GOLDEN_MACRO_RESOLVER"
if rg -n 'bin="\$root/target/release/nyash"|nyash binary not found|run nyash --dump-expanded-ast-json' "$GOLDEN_MACRO_DIR"; then
  guard_fail "$TAG" "golden macro scripts must use shared Hakorune-first resolver"
fi
require_fixed "env_bool_with_alias" "$ENV_RS"
require_fixed "env_string_with_alias" "$ENV_RS"
require_fixed "env_string_trimmed_with_alias" "$ENV_RS"
require_fixed "env_present_with_alias" "$ENV_RS"
require_fixed "Phase-1 compatibility / selfhost CLI environment helpers" "$STAGE1_ENV_RS"
require_fixed "phase-1 compatibility stub routing" "$STAGE1_ENV_RS"
require_fixed "phase-1 compatibility bridge runtime defaults" "$STAGE1_ENV_RS"
require_fixed "Phase-1 compatibility mode hint" "$STAGE1_ENV_RS"
require_fixed "phase-1 compatibility emit-mir mode" "$STAGE1_ENV_RS"
if rg -n 'Stage-1 / selfhost CLI environment helpers|enable Stage-1 stub routing|when Stage-1 stub calls|Stage-1 shared result-line|Stage-1 bridge runtime defaults|Stage-1 mode hint|when Stage-1 should emit|passed to Stage-1 stub|for Stage-1 emit-mir mode|for Stage-1 run mode|Optional Stage-1|Stage-1 debug flag' "$STAGE1_ENV_RS"; then
  guard_fail "$TAG" "src/config/env/stage1.rs comments must use phase-1 compatibility wording"
fi
require_fixed "parser syntax-3 gate" "$PARSER_FLAGS_RS"
require_fixed 'compatibility tokens `stage3`/`parser-stage3`' "$PARSER_FLAGS_RS"
require_fixed "syntax-3 is standard syntax" "$PARSER_FLAGS_RS"
require_fixed "syntax-3 gate is on" "$PARSER_FLAGS_RS"
require_fixed 'Pass `-- --syntax-3` to child selfhost compiler' "$SELFHOST_FLAGS_RS"
require_fixed '`--stage3` remains a compatibility alias' "$SELFHOST_FLAGS_RS"
if rg -n 'parser Stage-3 gate|Stage-3 is standard syntax|Stage.3 gate is on|allow Stage-3 surface' "$PARSER_FLAGS_RS" "$SELFHOST_FLAGS_RS"; then
  guard_fail "$TAG" "Rust env comments must describe syntax-3 while keeping stage3 compatibility tokens"
fi
require_fixed "env_string_trimmed_with_alias(\"HAKO_ROOT\", \"NYASH_ROOT\")" "$ENV_PATHS_RS"
require_fixed "env_string_trimmed_with_alias(\"HAKO_BIN\", \"NYASH_BIN\")" "$ENV_PATHS_RS"
require_fixed "HAKORUNE_*" "$ENV_DOC"
require_fixed "HAKO_ROOT" "$ENV_DOC"
require_fixed "HAKO_BIN" "$ENV_DOC"
require_fixed 'mode-B .hako reader compatibility alias' "$ENV_DOC"
require_fixed "JSON v0/phase-1 compatibility" "$ENV_DOC"
require_fixed "## Phase-1 Compatibility / selfhost CLI" "$ENV_DOC"
require_fixed "phase-1 compatibility stub" "$ENV_DOC"
require_fixed "syntax-3 legacy alias" "$ENV_DOC"
require_fixed "bootstrap cleanup/catch boundary" "$ENV_DOC"
require_fixed "phase-1 compatibility helper" "$ENV_DOC"
if rg -n 'JSON v0/Stage-1|## Stage-1 / selfhost CLI|Stage-1 stub|Stage-1 経路|Stage-1 MIR launcher|Stage-3 構文|Stage-3 旧エイリアス|Stage-3 legacy alias|Current Stage0 keeps|Stage0 cleanup/catch boundary|explicit Stage0 keep lane|stage1 helper|Stage-1 emit系|Stage-3（推奨）' "$ENV_DOC"; then
  guard_fail "$TAG" "environment reference must use phase-1 / syntax-3 / bootstrap wording for migrated env surfaces"
fi
require_fixed "mode-B/selfhost compatibility dev verify toggle" "$VERIFICATION_FLAGS_RS"
require_fixed "mode-B compatibility routes" "$VERIFICATION_FLAGS_RS"
if rg -n 'Stage-B/selfhost|Stage-B 経路' "$VERIFICATION_FLAGS_RS"; then
  guard_fail "$TAG" "dev verify env comments must say mode-B compatibility, not Stage-B route"
fi
require_fixed "mode-B compatibility toggles section" "$STAGE1_BRIDGE_ENV_PARSER_STAGEB_RS"
require_fixed "mode-B module payload apply" "$STAGE1_BRIDGE_ENV_PARSER_STAGEB_RS"
if rg -n 'Stage-B toggles|Stage-B module payload' "$STAGE1_BRIDGE_ENV_PARSER_STAGEB_RS"; then
  guard_fail "$TAG" "stage1 bridge env comments must say mode-B compatibility, not Stage-B toggles"
fi
require_fixed "Phase-1 Compatibility Rust Boundary" "$RUST_STAGE1_README"
require_fixed "legacy bootstrap artifact/proof labels" "$RUST_STAGE1_README"
require_fixed "phase-1 compatibility bridge orchestration" "$RUST_STAGE1_README"
require_fixed "Rust phase-1 compatibility bootstrap boundary" "$RUST_STAGE1_MOD_RS"
require_fixed "Phase-1 compatibility Program(JSON v0) façade" "$RUST_STAGE1_PROGRAM_JSON_RS"
require_fixed "phase-1 compatibility bridge emit-program route" "$RUST_STAGE1_PROGRAM_JSON_RS"
require_fixed "current phase-1 compatibility mode contract" "$RUST_STAGE1_PROGRAM_JSON_RS"
require_fixed "phase-1 compatibility Program(JSON v0)" "$RUST_STAGE1_PROGRAM_JSON_ROUTING_RS"
require_fixed "Phase-1 Compatibility Program JSON v0 Layout" "$RUST_STAGE1_PROGRAM_JSON_README"
if rg -n 'Stage1 Rust Boundary|Rust-side Stage1 bootstrap boundary|Rust-owned Stage1 source|artifact-stage directory|move Stage1 bridge orchestration|encode Stage2 artifact flow|Rust Stage1 bootstrap boundary|Stage1 Program\(JSON v0\) façade|Rust Stage1 bridge emit-program route|current stage1 mode contract|SSOT for Stage1 Program|# Stage1 Program JSON v0 Layout' \
  "$RUST_STAGE1_README" \
  "$RUST_STAGE1_MOD_RS" \
  "$RUST_STAGE1_PROGRAM_JSON_RS" \
  "$RUST_STAGE1_PROGRAM_JSON_ROUTING_RS" \
  "$RUST_STAGE1_PROGRAM_JSON_README"; then
  guard_fail "$TAG" "src/stage1 boundary comments must use phase-1 compatibility wording"
fi
require_fixed "mode-B compatibility module payload generation" "$STAGE1_BRIDGE_README"
require_fixed "mode-B compatibility aliases" "$STAGE1_BRIDGE_ENV_RS"
require_fixed "mode-B compatibility alias" "$STAGE1_BRIDGE_MODULES_RS"
require_fixed "mode-B compatibility readers" "$STAGE1_BRIDGE_MODULES_RS"
if rg -n 'Stage-B module payload|Stage-B compatibility aliases|compatibility alias for Stage-B routes|Stage-B compatibility readers' "$STAGE1_BRIDGE_README" "$STAGE1_BRIDGE_ENV_RS" "$STAGE1_BRIDGE_MODULES_RS"; then
  guard_fail "$TAG" "stage1 bridge module/env docs must say mode-B compatibility"
fi
require_fixed "Phase-1 Compatibility Bridge" "$STAGE1_BRIDGE_README"
require_fixed "Phase-1 compatibility CLI bridge" "$ROOT_DIR/src/runner/stage1_bridge/mod.rs"
require_fixed "Phase-1 stub route facade" "$STAGE1_BRIDGE_README"
require_fixed "Phase-1 compatibility bridge route executor" "$ROOT_DIR/src/runner/stage1_bridge/route_exec.rs"
require_fixed "Phase-1 compatibility bridge stub child command builder" "$ROOT_DIR/src/runner/stage1_bridge/stub_child.rs"
require_fixed "Phase-1 compatibility bridge Program(JSON v0) emit facade" "$ROOT_DIR/src/runner/stage1_bridge/program_json/mod.rs"
if rg -n 'Stage[‑-]1 CLI bridge|Stage[‑-]1 bridge|Stage1 bridge|Stage1 stub|stage1 stub route|stage1-stub execution|Stage1 CLI|Stage1 args|Stage[‑-]1 alias|Stage[‑-]1 using|Stage0 thin|Stage1 / Stage2' "${STAGE1_BRIDGE_PHASE_COMMENT_FILES[@]}"; then
  guard_fail "$TAG" "stage1 bridge README/comments must use phase-1 compatibility wording for migrated comments"
fi
require_fixed 'visible_alias("syntax-3")' "$CLI_ARGS_RS"
require_fixed "syntax3_alias_sets_stage3_parser_flag" "$CLI_ARGS_TESTS_RS"
require_fixed "syntax-3 keyword diagnostic" "$MIR_BUILDER_BUILD_RS"
require_fixed "mode-B compatibility routes" "$MIR_BUILDER_BUILD_RS"
if rg -n 'Stage-3 keyword|for Stage-B' "$MIR_BUILDER_BUILD_RS"; then
  guard_fail "$TAG" "MIR builder undefined-variable hints must use syntax-3 / mode-B wording"
fi
require_fixed 'args.push("--syntax-3".to_string())' "$SELFHOST_STAGE_A_SPAWN_RS"
require_fixed "mode-A compatibility spawn payload builders" "$SELFHOST_STAGE_A_SPAWN_RS"
require_fixed "mode-B compatibility contract expects raw source" "$SELFHOST_STAGE_A_SPAWN_RS"
require_fixed "mode-A compatibility Program(JSON v0) keep boundary" "$SELFHOST_STAGE_A_COMPAT_BRIDGE_RS"
require_fixed "mode-A compatibility keep: Program(JSON v0)" "$SELFHOST_STAGE_A_COMPAT_BRIDGE_RS"
require_fixed "mode-A compatibility route orchestration helper" "$SELFHOST_STAGE_A_ROUTE_RS"
require_fixed "mode-A compatibility runtime route policy helpers" "$SELFHOST_STAGE_A_POLICY_RS"
require_fixed "Resolve mode-A compatibility child payload ownership boundary" "$SELFHOST_COMMON_JSON_RS"
require_fixed "Route-neutral bootstrap capture plumbing" "$SELFHOST_STAGE0_CAPTURE_RS"
require_fixed "Bootstrap capture route builders" "$SELFHOST_STAGE0_CAPTURE_ROUTE_RS"
require_fixed "Build a bootstrap capture command for the requested backend" "$SELFHOST_STAGE0_CAPTURE_ROUTE_RS"
require_fixed "Build a bootstrap capture command for non-VM routes" "$SELFHOST_STAGE0_CAPTURE_ROUTE_RS"
require_fixed "VM-backed bootstrap capture route" "$SELFHOST_STAGE0_CAPTURE_ROUTE_RS"
require_fixed "current mode-A compatibility routes use the non-VM builder" "$SELFHOST_STAGE0_CAPTURE_ROUTE_RS"
require_fixed "mode-A compatibility child spawn/setup" "$RUNNER_SELFHOST_RS"
if rg -n 'Route-neutral Stage0 capture|Stage0 capture route builders|Build a stage0 capture command|VM-backed stage0 capture route|Stage-A (Program|spawn|route|runtime|payload|compat|child|routes|capture)|stage-a compat keep|stage-a compat fallback' "$SELFHOST_STAGE_A_SPAWN_RS" "$SELFHOST_STAGE_A_COMPAT_BRIDGE_RS" "$SELFHOST_STAGE_A_ROUTE_RS" "$SELFHOST_STAGE_A_POLICY_RS" "$SELFHOST_COMMON_JSON_RS" "$SELFHOST_STAGE0_CAPTURE_RS" "$SELFHOST_STAGE0_CAPTURE_ROUTE_RS" "$RUNNER_SELFHOST_RS"; then
  guard_fail "$TAG" "selfhost mode-A compatibility route wording must not reintroduce Stage-A comments/logs"
fi
require_fixed 'token == "--syntax-3" || token == "--stage3"' "$HH_COMPILER_ENTRY"
require_fixed 't == "--syntax-3" || t == "--stage3"' "$HH_STAGEB_ARGS"
require_fixed "mode-B compatibility emit/adapter lane" "$HH_COMPILER_README"
require_fixed "mode-B compatibility compiler entry" "$HH_COMPILER_STAGEB_ENTRY"
require_fixed "mode-B compatibility boundary" "$HH_COMPILER_STAGEB_ENTRY"
require_fixed "mode-B compatibility adapter lane" "$HH_COMPILER_STAGEB_ENTRY"
require_fixed "mode-B compatibility entry" "$HH_STAGEB_ARGS"
require_fixed "mode-B compatibility CLI token packaging" "$HH_STAGEB_BUILD_OPTIONS"
require_fixed "mode-B compatibility entry-local handoff" "$HH_STAGEB_COMPILE_ADAPTER"
require_fixed "mode-B compatibility entry" "$HH_STAGEB_OUTPUT"
if rg -n 'Stage-B (emit/adapter lane|compiler entry|boundary|entry should|adapter lane|CLI token packaging|entry-local handoff|entry\.)' "$HH_COMPILER_README" "$HH_COMPILER_STAGEB_ENTRY" "$HH_STAGEB_ARGS" "$HH_STAGEB_BUILD_OPTIONS" "$HH_STAGEB_COMPILE_ADAPTER" "$HH_STAGEB_OUTPUT"; then
  guard_fail "$TAG" "HHako entry comments must say mode-B compatibility, not Stage-B"
fi
require_fixed "mode-B compatibility adapter module" "$HH_COMPILER_ENTRY"
require_fixed "mode-A compatibility flags" "$HH_COMPILER_ENTRY"
require_fixed "mode-B compatibility flags" "$HH_COMPILER_ENTRY"
require_fixed "Minimal parser utilities (mode-A compatibility)" "$HH_COMPILER_ENTRY"
require_fixed "mode-A compatibility fallback behavior" "$HH_COMPILER_ENTRY"
require_fixed "mode-B compatibility route is SSOT" "$HH_COMPILER_ENTRY"
require_fixed "String indexing not supported in mode-A compatibility" "$HH_COMPILER_ENTRY"
if rg -n 'Stage-A flags|Stage-B flags|Minimal parser utilities \(Stage-A\)|Stage-A fallback behavior|Stage-B SSOT helper|unsupported in Stage-A|String indexing not supported in Stage-A|Stage.B 経路|Stage-A は --min-json' "$HH_COMPILER_ENTRY"; then
  guard_fail "$TAG" "compiler.hako route comments/diagnostics must say mode-A/mode-B compatibility"
fi
require_fixed "legacy mode-B compatibility bundling resolver fixture" "$HH_BUNDLE_RESOLVER"
require_fixed "Live mode-B compatibility source-to-Program production goes through BuildBox" "$HH_BUNDLE_RESOLVER"
require_fixed "mode-B compatibility bundler" "$HH_BUNDLE_RESOLVER"
require_fixed "Live mode-B compatibility source-to-Program production goes through BuildBox" "$HH_STAGEB_BODY_EXTRACTOR"
require_fixed "mode-B compatibility body extractor" "$HH_STAGEB_BODY_EXTRACTOR"
require_fixed "mode-B compatibility parser" "$HH_STAGEB_BODY_EXTRACTOR"
require_fixed "mode-B compatibility keyword expression JSON cleanup" "$HH_STAGEB_KEYWORD_EXPR_STRIP"
if rg -n 'legacy Stage-B bundling resolver|Live Stage-B source-to-Program|Stage.B bundler|inside Stage.B body extractor|for Stage-B parser|Stage-B keyword expression JSON cleanup' "$HH_BUNDLE_RESOLVER" "$HH_STAGEB_BODY_EXTRACTOR" "$HH_STAGEB_KEYWORD_EXPR_STRIP"; then
  guard_fail "$TAG" "HHako compat fixture comments must say mode-B compatibility"
fi
require_fixed "mode-B compatibility driver entry trace/depth guard helpers" "$HH_STAGEB_DRIVER_GUARD"
require_fixed "trace helper for mode-B compatibility" "$HH_STAGEB_TRACE"
require_fixed "Keeps mode-B compatibility behavior unchanged" "$HH_STAGEB_TRACE"
require_fixed "mode-B compatibility main/body pattern detection helper" "$HH_STAGEB_MAIN_DETECTION"
require_fixed "mode-A compatibility fallback dependency" "$HH_STAGEB_MAIN_DETECTION"
require_fixed "Used by both mode-B and mode-A compatibility fallback paths" "$HH_STAGEB_MAIN_DETECTION"
require_fixed "mode-B compatibility source-route Rune helper" "$HH_STAGEB_RUNE"
require_fixed "mode-B compatibility user-box field declaration scanner" "$HH_STAGEB_USER_BOX_DECL_SCANNER"
if rg -n 'Stage-B driver entry|helper for Stage-B|Keeps Stage-B behavior|Stage-B main/body|Stage-A fallback dependency|Used by both Stage-B and Stage-A fallback|Stage-B/source-route Rune helper|Stage-B user-box field declaration scanner' "$HH_STAGEB_DRIVER_GUARD" "$HH_STAGEB_TRACE" "$HH_STAGEB_MAIN_DETECTION" "$HH_STAGEB_RUNE" "$HH_STAGEB_USER_BOX_DECL_SCANNER"; then
  guard_fail "$TAG" "HHako helper comments must say mode-B/mode-A compatibility, not Stage-A/B"
fi
require_fixed "mode-B compatibility Program(JSON) stdout capture remains an explicit compat/debug" "$STAGEB_PROGRAM_JSON_CAPTURE_CALLER_GUARD"
require_fixed "known MIR emit / mode-B helper surfaces only" "$STAGEB_PROGRAM_JSON_CAPTURE_CALLER_GUARD"
require_fixed "keep mode-B compatibility Program(JSON) capture behind hakorune_emit_mir.sh or mode-B helper surfaces only" "$STAGEB_PROGRAM_JSON_CAPTURE_CALLER_GUARD"
if rg -n 'Stage-B Program\(JSON\) capture|Stage-B helper surfaces|Stage-B Program\(JSON\) stdout capture' "$STAGEB_PROGRAM_JSON_CAPTURE_CALLER_GUARD" "$QUICK_STEPS"; then
  guard_fail "$TAG" "Program(JSON) capture caller guard wording must say mode-B compatibility"
fi
require_fixed "lowers to the phase-1 compatibility runtime helper" "$STAGE1_EMIT_PROGRAM_JSON_RUNTIME_HELPER_GUARD"
require_fixed "phase-1 compatibility Program(JSON) execution is a probe-only route" "$STAGE1_PROGRAM_JSON_COMPAT_CALLER_GUARD"
require_fixed "keep phase-1 compatibility Program(JSON) execution behind phase29ch explicit probe only" "$STAGE1_PROGRAM_JSON_COMPAT_CALLER_GUARD"
if rg -n 'Stage1 emit-program runtime-helper guard|Stage1 Program\(JSON\) compat caller guard|Stage1 runtime helper|Stage1 Program\(JSON\) compat execution' "$QUICK_STEPS" "$STAGE1_EMIT_PROGRAM_JSON_RUNTIME_HELPER_GUARD" "$STAGE1_PROGRAM_JSON_COMPAT_CALLER_GUARD"; then
  guard_fail "$TAG" "Stage1 Program(JSON) guard wording must say phase-1 compatibility"
fi
if rg -n 'Stage0 shape inventory guard' "$QUICK_STEPS"; then
  guard_fail "$TAG" "Stage0 shape quick-gate label must say GlobalCallTarget shape inventory guard"
fi
require_fixed "Function definition scanner for mode-B compatibility compiler" "$HH_FUNC_SCANNER"
require_fixed "mode-B compatibility VM path can lose string-state" "$HH_FUNC_SCANNER_HELPERS"
if rg -n 'Function definition scanner for Stage-B compiler|Stage-B VM path can lose string-state' "$HH_FUNC_SCANNER" "$HH_FUNC_SCANNER_HELPERS"; then
  guard_fail "$TAG" "FuncScanner comments must say mode-B compatibility"
fi
require_fixed "bundle-aware mode-B compatibility adapter" "$HH_BUILD_BUNDLE_FACADE"
require_fixed "bootstrap source execution does not import bundle" "$HH_BUILD_BUNDLE_FACADE"
require_fixed "Live mode-B compatibility bundle entry" "$HH_BUILD_README"
require_fixed "mode-A bridge callers" "$HH_MIRBUILDER_README"
require_fixed "mode-B compatibility Program(JSON v0)" "$HH_MIRBUILDER_README"
require_fixed "mode-B compatibility MIR emitter can stall" "$HH_PARSER_CONTROL_BOX"
require_fixed "disabled in mode-B compatibility runtime" "$HH_PARSER_CONTROL_BOX"
require_fixed "syntax-3 path" "$HH_PARSER_CONTROL_BOX"
require_fixed "mode-B compatibility/selfhost safety valve" "$HH_PARSER_STMT_CORE"
require_fixed "mode-B compatibility compiler code" "$HH_PARSER_STMT_CORE"
require_fixed "syntax-3: try-less postfix handler path" "$HH_PARSER_STMT_CORE"
require_fixed "mode-B compatibility / selfhost callers" "$HH_PARSER_BOX"
require_fixed "mode-B compatibility/selfhost callers" "$HH_PARSER_BOX"
require_fixed "syntax-3 only" "$HH_PARSER_BOX"
require_fixed "mode-B compatibility VM quirks" "$HH_PARSER_CONTROL_BOX"
require_fixed "Required by mode-B compatibility fixtures" "$HH_PARSER_EXPR_BOX"
require_fixed "mode-B compatibility parser routes" "$HH_PARSER_RUNE_CONTRACT"
require_fixed "mode-B compatibility VM path can lose" "$HH_PARSER_NUMBER_SCAN"
require_fixed "syntax-3" "$HH_PARSER_EXCEPTION_BOX"
if rg -n 'Stage-B adapter|Live Stage-B bundle entry|Stage-A bridge callers|Stage-B Program\(JSON v0\)|Stage-B MIR emitter|Stage-B runtime|Stage-B/selfhost|Stage-B compiler code|Stage-B fixtures|Stage-B parser routes|Stage-B VM path|Stage-3|Stage.B' \
  "$HH_BUILD_BUNDLE_FACADE" \
  "$HH_BUILD_README" \
  "$HH_MIRBUILDER_README" \
  "$HH_PARSER_CONTROL_BOX" \
  "$HH_PARSER_STMT_CORE" \
  "$HH_PARSER_BOX" \
  "$HH_PARSER_EXCEPTION_BOX" \
  "$HH_PARSER_EXPR_BOX" \
  "$HH_PARSER_RUNE_CONTRACT" \
  "$HH_PARSER_NUMBER_SCAN"; then
  guard_fail "$TAG" "HHako parser/build comments must say mode-A/mode-B compatibility or syntax-3"
fi
require_fixed "mode-B compatibility delegate call" "$HH_TEST_FUNCSCANNER_SKIP_WS"
require_fixed "mode-B compatibility path" "$HH_TEST_FUNCSCANNER_SKIP_WS"
require_fixed "mode-B compatibility minimal test harness" "$HH_TEST_STAGEB_MIN_SAMPLE"
require_fixed "mode-B compatibility compilation without SSA errors" "$HH_TEST_STAGEB_MIN_SAMPLE"
if rg -n 'Stage0 source execution|Stage-B delegate call|closer to Stage-B path|Stage-B minimal test harness|Stage-B compilation' \
  "$HH_BUILD_BUNDLE_FACADE" \
  "$HH_TEST_FUNCSCANNER_SKIP_WS" \
  "$HH_TEST_STAGEB_MIN_SAMPLE"; then
  guard_fail "$TAG" "HHako build/test comments must say bootstrap or mode-B compatibility"
fi
require_fixed "phase-1 JSON" "$ROOT_DIR/lang/src/compiler/pipeline_v2/README.md"
require_fixed "bootstrap/Resolver" "$ROOT_DIR/lang/src/compiler/pipeline_v2/README.md"
require_fixed "Pipeline Guard（phase-2 / phase-3）" "$ROOT_DIR/lang/src/compiler/pipeline_v2/README.md"
require_fixed "phase-1 guard compatibility names" "$ROOT_DIR/lang/src/compiler/pipeline_v2/README.md"
require_fixed "phase-2/3 compatibility names" "$ROOT_DIR/lang/src/compiler/pipeline_v2/README.md"
require_fixed "using-alias heads in phase-1 names" "$ROOT_DIR/lang/src/compiler/pipeline_v2/alias_preflight_box.hako"
require_fixed "phase-1 JSON から Return(Call" "$ROOT_DIR/lang/src/compiler/pipeline_v2/call_extract_box.hako"
require_fixed "phase-1 JSON から Compare" "$ROOT_DIR/lang/src/compiler/pipeline_v2/compare_extract_box.hako"
require_fixed "syntax-3 acceptance in parser" "$ROOT_DIR/lang/src/compiler/pipeline_v2/execution_pipeline_box.hako"
require_fixed "Emit phase-1 JSON with meta.usings" "$ROOT_DIR/lang/src/compiler/pipeline_v2/execution_pipeline_box.hako"
require_fixed "mode-B compatibility entry" "$ROOT_DIR/lang/src/compiler/pipeline_v2/flow_entry.hako"
require_fixed "minimal phase-1 JSON header" "$ROOT_DIR/lang/src/compiler/pipeline_v2/header_emit_box.hako"
require_fixed "phase-1 JSON から Return(Method" "$ROOT_DIR/lang/src/compiler/pipeline_v2/method_extract_box.hako"
require_fixed "Accept phase-1 AST JSON" "$ROOT_DIR/lang/src/compiler/pipeline_v2/mir_builder_box.hako"
require_fixed "phase-1 JSON から Return(New" "$ROOT_DIR/lang/src/compiler/pipeline_v2/new_extract_box.hako"
require_fixed "Lightweight phase-1 scanner" "$ROOT_DIR/lang/src/compiler/pipeline_v2/pipeline.hako"
require_fixed "Compare values from phase-1 JSON" "$ROOT_DIR/lang/src/compiler/pipeline_v2/pipeline_helpers_box.hako"
require_fixed "phase-1 JSON at this phase" "$ROOT_DIR/lang/src/compiler/pipeline_v2/signature_verifier_box.hako"
require_fixed "phase-1 args JSON" "$ROOT_DIR/lang/src/compiler/pipeline_v2/signature_verifier_box.hako"
require_fixed "Parse phase-1 args JSON" "$ROOT_DIR/lang/src/compiler/pipeline_v2/stage1_args_parser_box.hako"
require_fixed "phase-1 JSON (Return Int / BinOp / Compare)" "$ROOT_DIR/lang/src/compiler/pipeline_v2/stage1_extract_flow.hako"
require_fixed "phase-1 JSON strings used by PipelineV2" "$ROOT_DIR/lang/src/compiler/pipeline_v2/stage1_json_scanner_box.hako"
require_fixed "from phase-1 scanner" "$ROOT_DIR/lang/src/compiler/pipeline_v2/stage1_name_args_normalizer_box.hako"
if rg -n 'Stage[‑-]1 JSON|Stage[‑-]1 AST JSON|Stage[‑-]1 args JSON|Stage[‑-]1 scanner|Stage[‑-]1 names|Stage[‑-]0/Resolver|Stage Guard|Stage[‑-]2:|Stage[‑-]3:|Stage[‑-]B entry|Stage[‑-]3 acceptance' "${PIPELINE_V2_STAGE_COMMENT_FILES[@]}"; then
  guard_fail "$TAG" "pipeline_v2 comments must say phase-1 / phase-2 / phase-3, mode-B compatibility, or syntax-3"
fi
require_fixed "lower-resolver compatibility pass" "$ROOT_DIR/src/mir/join_ir/lowering/value_id_ranges.rs"
require_fixed "mode-B compatibility body extractor" "$ROOT_DIR/src/mir/join_ir/lowering/value_id_ranges.rs"
require_fixed "mode-B compatibility FuncScanner" "$ROOT_DIR/src/mir/join_ir/lowering/value_id_ranges.rs"
require_fixed "lower-resolver compatibility entries loop lowering" "$ROOT_DIR/src/mir/join_ir/lowering/mod.rs"
require_fixed "phase-1 compatibility 実用関数" "$ROOT_DIR/src/mir/join_ir/lowering/mod.rs"
require_fixed "mode-B compatibility 実用関数" "$ROOT_DIR/src/mir/join_ir/lowering/mod.rs"
require_fixed "tests / phase-1 compatibility / explicit approvals" "$ROOT_DIR/src/mir/join_ir/lowering/if_lowering_router.rs"
require_fixed "phase-1 compatibility rollout" "$ROOT_DIR/src/mir/join_ir/lowering/if_lowering_router.rs"
require_fixed "lower-resolver lowerer" "$ROOT_DIR/src/mir/join_ir/lowering/loop_view_builder.rs"
if rg -n 'Stage[‑-]1 using resolver|Stage[‑-]1 UsingResolver|Stage[‑-]B body extractor|Stage[‑-]B FuncScanner|Stage[‑-]1 実用関数|Stage[‑-]B 実用関数|stage1 lowerer|stage1 用|Stage[‑-]1 rollout|Stage1 keeps' "${JOINIR_LOWERING_STAGE_COMMENT_FILES[@]}"; then
  guard_fail "$TAG" "JoinIR lowering comments must say lower-resolver, phase-1 compatibility, or mode-B compatibility"
fi
require_fixed "mode-B compatibility currently emits this as a statement wrapper" "$JSON_V0_BRIDGE_AST_RS"
require_fixed 'mode-B compatibility legacy encoding for `if !(cond) { ... }`' "$JSON_V0_BRIDGE_IF_LEGACY_RS"
require_fixed "mode-B legacy if-not: invalid BlockExpr.tail" "$JSON_V0_BRIDGE_IF_LEGACY_RS"
require_fixed 'mode-B compatibility legacy encoding: `fn(x) { ... }`' "$JSON_V0_BRIDGE_LAMBDA_LEGACY_RS"
require_fixed "mode-B compatibility / JSON v0" "$JSON_V0_BRIDGE_LOOP_RS"
require_fixed "no bootstrap desugar" "$JSON_V0_BRIDGE_LOOP_RANGE_RS"
require_fixed "mode-B compatibility JSON often uses" "$JSON_V0_BRIDGE_PROGRAM_RS"
require_fixed "mode-B compatibility currently emits tail" "$JSON_V0_BRIDGE_BLOCK_EXPR_RS"
if rg -n 'Stage-B currently emits|Stage-B legacy encoding|Stage-B JSON often uses|Stage-B / JSON v0|no Stage0 desugar|stageb legacy if-not' \
  "$JSON_V0_BRIDGE_AST_RS" \
  "$JSON_V0_BRIDGE_IF_LEGACY_RS" \
  "$JSON_V0_BRIDGE_LAMBDA_LEGACY_RS" \
  "$JSON_V0_BRIDGE_LOOP_RS" \
  "$JSON_V0_BRIDGE_LOOP_RANGE_RS" \
  "$JSON_V0_BRIDGE_PROGRAM_RS" \
  "$JSON_V0_BRIDGE_BLOCK_EXPR_RS"; then
  guard_fail "$TAG" "JSON v0 bridge comments/diagnostics must say mode-B compatibility or bootstrap"
fi
require_fixed "mode-B compatibility user_box_decls scanner probe failed" "$K2_WIDE_STAGEB_FIELD_TYPE_ANNOTATION_GUARD"
require_fixed "mode-B compatibility parser route failed" "$K2_WIDE_STAGEB_NUMERIC_LITERAL_SUFFIX_GUARD"
if rg -n 'Stage-B user_box_decls scanner probe failed|Stage-B parser route failed' "$K2_WIDE_STAGEB_FIELD_TYPE_ANNOTATION_GUARD" "$K2_WIDE_STAGEB_NUMERIC_LITERAL_SUFFIX_GUARD"; then
  guard_fail "$TAG" "K2-wide stageb guard diagnostics must say mode-B compatibility"
fi
require_fixed "selfhost mode-B compatibility wrapper missing/executable" "$SELFHOST_PLANNER_REQUIRED_DEV_GATE"
require_fixed "keep mode-B compatibility compiler route" "$SELFHOST_PLANNER_REQUIRED_DEV_GATE"
require_fixed "Pin mode-B compatibility FuncScanner delegated box header" "$SELFHOST_STAGEB_FUNCSCANNER_BOX_SMOKE"
require_fixed "Pin mode-B compatibility FuncScanner method-decl boundary" "$SELFHOST_STAGEB_FUNCSCANNER_METHOD_BOUNDARY_SMOKE"
require_fixed "Pin mode-B compatibility legacy lambda pair" "$SELFHOST_STAGEB_LAMBDA_LITERAL_PAIR_SMOKE"
require_fixed "Pin mode-B compatibility FuncScanner parity" "$SELFHOST_STAGEB_FUNCSCANNER_TYPED_PARAMS_SMOKE"
require_fixed "selfhost mode-B compatibility route parity smoke" "$SELFHOST_STEADY_STATE_SMOKE"
require_fixed "Compare mode-B compatibility output" "$SELFHOST_STAGEB_ROUTE_PARITY_SMOKE"
require_fixed "Contract smoke for phase-1 compatibility bootstrap capability" "$SELFHOST_STAGE1_CONTRACT_SMOKE"
require_fixed "selfhost syntax-3 stable paths smoke test" "$SELFHOST_STABLE_PATHS_SMOKE"
if rg -n 'selfhost Stage-B wrapper missing/executable|keep Stage-B compiler route|Pin Stage-B FuncScanner delegated box header|Pin Stage-B legacy lambda pair|Pin Stage-B FuncScanner method-decl boundary|Pin Stage-B FuncScanner parity|selfhost Stage-B route parity smoke|Compare Stage-B output|Contract smoke for Stage1 bootstrap capability|selfhost Stage-3 stable paths smoke test' \
  "$SELFHOST_PLANNER_REQUIRED_DEV_GATE" \
  "$SELFHOST_STAGEB_FUNCSCANNER_BOX_SMOKE" \
  "$SELFHOST_STAGEB_FUNCSCANNER_METHOD_BOUNDARY_SMOKE" \
  "$SELFHOST_STAGEB_LAMBDA_LITERAL_PAIR_SMOKE" \
  "$SELFHOST_STAGEB_FUNCSCANNER_TYPED_PARAMS_SMOKE" \
  "$SELFHOST_STEADY_STATE_SMOKE" \
  "$SELFHOST_STAGEB_ROUTE_PARITY_SMOKE" \
  "$SELFHOST_STAGE1_CONTRACT_SMOKE" \
  "$SELFHOST_STABLE_PATHS_SMOKE"; then
  guard_fail "$TAG" "selfhost smoke comments/diagnostics must say mode-B, phase-1, or syntax-3 compatibility"
fi
require_fixed '.arg("--syntax-3")' "$PHASE29CI_STAGEB_BODY_EXTRACT_TEST"
require_fixed '`--syntax-3` with compatibility alias `--stage3`' "$REFERENCE_EBNF"
require_fixed 'selfhost: `--syntax-3`' "$REFERENCE_STATEMENTS"
require_fixed '--syntax-3` (syntax-3 surface enable; `--stage3` remains a compatibility alias)' "$SELFHOST_QUICKSTART"
require_fixed "phase-1 compatibility Program(JSON v0) runtime helper route" "$CHECK_INDEX"
require_fixed "GlobalCallTarget shape inventory SSOT" "$CHECK_INDEX"
require_fixed "mode-B compatibility Program(JSON) stdout capture helper" "$CHECK_INDEX"
require_fixed 'mode-B compatibility `.hako` parser' "$CHECK_INDEX"
require_fixed "mode-B compatibility FuncScanner / JSON builder" "$CHECK_INDEX"
require_fixed "mode-B compatibility enrichment seam" "$CHECK_INDEX"
require_fixed "phase-1 compatibility Program(JSON) compat execution helper" "$CHECK_INDEX"
if rg -n 'Stage1 Program\(JSON v0\) runtime helper route|Stage0 LLVM line shape inventory SSOT|Stage-B Program\(JSON\) stdout capture helper|Stage-B `.hako` parser|Stage-B FuncScanner / JSON builder|Stage-B enrichment seam|Stage1 Program\(JSON\) compat execution helper' "$CHECK_INDEX"; then
  guard_fail "$TAG" "check scripts index descriptions must use mode-B / phase-1 / GlobalCallTarget wording for migrated guards"
fi
require_fixed "phase1-route=fail" "$ROOT_DIR/docs/tools/README.md"
require_fixed "phase-1 compatibility 直接実行" "$ROOT_DIR/docs/tools/script-index.md"
require_fixed "phase-1 CLI compatibility 実行ヘルパ" "$ROOT_DIR/docs/tools/script-index.md"
require_fixed "syntax-3 same-result sanity check" "$ROOT_DIR/docs/tools/script-index.md"
if rg -n 'stage1-route|stage1 直接実行|Stage1 CLI 実行ヘルパ|current Stage1 shell compat|Stage3 same-result' "$ROOT_DIR/docs/tools/README.md" "$ROOT_DIR/docs/tools/script-index.md"; then
  guard_fail "$TAG" "docs/tools quick entries must use phase-1 / syntax-3 wording for migrated rows"
fi
require_fixed "STAGE-TERM-EXISTING-NAME-INVENTORY-001" "$STAGE_TERM_INVENTORY"
require_fixed "classification-only inventory" "$STAGE_TERM_INVENTORY"
require_fixed "direct renames are forbidden" "$STAGE_TERM_INVENTORY"
require_fixed "hakorune-stage-term-existing-name-migration-inventory.md" "$SSOT"
require_fixed "tools/checks/naming_charter_guard.sh" "$CHECK_INDEX"
require_fixed "naming_charter_guard.sh" "$QUICK_STEPS"
require_fixed "hakorune-naming-and-rename-task-order-ssot.md" "$DOCS_LAYOUT"

NAMING_DIFF_ALLOWED_PATHS=(
  "docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md"
  "docs/development/current/main/design/hakorune-stage-term-existing-name-migration-inventory.md"
  "docs/reference/language/EBNF.md"
  "docs/reference/language/statements.md"
  "lang/README.md"
  "docs/tools/README.md"
  "docs/tools/script-index.md"
  "docs/reference/environment-variables.md"
  "src/config/env/stage1.rs"
  "src/stage1/README.md"
  "src/stage1/mod.rs"
  "src/stage1/program_json_v0.rs"
  "src/stage1/program_json_v0/routing.rs"
  "src/stage1/program_json_v0/README.md"
  "src/config/env/parser_flags.rs"
  "src/config/env/selfhost_flags.rs"
  "src/cli/args.rs"
  "src/cli/args/tests.rs"
  "src/config/env/verification_flags.rs"
  "src/mir/builder/builder_build.rs"
  "src/runner/stage1_bridge/README.md"
  "src/runner/stage1_bridge/mod.rs"
  "src/runner/stage1_bridge/args.rs"
  "src/runner/stage1_bridge/env.rs"
  "src/runner/stage1_bridge/modules.rs"
  "src/runner/stage1_bridge/entry_guard.rs"
  "src/runner/stage1_bridge/emit_paths.rs"
  "src/runner/stage1_bridge/plan.rs"
  "src/runner/stage1_bridge/route_exec.rs"
  "src/runner/stage1_bridge/route_exec/README.md"
  "src/runner/stage1_bridge/route_exec/direct.rs"
  "src/runner/stage1_bridge/route_exec/stub.rs"
  "src/runner/stage1_bridge/direct_route/README.md"
  "src/runner/stage1_bridge/direct_route/mod.rs"
  "src/runner/stage1_bridge/direct_route/compile.rs"
  "src/runner/stage1_bridge/direct_route/emit.rs"
  "src/runner/stage1_bridge/env/README.md"
  "src/runner/stage1_bridge/env/parser_stageb.rs"
  "src/runner/stage1_bridge/env/runtime_defaults.rs"
  "src/runner/stage1_bridge/env/stage1_aliases.rs"
  "src/runner/stage1_bridge/program_json/README.md"
  "src/runner/stage1_bridge/program_json/mod.rs"
  "src/runner/stage1_bridge/program_json_entry/README.md"
  "src/runner/stage1_bridge/stub_child.rs"
  "src/runner/stage1_bridge/stub_delegate.rs"
  "src/runner/stage1_bridge/stub_emit.rs"
  "src/runner/stage1_bridge/stub_emit/README.md"
  "src/runner/stage1_bridge/stub_emit/parse.rs"
  "src/runner/stage1_bridge/stub_emit/writeback.rs"
  "src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs"
  "src/runner/modes/common_util/selfhost/stage_a_route.rs"
  "src/runner/modes/common_util/selfhost/stage_a_policy.rs"
  "src/runner/modes/common_util/selfhost/stage_a_spawn.rs"
  "src/runner/modes/common_util/selfhost/json.rs"
  "src/runner/modes/common_util/selfhost/stage0_capture_route.rs"
  "src/runner/selfhost.rs"
  "lang/src/compiler/entry/compiler.hako"
  "lang/src/compiler/README.md"
  "lang/src/compiler/entry/compiler_stageb.hako"
  "lang/src/compiler/entry/stageb_args_box.hako"
  "lang/src/compiler/entry/stageb_build_options_box.hako"
  "lang/src/compiler/entry/stageb_compile_adapter_box.hako"
  "lang/src/compiler/entry/stageb_output_box.hako"
  "lang/src/compiler/entry/bundle_resolver.hako"
  "lang/src/compiler/entry/stageb_body_extractor_box.hako"
  "lang/src/compiler/entry/stageb_keyword_expr_strip_box.hako"
  "lang/src/compiler/entry/stageb_driver_guard_box.hako"
  "lang/src/compiler/entry/stageb_trace_box.hako"
  "lang/src/compiler/entry/stageb_main_detection_box.hako"
  "lang/src/compiler/entry/stageb/stageb_rune_box.hako"
  "lang/src/compiler/entry/stageb/stageb_user_box_decl_scanner_box.hako"
  "lang/src/compiler/entry/func_scanner.hako"
  "lang/src/compiler/entry/func_scanner_helpers.hako"
  "lang/src/compiler/build/README.md"
  "lang/src/compiler/build/build_bundle_facade_box.hako"
  "lang/src/compiler/mirbuilder/README.md"
  "lang/src/compiler/parser/parser_box.hako"
  "lang/src/compiler/parser/expr/parser_expr_box.hako"
  "lang/src/compiler/parser/scan/parser_number_scan_box.hako"
  "lang/src/compiler/parser/rune/rune_contract_box.hako"
  "lang/src/compiler/parser/stmt/parser_control_box.hako"
  "lang/src/compiler/parser/stmt/parser_exception_box.hako"
  "lang/src/compiler/parser/stmt/parser_stmt_box/core.hako"
  "lang/src/compiler/tests/funcscanner_skip_ws_min.hako"
  "lang/src/compiler/tests/stageb_min_sample.hako"
  "tools/checks/k2_wide_stageb_field_type_annotation_alignment_guard.sh"
  "tools/checks/k2_wide_stageb_numeric_literal_suffix_alignment_guard.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase120_stable_paths.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_planner_required_dev_gate_vm.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stage1_contract_smoke_vm.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_lambda_literal_pair_min_vm.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_steady_state_vm.sh"
  "tools/smokes/v2/profiles/integration/selfhost/phase29cc_selfhost_stageb_funcscanner_typed_params_implements_min_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/phase29y_hako_binary_only_selfhost_readiness_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/phase29y_hako_run_binary_only_ported_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/phase21_5_perf_bench_compile_run_split_contract_vm.sh"
  "tools/smokes/v2/profiles/integration/apps/lib/README.md"
  "tests/phase29ci_stageb_body_extract.rs"
  "src/runner/json_v0_bridge/ast.rs"
  "src/runner/json_v0_bridge/lowering/if_legacy.rs"
  "src/runner/json_v0_bridge/lowering/lambda_legacy.rs"
  "src/runner/json_v0_bridge/lowering/loop_.rs"
  "src/runner/json_v0_bridge/lowering/loop_range.rs"
  "src/runner/json_v0_bridge/lowering/program.rs"
  "src/runner/json_v0_bridge/lowering/expr/block_expr.rs"
  "lang/src/compiler/pipeline_v2/README.md"
  "lang/src/compiler/pipeline_v2/alias_preflight_box.hako"
  "lang/src/compiler/pipeline_v2/call_extract_box.hako"
  "lang/src/compiler/pipeline_v2/compare_extract_box.hako"
  "lang/src/compiler/pipeline_v2/execution_pipeline_box.hako"
  "lang/src/compiler/pipeline_v2/flow_entry.hako"
  "lang/src/compiler/pipeline_v2/header_emit_box.hako"
  "lang/src/compiler/pipeline_v2/method_extract_box.hako"
  "lang/src/compiler/pipeline_v2/mir_builder_box.hako"
  "lang/src/compiler/pipeline_v2/new_extract_box.hako"
  "lang/src/compiler/pipeline_v2/pipeline.hako"
  "lang/src/compiler/pipeline_v2/pipeline_helpers_box.hako"
  "lang/src/compiler/pipeline_v2/signature_verifier_box.hako"
  "lang/src/compiler/pipeline_v2/stage1_args_parser_box.hako"
  "lang/src/compiler/pipeline_v2/stage1_extract_flow.hako"
  "lang/src/compiler/pipeline_v2/stage1_json_scanner_box.hako"
  "lang/src/compiler/pipeline_v2/stage1_name_args_normalizer_box.hako"
  "src/mir/join_ir/lowering/value_id_ranges.rs"
  "src/mir/join_ir/lowering/mod.rs"
  "src/mir/join_ir/lowering/if_lowering_router.rs"
  "src/mir/join_ir/lowering/common/cfg_shape.rs"
  "src/mir/join_ir/lowering/generic_case_a/stage1_using_resolver.rs"
  "src/mir/join_ir/lowering/generic_case_a/mod.rs"
  "src/mir/join_ir/lowering/loop_to_join/case_a_entrypoints.rs"
  "src/mir/join_ir/lowering/stage1_using_resolver/dispatch.rs"
  "src/mir/join_ir/lowering/stage1_using_resolver.rs"
  "src/mir/join_ir/lowering/loop_view_builder.rs"
  "tools/selfhost/proof/run_stageb_compiler_vm.sh"
  "tools/checks/stageb_program_json_capture_caller_guard.sh"
  "tools/checks/stage1_emit_program_json_runtime_helper_guard.sh"
  "tools/checks/stage1_program_json_compat_caller_guard.sh"
  "docs/development/selfhosting/quickstart.md"
  "docs/development/current/main/DOCS_LAYOUT.md"
  "docs/tools/check-scripts-index.md"
  "tools/checks/naming_charter_guard.sh"
  "tools/checks/lib/dev_gate_quick_steps.sh"
  "CURRENT_TASK.md"
)
guard_require_unique_values "naming diff allowed path" "${NAMING_DIFF_ALLOWED_PATHS[@]}"

is_allowed_path() {
  local allowed_path
  for allowed_path in "${NAMING_DIFF_ALLOWED_PATHS[@]}"; do
    if [[ "$1" == "$allowed_path" ]]; then
      return 0
    fi
  done
  return 1
}

check_added_stage_terms_in_diff() {
  local mode="$1"
  local tmp
  local allowed_paths_tmp
  tmp="$(mktemp "/tmp/${TAG}.${mode}.diff.XXXXXX")"
  allowed_paths_tmp="$(mktemp "/tmp/${TAG}.${mode}.allowed.XXXXXX")"
  printf "%s\n" "${NAMING_DIFF_ALLOWED_PATHS[@]}" >"$allowed_paths_tmp"
  if [[ "$mode" == "cached" ]]; then
    git -C "$ROOT_DIR" diff --cached --unified=0 -- >"$tmp"
  else
    git -C "$ROOT_DIR" diff --unified=0 -- >"$tmp"
  fi

  if awk -v allowed_paths_file="$allowed_paths_tmp" '
    BEGIN {
      while ((getline allowed_path < allowed_paths_file) > 0) {
        allowed_paths[allowed_path] = 1
      }
      close(allowed_paths_file)
    }
    /^\+\+\+ b\// {
      file = substr($0, 7)
      allowed = (file in allowed_paths)
      next
    }
    /^\+\+\+ / { next }
    /^\+/ && !allowed && /(^|[^A-Za-z0-9_])(Stage-[A-Za-z0-9_-]+|Stage[0-9]+|stage[0-9]+|stage-[A-Za-z0-9_-]+)/ {
      print
      found = 1
    }
    END { exit found ? 0 : 1 }
  ' "$tmp"; then
    rm -f "$tmp" "$allowed_paths_tmp"
    guard_fail "$TAG" "new unqualified stage term added outside naming charter in ${mode} diff; classify it by layer first"
  fi
  rm -f "$tmp" "$allowed_paths_tmp"
}

check_added_stage_terms_in_untracked() {
  local path
  while IFS= read -r path; do
    [[ -z "$path" ]] && continue
    if is_allowed_path "$path"; then
      continue
    fi
    if [[ -f "$ROOT_DIR/$path" ]] && rg -n '(^|[^A-Za-z0-9_])(Stage-[A-Za-z0-9_-]+|Stage[0-9]+|stage[0-9]+|stage-[A-Za-z0-9_-]+)' "$ROOT_DIR/$path"; then
      guard_fail "$TAG" "new unqualified stage term added in untracked file: $path"
    fi
  done < <(git -C "$ROOT_DIR" ls-files --others --exclude-standard)
}

if git -C "$ROOT_DIR" rev-parse --is-inside-work-tree >/dev/null 2>&1; then
  check_added_stage_terms_in_diff "unstaged"
  check_added_stage_terms_in_diff "cached"
  check_added_stage_terms_in_untracked
fi

echo "[$TAG] ok"
