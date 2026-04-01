# Phase 278 P0: Remove deprecated PHI debug env vars

Status: ✅ completed (2025-12-22)

Goal: remove legacy PHI debug environment variables that were consolidated in Phase 277 P2, so the ecosystem converges to a single, memorable set.

Reference:
- Consolidation doc: `docs/development/current/main/phases/phase-277/P2-COMPLETION.md`
- Env var reference: `docs/reference/environment-variables.md` (PHI デバッグ関連)

Target SSOT (post-Phase 278):
- `NYASH_LLVM_DEBUG_PHI=1`
- `NYASH_LLVM_DEBUG_PHI_TRACE=1`
- `NYASH_LLVM_PHI_STRICT=1`

Implementation guide:
- `docs/development/current/main/phases/phase-278/P0-INSTRUCTIONS.md`

Completion:
- `docs/development/current/main/phases/phase-278/P0-COMPLETION.md`
