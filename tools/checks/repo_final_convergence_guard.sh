#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="repo-final-convergence-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

MATRIX="$ROOT_DIR/docs/development/current/main/investigations/repo-final-convergence-audit0-g0-disposition.toml"
ROLE_MANIFEST="$ROOT_DIR/docs/development/current/main/design/fixtures/mir-authority-role-manifest-d0-v1.tsv"
JOINMODULE_RECEIPT="$ROOT_DIR/docs/development/current/main/design/fixtures/mir-legacy-joinmodule-disposition-d0-v1.tsv"
CONTEXT_CENSUS="$ROOT_DIR/docs/development/current/main/design/fixtures/mir-context-owner-census-d0-v1.tsv"
GUARD_INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$MATRIX" "$ROLE_MANIFEST" "$JOINMODULE_RECEIPT" "$CONTEXT_CENSUS" "$GUARD_INDEX"

echo "[$TAG] validating G0 disposition matrix"

python3 - "$ROOT_DIR" "$MATRIX" "$ROLE_MANIFEST" "$JOINMODULE_RECEIPT" "$CONTEXT_CENSUS" "$GUARD_INDEX" <<'PY'
import pathlib
import sys
import tomllib

tag = "repo-final-convergence-guard"
root = pathlib.Path(sys.argv[1]).resolve()
matrix_path = pathlib.Path(sys.argv[2]).resolve()
role_manifest_path = pathlib.Path(sys.argv[3]).resolve()
joinmodule_path = pathlib.Path(sys.argv[4]).resolve()
context_census_path = pathlib.Path(sys.argv[5]).resolve()
guard_index_path = pathlib.Path(sys.argv[6]).resolve()


def fail(message: str) -> None:
    print(f"[{tag}] ERROR: {message}", file=sys.stderr)
    sys.exit(1)


REQUIRED_FIELDS = {
    "path",
    "role",
    "owner",
    "lifecycle",
    "caller_class",
    "default_fallback",
    "retire_when",
    "evidence",
    "loop_coverage_parity",
    "legacy_joinmodule_disposition",
}
ROLES = {
    "pipeline-order-authority",
    "historical-supporting",
    "facade",
    "authority-manifest",
    "census-receipt",
    "disposition-receipt",
    "closeout-receipt",
    "live-pointer",
    "pointer-mirror",
    "navigation-index",
    "registry-store",
    "guard",
    "tooling",
    "parity-fixture",
    "module-doc",
    "guard-index",
}
LIFECYCLES = {"durable", "historical", "retired", "quarantined", "active-receipt"}
CALLER_CLASSES = {
    "production",
    "docs",
    "guarded-tools",
    "oracle",
    "test-only",
    "none",
}
LOOP_PARITY = {"n/a", "parity-held", "parity-gate"}
JOINMODULE_DISPOSITIONS = {
    "n/a",
    "receipt-authority",
    "oracle-only",
    "shadow-only",
    "retained-with-caller",
    "retired",
}
TOP_LEVEL_KEYS = {"schema_version", "audit_row", "owner_card", "record"}


def rel_exists(repo_rel: str) -> bool:
    return (root / repo_rel).exists()


def tsv_rows(path: pathlib.Path) -> list[list[str]]:
    rows = []
    for raw in path.read_text(encoding="utf-8").splitlines():
        line = raw.rstrip("\n")
        if not line or line.startswith("#"):
            continue
        rows.append(line.split("\t"))
    return rows


doc = tomllib.loads(matrix_path.read_text(encoding="utf-8"))
unknown_top = set(doc) - TOP_LEVEL_KEYS
if unknown_top:
    fail(f"unknown top-level field(s): {sorted(unknown_top)}")
if doc.get("schema_version") != 1:
    fail("schema_version must be 1")
if doc.get("audit_row") != "REPO-FINAL-CONVERGENCE-AUDIT0-G0":
    fail("audit_row must be REPO-FINAL-CONVERGENCE-AUDIT0-G0")
owner_card = doc.get("owner_card")
if not isinstance(owner_card, str) or not rel_exists(owner_card):
    fail("owner_card must point to an existing repository path")

records = doc.get("record")
if not isinstance(records, list) or not records:
    fail("matrix has no [[record]] rows")

seen: set[str] = set()
authority_records = []
for index, rec in enumerate(records, 1):
    if not isinstance(rec, dict):
        fail(f"record {index} is not a table")
    fields = set(rec)
    missing = REQUIRED_FIELDS - fields
    if missing:
        fail(f"record {index} missing field(s): {sorted(missing)}")
    extra = fields - REQUIRED_FIELDS
    if extra:
        fail(f"record {index} has unknown field(s): {sorted(extra)}")

    path = rec["path"]
    if not isinstance(path, str) or not path or path in seen:
        fail(f"record {index} has duplicate or empty path: {path!r}")
    seen.add(path)
    if not rel_exists(path):
        fail(f"record {index} path does not exist: {path}")

    if rec["role"] not in ROLES:
        fail(f"record {index} unclassified role: {rec['role']!r}")
    if rec["lifecycle"] not in LIFECYCLES:
        fail(f"record {index} unclassified lifecycle: {rec['lifecycle']!r}")
    if rec["caller_class"] not in CALLER_CLASSES:
        fail(f"record {index} unclassified caller_class: {rec['caller_class']!r}")
    if rec["loop_coverage_parity"] not in LOOP_PARITY:
        fail(f"record {index} unclassified loop_coverage_parity: {rec['loop_coverage_parity']!r}")
    if rec["legacy_joinmodule_disposition"] not in JOINMODULE_DISPOSITIONS:
        fail(
            f"record {index} unclassified legacy_joinmodule_disposition: "
            f"{rec['legacy_joinmodule_disposition']!r}"
        )
    if rec["default_fallback"] != "none":
        fail(f"record {index} default_fallback must be 'none': {rec['default_fallback']!r}")
    if not isinstance(rec["owner"], str) or not rec["owner"].strip():
        fail(f"record {index} missing owner")
    if not isinstance(rec["retire_when"], str) or not rec["retire_when"].strip():
        fail(f"record {index} missing retire_when")
    evidence = rec["evidence"]
    if not isinstance(evidence, list) or not evidence:
        fail(f"record {index} missing evidence")
    for ev in evidence:
        if not isinstance(ev, str) or not rel_exists(ev):
            fail(f"record {index} evidence path does not exist: {ev!r}")

    if rec["role"] == "pipeline-order-authority":
        authority_records.append(rec)

# Claim: exactly one pipeline-order authority, at the pinned path.
if len(authority_records) != 1:
    fail(f"expected exactly 1 pipeline-order-authority record, got {len(authority_records)}")
if authority_records[0]["path"] != "docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md":
    fail("pipeline-order-authority record is not mirbuilder-final-pipeline-ssot.md")
if authority_records[0]["lifecycle"] != "durable":
    fail("pipeline-order-authority record must be lifecycle=durable")

# Claim: compiler-pipeline-ssot.md is historical/supporting only.
by_path = {rec["path"]: rec for rec in records}
legacy = by_path.get("docs/development/current/main/design/compiler-pipeline-ssot.md")
if legacy is None or legacy["lifecycle"] != "historical" or legacy["role"] != "historical-supporting":
    fail("compiler-pipeline-ssot.md must be recorded as historical-supporting/historical")

# Claim: src/mir/mod.rs is a durable facade.
facade = by_path.get("src/mir/mod.rs")
if facade is None or facade["role"] != "facade" or facade["lifecycle"] != "durable":
    fail("src/mir/mod.rs must be recorded as a durable facade")

# Claim: current-entry parity set is recorded.
for parity_path, role in (
    ("docs/development/current/main/CURRENT_STATE.toml", "live-pointer"),
    ("CURRENT_TASK.md", "pointer-mirror"),
    ("docs/development/current/main/05-Restart-Quick-Resume.md", "pointer-mirror"),
    ("docs/development/current/main/10-Now.md", "pointer-mirror"),
):
    rec = by_path.get(parity_path)
    if rec is None or rec["role"] != role:
        fail(f"current-entry parity record missing or wrong role: {parity_path}")

# Claim: loop coverage parity and JoinModule disposition are explicit.
if not any(rec["loop_coverage_parity"] == "parity-held" for rec in records):
    fail("no record carries loop_coverage_parity=parity-held")
if not any(rec["loop_coverage_parity"] == "parity-gate" for rec in records):
    fail("no record carries loop_coverage_parity=parity-gate")
if not any(rec["legacy_joinmodule_disposition"] == "receipt-authority" for rec in records):
    fail("no record carries legacy_joinmodule_disposition=receipt-authority")

# Claim: the registry store record exists and is durable.
registry = by_path.get("docs/development/current/main/design/registry/manifest.toml")
if registry is None or registry["role"] != "registry-store" or registry["lifecycle"] != "durable":
    fail("registry/manifest.toml must be recorded as a durable registry-store")

# Cross-check: authority-role manifest — every data row classified.
ROLE_ENUM = {"meaning", "substrate", "host/backend", "oracle", "quarantine"}
role_rows = tsv_rows(role_manifest_path)
if not role_rows:
    fail("authority-role manifest has no data rows")
for lineno, cols in enumerate(role_rows, 1):
    if len(cols) < 4 or cols[1] not in ROLE_ENUM or not cols[2].strip() or not cols[3].strip():
        fail(f"authority-role manifest row {lineno} is unclassified: {cols!r}")

# Cross-check: JoinModule disposition receipt — every row dispositioned.
JM_ENUM = {"retain", "quarantine", "retire"}
jm_rows = tsv_rows(joinmodule_path)
if not jm_rows:
    fail("JoinModule disposition receipt has no data rows")
for lineno, cols in enumerate(jm_rows, 1):
    if (
        len(cols) < 7
        or cols[2] not in JM_ENUM
        or not cols[3].strip()
        or not cols[4].strip()
        or cols[5] != "none"
        or not cols[6].strip()
    ):
        fail(f"JoinModule disposition row {lineno} is unclassified: {cols!r}")

# Cross-check: context census — every row has a named owner/disposition.
ctx_rows = tsv_rows(context_census_path)
if not ctx_rows:
    fail("context owner census has no data rows")
for lineno, cols in enumerate(ctx_rows, 1):
    if len(cols) < 8 or not cols[7].strip() or "unclassified" in cols[7].lower():
        fail(f"context census row {lineno} is unclassified: {cols!r}")

# Claim: this guard is registered in the check-scripts index.
index_text = guard_index_path.read_text(encoding="utf-8")
if "tools/checks/repo_final_convergence_guard.sh" not in index_text:
    fail("repo_final_convergence_guard.sh is not registered in check-scripts-index.md")

print(f"[{tag}] matrix ok: {len(records)} records")
PY

echo "[$TAG] checking current-entry parity via pointer guard"
bash "$ROOT_DIR/tools/checks/current_state_pointer_guard.sh"

echo "[$TAG] ok"
