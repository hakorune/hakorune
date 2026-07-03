#!/usr/bin/env bash

resolve_hakorune_golden_bin() {
  local root="$1"
  local primary="${HAKORUNE_BIN:-$root/target/release/hakorune}"
  local fallback="$root/target/release/nyash"
  local bin="${NYASH_BIN:-$primary}"

  if [ ! -x "$bin" ] && [ -z "${NYASH_BIN:-}" ] && [ -x "$fallback" ]; then
    bin="$fallback"
  fi

  if [ ! -x "$bin" ]; then
    echo "Hakorune binary not found at $bin; build first (cargo build --release) or set NYASH_BIN for compatibility override" >&2
    return 1
  fi

  printf '%s\n' "$bin"
}
