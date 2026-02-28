#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-211-plg-hm2-min2-core6-static-wave2-compat-ceiling-lock-ssot.md"
PLUGIN_FACTORY="src/box_factory/plugin.rs"
HM1_SMOKE="tools/smokes/v2/profiles/integration/apps/phase29cc_plg_hm1_contract_tests_vm.sh"
DEV_GATE="tools/checks/dev_gate.sh"

if [ ! -f "$DOC" ]; then
  echo "[plg-hm2-min2-guard] missing lock doc: $DOC" >&2
  exit 1
fi

if [ ! -f "$PLUGIN_FACTORY" ]; then
  echo "[plg-hm2-min2-guard] missing plugin factory source: $PLUGIN_FACTORY" >&2
  exit 1
fi

for keyword in \
  "PLG-HM2-min2" \
  "Core6" \
  "Wave2 compat ceiling" \
  "MathBox/NetClientBox" \
  "plugin-module-core8"; do
  if ! rg -F -q "$keyword" "$DOC"; then
    echo "[plg-hm2-min2-guard] missing keyword in lock doc: $keyword" >&2
    exit 1
  fi
done

for core_box in \
  '"ArrayBox"' \
  '"StringBox"' \
  '"MapBox"' \
  '"ConsoleBox"' \
  '"FileBox"' \
  '"PathBox"'; do
  if ! rg -q "$core_box" "$PLUGIN_FACTORY"; then
    echo "[plg-hm2-min2-guard] missing Core6 box in plugin factory: $core_box" >&2
    exit 1
  fi
done

for compat_box in '"MathBox"' '"NetClientBox"'; do
  if ! rg -q "$compat_box" "$PLUGIN_FACTORY"; then
    echo "[plg-hm2-min2-guard] missing Wave2 compat marker in plugin factory/tests: $compat_box" >&2
    exit 1
  fi
done

for test_name in \
  "should_skip_dynamic_route_core4_contract" \
  "should_skip_dynamic_route_file_path_contract" \
  "should_keep_dynamic_route_math_net_compat_contract"; do
  if ! rg -q "$test_name" "$PLUGIN_FACTORY" "$HM1_SMOKE"; then
    echo "[plg-hm2-min2-guard] missing route matrix contract test wiring: $test_name" >&2
    exit 1
  fi
done

if ! rg -q "phase29cc_plg_hm2_min2_core6_wave2_ceiling_guard.sh" "$DEV_GATE"; then
  echo "[plg-hm2-min2-guard] dev_gate missing HM2-min2 guard wiring" >&2
  exit 1
fi

echo "[plg-hm2-min2-guard] ok"
