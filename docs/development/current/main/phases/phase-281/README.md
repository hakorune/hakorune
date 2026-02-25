# Phase 281: Composition Adoption (Pattern6/7)

Status: **P0-P3 ✅ complete (2025-12-23)**

Goal:
- Phase 280 で SSOT 化した `compose::*`（Frag 合成）を、実際の pattern（Plan line）へ段階的に適用する。
- “手組み Frag” を減らし、CFG 構築を **Frag 合成 SSOT**へ収束させる。

## SSOT References

- Frag/emit SSOT: `docs/development/current/main/design/edgecfg-fragments.md`
- Composition SSOT: `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
- Plan line SSOT（Pattern6/7）: `docs/development/current/main/phases/phase-273/README.md`
- Phase 280（positioning）: `docs/development/current/main/phases/phase-280/README.md`

## P0 完了（Pattern7）

P0 では、Pattern7（SplitScan）の “body の cond_match 分岐” を `compose::if_()` に置換した。

- 対象: `src/mir/builder/control_flow/plan/normalizer.rs`
- 方針: 最小差分（header/step は手組みのまま、body だけ compose へ）
- 受け入れ: VM/LLVM smoke が同じ exit code で PASS

完了メモ: `docs/development/current/main/phases/phase-281/P0-COMPLETION.md`

## P1: Pattern6正規形 + cleanup()設計

Status: **✅ Complete (2025-12-23)**

Pattern6 は early return（found_bb→Return）が絡むため、P1 では "どの合成語彙に落とすか" を設計で固定し、**実装は P2 に defer** した。

### Pattern6 CFG構造

**Blocks (6 total)**:
- preheader_bb: ループ入口
- header_bb: PHI (i_current)、ループ条件 (i <= bound)
- body_bb: マッチ条件 (window == needle)
- found_bb: 早期脱出 (Return i_current)
- step_bb: インクリメント (i_next = i + 1)
- after_bb: ループ出口 (not found)

**CFG Diagram**:
```
preheader_bb
    ↓ Jump
header_bb [phi: i_current ← preheader:i_init, step:i_next]
    ├─ Branch(cond_loop: i <= bound)
    │
    ├─→ body_bb (continue)
    │   ├─ Branch(cond_match: window == needle)
    │   │
    │   ├─→ found_bb (match found)
    │   │   └─ Return(i_current) ──> EXIT FUNCTION
    │   │
    │   └─→ step_bb (no match)
    │       ├─ i_next = i + 1
    │       └─ Jump → header_bb (back-edge)
    │
    └─→ after_bb (exhausted)
```

**Current Structure** (手組みFrag、P1維持):
```rust
// normalize_scan_with_init() lines 298-338
let branches = vec![
    BranchStub { from: header_bb, cond: cond_loop, then: body_bb, else: after_bb },
    BranchStub { from: body_bb, cond: cond_match, then: found_bb, else: step_bb },
];
let wires = vec![
    EdgeStub { from: step_bb, kind: Normal, target: Some(header_bb) },  // back-edge
    EdgeStub { from: found_bb, kind: Return, target: None, args: ret_found_args },
];
```

### compose::if_()が使えない理由（技術的ブロッカー）

**Pattern7 (P0 Success)**:
```
body_bb → then_bb (Normal) ┐
       → else_bb (Normal) ┘ → step_bb (両方が join に収束)
```
✅ **Symmetric exits** → compose::if_()が完璧にフィット

**Pattern6 (P1 Challenge)**:
```
body_bb → found_bb (Return) → EXIT FUNCTION (関数脱出)
       → step_bb (Normal)   → header_bb (ループ継続)
```
❌ **Asymmetric exits** → compose::if_()の契約外（Normal合流前提が壊れる）

**compose::if_()の主契約**:
- Input: then_frag/else_frag 両方が Normal exits を持つ
- Output: 両方を join_frag.entry に wire
- Pattern6: found_frag が Return exit → join に収束しない

**結論**: 無理に compose::if_() を使うと：
1. body_bb から重複 terminator（BranchStub 2個）
2. 1 block = 1 terminator 不変条件違反
3. Return exit の伝播経路が未定義

### 正規形（P2以降の目標）

**cleanup()を使った合成**:
```rust
// 将来の P2 実装イメージ
let main_loop = /* body→step normal flow */;
let early_exit = /* body→found Return */;
let combined = compose::cleanup(main_loop, early_exit);
```

**cleanup()の役割**:
- 非対称 exit（Normal + Return）を統一的に扱う
- early exit を上位 Frag の exits に伝播
- main と cleanup の境界を明示的に管理

### P1決定: cleanup()契約のみ設計（実装defer）

**Rationale**:
1. compose::if_()は Normal 合流専用（early exit 非対応）
2. cleanup()契約が SSOT 化されてない（P0では不要だった）
3. Pattern6 実装前に契約を固定する必要

**P1 Actions**:
- ✅ cleanup()の契約定義（シグネチャ、入力条件、出力保証）
- ✅ 最小実装（Fail-Fast stub）
- ✅ unit test 追加（契約確認）
- ❌ Pattern6の実コード置換（P2に defer）

**P2でやること**:
1. cleanup()本体実装（P1契約を満たす）
2. normalize_scan_with_init()をcleanup()ベースに置換
3. Pattern6 smokes維持（挙動不変）

## P2: cleanup(Return) 実装 + Pattern6 移行

Status: **✅ Complete (2025-12-23)**

cleanup(Return)の本体実装を完了し、Pattern6（normalize_scan_with_init）を `compose::cleanup()` ベースに置換した。

**実装内容**:
- `compose::cleanup()` に Return exit 伝播ロジック追加
- Pattern6 の found_bb（early return）を cleanup() で統合
- 手組み BranchStub/EdgeStub を compose API に置換

**検証結果**:
- VM smoke: `phase258_p0_index_of_string_vm.sh` ✅ PASS (exit 0)
- LLVM smoke: `phase258_p0_index_of_string_llvm_exe.sh` ✅ PASS (exit 0)
- 挙動不変（Pattern6 の early return が正常動作）

## P3: cleanup(Normal) 追加 + hand-roll ゼロ化

Status: **✅ Complete (2025-12-23)**

cleanup(Normal) を追加し、normalize_scan_with_init() の step back-edge を compose::cleanup() に統合した。これにより Pattern6 の**手組み Frag が完全にゼロ化**された。

**実装内容**:
- `compose::cleanup()` に Normal exit wiring 追加（Return と統一的に扱う）
- Pattern6 の step_bb→header_bb back-edge を cleanup() で配線
- EdgeStub 直接生成コードを全削除

**検証結果**:
- VM smoke: `phase258_p0_index_of_string_vm.sh` ✅ PASS (exit 0)
- LLVM smoke: `phase258_p0_index_of_string_llvm_exe.sh` ✅ PASS (exit 0)
- Pattern6/7 両方が compose API 100%（手組みゼロ）

**Phase 281 完全達成**: すべての Plan line パターンが Frag 合成 SSOT に収束 🎉

## Non-Goals

- 新しい env var の追加はしない
- by-name hardcode での一時しのぎはしない
- `emit_frag()` 以外で terminator を生成しない
