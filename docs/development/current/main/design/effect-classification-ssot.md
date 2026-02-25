---
Status: SSOT
Scope: MIR/PlanFrag の effect 分類と「許される変形」法典（最適化/RC挿入/観測の安全領域）
Related:
- docs/development/current/main/design/planfrag-ssot-registry.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/joinir-plan-frag-ssot.md
- src/mir/effect.rs
- src/mir/instruction/methods.rs
---

# Effect Classification SSOT

目的: “後段の最適化/パスが何をして良いか” を effect で固定し、JoinIR/PlanFrag/CorePlan/RC insertion/観測が相互に壊さないようにする。

この SSOT は「新しい最適化を増やす」ためではなく、**安全境界を先に定義**するためにある。

## 1. SSOT: primary categories

MIR の一次分類（primary category）は次の 4 つ。

- `Pure`: 副作用なし（CSE/DCE/再順序の候補になり得る）
- `Mut`: 状態変化（heap 書き込み、所有権/RC 操作、alloc など）
- `Io`: 外部効果（I/O、FFI、global、debug/log、panic 等）
- `Control`: 制御効果（exit/panic/throw/branch など、実行の流れに影響）

SSOT 実装:
- `src/mir/effect.rs` の `EffectMask::primary_category()`

## 2. SSOT: effect は “命令の意味” に属する

effect は、最適化や lowerer の都合で付け替えてはいけない。

- SSOT 実装: `MirInstruction::effects()`（`src/mir/instruction/methods.rs`）
- 禁止: “扱いやすいから” という理由で PURE に偽装すること

## 3. PlanFrag/CorePlan の責務（effect に関する境界）

### 3.1 CorePlan の effect 語彙

`CoreEffectPlan` の語彙は最小で、`MethodCall` には `EffectMask` を必ず持つ。

- 参照: `src/mir/builder/control_flow/plan/mod.rs`（`CoreEffectPlan`）

SSOT ルール:
- `MethodCall.effects` は **正しい effect** を持つ（少なくとも PURE/MUT/IO/CONTROL の一次分類が破綻しない）
- `BinOp/Compare/Const` は PURE（ただし panic/throw の可能性を表現する場合は Io/Control に寄せるのを別フェーズで検討）

### 3.2 JoinIR/PlanFrag の “観測” は effect ではなく運用で隔離

strict/dev のタグ観測は release 挙動を変えないことが最優先。
MIR 内に観測命令を挿入する場合は Io/Debug として扱い、最適化で消されない/動かないことを前提にする。

## 4. “許される変形” の最小法典

ここでは **SSOT の最小**のみを書く（詳細な最適化仕様は別ドキュメントで追加）。

### 4.1 DCE（dead code elimination）

- 削除してよい: `effects().is_pure()` かつ dst が未使用
- 削除してはいけない: 非 pure（Mut/Io/Control）を含む命令

実装参照:
- `src/mir/passes/dce.rs`

### 4.2 CSE（common subexpression elimination）

- 候補: `effects().is_pure()` の命令のみ（同一オペランド・同一 op）

実装参照:
- `src/mir/passes/cse.rs`

### 4.3 再順序（reordering）

SSOT 最小ルール:
- `Io` は再順序禁止（外部観測が変わる）
- `Control` は再順序禁止（CFG/exit に影響）
- `Mut` は原則再順序禁止（alias/依存がないことを証明できない限り）
- `Pure` は再順序可能だが、`Control` を跨いだ移動はしない（別途 “basic block 内限定” 等で運用）

## 5. RC insertion と effect

RC insertion は “後から追加される Mut” を注入するパス。

SSOT ルール（最小）:
- `ReleaseStrong` / `RetainStrong` は Mut（write）として扱う
- DCE で消さない
- Control（return/exit）周辺の順序は保持する（cleanup の意味論固定）

参照（実装/フェーズ）:
- `src/mir/passes/rc_insertion.rs`

## 6. Next

次に固めると強い補助 SSOT:
- “effect と ExitKind/cleanup” の関係（unwind を含む設計）
- “panic/throw” の effect 付与（Pure/Io/Control の境界を明文化）

