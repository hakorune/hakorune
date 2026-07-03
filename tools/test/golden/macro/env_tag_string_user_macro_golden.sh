#!/usr/bin/env bash
set -euo pipefail

root=$(cd "$(dirname "$0")"/../../../.. && pwd)
source "$root/tools/test/golden/macro/lib/resolve_hakorune.sh"
bin="$(resolve_hakorune_golden_bin "$root")"
src="apps/tests/macro/strings/env_tag_demo.hako"


export NYASH_MACRO_ENABLE=1
export NYASH_MACRO_PATHS="apps/macros/examples/env_tag_string_macro.hako"
export NYASH_MACRO_CAP_ENV=1

raw=$("$bin" --dump-expanded-ast-json "$src")
out=$(printf '%s' "$raw" | "$bin" --macro-expand-child apps/macros/examples/env_tag_string_macro.hako)
echo "$out" | rg -q '"value":"hello \[ENV\]"' && echo "[OK] env-tag macro applied" && exit 0
echo "[FAIL] env-tag macro did not apply" >&2
echo "$out" >&2
exit 2
