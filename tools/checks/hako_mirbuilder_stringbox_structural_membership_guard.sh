#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
SOURCE="$ROOT_DIR/lang/src/mir/builder/internal/lower_return_method_string_length_box.hako"
TAG="hako-mirbuilder-stringbox-structural-membership"

if [[ ! -f "$SOURCE" ]]; then
  echo "[$TAG] missing lowerer: $SOURCE" >&2
  exit 1
fi

line_count="$(wc -l < "$SOURCE" | tr -d '[:space:]')"
if (( line_count >= 800 )); then
  echo "[$TAG] lowerer exceeds 800-line hard stop: $line_count" >&2
  exit 1
fi

forbidden=(
  '_read_first_string_literal_between'
  '_read_method_first_string_arg'
  'k_class'
  'index_of_from(s, "\\"type\\":\\"Str\\""'
)
for pattern in "${forbidden[@]}"; do
  if rg -n -F -- "$pattern" "$SOURCE" >/dev/null; then
    echo "[$TAG] forbidden unbounded recognition remains: $pattern" >&2
    exit 1
  fi
done

required=(
  '_find_return_method'
  '_object_shape_ok'
  '_object_value_start'
  '_array_single_direct_string'
  '_skip_json_value'
  'JsonCursorBox.seek_obj_end'
  'JsonCursorBox.seek_array_end'
)
for pattern in "${required[@]}"; do
  if ! rg -n -F -- "$pattern" "$SOURCE" >/dev/null; then
    echo "[$TAG] bounded structural reader missing: $pattern" >&2
    exit 1
  fi
done

if ! rg -n 'mname == "length"|mname == "size"|mname == "indexOf"' "$SOURCE" >/dev/null; then
  echo "[$TAG] selected method membership is missing" >&2
  exit 1
fi

echo "[$TAG] PASS (lines=$line_count; bounded-membership helpers present)"
