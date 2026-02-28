---
Status: Done
Decision: accepted
Date: 2026-02-28
Scope: PLG-HM2-min1 として plugin de-rust 後の Rust recovery line（Core+Wave2）を CI で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-209-plg-hm1-core8-module-provider-lock-ssot.md
  - docs/development/current/main/design/code-retirement-history-policy-ssot.md
  - .github/workflows/portability-ci.yml
  - tools/checks/phase29cc_plg_hm2_rust_recovery_line_guard.sh
  - tools/checks/dev_gate.sh
---

# 29cc-210 PLG-HM2 Core+Wave2 Rust Recovery Line Lock

## Purpose
plugin の mainline は `.hako module provider` を維持したまま、万一の rollback/portability 検証用に Rust plugin build line を CI で維持する。  
この lock は「実装の二重保持」ではなく「履歴+CI」による recovery 契約を固定する。

## Decision
1. recovery line 対象は Core+Wave2（`Array/Map/String/Console/File/Path/Math/Net`）に固定する。
2. `.github/workflows/portability-ci.yml` に plugin matrix job を追加し、`linux/windows/macos` の 3 OS で Rust plugin crate を build-check する。
3. 退役コードの repo 内 archive 保存は行わない。保存方針は `code-retirement-history-policy-ssot`（tag+commit 境界）を正本とする。
4. guard `phase29cc_plg_hm2_rust_recovery_line_guard.sh` を portability gate に接続し、workflow/docs/dev_gate の整合崩れを fail-fast で検知する。

## Acceptance
- `bash tools/checks/phase29cc_plg_hm2_rust_recovery_line_guard.sh`
- `tools/checks/dev_gate.sh portability`

## Next (fixed order)
1. `none`（fulfilled by HM2-min2/min3 completion; monitor-only）
