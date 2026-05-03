#!/usr/bin/env bash
# Archived Phase 21.6 dev env helper (historical evidence only).
set -euo pipefail

export HAKO_SELFHOST_BUILDER_FIRST=1
export NYASH_USE_NY_COMPILER=0
export HAKO_DISABLE_NY_COMPILER=1
export NYASH_FEATURES=stage3
export NYASH_FEATURES=stage3
export NYASH_PARSER_ALLOW_SEMICOLON=1
export NYASH_ENABLE_USING=1
export HAKO_ENABLE_USING=1
export NYASH_LLVM_BACKEND=${NYASH_LLVM_BACKEND:-crate}

echo "[phase216] env set: selfhost-first + stage-b + crate backend"
