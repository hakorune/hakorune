#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="h2-compile-front-owner-handoff-i0"
LOAN="$ROOT/src/parser/normal_callable_program_source/semantic_syntax_loan.rs"
BATCH="$ROOT/src/mir/callable_semantic_batch/issuer.rs"
BATCH_MODEL="$ROOT/src/mir/callable_semantic_batch/model.rs"
PACKAGE="$ROOT/src/mir/normal_callable_semantic_package/install.rs"
PORT="$ROOT/src/mir/builder/normal_callable_semantic_loan_port.rs"
LOCAL="$ROOT/src/mir/builder/stmts/local_statement_descent.rs"
DIAG="$ROOT/src/mir/builder/generic_loop_admission_observation.rs"
RAW_LOOP="$ROOT/src/mir/builder/raw_loop_child_entry.rs"
RAW_PORT="$ROOT/src/mir/builder/recursive_child_lowering.rs"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$LOAN" "$BATCH" "$BATCH_MODEL" "$PACKAGE" "$PORT" \
  "$LOCAL" "$DIAG" "$RAW_LOOP" "$RAW_PORT"

python3 - "$LOAN" "$BATCH" "$BATCH_MODEL" "$PACKAGE" "$PORT" "$LOCAL" "$DIAG" "$RAW_LOOP" "$RAW_PORT" <<'PY'
import sys
from pathlib import Path

loan, batch, batch_model, package, port, local, diag, raw_loop, raw_port = map(Path, sys.argv[1:])
texts = {path: path.read_text(encoding="utf-8") for path in (loan, batch, batch_model, package, port, local, diag, raw_loop, raw_port)}

for path, text in texts.items():
    lines = len(text.splitlines())
    if lines >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}={lines}")

loan_text = texts[loan]
if loan_text.count("struct CallableMethodSourceObservationV1") != 1:
    raise SystemExit("parser must have one method-source observation carrier")
if loan_text.count("fn issue_method_source_observation(") != 1:
    raise SystemExit("parser must have one method-source observation issuer")
for needle in (
    "CallableDeclarationIdentityV1",
    "ResolverSourceInvocationProvenanceV1",
    "ResolverBoxMethodSourceSiteV1",
    "ResolverSourceInvocationProvenanceV1::from_parser_brand",
    "ResolverBoxMethodSourceSiteV1::new",
):
    if needle not in loan_text:
        raise SystemExit(f"parser observation lost co-sealed field/issuer: {needle}")
if "if !gate_path.is_empty()" not in loan_text:
    raise SystemExit("bounded I0 must reject generated/member-gate methods")

if "syntax.method_source_observation().cloned()" not in texts[batch]:
    raise SystemExit("semantic batch must inherit the parser-issued observation")
if "method_source_observation" not in texts[batch_model]:
    raise SystemExit("resolved batch row lost method-source observation carriage")
if "with_lowering_input_and_method_source" not in texts[package]:
    raise SystemExit("installed package must expose only the batch-backed observation loan")
if "with_callable_method_source_observation" not in texts[port]:
    raise SystemExit("raw callable port must transport the observation exactly once")

local_text = texts[local]
if local_text.count("observe_initializer(") < 4:
    raise SystemExit("local descent must observe all supported initializer paths")
if local_text.count("scoped.complete_exact_demands_v1()?;\n        input.observe_initializer") < 3:
    raise SystemExit("initializer observation must happen after evaluated-value completion")

diag_text = texts[diag]
for needle in ("debug_enabled()", "LocalInitializerObservationV1", "GenericLoopAdmissionObservationV1"):
    if needle not in diag_text:
        raise SystemExit(f"diagnostic seam missing default-off/source-aware contract: {needle}")
for forbidden in ("Verified", "Prepared", "MirBuilder", "ValueId::new", "prepare_generic_loop_carrier"):
    if forbidden in diag_text:
        raise SystemExit(f"diagnostic seam gained semantic/repair authority: {forbidden}")

raw_loop_text = texts[raw_loop]
for needle in ("missing-method-source-observation", "foreign-method-source-observation", "same_as(method.identity())"):
    if needle not in raw_loop_text:
        raise SystemExit(f"loop entry lost fail-fast co-seal validation: {needle}")
raw_port_text = texts[raw_port]
if "issue_for_loop(source)" not in raw_port_text:
    raise SystemExit("raw loop route must issue one first-admission observation")
for forbidden in ("method_name", "inventory_ordinal", "repair_value", "fallback", "retry"):
    if forbidden in raw_port_text:
        raise SystemExit(f"compile-front handoff gained forbidden repair/fallback: {forbidden}")

print("parser_method_source_observation_issuer=1")
print("semantic_batch_package_raw_port_transport=1")
print("initializer_observation_after_value=1")
print("default_off_diagnostic_seam=1")
print("foreign_missing_pairing_fail_fast=1")
print("generic_loop_semantic_change=0")
print("source_files_below_800=1")
print("summary=ok")
PY

echo "[$TAG] ok"
