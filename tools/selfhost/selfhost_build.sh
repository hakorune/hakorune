#!/usr/bin/env bash
# selfhost_build.sh — Direct/core-first selfhost facade
#
# Goals:
# - Take a Hako source (.hako), compile MIR/EXE requests through direct MIR.
# - Keep direct MIR, EXE artifact, run, and route-main logic in helper files.
# - Optionally run via direct MIR(JSON) to verify exit code.
# - Optionally build an executable via ny-llvmc.
#
# Usage:
#   tools/selfhost/selfhost_build.sh --in source.hako (--mir out.json | --exe out | --run)
#   Options:
#     --in FILE     Input .hako source file (required)
#     --json FILE   Retired wrapper surface (compat-only; rejected with redirect)
#     --run         Run via direct MIR(JSON) after compilation
#     --mir FILE    Emit MIR(JSON) to FILE; MIR-only requests use the direct route
#     --exe FILE    Build native EXE via ny-llvmc
#     --keep-tmp    Retired; use tools/dev/program_json_v0/stageb_artifact_probe.sh
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
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_direct.sh" ]; then
  # Direct MIR owner lives in its own helper file.
  # Keep this script focused on run / exe-artifact / dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_direct.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_run.sh" ]; then
  # Run route helpers live in their own helper file.
  # Keep this script focused on exe-artifact / dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_run.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_exe.sh" ]; then
  # EXE artifact owner lives in its own helper file.
  # Keep this script focused on dispatcher routing.
  source "$ROOT/tools/selfhost/lib/selfhost_build_exe.sh"
fi
if [ -f "$ROOT/tools/selfhost/lib/selfhost_build_route.sh" ]; then
  # Route-main orchestration lives in its own helper file.
  # Keep this script as a thin facade over the route main.
  source "$ROOT/tools/selfhost/lib/selfhost_build_route.sh"
fi

selfhost_build_main "$@"
