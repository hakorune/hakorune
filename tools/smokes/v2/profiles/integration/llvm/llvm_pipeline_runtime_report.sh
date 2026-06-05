#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
if ROOT_GIT=$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null); then
  ROOT="$ROOT_GIT"
else
  ROOT="$(cd "$SCRIPT_DIR/../../../../../.." && pwd)"
fi
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/hako_llvm_pipeline_report.XXXXXX")"
APP="$TMP_DIR/main.hako"
REPORT="$TMP_DIR/pipeline.kv"
STDOUT_LOG="$TMP_DIR/stdout.log"
STDERR_LOG="$TMP_DIR/stderr.log"
trap 'rm -rf "$TMP_DIR"' EXIT

cat >"$APP" <<'HAKO'
static box Main {
    main(args) {
        return 0
    }
}
HAKO

cargo build -q --bin hakorune

set +e
NYASH_LLVM_PIPELINE_REPORT_OUT="$REPORT" \
  "$ROOT/target/debug/hakorune" --backend llvm "$APP" >"$STDOUT_LOG" 2>"$STDERR_LOG"
RC=$?
set -e

if [ "$RC" -ne 42 ]; then
  echo "[TEST/FAIL] expected mock LLVM exit=42, got $RC" >&2
  tail -n 40 "$STDOUT_LOG" >&2 || true
  tail -n 40 "$STDERR_LOG" >&2 || true
  exit 1
fi

grep -q '^output_contract=hako-llvm-pipeline-runtime-report-v0$' "$REPORT"
grep -q '^tool_surface=llvm_runner_pipeline_report$' "$REPORT"
grep -q '^observation_only=1$' "$REPORT"
grep -q '^behavior_change=0$' "$REPORT"
grep -q '^mir_future_rewrite_route=env_forced_llvm_future_externs$' "$REPORT"
grep -q '^pipeline_joinir_experiment_enabled=0$' "$REPORT"
grep -q '^method_id_injector_mutation_count=0$' "$REPORT"
grep -q '^execution_backend=mock$' "$REPORT"
grep -q '^llvm_fallback_used=1$' "$REPORT"
grep -q '^llvm_fallback_reason=harness_unavailable_or_not_requested$' "$REPORT"
grep -q '^pyvm_requested=0$' "$REPORT"
grep -q '^harness_requested=0$' "$REPORT"
grep -q '^mock_fallback_used=1$' "$REPORT"
grep -q '^product_activation=0$' "$REPORT"
grep -q '^hook_installed=0$' "$REPORT"
grep -q '^global_allocator_product_claim=0$' "$REPORT"
grep -q '^winner_claim=0$' "$REPORT"
grep -q '^summary=ok$' "$REPORT"

echo "[TEST/OK] llvm_pipeline_runtime_report"
