#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-d1b-cataloged-affine-loan-lifecycle"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml"
MANIFEST="$ROOT_DIR/tools/checks/guard_rows.toml"
MIR_ROOT="$ROOT_DIR/src/mir"

fail() {
  echo "[$TAG] $*" >&2
  exit 1
}

[[ $# -le 1 ]] || fail "usage: $0 [readiness|cataloged_i0]"
PHASE="${1:-readiness}"
case "$PHASE" in
  readiness|cataloged_i0) ;;
  *) fail "unknown phase: $PHASE" ;;
esac

for file in "$CARD" "$MANIFEST" "$MIR_ROOT"; do
  [[ -e "$file" ]] || fail "missing owner ${file#$ROOT_DIR/}"
done

python3 - "$ROOT_DIR" "$CARD" "$MANIFEST" "$MIR_ROOT" "$PHASE" <<'PY'
from pathlib import Path
import sys
import tomllib

root, card_path, manifest_path, mir_root = map(Path, sys.argv[1:5])
phase = sys.argv[5]

with card_path.open("rb") as stream:
    card = tomllib.load(stream)
with manifest_path.open("rb") as stream:
    manifest = tomllib.load(stream)

guard_id = "mir-call-d1b-cataloged-affine-loan-lifecycle"
guard_script = "tools/checks/mir_call_d1b_cataloged_affine_loan_lifecycle_guard.sh"
rows = manifest.get("rows")
if not isinstance(rows, list):
    raise SystemExit("guard_rows.toml rows table is missing")
matches = [row for row in rows if row.get("id") == guard_id]
if len(matches) != 1:
    raise SystemExit(f"expected one registry row for {guard_id}, found {len(matches)}")
row = matches[0]
if row.get("profiles") != ["pilot", "quick-static"]:
    raise SystemExit("lifecycle guard profiles drifted")
if row.get("cmd") != ["bash", guard_script]:
    raise SystemExit("lifecycle guard command drifted")
if sum(1 for item in rows if item.get("id") == guard_id) != 1:
    raise SystemExit("lifecycle guard id is duplicated")

registration = card.get("guard_registration_row")
if not isinstance(registration, dict):
    raise SystemExit("active card guard_registration_row is missing")
if registration.get("execution_row") != "MIR-CALL-D1B-D0-SIG-CLOSE-E-GUARD-REGISTRATION":
    raise SystemExit("guard-only execution row drifted")
if registration.get("status") != "selected_fast_guard_only":
    raise SystemExit("guard-only status drifted")
allowed_files = registration.get("allowed_files")
expected_files = {
    guard_script,
    "tools/checks/guard_rows.toml",
    "docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml",
    "docs/development/current/main/CURRENT_STATE.toml",
}
if set(allowed_files or []) != expected_files:
    raise SystemExit("guard-only allowed file boundary drifted")

if card.get("implementation_permission") is not False:
    raise SystemExit("semantic implementation permission opened during guard-only row")
readiness_audit = card.get("d0_sig_close_f_readiness_audit_2026_08_26")
if not isinstance(readiness_audit, dict):
    raise SystemExit("active card readiness audit is missing")
spec = readiness_audit.get("guard_spec", "")
if guard_script not in spec or "Unknown phase fails closed" not in spec:
    raise SystemExit("active card does not name the fail-closed lifecycle guard")

if phase == "readiness":
    # This phase freezes the pre-implementation boundary.  It must fail if a
    # partial semantic surface appears before the dedicated implementation row.
    forbidden_semantic_symbols = (
        "BuilderInstallConsumerV1",
        "BuilderInstallTokenV1",
        "BuilderPrivateInstalledCallableBundleV1",
        "RawDirectCallDispositionLoanV1",
        "RawDirectCallDispositionPortV1",
        "with_normal_callable_install_once",
    )
    source_files = [path for path in mir_root.rglob("*.rs")]
    source_text = "\n".join(path.read_text() for path in source_files)
    for token in forbidden_semantic_symbols:
        if token in source_text:
            raise SystemExit(f"semantic surface appeared during guard-only row: {token}")

    lifecycle = (mir_root / "builder/normal_default_root_catalog_lifecycle.rs").read_text()
    lowering = (mir_root / "builder/program_root_lowering.rs").read_text()
    post_install = (mir_root / "builder/normal_default_root_catalog_post_install.rs").read_text()
    for token in ("prepare_install", ".commit()", "source_ast()", "with_bound_source"):
        if token not in lifecycle:
            raise SystemExit(f"readiness caller census edge disappeared: {token}")
    for token in ("NormalCallableSemanticPackageMode::Installed", "begin_lowering"):
        if token not in lowering:
            raise SystemExit(f"readiness lowering edge disappeared: {token}")
    if "NormalCallableSemanticPackageMode" not in post_install:
        raise SystemExit("readiness post-install relay disappeared")

    if "RawDirectCallDispositionLoanV1" not in readiness_audit.get("raw_capability_decision", ""):
        raise SystemExit("active card no longer names the raw loan boundary")

elif phase == "cataloged_i0":
    # This phase is intentionally future-facing.  Running it before the
    # Cataloged implementation lands must fail closed rather than silently
    # treating readiness evidence as implementation evidence.
    source_files = [path for path in mir_root.rglob("*.rs")]
    source_text = "\n".join(path.read_text() for path in source_files)
    required = (
        "RawDirectCallDispositionLoanV1",
        "take_once",
        "residual",
    )
    for token in required:
        if token not in source_text:
            raise SystemExit(f"cataloged_i0 phase is not landed: missing {token}")
    if "PreparedRawOrdinaryFunctionCompletionV1::Targeted" in source_text:
        raise SystemExit("generic Targeted payload remains")
    if "PreparedRawOrdinaryFunctionCompletionV1::CatalogedTargeted" in source_text:
        raise SystemExit("Cataloged direct payload remains")
    if source_text.count("MirInstruction::call(") < 1:
        raise SystemExit("canonical Call issuer is missing")
    for token in ("resolve_call_target", "try_unique_static_method_recovery", "make_name_const_result"):
        if token in source_text:
            raise SystemExit(f"late recovery/name-Const edge remains: {token}")

print(f"[{guard_id}] phase={phase} ok")
PY
