#!/bin/bash
# run.sh - スモークテストv2 単一エントリポイント
# Usage: ./run.sh --profile {quick|integration|full} [options]

set -euo pipefail

# スクリプトディレクトリ
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# デフォルト値
PROFILE="quick"
SUITE=""
FORMAT="text"
JOBS=1
TIMEOUT=""
VERBOSE=false
DRY_RUN=false
FILTER=""
FORCE_CONFIG=""
TRACE_VM=false
SKIP_PREFLIGHT=false

# カラー定義
readonly RED='\033[0;31m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[1;33m'
readonly BLUE='\033[0;34m'
readonly BOLD='\033[1m'
readonly NC='\033[0m'

# ログ関数
log_info() {
    echo -e "${BLUE}[INFO]${NC} $*" >&2
}

log_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $*" >&2
}

log_warn() {
    echo -e "${YELLOW}[WARN]${NC} $*" >&2
}

log_error() {
    echo -e "${RED}[ERROR]${NC} $*" >&2
}

log_header() {
    echo -e "${BOLD}$*${NC}" >&2
}

# ヘルプ表示
show_help() {
    cat << 'EOF'
Smoke Tests v2 - Nyash 2-Pillar Testing System

Usage:
  ./run.sh --profile PROFILE [options]

Profiles:
  quick        Development-time fast checks (1-2 min)
  integration  Basic VM ↔ LLVM parity checks (5-10 min)
  full         Complete matrix testing (15-30 min)

Options:
  --profile PROFILE         Test profile to run
  --suite SUITE            Optional suite manifest under suites/<profile>/
  --filter "PATTERN"        Test filter (e.g., "boxes:string")
  --format FORMAT           Output format: text|json|junit
  --jobs N                  Parallel execution count
  --timeout SEC             Timeout per test in seconds
  --force-config CONFIG     Force configuration: rust_vm_dynamic|llvm_static
  --verbose                 Enable verbose output
  --trace-vm                Enable VM trace (NYASH_VM_TRACE=1, HAKO_SILENT_TAGS=0)
  --dry-run                 Show test list without execution
  --skip-preflight          Skip preflight checks (useful for repeated gate runs)
  --help                    Show this help

Examples:
  # Quick development check
  ./run.sh --profile quick

  # Quick with Normalizer ON and heavy AOT cases present
  # Tip: increase timeout for EXE build/link reps (e.g., phase2100)
  HAKO_MIR_BUILDER_JSONFRAG_NORMALIZE=1 ./run.sh --profile quick --timeout 120

  # Integration with filter
  ./run.sh --profile integration --filter "plugins:*"

  # Integration suite manifest
  ./run.sh --profile integration --suite presubmit

  # Full testing with JSON output
  ./run.sh --profile full --format json --jobs 4 --timeout 300

  # Dry run to see what would be tested
  ./run.sh --profile integration --dry-run

Environment Variables:
  SMOKES_FORCE_CONFIG       Force specific configuration
  SMOKES_PLUGIN_MODE        Plugin mode: dynamic|static
  SMOKES_REPRO              On failure, rerun the same test N times (flake detection; still fails)
  CI                        CI environment detection
EOF
}

# 引数パース
parse_arguments() {
    while [[ $# -gt 0 ]]; do
        case $1 in
            --profile)
                PROFILE="$2"
                shift 2
                ;;
            --suite)
                SUITE="$2"
                shift 2
                ;;
            --filter)
                FILTER="$2"
                shift 2
                ;;
            --format)
                FORMAT="$2"
                shift 2
                ;;
            --jobs)
                JOBS="$2"
                shift 2
                ;;
            --timeout)
                TIMEOUT="$2"
                shift 2
                ;;
            --force-config)
                FORCE_CONFIG="$2"
                shift 2
                ;;
            --verbose)
                VERBOSE=true
                shift
                ;;
            --trace-vm)
                TRACE_VM=true
                shift
                ;;
            --dry-run)
                DRY_RUN=true
                shift
                ;;
            --skip-preflight)
                SKIP_PREFLIGHT=true
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                log_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done

    # プロファイル検証
    case "$PROFILE" in
        quick|integration|full|plugins)
            ;;
        *)
            log_error "Invalid profile: $PROFILE"
            log_error "Valid profiles: quick, integration, full, plugins"
            exit 1
            ;;
    esac

    # フォーマット検証
    case "$FORMAT" in
        text|json|junit)
            ;;
        *)
            log_error "Invalid format: $FORMAT"
            log_error "Valid formats: text, json, junit"
            exit 1
            ;;
    esac
}

trim_manifest_line() {
    printf '%s' "$1" | sed -e 's/^[[:space:]]*//' -e 's/[[:space:]]*$//'
}

declare -A SUITE_ALLOWLIST=()
declare -A SUITE_DISCOVERED=()

load_suite_manifest() {
    if [ -z "$SUITE" ]; then
        return 0
    fi

    local manifest="$SCRIPT_DIR/suites/$PROFILE/$SUITE.txt"
    local raw_line=""
    local line=""
    local line_no=0

    if [ ! -f "$manifest" ]; then
        log_error "Suite manifest not found: $manifest"
        return 1
    fi

    SUITE_ALLOWLIST=()
    SUITE_DISCOVERED=()

    while IFS= read -r raw_line || [ -n "$raw_line" ]; do
        line_no=$((line_no + 1))
        line="$(trim_manifest_line "$raw_line")"
        case "$line" in
            ""|\#*)
                continue
                ;;
        esac

        if [[ "$line" = /* ]]; then
            log_error "Suite manifest must use profile-relative paths: $manifest:$line_no"
            return 1
        fi

        if [ -n "${SUITE_ALLOWLIST[$line]+x}" ]; then
            log_error "Duplicate suite entry: $manifest:$line_no -> $line"
            return 1
        fi

        SUITE_ALLOWLIST["$line"]=1
        SUITE_DISCOVERED["$line"]=0
    done < "$manifest"

    if [ ${#SUITE_ALLOWLIST[@]} -eq 0 ]; then
        log_error "Suite manifest is empty: $manifest"
        return 1
    fi
}

validate_suite_manifest_hits() {
    if [ -z "$SUITE" ]; then
        return 0
    fi

    local manifest="$SCRIPT_DIR/suites/$PROFILE/$SUITE.txt"
    local entry=""
    local missing=()

    for entry in "${!SUITE_ALLOWLIST[@]}"; do
        if [ "${SUITE_DISCOVERED[$entry]:-0}" != "1" ]; then
            missing+=("$entry")
        fi
    done

    if [ ${#missing[@]} -eq 0 ]; then
        return 0
    fi

    log_error "Suite manifest contains non-live or missing entries: $manifest"
    for entry in "${missing[@]}"; do
        echo "  - $entry" >&2
    done
    return 1
}

# 環境設定
setup_environment() {
    log_info "Setting up environment for profile: $PROFILE"

    # 共通ライブラリ読み込み
    source "$SCRIPT_DIR/lib/test_runner.sh"
    source "$SCRIPT_DIR/lib/plugin_manager.sh"
    source "$SCRIPT_DIR/lib/result_checker.sh"
    source "$SCRIPT_DIR/lib/preflight.sh"

    # 設定読み込み
    if [ -n "$FORCE_CONFIG" ]; then
        export SMOKES_FORCE_CONFIG="$FORCE_CONFIG"
        log_info "Forced configuration: $FORCE_CONFIG"
    fi

    source "$SCRIPT_DIR/configs/auto_detect.conf"
    auto_configure "$PROFILE"

    # プロファイル専用設定
    export SMOKES_CURRENT_PROFILE="$PROFILE"
    # JoinIR は常時 ON（NYASH_JOINIR_CORE は deprecated/no-op）

    # コマンドライン引数の環境変数設定
    if [ -n "$TIMEOUT" ]; then
        export SMOKES_DEFAULT_TIMEOUT="$TIMEOUT"
    fi

    if [ "$VERBOSE" = true ]; then
        export NYASH_CLI_VERBOSE=1
        export SMOKES_LOG_LEVEL="debug"
    fi

    if [ "$TRACE_VM" = true ]; then
        export NYASH_VM_TRACE=1
        export HAKO_SILENT_TAGS=0
    fi

    export SMOKES_PARALLEL_JOBS="$JOBS"
    export SMOKES_OUTPUT_FORMAT="$FORMAT"
    export SMOKES_TEST_FILTER="$FILTER"

    # 作業ディレクトリ移動（Nyashプロジェクトルートへ）
    cd "$SCRIPT_DIR/../../.."
    log_info "Working directory: $(pwd)"
}

# プリフライトチェック
run_preflight() {
    log_info "Running preflight checks..."

    if ! preflight_all; then
        log_error "Preflight checks failed"
        log_error "Run with --verbose for detailed information"
        exit 1
    fi

    log_success "Preflight checks passed"
}

dump_fail_env() {
    local force_cfg="${SMOKES_FORCE_CONFIG:-}"
    if [ -z "$force_cfg" ]; then
        force_cfg="<auto>"
    fi
    echo -e "${YELLOW}[WARN]${NC} env: profile=${SMOKES_CURRENT_PROFILE:-$PROFILE} config=${force_cfg} plugin_mode=${SMOKES_PLUGIN_MODE:-<unset>} backend=${NYASH_BACKEND:-default} compare_adopt=${NYASH_OPERATOR_BOX_COMPARE_ADOPT:-<unset>} add_adopt=${NYASH_OPERATOR_BOX_ADD_ADOPT:-<unset>} tolerate_void=${NYASH_VM_TOLERATE_VOID:-<unset>} vm_trace=${NYASH_VM_TRACE:-<unset>} silent_tags=${HAKO_SILENT_TAGS:-<unset>}"
}

# テストファイル検索
find_test_files() {
    local profile_dir="$SCRIPT_DIR/profiles/$PROFILE"
    local test_files=()
    local prune_dirs="${SMOKES_DISCOVERY_PRUNE_DIRS:-archive:lib:tmp:fixtures}"
    if [ -n "$SUITE" ]; then
        prune_dirs="${SMOKES_DISCOVERY_PRUNE_DIRS_WITH_SUITE:-lib:tmp:fixtures}"
    fi
    local -a prune_names=()
    local -a find_expr=()
    local prune_added=0
    local have_llvm=0
    if [ "${SMOKES_FORCE_LLVM:-0}" = "1" ]; then
        have_llvm=1
    fi
    local cli_bin="${NYASH_BIN_RESOLVED:-./target/release/hakorune}"
    if [ -x "$cli_bin" ]; then
        if "$cli_bin" --version 2>/dev/null | grep -q "features.*llvm"; then
            have_llvm=1
        else
            # Fallback detection: check for LLVM harness symbols in the binary
            if strings "$cli_bin" 2>/dev/null | grep -E -q 'ny-llvmc|NYASH_LLVM_USE_HARNESS'; then
                have_llvm=1
            fi
        fi
    fi

    if [ ! -d "$profile_dir" ]; then
        log_error "Profile directory not found: $profile_dir"
        return 1
    fi

    load_suite_manifest || return 1

    IFS=':' read -r -a prune_names <<< "$prune_dirs"
    for prune_name in "${prune_names[@]}"; do
        if [ -z "$prune_name" ]; then
            continue
        fi
        if [ $prune_added -eq 0 ]; then
            find_expr+=( "(" -type d "(" -name "$prune_name" )
        else
            find_expr+=( -o -name "$prune_name" )
        fi
        prune_added=1
    done

    if [ $prune_added -eq 1 ]; then
        find_expr+=( ")" -prune ")" -o "(" -type f -name "*.sh" -print0 ")" )
    else
        find_expr=( -type f -name "*.sh" -print0 )
    fi

    # テストファイル検索
    while IFS= read -r -d '' file; do
        local relative_path
        relative_path=$(realpath --relative-to="$profile_dir" "$file")

        if [ -n "$SUITE" ]; then
            if [ -z "${SUITE_ALLOWLIST[$relative_path]+x}" ]; then
                continue
            fi
            SUITE_DISCOVERED["$relative_path"]=1
        fi

        # フィルタ適用
        if [ -n "$FILTER" ]; then
            if ! grep -q "$FILTER" <<<"$relative_path"; then
                continue
            fi
        fi
        # LLVM未ビルド時は AST(LLVM) 系テストをスキップ
        if [ $have_llvm -eq 0 ] && grep -q "_ast\\.sh$" <<<"$file"; then
            log_warn "Skipping (no LLVM): $file"
            continue
        fi
        test_files+=("$file")
    done < <(find "$profile_dir" "${find_expr[@]}")

    validate_suite_manifest_hits || return 1

    if [ ${#test_files[@]} -gt 0 ]; then
        printf '%s\n' "${test_files[@]}"
    fi
}

# 単一テスト実行
run_single_test() {
    local test_file="$1"
    local test_name
    test_name=$(basename "$test_file" .sh)

    if [ "$FORMAT" = "text" ]; then
        echo -n "Running $test_name... "
    fi

    local start_time end_time duration exit_code
    start_time=$(date +%s.%N)

    # タイムアウト付きテスト実行
    local timeout_cmd=""
    if [ -n "${SMOKES_DEFAULT_TIMEOUT:-}" ]; then
        timeout_cmd="timeout ${SMOKES_DEFAULT_TIMEOUT}"
    fi

    # 詳細ログ: 失敗時のみテイル表示
    local log_file
    log_file="/tmp/hakorune_smoke_$(date +%s)_$$.log"
    if $timeout_cmd bash "$test_file" >"$log_file" 2>&1; then
        exit_code=0
    else
        exit_code=$?
    fi

    end_time=$(date +%s.%N)
    duration=$(echo "$end_time - $start_time" | bc -l)
    if [[ "$duration" == -* ]]; then
        duration=0
    fi

    # 結果出力
    case "$FORMAT" in
        text)
            if [ $exit_code -eq 0 ]; then
                echo -e "${GREEN}PASS${NC} (${duration}s)"
            else
                echo -e "${RED}FAIL${NC} (exit=$exit_code, ${duration}s)"
                dump_fail_env
                echo -e "${YELLOW}[WARN]${NC} Test file: $test_file"
                echo -e "${YELLOW}[WARN]${NC} Re-run (runner): ./tools/smokes/v2/run.sh --profile \"$PROFILE\" --filter \"$(basename "$test_file")\""
                echo -e "${YELLOW}[WARN]${NC} Re-run (direct): bash \"$test_file\""
                local TAIL_N="${SMOKES_NOTIFY_TAIL:-80}"
                echo "----- LOG (tail -n $TAIL_N) -----"
                tail -n "$TAIL_N" "$log_file" || true
                echo "----- END LOG -----"

                # Optional: flake detection by rerunning the exact same test script
                local repro_n="${SMOKES_REPRO:-0}"
                if [[ "$repro_n" =~ ^[0-9]+$ ]] && [ "$repro_n" -gt 0 ]; then
                    local retry
                    local passed_on_retry=0
                    local pass_log=""
                    for retry in $(seq 1 "$repro_n"); do
                        local retry_log="/tmp/hakorune_smoke_retry_${test_name}_$(date +%s)_$$_${retry}.log"
                        if $timeout_cmd bash "$test_file" >"$retry_log" 2>&1; then
                            passed_on_retry=1
                            pass_log="$retry_log"
                            break
                        fi
                    done
                    if [ "$passed_on_retry" -eq 1 ]; then
                        echo -e "${YELLOW}[WARN]${NC} Flaky test detected: failed first, passed on retry ${retry}/${repro_n}"
                        echo -e "${YELLOW}[WARN]${NC} First-failure log: $log_file"
                        echo -e "${YELLOW}[WARN]${NC} Passing-retry log: $pass_log"
                    else
                        echo -e "${YELLOW}[WARN]${NC} Repro: still failing after ${repro_n} retries (log: $log_file)"
                    fi
                fi
            fi
            ;;
        json)
            local status_json
            status_json=$([ $exit_code -eq 0 ] && echo "pass" || echo "fail")
            echo "{\"name\":\"$test_name\",\"path\":\"$test_file\",\"status\":\"$status_json\",\"duration\":$duration,\"exit\":$exit_code}"
            ;;
        junit)
            # JUnit形式は後でまとめて出力（pathも保持）
            echo "$test_name:$exit_code:$duration:$test_file" >> /tmp/junit_results.txt
            ;;
    esac

    # 後始末
    if [ $exit_code -eq 0 ]; then
        rm -f "$log_file" 2>/dev/null || true
    else
        # Keep failure log when repro is enabled; otherwise cleanup as before.
        local repro_n="${SMOKES_REPRO:-0}"
        if [[ "$repro_n" =~ ^[0-9]+$ ]] && [ "$repro_n" -gt 0 ]; then
            : # keep
        else
            rm -f "$log_file" 2>/dev/null || true
        fi
    fi
    return $exit_code
}

# テスト実行
run_tests() {
    local test_files
    local test_files_raw=""
    if ! test_files_raw="$(find_test_files)"; then
        return 1
    fi
    if [ -n "$test_files_raw" ]; then
        mapfile -t test_files <<< "$test_files_raw"
    else
        test_files=()
    fi

    if [ ${#test_files[@]} -eq 0 ]; then
        log_warn "No test files found for profile: $PROFILE"
        if [ -n "$SUITE" ]; then
            log_warn "Suite applied: $SUITE"
        fi
        if [ -n "$FILTER" ]; then
            log_warn "Filter applied: $FILTER"
        fi
        exit 0
    fi

    log_info "Found ${#test_files[@]} test files"

    # Dry run
    if [ "$DRY_RUN" = true ]; then
        log_header "Test files that would be executed:"
        for file in "${test_files[@]}"; do
            echo "  $(realpath --relative-to="$SCRIPT_DIR" "$file")"
        done
        exit 0
    fi

    # テスト実行開始
    log_header "Starting $PROFILE profile tests"

    local passed=0
    local failed=0
    local start_time
    start_time=$(date +%s.%N)

    # JSON形式の場合はヘッダー出力
    if [ "$FORMAT" = "json" ]; then
        echo '{"profile":"'$PROFILE'","tests":['
    fi

    # JUnit用ファイル初期化
    if [ "$FORMAT" = "junit" ]; then
        echo -n > /tmp/junit_results.txt
    fi

    # テスト実行
    local first_test=true
    for test_file in "${test_files[@]}"; do
        if [ "$FORMAT" = "json" ] && [ "$first_test" = false ]; then
            echo ","
        fi
        first_test=false

        if run_single_test "$test_file"; then
            passed=$((passed+1))
        else
            failed=$((failed+1))
            # Fast fail モード
            if [ "${SMOKES_FAST_FAIL:-0}" = "1" ]; then
                log_warn "Fast fail enabled, stopping on first failure"
                break
            fi
        fi
    done

    # 結果出力
    local end_time total_duration
    end_time=$(date +%s.%N)
    total_duration=$(echo "$end_time - $start_time" | bc -l)
    if [[ "$total_duration" == -* ]]; then
        total_duration=0
    fi

    case "$FORMAT" in
        text)
            echo ""
            log_header "Test Results Summary"
            echo "Profile: $PROFILE"
            echo "Total: $((passed + failed))"
            echo "Passed: $passed"
            echo "Failed: $failed"
            echo "Duration: ${total_duration}s"

            if [ $failed -eq 0 ]; then
                log_success "All tests passed! ✨"
            else
                log_error "$failed test(s) failed"
            fi
            ;;
        json)
            echo '],"summary":{"total":'$((passed + failed))',"passed":'$passed',"failed":'$failed',"duration":'$total_duration'}}'
            ;;
        junit)
            cat << EOF
<?xml version="1.0" encoding="UTF-8"?>
<testsuite name="smokes_$PROFILE" tests="$((passed + failed))" failures="$failed" time="$total_duration">
EOF
            while IFS=':' read -r name exit_code duration path; do
                if [ "$exit_code" = "0" ]; then
                    echo "  <testcase name=\"$name\" time=\"$duration\" classname=\"$path\"/>"
                else
                    echo "  <testcase name=\"$name\" time=\"$duration\" classname=\"$path\"><failure message=\"exit=$exit_code\"/></testcase>"
                fi
            done < /tmp/junit_results.txt
            echo "</testsuite>"
            rm -f /tmp/junit_results.txt
            ;;
    esac

    # Post-run cleanup: remove smokes-generated crate EXE object files under /tmp
    rm -f /tmp/ny_crate_backend_exe_*.o 2>/dev/null || true

    # 終了コード
    [ $failed -eq 0 ]
}

# メイン処理
main() {
    # 引数パース
    parse_arguments "$@"

    # バナー表示
    if [ "$FORMAT" = "text" ]; then
        log_header "🔥 Hakorune Smoke Tests v2 - 2-Pillar Testing System"
        log_info "Profile: $PROFILE | Format: $FORMAT | Jobs: $JOBS"
        if [ -n "$SUITE" ]; then
            log_info "Suite: $SUITE"
        fi
        if [ -n "$FILTER" ]; then
            log_info "Filter: $FILTER"
        fi
        echo ""
    fi

    # 環境設定
    setup_environment

    # プリフライト
    if [ "$SKIP_PREFLIGHT" = true ]; then
        log_info "Skipping preflight checks (--skip-preflight)"
    else
        run_preflight
    fi

    # テスト実行
    run_tests
}

# エラーハンドリング
trap 'log_error "Script interrupted"; exit 130' INT TERM

# メイン実行
main "$@"
