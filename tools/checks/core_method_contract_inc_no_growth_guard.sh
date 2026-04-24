#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="core-method-contract-inc-no-growth-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ALLOWLIST="$ROOT_DIR/tools/checks/core_method_contract_inc_no_growth_allowlist.tsv"
MANIFEST="$ROOT_DIR/lang/src/runtime/meta/generated/core_method_contract_manifest.json"
C_POLICY="$ROOT_DIR/lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$ALLOWLIST" "$MANIFEST" "$C_POLICY"

echo "[$TAG] checking .inc method-name classifier no-growth baseline"

python3 - "$ROOT_DIR" "$ALLOWLIST" "$MANIFEST" "$C_POLICY" <<'PY'
import collections
import json
import pathlib
import re
import sys

tag = "core-method-contract-inc-no-growth-guard"
root = pathlib.Path(sys.argv[1]).resolve()
allowlist_path = pathlib.Path(sys.argv[2]).resolve()
manifest_path = pathlib.Path(sys.argv[3]).resolve()
c_policy_path = pathlib.Path(sys.argv[4]).resolve()

TARGET_FUNCTIONS = (
    "classify_generic_method_emit_kind",
    "classify_generic_method_set_route",
)
KIND_BY_ARG = {
    "mname": "method",
    "bname": "box",
}
KNOWN_ANCHORS = {
    "core-method-contract",
    "core-box-contract",
    "constructor-compat",
    "runtime-data-compat",
}


def rel(path: pathlib.Path) -> str:
    return path.resolve().relative_to(root).as_posix()


def fail(msg: str) -> None:
    print(f"[{tag}] ERROR: {msg}", file=sys.stderr)
    sys.exit(1)


def extract_body(text: str, name: str) -> str:
    idx = text.find(name)
    if idx < 0:
        fail(f"missing function {name}")
    open_idx = text.find("{", idx)
    if open_idx < 0:
        fail(f"missing function body for {name}")
    depth = 0
    in_string = False
    escape = False
    for pos in range(open_idx, len(text)):
        ch = text[pos]
        if in_string:
            if escape:
                escape = False
            elif ch == "\\":
                escape = True
            elif ch == '"':
                in_string = False
            continue
        if ch == '"':
            in_string = True
            continue
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[open_idx + 1 : pos]
    fail(f"unterminated function body for {name}")


def load_contract_surfaces() -> tuple[set[str], set[str]]:
    data = json.loads(manifest_path.read_text(encoding="utf-8"))
    methods: set[str] = set()
    boxes: set[str] = set()
    for row in data.get("rows", []):
        boxes.add(row["box"])
        methods.add(row["canonical"])
        methods.update(row.get("aliases", []))
    return methods, boxes


def load_allowlist() -> dict[tuple[str, str, str, str], tuple[int, str, str]]:
    out: dict[tuple[str, str, str, str], tuple[int, str, str]] = {}
    for lineno, raw in enumerate(allowlist_path.read_text(encoding="utf-8").splitlines(), 1):
        line = raw.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split("\t")
        if len(parts) != 7:
            fail(f"bad allowlist row {lineno}: expected 7 tab-separated fields")
        path, function, kind, literal, max_count, anchor, deletion_condition = parts
        if function not in TARGET_FUNCTIONS:
            fail(f"bad allowlist row {lineno}: unexpected function {function}")
        if kind not in {"method", "box"}:
            fail(f"bad allowlist row {lineno}: unexpected kind {kind}")
        if anchor not in KNOWN_ANCHORS:
            fail(f"bad allowlist row {lineno}: unexpected manifest_anchor {anchor}")
        if not deletion_condition or deletion_condition == "-":
            fail(f"bad allowlist row {lineno}: deletion_condition is required")
        try:
            limit = int(max_count)
        except ValueError:
            fail(f"bad allowlist row {lineno}: max_count must be integer")
        if limit < 1:
            fail(f"bad allowlist row {lineno}: max_count must be positive")
        key = (path, function, kind, literal)
        if key in out:
            fail(f"duplicate allowlist row {lineno}: {key}")
        out[key] = (limit, anchor, deletion_condition)
    return out


contract_methods, contract_boxes = load_contract_surfaces()
allowlist = load_allowlist()
actual: collections.Counter[tuple[str, str, str, str]] = collections.Counter()
c_policy = c_policy_path.read_text(encoding="utf-8")
c_policy_rel = rel(c_policy_path)
strcmp_arg_first = re.compile(r"!?strcmp\s*\(\s*(mname|bname)\s*,\s*\"([^\"]+)\"\s*\)")
strcmp_literal_first = re.compile(r"!?strcmp\s*\(\s*\"([^\"]+)\"\s*,\s*(mname|bname)\s*\)")

for function in TARGET_FUNCTIONS:
    body = extract_body(c_policy, function)
    for arg_name, literal in strcmp_arg_first.findall(body):
        actual[(c_policy_rel, function, KIND_BY_ARG[arg_name], literal)] += 1
    for literal, arg_name in strcmp_literal_first.findall(body):
        actual[(c_policy_rel, function, KIND_BY_ARG[arg_name], literal)] += 1

failed = False
for key, count in sorted(actual.items()):
    if key not in allowlist:
        print(
            f"[{tag}] ERROR: new untracked .inc string classifier: "
            f"{key[0]} {key[1]} {key[2]} {key[3]} count={count}",
            file=sys.stderr,
        )
        failed = True
        continue
    limit, anchor, _ = allowlist[key]
    if count > limit:
        print(
            f"[{tag}] ERROR: .inc string classifier grew: "
            f"{key[0]} {key[1]} {key[2]} {key[3]} {count}>{limit}",
            file=sys.stderr,
        )
        failed = True
    kind = key[2]
    literal = key[3]
    if anchor == "core-method-contract" and (kind != "method" or literal not in contract_methods):
        print(
            f"[{tag}] ERROR: allowlisted method literal is not in CoreMethodContract manifest: {literal}",
            file=sys.stderr,
        )
        failed = True
    if anchor == "core-box-contract" and (kind != "box" or literal not in contract_boxes):
        print(
            f"[{tag}] ERROR: allowlisted box literal is not in CoreMethodContract manifest: {literal}",
            file=sys.stderr,
        )
        failed = True

removed = sorted(set(allowlist) - set(actual))
reduced = sorted(key for key, (limit, _, _) in allowlist.items() if key in actual and actual[key] < limit)

if removed:
    print(f"[{tag}] NOTE: {len(removed)} allowlist rows no longer have classifiers; prune recommended")
if reduced:
    print(f"[{tag}] NOTE: {len(reduced)} allowlist rows reduced classifier count; prune recommended")

if failed:
    sys.exit(1)

print(f"[{tag}] ok classifiers={sum(actual.values())} rows={len(actual)}")
PY
