#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="macos-portability-guard"
PLUGIN_LOADER_RS="$ROOT_DIR/src/runtime/plugin_loader_v2/enabled/loader/library.rs"
LLVM_CODEGEN_RS="$ROOT_DIR/src/host_providers/llvm_codegen.rs"
FFI_BUILD_SH="$ROOT_DIR/tools/build_hako_llvmc_ffi.sh"
PLAN_DOC="$ROOT_DIR/docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md"

cd "$ROOT_DIR"

guard_require_command "$TAG" rg
guard_require_files "$TAG" "$PLUGIN_LOADER_RS" "$LLVM_CODEGEN_RS" "$FFI_BUILD_SH" "$PLAN_DOC"

echo "[$TAG] checking macOS portability contracts"

guard_expect_in_file \
  "$TAG" \
  'cfg!\(target_os = "macos"\)' \
  "$PLUGIN_LOADER_RS" \
  "plugin loader must include macOS branch"
guard_expect_in_file \
  "$TAG" \
  'with_extension\("dylib"\)' \
  "$PLUGIN_LOADER_RS" \
  "plugin loader must include .dylib candidate mapping"

guard_expect_in_file \
  "$TAG" \
  'ffi_library_default_candidates' \
  "$LLVM_CODEGEN_RS" \
  "llvm_codegen must centralize FFI library candidate resolution"
guard_expect_in_file \
  "$TAG" \
  'libhako_llvmc_ffi\.dylib' \
  "$LLVM_CODEGEN_RS" \
  "llvm_codegen must include .dylib candidate for macOS"

guard_expect_in_file \
  "$TAG" \
  'Darwin' \
  "$FFI_BUILD_SH" \
  "FFI build script must branch for Darwin"
guard_expect_in_file \
  "$TAG" \
  'libhako_llvmc_ffi\.dylib' \
  "$FFI_BUILD_SH" \
  "FFI build script must emit .dylib on macOS"

guard_expect_in_file \
  "$TAG" \
  'Rust maintenance after selfhost' \
  "$PLAN_DOC" \
  "de-rust post-G1 plan must include Rust maintenance policy section"

echo "[$TAG] ok"
