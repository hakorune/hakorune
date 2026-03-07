# Phase 282: Router Shrinkage (pattern番号の症状ラベル化)

Status: Active SSOT / ✅ P0–P9a complete (2025-12-23)

Reading note:
- この文書の `Pattern1..9` は numbered route label の縮退ルールを説明する historical naming token だよ。
- current runtime mainline の主語は route family / extractor / plan line / joinir line で読む。

Goal:
- numbered route label（Pattern1/2/…）を “症状ラベル（テスト名）” に縮退させ、router の責務を「抽出の配線」へ収束させる。
- CFG 構築は `Frag/ExitKind` 合成 SSOT（`compose::*` + `emit_frag()`）へ一本化する。

## SSOT References

- Frag/emit SSOT: `docs/development/current/main/design/edgecfg-fragments.md`
- Composition SSOT: `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
- JoinIR overview: `docs/development/current/main/joinir-architecture-overview.md`
- Plan line（scan_with_init / split_scan; historical labels: Pattern6/7）: `docs/development/current/main/phases/phase-273/README.md`
- Composition adoption（scan_with_init / split_scan; historical labels: Pattern6/7）: `docs/development/current/main/phases/phase-281/README.md`

## Problem

pattern番号が router の分岐点として肥大化すると、以下が起きる：
- “認識（extract）” と “CFG構築（lower）” が混ざる
- 分岐の追加が雪だるま式に増える（実質、2つ以上のコンパイラを育てる）
- 収束先（SSOT）が曖昧になり、バグ修正の導線が折れる

## Target Shape (SSOT)

router の役割をここまで縮退させる：
- **やる**: Extractor の列挙、Ok(None)/Err の境界管理、Plan/JoinIR の入口選択
- **やらない**: CFG構築のディテール、terminator 生成、推測 fallback

CFG構築は以下に収束させる：
- Plan line: `Extractor → DomainPlan → Normalizer → CorePlan → Lowerer → emit_frag()`
- JoinIR line: `cf_loop → (必要な抽出) → Frag composition → emit_frag()`

## P0 (docs-only) — SSOT 固定

- router の責務/禁止事項を明文化（by-name hardcode禁止、silent fallback禁止）
- "pattern番号の意味" を SSOT として定義（テスト名/症状ラベル）
- 入口リンクを `10-Now.md` に固定

### Router Responsibilities SSOT

**Router がやること**:
1. Extraction 戦略の列挙（Plan-based, JoinIR table-based）
2. Extractor を優先順で呼び出し（Plan line → JoinIR table）
3. Ok(None)/Err の境界管理（Fail-Fast原則）
4. Entrypoint の選択（Plan line vs JoinIR line）
5. Routing 決定のログ出力（debug mode のみ、既存 trace 機構）

**Router がやらないこと**（禁止事項）:
1. CFG 構築（block 割り当て、PHI 挿入、terminator 生成）
2. route-family 固有の lowering ロジック（Normalizer/Lowerer に委譲）
3. Silent fallback（エラーは明示的に）
4. By-name hardcode（関数名マッチング禁止、debug 以外）
5. Mock path fallback（test 専用パターンの本番使用禁止）

**numbered route label = 症状ラベル**（Phase 280 SSOT positioning）:
- ✅ 正しい用途: テスト名（`loop_if_phi.hako` → Pattern3_WithIfPhi）、debug ログ
- ❌ 禁止: CFG 分岐（`if pattern == 6 then ...`）、アーキテクチャ SSOT（Frag composition が SSOT）

**Detection 戦略**（Phase 282 P3-P7 migration 完了）:
- **ExtractionBased** (全Pattern統一): extract_*() 成功 → match（SSOT 単一）
  - Pattern1-5: Phase 282 P3-P7 で ExtractionBased 移行完了
  - Pattern6/7: Phase 273 Plan-based (extract_*_plan)
  - Pattern8/9: Phase 259/270 で既に ExtractionBased
  - pattern_kind: **safety valve のみ**（O(1) perf guard、検出ロジックではない）

### Pattern Detection SSOT Table (Phase 282 P3-P7 Complete)

**全Pattern統一ルール**:
1. **SSOT = extract**: Extraction 関数が検出の唯一の真実（pattern_kind ではない）
2. **Safety valve**: pattern_kind は O(1) 早期reject のみ（検出ロジックに使わない）
3. **Re-extract**: lower() は必ず再extract して SSOT 強制（can_lower 通過を信じない）

| Pattern | SSOT Entrypoint | Safety Valve | Re-extract | Phase |
|---------|-----------------|--------------|------------|-------|
| **Pattern1** | `extractors/pattern1.rs::extract_*` | `pattern_kind==Minimal` | ✅ | P282 P3 |
| **Pattern2** | `extractors/pattern2.rs::extract_*` | `pattern_kind==Basic` | ✅ | P282 P4 |
| **Pattern3** | `extractors/pattern3.rs::extract_*` | `pattern_kind==WithIfPhi` | ✅ | P282 P5 |
| **Pattern4** | `extractors/pattern4.rs::extract_*` | `pattern_kind==Carrier` | ✅ | P282 P6 |
| **Pattern5** | `extractors/pattern5.rs::extract_*` | `pattern_kind==InfiniteEarlyExit` | ✅ | P282 P7 |
| **Pattern6** | `pattern6_scan_with_init.rs::extract_scan_with_init_plan()` | (none, Plan-based) | ✅ | P273 P1 |
| **Pattern7** | `pattern7_split_scan.rs::extract_split_scan_plan()` | (none, Plan-based) | ✅ | P273 P2 |
| **Pattern8** | `pattern8_scan_bool_predicate.rs` (can_lower) | (none, ExtractionBased) | ✅ | P259 |
| **Pattern9** | `pattern9_accum_const_loop.rs` (can_lower) | (none, ExtractionBased) | ✅ | P270 |

**pattern_kind の正しい使い方**:
- ✅ **O(1) guard**: `if pattern_kind != Expected { return false }` (perf最適化)
- ✅ **Debug logging**: `pattern_kind={:?}` (診断情報)
- ❌ **検出ロジック**: `if pattern_kind == X then lower_X()` (禁止、SSOT違反)
- ❌ **CFG 分岐**: `match pattern_kind { ... }` (禁止、extract が SSOT)

**移行完了状態** (Phase 282 P8):
- すべての Pattern が extract_*() を SSOT として使用
- pattern_kind は早期reject の補助にすぎない
- lower() は必ず re-extract して SSOT 強制（二重検証）

**SSOT 参照**:
- Frag composition: `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
- Plan line: `src/mir/builder/control_flow/plan/normalizer.rs`
- Terminator 生成: `emit_frag()` (Phase 267)

### Ok(None)/Err Boundary Rules

**Fail-Fast 原則**:
- `Ok(None)`: Extraction がパターンに一致しなかった（次を試す）
- `Err(String)`: Extraction が一致したが検証失敗（即座に fail）
- **禁止**: Close-but-unsupported → Ok(None)（Err で返すべき）

**例**:

```rust
// ✅ 正しい: 一致しない
extract_scan_with_init_plan() → Ok(None)  // 次のパターンを試す

// ✅ 正しい: 一致したが非サポート
extract_scan_with_init_plan() → Err("P1 scope: reverse scan not supported")

// ❌ 禁止: Silent fallback
extract_scan_with_init_plan() → Ok(None) for unsupported cases
// （非サポートケースを Ok(None) で返してはいけない）
```

**境界の位置**（router.rs 実装）:
- Plan line（lines 310-337, 341-368）: extract → match → Normalize → Verify → Lower
- JoinIR table（lines 376-382）: can_lower → lower
- No match（lines 384-395）: Ok(None) を caller に返す（Err ではない）

### Entrypoint Table

| Entrypoint | Entry Condition | Patterns | SSOT Downstream |
|------------|----------------|----------|-----------------|
| **Plan line** | extract_*_plan() が Ok(Some) | Pattern6/7 | Normalizer → CorePlan → Lowerer → emit_frag |
| **JoinIR table** | LOOP_PATTERNS 反復 | Pattern1-5,8-9 | cf_loop 抽出 → Frag composition → emit_frag |
| **No match** | すべての extract が Ok(None) | (none) | Err を caller に返す |

**Plan line の責務**（Phase 273 SSOT）:
1. **Extraction**: extract_*_plan()（pure、builder 不要）
2. **Normalization**: PlanNormalizer（pattern 知識の展開）
3. **Verification**: PlanVerifier（fail-fast 検証）
4. **Lowering**: PlanLowerer（block/value 割り当て + Frag composition）
5. **Emission**: emit_frag()（terminator SSOT）

**JoinIR table の責務**（Phase 194+ table-driven）:
1. **Detection**: can_lower()（pattern により structure-based / extraction-based が混在）
2. **Extraction**: cf_loop 抽出
3. **収束先**: Frag composition → emit_frag（内部パイプライン詳細は最小化）

**収束先の統一**（Phase 282 Goal）:
- すべての entrypoint が最終的に `emit_frag()` に収束（terminator 生成の SSOT）
- CFG 構築の詳細は router が持たない（抽出の配線のみ）

## P1 (code, minimal) — 配線の可視化

## Follow-up (design-first): Return as ExitKind SSOT

移行期間中に一番 “ズレやすい” のは `return` まわりなので、pattern 個別実装へ散らさず、
`ExitKind` + `compose::*` / `emit_frag()` に収束させる方針を別フェーズで扱う。

- 入口: `docs/development/current/main/30-Backlog.md`（Phase 284）

## P9a (refactor, behavior-preserving) — Extractor 重複削減（Common helpers）

Pattern1–5 の extractor が持っていた “再帰カウント/検出/条件ヘルパ” の重複を、pure helper に集約する。

- 追加: `src/mir/builder/control_flow/plan/extractors/common_helpers.rs`
  - historical path token: `extractors/common_helpers.rs` under the old `joinir/patterns/` lane
- 方針:
  - **SSOT=extract** は維持（判定は各 pattern extractor の責務）
  - helper は pure（builder 触らない）・**silent fallback を作らない**
  - Pattern3 の “if-else PHI” は専用ロジックが多いため、段階的に扱う（P9b 以降）

- router に “経路ログ” を追加（既定OFF、debugのみに限定）
- pattern番号ではなく “entrypoint（Plan/JoinIR）” をログの主語にする

## Acceptance (Phase 282)

- router の責務が docs で SSOT 化されている
- router の変更が「extractor配線」のみになっている（CFG構築の詳細を持たない）
- 既存の VM/LLVM smokes に退行がない
