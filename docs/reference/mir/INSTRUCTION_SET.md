# Nyash MIR Instruction Set (Canonical SSOT)

Status: Canonical (Source of Truth) — transitioning
Last Updated: 2026-04-09

この文書はNyashのMIR命令セットの唯一の参照だよ。
`src/mir/contracts/backend_core_ops.rs` の ledger と、この文書の機械可読カウントが一致することを SSOT 契約とする。
Core-26 / Core-15 / Core-14 は運用プロファイル（目標）で、実装語彙（kept/removed）とは別軸。

Primary implementation pointers:

- `src/mir/instruction.rs` — canonical enum shape
- `src/mir/contracts/backend_core_ops.rs` — kept/removed ledger, backend allowlists, doc-sync tests
- `src/runner/mir_json_emit/mod.rs` — JSON metadata emission contract
- `docs/reference/mir/metadata-facts-ssot.md` — function metadata JSON SSOT

注意: Debug/Safepointはビルドモードでの降格用メタ命令であり、コア命令数には数えない。
注意: KeepAlive/ReleaseStrong は lifecycle conformance のための命令（weak/hidden-root 対策）であり、Core-14/15 の最小コア数には数えない（ただし backend が実行する “実命令” である）。

## SSOT Ledger (machine-readable)

以下の行は CI/テストで参照する契約値（編集時は実装と同時更新）。

DOC_SYNC_MIR_KEPT_COUNT=33
DOC_SYNC_MIR_LOWERED_AWAY_COUNT=0
DOC_SYNC_MIR_REMOVED_COUNT=16
DOC_SYNC_MIR_VOCABULARY_COUNT=49
DOC_SYNC_MIR14_COUNT=13
DOC_SYNC_CORE26_COUNT=26

Transition Note

- Builder/Rewrite/JIT は既に以下の統合を段階適用中
  - TypeCheck/Cast → TypeOp
  - WeakNew/WeakLoad → WeakRef
  - BarrierRead/BarrierWrite → Barrier
  - Print → Call + `Callee::Extern("env.console.log")`
  - Box/Extern callsite → Call + `Callee::{Method|Extern}`
- Phase 12（ビルトインBox廃止）での追加統合
  - PluginInvoke → Call(`Callee::Method`) 完全統合（ビルトインフォールバックがなくなるため区別不要）
  - VM層でC ABI/Nyash ABI/Scriptプラグインを自動判定
- VM/JIT の代表的な Core-15 カバー手順は `docs/reference/mir/MIR15_COVERAGE_CHECKLIST.md` を参照。
- Core-14 安定後に本ドキュメントの「Core Instructions」を14命令へ更新し、マッピング表を併記する。
- Phase 285–287: lifecycle conformance のため `KeepAlive` / `ReleaseStrong` を追加（語彙の明確化）。
- Phase 163x: canonical sum lane のため `SumMake` / `SumTag` / `SumProject` を追加（Core profile とは別の kept vocabulary）。

## Canonical Sum Lane (kept vocabulary; Core profile out-of-count)

- SumMake
- SumTag
- SumProject

## Current Kept Vocabulary（33）

This is the current executable kept vocabulary from
`src/mir/contracts/backend_core_ops.rs::MIR_INSTRUCTION_KEPT_TAGS`.
It is the right inventory to consult when implementation, JSON emit, or backend
allowlists are being updated.

### Core data/control

- Const
- Copy
- Load
- Store
- UnaryOp
- BinOp
- Compare
- Branch
- Jump
- Phi
- Return
- Select

### Object / field / construction

- NewBox
- NewClosure
- FieldGet
- FieldSet
- RefNew

### Call / type / lifecycle

- Call
- TypeOp
- WeakRef
- Barrier
- KeepAlive
- ReleaseStrong

### Sum lane

- SumMake
- SumTag
- SumProject

### Exceptions / async / meta

- Throw
- Catch
- Safepoint
- FutureNew
- FutureSet
- Await
- Debug

## Canonical Sum JSON Ops

`SumMake` / `SumTag` / `SumProject` are part of the kept vocabulary and are emitted
to MIR JSON as dedicated ops. The concrete JSON shape comes from
`src/runner/mir_json_emit/emitters/sum.rs`.

| MIR instruction | JSON op | Required fields | Optional fields |
| --- | --- | --- | --- |
| `SumMake` | `sum_make` | `dst`, `enum`, `variant`, `tag` | `payload`, `payload_type` |
| `SumTag` | `sum_tag` | `dst`, `value`, `enum` | none |
| `SumProject` | `sum_project` | `dst`, `value`, `enum`, `variant`, `tag` | `payload_type` |

Notes:

- `payload_type` is a lowering hint, not a second semantic dialect.
- Canonical MIR still keeps singular payload shape; tuple multi-payload stays on
  compat transport above MIR.
- `sum_make` / `sum_project` can be paired with the phase-163x metadata chain in
  `functions[].metadata`; see `docs/reference/mir/metadata-facts-ssot.md`.

## Core Instructions（26）
- Const
- Copy
- Load
- Store
- UnaryOp
- BinOp
- Compare
- Jump
- Branch
- Phi
- Return
- Call（`Callee::{Global|Method|Extern|Value}`）
- NewBox
- ArrayGet
- ArraySet
- RefNew
- RefGet
- RefSet
- Await
- Print（Deprecated: ビルダーは発行しない。代わりに `Call + Callee::Extern(env.console.log)` を使用）
- TypeOp（TypeCheck/Cast統合）
- WeakRef（WeakNew/WeakLoad統合）
- Barrier（Read/Write統合）

## Lifecycle (実命令; Core-14/15 のカウント外)

これらは “最適化のための寿命” ではなく、言語の binding scope / explicit drop と weak 観測（`weak_to_strong()`）の整合を保つための命令。

- KeepAlive（PURE）
  - Meaning: 指定された値を、そのスコープ終端まで “生存しているものとして扱う” ための指示（liveness/DCE対策）。
  - Backend: 実行時の処理は no-op でも良いが、解析上は used-values として扱う必要がある。
- ReleaseStrong（WRITE）
  - Meaning: 指定された値が保持している strong root を明示的に落とす（典型: 変数上書き）。
  - Backend: 同一 object を参照する alias（SSA copy 等）も含めて解放対象にできる（実装都合）。

## Core-15（Target; 移行中の最小コア）
- 基本演算(5): Const, UnaryOp, BinOp, Compare, TypeOp
- メモリ(2): Load, Store
- 制御(4): Branch, Jump, Return, Phi
- 呼び出し(1): Call（Global/Method/Extern を callee で識別）
- Box(1): NewBox
- 配列(2): ArrayGet, ArraySet

## Core-14（Phase 12 Target; PluginInvoke統合後）
- 基本演算(5): Const, UnaryOp, BinOp, Compare, TypeOp
- メモリ(2): Load, Store
- 制御(4): Branch, Jump, Return, Phi
- 呼び出し(1): Call（`Callee::Method` が旧 BoxCall を吸収）
- Box(1): NewBox
- 配列(2): ArrayGet, ArraySet

Notes
- Print/Debug/Safepointはメタ/Extern化（Print→Call + Callee::Extern）。
- WeakRef/Barrier は統合済み（旧WeakNew/WeakLoad/BarrierRead/WriteはRewriteで互換）。
- Call は canonical callsite（Method/Extern/Global/Value）として運用。
- Phase 12: ビルトインBox廃止により、Method call と PluginInvoke の区別が不要に。VM層でABI判定。

## Meta (降格対象; カウント外)
- Debug
- Safepoint

## Function Metadata JSON

The instruction ledger is only half of the current MIR contract. Metadata emitted
under `functions[].metadata` is documented separately so instruction docs stay
focused:

- `docs/reference/mir/metadata-facts-ssot.md`

Current emitted keys:

- `value_types`
- `storage_classes`
- `string_corridor_facts`
- `string_corridor_candidates`
- `thin_entry_candidates`
- `thin_entry_selections`
- `sum_placement_facts`
- `sum_placement_selections`
- `sum_placement_layouts`

## 同期ルール
- 命令の追加/削除/統合は、まずこの文書を更新し、次に実装（列挙/Printer/Verifier/Optimizer/VM）を同期する。
- `DOC_SYNC_MIR_*` の値は `src/mir/contracts/backend_core_ops.rs` の ledger と一致させる（不一致はテスト失敗）。
- Core-26 / Core-14 のようなプロファイル数は、実装語彙（kept/removed）と混算しない。
