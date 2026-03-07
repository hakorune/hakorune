# Phase 284: Return as ExitKind SSOT（route familyに散らさない）

Status: P1 Complete (2025-12-23)

## Goal

`return` を “pattern 個別の特例” として増やさず、`ExitKind::Return` と `compose::*` / `emit_frag()` に収束させる。
移行期間中の検出穴（Ok(None) による黙殺）を消し、Fail-Fast を構造で担保する。

## SSOT References

- Frag/ExitKind 設計: `docs/development/current/main/design/edgecfg-fragments.md`
- Composition API: `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
- Terminator emission: `src/mir/builder/control_flow/edgecfg/api/emit.rs`（`emit_frag()`）
- Router SSOT（SSOT=extract / safety valve）: `docs/development/current/main/phases/phase-282/README.md`

## Problem（移行期間の弱さ）

- route family 単位で `return` を “未対応” にすると、検出戦略（Ok(None)/Err）次第で **静かに別経路へ落ちる**。
- その結果、同じソースでも「どの lowerer が `return` を解釈したか」が曖昧になり、SSOT が割れる。

## Core SSOT（決めること）

### 1) 返り値の意味（ExitKind）

- `return expr` は `ExitKind::Return` として表現する。
- 返り値（ValueId）は `EdgeArgs` で運ぶ（Return edge が value を持つ）。
- Return は **必ず emit 側で terminator になる**（pattern 側で命令を直に生成しない）。

### 2) Detect の境界（Ok(None) / Err）

- `Ok(None)`: 一致しない（次の extractor へ）
- `Err(...)`: 一致したが未対応（close-but-unsupported）→ **Fail-Fast**

Phase 284 の完了条件は「`return` を含むケースが close-but-unsupported ではなく SSOT 経路で処理される」状態に寄せること。

### 3) 実装の集約点（どこに寄せるか）

- `return` の lowering は **ExitKind + compose + emit_frag** に集約する。
- pattern の extractor は “認識” のみ（SSOT=extract）。`return` の解釈ロジックを増やさない。

補足: Phase 284 は “return だけのため” ではない。ここで固定するのは **Exit 正規化**（ExitKind の語彙化）で、
`return/break/continue/(将来の unwind)` を同じ土台に載せるのが狙い。
「Jump/Branch の配線で exit を表現できる」状態ができると、return はその一例として自然に入る。

## Responsibility Map（迷子防止）

このフェーズで一番起きやすい事故は「`return` をどこで処理するべきか分からず、pattern 側へ散布してしまう」こと。
そこで、**どの経路で lower されるか**を前提に責務を固定する。

### A) Plan line（scan_with_init / split_scan; historical labels: Pattern6/7）

- 入口: `src/mir/builder/control_flow/joinir/route_entry/router.rs`（route=plan）
  - historical path token: `src/mir/builder/control_flow/joinir/patterns/router.rs`
- SSOT:
  - `src/mir/builder/control_flow/plan/normalizer/mod.rs`（Frag 構築: branches/wires/exits）
  - `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`（合成 SSOT）
  - `src/mir/builder/control_flow/edgecfg/api/emit.rs`（`emit_frag()` terminator SSOT）
- ここでは `return` を **Return edge（ExitKind::Return）**として組み立てるのが自然。

### B) JoinIR line（historical numbered labels: Pattern1–5,9）

- 入口: `src/mir/builder/control_flow/joinir/route_entry/router.rs`（route=joinir）
  - same historical route-entry lane as above (`router.rs`)
- SSOT:
  - JoinIR 生成（pattern 固有の JoinIR lowerer）
  - `src/mir/builder/control_flow/plan/conversion_pipeline.rs`（JoinIR→MIR→merge の current single entry）
    - historical joinir/patterns lane token: `conversion_pipeline.rs`
  - `src/mir/builder/control_flow/joinir/merge/mod.rs`（Return merge / exit block SSOT）
- **注意**: `src/mir/builder/control_flow/plan/normalizer/mod.rs` は Plan line 専用なので、
  Pattern4/5 の return 問題の root fix をここへ寄せても効かない。

### 禁止事項（Phase 284 の憲法）

- ❌ historical `Pattern4/5` の `lower()` へ「return を特別扱いする if」を散布しない（SSOTが割れる）
- ❌ Extractor が `return` を見つけた時に `Ok(None)` で黙殺しない（silent reroute 禁止）
- ✅ `return` の “対応/非対応” は **共通入口の Fail-Fast**で固定する（P1 で実装）

## Scope

### P0（docs-only） ✅ COMPLETE

- `return` を ExitKind として扱う SSOT を文章で固定する（本ファイル + 参照先リンク）。
- 移行期間のルール（Ok(None)/Err の境界、黙殺禁止）を Phase 282 と整合させる。

### P1（code） ✅ COMPLETE (2025-12-23)

**実装完了内容**:
1. **return_collector.rs** - Return statement detection SSOT (既存)
2. **return_jump_emitter.rs** - Return jump emission helper (Pattern4/5 reuse) ⭐NEW
3. **block_remapper.rs** - Block ID remap SSOT (Phase 284 P1 Fix) ⭐NEW
4. **Loop refactoring** - loop_with_continue_minimal.rs simplified (~100 lines removed)
5. **Instruction/terminator updates** - Use block_remapper SSOT

**コード品質向上**:
- Return handling: ~100 lines inline code → 1 function call
- Block remapping: Duplicate logic → SSOT function
- Future reusability: Pattern5 can now reuse return_jump_emitter

### P2（smoke 固定） ✅ COMPLETE (2025-12-26)

**目的**: return を含む loop を VM/LLVM 両方で同一結果にし、integration smoke で固定。

**対象 fixture（既存再利用優先）**:
- `apps/tests/phase286_pattern5_return_min.hako` (exit 7) - Return-in-infinite-loop

**smoke スクリプト**:
- `tools/smokes/v2/profiles/integration/apps/archive/phase284_p2_return_in_loop_vm.sh` (VM)
- `tools/smokes/v2/profiles/integration/apps/archive/phase284_p2_return_in_loop_llvm.sh` (LLVM harness)

**受け入れ条件**:
- integration（VM）PASS
- integration（LLVM harness）PASS（または理由付き段階完了）
- quick 154/154 PASS 維持

**詳細手順**: `P2-INSTRUCTIONS.md`

### P3+（将来）

- 他の return パターン（Pattern8 等）の smoke 追加
- LLVM AOT 経路での return 検証

## Acceptance

- P0: `return` の SSOT（ExitKind/compose/emit）と detect 境界が明文化されている
- P1+: `return` を含む loop fixture が VM/LLVM で同一結果になり、smoke で固定されている

## P1 の実装方針（design-first 注記）

P1 の root fix は「PlanNormalizer へ寄せる」ではなく、**JoinIR line の共通入口**へ寄せる：

- 入口候補:
  - `src/mir/builder/control_flow/plan/conversion_pipeline.rs`（current single entry; historical joinir/patterns lane: `conversion_pipeline.rs`）
  - もしくは JoinIR lowerer 側に “Return collector” を 1 箇所だけ作り、Pattern4/5 はそれを呼ぶだけにする

どちらにしても、目的は同じ：
- pattern 側へロジックを増やさず（散布しない）
- `ExitKind::Return` へ収束させる（MIR では Return 終端として生成される）
