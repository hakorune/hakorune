---
Status: SSOT
Decision: provisional
Date: 2026-03-31
Scope: `stage0/stage1/stage2-mainline/stage2+` の実行・証跡軸、`K0/K1/K2` build/runtime stage 軸、`owner/substrate` の責務軸を分離して、phase stop-line と end-state completion の混線を防ぐ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/kernel-replacement-axis-ssot.md
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

- `stage0/stage1/stage2-mainline/stage2+` と `owner/substrate` を同じ progress bar として読まない。
- `K-axis` は `K0 / K1 / K2` build/runtime stage 軸として読み、task pack 名や `stage0/stage1/stage2-mainline/stage2+` に上書きしない。
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
| `stage2-mainline` | daily selfhost mainline / current distribution lane | target mainline |
| `stage2+` | umbrella / end-state distribution target | umbrella reading |

### 1.2 Owner axis

| Axis | Meaning | Target |
| --- | --- | --- |
| compiler authority | compiler meaning/policy owner | `.hako` |
| kernel authority | runtime/kernel meaning/policy owner | `.hako` |
| backend authority | backend daily owner | `.hako -> thin boundary` |
| substrate | bootstrap / ABI / raw memory / handle / GC / LLVM leaf | native keep unless separately retired |

### 1.3 K-axis (peer reading)

- `K-axis` is owned by `kernel-replacement-axis-ssot.md`.
- canonical reading is:
  - `K0` = all-Rust hakorune
  - `K1` = `.hako kernel` migration stage
  - `K2` = `.hako kernel` mainline / `zero-rust` daily-distribution stage
    - `K2-core` = first task pack inside `K2` (`RawArray first`)
    - `K2-wide` = second task pack inside `K2` (`RawMap second + capability widening + metal review`)
- task packs (`boundary lock`, semantic owner swap, `RawArray`, `RawMap`, capability widening, metal keep shrink) are tracked separately from both `stage` and `K-axis`.

## 2. Reading Rules

1. `stage0` keep は owner migration の失敗を意味しない。
2. `stage1` proof は `stage2-mainline` daily mainline 完了を意味しない。
3. phase の `done-enough stop line` は owner axis 上の局所 closeout であり、end-state completion とは別。
4. `kernel authority zero` は `substrate zero` ではない。
5. `buildability keep` は preservation-first で残してよいが、daily owner を逆流させてはいけない。
6. Rune のような declaration-contract layer は `.hako` compiler authority 側に属するが、substrate migration を意味しない。
7. `stage2-mainline` は `.hako` authority mainline を意味するが、native zero や Rust source zero を意味しない。
8. `stage2+` は umbrella / end-state 読みであり、daily lane 名ではない。
9. default distribution shape is `hakoruneup + self-contained release bundle`; stage axis reading と packaging shape を混線させない。
10. boundary truth belongs to `hako.abi / hako.value_repr / ownership-layout manifest`, not to `.inc` partitions.
11. collection cleanup detail belongs to owner/substrate SSOTs; this doc keeps stage vocabulary only and does not use domain-phase progress as a stage definition.
12. exact `stage1 -> stage2-mainline` entry order and the first optimization wave are owned by `stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md`; this doc owns axis vocabulary only.
13. `K-axis` belongs to `kernel-replacement-axis-ssot.md`; do not reuse `stage0/stage1/stage2-mainline/stage2+` as `K0/K1/K2` synonyms, and do not reuse task-pack nouns as stage names.
14. `stage1` proof should already push non-OS/non-substrate compiler residue toward `.hako`; a Stage2 artifact that still depends on broad Rust compiler meaning is artifact selfhost, not owner selfhost.

## 3. Current De-Rust Reading

Canonical short read:

- `stage0 keep / stage1 bridge+proof / stage2-mainline daily mainline / stage2+ umbrella`
- `K-axis` is reported separately as `K0 / K1 / K2`, not as a stage alias.

### 3.1 Stage axis now

- `stage0`: Rust bootstrap / recovery keep
- `stage1`: bridge / proof line
- `stage2-mainline`: `.hako` mainline target / daily distribution lane
- `stage2+`: umbrella / end-state distribution target

Stage1 quality bar:

- the acceptable Rust residue in `stage1` should shrink toward:
  - OS / process / file / env boundaries
  - backend / ABI / alloc / GC / kernel substrate
  - explicit compat/bootstrap keep
- parser meaning / mirbuilder meaning / canonical MIR policy / route policy should move toward `.hako` as early as possible inside the stage1 line
- therefore `stage1 -> stage2-mainline` is not just “artifact builds artifact”; it is also an owner-reduction gate

### 3.2 Owner axis now

- compiler authority: separate active lane
- kernel authority: owner-first bounded stop-line landed; current compiler semantic tables live under `runtime/meta/`
- backend authority: queued / separate lane
- substrate: Rust/C keep
- `K-axis` reading:
  - `K0` stays the all-Rust baseline / bootstrap reference
  - the current collection semantic-owner wave is a `K1 done-enough` stop-line
  - `K2-core` is the next structural task pack inside `K2`
- current kernel-side owner/substrate detail is owned by `collection-raw-substrate-contract-ssot.md` and `stage2-collection-substrate-cleanup-ssot.md`
- current stage2-mainline first-wave reading is `route/perf only`, with Rune optimization metadata still `parse/noop` and backend-active optimization deferred

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

- using `stage1` success as proof that `stage2-mainline` owner migration is finished
- using `done-enough` wording to mean `finished`
- reopening perf merely because a phase acceptance set is green
- using `stage0/stage1/stage2-mainline/stage2+` as synonyms for `K0 / K1 / K2`
- using task-pack nouns (`boundary lock`, semantic owner swap, `RawArray`, `RawMap`) as if they were the `K-axis` definitions
