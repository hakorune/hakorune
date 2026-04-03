#!/usr/bin/env bash
set -euo pipefail

# stage1_smoke.sh — legacy embedded Stage‑1 bridge smoke
#
# 役割:
#   - rust Stage0 bridge の embedded Stage1 child route を軽く監視する。
#   - current mainline Stage1 proof ではない。
#   - current mainline smoke は `tools/selfhost/stage1_mainline_smoke.sh` を使う。
#   - ここでは embedded temp entry 経由で Stage1Cli.stub を叩き、
#     - emit program-json
#     - emit mir-json
#   の 2 経路が正常に JSON を出力することを確認する軽量スモーク。

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT_DIR}/target/release/hakorune"

set +e
if [[ ! -x "${BIN}" ]]; then
  echo "[stage1-smoke] missing binary: ${BIN}" >&2
  exit 97
fi
set -e

case "${1:-}" in
  ""|"help"|-h|--help)
    cat <<EOF
Usage: tools/stage1_smoke.sh [program-json|mir-json|all]

  note         : legacy embedded bridge smoke (not the daily mainline route)
  program-json : apps/tests/stage1_using_minimal.hako で Program(JSON v0) を確認
  mir-json     : apps/tests/stage1_using_minimal.hako で MIR(JSON) を確認
  run          : apps/tests/stage1_run_min.hako で run(vm) 経路（現状は MIR 出力のみ）を確認
  all          : 3 経路すべて実行（既定）

Current mainline:
  tools/selfhost/stage1_mainline_smoke.sh [--bin <stage1-cli-artifact>] [<source.hako>]
EOF
    exit 0
    ;;
esac

MODE="${1:-all}"

SRC_MIN="apps/tests/phase29bq_hako_mirbuilder_phase1_literal_return_min.hako"
SRC_RUN="apps/tests/stage1_run_min.hako"

run_prog_json() {
  echo "[stage1-smoke] emit program-json: ${SRC_MIN}" >&2
  NYASH_USE_STAGE1_CLI=1 \
  STAGE1_EMIT_PROGRAM_JSON=1 \
  STAGE1_SOURCE="${SRC_MIN}" \
  "${BIN}" -- stage1_stub_test > /tmp/stage1_smoke_program.json
  echo "[stage1-smoke] program-json length=$(wc -c < /tmp/stage1_smoke_program.json)" >&2
}

run_mir_json() {
  echo "[stage1-smoke] emit mir-json: ${SRC_MIN}" >&2
  NYASH_USE_STAGE1_CLI=1 \
  STAGE1_EMIT_MIR_JSON=1 \
  STAGE1_SOURCE="${SRC_MIN}" \
  "${BIN}" -- stage1_stub_test > /tmp/stage1_smoke_mir.json
  echo "[stage1-smoke] mir-json length=$(wc -c < /tmp/stage1_smoke_mir.json)" >&2
}

run_run_vm() {
  echo "[stage1-smoke] run (vm backend): ${SRC_RUN}" >&2
  NYASH_STAGE1_MODE="run" \
  NYASH_STAGE1_INPUT="${SRC_RUN}" \
  NYASH_STAGE1_BACKEND="vm" \
  "${BIN}" -- stage1_stub_test > /tmp/stage1_smoke_run_vm.out
  echo "[stage1-smoke] run(vm) output length=$(wc -c < /tmp/stage1_smoke_run_vm.out)" >&2
}

if [[ "${MODE}" == "program-json" ]]; then
  run_prog_json
elif [[ "${MODE}" == "mir-json" ]]; then
  run_mir_json
elif [[ "${MODE}" == "run" ]]; then
  run_run_vm
else
  run_prog_json
  run_mir_json
  run_run_vm
fi

exit 0
