# EdgeCFG Flow Fragments（Frag / ExitKind）— Structured→CFG lowering SSOT

Status: Active SSOT
Last updated: 2025-12-23
Phase: 280 (Composition SSOT Positioning)

Related:
- North star（CFG/ABI）: `docs/development/current/main/design/join-explicit-cfg-construction.md`
- Catch/Cleanup/Async: `docs/development/current/main/design/exception-cleanup-async.md`

## 目的（なぜ必要？）

EdgeCFG（block-parameterized CFG / edge-args SSOT）が固まると、次に残る “泥沼” はここだけになる:

- **構造化制御（if/loop + catch/cleanup）→ CFG** の lowering で起きる **exit 配線問題**
- 「pattern番号で推測分岐」が増殖しやすい領域（長期的には消したい）

この文書は「pattern番号の列挙」を設計の中心にしないために、Structured→CFG の lowering を
**合成代数（fragment composition）**として SSOT 化する。

結論（本書の北極星）:

- “分岐の中心” は pattern番号ではなく **ExitKind** と **Frag（fragment）** に置く
- 値の合流は EdgeCFG の **block params + edge-args** で表し、PHI/推測/メタに逃げない
- pattern は「Extractor（形の認識）/ Plan（最小要件の抽出）」までに縮退し、merge/配線層へ逆流させない

## “フロー” は 2 層ある（混ぜると崩れる）

1. **CFG層（EdgeCFG / plumbing）**
   - terminator 語彙: `Jump/Branch/Return/Invoke`
   - edge-args: terminator operand が SSOT
   - out_edges の参照点が SSOT（複数 edge 前提）

2. **Structured→CFG lowering 層（flow composition）**
   - `if/loop/catch/cleanup/seq` を “Frag の合成” として書く
   - 難しさの本体は **exit（脱出）の種類** と **ネスト** と **合流**

## コア概念（最小の強い箱）

### ExitKind（脱出の種類を一次概念にする）

最低限の ExitKind:

- `Normal`（fallthrough）
- `Break(loop_id)` / `Continue(loop_id)`
- `Return`
- `Unwind`（Invoke.err / catch へ）
- `Cancel`（async の drop/cancel 用。今は予約）

### EdgeStub（未配線の脱出エッジ）

“どこへ飛ぶべきか未確定” な edge を表す。最終的に EdgeCFG の terminator edge に落ちる。

例（概念）:

- `from: BlockId`
- `kind: ExitKind`
- `args: EdgeArgs`（ターゲット params に対応する値。target が未確定でも “役割” はここで決める）

### Frag（fragment）

```text
Frag = { entry_block, exits: Map<ExitKind, Vec<EdgeStub>> }
```

- `entry_block`: 断片の入口
- `exits`: 断片から外へ出る未配線 edge の集合

## 合成則（pattern列挙を写像へ落とす）

### seq(a, b)

- `a.exits[Normal]` を `b.entry` へ接続する（edge-args を必要に応じて写像）
- それ以外の exit は上位へ伝搬する

### if(cond, t, e)

- header に `Branch(cond, t.entry, e.entry)` を置く
- `t.Normal` と `e.Normal` は join へ集める（必要なら join block params を作る）
- `Break/Continue/Return/Unwind` は上位へ伝搬

### loop(body)

- header / latch / after を組み、`Continue` を header に戻す
- `Break` を after へ出す
- `Return/Unwind` は上位へ伝搬

### cleanup(body, cleanup_block)（finally の後継）

狙い: “脱出 edge 正規化”

- body の全 exit（Normal/Break/Continue/Return/Unwind/Cancel）を cleanup 経由へリライトする
- cleanup 後に “元の exit” を再発射する（ExitTag + payload を block params で運ぶ）

重要: 例外 edge（Invoke.err）も漏れなく cleanup に寄せる。

## pattern は最終的に消える？（設計としての答え）

消える（実装の中心概念から降格する）。

- pattern番号は **回帰テスト名/症状ラベル**としては残して良い
- 実装の中心は `Frag/ExitKind/join(block params)` の合成則になる
- 各 pattern 実装は "Extractor（形の認識）→ Frag 合成呼び出し" の薄い層へ縮退する

---

## Composition SSOT (Phase 280)

**Status**: Active SSOT
**Purpose**: Pattern number absorption destination
**Date**: 2025-12-23

### Why Composition is SSOT

Pattern numbers (1-9+) are **symptom labels** for regression tests, NOT architectural concepts.
The architectural SSOT is **Frag composition rules** (`seq`/`if`/`loop`/`cleanup`).

**Upstream (Extractor/Normalizer)**: Finish "shape recognition" and extract pattern-specific knowledge
**Downstream (Composition)**: Use Frag composition rules to build CFG converging to SSOT
**Terminator Generation**: `emit_frag()` as sole SSOT (Phase 267 P0)

### Composition Input/Output Contract

- **Input**: `Frag` (entry + exits + wires + branches)
- **Output**: `Frag` (new entry + merged exits + merged wires + merged branches)
- **Guarantee**: Composition preserves invariants (`verify_frag_invariants_strict`)
- **No Allocation**: Caller (Normalizer) allocates `BasicBlockId`/`ValueId`
- **Pure Transform**: Composition rearranges `exits`/`wires`/`branches` only

### Composition Rules (Canonical Operations)

#### `seq(a, b)`: Sequential composition

**Composition Law**:
- `a.Normal` exits → `wires` (target = `Some(b.entry)`)
- Non-Normal exits (Return/Break/Continue/Unwind) → propagate upward (`exits`)
- Result: `seq.entry = a.entry`, `seq.exits = a.non-Normal + b.all`

**Contract**:
- **Caller allocates**: `b.entry` (`BasicBlockId`)
- **Composition wires**: `a.Normal` → `b.entry`

#### `if_(header, cond, t, e, join_frag)`: Conditional composition

**Composition Law**:
- `header` → `t.entry`/`e.entry` (`BranchStub` → `branches`)
- `t/e.Normal` → `join_frag.entry` (`EdgeStub` → `wires`)
- Non-Normal exits → propagate upward (`exits`)
- Result: `if.entry = header`, `if.exits = t/e.non-Normal + join_frag.all`

**Contract**:
- **Caller allocates**: `header`, `t.entry`, `e.entry`, `join_frag.entry` (`BasicBlockId`), `cond` (`ValueId`)
- **Caller provides**: `then_entry_args`, `else_entry_args` (`EdgeArgs`) - Phase 268 P1 SSOT
- **Composition wires**: `t/e.Normal` → `join_frag.entry`

#### `loop_(loop_id, header, after, body)`: Loop composition

**Composition Law**:
- `Continue(loop_id)` → `header` (`EdgeStub` → `wires`)
- `Break(loop_id)` → `after` (`EdgeStub` → `wires`)
- Normal/Return/Unwind → propagate upward (`exits`)
- Result: `loop.entry = header`, `loop.exits = Normal/Return/Unwind only` (no Break/Continue)

**Contract**:
- **Caller allocates**: `loop_id` (`LoopId`), `header`, `after` (`BasicBlockId`)
- **Composition wires**: `Continue(loop_id)` → `header`, `Break(loop_id)` → `after`

#### `cleanup(body, cleanup_block)`: Cleanup composition (TODO: Phase 280+)

**Planned Composition Law** (Future):
- All exits (Normal/Break/Continue/Return/Unwind) → `cleanup` (`EdgeStub` → `wires`)
- Cleanup re-dispatches original exit (`ExitTag` + payload via block params)
- `Invoke.err` also routed through cleanup

**Status**: Signature fixed, implementation TODO (Phase 280+)

---

## Composition Laws (Invariants)

### Wires/Exits Separation (Phase 265 P2)

**Invariants**:
- **`wires`**: `target = Some(...)` only (internal wiring, resolved)
- **`exits`**: `target = None` only (external exit, unresolved)
- **Exception**: `Return` may have `target = None` in `wires` (`emit_wires` ignores it)

**Why separate?**
- Prevents resolved wiring from being re-wired in next composition
- Makes composition semantics clear: `wires` = done, `exits` = propagate upward

**Fail-Fast Enforcement**:
- `verify_frag_invariants_strict()` checks `wires`/`exits` separation
- Normal/Break/Continue/Unwind require `target = Some` in `wires`
- `Return` allows `target = None` (meaningless for `Return`)

### Terminator Uniqueness (Phase 267 P0)

**Invariant**: 1 block = 1 terminator

**Enforcement** (`emit_frag`):
- From grouping: ensures 1 block = 1 terminator
- Same block cannot have both `wire` and `branch`
- Same block cannot have multiple `wires` (from-grouping detects violation)

### Entry Consistency

**Invariants**:
- `Frag.entry` must be a valid `BasicBlockId`
- Composition preserves entry validity
- Entry points to first block in composed CFG fragment

---

## Fail-Fast Invariants (Phase 266+)

### Pre-emission Verification (Two Levels)

#### `verify_frag_invariants()` (warning-only)

- **Purpose**: Legacy compatibility mode
- **Behavior**: Logs warnings but doesn't fail
- **Usage**: Used by existing code during migration

#### `verify_frag_invariants_strict()` (Err on violation)

- **Purpose**: New code enforcement
- **Behavior**: Returns `Err` on invariant violation
- **Usage**: Called by `emit_frag()` automatically
- **Enforces**: wires/exits separation, target constraints

**Invariants checked**:
1. **Wires/Exits Separation**:
   - wires have `target = Some` (except Return)
   - exits have `target = None`
2. **Target Validity**:
   - Normal/Break/Continue/Unwind require `target = Some` in wires
   - Return allows `target = None`

### Emission-time Verification (`emit_frag` SSOT)

**`emit_frag()` responsibilities** (Phase 267 P0):
1. Call `verify_frag_invariants_strict()` before emission
2. Detect `target = None` violations (except Return)
3. Enforce 1 block = 1 terminator (from-grouping)
4. Detect wire/branch conflicts (same block)

**Terminator emission**:
- `wires` → Jump/Return terminators (`emit_wires` internally)
- `branches` → Branch terminators (`set_branch_with_edge_args`)
- Phase 260 terminator API (SSOT): `set_jump_with_edge_args`, `set_branch_with_edge_args`

### Composition-side Invariants

**Assumptions**:
- Composition functions **assume** input `Frag` is valid
- Composition **preserves** validity (output passes `verify_frag_invariants_strict`)
- Caller (Normalizer) responsible for initial `Frag` validity

---

## Ownership (Who Allocates What)

### Allocator Responsibilities (3-tier)

#### Tier 1: Normalizer (Pattern-Specific)

**Responsibilities**:
- Allocates `BasicBlockId` (`builder.next_block_id()`)
- Allocates `ValueId` (`builder.next_value_id()`)
- Knows pattern semantics (scan, split, etc.)
- Constructs initial `Frag` with valid blocks/values

**Example** (Pattern6 ScanWithInit):
```rust
let header_bb = builder.next_block_id();
let body_bb = builder.next_block_id();
let step_bb = builder.next_block_id();
// ... allocate all blocks upfront
```

#### Tier 2: Composition API (Pattern-Agnostic)

**Responsibilities**:
- Receives pre-allocated `BasicBlockId`/`ValueId`
- Rearranges `exits`/`wires`/`branches`
- Pure CFG transformation (no allocation, no semantics)

**Example** (compose::seq):
```rust
pub fn seq(a: Frag, b: Frag) -> Frag {
    // Assume a.entry, b.entry are pre-allocated
    // Just rearrange exits/wires
}
```

**Why no allocation?**
- Separation of concerns: allocation (pattern-specific) vs wiring (generic)
- Composability: composition functions can be called in any order without ID conflicts
- Testability: composition can be tested with fixed IDs (deterministic)

#### Tier 3: Lowerer (MIR Emission)

**Responsibilities**:
- Calls `emit_frag()` to generate MIR terminators
- Uses Phase 260 terminator API (`set_jump_with_edge_args`, `set_branch_with_edge_args`)
- No allocation, no CFG construction

**Example** (PlanLowerer::lower_loop):
```rust
emit_frag(func, &loop_plan.frag)?;  // Emits all terminators
```

### Ownership Flow Diagram

```
Pattern6/7 Normalizer
    ↓ (allocate blocks/values)
DomainPlan → CorePlan
    ↓ (pre-allocated IDs)
Composition API (seq/if/loop)
    ↓ (rearranged Frag)
PlanLowerer
    ↓ (emit_frag)
MIR Terminator Instructions
```

---

## 実装の入口（SSOT API を先に作る）

目的: “どこを触ればいいか” を 1 箇所に固定し、推測・部分続行・場当たり分岐を減らす。

推奨の入口:

- `EdgeCFG` の plumbing API（既存）: `BasicBlock::out_edges()` 等
- Structured→CFG の入口 API（新規）: `Frag` / `ExitKind` / `compose::{seq, if_, loop_ , cleanup}` 等

物理配置（案）:

- `src/mir/builder/control_flow/edgecfg/api/`（または `.../joinir/api/` に併設してもよい）
  - `frag.rs` / `exit_kind.rs` / `compose.rs` / `patch.rs`

## verify（Fail-Fast の置き場所）

- **NormalizeBox 直後**: terminator 語彙固定・edge-args 長さ一致・cond付きJump禁止など “意味SSOT” を確定
- **merge直前**: boundary/ABI/edge-args の矛盾を即死させ “配線SSOT” を確定
- **--verify**: PHI predecessor / CFG cache 整合 / edge-args の長さ一致を常設
- **Pattern6/7 extractor**: 形は近いが契約違反のケースは `Ok(None)` で流さず freeze（例: SplitScan の `else i = i + 1` 破り、ScanWithInit の `i = i + 1` 破り）

## 直近の導入ステップ（最小で始める）

1. `Frag/ExitKind/EdgeStub` の型を追加（docs+code 入口 SSOT）
2. `seq/if/loop` の合成だけ実装（cleanup/Invoke は後段）
3. 既存 pattern のうち 1 本だけ `Frag` 合成に寄せる（Pattern8 推奨）
4. 2 本目で再利用が見えたら "pattern番号での枝刈り" を削って合成側へ寄せる

## Loop に関する注意（JoinIR-only）

- `cf_loop` は JoinIR-only（Hard Freeze）。EdgeCFG の “loop 直適用” を急いで別経路に生やすと SSOT が割れる。
- loop の EdgeCFG 化は、まず **BasicBlockId 層で持っている箇所（Phase 268 の if_form のような場所）**から適用を進める。
- JoinIR 側の loop は Phase 270 で **fixture/smoke による SSOT 固定**を先に行い、壊れたら最小差分で直す。

## compose SSOT（使い分けガイド）

- `compose::if_`: then/else が **Normal exit** で step/merge に合流する形（Pattern7 split scan の if/else）
- `compose::cleanup`: main frag に cleanup frag（Return/Normal）を合成して早期離脱を明示する形（Pattern6 scan-with-init）
- どちらも **Frag の validity を前提** にし、Normalizer が入口で妥当性を保証する
- Pattern6/7 contract SSOT: `docs/development/current/main/design/pattern6-7-contracts.md`

補足（Phase 270）:
- Pattern1（simple_while_minimal）は test-only stub のため、一般ループの “基準” には使えない。
- Phase 270 では “最小の固定形” を Pattern9（AccumConstLoop）として追加し、後で Frag 合成側へ吸収される前提で橋渡しにする。

## Bridge patterns（撤去条件SSOT）

ここで言う “bridge pattern” は、既存の JoinIR ルートを壊さずに **最小の固定形を先に通す**ための一時パターン。
（例: Phase 270 の `Pattern9_AccumConstLoop`）

- 原則:
  - bridge pattern は **汎用化しない**（固定形SSOT + fixture/smoke で仕様を固定するだけ）。
  - 将来は `Frag/ExitKind` 合成側へ **吸収して削除**する前提で追加する。

### Bridge contract（テンプレ / SSOT）

bridge pattern を追加する場合は、最低限この “撤去条件” を先に書く（書けないなら追加しない）。

- **固定する fixture/smoke（SSOT）**
  - fixture（最小）と smoke（integration）を必ず紐づける
  - 「何が通れば撤去できるか」を machine-checkable にする
- **置換先（吸収先）の SSOT がある**
  - Pattern番号列挙の反対側に、必ず “吸収先” を書く（例: `Frag/ExitKind` 合成、もしくは emission 入口）
  - 吸収先が未確定な場合でも “層” は確定させる（pattern層にロジックを増やさない）
- **撤去条件（最低限）**
  1. 置換先（吸収先）で同じ fixture/smoke が PASS する
  2. bridge pattern 依存の分岐が router から消せる（最小差分で削除できる）
  3. quick/integration の FAIL 位置が悪化しない（既知Failは増やさない）
- **撤去手順（最小）**
  - router から bridge pattern を外す
  - fixture/smoke（+ quick）で PASS 維持
  - ファイル削除（または historical へ隔離）し、SSOT から参照を外す

### Phase 271: `Pattern9_AccumConstLoop` 撤去条件（SSOT）

Phase 270 の “JoinIR-only minimal loop” を通すための橋渡し。将来は Frag 合成側へ吸収して削除する。

- **固定 fixture/smoke**
  - fixture: `apps/tests/phase270_p0_loop_min_const.hako`（exit=3）
  - smoke: `tools/smokes/v2/profiles/integration/apps/phase270_p0_loop_min_const_vm.sh`
- **吸収先（層）**
  - Structured→CFG lowering 層（`Frag/ExitKind` 合成）またはその emission 入口（pattern層は extractor に縮退）
- **撤去条件**
  1. 上記 fixture/smoke が、bridge pattern を使わない経路で PASS する（Frag/emit_frag 側で loop を構築できる）
  2. Pattern9 が router から削除されても coverage が落ちない（同 fixture が同じルートで通る）
  3. `tools/smokes/v2/run.sh --profile quick` が悪化しない
- **撤去手順**
  - Pattern9 の router 分岐を削除 → smoke PASS → Pattern9 実装を削除（または historical 化）

## 実装入口（コード SSOT）

**Phase 280: Composition SSOT Positioning Complete (2025-12-23)**

- **Status**: Active SSOT
- **Documentation**: Full composition SSOT positioning (5 sections above)
- **Implementation**: Composition API exists and tested (seq/if/loop)
- **Pattern Preparation**: Pattern6/7 hand-rolled locations documented for Phase 281 migration

**Phase 264 (歴史/別案件)**: Entry API Creation (別スコープ: BundleResolver loop fix)

- Note: Phase 264 は別案件（BundleResolver loop pattern fix）
- Composition API 入口作成は Phase 264 で完了したが、SSOT positioning は Phase 280 で確立
- 物理配置: `src/mir/builder/control_flow/edgecfg/api/`
- コア型: `ExitKind`, `EdgeStub`, `Frag`
- 合成関数: `seq`, `if_`, `loop_`, `cleanup`（シグネチャのみ、中身TODO）
- 検証: `verify_frag_invariants`（空実装）

**Phase 265 P0 で最小実装完了**

- `compose::loop_()`: exit集合の分類実装（配線なし、P1以降）
- `verify_frag_invariants()`: 最小検証追加（デバッグガード付き）
- Pattern8適用: P0ではやらない（偽Frag回避、P1から実戦投入）

**Phase 265 P1: 配線ロジック実装完了**

**目的**: Frag/ExitKind が BasicBlockId 層で配線できることを証明

**実装完了内容**:
- EdgeStub に `target: Option<BasicBlockId>` 追加
- compose::loop_() が Continue → header, Break → after への配線を実行
- verify_frag_invariants() が配線契約を検証（デバッグモード）
- test-only PoC で配線の実証完了（5個のテスト）

**配線契約**:
- Continue(loop_id) の EdgeStub.target = Some(header)
- Break(loop_id) の EdgeStub.target = Some(after)
- Normal/Return/Unwind の EdgeStub.target = None（上位へ伝搬）

**Phase 265 P1 のスコープ**:
- ✅ Frag 層での配線ロジック
- ✅ BasicBlockId 層でのテスト証明
- ❌ MIR 命令生成（Phase 266+）
- ❌ NormalizedShadow/JoinIR層への適用（Phase 266+、JoinIR-VM Bridge 改修後）

**Phase 265 P2 完了！（2025-12-21）**

**実装完了内容**:
- ✅ Frag に `wires: Vec<EdgeStub>` フィールド追加
- ✅ wires/exits 分離設計確立
  - **exits**: target = None のみ（未配線、外へ出る exit）
  - **wires**: target = Some(...) のみ（配線済み、内部配線）
- ✅ loop_() を wires 対応に更新（Break/Continue → wires）
- ✅ seq(a, b) 実装完了（a.Normal → wires）
- ✅ if_(header, cond, t, e, join_frag) 実装完了（t/e.Normal → wires）
- ✅ verify_frag_invariants() に wires/exits 分離契約追加（警告のみ）
- ✅ 全テスト PASS（13個: frag 3個 + compose 9個 + verify 1個）

**設計判断の記録**:

1. **なぜ wires/exits を分離するか？**
   - 問題: 解決済み配線と未解決 exit を混ぜると、次の合成で内部配線が再度配線対象になる
   - 決定: wires/exits を分離し、不変条件を強化
   - 理由: 合成の意味が素直になり、Phase 266 で wires を MIR terminator に落とすだけ

2. **なぜ if_ は join_frag を受け取るか？**
   - 問題: join: BasicBlockId だと、if の Normal exit が「join block」か「join 以降」か曖昧
   - 決定: join_frag: Frag を受け取る
   - 理由: if の Normal exit = join 以降（join_frag.exits）が明確、PHI 生成の柔軟性確保

3. **なぜ verify は警告のみか？**
   - P2 の役割: wires/exits 分離の証明に集中（MIR 命令生成なし）
   - Phase 266 で MIR 生成時に verify を厳格化（target 違反 → Err）

**次フェーズへの橋渡し**:

次フェーズ（Phase 266）: wires → MIR terminator 生成（test-only PoC）
- wires を MIR terminator に落とす SSOT を追加（`emit_wires`）
- verify の strict 版を追加（`verify_frag_invariants_strict`、段階導入）

次フェーズ（Phase 267）: JoinIR/NormalizedShadow への適用 + Branch 生成
- NormalizedShadow/JoinIR で Frag/wires を実戦投入（層境界を守って段階的に）
- Branch の terminator 生成（wires → MIR）を追加

Phase 267: Pattern6/7/8 への展開
- Pattern6 (ScanWithInit) を Frag 化
- Pattern7 (SplitScan) を Frag 化
- Pattern8 (BoolPredicateScan) を Frag 化
- 再利用性の確認（pattern番号分岐削減）

現時点では既存 pattern6/7/8 や merge/EdgeCFG は未改変（合成能力の証明のみ）。

**Phase 266 P0-P2 完了！（2025-12-21）**

**実装完了内容**:
- ✅ emit.rs 作成（wires → MIR terminator 変換の SSOT）
  - emit_wires() 実装（from グループ化 + Return の target=None 許可）
  - unit test 4個（jump/return/unwired/multiple_from_same_block）
- ✅ verify_frag_invariants_strict() 追加（段階導入を壊さない）
  - 既存の verify_frag_invariants() は変更なし（警告のまま）
  - wires/exits 分離契約を Err 化（Return の target=None は許可）
- ✅ mod.rs 更新（emit module エクスポート）
- ✅ 全テスト PASS（1392 passed: 既存 1388 + 新規 4個）

**実装の核心原則**:

1. **from ごとにグループ化して1本だけ許可**
   - BTreeMap で from ごとにグループ化
   - 1 block = 1 terminator 制約を厳格に強制

2. **Return は target=None を許可**
   - Return は target が意味を持たない（emit_wires で無視される）
   - Fail-Fast 対象は Normal/Break/Continue/Unwind の target=None のみ

3. **verify_frag_invariants_strict() 別名で用意**
   - 既存の verify_frag_invariants() は警告のまま維持
   - 新規 verify_frag_invariants_strict() で Err 化
   - PoC/emit 側だけ strict を使用（段階導入を壊さない）

4. **Phase 260 terminator 語彙ルールを厳守**
   - Jump: set_jump_with_edge_args() を使用
   - Return: set_terminator() + set_return_env() を使用

**設計判断の記録**:

1. **なぜ from グループ化が必要か？**
   - 問題: 同じ block に複数 terminator を設定すると上書きになる
   - 決定: from ごとにグループ化し、1本だけ許可（Fail-Fast）
   - 理由: 1 block = 1 terminator は MIR の不変条件

2. **なぜ Return は target=None を許可するか？**
   - 問題: Return は呼び出し元に戻るので、target が意味を持たない
   - 決定: Return のみ target=None を許可
   - 理由: Normal/Break/Continue/Unwind は明確な target が必要

3. **なぜ verify_frag_invariants_strict() を別名にしたか？**
   - 問題: 既存の verify_frag_invariants() を Err 化すると、既存コードが壊れる
   - 決定: 新規に strict 版を追加し、段階導入
   - 理由: Phase 267+ で既存コードを段階的に strict へ移行

**次フェーズへの橋渡し**:

## Phase 267（P0完了）: Branch の第一級化（BranchStub + emit_frag）

- 目的: Frag に Branch を第一級で追加し、wires（Jump/Return）と同様に MIR terminator へ落とす入口を作る。
- 追加:
  - `BranchStub`（header→then/else の分岐を表現）
  - `Frag.branches: Vec<BranchStub>`（Branch 専用、wires と分離）
  - `emit_frag(function, frag)`（SSOT: `emit_wires` + `set_branch_with_edge_args`、1 block=1 terminator を Fail-Fast）
- スコープ:
  - ✅ BasicBlockId 層で unit test により PoC 証明
  - ❌ NormalizedShadow/JoinIR への実適用は Phase 268 に繰り越し（層境界維持）

詳細: `docs/development/current/main/phases/phase-267/README.md`

## Phase 268（完了）: if_form.rs への Frag 適用 + compose::if_ Entry Edge-args SSOT化

### P0: 最小適用（emission 層経由）

- 目的: EdgeCFG Fragment を "層を跨がずに" 実戦投入する
- 戦略: `if_form.rs` に直接 Frag 構築コードを書かず、`emission/branch.rs` に薄い入口関数 `emit_conditional_edgecfg()` を追加
- 理由:
  1. **層が綺麗**: Frag 構築ロジックを emission 層に閉じ込める
  2. **差分が小さい**: if_form.rs は API 呼び出し差し替えのみ（3箇所削除 + 1箇所追加）
  3. **デバッグ容易**: 層境界が明確で問題切り分けが簡単
- 実装:
  - emission/branch.rs に `emit_conditional_edgecfg()` 追加
  - if_form.rs の `emit_conditional()` + `emit_jump()` 2箇所を削除し、新規 API 呼び出しに置換
- テスト結果:
  - ✅ cargo build --release: 成功
  - ✅ cargo test --lib --release: 1444/1444 PASS
  - ✅ quick smoke: 45/46 PASS

### P1: compose::if_() Entry Edge-args SSOT化

- 目的: compose::if_() の then/else entry edge-args を呼び出し側 SSOT にし、TODO 削除（Phase 267 P2+ からの継続）
- **核心原則**: compose::if_() 内部で then/else entry edge-args を "勝手に空 Vec で生成" しない → 呼び出し側が明示的に渡す
- 実装:
  - compose::if_() シグネチャ変更: `if_(header, cond, t, then_entry_args, e, else_entry_args, join_frag)`
  - emission/branch.rs::emit_conditional_edgecfg() から空 EdgeArgs を then/else 両方に渡す
  - EdgeCFG テスト更新（compose.rs 2箇所、emit.rs 1箇所）
  - TODO コメント削除完了
- テスト結果:
  - ✅ cargo build --release: 成功
  - ✅ cargo test --lib --release: 1444/1444 PASS
  - ✅ quick smoke: 45/46 PASS

### アーキテクチャ

```
if_form.rs (MirBuilder 層)
  ↓ 呼び出し
emission/branch.rs::emit_conditional_edgecfg() (emission 層: 薄ラッパー)
  ↓ 内部で使用
Frag 構築 + compose::if_() + emit_frag() (EdgeCFG Fragment API)
  ↓ 最終的に呼び出し
set_branch_with_edge_args() / set_jump_with_edge_args() (Phase 260 SSOT)
```

詳細: `docs/development/current/main/phases/phase-268/README.md`

Phase 267: JoinIR Pattern への適用
- NormalizedShadow への Frag 適用
- Pattern6/7/8 を Frag 化
- Branch 生成 + pattern番号分岐削減
- fixture + smoke test
