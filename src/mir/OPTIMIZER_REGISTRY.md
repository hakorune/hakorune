# Optimizer Namespace Registry (SSOT)

目的: `src/mir/` 配下の optimizer 系4 namespace（`optimizer/` / `optimizer_passes/` /
`passes/` / `phi_core/loopform/passes/`）の責務境界を 1 枚に固定し、
「どの箱に何を足すか / どれを再利用できるか」で迷子にならないようにする。

前提:
- 挙動不変（本 registry は観測・分類のみ。物理 rename / import rewrite は行わない）
- coupling 軸（`&mut MirOptimizer` を取るか、`&mut MirModule` のみか）が分類の SSOT

## Namespaces

| Namespace | 責務 (What it is) | signature 軸 | 外部再利用 | 入口 |
|---|---|---|---|---|
| `optimizer/` | optimizer orchestration / pipeline owner。`MirOptimizer` schedule を所有し、全 wave（normalize → cleanup → late-call/inline → diagnostics）の順序を決める | — (orchestrator) | `MirOptimizer` 経由のみ | `src/mir/optimizer.rs`（sibling entry: `mod core; pub use core::{MirOptimizer, ...}`） |
| `optimizer_passes/` | MirOptimizer 結合用 built-in pass composition layer。`MirOptimizer` の debug/state を必要とする正規化・診断・ピープホール群 | `(opt: &mut MirOptimizer, module: &mut MirModule)` | ❌ 外部非公開（消費者は `optimizer/core.rs` のみ） | `src/mir/optimizer_passes/mod.rs` |
| `passes/` | reusable MIR transforms / analyses。optimizer に依存しない独立変換（DCE/CSE/escape/inlining/memory-effect 等）。runner/bin の外部消費者も持つ | `(module: &mut MirModule)` のみ | ✅ 外部再利用可 | `src/mir/passes/mod.rs` |
| `phi_core/loopform/passes/` | loopform / PHI-specific lowering boundary。optimizer namespace の**外**（PHI 構築パス。最適化パスではない） | （loopform 固有） | loopform 内のみ | `src/mir/phi_core/loopform/passes/mod.rs` |

## 分類ルール（SSOT — coupling 軸）

新しい pass を追加するとき、置き場所は **signature の coupling 軸** で一意に決まる:

1. `&mut MirOptimizer` が必要（debug flag / optimizer state に触れる）
   → `optimizer_passes/`
2. `&mut MirModule` のみで完結（stateless・他 backend でも再利用可能）
   → `passes/`
3. 新しい wave / schedule / orchestration そのもの
   → `optimizer/` の `core.rs`
4. loop の PHI 構造 / loopform lowering
   → `phi_core/loopform/`（本 registry の管轄外）

must-not:
- `optimizer_passes/` の pass を外部（runner/bin）から直接呼ばない。外部再利用可能な変換は `passes/` に置く。
- `passes/` の pass が `MirOptimizer` を要求しない。coupling が必要なら `optimizer_passes/` へ移動。
- optimizer schedule の順序を個別 pass の中に埋め込まない。順序は `optimizer/core.rs` が唯一の所有者。

## なぜ rename しないか

監査（design audit）は「`optimizer_passes/` と `passes/` が名前で衝突している → rename」と提案したが、
実測の結論は **rename しないのが正解**:

- **名前は正確**: `optimizer_passes/` は「optimizer 結合パス」、`passes/` は「再利用変換」。coupling 軸で一意。
- **クリーンな rename 先が存在しない**:
  - `early_normalize/` → ❌ `reorder` / `intrinsics` / `boxfield` は `run_late_call_and_inline` で**後期**実行（早期ではない）
  - `normalize/` → ❌ `boxfield` / `intrinsics` は「optimize」（normalize ではない）
  - `optimizer_internal/` → 正確だが冗長・不格好、決定的改善なし
- **物理 rename は挙動不変なのに import churn のみ増大**。特に `passes/` は `runner/bin` の外部消費者 6+ を持ち、rename は広範囲の無意味な書き換えを生む。

境界の読みにくさは **ドキュメント不足**（本 registry で解消）であり、命名バグではない。

## retire_when（再評価条件）

この 4-namespace 構成を維持する。以下のいずれかが起きた時に限り構成を再評価する:

- いずれかの namespace が**空**になった（pass が全て別 namespace へ流出した）
- coupling 軸（`&mut MirOptimizer` vs `&mut MirModule`）が崩れた（`passes/` の pass が `MirOptimizer` を要求し始める等）
- 新たな第5の coupling 区分が必要になった（現行2軸で分類不能な pass が出た）

それまでは構成不変。rename 提案が出たら、まず本 registry の coupling 軸で分類可能か確認すること。

## 参照

- orchestrator 実装: `src/mir/optimizer/core.rs`（wave 順序の SSOT）
- `optimizer_passes/` 消費点: `src/mir/optimizer/core.rs` 内の9箇所（外部消費ゼロ）
- `passes/` 外部消費者: `src/runner/modes/*`, `src/runner/product/llvm/*`, `src/bin/rc_insertion_selfcheck/*`
- REGISTRY 慣習（兄弟 doc）: `src/mir/builder/control_flow/plan/REGISTRY.md`
