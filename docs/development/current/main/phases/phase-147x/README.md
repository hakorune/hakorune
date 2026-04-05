# Phase 147x: semantic optimization contract selection

- Status: Active
- 目的: 最適化 authority を `.hako owner -> MIR canonical contract -> Rust executor -> LLVM generic optimization/codegen` に固定する
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`

## Decision Now

- optimization authority は Rust helper に置かない
- `.hako` が route / retained-form / boundary を決める
- MIR が canonical contract を持つ
- Rust は executor / accelerator に徹する
- LLVM には owner-aware placement を渡さない

## Immediate Slices

1. `phase-148x borrowed text and sink contract freeze`
2. `phase-149x concat const-suffix vertical slice`
3. `phase-150x array string-store vertical slice`
4. `phase-137x main kilo reopen selection`

## Notes

- `BorrowedText` / `TextSink` は使ってよい
- ただし Rust public authority 名にしない
- contract-first で切る
