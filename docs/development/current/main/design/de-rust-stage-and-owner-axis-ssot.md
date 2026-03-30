---
Status: SSOT
Decision: provisional
Date: 2026-03-22
Scope: `stage0/stage1/stage2+` の実行・証跡軸と、`owner/substrate` の責務軸を分離して、phase stop-line と end-state completion の混線を防ぐ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md
  - docs/development/current/main/design/artifact-policy-ssot.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - lang/README.md
---

# De-Rust Stage vs Owner Axis (SSOT)

## Purpose

- `stage0/stage1/stage2+` と `owner/substrate` を同じ progress bar として読まない。
- phase の `done-enough stop line` と end-state の `finished` を分離する。
- bootstrap/buildability の keep と `.hako` owner shift を別軸で読む。
- artifact/lane の daily policy は parent SSOT `execution-lanes-and-axis-separation-ssot.md` を正本にする。
- artifact-role detail と future interpreter reservation は `artifact-policy-ssot.md` を正本にする。

## 1. Two Axes

### 1.1 Stage axis

| Axis | Meaning | Keep / Target |
| --- | --- | --- |
| `stage0` | Rust bootstrap / first-build / recovery lane | explicit keep |
| `stage1` | proof / bridge line for domain-phase owner slices | proof-only or bring-up |
| `stage2+` | daily selfhost mainline / final distribution target | target mainline |

### 1.2 Owner axis

| Axis | Meaning | Target |
| --- | --- | --- |
| compiler authority | compiler meaning/policy owner | `.hako` |
| kernel authority | runtime/kernel meaning/policy owner | `.hako` |
| backend authority | backend daily owner | `.hako -> thin boundary` |
| substrate | bootstrap / ABI / raw memory / handle / GC / LLVM leaf | native keep unless separately retired |

## 2. Reading Rules

1. `stage0` keep は owner migration の失敗を意味しない。
2. `stage1` proof は `stage2+` daily mainline 完了を意味しない。
3. phase の `done-enough stop line` は owner axis 上の局所 closeout であり、end-state completion とは別。
4. `kernel authority zero` は `substrate zero` ではない。
5. `buildability keep` は preservation-first で残してよいが、daily owner を逆流させてはいけない。
6. Rune のような declaration-contract layer は `.hako` compiler authority 側に属するが、substrate migration を意味しない。
7. `stage2+` は `.hako` authority mainline を意味するが、native zero や Rust source zero を意味しない。
8. default distribution shape is `hakoruneup + self-contained release bundle`; stage axis reading と packaging shape を混線させない。
9. boundary truth belongs to `hako.abi / hako.value_repr / ownership-layout manifest`, not to `.inc` partitions.
10. collection cleanup detail belongs to owner/substrate SSOTs; this doc keeps stage vocabulary only and does not use domain-phase progress as a stage definition.
11. exact `stage1 -> stage2+` entry order and the first optimization wave are owned by `stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md`; this doc owns axis vocabulary only.

## 3. Current De-Rust Reading

Canonical short read:

- `stage0 keep / stage1 bridge+proof / stage2+ final mainline`

### 3.1 Stage axis now

- `stage0`: Rust bootstrap / recovery keep
- `stage1`: bridge / proof line
- `stage2+`: `.hako` mainline target / final distribution target

### 3.2 Owner axis now

- compiler authority: separate active lane
- kernel authority: owner-first bounded stop-line landed; current compiler semantic tables live under `runtime/meta/`
- backend authority: queued / separate lane
- substrate: Rust/C keep
- current kernel-side owner/substrate detail is owned by `collection-raw-substrate-contract-ssot.md` and `stage2-collection-substrate-cleanup-ssot.md`
- current stage2+ first-wave reading is `route/perf only`, with Rune optimization metadata still `parse/noop` and backend-active optimization deferred

## 4. Phase-29cm Interpretation

- `phase-29cm` has reached a `done-enough` owner-shift stop line for the current collection wave.
- This means:
  - `.hako` ring1 is the visible owner frontier for `ArrayBox` / `MapBox`
  - `RuntimeDataBox` is facade-only
- This does **not** mean:
  - the end-state collection migration is complete
  - raw substrate ownership has left Rust
  - perf should automatically reopen

Current exact residue below the owner frontier:
- `nyash.array.len_h`
- `nyash.array.push_hh` (compat-only after `slot_append_hh` daily retarget)
- `nyash.map.entry_count_h`

Therefore the next fixed order is:
1. deepen the collection boundary below those transitional exports
2. only then reopen raw substrate perf

## 5. Litmus Questions

When a reader asks “is this done?”, answer these in order:

1. Which axis: `stage` or `owner`?
2. Which level: `stop line`, `done-enough`, or `end-state complete`?
3. Is the remaining residue `method-shaped owner logic` or `raw substrate`?

If the residue is still method-shaped and still crossed by the daily `.hako` path, the phase is not end-state complete.

## 6. Non-Goals

- using `stage1` success as proof that `stage2+` owner migration is finished
- using `done-enough` wording to mean `finished`
- reopening perf merely because a phase acceptance set is green
