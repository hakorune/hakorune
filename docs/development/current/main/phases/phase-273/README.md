# Phase 273: Plan Extractor (Pure) + PlanLowerer SSOT

Status: ✅ P0/P1/P2/P3/P4 completed (2025-12-22, updated 2025-12-23)

Goal:
- numbered route label 列挙の裾広がりを止める。
- route family は "検出して Plan を返すだけ" に降格し、CFG/PHI/block/value の生成責務を 1 箇所に閉じ込める。
- P1: DomainPlan → CorePlan の 2層構造で "収束" を強める

Historical note:
- この phase 文書は 2025-12 時点の DomainPlan/CorePlan migration record だよ。
- current runtime mainline は recipe-first / route-first なので、下の `Pattern6/7` は historical migration label、`joinir/patterns/*` は historical path token として読む。

---

## P2 完了 (2025-12-22)

P2 では split_scan route（legacy Pattern7 label）を Plan ラインへ移行し、P1 の CorePlan を保ったまま “収束圧” を上げた。

- ✅ split_scan route: Extractor → DomainPlan → Normalizer → CorePlan → Lowerer（MIR/Frag/emit_frag）へ統一
- ✅ CoreLoopPlan: `block_effects / phis / frag / final_values` で一般化（scan_with_init / split_scan が同一 CorePlan に収束）
- ✅ CoreEffectPlan: `dst: Option<ValueId>` + `effects: EffectMask` で副作用（例: `push`）を表現可能にした
- ✅ Lowerer: “split” の知識を持たず、CorePlan のみを処理（route-agnostic 維持）

## P3/P4（2025-12-22〜2025-12-23）

- P3: legacy fallback を撤去し、generalized CoreLoopPlan を SSOT 化（構造で揺れを消す）
- P4: Plan line を current operational SSOT として文書化（導線固定）

## P1 完了 (2025-12-22)

### アーキテクチャ

```
DomainPlan (Pattern固有)
    ↓ PlanNormalizer (SSOT)
CorePlan (固定語彙 - 構造ノードのみ)
    ↓ PlanVerifier (fail-fast)
    ↓ PlanLowerer
MIR (block/value/phi)
```

### SSOT Entry Point

**Files**:
- `src/mir/builder/control_flow/plan/mod.rs` - DomainPlan/CorePlan 型定義
- `src/mir/builder/control_flow/plan/normalizer.rs` - PlanNormalizer（DomainPlan → CorePlan）
- `src/mir/builder/control_flow/plan/verifier.rs` - PlanVerifier（fail-fast 検証）
- `src/mir/builder/control_flow/plan/lowerer.rs` - PlanLowerer（CorePlan → MIR）

### 原則

- Extractor は **pure**（builder 触り厳禁、DomainPlan を返すのみ）
- Normalizer は **SSOT**（pattern 固有知識はここに集約）
- CorePlan の式は **ValueId 参照のみ**（String 禁止 → 第2の言語処理系を作らない）
- Lowerer は **pattern-agnostic**（CorePlan のみを処理）
- terminator SSOT: Frag → emit_frag()

---

## CorePlan 固定語彙

```rust
pub enum CorePlan {
    Seq(Vec<CorePlan>),
    Loop(CoreLoopPlan),
    If(CoreIfPlan),
    Effect(CoreEffectPlan),
    Exit(CoreExitPlan),
}

pub enum CoreEffectPlan {
    MethodCall { dst, object, method, args, effects },
    BinOp { dst, lhs, op, rhs },
    Compare { dst, lhs, op, rhs },
    Const { dst, value },
}
```

**増殖禁止ルール**:
- ノード種別（variant）の追加は禁止
- `EffectPlan::ScanInit` のような scan専用 variant は禁止
- データ（フィールド、パラメータ）の追加は許容

---

## P1 Implementation Summary

**Files changed** (7 total):
- Modified: `src/mir/builder/control_flow/plan/mod.rs` - DomainPlan/CorePlan 型定義 (~220 lines)
- New: `src/mir/builder/control_flow/plan/normalizer.rs` - PlanNormalizer (~290 lines)
- New: `src/mir/builder/control_flow/plan/verifier.rs` - PlanVerifier (~180 lines)
- Modified: `src/mir/builder/control_flow/plan/lowerer.rs` - CorePlan 対応 (~250 lines)
- Modified (historical path token): `src/mir/builder/control_flow/joinir/patterns/pattern6_scan_with_init.rs` - DomainPlan 返却
- Modified (current route-entry family): `src/mir/builder/control_flow/joinir/route_entry/router.rs` - Normalizer + Verifier 経由

**Regression test**:
- ✅ phase254_p0_index_of_vm.sh (fixed needle, forward scan)
- ✅ phase258_p0_index_of_string_vm.sh (dynamic needle)

---

## LLVM harness の落とし穴（Phase 258 で露出）

Phase 258 の `index_of_string`（dynamic needle）で、VM では正しいのに LLVM で `Result: 0` になるケースが露出した。
原因は Phase 273 P1 の本線（DomainPlan→CorePlan→emit_frag）ではなく、LLVM harness / AOT ランタイム側の “契約” だった。

### 1) `params` を使わないと引数が silently に潰れる

MIR JSON の `params`（ValueId の引数順）を使わず、heuristic で「未定義の use」を拾うと、
`box` フィールド等を見落とした場合に **v1 が arg0 に誤マップ**され、needle が haystack と同一扱いになる。

- Fix: `src/llvm_py/builders/function_lower.py` で `func_data["params"]` を SSOT として優先する

### 2) “raw integer vs handle” 衝突で `Result` が 0 になる

AOT ランタイム（nyrt）は `ny_main()` の返り値が **raw i64** か **handle(i64)** かを区別できない。
正しい raw 返り値（例: `6`）が、たまたま生成済みの handle id と衝突すると、IntegerBox ではないため `Result: 0` になりうる。

- Fix: `crates/nyash_kernel/src/lib.rs` の exit_code 抽出で、handle が IntegerBox 以外なら raw i64 として扱う

## References

- JoinIR SSOT overview: `docs/development/current/main/joinir-architecture-overview.md`
- Frag SSOT: `docs/development/current/main/design/edgecfg-fragments.md`
- Phase 272（scan_with_init / split_scan, Frag適用）: `docs/development/current/main/phases/phase-272/README.md`

## Instructions

- P0 Claude Code: `docs/development/current/main/phases/phase-273/P0-CLAUDE.md`
- P1 Claude Code: `docs/development/current/main/phases/phase-273/P1-CLAUDE.md`
- P2 Completion: `docs/development/current/main/phases/phase-273/P2-COMPLETION.md`
- P3 Claude Code: `docs/development/current/main/phases/phase-273/P3-CLAUDE.md`

## P3 完了 (2025-12-23)

P3 では scan_with_init route（legacy Pattern6 label）を generalized CoreLoopPlan に移行し、legacy fallback を撤去して Plan ラインの収束を完成させた。

- ✅ scan_with_init route: generalized CoreLoopPlan（`frag/block_effects/phis/final_values`）へ完全移行
- ✅ CoreLoopPlan: すべてのフィールドを必須化（`Option` 削除）
- ✅ Lowerer: `lower_loop_legacy()` 撤去、CorePlan SSOT 化（Fail-Fast）
- ✅ PlanLowerer: route 固有参照（`emit_scan_with_init_edgecfg()` 等）を完全削除
- ✅ route_loop_pattern(): Plan ラインを明示的 SSOT として文書化

### SSOT Documentation Entry Points

P3 完了により、以下が Plan ライン SSOT の入口となった：

1. **ルーティング SSOT**: `src/mir/builder/control_flow/joinir/route_entry/router.rs`
   - scan_with_init / split_scan の Plan-based entry points
   - legacy numbered route labels は traceability-only

2. **型定義 SSOT**: `src/mir/builder/control_flow/plan/mod.rs`
   - `DomainPlan { ScanWithInit, SplitScan, ... }`
   - `CorePlan { Seq, Loop, If, Effect, Exit }`
   - `CoreLoopPlan { block_effects, phis, frag, final_values }`

3. **正規化 SSOT**: `src/mir/builder/control_flow/plan/normalizer.rs`
   - Pattern 固有知識の一元管理（ScanWithInit/SplitScan normalization）

4. **検証 SSOT**: `src/mir/builder/control_flow/plan/verifier.rs`
   - fail-fast 不変条件チェック（V2-V9）

5. **降格 SSOT**: `src/mir/builder/control_flow/plan/lowerer.rs`
   - Pattern 知識なし、CorePlan のみ処理
   - emit_frag() で terminator SSOT

### P3+ Legacy Removal (2025-12-23)

P3 完了後、さらにレガシーコードを削除：

- ✅ `emit_scan_with_init_edgecfg()` 関数削除（~144 lines）
- ✅ `CoreCarrierInfo` 構造体削除（~15 lines）
- ✅ `verify_carrier()` 関数削除（~15 lines）
- ✅ 未使用 import 削除（cargo fix、~30 files）

**Total lines removed**: ~174 lines (net reduction)

---

## P4 Proposal (Documentation Finalization)

P4 では、アーキテクチャドキュメントを Plan ラインで完全更新し、「現行 SSOT」を明確に標記する：

1. **router.rs docstring 更新** ✅
   - "Phase 273 P3: Plan Line is Current SSOT for scan_with_init / split_scan" 追加
   - ルーティング戦略を明示（Plan entry points → legacy table）
   - SSOT Entry Points を列挙

2. **joinir-architecture-overview.md 更新**
   - Section 2.1.2 追加（Plan-based patterns 専用セクション）
   - Section 0 の「target shape」を「current operational shape」に更新
   - routing order diagram を Plan entry points 込みで再描画

3. **Phase 273 README.md 更新** ✅
   - P3 completion section 追加
   - P4 proposal → "Documentation Finalization"

---

## Future Work (P5+)

1. **split_scan / bool_predicate_scan / accum_const_loop DomainPlan 追加**: Split, BoolPredicate 等を DomainPlan に追加
2. **Normalizer 拡張**: 各 DomainPlan → CorePlan 変換
3. **全 route family の Plan 化**: loop_simple_while などを段階的に Plan 化
