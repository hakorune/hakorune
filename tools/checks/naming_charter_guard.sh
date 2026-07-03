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
NY_PARSER_BRIDGE_SMOKE="$ROOT_DIR/tools/ny_parser_bridge_smoke.sh"
PHI_TRACE_RUN="$ROOT_DIR/tools/debug/phi/phi_trace_run.sh"
ENV_RS="$ROOT_DIR/src/config/env.rs"
ENV_PATHS_RS="$ROOT_DIR/src/config/env/paths.rs"
ENV_DOC="$ROOT_DIR/docs/reference/environment-variables.md"

guard_require_command "$TAG" rg
guard_require_command "$TAG" git
guard_require_files "$TAG" "$SSOT" "$CHECK_INDEX" "$QUICK_STEPS" "$DOCS_LAYOUT" "$CARGO_TOML" "$README_MD" "$HACO_WRAPPER" "$MAIN_RS" "$HAKORUNE_BIN_RS" "$HAKORUNE_COMPAT_BIN_RS" "$BUILD_SHARED_RS" "$BUILD_PRODUCT_RS" "$BUILD_ENGINEERING_RS" "$HAKO_CHECK_SH" "$BUILD_LLVM_PS" "$BUILD_AOT_PS" "$USING_UNRESOLVED_SMOKE" "$USING_RESOLVE_SMOKE" "$USING_STRICT_PATH_FAIL_SMOKE" "$DEV_SELFHOST_LOOP" "$ENGINEERING_PARITY" "$SELFHOST_EXE_STAGEB" "$NY_PARSER_BRIDGE_SMOKE" "$PHI_TRACE_RUN" "$ENV_RS" "$ENV_PATHS_RS" "$ENV_DOC"

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
require_fixed "HAKORUNE-PARSER-BRIDGE-SMOKE-BINARY-RESOLUTION-001" "$SSOT"
require_fixed "HAKORUNE-PHI-TRACE-RUNNER-BINARY-RESOLUTION-001" "$SSOT"
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
require_fixed 'BIN="$ROOT_DIR/target/release/hakorune"' "$HAKO_CHECK_SH"
require_fixed 'BIN="$ROOT_DIR/target/release/nyash"' "$HAKO_CHECK_SH"
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
require_fixed 'NYASH_BIN="$ROOT_DIR/target/release/hakorune"' "$SELFHOST_EXE_STAGEB"
require_fixed 'NYASH_BIN="$ROOT_DIR/target/release/nyash"' "$SELFHOST_EXE_STAGEB"
if rg -n "resolve_nyash_bin|nyash/hakorune binary not found" "$SELFHOST_EXE_STAGEB"; then
  guard_fail "$TAG" "selfhost EXE Stage-B helper must use Hakorune-first resolver naming"
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
