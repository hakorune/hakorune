#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT"

require_text() {
  local file="$1"
  local text="$2"
  if ! grep -Fq "$text" "$file"; then
    echo "[delegate-exposes-lowering] missing '$text' in $file" >&2
    exit 1
  fi
}

require_text docs/development/current/main/design/delegation-exposes-lowering-ssot.md "DEL-003 Stage1 Delegate Exposes Lowering"
require_text docs/development/current/main/phases/phase-293x/293x-274-DEL-003-STAGE1-DELEGATE-EXPOSES-LOWERING.md "Status: complete"
require_text src/parser/delegate_lowering.rs "lower_delegate_exposes"
require_text src/parser/delegate_lowering.rs "build_forwarding_method"
require_text src/parser/mod.rs "DelegateLowering"
require_text src/tests/parser_delegate_surface.rs "parser_delegate_surface_rejects_local_method_collision"
require_text docs/tools/check-scripts-index.md "k2_wide_delegate_exposes_lowering_guard.sh"

cargo test -q parser_delegate_surface

echo "[delegate-exposes-lowering] OK"
