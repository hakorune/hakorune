---
Status: Active
Decision: provisional
Date: 2026-02-13
Scope: X46 完了後の runtime de-rust handoff を順序固定し、route drift を抑止したまま `.hako` VM 実行レーンへ戻すための実行契約。
Related:
  - docs/development/current/main/phases/phase-29x/29x-72-cache-gate-integration-done-sync-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-75-vm-route-pin-inventory-guard-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-76-vm-hako-strict-dev-replay-gate-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-77-newclosure-contract-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-78-core-cabi-delegation-inventory-guard-ssot.md
  - docs/development/current/main/design/de-rust-post-g1-runtime-plan-ssot.md
  - docs/development/current/main/design/runtime-gc-policy-and-order-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
  - docs/reference/abi/nyrt_c_abi_v0.md
---

# 29x-73: Post-X46 Runtime Handoff Sequencing (SSOT)

## 0. Conclusion

- X47-X53 を post-X46 handoff レーンとして固定する。
- 目的は「cache lane 完了後に止まらないこと」と「route drift を制御しながら `.hako` VM 実行レーンを再拡大すること」。
- GC/cycle collector はこのレーンでは扱わない（optional/last のまま維持）。

## 1. Fixed order (X47-X53)

1. X47: handoff bootstrap（docs-only）
   - `README/29x-90/29x-91/CURRENT_TASK` に X47-X53 導線を同期する。
2. X48: route pin inventory + guard
   - `NYASH_VM_HAKO_PREFER_STRICT_DEV=0` 固定点を棚卸しし、増殖を guard で停止する。
3. X49: vm-hako strict/dev replay gate
   - rust-vm pin なしの vm-hako replay gate を追加し、既存 gate と並走で安定性を観測する。
4. X50: NewClosure contract lock
   - `NewClosure` は「fail-fast 維持」または「最小 parity 実装」のどちらかを Decision 明記で固定し、silent fallback を禁止する。
5. X51: Core C ABI delegation inventory
   - runtime route/verifier/safety/lifecycle の非canonical 呼び出し混入を棚卸しし、guard で固定する。
6. X52: handoff gate integration
   - X48-X51 の guard/smoke を 1 コマンド化し、daily/milestone で再生可能にする。
7. X53: done sync + rollback lock
   - 完了判定、rollback 条件、残課題（GC optional/非目標）を docs で同期する。

## 2. Acceptance

- X47-X53 の順序が docs 上で一意に辿れる。
- route pin の追加/拡散が guard で検出できる。
- vm-hako strict/dev replay を既存 gate と分離して観測できる。
- Core C ABI の canonical boundary 逸脱が guard で fail-fast する。

## 3. Non-goals

- GC/finalizer の新規実装
- ABI レーンの追加（Core C ABI / TypeBox ABI v2 以外）
- selfhost failure-driven の通常運用導線変更（X47-X53 は拡張レーンとして並走）
