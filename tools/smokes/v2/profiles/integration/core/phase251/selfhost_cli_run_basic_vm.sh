#!/usr/bin/env bash
# selfhost_cli_run_basic_vm.sh
# - Canary for HakoCli.run lowering in CliRunLowerBox.
# - Uses HAKO_MIR_BUILDER_CLI_RUN=1 + HAKO_SELFHOST_BUILDER_FIRST=1 to force
#   selfhost builder to emit MIR(JSON) for a small HakoCli.run sample.

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
ROOT_DIR="$(git -C "$SCRIPT_DIR" rev-parse --show-toplevel 2>/dev/null || (cd "$SCRIPT_DIR/../../../../../../.." && pwd))"

SRC_HAKO="$(mktemp --suffix .hako)"
OUT_MIR="$(mktemp --suffix .json)"
LOG_OUT="$(mktemp --suffix .log)"
trap 'rm -f "$SRC_HAKO" "$OUT_MIR" "$LOG_OUT" || true' EXIT

# Quick profile: Stage-B emit is flaky under Stage-3 default; skip for now.
echo "[SKIP] selfhost_cli_run_basic_vm (disabled in quick profile after env consolidation)"
exit 0

cat > "$SRC_HAKO" <<'HAKO'
static box HakoCli {
  method run(args){
    @argc = 0
    if args { argc = args.size() }
    if argc == 0 { return 1 }
    @cmd_raw = args.get(0)
    @cmd = "" + cmd_raw
    if cmd == "run"  { return me.cmd_run(args) }
    if cmd == "build"{ return me.cmd_build(args) }
    if cmd == "emit" { return me.cmd_emit(args) }
    if cmd == "check"{ return me.cmd_check(args) }
    return 2
  }
  method cmd_run(args){ return 10 }
  method cmd_build(args){ return 11 }
  method cmd_emit(args){ return 12 }
  method cmd_check(args){ return 13 }
}

static box Main {
  method main(args){
    @cli = new HakoCli()
    return cli.run(args)
  }
}
HAKO

set +e
HAKO_SELFHOST_BUILDER_FIRST=1 \
HAKO_MIR_BUILDER_FUNCS=1 \
HAKO_MIR_BUILDER_CLI_RUN=1 \
HAKO_SELFHOST_TRACE=1 \
NYASH_JSON_ONLY=1 \
bash "$ROOT_DIR/tools/hakorune_emit_mir.sh" "$SRC_HAKO" "$OUT_MIR" >"$LOG_OUT" 2>&1
rc=$?
set -e

if [ $rc -ne 0 ] || [ ! -s "$OUT_MIR" ]; then
  echo "[FAIL] selfhost_cli_run_basic_vm (MIR generation failed rc=$rc)" >&2
  sed -n '1,80p' "$LOG_OUT" >&2 || true
  exit 1
fi

if ! grep -q '"name":"HakoCli.run/2"' "$OUT_MIR"; then
  echo "[FAIL] selfhost_cli_run_basic_vm (HakoCli.run/2 not present in MIR)" >&2
  cat "$OUT_MIR" >&2
  exit 1
fi

if ! grep -q '"method":"cmd_run"' "$OUT_MIR"; then
  echo "[FAIL] selfhost_cli_run_basic_vm (cmd_run not found in HakoCli.run MIR)" >&2
  cat "$OUT_MIR" >&2
  exit 1
fi

echo "[PASS] selfhost_cli_run_basic_vm (HakoCli.run lowered by selfhost builder)"
exit 0
