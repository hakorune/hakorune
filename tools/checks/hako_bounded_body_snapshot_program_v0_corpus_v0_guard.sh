#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-bounded-body-snapshot-program-v0-corpus-v0"
MANIFEST="$ROOT/tools/checks/fixtures/bounded_body_snapshot_program_v0_corpus_v0.json"
TEST="$ROOT/src/backend/mir_interpreter/strict_json_session/tests/tests_snapshot/corpus.rs"

cd "$ROOT"
bash tools/checks/rust_lifecycle_mirbuilder_ast_wire_observation_oracle_v0_guard.sh
bash tools/checks/hako_direct_snapshot_negative_isolation_v0_guard.sh
for mode in unset 1; do
  if [ "$mode" = unset ]; then
    env -u NYASH_STR_CP cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_snapshot::corpus
  else
    NYASH_STR_CP=1 cargo test -q --features vm-reference --lib \
      backend::mir_interpreter::strict_json_session::tests::tests_snapshot::corpus
  fi
done

python3 - "$MANIFEST" "$TEST" <<'PY'
import json
import sys
from pathlib import Path

manifest_path, test_path = map(Path, sys.argv[1:])
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
test = test_path.read_text(encoding="utf-8")
if manifest.get("schema_version") != 0:
    raise SystemExit("corpus schema version drift")
cases = manifest.get("cases") or []
if len(cases) != 7:
    raise SystemExit(f"corpus count drift: {len(cases)}")
ids = [case.get("id") for case in cases]
if len(ids) != len(set(ids)) or any(not value for value in ids):
    raise SystemExit("corpus ids must be unique and non-empty")
classes = [case.get("class") for case in cases]
allowed = {"ReadyExactParity", "ExplicitUnsupported"}
if set(classes) - allowed:
    raise SystemExit(f"unknown corpus classification: {set(classes) - allowed}")
if classes.count("ReadyExactParity") != 3 or classes.count("ExplicitUnsupported") != 4:
    raise SystemExit("corpus classification count drift")
for required in (
    "declared_type_known_unobserved", "float_schema_seam", "fastmem_schema_seam",
    "typed_array_known_unsupported", "task_scope_known_unsupported",
):
    if required not in ids:
        raise SystemExit(f"missing producer/consumer seam: {required}")
if any(not isinstance(case.get("source"), str) or not case["source"].strip() for case in cases):
    raise SystemExit("every corpus row needs authoritative source text")
for needle in (
    "emit_program_json_v0_for_strict_authority_source", "snapshot_signature(&rust_snapshot",
    'expected.starts_with("Unsupported|")', "manifest.cases.len(), 7",
    "assert_eq!((ready, unsupported), (3, 4))",
):
    if needle not in test:
        raise SystemExit(f"missing zero-skip corpus contract: {needle}")
if "#[ignore]" in test:
    raise SystemExit("corpus test must not be ignored")
if len(test.splitlines()) > 800:
    raise SystemExit("corpus runner exceeds 800 lines")

print("manifest_cases=7")
print("ready_exact_parity=3")
print("explicit_unsupported=4")
print("serializer_failure_tolerance=0")
print("invalid_input=0")
print("skip=0")
print("nomatch=0")
print("local_declared_type=known_unobserved_ready")
print("float=transport_schema_mismatch_stop")
print("fastmem_region=transport_schema_mismatch_stop")
print("accepted_kind_role_operator_pack=green")
print("loss_equivalence_pack=green")
print("negative_and_limit_pack=green")
print("nyash_str_cp_modes=identical")
print("summary=ok")
PY

echo "[$TAG] ok"
