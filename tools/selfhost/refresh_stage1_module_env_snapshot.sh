#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
SNAPSHOT_PATH="$ROOT_DIR/src/runner/stage1_bridge/embedded_stage1_modules_snapshot.json"

# shellcheck source=tools/smokes/v2/lib/env.sh
source "$ROOT_DIR/tools/smokes/v2/lib/env.sh"

modules_list="$(collect_stageb_modules_list "$ROOT_DIR")"
module_roots_list="$(collect_stageb_module_roots_list "$ROOT_DIR")"

if [[ -z "$modules_list" ]]; then
    echo "[stage1-module-snapshot] ERROR: collect_stageb_modules_list returned empty" >&2
    exit 1
fi

if [[ -z "$module_roots_list" ]]; then
    echo "[stage1-module-snapshot] ERROR: collect_stageb_module_roots_list returned empty" >&2
    exit 1
fi

STAGE1_MODULES_LIST="$modules_list" \
STAGE1_MODULE_ROOTS_LIST="$module_roots_list" \
SNAPSHOT_PATH="$SNAPSHOT_PATH" \
python3 - <<'PY'
import json
import os
from pathlib import Path

modules_list = os.environ["STAGE1_MODULES_LIST"]
module_roots_list = os.environ["STAGE1_MODULE_ROOTS_LIST"]
snapshot_path = Path(os.environ["SNAPSHOT_PATH"])

body = {
    "schema": "stage1_module_env_snapshot/v1",
    "modules_list": modules_list,
    "module_roots_list": module_roots_list,
}
snapshot_path.write_text(json.dumps(body, ensure_ascii=False, indent=2) + "\n")

modules_count = 0 if not modules_list else len(modules_list.split("|||"))
roots_count = 0 if not module_roots_list else len(module_roots_list.split("|||"))

print(f"[stage1-module-snapshot] wrote {snapshot_path}")
print(f"[stage1-module-snapshot] modules={modules_count} roots={roots_count}")
PY
