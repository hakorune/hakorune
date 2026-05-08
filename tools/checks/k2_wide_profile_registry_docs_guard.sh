#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-profile-registry-docs"
cd "$ROOT_DIR"

REGISTRY="docs/reference/mir/rune-profile-registry.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
RUNE_PROFILE_SSOT="docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md"
SUBSTRATE_DOC="docs/reference/runtime/substrate-capabilities.md"
CARD="docs/development/current/main/phases/phase-293x/293x-063-M12B-PROFILE-REGISTRY-DOCS.md"

echo "[$TAG] running M12b Profile registry docs guard"

for file in "$REGISTRY" "$TASKBOARD" "$RUNE_PROFILE_SSOT" "$SUBSTRATE_DOC" "$CARD"; do
  if [ ! -f "$file" ]; then
    echo "[$TAG] missing file: $file" >&2
    exit 1
  fi
done

for profile in allocator.fast allocator.slow substrate.leaf intrinsic.leaf raw.layout; do
  rg -F -q "\`$profile\`" "$REGISTRY"
  rg -F -q "\`$profile\`" "$CARD"
done

rg -F -q 'Status: M12c live-narrow profile expansion.' "$REGISTRY"
rg -F -q '`@rune Profile(...)` is accepted for the reserved profile names in this file.' "$REGISTRY"
rg -F -q 'Backends, `.inc`, and ll_emit must never branch on profile names.' "$REGISTRY"
rg -F -q 'Profile-derived `CapabilityPlan allow=[...]` metadata emission' "$REGISTRY"
rg -F -q '| `M12b Profile registry docs` | `live-docs` |' "$TASKBOARD"
rg -F -q 'M12b Profile registry docs [live-docs]' "$RUNE_PROFILE_SSOT"
rg -F -q 'Profile registry SSOT: `docs/reference/mir/rune-profile-registry.md`.' "$SUBSTRATE_DOC"
rg -F -q 'M12b is live-docs only.' "$CARD"

if rg -F -q '"Capability"' src/ast/attrs.rs lang/src/compiler/parser/rune/rune_contract_box.hako; then
  echo "[$TAG] ERROR: Capability parser surface must stay disabled in M12b" >&2
  exit 1
fi

if rg -F -q 'Profile(' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume Profile names" >&2
  exit 1
fi

echo "[$TAG] ok"
