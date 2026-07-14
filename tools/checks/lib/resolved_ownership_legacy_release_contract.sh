#!/usr/bin/env bash

# Private RET-P0 helper. The public authority guard remains a bounded facade.

guard_resolved_ownership_legacy_release_contract() {
  local tag="$1"
  local root="$2"
  local inventory="$root/tools/checks/fixtures/legacy_release_strong_inventory_v1.json"
  local validator="$root/tools/checks/lib/resolved_ownership_legacy_release_inventory.py"
  local helper="${BASH_SOURCE[0]}"

  guard_require_files "$tag" "$inventory" "$validator" "$helper"
  python3 "$validator" "$root" "$inventory"

  local file lines
  for file in "$inventory" "$validator" "$helper"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "D′ RET-P0 source/check reached the 800-line stop boundary: $file ($lines)"
    fi
  done
}
