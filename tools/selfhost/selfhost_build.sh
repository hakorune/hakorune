#!/usr/bin/env bash
# selfhost_build.sh — Direct/core-first selfhost facade
#
# Goals:
# - Take a Hako source (.hako), compile MIR/EXE requests through direct MIR,
#   and keep Stage-B Program(JSON v0) production for run/debug/explicit raw routes.
# - Keep Stage‑B producer, direct MIR, EXE artifact, and final dispatcher logic in helper files.
# - Optionally run via Gate‑C/Core Direct (in‑proc) to verify exit code.
# - Optionally build an executable via ny-llvmc.
#
# Usage:
#   tools/selfhost/selfhost_build.sh --in source.hako (--mir out.json | --exe out | --run | --keep-tmp)
#   Options:
#     --in FILE     Input .hako source file (required)
#     --json FILE   Retired wrapper surface (compat-only; rejected with redirect)
#     --run         Run via Core-Direct after compilation
#     --mir FILE    Emit MIR(JSON) to FILE; MIR-only requests use the direct route
#     --exe FILE    Build native EXE via ny-llvmc
#     --keep-tmp    Keep and print the temporary Stage-B artifact path
#     --core        Deprecated (JoinIR Core は常時 ON のため無視・警告のみ)
#     --strict      Phase 81: Enable Strict mode (fail-fast, no fallback)
#   Env:
#     NYASH_BIN: path to hakorune/nyash binary (auto-detected if omitted)
#     NYASH_ROOT: repo root (auto-detected)
#     NYASH_JOINIR_CORE: Deprecated (常時 ON のため無視・警告のみ)
#     NYASH_JOINIR_STRICT: Set to 1 for Strict mode
#
set -euo pipefail

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
BIN="${NYASH_BIN:-}"
if [ -z "${BIN}" ]; then
  if [ -x "$ROOT/target/release/hakorune" ]; then BIN="$ROOT/target/release/hakorune";
  elif [ -x "$ROOT/target/release/nyash" ]; then BIN="$ROOT/target/release/nyash";
  else echo "[selfhost] error: NYASH_BIN not set and no binary found under target/release" >&2; exit 2; fi
fi
SMOKE_ENV_SKIP_EXPORTS=1
if [ -f "$ROOT/tools/smokes/v2/lib/env.sh" ]; then
  source "$ROOT/tools/smokes/v2/lib/env.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_stageb.sh" ]; then
  # Stage-B producer owner lives in its own helper file.
  # Keep this script focused on direct-run / exe-artifact / dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_stageb.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_direct.sh" ]; then
  # Direct MIR / core-direct owner lives in its own helper file.
  # Keep this script focused on exe-artifact / dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_direct.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_exe.sh" ]; then
  # EXE artifact owner lives in its own helper file.
  # Keep this script focused on dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_exe.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_dispatch.sh" ]; then
  # Final route dispatcher lives in its own helper file.
  # Keep this script focused on arg parsing and top-level tail orchestration.
  source "$ROOT/tools/selfhost/lib/selfhost_build_dispatch.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_route.sh" ]; then
  # Route-main orchestration lives in its own helper file.
  # Keep this script as a thin facade over the route main.
  source "$ROOT/tools/selfhost/lib/selfhost_build_route.sh"
fi

selfhost_build_main "$@"
