---
Status: Active
Scope: design-first + minimal code（仕様不変）
Related:
- docs/development/current/main/phases/phase-29am/README.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
---

# Phase 29am P3: ExitMap/CoreExitPlan alignment（design + minimal code）

Date: 2025-12-29  
Status: Ready for execution  
Scope: Exit の表現を “Frag/ExitMap SSOT” に寄せる（CorePlan 側の Exit を増やさない）

## Why

CorePlan 移行を進めると、`CorePlan::Exit` をどこに置けるかが問題になる。

- body_bb は Effect-only（P1/P2 で固定）
- Exit は Control を含むため、無制限に Seq/If の中へ置くと CFG/Frag の SSOT が崩れる

したがって、Exit は “CorePlanの任意の位置” ではなく、**Frag/ExitMap が SSOT**として保持する方向へ寄せる。

## Objective

- CorePlan で `Exit` を表現する責務境界を明確化する
- “Exit = Frag/ExitMap の語彙” を前提に、CorePlan 側の Exit 乱用を防ぐ

## Non-goals

- unwind を実装する（別フェーズ）
- 既存の JoinIR/PlanFrag ルーティング変更

## Implementation（最小）

### Step 1: verifier の禁止を明文化（docs + optional stricter checks）

- `CoreExitPlan::Break/Continue` は Loop 内でのみ許可（既存 V3 を維持）
- `CoreExitPlan::Return` を loop body に入れない（V12 で禁止済み）

必要なら:
- “CorePlan::Exit は Seq/If の最後にのみ許可” を強化（V11 の運用を If/else まで徹底）

### Step 2: Normalizer/Composer の方針（design）

- Exit は `Frag.exits` / `ExitMap` に乗せる（emit_frag SSOT）
- CorePlan は “exit へ向かう条件値/edge_args” を生成するだけに寄せる

## Verification

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Commit

- `git add -A && git commit -m "phase29am(p3): align core exit with frag exitmap"`

