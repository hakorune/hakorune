#!/usr/bin/env bash

# Private SSA-I0-PROFILE helper. The sealed Rust product is executable in
# focused fixtures while production routing remains disconnected.

guard_resolved_trivial_owner_profile_contract() {
  local tag="$1"
  local root="$2"
  local profile="$root/tools/checks/fixtures/canonical_trivial_owner_profile_v1.json"
  local validator="$root/tools/checks/lib/resolved_trivial_owner_profile.py"
  local owner="$root/src/mir/resolved_value_profile"
  local helper="${BASH_SOURCE[0]}"
  local files=(
    "$profile"
    "$validator"
    "$owner/README.md"
    "$owner/analyzer.rs"
    "$owner/consumption.rs"
    "$owner/coverage.rs"
    "$owner/error.rs"
    "$owner/mod.rs"
    "$owner/operator.rs"
    "$owner/parameter_entry.rs"
    "$owner/parameter_tests.rs"
    "$owner/product.rs"
    "$owner/tests.rs"
    "$helper"
  )

  guard_require_files "$tag" "${files[@]}"
  python3 "$validator" "$root" "$profile"
  cargo test -q --manifest-path "$root/Cargo.toml" --lib \
    mir::resolved_value_profile

  local file lines
  for file in "${files[@]}"; do
    lines="$(wc -l < "$file" | tr -d '[:space:]')"
    if (( lines >= 800 )); then
      guard_fail "$tag" "D′ SSA-I0-PROFILE source/check reached the 800-line stop boundary: $file ($lines)"
    fi
  done
}
