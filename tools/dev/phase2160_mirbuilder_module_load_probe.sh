#!/bin/bash
# phase2160_mirbuilder_module_load_probe.sh
# Pin that MirBuilder compare-family modules no longer hang during module load.

set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
BIN="$ROOT/target/release/hakorune"

if [ ! -x "$BIN" ]; then
  echo "[FAIL] phase2160_mirbuilder_module_load_probe: missing executable: $BIN" >&2
  exit 2
fi

run_probe() {
  local module="$1"
  local label="$2"
  local tmp_hako
  tmp_hako="$(mktemp --suffix .phase2160.module_load_probe.hako)"
  cat >"$tmp_hako" <<EOF
using "$module" as ProbeBox

static box Main {
  method main(args) {
    print("[$label]")
    return 0
  }
}
EOF

  set +e
  local out
  out="$(timeout 8 env \
    NYASH_DISABLE_PLUGINS=1 \
    NYASH_NYRT_SILENT_RESULT=1 \
    NYASH_FEATURES=stage3 \
    NYASH_ENABLE_USING=1 \
    HAKO_ENABLE_USING=1 \
    NYASH_ALLOW_USING_FILE=1 \
    HAKO_ALLOW_USING_FILE=1 \
    NYASH_USING_AST=1 \
    "$BIN" --backend vm "$tmp_hako" 2>&1)"
  local rc=$?
  set -e
  rm -f "$tmp_hako"

  if [ "$rc" -eq 124 ]; then
    printf '%s\n' "$out" | tail -n 40 || true
    echo "[FAIL] phase2160_mirbuilder_module_load_probe: timeout on $module" >&2
    exit 1
  fi
  if [ "$rc" -ne 0 ]; then
    printf '%s\n' "$out" | tail -n 40 || true
    echo "[FAIL] phase2160_mirbuilder_module_load_probe: rc=$rc on $module" >&2
    exit 1
  fi
  if ! printf '%s\n' "$out" | grep -q "\[$label\]"; then
    printf '%s\n' "$out" | tail -n 40 || true
    echo "[FAIL] phase2160_mirbuilder_module_load_probe: marker missing for $module" >&2
    exit 1
  fi
}

run_probe "hako.mir.builder.internal.lower_if_compare" "probe:if-compare"
run_probe "hako.mir.builder.min" "probe:builder-min"
run_probe "hako.mir.builder" "probe:builder-full"

echo "[PASS] phase2160_mirbuilder_module_load_probe"
