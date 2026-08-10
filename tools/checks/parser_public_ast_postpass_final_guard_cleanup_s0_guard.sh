#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="parser-public-ast-postpass-final-guard-cleanup-s0"
TASK="$ROOT/docs/development/current/main/investigations/parser-public-ast-postpass-final-guard-cleanup-s0-implementation-task-2026-08-09.md"
STATE="$ROOT/docs/development/current/main/CURRENT_STATE.toml"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

SEAL_MOD="$ROOT/src/parser/source_seal/mod.rs"
SEAL_MODEL="$ROOT/src/parser/source_seal/model.rs"
SEAL_GATE="$ROOT/src/parser/source_seal/gate_projection.rs"
SEAL_FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$TASK" "$STATE" "$INDEX" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" \
  "$ROOT/tools/checks/frontend_parsed_box_source_seal_r6_s3b_b2_guard.sh" \
  "$ROOT/tools/checks/frontend_parsed_box_source_seal_r6_s3b_b3_guard.sh" \
  "$ROOT/tools/checks/frontend_parsed_box_source_seal_r6_s3b_d_i0_guard.sh" \
  "$ROOT/tools/checks/parser_public_ast_postpass_final_d0_guard.sh" \
  "$ROOT/tools/checks/parser_public_ast_postpass_final_noelse_i0_guard.sh" \
  "$ROOT/tools/checks/parser_public_ast_postpass_final_retire_s0_guard.sh" \
  "$ROOT/tools/checks/parser_public_ast_postpass_cutover_d0_guard.sh"

for guard in \
  frontend_parsed_box_source_seal_r6_s3b_b2_guard.sh \
  frontend_parsed_box_source_seal_r6_s3b_b3_guard.sh \
  frontend_parsed_box_source_seal_r6_s3b_d_i0_guard.sh \
  parser_public_ast_postpass_final_d0_guard.sh \
  parser_public_ast_postpass_final_noelse_i0_guard.sh \
  parser_public_ast_postpass_final_retire_s0_guard.sh \
  parser_public_ast_postpass_cutover_d0_guard.sh; do
  bash "$ROOT/tools/checks/$guard"
done

python3 - "$TASK" "$STATE" "$INDEX" "$SEAL_MOD" "$SEAL_MODEL" "$SEAL_GATE" "$SEAL_FINALIZE" \
  "$ROOT/tools/checks/frontend_parsed_box_source_seal_r6_s3b_b2_guard.sh" \
  "$ROOT/tools/checks/frontend_parsed_box_source_seal_r6_s3b_b3_guard.sh" \
  "$ROOT/tools/checks/frontend_parsed_box_source_seal_r6_s3b_d_i0_guard.sh" <<'PY'
import sys
from pathlib import Path

task_path, state_path, index_path = map(Path, sys.argv[1:4])
seal_paths = list(map(Path, sys.argv[4:8]))
b2_path, b3_path, di0_path = map(Path, sys.argv[8:11])
task = task_path.read_text(encoding="utf-8")
state = state_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")
b2 = b2_path.read_text(encoding="utf-8")
b3 = b3_path.read_text(encoding="utf-8")
di0 = di0_path.read_text(encoding="utf-8")

if not any(
    status in task
    for status in ("Status: active bounded implementation", "Status: closed implementation receipt")
):
    raise SystemExit("cleanup task must be active or carry its landed receipt")
active = all(
    needle in state
    for needle in (
        'work_mode = "fast"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-GUARD-CLEANUP-S0"',
        'current_blocker_token = "PARSER-PUBLIC-AST-POSTPASS-FINAL-GUARD-CLEANUP-S0:',
    )
)
closed = all(
    needle in state
    for needle in (
        'work_mode = "design_stop"',
        'current_execution_row = "PARSER-PUBLIC-AST-POSTPASS-FINAL-CLOSEOUT-D0"',
    )
)
if not (active or closed):
    raise SystemExit("CURRENT_STATE missing cleanup active/closeout pointer")
if 'latest_card = "parser-public-ast-postpass-final-guard-cleanup-s0"' not in state:
    raise SystemExit("CURRENT_STATE latest card must retain the cleanup receipt")

for script, label in ((b2, "B2"), (b3, "B3"), (di0, "D-I0")):
    if "source_gate_prune.rs" in script:
        raise SystemExit(f"{label} guard still requires retired source_gate_prune.rs")

for needle in (
    "PARSER-MEMBER-GATE-NESTED-SOURCE-PATH-D0",
    "PARSER-DIRECT-BIRTH-MIGRATION-TRANSPORT-D0",
    "PARSER-LEGACY-WHILE-GRAMMAR-FREEZE-D0",
    "PARSER-LEGACY-FOR-GRAMMAR-FREEZE-D0",
    "PARSER-LEGACY-LOOP-GRAMMAR-FREEZE-D0",
):
    if needle not in task:
        raise SystemExit(f"known baseline red is not classified: {needle}")

if "parser_public_ast_postpass_final_guard_cleanup_s0_guard.sh" not in index:
    raise SystemExit("check index must list the cleanup guard")

for path, limit in (
    *((path, 760) for path in seal_paths),
    ("src/parser/build_cfg/prune.rs", 800),
    ("src/parser/source_seal_finalizer.rs", 800),
    ("src/parser/source_seal_finalizer_tests.rs", 800),
):
    path = path if isinstance(path, Path) else Path.cwd() / path
    if len(path.read_text(encoding="utf-8").splitlines()) >= limit:
        raise SystemExit(f"{path} reached the {limit}-line boundary")

print("active_guard_set=1")
print("retired_helper_absent_from_active_guards=1")
print("baseline_reds_classified=1")
print("source_line_limits=1")
print("summary=ok")
PY

echo "[$TAG] ok"
