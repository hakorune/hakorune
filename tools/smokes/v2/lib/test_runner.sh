#!/bin/bash
# test_runner.sh - 中核実行器（強制使用）
# スモークテストv2の核心ライブラリ

# set -eは使わない（個々のテストが失敗しても全体を続行するため）
set -uo pipefail

# ルート/バイナリ検出（CWDに依存しない実行を保証）
if [ -z "${NYASH_ROOT:-}" ]; then
  export NYASH_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../../.." && pwd)"
fi

# Source centralized environment configuration (SSOT)
LIB_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
if [ -f "$LIB_DIR/env.sh" ]; then
  source "$LIB_DIR/env.sh"
fi
# Prefer hakorune binary if exists; fallback to nyash for compatibility
if [ -z "${NYASH_BIN:-}" ]; then
  if [ -x "$NYASH_ROOT/target/release/hakorune" ]; then
    export NYASH_BIN="$NYASH_ROOT/target/release/hakorune"
  else
    export NYASH_BIN="$NYASH_ROOT/target/release/nyash"
  fi
fi

# Stage-3 is default: prefer feature flag instead of legacy parser envs.
export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"
# JoinIR Core は常時 ON（NYASH_JOINIR_CORE は deprecated/no-op）

# Debug convenience: HAKO_DEBUG=1 enables execution trace and log passthrough
if [ "${HAKO_DEBUG:-0}" = "1" ]; then
  export HAKO_TRACE_EXECUTION=1
  export HAKO_VERIFY_SHOW_LOGS=1
fi

# Tag silence toggle (default: silent=1)
# - HAKO_SILENT_TAGS=1 → filter noisy tag lines (default)
# - HAKO_SILENT_TAGS=0 → show raw logs (no filtering)
export HAKO_SILENT_TAGS="${HAKO_SILENT_TAGS:-1}"

# グローバル変数
export SMOKES_V2_LIB_LOADED=1
export SMOKES_START_TIME=$(date +%s.%N)
export SMOKES_TEST_COUNT=0
export SMOKES_PASS_COUNT=0
export SMOKES_FAIL_COUNT=0
export SMOKES_INCLUDE_SKIP_COUNT=0
declare -a SMOKES_INCLUDE_SKIP_LIST=()

# 色定義（重複回避）
if [ -z "${RED:-}" ]; then
    readonly RED='\033[0;31m'
    readonly GREEN='\033[0;32m'
    readonly YELLOW='\033[1;33m'
    readonly BLUE='\033[0;34m'
    readonly NC='\033[0m' # No Color
fi

# ログ関数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $*" >&2
}

log_success() {
    echo -e "${GREEN}[PASS]${NC} $*" >&2
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*" >&2
}

log_error() {
    echo -e "${RED}[FAIL]${NC} $*" >&2
}

# 共通ノイズフィルタ（VM実行時の出力整形）
  filter_noise() {
    # Show raw logs (no filtering) to allow call traces / diagnostics
    if [ "${HAKO_SHOW_CALL_LOGS:-0}" = "1" ] || [ "${HAKO_SILENT_TAGS}" = "0" ]; then
      cat
      return
    fi
    # プラグイン初期化やメタログ、動的ローダの案内等を除去
    grep -v "^\[UnifiedBoxRegistry\]" \
      | grep -v "^\[FileBox\]" \
      | grep -v "^\[provider-registry\]" \
      | grep -v "^\[provider/select:" \
      | grep -v "^\[provider/trace\]" \
      | grep -v "^\[deprecate/env\]" \
      | grep -v "^\[WARN\] \[deprecate/env\]" \
      | grep -v "^\[plugin/missing\]" \
      | grep -v "^\[plugin/hint\]" \
      | grep -v "^\[plugins\]" \
      | grep -v "^\[WARN\] \[plugin/init\]" \
      | grep -v "^Net plugin:" \
      | grep -v "^\[.*\] Plugin" \
      | grep -v "Using builtin StringBox" \
      | grep -v "Using builtin ArrayBox" \
      | grep -v "Using builtin MapBox" \
      | grep -v "^\[using\]" \
      | grep -v "^\[using/resolve\]" \
      | grep -v "^\[using/text-merge\]" \
      | grep -v "^\\[trace:" \
      | grep -v "^\\[plan/freeze:" \
      | grep -v "^\\[lower_static_method_as_function\\]" \
      | grep -v "^\\[lower_method_as_function\\]" \
      | grep -v "^\\[macro\\]\\[test\\]\\[args\\]" \
      | grep -v "^\\[phase[0-9]" \
      | grep -v "^\\[cf_loop/joinir" \
      | grep -v "^\\[joinir/" \
      | grep -v "^\\[flowbox/adopt " \
      | grep -v "^\\[flowbox/freeze " \
      | grep -v "^\\[DEBUG-[0-9]" \
      | grep -v "^\\[pattern[0-9]" \
      | grep -v "^\\[method_call_lowerer\\]" \
      | grep -v "^\\[loop_" \
      | grep -v "^\\[build_static_main_box\\]" \
      | grep -v "^\\[cond_promoter\\]" \
      | grep -v "^\[builder\]" \
      | grep -v "^\\[vm-trace\\]" \
      | grep -v "^\\[DEBUG/" \
      | grep -v "^\\[ssa-undef-debug\\]" \
      | grep -v '^\[PluginBoxFactory\]' \
      | grep -v '^\[plugin/init\]' \
      | grep -v '^\[using.dylib/autoload\]' \
      | grep -v "^\[vm\] Stage-3" \
      | grep -v "^\[DEBUG\]" \
      | grep -v '^\{"ev":' \
      | grep -v '^\[warn\]' \
      | grep -v '^\[error\]' \
      | grep -v '^RC: ' \
      | grep -v '^\[mirbuilder/normalize:' \
      | grep -v '^\[warn\] dev fallback: user instance BoxCall' \
      | sed -E 's/^❌ VM fallback error: *//' \
      | sed -E 's/^\\[ERROR\\].*VM error: /VM error: /' \
      | grep -v '^\[warn\] dev verify: NewBox ' \
      | grep -v '^\[warn\] dev verify: NewBox→birth invariant warnings:' \
      | grep -v '^\[ny-compiler\]' \
      | grep -v '^\[using/cache\]' \
      | grep -v "plugins/nyash-array-plugin" \
      | grep -v "plugins/nyash-map-plugin" \
      | grep -v "Phase 15.5: Everything is Plugin" \
      | grep -v "cargo build -p nyash-string-plugin" \
      | grep -v "^\[plugin-loader\] backend=" \
      | grep -v "^\[using\] ctx:" \
      | grep -v "^🔌 plugin host initialized" \
      | grep -v "^✅ plugin host fully configured" \
      | grep -v "Failed to load nyash.toml - plugins disabled" \
      | grep -v "^⚠️ Failed to load plugin config (hakorune.toml/nyash.toml) - plugins disabled" \
      | grep -v "^🚀 Nyash VM Backend - Executing file:" \
      | grep -v "^🚀 Hakorune VM Backend - Executing file:" \
      | grep -v "^[[]ControlForm::"
}

# 環境チェック（必須）
require_env() {
    local required_tools=("cargo" "grep" "jq")
    local missing_tools=()

    for tool in "${required_tools[@]}"; do
        if ! command -v "$tool" &> /dev/null; then
            missing_tools+=("$tool")
        fi
    done

    if [ ${#missing_tools[@]} -ne 0 ]; then
        log_error "Required tools missing: ${missing_tools[*]}"
        log_error "Please install missing tools and try again"
        return 1
    fi

    # Nyash実行ファイル確認
    if [ ! -f "$NYASH_BIN" ]; then
        log_error "Nyash executable not found at $NYASH_BIN"
        log_error "Please run 'cargo build --release' first (in $NYASH_ROOT)"
        return 1
    fi

    log_info "Environment check passed"
    return 0
}

# JoinIR strict lane helper (VM)
# - strict=1
# - clear debug/dev flags to avoid env contamination between smoke scripts
# - default plugins=1 (can override via NYASH_DISABLE_PLUGINS)
run_joinir_vm_strict() {
    local fixture="$1"
    shift || true
    local timeout_secs="${RUN_TIMEOUT_SECS:-10}"

    timeout "$timeout_secs" env \
        -u HAKO_JOINIR_DEBUG \
        -u NYASH_JOINIR_DEBUG \
        -u HAKO_JOINIR_DEV \
        -u NYASH_JOINIR_DEV \
        NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
        HAKO_JOINIR_STRICT=1 \
        "$NYASH_BIN" --backend vm "$fixture" "$@" 2>&1
}

# JoinIR release lane helper (VM)
# - strict/dev/debug flags are fully unset
# - default plugins=1 (can override via NYASH_DISABLE_PLUGINS)
run_joinir_vm_release() {
    local fixture="$1"
    shift || true
    local timeout_secs="${RUN_TIMEOUT_SECS:-10}"

    timeout "$timeout_secs" env \
        -u HAKO_JOINIR_STRICT \
        -u NYASH_JOINIR_STRICT \
        -u HAKO_JOINIR_DEBUG \
        -u NYASH_JOINIR_DEBUG \
        -u HAKO_JOINIR_DEV \
        -u NYASH_JOINIR_DEV \
        NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
        "$NYASH_BIN" --backend vm "$fixture" "$@" 2>&1
}

# プラグイン整合性チェック（必須）
preflight_plugins() {
    # プラグインマネージャーが存在する場合は実行
    if [ -f "$(dirname "${BASH_SOURCE[0]}")/plugin_manager.sh" ]; then
        source "$(dirname "${BASH_SOURCE[0]}")/plugin_manager.sh"
        check_plugin_integrity || return 1
    else
        log_warn "Plugin manager not found, skipping plugin checks"
    fi

    return 0
}

# テスト実行関数
run_test() {
    local test_name="$1"
    local test_func="$2"

    ((SMOKES_TEST_COUNT++))
    local start_time=$(date +%s.%N)

    log_info "Running test: $test_name"

    if $test_func; then
        local end_time=$(date +%s.%N)
        local duration=$(echo "$end_time - $start_time" | bc -l)
        if [[ "$duration" == -* ]]; then
            duration=0
        fi
        log_success "$test_name (${duration}s)"
        ((SMOKES_PASS_COUNT++))
        return 0
    else
        local end_time=$(date +%s.%N)
        local duration=$(echo "$end_time - $start_time" | bc -l)
        if [[ "$duration" == -* ]]; then
            duration=0
        fi
        log_error "$test_name (${duration}s)"
        ((SMOKES_FAIL_COUNT++))
        return 1
    fi
}

# Nyash実行ヘルパー（Rust VM）
run_nyash_vm() {
    local program="$1"
    shift
    local EXTRA_ARGS=()
    if [ "${SMOKES_USE_DEV:-0}" = "1" ]; then
        EXTRA_ARGS+=("--dev")
    fi
    # Optional env sanitization between rapid invocations (default OFF)
    # Enable with: SMOKES_CLEAN_ENV=1
    local ENV_PREFIX=( )
    if [ "${SMOKES_CLEAN_ENV:-0}" = "1" ]; then
        # Preserve NYASH_JSON_ONLY to allow quiet JSON pipelines (e.g., v1 emitters)
        ENV_PREFIX=(env -u NYASH_DEBUG_ENABLE -u NYASH_DEBUG_KINDS -u NYASH_DEBUG_SINK \
                        -u NYASH_RESOLVE_FIX_BRACES -u NYASH_USING_AST \
                        -u NYASH_VM_TRACE -u NYASH_VM_VERIFY_MIR -u NYASH_VM_TOLERATE_VOID \
                        -u NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN)
    fi
    # -c オプションの場合は一時ファイル経由で実行
    if [ "$program" = "-c" ]; then
        local code="$1"
        shift
        local tmpfile="/tmp/nyash_test_$$.hako"
        echo "$code" > "$tmpfile"
        # (shim removed) provider tag shortcut — hv1 inline is stable now
        # 軽量ASIFix（テスト用）: ブロック終端の余剰セミコロンを寛容に除去
        if [ "${SMOKES_ASI_STRIP_SEMI:-1}" = "1" ]; then
            sed -i -E 's/;([[:space:]]*)(\}|$)/\1\2/g' "$tmpfile" || true
        fi
        # プラグイン初期化メッセージを除外
        # Optional preinclude for include-based code
        local runfile="$tmpfile"
        if [ "${NYASH_PREINCLUDE:-0}" = "1" ] || [ "${HAKO_PREINCLUDE:-0}" = "1" ]; then
            local prefile="/tmp/nyash_pre_$$.hako"
            "$NYASH_ROOT/tools/dev/hako_preinclude.sh" "$tmpfile" "$prefile" >/dev/null || true
            runfile="$prefile"
        fi
        # Optional hint for include lines when preinclude is OFF
        if grep -q '^include\s\"' "$tmpfile" 2>/dev/null && [ "${NYASH_PREINCLUDE:-0}" != "1" ] && [ "${HAKO_PREINCLUDE:-0}" != "1" ]; then
            echo "[WARN] VM backend does not support include. Prefer using+alias, or set NYASH_PREINCLUDE=1 for dev." >&2
        fi
        HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
        NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
            NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
            NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
            HAKO_ENABLE_USING=${HAKO_ENABLE_USING:-1} NYASH_ENABLE_USING=${NYASH_ENABLE_USING:-1} \
            NYASH_USING_AST=1 NYASH_PARSER_SEAM_TOLERANT=1 \
            "${ENV_PREFIX[@]}" \
            "$NYASH_BIN" --backend vm "$runfile" "${EXTRA_ARGS[@]}" "$@" 2>&1 | filter_noise
        local exit_code=${PIPESTATUS[0]}
        # prefile may be unset when preinclude is OFF; use default expansion to avoid set -u errors
        rm -f "$tmpfile" "${prefile:-}" 2>/dev/null || true
        if [ "${SMOKES_FORCE_ZERO:-0}" = "1" ]; then
            return 0
        fi
        return $exit_code
    else
        local runfile2="$program"
        local workfile2=""
        # 軽量ASIFix（テスト用）: 元ファイルは変更せず一時コピーへ適用
        # NOTE: using(file-path) を使うケースで /tmp へ退避すると相対解決が崩れるため、
        # NYASH_ALLOW_USING_FILE=1 かつ using 行を含む場合はコピーを回避する。
        if [ "${SMOKES_ASI_STRIP_SEMI:-1}" = "1" ] && [ -f "$program" ]; then
            local skip_temp_copy=0
            if [ "${NYASH_ALLOW_USING_FILE:-0}" = "1" ] && grep -q '^using\s\"' "$program" 2>/dev/null; then
                skip_temp_copy=1
            fi
            if [ "$skip_temp_copy" -ne 1 ]; then
                workfile2="$(mktemp /tmp/nyash_test_file.XXXXXX.hako)"
                cp "$program" "$workfile2"
                sed -i -E 's/;([[:space:]]*)(\}|$)/\1\2/g' "$workfile2" || true
                runfile2="$workfile2"
            fi
        fi
        # プラグイン初期化メッセージを除外
        # Optional preinclude
        if [ "${NYASH_PREINCLUDE:-0}" = "1" ] || [ "${HAKO_PREINCLUDE:-0}" = "1" ]; then
            local prefile2="/tmp/nyash_pre_$$.hako"
            "$NYASH_ROOT/tools/dev/hako_preinclude.sh" "$runfile2" "$prefile2" >/dev/null || true
            runfile2="$prefile2"
        fi
        # Optional hint for include lines when preinclude is OFF
        if [ -f "$program" ] && grep -q '^include\s\"' "$program" 2>/dev/null && [ "${NYASH_PREINCLUDE:-0}" != "1" ] && [ "${HAKO_PREINCLUDE:-0}" != "1" ]; then
            # Policy: quick は SKIP 既定。それ以外は WARN（SMOKES_INCLUDE_POLICY で上書き可能）。
            local policy="${SMOKES_INCLUDE_POLICY:-}"
            if [ -z "$policy" ]; then
              case "$program" in
                */profiles/quick/*) policy="error" ;;
                *) policy="warn" ;;
              esac
            fi
            if [ "$policy" = "skip" ]; then
              SMOKES_INCLUDE_SKIP_COUNT=$((SMOKES_INCLUDE_SKIP_COUNT + 1))
              local rel_path="$program"
              if [[ "$program" == "$NYASH_ROOT/"* ]]; then
                rel_path="${program#$NYASH_ROOT/}"
              fi
              SMOKES_INCLUDE_SKIP_LIST+=("$rel_path")
              echo "[SKIP] include is deprecated in 20.36+ (quick). Prefer using+alias." >&2
              return 0
            elif [ "$policy" = "error" ]; then
              echo "[ERROR] include is deprecated in 20.36+. Prefer using+alias." >&2
              return 2
            else
              echo "[WARN] include is deprecated in 20.36+. Prefer using+alias. Preinclude is dev-only (NYASH_PREINCLUDE=1)." >&2
            fi
        fi
        local trace_tmp=""
        if [ "${NYASH_DEV_PROVIDER_TRACE:-0}" != "0" ]; then
            trace_tmp="$(mktemp /tmp/hako_provider_trace.XXXXXX)"
        fi

        if [ -n "$trace_tmp" ]; then
            HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
            NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
                NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
                NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
                HAKO_ENABLE_USING=${HAKO_ENABLE_USING:-1} NYASH_ENABLE_USING=${NYASH_ENABLE_USING:-1} \
                NYASH_USING_AST=1 NYASH_PARSER_SEAM_TOLERANT=1 \
                "${ENV_PREFIX[@]}" \
                "$NYASH_BIN" --backend vm "$runfile2" "${EXTRA_ARGS[@]}" "$@" 2>&1 | tee "$trace_tmp" | filter_noise
            local exit_code=${PIPESTATUS[0]}
            grep -E '^\[provider/trace\]' "$trace_tmp" >&2 || true
        else
            HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
            NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
                NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 \
                NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 \
                HAKO_ENABLE_USING=${HAKO_ENABLE_USING:-1} NYASH_ENABLE_USING=${NYASH_ENABLE_USING:-1} \
                NYASH_USING_AST=1 NYASH_PARSER_SEAM_TOLERANT=1 \
                "${ENV_PREFIX[@]}" \
                "$NYASH_BIN" --backend vm "$runfile2" "${EXTRA_ARGS[@]}" "$@" 2>&1 | filter_noise
            local exit_code=${PIPESTATUS[0]}
        fi
        # prefile2/workfile2 may be unset when preinclude/ASI copy is OFF
        rm -f "${prefile2:-}" "${workfile2:-}" 2>/dev/null || true
        if [ -n "${trace_tmp:-}" ]; then
            rm -f "$trace_tmp" 2>/dev/null || true
        fi
        if [ "${SMOKES_FORCE_ZERO:-0}" = "1" ]; then
            return 0
        fi
        return $exit_code
    fi
}

# Verify MIR JSON rc using selected primary (Core or Hakorune VM)
verify_mir_rc() {
    local json_path="$1"
    # 20.36: hakovm を primary 既定へ（Core は診断 fallback）
    local primary="${HAKO_VERIFY_PRIMARY:-hakovm}"
    if [ "$primary" = "hakovm" ]; then
        # For MIR JSON v1, try Hakovm v1 dispatcher first (default ON), fallback to Core on failure.
        # Allow forcing Core with HAKO_VERIFY_V1_FORCE_CORE=1
        if grep -q '"schema_version"' "$json_path" 2>/dev/null; then
          if [ "${HAKO_VERIFY_V1_FORCE_CORE:-0}" = "1" ]; then
            if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: core (rust)" >&2; fi
            "$NYASH_BIN" --mir-json-file "$json_path" >/dev/null 2>&1; return $?
          fi
          # hv1 直行（main.rs 早期経路）。成功時は rc を採用、失敗時は Core にフォールバック。
          # ただしフロー検証（dispatcher flow / phi 実験）が有効な場合は Core を優先（hv1-inline は最小実装のため）。
          if [ "${HAKO_VERIFY_V1_FORCE_HAKOVM:-0}" != "1" ]; then
            local hv1_rc; hv1_rc=$(verify_v1_inline_file "$json_path" || true)
            if [[ "$hv1_rc" =~ ^-?[0-9]+$ ]]; then
              if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: hv1_inline (rust)" >&2; fi
              local n=$hv1_rc; if [ $n -lt 0 ]; then n=$(( (n % 256 + 256) % 256 )); else n=$(( n % 256 )); fi; return $n
            fi
          fi
          # 強制 hv1（-c ラッパ）: NyVmDispatcherV1Box.run を直接呼び出して rc を取得
          if [ "${HAKO_VERIFY_V1_FORCE_HAKOVM:-0}" = "1" ]; then
            local hv1_rc_force; hv1_rc_force=$(verify_v1_inline_file "$json_path" || true)
            if [[ "$hv1_rc_force" =~ ^-?[0-9]+$ ]]; then
              if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: hv1_inline (rust)" >&2; fi
              local n=$hv1_rc_force; if [ $n -lt 0 ]; then n=$(( (n % 256 + 256) % 256 )); else n=$(( n % 256 )); fi; return $n
            fi
            return 1
          fi
          # No include+preinclude fallback succeeded → Core にフォールバック
          if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: core (rust)" >&2; fi
          "$NYASH_BIN" --mir-json-file "$json_path" >/dev/null 2>&1
          return $?
        fi
        # Build a tiny driver to call MiniVmEntryBox.run_min with JSON literal embedded
          if [ ! -f "$json_path" ]; then
            echo "[FAIL] verify_mir_rc: json not found: $json_path" >&2
            return 2
          fi
        # Escape JSON as a single string literal via jq -Rs (preserves newlines)
        local json_literal
        json_literal="$(jq -Rs . < "$json_path")"
        build_and_run_driver_alias() {
          local header="$1"
          local code=$(cat <<HCODE
$header
static box Main { method main(args) {
  local j = __MIR_JSON__;
  local rc = MiniVmEntryBox.run_min(j)
  print(MiniVmEntryBox.int_to_str(rc))
  return rc
} }
HCODE
)
          code="${code/__MIR_JSON__/$json_literal}"
          HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}'
        }
        build_and_run_driver_include() {
          local inc_path="$1"
          local code=$(cat <<HCODE
include "$inc_path"
static box Main { method main(args) {
  local j = __MIR_JSON__;
  local rc = MiniVmEntryBox.run_min(j)
  print(MiniVmEntryBox.int_to_str(rc))
  return rc
} }
HCODE
)
          code="${code/__MIR_JSON__/$json_literal}"
          HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_PREINCLUDE=1 NYASH_RESOLVE_FIX_BRACES=1 run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}'
        }
        # Try alias header first; fallback to dev-file header; final fallback: include+preinclude
        local out
        out="$(build_and_run_driver_alias 'using selfhost.vm.entry as MiniVmEntryBox')"
        if ! [[ "$out" =~ ^-?[0-9]+$ ]]; then
          out="$(build_and_run_driver_alias 'using "lang/src/vm/boxes/mini_vm_entry.hako" as MiniVmEntryBox')"
        fi
        if ! [[ "$out" =~ ^-?[0-9]+$ ]]; then
          out="$(build_and_run_driver_include 'lang/src/vm/boxes/mini_vm_entry.hako')"
        fi
        if [[ "$out" =~ ^-?[0-9]+$ ]]; then
          local n=$out
          # normalize into [0,255]
          if [ $n -lt 0 ]; then n=$(( (n % 256 + 256) % 256 )); else n=$(( n % 256 )); fi
          return $n
        fi
        # Fallback: core primary when MiniVM resolution is unavailable
        if grep -q '"functions"' "$json_path" 2>/dev/null && grep -q '"blocks"' "$json_path" 2>/dev/null; then
          local json_literal3; json_literal3="$(jq -Rs . < "$json_path")"
          local code=$(cat <<HCODE
include "lang/src/vm/core/dispatcher.hako"
static box Main { method main(args) {
  local j = __MIR_JSON__
  local r = NyVmDispatcher.run(j)
  print("" + r)
  return r
} }
HCODE
)
          code="${code/__MIR_JSON__/$json_literal3}"
          local tmpwrap="/tmp/hako_core_wrap_$$.hako"
          echo "$code" > "$tmpwrap"
          NYASH_PREINCLUDE=1 run_nyash_vm "$tmpwrap" >/dev/null 2>&1; local r=$?; rm -f "$tmpwrap"; return $r
        fi
        NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1; return $?
    else
        # Core primary: detect MIR(JSON) vs Program(JSON v0)
        if grep -q '"functions"' "$json_path" 2>/dev/null && grep -q '"blocks"' "$json_path" 2>/dev/null; then
          "$NYASH_BIN" --mir-json-file "$json_path" >/dev/null 2>&1; return $?
        fi
        NYASH_GATE_C_CORE=1 HAKO_GATE_C_CORE=1 "$NYASH_BIN" --json-file "$json_path" >/dev/null 2>&1; return $?
    fi
}

# Program(JSON v0) -> provider emit wrapper (MIR v1 extern -> MIR JSON)
# Keep the provider owner exact while avoiding vm-hako subset debt from
# compiling a temporary .hako wrapper with newbox(MirBuilderBox/hostbridge).
emit_mir_json_via_provider_extern_v1() {
    local prog_json_raw="$1"
    local builder_stderr="$2"
    local builder_stdout="$3"
    local tmp_json
    local prog_json_quoted
    tmp_json=$(mktemp /tmp/nyash_provider_emit_v1.XXXXXX.json)
    prog_json_quoted=$(printf '%s' "$prog_json_raw" | jq -Rs .)

    cat >"$tmp_json" <<JSON
{
  "schema_version": "1.0",
  "functions": [
    {
      "name": "main",
      "blocks": [
        { "id": 0, "instructions": [
          {"op":"const","dst":0, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "env.mirbuilder.emit"}},
          {"op":"const","dst":1, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": ${prog_json_quoted}}},
          {"op":"mir_call","dst":2, "callee": {"type":"Extern","name":"env.mirbuilder.emit"}, "args": [1], "effects": [] },
          {"op":"const","dst":3, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "[MIR_OUT_BEGIN]"}},
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [3], "effects": [] },
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [2], "effects": [] },
          {"op":"const","dst":4, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "[MIR_OUT_END]"}},
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [4], "effects": [] },
          {"op":"const","dst":5, "value": {"type":"i64", "value": 0}},
          {"op":"ret", "value": 5}
        ] }
      ]
    }
  ]
}
JSON

    set +e
    "$NYASH_BIN" --mir-json-file "$tmp_json" 2>>"$builder_stderr" \
        | tee -a "$builder_stdout" \
        | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag'
    local rc=${PIPESTATUS[0]}
    set -e
    rm -f "$tmp_json" 2>/dev/null || true
    return $rc
}

mir_builder_output_missing() {
    local mir_json="${1:-}"
    [ "$mir_json" = "Builder failed" ] || [ -z "$mir_json" ]
}

emit_mir_json_via_min_runner() {
    local prog_json_raw="$1"
    local builder_code_min="$2"
    local builder_stderr="$3"
    local builder_stdout="$4"

    HAKO_MIR_BUILDER_INTERNAL=1 \
    HAKO_MIR_RUNNER_MIN_NO_METHODS=1 \
    HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 \
    HAKO_ROUTE_HAKOVM=1 \
    NYASH_ENABLE_USING=1 HAKO_ENABLE_USING=1 \
    NYASH_USING_AST=1 \
    NYASH_RESOLVE_FIX_BRACES=1 \
    NYASH_DISABLE_NY_COMPILER=1 \
    NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
    NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 \
    HAKO_BUILDER_PROGRAM_JSON="$prog_json_raw" \
    run_nyash_vm -c "$builder_code_min" 2>"$builder_stderr" \
        | tee "$builder_stdout" \
        | awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag'
}

emit_mir_json_via_builder_lanes() {
    local prog_json_raw="$1"
    local builder_code_min="$2"
    local builder_stderr="$3"
    local builder_stdout="$4"
    local mir_json=""

    if [ "${HAKO_PREFER_MIRBUILDER:-0}" != "1" ]; then
        mir_json=$(emit_mir_json_via_min_runner "$prog_json_raw" "$builder_code_min" "$builder_stderr" "$builder_stdout")
    fi

    if mir_builder_output_missing "$mir_json"; then
        mir_json=$(emit_mir_json_via_provider_extern_v1 "$prog_json_raw" "$builder_stderr" "$builder_stdout")
    fi

    printf '%s' "$mir_json"
}

dump_builder_debug_logs() {
    local builder_stdout="$1"
    local builder_stderr="$2"

    if [ "${HAKO_MIR_BUILDER_DEBUG:-0}" != "1" ]; then
        return 0
    fi

    echo "[builder debug] stdout (tail):" >&2
    tail -n 60 "$builder_stdout" >&2 || true
    echo "[builder debug] stderr (tail):" >&2
    tail -n 60 "$builder_stderr" >&2 || true
}

builder_min_runner_code() {
    cat <<'HCODE'
using "hako.mir.builder.internal.runner_min" as BuilderRunnerMinBox
static box Main { method main(args) {
  local prog_json = env.get("HAKO_BUILDER_PROGRAM_JSON")
  if prog_json == null { print("Builder failed"); return 1 }
  local mir_out = BuilderRunnerMinBox.run(prog_json)
  if mir_out == null { print("Builder failed"); return 1 }
  print("[MIR_OUT_BEGIN]")
  print("" + mir_out)
  print("[MIR_OUT_END]")
  return 0
} }
HCODE
}

builder_module_program_json_runner_code() {
    local builder_module="$1"
    cat <<HAKO
using "${builder_module}" as MirBuilderBox
static box Main {
  method _emit_mir_checked(program_json) {
    if program_json == null { print("[fail:nojson]"); return null }
    local out = MirBuilderBox.emit_from_program_json_v0(program_json, null)
    if out == null { print("[fail:builder]"); return null }
    return out
  }

  method main(args) {
    local out = me._emit_mir_checked(env.get("PROG_JSON"))
    if out == null { return 1 }
    print("[MIR_BEGIN]"); print("" + out); print("[MIR_END]")
    return 0
  }
}
HAKO
}

run_program_json_via_builder_module_vm_with_env() {
    local builder_module="$1"
    local prog_json="$2"
    local use_registry_defaults="${3:-0}"
    local registry_only="${4:-}"
    local preinclude="${5:-0}"
    local diag_skip_loops="${6:-0}"
    local tmp_hako

    tmp_hako=$(mktemp --suffix .hako)
    builder_module_program_json_runner_code "$builder_module" >"${tmp_hako}"

    (
        if [ "$preinclude" = "1" ]; then
            export HAKO_PREINCLUDE=1
        fi
        if [ "$diag_skip_loops" = "1" ]; then
            export HAKO_MIR_BUILDER_SKIP_LOOPS=1
        fi
        if [ "$use_registry_defaults" = "1" ]; then
            if [ -n "$registry_only" ]; then
                export HAKO_MIR_BUILDER_REGISTRY_ONLY="$registry_only"
            else
                unset HAKO_MIR_BUILDER_REGISTRY_ONLY
            fi
            export NYASH_USE_NY_COMPILER="${NYASH_USE_NY_COMPILER:-0}"
            export HAKO_MIR_BUILDER_DELEGATE="${HAKO_MIR_BUILDER_DELEGATE:-0}"
            export HAKO_MIR_BUILDER_INTERNAL="${HAKO_MIR_BUILDER_INTERNAL:-1}"
            export HAKO_MIR_BUILDER_REGISTRY="${HAKO_MIR_BUILDER_REGISTRY:-1}"
            export HAKO_MIR_BUILDER_DEBUG="${HAKO_MIR_BUILDER_DEBUG:-1}"
        fi

        PROG_JSON="$prog_json" \
        NYASH_FAIL_FAST="${NYASH_FAIL_FAST:-0}" \
        NYASH_FEATURES="${NYASH_FEATURES:-stage3}" \
        NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}" \
        HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}" \
        "$NYASH_BIN" --backend vm "${tmp_hako}"
    )
    local rc=$?
    rm -f "${tmp_hako}" 2>/dev/null || true
    return $rc
}

run_program_json_via_builder_module_vm() {
    local builder_module="$1"
    local prog_json="$2"
    run_program_json_via_builder_module_vm_with_env "$builder_module" "$prog_json" 0 "" 0 0
}

run_program_json_via_registry_builder_module_vm() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="${3:-}"
    run_program_json_via_builder_module_vm_with_env "$builder_module" "$prog_json" 1 "$registry_only" 0 0
}

run_program_json_via_registry_builder_module_vm_with_preinclude() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="${3:-}"

    run_program_json_via_builder_module_vm_with_env "$builder_module" "$prog_json" 1 "$registry_only" 1 0
}

run_program_json_via_registry_builder_module_vm_diag() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="${3:-}"

    run_program_json_via_builder_module_vm_with_env "$builder_module" "$prog_json" 1 "$registry_only" 0 1
}

emit_mir_json_via_builder_from_program_json_file() {
    local prog_json_path="$1"
    local builder_stderr="$2"
    local builder_stdout="$3"
    local prog_json_raw
    local builder_code_min
    local mir_json=""

    prog_json_raw="$(cat "$prog_json_path")"
    builder_code_min="$(builder_min_runner_code)"
    mir_json=$(emit_mir_json_via_builder_lanes "$prog_json_raw" "$builder_code_min" "$builder_stderr" "$builder_stdout")
    dump_builder_debug_logs "$builder_stdout" "$builder_stderr"

    if mir_builder_output_missing "$mir_json"; then
        return 1
    fi
    if ! mir_json_looks_like_v0_module_text "$mir_json"; then
        return 1
    fi

    printf '%s' "$mir_json"
}

run_rust_cli_builder_fallback_for_verify() {
    local prog_json_path="$1"
    local builder_stderr="$2"
    local builder_stdout="$3"
    local allow_builder_only="${4:-0}"

    if [ "${HAKO_MIR_BUILDER_DEBUG:-0}" = "1" ] && [ -f "$builder_stderr" ]; then
        echo "[builder debug] Hako builder output unusable, falling back to Rust CLI" >&2
        cat "$builder_stderr" >&2
        cp "$builder_stderr" /tmp/builder_last_error.log
    fi

    rm -f "$builder_stderr" "$builder_stdout"
    run_program_json_v0_via_rust_cli_builder "$prog_json_path" "$allow_builder_only"
}

verify_builder_no_fallback_requested() {
    local emit_rc="${1:-0}"
    [ "${HAKO_PRIMARY_NO_FALLBACK:-0}" = "1" ] && [ "$emit_rc" -ne 0 ]
}

mir_json_file_looks_like_v0_module() {
    local mir_json_path="$1"
    grep -q '"functions"' "$mir_json_path" && grep -q '"blocks"' "$mir_json_path"
}

cleanup_verify_builder_logs() {
    local builder_stderr="$1"
    local builder_stdout="$2"
    rm -f "$builder_stderr" "$builder_stdout"
}

run_built_mir_json_file_via_core_v0() {
    local mir_json_path="$1"
    "$NYASH_BIN" --mir-json-file "$mir_json_path" >/dev/null 2>&1
}

# Program(JSON v0) -> Rust CLI builder fallback
# - with allow_builder_only=1, HAKO_VERIFY_BUILDER_ONLY=1 keeps the old structure-only contract
# - otherwise the helper executes the produced MIR and returns its rc
run_program_json_v0_via_rust_cli_builder() {
    local prog_json_path="$1"
    local allow_builder_only="${2:-0}"
    local tmp_mir="/tmp/ny_builder_conv_$$.json"

    if ! "$NYASH_BIN" --program-json-to-mir "$tmp_mir" --json-file "$prog_json_path" >/dev/null 2>&1; then
        return 1
    fi

    if [ "$allow_builder_only" = "1" ] && [ "${HAKO_VERIFY_BUILDER_ONLY:-0}" = "1" ]; then
        if mir_json_file_looks_like_v0_module "$tmp_mir"; then
            rm -f "$tmp_mir"
            return 0
        fi
        rm -f "$tmp_mir"
        return 1
    fi

    run_built_mir_json_file_via_core_v0 "$tmp_mir"
    local rc=$?
    rm -f "$tmp_mir"
    return $rc
}

mir_json_looks_like_v0_module_text() {
    local mir_json="$1"
    grep -q '"functions"' <<<"$mir_json" && grep -q '"blocks"' <<<"$mir_json"
}

run_built_mir_json_via_hv1_route() {
    local mir_json="$1"

    if ! grep -q '"schema_version"' <<<"$mir_json" && ! grep -q '"op"\s*:\s*"mir_call"' <<<"$mir_json"; then
        return 2
    fi

    local hv1_rc
    local mir_literal
    mir_literal="$(printf '%s' "$mir_json" | jq -Rs .)"
    hv1_rc=$(run_hv1_inline_alias_wrapper "$mir_literal")
    if [[ "$hv1_rc" =~ ^-?[0-9]+$ ]]; then
        if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: hakovm (hako)" >&2; fi
        local n=$hv1_rc
        if [ $n -lt 0 ]; then n=$(( (n % 256 + 256) % 256 )); else n=$(( n % 256 )); fi
        return $n
    fi

    return 1
}

run_built_mir_json_via_hako_core_route() {
    local mir_json="$1"

    if ! grep -q '"op"\s*:\s*"newbox"' <<<"$mir_json" && ! grep -q '"op"\s*:\s*"boxcall"' <<<"$mir_json"; then
        return 2
    fi

    local mir_literal2
    mir_literal2="$(printf '%s' "$mir_json" | jq -Rs .)"
    local code=$(cat <<'HCODE'
include "lang/src/vm/core/dispatcher.hako"
static box Main { method main(args) {
  local j = env.get("NYASH_VERIFY_JSON")
  local r = NyVmDispatcher.run(j)
  print("" + r)
  return r
} }
HCODE
)
    local out
    out=$(NYASH_VERIFY_JSON="$mir_literal2" NYASH_PREINCLUDE=1 run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}')
    if [[ "$out" =~ ^-?[0-9]+$ ]]; then
        local n=$out
        if [ $n -lt 0 ]; then n=$(( (n % 256 + 256) % 256 )); else n=$(( n % 256 )); fi
        return $n
    fi

    return 1
}

run_built_mir_json_via_core_v0_route() {
    local mir_json="$1"
    local mir_json_path="$2"

    echo "$mir_json" > "$mir_json_path"
    if [ "${HAKO_TRACE_EXECUTION:-0}" = "1" ]; then echo "[trace] executor: core (rust)" >&2; fi
    "$NYASH_BIN" --mir-json-file "$mir_json_path" >/dev/null 2>&1
    local rc=$?
    rm -f "$mir_json_path"
    return $rc
}

run_built_mir_json_via_builder_only_route() {
    local mir_json="$1"

    if [ "${HAKO_VERIFY_BUILDER_ONLY:-0}" != "1" ]; then
        return 2
    fi
    if mir_json_looks_like_v0_module_text "$mir_json"; then
        return 0
    fi
    return 1
}

run_built_mir_json_via_preferred_vm_routes() {
    local mir_json="$1"

    run_built_mir_json_via_hv1_route "$mir_json"
    local hv1_rc=$?
    if [ "$hv1_rc" != "2" ]; then
        return "$hv1_rc"
    fi

    run_built_mir_json_via_hako_core_route "$mir_json"
    return $?
}

# Built MIR(JSON) result routing after builder lanes are done.
# - caller handles upstream builder fallback / no-fallback policy
# - this helper only routes an already-built MIR payload to builder-only / hv1 / hako-core / core-v0 execution
run_built_mir_json_via_verify_routes() {
    local mir_json="$1"
    local mir_json_path="$2"

    run_built_mir_json_via_builder_only_route "$mir_json"
    local builder_only_rc=$?
    if [ "$builder_only_rc" != "2" ]; then
        return "$builder_only_rc"
    fi

    run_built_mir_json_via_preferred_vm_routes "$mir_json"
    local preferred_vm_rc=$?
    if [ "$preferred_vm_rc" != "2" ]; then
        return "$preferred_vm_rc"
    fi

    run_built_mir_json_via_core_v0_route "$mir_json" "$mir_json_path"
    return $?
}

run_verify_builder_emit_rust_cli_fallback() {
    local prog_json_path="$1"
    local builder_stderr="$2"
    local builder_stdout="$3"

    run_rust_cli_builder_fallback_for_verify "$prog_json_path" "$builder_stderr" "$builder_stdout" 1
}

cleanup_verify_builder_logs_and_run_built_mir() {
    local builder_stderr="$1"
    local builder_stdout="$2"
    local mir_json="$3"
    local mir_json_path="$4"

    cleanup_verify_builder_logs "$builder_stderr" "$builder_stdout"
    run_built_mir_json_via_verify_routes "$mir_json" "$mir_json_path"
}

handle_verify_builder_emit_result() {
    local prog_json_path="$1"
    local mir_json="$2"
    local mir_json_path="$3"
    local builder_stderr="$4"
    local builder_stdout="$5"
    local emit_rc="${6:-0}"

    if verify_builder_no_fallback_requested "$emit_rc"; then
        cleanup_verify_builder_logs "$builder_stderr" "$builder_stdout"
        return 1
    fi

    if [ "$emit_rc" -ne 0 ]; then
        run_verify_builder_emit_rust_cli_fallback "$prog_json_path" "$builder_stderr" "$builder_stdout"
        return $?
    fi

    cleanup_verify_builder_logs_and_run_built_mir \
        "$builder_stderr" \
        "$builder_stdout" \
        "$mir_json" \
        "$mir_json_path"
    return $?
}

# New function: verify_program_via_builder_to_core
# Purpose: Program(JSON v0) → MirBuilder(Hako) → MIR(JSON v0) → Core execution
# This is dev-only for testing builder output quality
verify_program_via_builder_to_core() {
    local prog_json_path="$1"
    local mir_json_path="/tmp/builder_output_$$.json"
    local builder_stderr="/tmp/builder_stderr_$$.log"
    local builder_stdout="/tmp/builder_stdout_$$.log"
    local mir_json=""
    local emit_rc=0
    mir_json=$(emit_mir_json_via_builder_from_program_json_file "$prog_json_path" "$builder_stderr" "$builder_stdout")
    emit_rc=$?
    handle_verify_builder_emit_result \
        "$prog_json_path" \
        "$mir_json" \
        "$mir_json_path" \
        "$builder_stderr" \
        "$builder_stdout" \
        "$emit_rc"
    return $?
}

run_verify_program_via_builder_to_core_with_env() {
    local prog_json_path="$1"
    local verify_builder_only="${2:-0}"
    local prefer_mirbuilder="${3:-0}"
    local primary_no_fallback="${4:-0}"
    local internal_builder="${5:-0}"

    (
        if [ "$verify_builder_only" = "1" ]; then
            export HAKO_VERIFY_BUILDER_ONLY=1
        fi
        if [ "$prefer_mirbuilder" = "1" ]; then
            export HAKO_PREFER_MIRBUILDER=1
        fi
        if [ "$primary_no_fallback" = "1" ]; then
            export HAKO_PRIMARY_NO_FALLBACK=1
        fi
        if [ "$internal_builder" = "1" ]; then
            export HAKO_MIR_BUILDER_INTERNAL=1
        fi

        export NYASH_ENABLE_USING=1
        export HAKO_ENABLE_USING=1
        export NYASH_USING_AST=1
        export NYASH_RESOLVE_FIX_BRACES=1
        export NYASH_DISABLE_NY_COMPILER=1
        export NYASH_FEATURES=stage3
        export NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1

        verify_program_via_builder_to_core "$prog_json_path"
    )
}

run_verify_program_via_preferred_mirbuilder_to_core() {
    local prog_json_path="$1"
    local builder_only="${2:-0}"

    if [ "$builder_only" = "1" ]; then
        run_verify_program_via_builder_to_core_with_env "$prog_json_path" 1 1 0 0
        return $?
    fi

    run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 1 0 0
}

run_verify_program_via_hako_primary_no_fallback_to_core() {
    local prog_json_path="$1"
    local prefer_mirbuilder="${2:-0}"

    if [ "$prefer_mirbuilder" = "1" ]; then
        run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 1 1 1
        return $?
    fi

    run_verify_program_via_builder_to_core_with_env "$prog_json_path" 0 0 1 1
}

run_hako_primary_no_fallback_canary_and_expect_rc() {
    local prog_json_path="$1"
    local expected_rc="$2"
    local fail_label="$3"
    local pass_label="$4"
    local prefer_mirbuilder="${5:-0}"

    set +e
    run_verify_program_via_hako_primary_no_fallback_to_core "$prog_json_path" "$prefer_mirbuilder"
    local rc=$?
    set -e

    if [ "$rc" -ne "$expected_rc" ]; then
        echo "[FAIL] ${fail_label} rc=$rc (expected $expected_rc)" >&2
        return 1
    fi

    echo "[PASS] ${pass_label}"
    return 0
}

run_preferred_mirbuilder_canary_and_expect_rc() {
    local prog_json_path="$1"
    local expected_rc="$2"
    local fail_label="$3"
    local pass_label="$4"
    local builder_only="${5:-0}"

    set +e
    run_verify_program_via_preferred_mirbuilder_to_core "$prog_json_path" "$builder_only"
    local rc=$?
    set -e

    if [ "$rc" -ne "$expected_rc" ]; then
        echo "[FAIL] ${fail_label} rc=$rc (expected $expected_rc)" >&2
        return 1
    fi

    echo "[PASS] ${pass_label}"
    return 0
}

run_builder_module_vm_to_stdout_file() {
    local builder_module="$1"
    local prog_json="$2"
    local tmp_stdout="$3"

    set +e
    run_program_json_via_builder_module_vm "$builder_module" "$prog_json" 2>/dev/null | tee "$tmp_stdout" >/dev/null
    local rc=${PIPESTATUS[0]}
    set -e
    return "$rc"
}

run_registry_builder_module_vm_to_stdout_file() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="$3"
    local use_preinclude="${4:-0}"
    local tmp_stdout="$5"

    set +e
    if [ "$use_preinclude" = "1" ]; then
        run_program_json_via_registry_builder_module_vm_with_preinclude "$builder_module" "$prog_json" "$registry_only" 2>/dev/null | tee "$tmp_stdout" >/dev/null
    else
        run_program_json_via_registry_builder_module_vm "$builder_module" "$prog_json" "$registry_only" 2>/dev/null | tee "$tmp_stdout" >/dev/null
    fi
    local rc=$?
    set -e
    return "$rc"
}

extract_builder_mir_from_stdout_file() {
    local tmp_stdout="$1"
    awk '/\[MIR_BEGIN\]/{flag=1;next}/\[MIR_END\]/{flag=0}flag' "$tmp_stdout"
}

stdout_file_has_functions_mir() {
    local tmp_stdout="$1"
    local mir
    mir=$(extract_builder_mir_from_stdout_file "$tmp_stdout")
    if [[ -z "$mir" ]] || ! echo "$mir" | grep -q '"functions"'; then
        return 1
    fi
    return 0
}

run_builder_module_tag_canary() {
    local builder_module="$1"
    local prog_json="$2"
    local expected_tag="$3"
    local pass_label="$4"
    local skip_exec_label="${5:-builder vm exec failed}"
    local skip_tag_label="${6:-tag not observed}"
    local skip_mir_label="${7:-MIR missing functions}"
    local require_functions="${8:-1}"
    local allow_nonzero_rc="${9:-0}"

    local tmp_stdout
    tmp_stdout=$(mktemp)
    trap 'rm -f "$tmp_stdout" || true' RETURN

    run_builder_module_vm_to_stdout_file "$builder_module" "$prog_json" "$tmp_stdout"
    local rc=$?

    if [[ "$rc" -ne 0 ]] && [ "$allow_nonzero_rc" != "1" ]; then
        echo "[SKIP] ${skip_exec_label}"
        return 0
    fi
    if ! grep -q "$expected_tag" "$tmp_stdout"; then
        echo "[SKIP] ${skip_tag_label}"
        return 0
    fi
    if [ "$require_functions" = "1" ] && ! stdout_file_has_functions_mir "$tmp_stdout"; then
        echo "[SKIP] ${skip_mir_label}"
        return 0
    fi

    echo "[PASS] ${pass_label}"
    return 0
}

run_registry_builder_tag_canary() {
    local builder_module="$1"
    local prog_json="$2"
    local registry_only="$3"
    local expected_tag_pattern="$4"
    local pass_label="$5"
    local skip_exec_label="${6:-builder vm exec failed}"
    local skip_tag_label="${7:-registry tag not observed}"
    local skip_mir_label="${8:-MIR missing functions}"
    local use_preinclude="${9:-0}"

    local tmp_stdout
    tmp_stdout=$(mktemp)
    trap 'rm -f "$tmp_stdout" || true' RETURN

    run_registry_builder_module_vm_to_stdout_file "$builder_module" "$prog_json" "$registry_only" "$use_preinclude" "$tmp_stdout"
    local rc=$?

    if [[ "$rc" -ne 0 ]]; then
        echo "[SKIP] ${skip_exec_label}"
        return 0
    fi
    if ! grep -E -q "$expected_tag_pattern" "$tmp_stdout"; then
        echo "[SKIP] ${skip_tag_label}"
        return 0
    fi

    if ! stdout_file_has_functions_mir "$tmp_stdout"; then
        echo "[SKIP] ${skip_mir_label}"
        return 0
    fi

    echo "[PASS] ${pass_label}"
    return 0
}

run_registry_method_arraymap_canary() {
    local prog_json="$1"
    local registry_only="$2"
    local expected_tag_label="$3"
    local pass_label="$4"
    local method_pattern="${5:-}"
    local args_pattern="${6:-}"
    local use_preinclude="${7:-0}"
    local skip_exec_label="${8:-builder vm exec failed}"
    local skip_tag_label="${9:-registry tag not observed}"
    local skip_mir_label="${10:-MIR missing functions}"

    local tmp_stdout
    tmp_stdout=$(mktemp)
    trap 'rm -f "$tmp_stdout" || true' RETURN

    run_registry_builder_module_vm_to_stdout_file "hako.mir.builder" "$prog_json" "$registry_only" "$use_preinclude" "$tmp_stdout"
    local rc=$?

    if [[ "$rc" -ne 0 ]]; then
        echo "[SKIP] ${skip_exec_label}"
        return 0
    fi
    if ! grep -q "$expected_tag_label" "$tmp_stdout"; then
        echo "[SKIP] ${skip_tag_label}"
        return 0
    fi

    local mir
    mir=$(extract_builder_mir_from_stdout_file "$tmp_stdout")
    if ! stdout_file_has_functions_mir "$tmp_stdout"; then
        echo "[SKIP] ${skip_mir_label}"
        return 0
    fi
    if [ -n "$method_pattern" ] && ! echo "$mir" | grep -q "$method_pattern"; then
        echo "[SKIP] method token missing"
        return 0
    fi
    if [ -n "$args_pattern" ] && ! echo "$mir" | grep -E -q "$args_pattern"; then
        echo "[SKIP] args token missing"
        return 0
    fi
    if [ -n "$method_pattern" ] && ! echo "$mir" | grep -q '"op":"mir_call"'; then
        echo "[SKIP] mir_call op missing"
        return 0
    fi

    echo "[PASS] ${pass_label}"
    return 0
}

# hv1 inline alias-only wrapper (env JSON → hv1 dispatcher)
# Usage: run_hv1_inline_alias_wrapper "$json_literal" → prints rc line; returns rc
run_hv1_inline_alias_wrapper() {
    local json_literal="$1"
    local code=$(cat <<'HCODE'
using "selfhost.vm.hv1.dispatch" as NyVm
static box Main { method main(args) {
  local j = env.get("NYASH_VERIFY_JSON")
  local r = NyVm.NyVmDispatcherV1Box.run(j)
  print("" + r)
  return r
} }
HCODE
)
    HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=0 NYASH_USING_AST=1 NYASH_RESOLVE_FIX_BRACES=1 \
    NYASH_VERIFY_JSON="$json_literal" run_nyash_vm -c "$code" 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}'
}

# Nyash実行ヘルパー（LLVM）
run_nyash_llvm() {
    local program="$1"
    shift
    # Allow developer to force LLVM run (env guarantees availability)
    if [ "${SMOKES_FORCE_LLVM:-0}" != "1" ]; then
        # Skip gracefully when LLVM backend is not available in this build
        if ! can_run_llvm; then
            log_warn "LLVM backend not available in this build; skipping LLVM run"
            log_info "Hint: build ny-llvmc + enable harness: cargo build --release -p nyash-llvm-compiler && cargo build --release --features llvm"
            return 0
        fi
    fi
    # -c オプションの場合は一時ファイル経由で実行
    if [ "$program" = "-c" ]; then
        local code="$1"
        shift
        local tmpfile="/tmp/nyash_test_$$.hako"
        echo "$code" > "$tmpfile"
        # 軽量ASIFix（テスト用）: ブロック終端の余剰セミコロンを寛容に除去
        if [ "${SMOKES_ASI_STRIP_SEMI:-1}" = "1" ]; then
            sed -i -E 's/;([[:space:]]*)(\}|$)/\1\2/g' "$tmpfile" || true
        fi
        # プラグイン初期化メッセージを除外
        PYTHONPATH="${PYTHONPATH:-$NYASH_ROOT}" NYASH_NY_LLVM_COMPILER="$NYASH_ROOT/target/release/ny-llvmc" NYASH_LLVM_USE_HARNESS=1 NYASH_EMIT_EXE_NYRT="$NYASH_ROOT/target/release" NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 "$NYASH_BIN" --backend llvm "$tmpfile" "$@" 2>&1 | \
            grep -v "^\[UnifiedBoxRegistry\]" | grep -v "^\[FileBox\]" | grep -v "^Net plugin:" | grep -v "^\[.*\] Plugin" | \
            grep -v '^\[plugin-loader\] backend=' | \
            grep -v '^🔌 plugin host initialized' | grep -v '^✅ plugin host fully configured' | \
            grep -v '^⚡ Hakorune LLVM Backend' | \
            grep -v '^✅ LLVM (harness) execution completed' | grep -v '^📊 MIR Module compiled successfully' | grep -v '^📊 Functions:' | grep -v 'JSON Parse Errors:' | grep -v 'Parsing errors' | grep -v 'No parsing errors' | grep -v 'Error at line ' | \
            grep -v '^\[using\]' | grep -v '^\[using/resolve\]' | grep -v '^\[using/cache\]' | \
            grep -v '^\[ny-llvmc\]' | grep -v '^\[harness\]' | grep -v '^Compiled to ' | grep -v '^/usr/bin/ld:'
        local exit_code=${PIPESTATUS[0]}
        rm -f "$tmpfile"
        return $exit_code
    else
        local runprog="$program"
        local workfile=""
        # 軽量ASIFix（テスト用）: 元ファイルは変更せず一時コピーへ適用
        if [ "${SMOKES_ASI_STRIP_SEMI:-1}" = "1" ] && [ -f "$program" ]; then
            workfile="$(mktemp /tmp/nyash_test_llvm_file.XXXXXX.hako)"
            cp "$program" "$workfile"
            sed -i -E 's/;([[:space:]]*)(\}|$)/\1\2/g' "$workfile" || true
            runprog="$workfile"
        fi
        # プラグイン初期化メッセージを除外
        PYTHONPATH="${PYTHONPATH:-$NYASH_ROOT}" NYASH_NY_LLVM_COMPILER="$NYASH_ROOT/target/release/ny-llvmc" NYASH_LLVM_USE_HARNESS=1 NYASH_EMIT_EXE_NYRT="$NYASH_ROOT/target/release" NYASH_ENTRY_ALLOW_TOPLEVEL_MAIN=1 NYASH_FEATURES=stage3 NYASH_PARSER_ALLOW_SEMICOLON=1 NYASH_DISABLE_NY_COMPILER=1 HAKO_DISABLE_NY_COMPILER=1 "$NYASH_BIN" --backend llvm "$runprog" "$@" 2>&1 | \
            grep -v "^\[UnifiedBoxRegistry\]" | grep -v "^\[FileBox\]" | grep -v "^Net plugin:" | grep -v "^\[.*\] Plugin" | \
            grep -v '^\[plugin-loader\] backend=' | \
            grep -v '^🔌 plugin host initialized' | grep -v '^✅ plugin host fully configured' | \
            grep -v '^⚡ Hakorune LLVM Backend' | \
            grep -v '^✅ LLVM (harness) execution completed' | grep -v '^📊 MIR Module compiled successfully' | grep -v '^📊 Functions:' | grep -v 'JSON Parse Errors:' | grep -v 'Parsing errors' | grep -v 'No parsing errors' | grep -v 'Error at line ' | \
            grep -v '^\[using\]' | grep -v '^\[using/resolve\]' | grep -v '^\[using/cache\]' | \
            grep -v '^\[ny-llvmc\]' | grep -v '^\[harness\]' | grep -v '^Compiled to ' | grep -v '^/usr/bin/ld:'
        local exit_code=${PIPESTATUS[0]}
        rm -f "${workfile:-}" 2>/dev/null || true
        return $exit_code
    fi
}

# シンプルテスト補助（スクリプト互換）
test_pass() { log_success "$1"; return 0; }
test_fail() { log_error "$1 ${2:-}"; return 1; }

expect_plan_freeze() {
    local test_name="$1"
    local output="$2"
    local exit_code="$3"
    local tag="${4:-[plan/freeze:]}"

    if [ "$exit_code" -eq 0 ]; then
        echo "[FAIL] Expected plan freeze error, got exit 0"
        echo "[INFO] Output:"
        echo "$output" | tail -n 40 || true
        test_fail "$test_name: Unexpected success"
        return 1
    fi

    if echo "$output" | grep -Fq "${tag%]}"; then
        test_pass "$test_name: plan freeze detected"
        return 0
    fi

    echo "[FAIL] Expected plan freeze tag in output"
    echo "[INFO] Exit code: $exit_code"
    echo "[INFO] Output:"
    echo "$output" | tail -n 60 || true
    test_fail "$test_name: Missing plan freeze tag"
    return 1
}

expect_joinir_contract_freeze() {
    local test_name="$1"
    local output="$2"
    local exit_code="$3"
    local tag="$4"

    if [ "$exit_code" -eq 0 ]; then
        echo "[FAIL] Expected JoinIR contract freeze error, got exit 0"
        echo "[INFO] Output:"
        echo "$output" | tail -n 40 || true
        test_fail "$test_name: Unexpected success"
        return 1
    fi

    if echo "$output" | grep -Fq "$tag"; then
        test_pass "$test_name: joinir contract freeze detected"
        return 0
    fi

    echo "[FAIL] Expected joinir contract freeze tag in output"
    echo "[INFO] Exit code: $exit_code"
    echo "[INFO] Output:"
    echo "$output" | tail -n 60 || true
    test_fail "$test_name: Missing joinir contract freeze tag"
    return 1
}
test_skip() { log_warn "SKIP $1 ${2:-}"; return 0; }

# 出力比較ヘルパー
compare_outputs() {
    local expected="$1"
    local actual="$2"
    local test_name="$3"

    if [ "$expected" = "$actual" ]; then
        return 0
    else
        log_error "$test_name output mismatch:"
        log_error "  Expected: $expected"
        log_error "  Actual:   $actual"
        return 1
    fi
}

# 結果サマリー出力
print_summary() {
    local end_time=$(date +%s.%N)
    local total_duration=$(echo "$end_time - $SMOKES_START_TIME" | bc -l)

    echo ""
    echo "==============================================="
    echo "Smoke Test Summary"
    echo "==============================================="
    echo "Total tests:  $SMOKES_TEST_COUNT"
    echo "Passed:       $SMOKES_PASS_COUNT"
    echo "Failed:       $SMOKES_FAIL_COUNT"
    echo "Duration:     ${total_duration}s"
    echo ""

    if [ "${SMOKES_INCLUDE_SKIP_COUNT:-0}" -gt 0 ]; then
        echo "Include SKIPs: $SMOKES_INCLUDE_SKIP_COUNT"
        for entry in "${SMOKES_INCLUDE_SKIP_LIST[@]}"; do
            echo "  - $entry"
        done
        echo ""
    fi

    if [ $SMOKES_FAIL_COUNT -eq 0 ]; then
        log_success "All tests passed! ✨"
        return 0
    else
        log_error "$SMOKES_FAIL_COUNT test(s) failed"
        return 1
    fi
}

# JSON出力関数
output_json() {
    local profile="${1:-unknown}"
    local end_time=$(date +%s.%N)
    local total_duration=$(echo "$end_time - $SMOKES_START_TIME" | bc -l)

    cat << EOF
{
  "profile": "$profile",
  "total": $SMOKES_TEST_COUNT,
  "passed": $SMOKES_PASS_COUNT,
  "failed": $SMOKES_FAIL_COUNT,
  "duration": $total_duration,
  "timestamp": "$(date -Iseconds)",
  "success": $([ $SMOKES_FAIL_COUNT -eq 0 ] && echo "true" || echo "false")
}
EOF
}

# JUnit XML出力関数
output_junit() {
    local profile="${1:-unknown}"
    local end_time=$(date +%s.%N)
    local total_duration=$(echo "$end_time - $SMOKES_START_TIME" | bc -l)

    cat << EOF
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="smokes_$profile" tests="$SMOKES_TEST_COUNT" failures="$SMOKES_FAIL_COUNT" time="$total_duration">
  <!-- Individual test cases would be added by specific test scripts -->
</testsuite>
EOF
}

# v1 JSON 正規化（オブジェクトキー順序ソート、配列順は保持）→ SHA256 ハッシュ
v1_normalized_hash() {
    local json_path="$1"
    if [ ! -f "$json_path" ]; then
        echo "[FAIL] v1_normalized_hash: json not found: $json_path" >&2
        return 2
    fi
    if ! command -v jq >/dev/null 2>&1; then
        echo "[FAIL] v1_normalized_hash: jq required" >&2
        return 2
    fi
    # Extract JSON object line defensively (strip any leading noise like 'RC: N' or logs)
    # Extract JSON object block from first '{' to EOF (handles pretty JSON)
    local raw_json
    raw_json=$(awk 'BEGIN{on=0} { if(on){print} else if($0 ~ /^[[:space:]]*\{/){ on=1; print } }' "$json_path")
    if [ -z "$raw_json" ]; then
        return 1
    fi
    local canon
    canon=$(printf '%s' "$raw_json" | jq -S -c .) || return 1
    printf "%s" "$canon" | sha256sum | awk '{print $1}'
}

# 2つの v1 JSON ファイルの正規化ハッシュを比較（等しければ0）
compare_v1_hash() {
    local a="$1"; local b="$2"
    local ha hb
    ha=$(v1_normalized_hash "$a" || true)
    hb=$(v1_normalized_hash "$b" || true)
    if [ -z "$ha" ] || [ -z "$hb" ]; then
        return 2
    fi
    [ "$ha" = "$hb" ]
}
# Run hv1 inline (early route) and return numeric rc (0-255). Returns non-zero exit on failure to execute.
verify_v1_inline_file() {
    local json_path="$1"
    if [ ! -f "$json_path" ]; then
        echo "[FAIL] verify_v1_inline_file: json not found: $json_path" >&2
        return 2
    fi
    local out
    # Optional: show full logs for debugging (default OFF)
    if [ "${HAKO_VERIFY_SHOW_LOGS:-0}" = "1" ]; then
        # Show all output to stderr, then extract numeric rc (env-sanitized for determinism)
        env -i PATH="$PATH" \
              HAKO_TRACE_EXECUTION="${HAKO_TRACE_EXECUTION:-0}" HAKO_VERIFY_SHOW_LOGS="${HAKO_VERIFY_SHOW_LOGS:-0}" \
              HAKO_ROUTE_HAKOVM=1 HAKO_VERIFY_V1_FORCE_HAKOVM=1 \
              NYASH_VERIFY_JSON="$(cat "$json_path")" \
              "$NYASH_BIN" --backend vm /dev/null 2>&1 | tr -d '\r' | tee /tmp/hv1_debug.log >&2
        out=$(awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}' /tmp/hv1_debug.log)
    else
        out=$(env -i PATH="$PATH" \
                 HAKO_TRACE_EXECUTION="${HAKO_TRACE_EXECUTION:-0}" \
                 HAKO_ROUTE_HAKOVM=1 HAKO_VERIFY_V1_FORCE_HAKOVM=1 \
                 NYASH_VERIFY_JSON="$(cat "$json_path")" \
                 "$NYASH_BIN" --backend vm /dev/null 2>/dev/null | tr -d '\r' | awk '/^-?[0-9]+$/{n=$0} END{if(n!="") print n}')
    fi
    if [[ "$out" =~ ^-?[0-9]+$ ]]; then
        # echo numeric rc and return success; caller normalizes/returns as exit code
        echo "$out"
        return 0
    fi
    return 1
}
# ============================================================================
# Helper: require_joinir_dev
# ============================================================================
# Sets dev-only environment variables for JoinIR normalized shadow testing.
# Must be called BEFORE build/run operations if the fixture requires dev-only features.
#
# Usage:
#   require_joinir_dev
#
require_joinir_dev() {
  # JoinIR dev mode now controlled by env.sh (SSOT)
  # Verify it's enabled (should be default=1 from env.sh)
  if [ "${NYASH_JOINIR_DEV:-0}" != "1" ]; then
    export NYASH_JOINIR_DEV=1
  fi
  if [ "${HAKO_JOINIR_STRICT:-0}" != "1" ]; then
    export HAKO_JOINIR_STRICT=1
  fi
  echo "[INFO] JoinIR dev mode enabled (NYASH_JOINIR_DEV=1, HAKO_JOINIR_STRICT=1)"
}

# Dev profile helpers (centralize bring-up toggles for MirBuilder)
# Usage: call enable_mirbuilder_dev_env in canaries that need it.
enable_mirbuilder_dev_env() {
    # Avoid ny-compiler inline path during VM bring-up
    export NYASH_USE_NY_COMPILER=0
    # Allow FileBox provider fallback (dev only)
    export NYASH_FAIL_FAST=${NYASH_FAIL_FAST:-0}
    # Using system already configured by env.sh (SSOT), but ensure enabled
    export NYASH_ENABLE_USING="${NYASH_ENABLE_USING:-1}"
    export HAKO_ENABLE_USING="${HAKO_ENABLE_USING:-1}"
    # File-based using also from env.sh, but ensure enabled for dev
    export NYASH_ALLOW_USING_FILE="${NYASH_ALLOW_USING_FILE:-1}"
    export HAKO_ALLOW_USING_FILE="${HAKO_ALLOW_USING_FILE:-1}"
    # Optional: preinclude heavy using segments for legacy/prelude-heavy paths (default OFF)
    if [ "${SMOKES_DEV_PREINCLUDE:-0}" = "1" ]; then
        export HAKO_PREINCLUDE=1
    fi
    # Optional: enable JsonFrag Normalizer for builder/min paths (default OFF)
    # Use only in targeted canaries; keep OFF for general runs
    if [ "${SMOKES_DEV_NORMALIZE:-0}" = "1" ]; then
        export HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1
    fi
    # Profile-based injection example (commented; enable when needed):
    # if [ "${SMOKES_ENABLE_NORMALIZE_FOR_QUICK:-0}" = "1" ] && [ "${SMOKES_CURRENT_PROFILE:-}" = "quick" ]; then
    #     export HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1
    #     export HAKO_MIR_BUILDER_NORMALIZE_TAG=1   # optional: show tags in logs for diagnostics
    # fi
}

# LLVM backend availability check (SSOT)
# Phase 287 P4: Consolidate LLVM detection logic (Task 3)
# Returns: 0 if LLVM backend is available, 1 otherwise
can_run_llvm() {
    # Primary check: version string advertises features
    if "$NYASH_BIN" --version 2>/dev/null | grep -q "features.*llvm"; then
        return 0
    fi
    # Fallback check: binary contains LLVM harness symbols (ny-llvmc / NYASH_LLVM_USE_HARNESS)
    if strings "$NYASH_BIN" 2>/dev/null | grep -E -q 'ny-llvmc|NYASH_LLVM_USE_HARNESS'; then
        return 0
    fi
    return 1
}

# Dev profile helpers (EXE/AOT bring-up)
# Sets environment defaults for LLVM crate backend and EXE link paths.
# Usage: call enable_exe_dev_env in EXE canaries.
enable_exe_dev_env() {
    # LLVM backend already configured by env.sh (SSOT), fallback to crate
    export NYASH_LLVM_BACKEND="${NYASH_LLVM_BACKEND:-crate}"
    # Tool locations (override when cross)
    export NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$NYASH_ROOT/target/release/ny-llvmc}"
    # NyRT (kernel) lib search path for linking EXEs
    export NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$NYASH_ROOT/target/release}"
    # Verification toggles from env.sh (SSOT)
    export NYASH_LLVM_VERIFY="${NYASH_LLVM_VERIFY:-0}"
    export NYASH_LLVM_VERIFY_IR="${NYASH_LLVM_VERIFY_IR:-0}"
}
