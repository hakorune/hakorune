# Stash Triage

Last updated: 2026-07-16

Purpose: track stash items for keep/drop/hold decisions. This is a triage log; SSOT for task status remains `docs/development/current/main/10-Now.md`.

## Current audit

Three read-only worker audits classified all 48 entries present before this
cleanup. Fifteen entries were dropped only after they were proven landed,
superseded, explicitly regressive, or effect-free. Thirty-three remain.

Use stash labels and hashes, never mutable `stash@{n}` ordinals.

### Active evidence — keep

| Hash | Label | Law |
| --- | --- | --- |
| `da77002211` | `wip/hmi-s0-v0-r0-i0 cross-file static formal loses MapBox type` | Current typed-formal consultation evidence. Never apply as implementation authority. Drop after TYPE0 and clean I0 close. |
| `58a8b63798` | `wip/hmi-s0-v0-r0 register storage fails field mutation` | Rejected owner-roundtrip evidence. Never apply. Drop after HMI R0-G0. |

### Project-parked evidence — keep

```text
wip/g0-source-projection before resolved-region-flow supersession
wip/c2-i0-source-projection before generic-baseline supersession
wip/hako-source-carrier-p1
wip/s3-b2-typed-carrier utf8-byte-authority-design-stop
wip/s3-hako-snapshot-reader before rust-algebra-design-stop
```

The first three permit selective recovery only. The last two are evidence-only
and must not be applied or partially cherry-picked.

### 2026-07-16 dropped

```text
e29886fcff  HMI T0 hardening prototype
fec969eb71  JSON iterative P0 escaped-key WIP
66725ad4dd  pre-iterative HMI T0 call-depth WIP
48a0b6520b  SSA-RC restored split
54f34a88fc  SSA-RC physical split
a94241e930  A2-C1 record probe
bab5acd7e6  A2-C1 neutral proof
43c88e075b  A2-C0 capture probe
a8f50bbb86  generic-loop progression A2 red WIP
3af1b029a6  canonical-loop snapshot design-stop WIP
a528b23e82  boxed-sum payload pre-consult WIP
4df80a2f17  generic-set route descriptor WIP
be0a9a216b  region-observer metadata WIP
138c42676b  MapGet runtime-load cycle regression
718da6b1f2  exit predecessor snapshot with no effect
```

### Remaining legacy policy

Old branch recovery archives, large AOT/Phase31 batches, and entries explicitly
marked `hold` below remain untouched. Ambiguous probes and paired cleanup
stashes also remain until a dedicated diff extraction proves a drop decision.

---

The table below is the historical 2026-01-29 audit. Its ordinals are no longer
current and must not be used operationally.

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
