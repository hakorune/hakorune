#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

DOC="docs/development/current/main/phases/phase-29cc/29cc-210-plg-hm2-core-wave2-rust-recovery-line-lock-ssot.md"
POLICY_DOC="docs/development/current/main/design/code-retirement-history-policy-ssot.md"
WORKFLOW=".github/workflows/portability-ci.yml"
DEV_GATE="tools/checks/dev_gate.sh"

if [ ! -f "$DOC" ]; then
  echo "[plg-hm2-recovery-guard] missing lock doc: $DOC" >&2
  exit 1
fi

if [ ! -f "$POLICY_DOC" ]; then
  echo "[plg-hm2-recovery-guard] missing retirement policy doc: $POLICY_DOC" >&2
  exit 1
fi

if [ ! -f "$WORKFLOW" ]; then
  echo "[plg-hm2-recovery-guard] missing workflow: $WORKFLOW" >&2
  exit 1
fi

for keyword in \
  "PLG-HM2-min1" \
  "Core+Wave2" \
  "portability-ci.yml" \
  "code-retirement-history-policy-ssot"; do
  if ! rg -F -q "$keyword" "$DOC"; then
    echo "[plg-hm2-recovery-guard] missing keyword in lock doc: $keyword" >&2
    exit 1
  fi
done

for os_name in "ubuntu-latest" "windows-latest" "macos-latest"; do
  if ! rg -q "$os_name" "$WORKFLOW"; then
    echo "[plg-hm2-recovery-guard] missing OS in workflow matrix: $os_name" >&2
    exit 1
  fi
done

for manifest in \
  "plugins/nyash-array-plugin/Cargo.toml" \
  "plugins/nyash-map-plugin/Cargo.toml" \
  "plugins/nyash-string-plugin/Cargo.toml" \
  "plugins/nyash-console-plugin/Cargo.toml" \
  "plugins/nyash-filebox-plugin/Cargo.toml" \
  "plugins/nyash-path-plugin/Cargo.toml" \
  "plugins/nyash-math-plugin/Cargo.toml" \
  "plugins/nyash-net-plugin/Cargo.toml"; do
  if ! rg -q "$manifest" "$WORKFLOW"; then
    echo "[plg-hm2-recovery-guard] missing plugin manifest in workflow: $manifest" >&2
    exit 1
  fi
done

if ! rg -q "plugin-recovery-check" "$WORKFLOW"; then
  echo "[plg-hm2-recovery-guard] missing plugin-recovery-check job in workflow" >&2
  exit 1
fi

if ! rg -q "phase29cc_plg_hm2_rust_recovery_line_guard.sh" "$DEV_GATE"; then
  echo "[plg-hm2-recovery-guard] dev_gate missing HM2 guard wiring" >&2
  exit 1
fi

echo "[plg-hm2-recovery-guard] ok"
