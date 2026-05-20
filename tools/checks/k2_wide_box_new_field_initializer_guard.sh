#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

TAG="k2-wide-box-new-field-initializer"

require_file() {
  local path="$1"
  if [[ ! -f "$path" ]]; then
    echo "[$TAG][fail] missing file: $path" >&2
    exit 1
  fi
}

require_grep() {
  local pattern="$1"
  local path="$2"
  if ! rg -q "$pattern" "$path"; then
    echo "[$TAG][fail] missing pattern in $path: $pattern" >&2
    exit 1
  fi
}

require_no_grep() {
  local pattern="$1"
  local path="$2"
  if rg -q "$pattern" "$path"; then
    echo "[$TAG][fail] forbidden pattern in $path: $pattern" >&2
    exit 1
  fi
}

require_file docs/development/current/main/phases/phase-293x/293x-992-BOX-INIT-001-NEW-BOX-FIELD-INITIALIZER.md
require_file docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md
require_file docs/reference/language/lifecycle.md
require_file docs/reference/language/EBNF.md
require_file src/parser/expr/primary.rs
require_file src/parser/expr_cursor.rs
require_file src/mir/builder/builder_build.rs
require_file src/mir/builder/fields.rs
require_file src/tests/parser_box_new_field_initializer_surface.rs
require_file src/tests/mir_box_new_field_initializer.rs

require_grep "field_initializers" src/ast/mod.rs
require_grep "parse_box_field_initializers" src/parser/expr/primary.rs
require_grep "parse_box_field_initializers" src/parser/expr_cursor.rs
require_grep "build_new_expression_with_field_initializers" src/mir/builder/builder_build.rs
require_grep "build_box_field_initializers" src/mir/builder/fields.rs
require_grep "\\[box-init/duplicate-field\\]" src/mir/builder/fields.rs
require_grep "\\[box-init/unknown-field\\]" src/mir/builder/fields.rs
require_grep "\\[box-init/coreplan-unsupported\\]" src/mir/builder/control_flow/plan/normalizer/helpers_value.rs
require_grep "new Box \\{ field: expr \\}" docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md
require_grep "box_init_block" docs/reference/language/EBNF.md
require_grep "No wildcard copy" docs/development/current/main/phases/phase-293x/293x-992-BOX-INIT-001-NEW-BOX-FIELD-INITIALIZER.md

cargo test -q parser_box_new_field_initializer_surface
cargo test -q mir_box_new_field_initializer

echo "[$TAG][ok] new-box field initializer surface is fixed"
