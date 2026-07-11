---
Status: Ready
Scope: docs-first + implementation instructions (CorePlan)
---

# Phase 29ca P0: Generic structured loop v0 (docs-first)

## Purpose

CorePlan の表現力を “パターン追加” ではなく “部品合成” で上げるために、
unknown loop を受理する最小モデル（generic structured loop v0）を SSOT として固定し、実装の導線を 1 枚にする。

## SSOT

- Unknown loop strategy: `docs/development/current/main/design/coreplan-unknown-loop-strategy-ssot.md`
- Generic loop v0: `docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`
- FlowBox interface: `docs/development/current/main/design/coreplan-flowbox-interface-ssot.md`

## Tools-side restriction (required)

tools/hako_check は CorePlan を広げる前に **restricted loop** へ整流する。

- 再帰禁止（VM depth=128 回避）
- nested loop 禁止
- continue 禁止
- step は末尾 1 箇所のみ

## Implementation instructions (P1 preview)

### Step 1: ExitIf の一般化（最小語彙）

現在の `ExitIfReturn` を “Return専用の特例” のまま増殖させない。
`ExitIf { cond, kind, payload }` へ一般化し、`kind` を `Return/Break/Continue/(Unwind予約)` に限定する。

要点:
- 任意 goto は絶対に入れない（ExitKind への脱出のみ）
- verifier で「Loop.body 内の許可位置」を強制（末尾のみ、Seq末尾のみ、など）
- payload 規約:
  - Return は `payload` 必須
  - Break/Continue は `payload` 不可（None 固定）
  - Unwind は予約（v0では accept しない）

### Step 2: verifier / lowerer を局所で固定

- verifier は Loop.body の語彙制約を fail-fast で固定する
- lowerer は ExitIf を “if cond { exit } else { next }” の最小CFGへ落とす（既存のIf loweringを再利用）

### Step 3: facts/composer の最小配線（段階的）

最初は “完全な一般ループ” を作らず、**落ちている具体ケース**（例: selfhost tooling の走査）を 1 個だけ選び、
その形を Facts→Composer で CorePlan 合成できるようにする。

この段階では:
- “既知パターンの追加” ではなく
- “generic loop v0 の構文表現（Loop + LeafEffects + ExitIf）” に落とす

v0 受理境界（必須）:
- value_join_needed == false
- cleanup_kinds_present は空
- exit_kinds_present は空 or Return/Break/Continue のみ
- loop_increment は「body の最終トップレベル 1個」だけ許可（move 先は step）
- body は LeafEffects + ExitIf 以外を含まない（if/loop/branch を持ち込まない）

### Step 4: strict/dev observability

strict/dev のみ:
- `flowbox/freeze code=...` を taxonomy SSOT に従って固定
- pattern名ではなく FlowBox スキーマ（box_kind + feature_set）でタグを出す

release は恒常ログ/意味論不変。

## First target (P1)

最初の受理対象は **selfhost tooling の “単純走査”** とする。
最低1件は `./tools/hako_check/deadcode_smoke.sh` の失敗点に合わせて採用し、
gate 回復を効果検証に使う（例: `HakoAnalysisBuilderBox._infer_call_arity/2`）。

## Acceptance (during migration)

移行期間は full quick/smoke を必須にしない（ただし最小の局所テストは推奨）:

- `cargo build --release`
- 追加した unit tests（CorePlan verifier/lowerer のみ）
- 可能なら `./tools/hako_check/deadcode_smoke.sh` の再実行（selfhost導線の確認）
