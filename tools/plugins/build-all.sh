#!/bin/bash
# build-all.sh — Build and stage dynamic plugin .so/.dylib/.dll into plugins/*
# Usage: tools/plugins/build-all.sh [crate_dir ...]

set -euo pipefail

ROOT="${NYASH_ROOT:-$(git rev-parse --show-toplevel 2>/dev/null || pwd)}"
cd "$ROOT"

detect_ext() {
  case "$(uname -s)" in
    Darwin) echo dylib ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT) echo dll ;;
    *) echo so ;;
  esac
}

lib_name_for() {
  local base="$1"; local ext="$2"
  if [ "$ext" = dll ]; then echo "${base}.dll"; else echo "lib${base}.${ext}"; fi
}

build_and_stage() {
  local crate_dir="$1"     # e.g., nyash-counter-plugin
  local base_name="$2"     # e.g., nyash_counter_plugin
  local ext
  ext=$(detect_ext)
  echo "[plugins/build-all] building: $crate_dir" >&2
  cargo build --release -p "$crate_dir" >/dev/null
  local src
  case "$ext" in
    dll)   src="target/release/${base_name}.dll" ;;
    dylib) src="target/release/lib${base_name}.dylib" ;;
    so)    src="target/release/lib${base_name}.so" ;;
  esac
  if [ ! -f "$src" ]; then
    echo "[plugins/build-all] WARN: built artifact not found: $src" >&2
    return 1
  fi
  local out_dir="plugins/${crate_dir}"
  mkdir -p "$out_dir"
  local dst="$out_dir/$(lib_name_for "$base_name" "$ext")"
  cp -f "$src" "$dst"
  echo "[plugins/build-all] staged: $dst" >&2
}

CRATES=(
  nyash-fixture-plugin:nyash_fixture_plugin
  nyash-counter-plugin:nyash_counter_plugin
  nyash-math-plugin:nyash_math_plugin
  nyash-string-plugin:nyash_string_plugin
  nyash-console-plugin:nyash_console_plugin
)

if [ "$#" -gt 0 ]; then
  # Accept explicit crate_dir list
  for dir in "$@"; do
    case "$dir" in
      nyash-*-plugin)
        base="${dir//-/_}" # hyphen→underscore
        build_and_stage "$dir" "$base" || true
        ;;
      *)
        echo "[plugins/build-all] WARN: unknown crate dir pattern: $dir" >&2
        ;;
    esac
  done
else
  for ent in "${CRATES[@]}"; do
    IFS=: read -r dir base <<<"$ent"
    if [ -d "plugins/$dir" ]; then
      build_and_stage "$dir" "$base" || true
    fi
  done
fi

echo "[plugins/build-all] done" >&2
