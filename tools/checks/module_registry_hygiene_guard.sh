#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
H_ALLOW="$ROOT_DIR/tools/checks/module_registry_hako_top_only_allowlist.txt"
N_ALLOW="$ROOT_DIR/tools/checks/module_registry_nyash_top_only_allowlist.txt"
OV_ALLOW="$ROOT_DIR/tools/checks/module_registry_override_allowlist.txt"

cd "$ROOT_DIR"

echo "[module-registry-hygiene-guard] checking modules.workspace + [modules] hygiene"

for required in "$H_ALLOW" "$N_ALLOW" "$OV_ALLOW" "$ROOT_DIR/hako.toml" "$ROOT_DIR/nyash.toml"; do
  if [[ ! -f "$required" ]]; then
    echo "[module-registry-hygiene-guard] ERROR: required file missing: $required" >&2
    exit 1
  fi
done

if ! command -v python3 >/dev/null 2>&1; then
  echo "[module-registry-hygiene-guard] ERROR: python3 is required" >&2
  exit 2
fi

python3 - "$ROOT_DIR" "$H_ALLOW" "$N_ALLOW" "$OV_ALLOW" <<'PY'
import collections
import pathlib
import sys
import tomllib

root = pathlib.Path(sys.argv[1])
h_allow = pathlib.Path(sys.argv[2])
n_allow = pathlib.Path(sys.argv[3])
ov_allow = pathlib.Path(sys.argv[4])


def load_allowlist(path):
    out = set()
    for raw in path.read_text().splitlines():
        line = raw.strip()
        if not line or line.startswith('#'):
            continue
        out.add(line)
    return out


def flatten_table(prefix, obj, out):
    if not isinstance(obj, dict):
        return
    for key, value in obj.items():
        name = f"{prefix}.{key}" if prefix else key
        if isinstance(value, str):
            out[name] = value
        elif isinstance(value, dict):
            flatten_table(name, value, out)


def normalize_path(path_value, base_dir):
    p = pathlib.Path(path_value)
    if not p.is_absolute():
        p = base_dir / p
    try:
        return str(p.resolve().relative_to(root.resolve()))
    except Exception:
        return str(p.resolve())


def load_workspace_exports(doc, cfg_path):
    workspace_members = (
        ((doc.get("modules") or {}).get("workspace") or {}).get("members") or []
    )
    exports = {}

    for member in workspace_members:
        member_path = pathlib.Path(member)
        if not member_path.is_absolute():
            member_path = cfg_path.parent / member_path
        if not member_path.exists():
            continue
        try:
            module_doc = tomllib.loads(member_path.read_text())
        except Exception:
            continue

        module_name = ((module_doc.get("module") or {}).get("name"))
        exports_tbl = module_doc.get("exports")
        if not module_name or not isinstance(exports_tbl, dict):
            continue

        flat_exports = {}
        flatten_table("", exports_tbl, flat_exports)
        for export_key, rel_path in flat_exports.items():
            full_name = f"{module_name}.{export_key}" if export_key else module_name
            exports[full_name] = normalize_path(rel_path, member_path.parent)

    return exports


def classify(cfg_name: str):
    cfg_path = root / cfg_name
    doc = tomllib.loads(cfg_path.read_text())

    top_modules: dict[str, str] = {}
    flatten_table("", doc.get("modules") or {}, top_modules)
    top_modules.pop("workspace", None)

    workspace = load_workspace_exports(doc, cfg_path)

    duplicates = {k for k, v in top_modules.items() if k in workspace and normalize_path(v, cfg_path.parent) == workspace[k]}
    overrides = {k for k, v in top_modules.items() if k in workspace and normalize_path(v, cfg_path.parent) != workspace[k]}
    top_only = {k for k in top_modules if k not in workspace}
    return duplicates, overrides, top_only


def print_set_delta(label, expected, actual):
    missing = sorted(expected - actual)
    unexpected = sorted(actual - expected)
    if missing:
        print(f"[module-registry-hygiene-guard] ERROR: {label} missing {len(missing)} entries")
        for item in missing[:8]:
            print(f"  - missing: {item}")
    if unexpected:
        print(f"[module-registry-hygiene-guard] ERROR: {label} unexpected {len(unexpected)} entries")
        for item in unexpected[:8]:
            print(f"  - unexpected: {item}")


def check_top_only_growth(cfg_name: str, baseline: set[str], actual: set[str]):
    added = sorted(actual - baseline)
    removed = sorted(baseline - actual)
    if added:
        print(
            f"[module-registry-hygiene-guard] ERROR: {cfg_name} has {len(added)} new top-only aliases "
            "(direct [modules] addition is forbidden; add exports in */hako_module.toml)"
        )
        for item in added[:8]:
            print(f"  - added: {item}")
        return False
    if removed:
        print(
            f"[module-registry-hygiene-guard] NOTE: {cfg_name} top-only alias removed {len(removed)} entries; "
            "allowlist prune recommended"
        )
    return True


expected_h_top_only = load_allowlist(h_allow)
expected_n_top_only = load_allowlist(n_allow)
expected_overrides = load_allowlist(ov_allow)

h_dup, h_ov, h_top = classify("hako.toml")
n_dup, n_ov, n_top = classify("nyash.toml")

failed = False

if h_dup:
    failed = True
    print(f"[module-registry-hygiene-guard] ERROR: hako.toml has exact duplicates: {len(h_dup)}")
    for key in sorted(h_dup)[:8]:
        print(f"  - {key}")
if n_dup:
    failed = True
    print(f"[module-registry-hygiene-guard] ERROR: nyash.toml has exact duplicates: {len(n_dup)}")
    for key in sorted(n_dup)[:8]:
        print(f"  - {key}")

if h_ov != expected_overrides:
    failed = True
    print_set_delta("hako.toml override set", expected_overrides, h_ov)
if n_ov != expected_overrides:
    failed = True
    print_set_delta("nyash.toml override set", expected_overrides, n_ov)

if not check_top_only_growth("hako.toml", expected_h_top_only, h_top):
    failed = True
if not check_top_only_growth("nyash.toml", expected_n_top_only, n_top):
    failed = True

if failed:
    sys.exit(1)


def prefix_counts(keys):
    c = collections.Counter(k.split(".")[0] for k in keys)
    parts = [f"{prefix}={count}" for prefix, count in sorted(c.items(), key=lambda kv: (-kv[1], kv[0]))]
    return ", ".join(parts)

print(
    f"[module-registry-hygiene-guard] hako.toml: top-only={len(h_top)} override={len(h_ov)} prefix=({prefix_counts(h_top)})"
)
print(
    f"[module-registry-hygiene-guard] nyash.toml: top-only={len(n_top)} override={len(n_ov)} prefix=({prefix_counts(n_top)})"
)
print("[module-registry-hygiene-guard] ok")
PY
