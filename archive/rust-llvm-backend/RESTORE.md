# RESTORE — Rust LLVM backend

To restore the archived backend back to the original location:

  git mv archive/rust-llvm-backend/llvm src/backend/

Rationale: Python llvmlite is the primary LLVM path. The Rust backend was deprecated and archived as part of the De‑Rust Phase‑0.
