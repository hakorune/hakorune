#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$repo_root"

files=(
  src/mir/compiler/canonical_source_identity.rs
  src/mir/compiler/canonical_core_dispatch.rs
  src/runner/reference/normal_file_vm_frontdoor.rs
  src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
)
for file in "${files[@]}"; do
  [[ -f "$file" ]] || { echo "missing source-digest file: $file" >&2; exit 1; }
  lines="$(wc -l < "$file")"
  (( lines < 760 )) || { echo "760-line split trigger exceeded: $file ($lines)" >&2; exit 1; }
done

frontdoor=src/runner/reference/normal_file_vm_frontdoor.rs
issuer_count="$(rg -o 'CanonicalSourceBytesDigestV1::from_utf8_bytes' "$frontdoor" | wc -l)"
[[ "$issuer_count" -eq 1 ]] || {
  echo "read_once must be the sole frontdoor digest issuer (count=$issuer_count)" >&2
  exit 1
}

if rg -n 'read_to_string|read\(' \
  src/mir/compiler/canonical_core_dispatch.rs \
  src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs; then
  echo "downstream source-plan owners must not reread source bytes" >&2
  exit 1
fi

rg -q 'source_digest: CanonicalSourceBytesDigestV1' "$frontdoor"
rg -q 'source_digest,' src/runner/reference/normal_file_vm_frontdoor/source_plan_input.rs
rg -q 'source_digest: CanonicalSourceBytesDigestV1' src/mir/compiler/canonical_core_dispatch.rs

echo "script direct-static canonical source digest guard: PASS"
