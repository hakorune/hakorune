#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="hako-aot-same-module-object-handle-contract-guard"

source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

FIXTURE="$ROOT_DIR/docs/development/current/main/design/fixtures/rust-lifecycle/hako-aot-same-module-object-handle-contract-v0.json"
HAKO_BIN="$ROOT_DIR/tools/bin/hako"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$FIXTURE" "$HAKO_BIN"

python3 - "$FIXTURE" <<'PY'
import json
import sys
from pathlib import Path

fixture = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
if fixture.get("kind") != "HakoAotSameModuleObjectHandleContractV1":
    raise SystemExit("bad fixture kind")
if fixture.get("token") != "HAKO-AOT-SAME-MODULE-OBJECT-HANDLE-CONTRACT-001":
    raise SystemExit("bad fixture token")

routes = fixture.get("required_routes") or []
expected = {
    "RecipeItemBox._array_or_empty/1": ("DirectAbi", "direct_function_call", "object_handle"),
    "RecipeItemBox.seq/1": ("DirectAbi", "direct_function_call", "map_handle"),
}
for symbol, (tier, emit_kind, return_shape) in expected.items():
    row = next((row for row in routes if row.get("symbol") == symbol), None)
    if row is None:
        raise SystemExit(f"missing route row: {symbol}")
    if row.get("tier") != tier or row.get("emit_kind") != emit_kind:
        raise SystemExit(f"bad route lowering row: {symbol}")
    if row.get("return_shape") != return_shape or row.get("reason") is not None:
        raise SystemExit(f"bad route contract row: {symbol}")

claims = fixture.get("claims") or {}
for key, value in claims.items():
    if value != 0:
        raise SystemExit(f"forbidden claim drift: {key}")
PY

cargo test -q same_module_body_accepts_collection_birth_receiver_only_call --lib
cargo test -q infers_object_handle_from_builtin_newbox_with_unknown_signature --lib
cargo test -q refresh_module_global_call_routes_accepts_unknown_signature_builtin_array_handle_return --lib
cargo build --release -q

TMP_DIR="$(mktemp -d /tmp/hakorune-same-module-object-handle.XXXXXX)"
cleanup() { rm -rf "$TMP_DIR" >/dev/null 2>&1 || true; }
trap cleanup EXIT

APP="$TMP_DIR/recipe_probe.hako"
JSON="$TMP_DIR/recipe_probe.json"

cat >"$APP" <<'HAKO'
using lang.compiler.mirbuilder.recipe.recipe_item_box as RecipeItemBox

static box Main {
  main() {
    local item = RecipeItemBox.seq([])
    return 0
  }
}
HAKO

bash "$HAKO_BIN" --backend mir --emit-mir-json "$JSON" "$APP" >/dev/null

python3 - "$JSON" <<'PY'
import json
import sys
from pathlib import Path

root = json.loads(Path(sys.argv[1]).read_text(encoding="utf-8"))
routes = {}
for fn in root.get("functions", []):
    name = fn.get("name") or (fn.get("signature") or {}).get("name")
    for route in (fn.get("metadata") or {}).get("global_call_routes") or []:
        symbol = route.get("symbol")
        if symbol in {
            "RecipeItemBox._array_or_empty/1",
            "RecipeItemBox.seq/1",
        }:
            routes[symbol] = route

def require(symbol, return_shape, proof):
    route = routes.get(symbol)
    if route is None:
        raise SystemExit(f"missing route: {symbol}")
    if route.get("reason") is not None:
        raise SystemExit(f"route still blocked: {symbol}: {route.get('reason')}")
    if route.get("tier") != "DirectAbi":
        raise SystemExit(f"route not DirectAbi: {symbol}: {route.get('tier')}")
    if route.get("emit_kind") != "direct_function_call":
        raise SystemExit(f"route not direct function call: {symbol}")
    if route.get("return_shape") != return_shape:
        raise SystemExit(f"bad return_shape for {symbol}: {route.get('return_shape')}")
    if route.get("proof") != proof:
        raise SystemExit(f"bad proof for {symbol}: {route.get('proof')}")

require(
    "RecipeItemBox._array_or_empty/1",
    "object_handle",
    "typed_global_call_same_module_object_handle",
)
require(
    "RecipeItemBox.seq/1",
    "map_handle",
    "typed_global_call_mir_schema_map_constructor",
)
PY

cat <<'REPORT'
output_contract=hako-aot-same-module-object-handle-contract-guard-v0
token=HAKO-AOT-SAME-MODULE-OBJECT-HANDLE-CONTRACT-001
recipe_item_array_or_empty_route=DirectAbi
recipe_item_seq_route=DirectAbi
phase_state_parse_aot_call_fixed=0
layer4_recipe_dto_parity_green=0
source_selfhost_claim=0
summary=ok
REPORT
