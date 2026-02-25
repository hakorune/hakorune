---
Status: Complete
Scope: CorePlan — unknown loop acceptance by composition (generic loop v0)
Related:
- docs/development/current/main/design/coreplan-unknown-loop-strategy-ssot.md
- docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md
- docs/development/current/main/phases/phase-29bg/README.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ca: CorePlan unknown loop acceptance (generic structured loop v0)

## Goal

「既知パターンに一致しない一般ループ」で `[joinir/freeze]` するケースを減らし、CorePlan を “小さな部品の合成” のまま表現力を上げる。

特に selfhost tooling（`hako_check`）のような“素朴な走査ループ”が止まる問題を、CorePlan 側の **generic structured loop v0** で吸える土台を作る。

## Non-goals

- CorePlan を汎用CFG命令セットにする（任意 goto 禁止）
- irreducible/multi-entry loop の受理（Freeze: unstructured）
- release 既定のログ/意味論変更

## Plan

- P0: docs-first SSOT 固定（generic loop v0 の語彙と境界）✅
- P1: 実装（ExitIf の一般化 + verifier/lowerer + facts/composer の最小配線）✅
- P2: selfhost tooling への適用方針整理（Phase 29bg との整合）✅
- P3: generic loop v0.1 continue（restricted）✅
- P4: closeout（selfhost gate / 境界SSOT / 次フェーズ候補）✅

## Selfhost bringup gate (SSOT)

- `./tools/hako_check_deadcode_smoke.sh`
- `bash tools/hako_check/run_tests.sh`

## Acceptance boundary (generic loop v0 / v0.1)

| 受理できる形 | 備考 |
| --- | --- |
| Loop body = leaf effects のみ | 既存 CoreEffectPlan の範囲内 |
| ExitIf(Return/Break/Continue) | ExitKind への脱出のみ（goto化禁止） |
| IfEffect(then-only, leaf-only) | else無し、ネスト無し |
| IfEffect の末尾に限り ExitIf(Continue) | then-only、末尾のみ |
| IfEffect 内の step は “continue直前の1回” のみ | loop_increment は then-body 末尾直前のみ許可 |

## Freeze / Ok(None) になる形（v0/v0.1 では未対応）

| 形 | 理由 |
| --- | --- |
| nested loop / multi-entry | unstructured |
| IfEffect の else / join / nested if | CFG化の増殖を防ぐ |
| IfEffect 内の ExitIf(Return/Break) | ExitIf はトップレベルのみ |
| in-body step（continue無し） | 正規化未対応（v0.2候補） |

## Next (候補を1つに絞る)

- P5: generic loop v0.2 “in-body step を末尾へ正規化”（continue無し）

## Instructions

- P0: `docs/development/current/main/phases/phase-29ca/P0-GENERIC-LOOP-V0-INSTRUCTIONS.md`
- P2: `docs/development/current/main/phases/phase-29ca/P2-SELFHOST-HAKO_CHECK-BRINGUP-INSTRUCTIONS.md`
- P3: `docs/development/current/main/phases/phase-29ca/P3-GENERIC-LOOP-V0_1-CONTINUE-INSTRUCTIONS.md`
- P4: `docs/development/current/main/phases/phase-29ca/P4-CLOSEOUT-INSTRUCTIONS.md`
