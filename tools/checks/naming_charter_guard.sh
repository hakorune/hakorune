#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="naming-charter-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

SSOT="$ROOT_DIR/docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md"
CHECK_INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"
QUICK_STEPS="$ROOT_DIR/tools/checks/lib/dev_gate_quick_steps.sh"
DOCS_LAYOUT="$ROOT_DIR/docs/development/current/main/DOCS_LAYOUT.md"
CARGO_TOML="$ROOT_DIR/Cargo.toml"
README_MD="$ROOT_DIR/README.md"
HACO_WRAPPER="$ROOT_DIR/tools/bin/hako"
MAIN_RS="$ROOT_DIR/src/main.rs"
HAKORUNE_BIN_RS="$ROOT_DIR/src/bin/hakorune.rs"
HAKORUNE_COMPAT_BIN_RS="$ROOT_DIR/src/bin/hakorune_compat.rs"
BUILD_SHARED_RS="$ROOT_DIR/src/runner/build_shared.rs"
BUILD_PRODUCT_RS="$ROOT_DIR/src/runner/build_product.rs"
BUILD_ENGINEERING_RS="$ROOT_DIR/src/runner/build_engineering.rs"
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
SELFHOST_STAGEB_PROOF_VM="$ROOT_DIR/tools/selfhost/proof/run_stageb_compiler_vm.sh"
SELFHOST_RUN_ROUTES="$ROOT_DIR/tools/selfhost/lib/selfhost_run_routes.sh"
SELFHOST_BUILD="$ROOT_DIR/tools/selfhost/selfhost_build.sh"
SELFHOST_MAINLINE_BUILD_STAGE1="$ROOT_DIR/tools/selfhost/mainline/build_stage1.sh"
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
SELFHOST_STAGEB_ROUTE_PARITY_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_route_parity_smoke_vm.sh"
COLLECTION_MAP_GET_SHARES_MAP_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/collections/map_get_shares_map.sh"
COLLECTION_MAP_GET_SHARES_ARRAY_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/collections/map_get_shares_array.sh"
COLLECTION_STRING_SIZE_ALIAS_SMOKE="$ROOT_DIR/tools/smokes/v2/profiles/quick/collections/string_size_alias.sh"
GOLDEN_MACRO_DIR="$ROOT_DIR/tools/test/golden/macro"
GOLDEN_MACRO_RESOLVER="$ROOT_DIR/tools/test/golden/macro/lib/resolve_hakorune.sh"
ENV_RS="$ROOT_DIR/src/config/env.rs"
ENV_PATHS_RS="$ROOT_DIR/src/config/env/paths.rs"
ENV_DOC="$ROOT_DIR/docs/reference/environment-variables.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" git
guard_require_files "$TAG" "$SSOT" "$CHECK_INDEX" "$QUICK_STEPS" "$DOCS_LAYOUT" "$CARGO_TOML" "$README_MD" "$HACO_WRAPPER" "$MAIN_RS" "$HAKORUNE_BIN_RS" "$HAKORUNE_COMPAT_BIN_RS" "$BUILD_SHARED_RS" "$BUILD_PRODUCT_RS" "$BUILD_ENGINEERING_RS" "$HAKO_CHECK_SH" "$BUILD_LLVM_PS" "$BUILD_AOT_PS" "$USING_UNRESOLVED_SMOKE" "$USING_RESOLVE_SMOKE" "$USING_STRICT_PATH_FAIL_SMOKE" "$DEV_SELFHOST_LOOP" "$ENGINEERING_PARITY" "$SELFHOST_EXE_STAGEB" "$HAKORUNE_EMIT_MIR" "$SELFHOST_STAGEB_PROOF_VM" "$SELFHOST_RUN_ROUTES" "$SELFHOST_BUILD" "$SELFHOST_MAINLINE_BUILD_STAGE1" "$NY_PARSER_BRIDGE_SMOKE" "$PHI_TRACE_RUN" "$TEST_SHLIB" "$EMIT_MIR_ROUTE" "$HAKO_MIN_BINOP_SMOKE" "$HAKO_MIN_IF_SMOKE" "$HAKO_MIN_INDEX_SMOKE" "$HAKO_MIN_COMPILE_RETURN_SMOKE" "$HAKO_MAP_ESCAPE_SMOKE" "$STAGEB_HELPERS" "$GATE_C_V1_FILE_SMOKE" "$NYVM_WRAPPER_SMOKE" "$FASTMEM_PARSER_PARITY_SMOKE" "$PARSER_OPT_ANNOTATIONS_SMOKE" "$PARSER_TRY_COMPAT_SMOKE" "$PARSER_MIN_METHODS_SMOKE" "$PARSER_RUNE_DECL_TRACE_SMOKE" "$GATE_C_OOB_STRICT_SMOKE" "$NY_MIR_BUILDER" "$PHASE29X_L1_CACHE" "$PHASE29X_L2_CACHE" "$SMOKE_PREFLIGHT" "$SMOKE_PLUGIN_MANAGER" "$SMOKE_TEST_RUNNER" "$SMOKE_AUTO_DETECT_CONF" "$FASTMEM_SOURCE_SYNTAX_SMOKE" "$FASTMEM_TERMINAL_LADDER_SMOKE" "$FASTMEM_SOURCE_MANIFEST_RUNNER" "$SELFHOST_JSON_V0_TRY_CATCH_CANARY" "$SELFHOST_STAGEB_ROUTE_PARITY_SMOKE" "$COLLECTION_MAP_GET_SHARES_MAP_SMOKE" "$COLLECTION_MAP_GET_SHARES_ARRAY_SMOKE" "$COLLECTION_STRING_SIZE_ALIAS_SMOKE" "$GOLDEN_MACRO_RESOLVER" "$ENV_RS" "$ENV_PATHS_RS" "$ENV_DOC"

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
require_fixed "NAMING-CHARTER-STAGE-TERM-DISAMBIGUATION-001" "$SSOT"
require_fixed "NYASH-TO-HAKORUNE-RENAME-ROADMAP-001" "$SSOT"
require_fixed "HAKORUNE-USER-FACING-DOCS-CANONICALIZATION-001" "$SSOT"
require_fixed "HAKORUNE-BINARY-PRIMARY-CUTOVER-INVENTORY-001" "$SSOT"
require_fixed "HAKORUNE-BINARY-DEFAULT-RUN-CUTOVER-001" "$SSOT"
require_fixed "HAKORUNE-RUNNER-BUILD-HELPER-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-WINDOWS-BUILD-SCRIPT-CUTOVER-INVENTORY-001" "$SSOT"
require_fixed "HAKORUNE-HAKO-CHECK-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-ROOT-POWERSHELL-BUILD-SCRIPT-CUTOVER-001" "$SSOT"
require_fixed "HAKORUNE-DEV-SELFHOST-SMOKE-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-ENGINEERING-PARITY-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-SELFHOST-EXE-STAGEB-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-CORE-EMIT-HELPER-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-SELFHOST-ROUTE-BINARY-DIAGNOSTICS-001" "$SSOT"
require_fixed "HAKORUNE-SELFHOST-MAINLINE-STAGE1-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-PARSER-BRIDGE-SMOKE-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-PHI-TRACE-RUNNER-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-TEST-SHLIB-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-SMOKE-EMIT-MIR-ROUTE-BINARY-ALIAS-001" "$SSOT"
require_fixed "HAKORUNE-BRIDGE-CANONICALIZE-STABLE-SMOKE-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-WRAPPER-EXECUTABLE-BIT-001" "$SSOT"
require_fixed "HAKORUNE-MIN-OPTIN-SMOKE-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-QUICK-SMOKE-MODE-A-DIAGNOSTIC-001" "$SSOT"
require_fixed "HAKORUNE-QUICK-SMOKE-MODE-B-DIAGNOSTIC-001" "$SSOT"
require_fixed "HAKORUNE-MAP-ESCAPE-OPTIN-SMOKE-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-GATE-C-NYVM-WRAPPER-SMOKE-BINARY-NAMING-001" "$SSOT"
require_fixed "HAKORUNE-PARSER-INTEGRATION-SMOKE-BINARY-NAMING-001" "$SSOT"
require_fixed "HAKORUNE-PARSER-TRY-COMPAT-SMOKE-BINARY-NAMING-001" "$SSOT"
require_fixed "HAKORUNE-PARSER-INTEGRATION-EXTRA-SMOKE-BINARY-NAMING-001" "$SSOT"
require_fixed "HAKORUNE-GOLDEN-MACRO-BINARY-RESOLVER-001" "$SSOT"
require_fixed "HAKORUNE-CURRENT-DIAGNOSTIC-BINARY-WORDING-001" "$SSOT"
require_fixed "HAKORUNE-SMOKE-TEST-RUNNER-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-SMOKE-AUTO-DETECT-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-GATE-C-OOB-STRICT-SMOKE-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-NY-MIR-BUILDER-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-PHASE29X-CACHE-HELPER-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-SMOKE-SHARED-PREFLIGHT-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-SMOKE-PREFLIGHT-STAGE-TERM-DIAGNOSTIC-001" "$SSOT"
require_fixed "HAKORUNE-HAKO-CHECK-WRAPPER-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-FASTMEM-HAKO-CHECK-SMOKE-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-COLLECTION-QUICK-SMOKE-BINARY-WORDING-001" "$SSOT"
require_fixed "HAKORUNE-ENV-ALIAS-INVENTORY-001" "$SSOT"
require_fixed "HAKORUNE-ENV-ALIAS-FIRST-CUT-001" "$SSOT"
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
require_fixed 'BIN_HAKORUNE="$ROOT_DIR/target/release/hakorune"' "$HACO_WRAPPER"
require_fixed 'BIN_NYASH="$ROOT_DIR/target/release/nyash"' "$HACO_WRAPPER"
require_fixed 'if [[ -x "$BIN_HAKORUNE" ]]; then' "$HACO_WRAPPER"
if [[ ! -x "$HACO_WRAPPER" ]]; then
  guard_fail "$TAG" "tools/bin/hako must be executable"
fi
require_fixed 'include!("../main.rs");' "$HAKORUNE_BIN_RS"
require_fixed 'include!("../main.rs");' "$HAKORUNE_COMPAT_BIN_RS"
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
if rg -n "resolve_nyash_bin|nyash/hakorune binary not found" "$SELFHOST_EXE_STAGEB"; then
  guard_fail "$TAG" "selfhost EXE Stage-B helper must use Hakorune-first resolver naming"
fi
for selfhost_route_script in "$SELFHOST_STAGEB_PROOF_VM" "$SELFHOST_RUN_ROUTES" "$SELFHOST_BUILD"; do
  require_fixed 'Hakorune' "$selfhost_route_script"
  if rg -n 'nyash binary not found|no binary found under target/release|no bootstrap binary found under target/release' "$selfhost_route_script"; then
    guard_fail "$TAG" "selfhost route diagnostics must name Hakorune while keeping NYASH_BIN as compatibility override"
  fi
done
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
require_fixed "env_string_trimmed_with_alias(\"HAKO_ROOT\", \"NYASH_ROOT\")" "$ENV_PATHS_RS"
require_fixed "env_string_trimmed_with_alias(\"HAKO_BIN\", \"NYASH_BIN\")" "$ENV_PATHS_RS"
require_fixed "HAKORUNE_*" "$ENV_DOC"
require_fixed "HAKO_ROOT" "$ENV_DOC"
require_fixed "HAKO_BIN" "$ENV_DOC"
require_fixed "tools/checks/naming_charter_guard.sh" "$CHECK_INDEX"
require_fixed "naming_charter_guard.sh" "$QUICK_STEPS"
require_fixed "hakorune-naming-and-rename-task-order-ssot.md" "$DOCS_LAYOUT"

is_allowed_path() {
  case "$1" in
    docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md | \
    docs/development/current/main/DOCS_LAYOUT.md | \
    docs/tools/check-scripts-index.md | \
    tools/checks/naming_charter_guard.sh | \
    tools/checks/lib/dev_gate_quick_steps.sh | \
    CURRENT_TASK.md)
      return 0
      ;;
    *)
      return 1
      ;;
  esac
}

check_added_stage_terms_in_diff() {
  local mode="$1"
  local tmp
  tmp="$(mktemp "/tmp/${TAG}.${mode}.diff.XXXXXX")"
  if [[ "$mode" == "cached" ]]; then
    git -C "$ROOT_DIR" diff --cached --unified=0 -- >"$tmp"
  else
    git -C "$ROOT_DIR" diff --unified=0 -- >"$tmp"
  fi

  if awk '
    /^\+\+\+ b\// {
      file = substr($0, 7)
      allowed = 0
      if (file == "docs/development/current/main/design/hakorune-naming-and-rename-task-order-ssot.md") { allowed = 1 }
      if (file == "docs/development/current/main/DOCS_LAYOUT.md") { allowed = 1 }
      if (file == "docs/tools/check-scripts-index.md") { allowed = 1 }
      if (file == "tools/checks/naming_charter_guard.sh") { allowed = 1 }
      if (file == "tools/checks/lib/dev_gate_quick_steps.sh") { allowed = 1 }
      if (file == "CURRENT_TASK.md") { allowed = 1 }
      next
    }
    /^\+\+\+ / { next }
    /^\+/ && !allowed && /(^|[^A-Za-z0-9_])(Stage-[A-Za-z0-9_-]+|Stage[0-9]+|stage[0-9]+|stage-[A-Za-z0-9_-]+)/ {
      print
      found = 1
    }
    END { exit found ? 0 : 1 }
  ' "$tmp"; then
    rm -f "$tmp"
    guard_fail "$TAG" "new unqualified stage term added outside naming charter in ${mode} diff; classify it by layer first"
  fi
  rm -f "$tmp"
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
