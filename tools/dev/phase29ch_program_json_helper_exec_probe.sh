#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
source "${ROOT}/tools/selfhost/lib/identity_routes.sh"
source "${ROOT}/tools/selfhost/lib/stage1_contract.sh"
STAGE1_BIN="${STAGE1_BIN:-target/selfhost/hakorune.stage1_cli}"
STAGE2_BIN="${STAGE2_BIN:-target/selfhost/hakorune.stage1_cli.stage2}"
ENTRY="${ENTRY:-apps/tests/hello_simple_llvm.hako}"

for bin in "$STAGE1_BIN" "$STAGE2_BIN"; do
  if [[ ! -x "$bin" ]]; then
    echo "[FAIL] missing selfhost bin: $bin" >&2
    exit 2
  fi
done

if [[ ! -f "$ENTRY" ]]; then
  echo "[FAIL] missing entry: $ENTRY" >&2
  exit 2
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

program_json="$tmp_dir/program.json"
helper_src="$tmp_dir/phase29ch_program_json_helper_exec_probe.hako"

if ! run_stage1_env_route "$STAGE1_BIN" "program-json" "$ENTRY" "$program_json"; then
  echo "[FAIL] failed to materialize Program(JSON) via env route" >&2
  exit 1
fi

python3 - "$program_json" "$helper_src" <<'PY'
import json
import pathlib
import sys

program_json_path = pathlib.Path(sys.argv[1])
helper_path = pathlib.Path(sys.argv[2])
program_json_text = program_json_path.read_text(encoding="utf-8")
program_literal = json.dumps(program_json_text, ensure_ascii=False)

helper_path.write_text(
    "using lang.mir.builder.MirBuilderBox as MirBuilderBox\n\n"
    "static box Main {\n"
    "  main() {\n"
    f"    local program_json_text = {program_literal}\n"
    "    local mir = MirBuilderBox.emit_from_program_json_v0(program_json_text, null)\n"
    "    print(\"\" + mir)\n"
    "    return 0\n"
    "  }\n"
    "}\n",
    encoding="utf-8",
)
PY

probe_raw_exec() {
  local bin="$1"
  local label="$2"
  local stdout_file="$tmp_dir/${label}.stdout"
  local stderr_file="$tmp_dir/${label}.stderr"
  set +e
  NYASH_NYRT_SILENT_RESULT=1 \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_FILEBOX_MODE=core-ro \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_MIR_BUILDER_DELEGATE=0 \
  "$bin" "$helper_src" >"$stdout_file" 2>"$stderr_file"
  local rc=$?
  set -e
  echo "[program-json-helper-exec] ${label}.raw_exec_rc=${rc}"
  if [[ "$rc" -ne 97 ]]; then
    echo "[FAIL] expected raw helper execution rc=97 for ${label}" >&2
    sed -n '1,40p' "$stderr_file" >&2 || true
    sed -n '1,40p' "$stdout_file" >&2 || true
    exit 1
  fi
}

probe_raw_exec "$STAGE1_BIN" "stage1"
probe_raw_exec "$STAGE2_BIN" "stage2"

echo "[program-json-helper-exec] result=PASS"
