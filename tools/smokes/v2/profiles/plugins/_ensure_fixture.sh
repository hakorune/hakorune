#!/bin/bash
# _ensure_fixture.sh - フィクスチャプラグイン自動準備

set -uo pipefail

detect_ext() {
  case "$(uname -s)" in
    Darwin) echo "dylib" ;;
    MINGW*|MSYS*|CYGWIN*|Windows_NT) echo "dll" ;;
    *) echo "so" ;;
  esac
}

lib_name_for() {
  local base="$1"  # e.g., nyash_fixture_plugin
  local ext="$2"
  if [ "$ext" = "dll" ]; then
      echo "${base}.dll"
  else
      echo "lib${base}.${ext}"
  fi
}

ensure_plugin_generic() {
  # Args: <crate_dir_name> <crate_name_base> (e.g., nyash-fixture-plugin nyash_fixture_plugin)
  local crate_dir="$1"
  local base_crate_name="$2"
  local root="${NYASH_ROOT:-$(cd "$(dirname "$0")/../../../.." && pwd)}"
  local ext="$(detect_ext)"
  local out_dir="$root/plugins/${crate_dir}"
  local out_file="$out_dir/$(lib_name_for ${base_crate_name} "$ext")"

  mkdir -p "$out_dir"
  if [ -f "$out_file" ]; then
    echo "[INFO] Plugin present: $out_file" >&2
    return 0
  fi

  echo "[INFO] Building plugin ($crate_dir) ..." >&2
  if cargo build --release -p "$crate_dir" >/dev/null 2>&1; then
    local src=""
    case "$ext" in
      dll) src="$root/target/release/${base_crate_name}.dll" ;;
      dylib) src="$root/target/release/lib${base_crate_name}.dylib" ;;
      so) src="$root/target/release/lib${base_crate_name}.so" ;;
    esac
    if [ -f "$src" ]; then
      cp -f "$src" "$out_file"
      echo "[INFO] Plugin installed: $out_file" >&2
      return 0
    fi
  fi
  echo "[WARN] Could not build/install plugin: $crate_dir" >&2
  return 1
}

ensure_fixture_plugin() {
  local root="${NYASH_ROOT:-$(cd "$(dirname "$0")/../../../.." && pwd)}"
  local ext="$(detect_ext)"
  local out_dir="$root/plugins/nyash-fixture-plugin"
  local out_file="$out_dir/$(lib_name_for nyash_fixture_plugin "$ext")"

  mkdir -p "$out_dir"
  if [ -f "$out_file" ]; then
    echo "[INFO] Fixture plugin present: $out_file" >&2
    return 0
  fi

  echo "[INFO] Building fixture plugin (nyash-fixture-plugin) ..." >&2
  if cargo build --release -p nyash-fixture-plugin >/dev/null 2>&1; then
    local src=""
    case "$ext" in
      dll) src="$root/target/release/nyash_fixture_plugin.dll" ;;
      dylib) src="$root/target/release/libnyash_fixture_plugin.dylib" ;;
      so) src="$root/target/release/libnyash_fixture_plugin.so" ;;
    esac
    if [ -f "$src" ]; then
      cp -f "$src" "$out_file"
      echo "[INFO] Fixture plugin installed: $out_file" >&2
      return 0
    fi
  fi
  echo "[WARN] Could not build/install fixture plugin (will SKIP related tests)" >&2
  return 1
}

ensure_counter_plugin() { ensure_plugin_generic nyash-counter-plugin nyash_counter_plugin; }
ensure_math_plugin()    { ensure_plugin_generic nyash-math-plugin nyash_math_plugin; }
ensure_string_plugin()  { ensure_plugin_generic nyash-string-plugin nyash_string_plugin; }
