#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-212-plg-hm2-min3-route-policy-matrix-lock-ssot.md"
ENV_FLAGS="src/config/env/box_factory_flags.rs"
ENV_CATALOG="src/config/env/catalog.rs"
REGISTRY="src/box_factory/registry.rs"
PLUGIN_FACTORY="src/box_factory/plugin.rs"
DEV_GATE="tools/checks/dev_gate.sh"

if [ ! -f "$DOC" ]; then
  echo "[plg-hm2-min3-guard] missing lock doc: $DOC" >&2
  exit 1
fi

for file in "$ENV_FLAGS" "$ENV_CATALOG" "$REGISTRY" "$PLUGIN_FACTORY" "$DEV_GATE"; do
  if [ ! -f "$file" ]; then
    echo "[plg-hm2-min3-guard] missing file: $file" >&2
    exit 1
  fi
done

for keyword in \
  "PLG-HM2-min3" \
  "Route Policy Matrix" \
  "module_first" \
  "dynamic_only" \
  "dynamic_first" \
  "strict_plugin_first" \
  "compat_plugin_first" \
  "plugin-module-core8"; do
  if ! rg -F -q "$keyword" "$DOC"; then
    echo "[plg-hm2-min3-guard] missing keyword in lock doc: $keyword" >&2
    exit 1
  fi
done

for policy in "strict_plugin_first" "compat_plugin_first" "builtin_first"; do
  if ! rg -q "\"$policy\"" "$REGISTRY" "$ENV_CATALOG"; then
    echo "[plg-hm2-min3-guard] missing factory policy contract: $policy" >&2
    exit 1
  fi
done

for mode in "module_first" "dynamic_only" "dynamic_first"; do
  if ! rg -q "\"$mode\"" "$ENV_FLAGS" "$ENV_CATALOG"; then
    echo "[plg-hm2-min3-guard] missing exec mode contract: $mode" >&2
    exit 1
  fi
done

if ! rg -q "PluginExecMode::ModuleFirst" "$PLUGIN_FACTORY"; then
  echo "[plg-hm2-min3-guard] plugin factory missing module_first routing branch" >&2
  exit 1
fi

if ! rg -q "phase29cc_plg_hm2_min3_route_policy_matrix_guard.sh" "$DEV_GATE"; then
  echo "[plg-hm2-min3-guard] dev_gate missing HM2-min3 guard wiring" >&2
  exit 1
fi

echo "[plg-hm2-min3-guard] ok"
