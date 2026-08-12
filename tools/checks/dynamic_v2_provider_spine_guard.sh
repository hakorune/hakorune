#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="dynamic-v2-provider-spine"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

ADMITTED="$ROOT_DIR/src/box_callable/admitted.rs"
MOD="$ROOT_DIR/src/box_callable/mod.rs"

guard_require_command "$TAG" rg
guard_require_command "$TAG" wc
guard_require_files "$TAG" "$ADMITTED" "$MOD"

lines="$(wc -l < "$ADMITTED" | tr -d '[:space:]')"
if (( lines >= 800 )); then
  guard_fail "$TAG" "provider spine reached the hard 800-line boundary: $lines"
fi

guard_expect_fixed_in_file "$TAG" "pub(crate) struct BoxCallableRegistryDraftV1" "$ADMITTED" \
  "provider facts must enter through a mutable draft"
guard_expect_fixed_in_file "$TAG" "pub(crate) struct BoxCallableProviderAdmissionSealV1" "$ADMITTED" \
  "one consuming admission seal must own the draft transition"
guard_expect_fixed_in_file "$TAG" "pub(crate) struct AdmittedBoxCallableRegistryV1" "$ADMITTED" \
  "admission must publish an immutable snapshot"
guard_expect_fixed_in_file "$TAG" "BTreeMap<BoxCallableKey, AdmittedBoxCallableEntryV1>" "$ADMITTED" \
  "admitted entries must have deterministic key order"
guard_expect_fixed_in_file "$TAG" "DuplicateKey" "$ADMITTED" \
  "duplicate callable keys must fail admission"
guard_expect_fixed_in_file "$TAG" "ProviderMismatch" "$ADMITTED" \
  "foreign provider facts must fail admission"
guard_expect_fixed_in_file "$TAG" "MissingMethodRoute" "$ADMITTED" \
  "method exports must have an exact route projection"
guard_expect_fixed_in_file "$TAG" "admission_consumes_draft_into_deterministic_immutable_snapshot" "$ADMITTED" \
  "positive admission evidence is required"
guard_expect_fixed_in_file "$TAG" "admission_rejects_duplicate_and_foreign_provider_facts" "$ADMITTED" \
  "duplicate/foreign negative evidence is required"

for type_name in BoxCallableRegistryDraftV1 AdmittedBoxCallableRegistryV1; do
  derive_line="$(rg -n -B2 "pub\(crate\) struct ${type_name}" "$ADMITTED" | rg '#\[derive' || true)"
  if printf '%s\n' "$derive_line" | rg -q -- 'Clone'; then
    guard_fail "$TAG" "${type_name} must remain move-only"
  fi
done

# This first BoxShape is intentionally cold.  It may not become a second
# production route before the complete TextScan/AOT activation cell exists.
if rg -n --glob '*.rs' \
  'BoxCallableProviderAdmissionSealV1::admit|BoxCallableRegistryDraftV1' \
  "$ROOT_DIR/src" --glob '!src/box_callable/admitted.rs' --glob '!src/box_callable/mod.rs'; then
  guard_fail "$TAG" "provider spine has an unscoped production caller before AOT activation"
fi

if rg -n -- '^[[:space:]]*(pub\([^)]*\)[[:space:]]+)?fn[[:space:]]+(lookup|select|invoke|fallback|retry)[[:space:]]*\(' "$ADMITTED"; then
  guard_fail "$TAG" "cold provider spine exposes a runtime/reselection entry point"
fi
if rg -n -F -- 'HashMap' "$ADMITTED"; then
  guard_fail "$TAG" "cold provider spine must retain deterministic BTreeMap storage"
fi

echo "[$TAG] ok"
