#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-profile-expansion-to-facts"
cd "$ROOT_DIR"

CARD="docs/development/current/main/phases/phase-293x/293x-064-M12C-PROFILE-EXPANSION-TO-FACTS.md"
REGISTRY="docs/reference/mir/rune-profile-registry.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
RUNE_PROFILE_SSOT="docs/development/current/main/design/rune-profile-effect-capability-plan-ssot.md"
SUBSTRATE_DOC="docs/reference/runtime/substrate-capabilities.md"
METADATA_FACTS="docs/reference/mir/metadata-facts-ssot.md"

echo "[$TAG] running M12c Profile expansion to facts guard"

for file in "$CARD" "$REGISTRY" "$TASKBOARD" "$RUNE_PROFILE_SSOT" "$SUBSTRATE_DOC" "$METADATA_FACTS"; do
  if [ ! -f "$file" ]; then
    echo "[$TAG] missing file: $file" >&2
    exit 1
  fi
done

cargo test -q parser_accepts_profile_rune_reserved_name_and_roundtrips_ast_json
cargo test -q parser_rejects_invalid_profile_rune_value
cargo test -q mir_expands_profile_allocator_fast_to_primitive_plan_facts
cargo test -q profile_expands_to_effect_and_capability_plans

rg -F -q 'pub mod rune_profile_registry;' src/lib.rs
rg -F -q 'pub struct RuneProfileExpansion' src/rune_profile_registry.rs
rg -F -q 'SUPPORTED_PROFILE_NAMES_MSG' src/rune_profile_registry.rs
rg -F -q 'allocator.fast|allocator.slow|substrate.leaf|intrinsic.leaf|raw.layout' src/rune_profile_registry.rs
rg -F -q '"Profile"' src/ast/attrs.rs
rg -F -q 'rune_profile_registry::SUPPORTED_PROFILE_NAMES_MSG' src/ast/attrs.rs
rg -F -q 'if name == "Profile" { return 1 }' lang/src/compiler/parser/rune/rune_contract_box.hako
rg -F -q 'Profile(allocator.fast|allocator.slow|substrate.leaf|intrinsic.leaf|raw.layout)' lang/src/compiler/parser/rune/rune_contract_box.hako
rg -F -q 'rune_profile_registry::expansion' src/mir/inline_plan.rs
rg -F -q 'rune_profile_registry::expansion' src/mir/effect_capability_plan.rs
rg -F -q 'source = format!("rune_profile:{}"' src/mir/inline_plan.rs
rg -F -q 'source: "rune_profile".to_string()' src/mir/effect_capability_plan.rs

rg -F -q '| `M12c Profile expansion to facts` | `live-narrow` |' "$TASKBOARD"
rg -F -q 'M12c Profile expansion to facts [live-narrow]' "$RUNE_PROFILE_SSOT"
rg -F -q '`M13 allocator fast-path EXE proof` is live-narrow.' "$RUNE_PROFILE_SSOT"
rg -F -q 'Status: M13 live-narrow allocator fast-path EXE proof.' "$REGISTRY"
rg -F -q 'Profile expansion to primitive facts [live-narrow]' "$SUBSTRATE_DOC"
rg -F -q 'Profile(allocator.fast)' "$METADATA_FACTS"
rg -F -q 'M12c is live-narrow.' "$CARD"

if rg -F -q '"Capability"' src/ast/attrs.rs lang/src/compiler/parser/rune/rune_contract_box.hako; then
  echo "[$TAG] ERROR: Capability parser surface must stay disabled in M12c" >&2
  exit 1
fi

if rg -F -q 'Profile(' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not consume Profile syntax" >&2
  exit 1
fi

if rg -F -q 'allocator.fast' lang/c-abi/shims lang/src/shared/backend -g '*.inc' -g '*.hako' -g '*.rs'; then
  echo "[$TAG] ERROR: backend/.inc must not branch on profile names" >&2
  exit 1
fi

echo "[$TAG] ok"
