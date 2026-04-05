# Phase 133x: mainline/compiler resume selection

- 目的: `phase-132x` closeout 後に、vm keep の parked debt を凍結したまま mainline/compiler 開発へ戻る。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - current compiler/mainline lane selection
- success:
  - `phase-132x` is landed
  - current no longer reads like vm retirement work
  - next active lane is a compiler/mainline lane, not caller-zero cleanup

## Decision Now

- `vm` default removal is landed
- explicit `vm` / `vm-hako` proof-debug lanes remain frozen keep
- caller-zero remains parked debt, not current work
- next work should advance compiler/mainline behavior, not legacy route cleanup

## Next

1. close `phase-132x`
2. pick the next compiler/mainline implementation lane
3. keep vm-family retirement work parked unless a new exact blocker appears
