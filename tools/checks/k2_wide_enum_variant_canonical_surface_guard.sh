#!/usr/bin/env bash
set -euo pipefail

card='docs/development/current/main/phases/phase-293x/293x-311-ENUMVAR-001-ENUM-VARIANT-CANONICAL-SURFACE.md'

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[enum-variant-surface] missing '$text' in $file" >&2
    exit 1
  fi
}

require_absent() {
  local file="$1"
  local text="$2"
  if grep -Fq "$text" "$file"; then
    echo "[enum-variant-surface] forbidden '$text' in $file" >&2
    exit 1
  fi
}

require_text "$card" "293x-311 ENUMVAR-001 enum variant canonical surface"
require_text docs/development/current/main/design/array-result-option-canonical-surface-ssot.md "Type::Variant"
require_text docs/reference/language/EBNF.md "transition_member := 'transition' TYPE_REF '::' IDENT '-' '>' TYPE_REF '::' IDENT 'by' IDENT"
require_text docs/reference/language/EBNF.md "qualified_ctor   := IDENT '::' IDENT"
require_text docs/development/current/main/design/transition-metadata-capsule-ssot.md "transition Enum::Value -> Enum::Value by method"
require_text docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md 'Legacy `Enum.A` metadata is accepted and normalized by `ENUMVAR-001`.'
require_absent docs/development/current/main/design/transition-metadata-capsule-ssot.md "transition Enum.Value -> Enum.Value"
require_absent docs/development/current/main/design/language-minimal-surface-ssot.md "transition PageState.Active"

if ! rg -n 'DoubleColon.*DOT|DOT.*DoubleColon|Enum\\.Value' src/parser/declarations/box_def/members/transitions.rs >/dev/null; then
  echo '[enum-variant-surface] transition parser does not document canonical/legacy separator handling' >&2
  exit 1
fi

cargo test -q source_to_program_json_v0_transports_transition_metadata --lib
cargo test -q source_to_program_json_v0_normalizes_legacy_dot_transition_refs --lib

echo "[enum-variant-surface] OK"
