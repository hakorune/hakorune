#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

OUT="/tmp/hako_field_initializer_route_probe.out"
EXPECTED="/tmp/hako_field_initializer_route_probe.expected"

echo "[field-initializer-route/smoke] EXE"
rm -f "$OUT" "$EXPECTED"
: >"$OUT"

run_case() {
  local name="$1"
  local label="$2"
  local source="apps/field_initializer_route_probe/${name}.hako"
  local exe="/tmp/hako_field_initializer_route_probe_${name}"
  local raw="${exe}.out.raw"

  rm -f "$exe" "$raw" "${exe}.err" "${exe}.build.log"
  rm -f tmp/nyash_cli_emit.json
  if ! ./target/release/hakorune --emit-exe "$exe" "$source" >"${exe}.build.log" 2>&1; then
    if rg -q "unsupported pure shape for current backend recipe" "${exe}.build.log"; then
      echo "${label}=unsupported_pure_shape" >>"$OUT"
      return
    fi
    cat "${exe}.build.log" >&2
    return 1
  fi
  "$exe" >"$raw" 2>"${exe}.err"
  sed '/^Result: /d' "$raw" >>"$OUT"
}

run_case same_file_default same_file_direct_default
run_case same_file_factory same_file_factory_default
run_case same_file_birth same_file_birth
run_case imported_default imported_factory_default
run_case imported_birth imported_factory_birth
run_case imported_ordered_like imported_factory_ordered_like
echo "summary=observed" >>"$OUT"

cat >"$EXPECTED" <<'EOF_EXPECTED'
same_file_direct_default=ok
same_file_factory_default=ok
same_file_birth=ok
imported_factory_default=unsupported_pure_shape
imported_factory_birth=unsupported_pure_shape
imported_factory_ordered_like=unsupported_pure_shape
summary=observed
EOF_EXPECTED

diff -u "$EXPECTED" "$OUT"
echo "[field-initializer-route/smoke] summary=ok"
