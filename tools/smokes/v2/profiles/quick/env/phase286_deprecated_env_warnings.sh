#!/usr/bin/env bash
# Phase 286A: Explicit setting warns for live compatibility flags.

set -euo pipefail

script_dir=$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)
project_root=$(cd "$script_dir/../../../../../.." && pwd)
hakorune="${NYASH_BIN:-$project_root/target/release/hakorune}"
tmp_dir=$(mktemp -d)
trap 'rm -rf "$tmp_dir"' EXIT
printf 'static box Main { main() { return 0 } }\n' > "$tmp_dir/source.hako"

run_expansion() {
    NYASH_CLI_VERBOSE=1 NYASH_RING0_LOG_LEVEL=DEBUG "$hakorune" --dump-expanded-ast-json "$tmp_dir/source.hako"
}

unset NYASH_MACRO_TOPLEVEL_ALLOW NYASH_MACRO_BOX_CHILD_RUNNER
run_expansion > "$tmp_dir/default.out" 2> "$tmp_dir/default.err"
NYASH_MACRO_TOPLEVEL_ALLOW=1 run_expansion > "$tmp_dir/retired.out" 2> "$tmp_dir/retired.err"
cmp "$tmp_dir/default.out" "$tmp_dir/retired.out"
cmp "$tmp_dir/default.err" "$tmp_dir/retired.err"

if "$hakorune" --macro-top-level-allow --dump-expanded-ast-json "$tmp_dir/source.hako" > "$tmp_dir/flag.out" 2> "$tmp_dir/flag.err"; then
    echo "FAIL: removed --macro-top-level-allow flag was accepted" >&2
    exit 1
fi
rg -q -- 'unexpected argument.*--macro-top-level-allow' "$tmp_dir/flag.err"

NYASH_MACRO_BOX_CHILD_RUNNER=1 run_expansion > "$tmp_dir/live.out" 2> "$tmp_dir/live.err"
rg -q '\[macro\]\[compat\] NYASH_MACRO_BOX_CHILD_RUNNER' "$tmp_dir/live.err" "$tmp_dir/live.out"
echo "PASS: macro compatibility flag retirement and warning"
