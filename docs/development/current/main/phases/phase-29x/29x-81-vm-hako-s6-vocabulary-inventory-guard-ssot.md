---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: X55 vm-hako S6 vocabulary inventory + guard を固定する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-80-postx53-runtime-core-sequencing-ssot.md
  - docs/development/current/main/design/de-rust-compiler-thin-rust-roadmap-ssot.md
  - tools/checks/phase29x_vm_hako_s6_vocab_allowlist.txt
  - tools/checks/phase29x_vm_hako_s6_vocab_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_vocab_guard_vm.sh
---

# 29x-81: VM-Hako S6 Vocabulary Inventory Guard (SSOT)

## 0. Goal

- vm-hako subset の語彙セット（op inventory）を allowlist で固定する。
- 語彙の増減は guard で fail-fast 検出し、X58 以降の BoxCount を 1語彙ずつ運用できる状態にする。

## 1. Contract

1. `phase29x_vm_hako_s6_vocab_allowlist.txt` は `check_vm_hako_subset_json` の op match と集合一致する。
2. allowlist にない op が subset inventory に追加された場合、guard は fail-fast する。
3. allowlist にある op が subset inventory から消えた場合、guard は fail-fast する。
4. legacy `debug_log` は inventory に再導入しない（guardで reject）。
5. `nop` 昇格は X58 の単独タスク（1語彙）として扱い、X55 には混在させない。

## 2. Evidence Command

1. `bash tools/checks/phase29x_vm_hako_s6_vocab_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_vm_hako_s6_vocab_guard_vm.sh`

## 3. Next Step

- X56 で vm-hako dual-run parity gate pack（success/reject split）を導入する。
