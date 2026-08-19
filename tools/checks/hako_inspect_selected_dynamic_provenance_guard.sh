#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-inspect-selected-dynamic-provenance-guard"
TEMP_DIR="$(mktemp -d /tmp/hako-inspect-selected-dynamic.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT

for source in \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_checked_callout_lowering.inc" \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_op_dispatch.inc" \
  "$ROOT/lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering.inc"; do
  if [[ "$(wc -l <"$source")" -ge 760 ]]; then
    echo "[$TAG] ERROR: source reached the 760-line split boundary: $source" >&2
    exit 1
  fi
done

cc -I"$ROOT/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/driver" \
  "$ROOT/lang/c-abi/tests/selected_dynamic_lowered_llvm_provenance_driver.c" \
  "$ROOT/lang/c-abi/shims/hako_aot.c" \
  "$ROOT/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT/plugins/nyash-json-plugin/c/yyjson/yyjson.c" -ldl

PYTHONPATH="$ROOT/tools/hako_check" python3 -m unittest \
  tools.hako_check.tests.test_inspect_origin_footprint \
  tools.hako_check.tests.test_inspect_provenance_dispositions \
  tools.hako_check.tests.test_inspect_provenance_model \
  tools.hako_check.tests.test_inspect_selected_dynamic_provenance

CARGO_BUILD_JOBS=4 bash "$ROOT/tools/hako_check.sh" \
  inspect selected-dynamic-provenance --out "$TEMP_DIR/bundle" \
  >"$TEMP_DIR/seal.out"

for reserved in --repo-root --driver; do
  set +e
  bash "$ROOT/tools/hako_check.sh" inspect selected-dynamic-provenance \
    "$reserved" /tmp/foreign --out "$TEMP_DIR/reserved-${reserved#--}" \
    >"$TEMP_DIR/reserved.out" 2>"$TEMP_DIR/reserved.err"
  reserved_status=$?
  set -e
  test "$reserved_status" -eq 2
  test ! -e "$TEMP_DIR/reserved-${reserved#--}"
  grep -q "reserved option: $reserved" "$TEMP_DIR/reserved.err"
done

set +e
"$TEMP_DIR/driver" "$TEMP_DIR/bundle/mir.raw.json" \
  "$TEMP_DIR/failure.o" "$TEMP_DIR/failure.ll" \
  "$TEMP_DIR/failure.tsv" 1 >"$TEMP_DIR/failure.out" 2>"$TEMP_DIR/failure.err" &
FAILURE_PID=$!
wait "$FAILURE_PID"
FAILURE_RC=$?
set -e
if [[ "$FAILURE_RC" -eq 0 || -e "$TEMP_DIR/failure.o" ||
      -e "$TEMP_DIR/failure.ll" || -e "$TEMP_DIR/failure.tsv" ||
      -e "/tmp/hako_pure_gen_${FAILURE_PID}.ll" ]]; then
  echo "[$TAG] ERROR: failed journal publication leaked an artifact" >&2
  exit 1
fi

python3 - "$TEMP_DIR/bundle" <<'PY'
import json
import hashlib
import pathlib
import sys

bundle = pathlib.Path(sys.argv[1])
identity = json.loads((bundle / "identity.json").read_text())
provenance = json.loads((bundle / "lowering.provenance.json").read_text())
footprint = json.loads((bundle / "origin-footprint.json").read_text())
payloads = {
    "producer.json", "source.full.hako", "mir.raw.json",
    "llvm.lowered-pre-opt.ir", "lowering.origins.tsv",
    "lowering.provenance.json", "object.bin", "asm.s",
    "origin-footprint.json", "summary.md",
}
assert set(identity["artifacts"]) == payloads
assert {path.name for path in bundle.iterdir()} == payloads | {"identity.json"}
for name, row in identity["artifacts"].items():
    assert hashlib.sha256((bundle / name).read_bytes()).hexdigest() == row["sha256"]
assert identity["mappings"] == {
    "source_to_mir": "exact",
    "mir_to_llvm": "issuer_exact_lowered_pre_opt",
    "lowered_llvm_to_final_llvm": "unavailable",
    "llvm_to_asm": "unavailable",
}
assert identity["shape_ready"] is False
assert provenance["candidate_input"]["issuer"] == "selected_dynamic_c1_lowerer"
assert provenance["candidate_input"]["llvm_boundary"] == "lowered_pre_opt"
assert provenance["coverage"] == {
    "mir_blocks": 10,
    "mir_edges": 10,
    "llvm_blocks": 32,
    "llvm_edges": 32,
}
assert len(provenance["relations"]) == 64
assert footprint["llvm_boundary"] == "lowered_pre_opt"
assert footprint["mir_llvm_correspondence"] == "issuer_exact"
assert footprint["lowered_llvm_to_machine"] == "unavailable"
assert footprint["asm"]["symbol"] == "ParserScanLoopBox.skip_while/4"
assert footprint["asm"]["origin_attribution"] == "unavailable"
assert footprint["asm"]["shape"]["instructions"] > 0
summary = (bundle / "summary.md").read_text()
assert "MIR → lowered LLVM: issuer_exact" in summary
assert "lowered LLVM → final LLVM: unavailable" in summary
assert "LLVM → ASM: unavailable" in summary
PY

echo "[$TAG] ok"
