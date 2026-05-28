#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-193-MIR-TYPED-FIELD-RESIDENCE-SSOT.md"

grep -q '^Decision: provisional$' "$DOC"
grep -q '^mir_typed_field_residence_ssot=accepted$' "$DOC"
grep -q '^transform_open=0$' "$DOC"
grep -q '^helper_abi_fallback=1$' "$DOC"
grep -q '^by_name_special_case=0$' "$DOC"
grep -q '^winner_claim=0$' "$DOC"
grep -q '^replacement_active=0$' "$DOC"
grep -q '^hook_installed=0$' "$DOC"
grep -q '^global_allocator=0$' "$DOC"
grep -q '^summary=ok$' "$DOC"

echo "mir_typed_field_residence_ssot_guard=ok"
