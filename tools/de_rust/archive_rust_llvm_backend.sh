#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
SRC_DIR="$ROOT/src/backend/llvm"
DST_DIR="$ROOT/archive/rust-llvm-backend/llvm"
RESTORE_MD="$ROOT/archive/rust-llvm-backend/RESTORE.md"

echo "[info] De‑Rust Phase‑0: archive Rust LLVM backend"

if [ ! -d "$SRC_DIR" ]; then
  echo "[warn] src/backend/llvm not found; nothing to do" >&2
  exit 0
fi

mkdir -p "$(dirname "$DST_DIR")"
git mv "$SRC_DIR" "$DST_DIR"

cat > "$RESTORE_MD" <<'EOF'
# RESTORE — Rust LLVM backend

To restore the archived backend back to the original location:

  git mv archive/rust-llvm-backend/llvm src/backend/

Rationale: Python llvmlite is the primary LLVM path. The Rust backend was deprecated and archived as part of the De‑Rust Phase‑0.
EOF

echo "[done] Archived to archive/rust-llvm-backend/. Review and commit when ready."

