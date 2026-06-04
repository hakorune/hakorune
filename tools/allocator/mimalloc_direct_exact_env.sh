#!/usr/bin/env bash
# Canonical environment for current mimalloc direct-exact measurements and
# parity proofs.
#
# Source this file before direct-exact mimalloc perf/proof work:
#   source tools/allocator/mimalloc_direct_exact_env.sh
#
# Or execute it as a small wrapper:
#   tools/allocator/mimalloc_direct_exact_env.sh --print
#   tools/allocator/mimalloc_direct_exact_env.sh --check
#   tools/allocator/mimalloc_direct_exact_env.sh -- <command> [args...]

mimalloc_direct_exact_env_apply() {
  export NYASH_FEATURES="${NYASH_FEATURES:-rune}"
  export NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}"
  export NYASH_SKIP_TOML_ENV="${NYASH_SKIP_TOML_ENV:-1}"
  export NYASH_GC_MODE="${NYASH_GC_MODE:-off}"
  export NYASH_SCHED_POLL_IN_SAFEPOINT="${NYASH_SCHED_POLL_IN_SAFEPOINT:-0}"
  export HAKO_TYPED_OBJECT_STORE="direct_slot_exact"
  export HAKO_ARRAY_SLOT_STORE="direct_array_i64_exact"
}

mimalloc_direct_exact_env_check() {
  local failed=0

  require_exact() {
    local name="$1"
    local expected="$2"
    local actual="${!name:-}"
    if [[ "$actual" != "$expected" ]]; then
      echo "[mimalloc-direct-exact-env] ${name} expected '${expected}', got '${actual}'" >&2
      failed=1
    fi
  }

  require_exact NYASH_FEATURES rune
  require_exact NYASH_DISABLE_PLUGINS 1
  require_exact NYASH_SKIP_TOML_ENV 1
  require_exact NYASH_GC_MODE off
  require_exact NYASH_SCHED_POLL_IN_SAFEPOINT 0
  require_exact HAKO_TYPED_OBJECT_STORE direct_slot_exact
  require_exact HAKO_ARRAY_SLOT_STORE direct_array_i64_exact

  if [[ "$failed" -ne 0 ]]; then
    return 1
  fi
}

mimalloc_direct_exact_env_print() {
  cat <<EOF
NYASH_FEATURES=${NYASH_FEATURES}
NYASH_DISABLE_PLUGINS=${NYASH_DISABLE_PLUGINS}
NYASH_SKIP_TOML_ENV=${NYASH_SKIP_TOML_ENV}
NYASH_GC_MODE=${NYASH_GC_MODE}
NYASH_SCHED_POLL_IN_SAFEPOINT=${NYASH_SCHED_POLL_IN_SAFEPOINT}
HAKO_TYPED_OBJECT_STORE=${HAKO_TYPED_OBJECT_STORE}
HAKO_ARRAY_SLOT_STORE=${HAKO_ARRAY_SLOT_STORE}
EOF
}

mimalloc_direct_exact_env_apply

if [[ "${BASH_SOURCE[0]}" == "$0" ]]; then
  set -euo pipefail
  case "${1:-}" in
    --print)
      mimalloc_direct_exact_env_print
      ;;
    --check)
      mimalloc_direct_exact_env_check
      echo "[mimalloc-direct-exact-env] ok"
      ;;
    --)
      shift
      mimalloc_direct_exact_env_check
      exec "$@"
      ;;
    -h|--help|"")
      cat >&2 <<'USAGE'
usage:
  tools/allocator/mimalloc_direct_exact_env.sh --print
  tools/allocator/mimalloc_direct_exact_env.sh --check
  tools/allocator/mimalloc_direct_exact_env.sh -- <command> [args...]

This is the canonical direct-exact mimalloc measurement/proof environment.
Do not hand-type HAKO_TYPED_OBJECT_STORE / HAKO_ARRAY_SLOT_STORE for current
mimalloc parity measurements or guards.
USAGE
      ;;
    *)
      echo "[mimalloc-direct-exact-env] unknown argument: $1" >&2
      exit 2
      ;;
  esac
fi
