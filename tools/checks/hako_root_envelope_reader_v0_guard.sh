#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-root-envelope-reader-v0"
READER="$ROOT/lang/src/compiler/analysis/bounded_body_snapshot/reader_root_v0.hako"
FIXTURE="$ROOT/tools/checks/fixtures/bounded_body_snapshot_root_reader_v0.hako"
SESSION="$ROOT/src/backend/mir_interpreter/strict_json_session.rs"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune
cargo test -q --features vm-reference --lib \
  backend::mir_interpreter::strict_json_session::tests::hako_root_reader_
cargo test -q --features vm-reference --lib \
  backend::mir_interpreter::strict_json_session::tests::duplicate_key_fails_before_hako_root_reader_effects
timeout 10s env NYASH_DISABLE_PLUGINS=1 NYASH_MIR_COMPILE_TRACE=1 \
  target/release/hakorune --dump-mir --no-optimize "$FIXTURE" \
  >/dev/null 2>/tmp/hako-root-envelope-reader-v0.timing.log
grep -q 'stage=build_module' /tmp/hako-root-envelope-reader-v0.timing.log
grep -q 'stage=semantic_refresh' /tmp/hako-root-envelope-reader-v0.timing.log

python3 - "$READER" "$FIXTURE" "$SESSION" <<'PY'
import sys
from pathlib import Path

reader_path, fixture_path, session_path = map(Path, sys.argv[1:])
reader = reader_path.read_text(encoding="utf-8")
fixture = fixture_path.read_text(encoding="utf-8")
session = session_path.read_text(encoding="utf-8")
for path in (reader_path, fixture_path, session_path):
    if len(path.read_text(encoding="utf-8").splitlines()) > 800:
        raise SystemExit(f"source exceeds 800 lines: {path}")
for needle in (
    "ProgramV0RootEnvelopeV0",
    "read_empty_program(session, root)",
    "object_key_at(session, root, i)",
    "program.version_must_be_zero",
    "program.kind_must_be_program",
    "object.forbidden_unknown_field",
    "reader.statement_family_pending",
    "new BoundedBodyAnalysisSnapshotV0(0, new ArrayBox(), 0)",
):
    if needle not in reader:
        raise SystemExit(f"missing root-reader contract: {needle}")
for forbidden in ("MapBox", "indexOf", "substring(", "ValidatedProgramV0BodyView", "MIRBuilder"):
    if forbidden in reader:
        raise SystemExit(f"forbidden root-reader dependency: {forbidden}")
for needle in ("_wrong_type(expected, actual)", "_kind_label(kind)"):
    if needle not in reader:
        raise SystemExit(f"missing local reader diagnostic: {needle}")
for needle in (
    "execute_function_with_strict_json_session",
    "VMValue::Integer(handle)",
    "VMValue::Integer(root)",
    "hako_root_reader_accepts_empty_program_and_closes_each_session",
    "rust_root_error_code",
):
    if needle not in session:
        raise SystemExit(f"missing invocation/parity contract: {needle}")
if "read(1, 0)" in fixture or "read_empty_program(1, 0)" in fixture:
    raise SystemExit("fixture hardcodes strict JSON session identity")
print("root_envelope_owner=hhako")
print("empty_snapshot_execution=green")
print("root_error_parity=green")
print("session_identity_hardcoded=0")
print("nonempty_statement_reader=pending_unsupported")
print("summary=ok")
PY

echo "[$TAG] ok"
