---
Status: Complete
Scope: CorePlan — generic loop v0.2 (in-body step normalization)
Related:
- docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md
- docs/development/current/main/design/coreplan-unknown-loop-strategy-ssot.md
- docs/development/current/main/phases/phase-29ca/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29cb: generic loop v0.2 (in-body step normalization)

## Goal

generic loop v0.1 が受理できない “途中 step” を、CorePlan の語彙を増やし過ぎず
正規化（末尾へ移動）で受理できる範囲を増やす。

重点:
- continue を増やさない（v0.2 では continue を前提にしない）
- 既定挙動/恒常ログは不変
- strict/dev は FlowBox タグで検証できる状態を維持

## Non-goals

- nested loop の受理
- in-body step が複数回出るケース
- if/else の join を伴う制御（v0.2 では禁止のまま）

## Plan

- P0: docs-first（正規化境界と gate を SSOT 化）✅
- P1: 実装（facts/normalizer + verifier/lowerer の最小拡張）✅
- P2: fixture/smoke 追加（strict/dev のタグ gate）✅
- P3: closeout（境界表の固定）✅

## Gate (SSOT)

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance boundary (v0/v0.1/v0.2)

| 形 | 受理 | 備考 |
| --- | --- | --- |
| leaf effects のみ | v0 | 既存 CoreEffectPlan の範囲 |
| ExitIf(Return/Break/Continue) | v0 | ExitKind のみ（goto禁止） |
| IfEffect (then-only, leaf-only) | v0 | else/ネスト/Joinは禁止 |
| IfEffect 末尾の ExitIf(Continue) | v0.1 | then-only、末尾のみ |
| in-body step を末尾へ正規化 | v0.2 | step 1回のみ |

## Freeze / Ok(None) 境界

| 形 | 扱い | 理由 |
| --- | --- | --- |
| nested loop / multi-entry | Freeze | unstructured |
| IfEffect の else / join / nested if | Freeze | CFG化の増殖を防ぐ |
| IfEffect 内の ExitIf(Return/Break) | Freeze | ExitIf はトップレベルのみ |
| in-body step が複数回 | Ok(None) | v0.2の範囲外 |
| in-body step + continue | Freeze | 制御が混ざるため禁止 |
| step 後の loop_var 再利用 | Freeze | 正規化が不安全 |

## Instructions

- P0: `docs/development/current/main/phases/phase-29cb/P0-INBODY-STEP-NORMALIZATION-INSTRUCTIONS.md`
- P1: `docs/development/current/main/phases/phase-29cb/P1-INBODY-STEP-NORMALIZATION-IMPLEMENTATION-INSTRUCTIONS.md`
