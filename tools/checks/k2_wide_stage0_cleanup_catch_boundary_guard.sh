#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="k2-wide-stage0-cleanup-catch-boundary"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "[$TAG][fail] missing file: $path" >&2
    exit 1
  fi
}

require_grep() {
  local path="$1"
  local pattern="$2"
  if ! rg -n --fixed-strings "$pattern" "$path" >/dev/null; then
    echo "[$TAG][fail] missing pattern in $path: $pattern" >&2
    exit 1
  fi
}

require_file docs/development/current/main/design/stage0-cleanup-catch-boundary-ssot.md
require_file docs/guides/exception-handling.md
require_file docs/guides/exceptions-stage3.md
require_file src/parser/statements/exceptions.rs
require_file src/parser/statements/mod.rs
require_file src/mir/builder/control_flow/exception/try_catch.rs
require_file src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs

boundary_doc="$(find docs/development/current/main/design -maxdepth 1 -name '*cleanup-catch-boundary-ssot.md' -print -quit)"
compat_guide="$(find docs/guides -maxdepth 1 -name 'exceptions-*.md' -print -quit)"

require_grep "$boundary_doc" 'Status: Superseded implementation-boundary inventory'
require_grep "$boundary_doc" 'accepted target is Result-only postfix `?`'
require_grep "$boundary_doc" 'one standalone `cleanup {}`'
require_grep docs/development/current/main/design/stage0-cleanup-catch-boundary-ssot.md "throw:"
require_grep docs/development/current/main/design/stage0-cleanup-catch-boundary-ssot.md "reserved/prohibited in the source surface"
require_grep docs/development/current/main/design/stage0-cleanup-catch-boundary-ssot.md "JoinIR strict:"
require_grep docs/development/current/main/design/stage0-cleanup-catch-boundary-ssot.md 'no JoinIR strict lowering of `TryCatch`'

require_grep docs/guides/exception-handling.md 'Status: Accepted C′ target guide; production activation 0.'
require_grep docs/guides/exception-handling.md 'Canonical v1 has no source `try`, `throw`, `catch`'
require_grep docs/guides/exception-handling.md '## Current implementation boundary'

require_grep "$compat_guide" 'Status: Historical/compatibility implementation inventory'
require_grep "$compat_guide" '## Accepted C′ boundary'

require_grep src/parser/statements/exceptions.rs "[freeze:contract][parser/throw_reserved]"
require_grep src/parser/statements/exceptions.rs "[freeze:contract][parser/try_reserved]"
require_grep src/parser/statements/mod.rs '[freeze:contract][parser/cleanup_canonical] use `cleanup { ... }`; `finally` is reserved as terminology only'

require_grep src/mir/builder/control_flow/exception/try_catch.rs "deferred returns and cleanup blocks"
require_grep src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs "Stage0 cleanup uses the MIR builder route"
require_grep src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs "throw is reserved/prohibited in Stage0"

echo "[$TAG] ok"
