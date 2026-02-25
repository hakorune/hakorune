# Stash Triage (Phase 29bq)

Last updated: 2026-01-29

Purpose: track stash items for keep/drop/hold decisions. This is a triage log; SSOT for task status remains `docs/development/current/main/10-Now.md`.

Source: `git stash list` captured on 2026-01-29 (post-drop update #7).

## Triage Table

| Stash | Title | Status | Notes | Next step |
| --- | --- | --- | --- | --- |
| stash@{0} | wip: deferred nested3 fixture (no commit) | hold | fixture 1件のみ（depth3 nested return）。試行では出力が -1 になり早期return未成立。現行では未受理なので gate 追加は保留。 | hold |
| stash@{1} | wip/joinir-entry-ssot-plan (unrelated to recipe unification) | hold | docs＋env統合＋trace変更が混在。CURRENT_TASK.md 追記は不可（ポインタ方針）。必要なら内容を分割して手動適用。 | hold |
| stash@{2} | stash: aot-untracked pending | hold | AOT/bench/docs/manifest/binary を含む大きな差分。現フェーズ外なので hold（適用は専用ブランチで）。 | hold |
| stash@{3} | stash: move AOT changes for phase33 | hold | 大量差分＋tmp/バイナリ含む。phase33/AOT 専用ブランチで扱うべき。 | hold |
| stash@{4} | Phase 31 runtime changes | hold | 別ブランチ（phase31-wip）起因。現フェーズ外。 | hold |
| stash@{5} | docs(selfhost): add CURRENT_TASK; Phase 15.7 note for nyash CLI; PHI JSON values format | hold | 別ブランチ（selfhost-docs-fix-20251001）起因。現フェーズ外。 | hold |
| stash@{6} | temp switch for cherry-pick | hold | master 起因。現フェーズ外。 | hold |
| stash@{7} | codex: switch to selfhost | hold | master 起因。現フェーズ外。 | hold |
| stash@{8} | phi-values-unify + --entry VM wiring + Strict docs/ENV + smokes | hold | master 起因。現フェーズ外。 | hold |
| stash@{9} | codex-sync-1759294010 | hold | master 起因。現フェーズ外。 | hold |
| stash@{10} | Phase 3.1 PHI fix - wrongly on selfhost branch | hold | master 起因。現フェーズ外。 | hold |
| stash@{11} | llvm: refactor compiler into aot, codegen, interpreter, helpers (#140) | hold | selfhosting-dev 起因。現フェーズ外。 | hold |
| stash@{12} | CURRENT_TASK PR #134 investigation updates | hold | selfhosting-dev 起因。現フェーズ外。 | hold |
| stash@{13} | WIP: SSA debugging progress | hold | selfhosting-dev-clean 起因。現フェーズ外。 | hold |

## Priority Order (initial)

- All remaining stashes are hold/out-of-scope for Phase 29bq. Revisit only if the related branch/task is reactivated.

## Completed (dropped)

- 2026-01-29: wip/entry-disjoint scan_methods_block_vs_base (hit strict_nested_loop_guard) — SSOT 追記と registry predicate 更新を反映後に drop。
- 2026-01-29: wip/phase29bq_loop_if_else_if_return (fails fast gate: planner None) — gate TSV 追加のみのため drop（fix後に再追加）。
- 2026-01-29: wip/phase29bq_loop_if_else_return_local (fails fast gate: planner None) — gate TSV 追加のみのため drop（fix後に再追加）。
- 2026-01-29: wip/phase29bq_loop_if_return_local (fast gate freeze: planner None) — gate TSV 追加のみのため drop（fix後に再追加）。
- 2026-01-29: wip/LoopCondContinueWithReturn (fails fast gate) — 旧構造＋未ガードログのため drop（現行は loop_cond_unified/variants）。
- 2026-01-29: wip/pre-balanced-depth-scan-view (unrelated changes) — CURRENT_TASK.md 追記を行わず drop。
- 2026-01-29: nested3 fixture gate trial failed (actual -1 vs expected 1) — re-stashed as hold.
