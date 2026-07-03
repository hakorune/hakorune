#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")"/../../../.. && pwd)
source "$root/tools/test/golden/macro/lib/resolve_hakorune.sh"
bin="$(resolve_hakorune_golden_bin "$root")"
src="apps/tests/macro/collections/map_multi.hako"
golden="$root/tools/test/golden/macro/map_multi.expanded.json"


export NYASH_MACRO_BOX_NY=1
export NYASH_MACRO_BOX_CHILD_RUNNER=0
export NYASH_MACRO_BOX_NY_PATHS="apps/macros/examples/map_insert_tag_macro.hako"
export NYASH_ENABLE_MAP_LITERAL=1
export NYASH_MACRO_BOX=1

out=$("$bin" --dump-expanded-ast-json "$src")

norm() { tr -d '\n\r\t ' <<< "$1"; }
if [ "$(norm "$out")" != "$(norm "$(cat "$golden")")" ]; then
  echo "Golden mismatch (user macro map_multi)" >&2
  diff -u <(echo "$out") "$golden" || true
  exit 2
fi

echo "[OK] golden user macro map_multi matched"
