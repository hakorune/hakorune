#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"

cd "$ROOT_DIR"

echo "[phase29cl-by-name-surrogate-archive-guard] checking compiled-stage1 surrogate routes stay archive-only"

if ! command -v rg >/dev/null 2>&1; then
  echo "[phase29cl-by-name-surrogate-archive-guard] ERROR: rg is required" >&2
  exit 2
fi

if rg -n '"lang\.compiler\.build\.build_box"|"selfhost\.shared\.backend\.llvm_backend"' \
  crates/nyash_kernel/src/tests.rs \
  -S >/dev/null; then
  echo "[phase29cl-by-name-surrogate-archive-guard] ERROR: kernel test harness still pins surrogate module-string callers" >&2
  rg -n '"lang\.compiler\.build\.build_box"|"selfhost\.shared\.backend\.llvm_backend"' \
    crates/nyash_kernel/src/tests.rs \
    -S >&2
  exit 1
fi

if rg -n 'invoke_by_name_i64|nyash\.plugin\.invoke_by_name_i64' \
  lang/src/runner/launcher.hako \
  lang/src/runner/stage1_cli.hako \
  lang/src/mir/builder/MirBuilderBox.hako \
  tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh \
  -S >/dev/null; then
  echo "[phase29cl-by-name-surrogate-archive-guard] ERROR: direct caller proof regressed back to invoke_by_name" >&2
  rg -n 'invoke_by_name_i64|nyash\.plugin\.invoke_by_name_i64' \
    lang/src/runner/launcher.hako \
    lang/src/runner/stage1_cli.hako \
    lang/src/mir/builder/MirBuilderBox.hako \
    tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh \
    -S >&2
  exit 1
fi

if ! rg -n 'BuildBox\.emit_program_json_v0' \
  lang/src/runner/launcher.hako \
  lang/src/runner/stage1_cli.hako \
  lang/src/mir/builder/MirBuilderBox.hako \
  -S >/dev/null; then
  echo "[phase29cl-by-name-surrogate-archive-guard] ERROR: direct BuildBox caller proof missing" >&2
  exit 1
fi

if ! rg -n 'LlvmBackendBox\.compile_obj|LlvmBackendBox\.link_exe' \
  tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh \
  lang/src/runner/launcher.hako \
  -S >/dev/null; then
  echo "[phase29cl-by-name-surrogate-archive-guard] ERROR: direct LlvmBackendBox caller proof missing" >&2
  exit 1
fi

echo "[phase29cl-by-name-surrogate-archive-guard] ok"
