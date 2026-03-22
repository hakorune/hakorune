---
Status: SSOT
Decision: provisional
Date: 2026-03-22
Scope: repo の物理構造を docs/設計の責務分離に追いつかせるための BoxShape cleanup 順序を固定する。即時の `src/mir` crate split や broad rename は扱わない。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cr/README.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
---

# Repo Physical Structure Cleanup (SSOT)

## Goal

- 設計文書の美しさを、repo の物理構造でも読める形へ寄せる。
- root / `CURRENT_TASK.md` / `src/mir` の認知負荷を下げる。
- cleanup を `BoxShape` として進め、受理形追加や broad rename と混ぜない。

## Pressure Snapshot

Local snapshot on 2026-03-22:

- `src/**/*.rs`: `1789` files / `342813` lines
- `lang/**/*.hako`: `451` files / `54853` lines
- `src/mir/**/*.rs`: `1031` files / `210851` lines
- `src/mir/builder` subdirectories: `92`

Current reading:

- 設計哲学は先に整っている
- 物理構造がまだ追いついていない
- まず必要なのは crate split ではなく、入口と衛生の整理

## Reading Rule

This wave is **BoxShape cleanup**.

Do:

- root hygiene
- entry-point thinning
- archive policy
- module/folder responsibility cleanup
- README / SSOT strengthening

Do not mix:

- new language acceptance
- runtime semantics change
- immediate `src/mir` crate split
- broad `nyash -> hako` rename

## Fixed Order

### P0. Root hygiene

Goal:

- repo root を “作業残骸置き場” ではなく “再起動入口” に戻す

Safe first buckets:

- `.gitignore` candidates:
  - `*.err`
  - `test_len_any`
- `tmp/` or scratch candidates:
  - `basic_test.hako`
  - `test.hako`
  - `test_joinir_debug.rs`
  - `test_numeric_core_phi.sh`
  - `test_simple_windows.c`
  - `test_using.nyash`
- `archive/` or doc-archive candidates:
  - `CURRENT_TASK_ARCHIVE_*.md`
  - consult / consultation zip bundles
  - completed one-off summary memos

Rule:

- root の非 allowlist 新規追加は禁止
- 一時物は `tmp/` / scratch
- 履歴物は archive

### P1. `CURRENT_TASK.md` slim

Goal:

- root pointer を cheap restart file に戻す

Keep in root:

- current blocker
- current priority
- exact next files
- reopen conditions
- recent accepted decisions only

Move out:

- long historical residue
- parked lane lore
- completed detail logs

Archive policy:

- archive when the slice is done
- archive when reopen condition is absent
- archive when SSOT/phase README pointers are enough to resume

### P2. `src/` top-level cleanup

Goal:

- flat top-level Rust scatter を減らす

Primary candidates:

- box-ish roots:
  - `box_trait.rs`
  - `box_arithmetic.rs`
  - `box_operators.rs`
  - `method_box.rs`
  - `type_box.rs`
- core-ish roots:
  - `value.rs`
  - `environment.rs`
  - `instance_v2.rs`

Rule:

- facade/re-export first
- physical move second

### P3. `src/mir` navigation-first cleanup

Goal:

- `src/mir` を crate split 前に読めるようにする

First non-destructive unit:

- strengthen `src/mir/builder/README.md`
- fix `builder/control_flow/plan/` reading order
- make the top-level map explicit:
  - `core`
  - `builder`
  - `join_ir`
  - `passes`
  - `policies`
  - `verifier`

Rule:

- entry modules and README first
- physical split later

### P4. `src/mir` physical clustering

Goal:

- giant files and local sprawl を減らす

Do:

- split oversized files
- separate helpers / tests / patterns from mixed owner files
- reduce direct deep-path reading

### P5. `src/mir` crate split preparation

Goal:

- only after P0-P4, prepare crate boundaries

Future targets:

- `hakorune-mir-core`
- `hakorune-mir-builder`
- `hakorune-mir-joinir`
- `hakorune-mir-passes`

Rule:

- do not split before the public/internal API seam is documented

### P6. Naming cleanup

Goal:

- finish `nyash -> hako` cleanup after structure is calmer

Rule:

- naming cleanup is late polish, not the first cleanup wave

## Non-Goals

- immediate `src/mir` crate split
- broad `nyash -> hako` rename
- mixing cleanup with active runtime/compiler blocker work
- turning `CURRENT_TASK.md` into a historical archive again

## First Safe Execution Unit

1. root hygiene contract
2. `CURRENT_TASK.md` archive/slim contract
3. `src/` / `src/mir` cleanup pointers

This is intentionally smaller than crate split.

## Acceptance

- a dedicated phase plan exists
- `CURRENT_TASK.md` points to it
- `10-Now.md` mentions the fixed order
- the first implementation slice is still docs/root-hygiene only
