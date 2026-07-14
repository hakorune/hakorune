#!/usr/bin/env bash

# D′ SSA-P0: behavior-neutral inventory of every canonical value/CFG/PHI/RC/
# publication/Return/old-If seam that must move, remain, isolate, or disappear.

source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/resolved_ownership_legacy_release_contract.sh"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/resolved_ownership_transition_planner_contract.sh"
source "$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)/resolved_trivial_owner_profile_contract.sh"

guard_resolved_binding_ssa_contract() {
  local tag="$1"
  local root="$2"
  local inventory="$root/tools/checks/fixtures/canonical_ssa_seam_inventory_v1.json"
  local validator="$root/tools/checks/lib/resolved_binding_ssa_inventory.py"
  local cfg_validator="$root/tools/checks/lib/resolved_binding_ssa_cfg.py"
  local phi_txn_validator="$root/tools/checks/lib/resolved_binding_ssa_phi_txn.py"
  local publication_validator="$root/tools/checks/lib/resolved_binding_ssa_publication.py"
  local builder_validator="$root/tools/checks/lib/resolved_binding_ssa_builder.py"
  local identity_validator="$root/tools/checks/lib/resolved_binding_ssa_identity.py"
  local mir_adapter_validator="$root/tools/checks/lib/resolved_binding_ssa_mir_adapter.py"
  local ownership_profile="$root/tools/checks/fixtures/canonical_ownership_production_profile_v1.json"
  local ownership_validator="$root/tools/checks/lib/resolved_ownership_production_profile.py"
  local helper="${BASH_SOURCE[0]}"

  guard_require_files \
    "$tag" \
    "$inventory" \
    "$validator" \
    "$cfg_validator" \
    "$phi_txn_validator" \
    "$publication_validator" \
    "$builder_validator" \
    "$identity_validator" \
    "$mir_adapter_validator" \
    "$ownership_profile" \
    "$ownership_validator" \
    "$helper"
  python3 "$validator" "$root" "$inventory"
  python3 "$cfg_validator" "$root"
  python3 "$phi_txn_validator" "$root"
  python3 "$publication_validator" "$root"
  python3 "$builder_validator" "$root"
  python3 "$identity_validator" "$root"
  python3 "$mir_adapter_validator" "$root"
  python3 "$ownership_validator" "$root" "$ownership_profile"
  guard_resolved_ownership_legacy_release_contract "$tag" "$root"
  guard_resolved_ownership_transition_planner_contract "$tag" "$root"
  guard_resolved_trivial_owner_profile_contract "$tag" "$root"

  local file lines
  for file in \
    "$inventory" \
    "$validator" \
    "$cfg_validator" \
    "$phi_txn_validator" \
    "$publication_validator" \
    "$builder_validator" \
    "$identity_validator" \
    "$mir_adapter_validator" \
    "$ownership_profile" \
    "$ownership_validator" \
    "$helper"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "D′ SSA-P0 source/check reached the 800-line stop boundary: $file ($lines)"
    fi
  done
}
