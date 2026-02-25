---
Status: Draft
Scope: CorePlan / generic_loop — allow “continue + early step” without CFG-language creep
---

# Phase 29ca P3: Generic loop v0.1 — `continue` support (restricted, compositional)

## Objective

`hako_check` などの selfhost tooling に残りがちな「`if (...) { step; continue }`」形を、CorePlan の “小部品合成” の範囲で受理できるようにする。

狙いは **再帰でループを代替しない**こと（VM call-depth=128 回避）と、**CorePlan を第二のCFG言語にしない**ことの両立。

## Non-goals

- 任意 goto / 任意ラベル分岐の導入（禁止）
- `IfEffect` を一般の `If` に拡張する（join/else/exit を増やさない）
- nested loop の一般受理（別 subset / 別フェーズ）
- release 既定のログ/意味論の変更
- 新しい環境変数の追加

## Current pain (examples)

典型形（通したい）:

```hako
while i < n {
    if is_ws(s[i]) {
        i = i + 1
        continue
    }
    ... (effects) ...
    i = i + 1
}
```

この形は「末尾stepのみ」「continueなし」制約だと落ちやすく、再帰化すると VM 深さ制限にぶつかりやすい。

## Design (SSOT for this step)

### CorePlan vocabulary (Loop body)

既存の `generic loop v0`（`docs/development/current/main/design/coreloop-generic-loop-v0-ssot.md`）を崩さず、最小の拡張として次だけ許す:

- Loop body の leaf vocabulary に `IfEffect(then-only, leaf-only)` は引き続き許可
- **`IfEffect` の then-body 末尾に限り** `ExitIf(kind=Continue)` を 1つだけ許可する
  - これにより「then-body で step を実行してから continue（以降のeffectsをスキップ）」が表現できる

禁止（このP3ではやらない）:
- `IfEffect` 内で `Return/Break` を出す
- `IfEffect` に else / join / nested IfEffect / nested Loop を入れる
- `ExitIf(kind=Continue)` が then-body の途中に現れる（必ず末尾）

### Step rule (early step)

受理する形は “二者択一” ではなく “排他的二箇所” にする:

- ループ末尾の step は従来通り（末尾に 1回）
- **continue-path の step は `IfEffect` then-body の中だけ**に許可（その直後に Continue で必ず iteration を抜ける）

これで「continue した iteration では末尾stepが走らない」ことが構造で保証され、二重stepの検証が局所で可能。

## Implementation steps (critical order)

### Step 1: Contract-first (verifier)

対象:
- `src/mir/builder/control_flow/plan/coreloop_body_contract.rs`
- `src/mir/builder/control_flow/plan/verifier.rs`

やること:
- Loop body 検証で `IfEffect` then-body を走査し、次の不変条件を fail-fast（strict/dev）で固定
  - then-body が leaf effects のみ
  - 例外として then-body 末尾に限り `ExitIf(kind=Continue, payload=None)` を 1個だけ許可
  - then-body に `ExitIf(kind!=Continue)` が出たら NG
  - then-body に 2個以上 `ExitIf` が出たら NG

### Step 2: Lowering (IfEffect contains ExitIf(Continue))

対象:
- `src/mir/builder/control_flow/plan/lowerer.rs`

やること:
- `IfEffect` then-body の末尾が `ExitIf(kind=Continue)` の場合:
  - then-body の leaf effects を emit
  - continue を “loop continue port” へ落とす（既存の Continue lowering と同じ出口へ）
  - else 側（cond=false）は通常通り fallthrough

注意:
- cond=true の “無条件 ExitIf” を増殖させず、既存の `ExitIf` lowering の導線を再利用する
- emit/merge は CorePlan 以外を見ない（再解析禁止）

### Step 3: GenericLoop facts/normalizer acceptance

対象:
- `src/mir/builder/control_flow/plan/generic_loop/facts.rs`
- `src/mir/builder/control_flow/plan/generic_loop/normalizer.rs`

やること（最小）:
- generic loop v0.1 として、Loop body の “continue-path early step” を facts で認識し、CoreEffectPlan の `IfEffect` に落とす
- “continue の外に early-step がある” / “continue が末尾stepをスキップしない” 形は accept しない（release は従来通り、strict/dev は `flowbox/freeze`）

### Step 4: Tooling target (one failing loop)

対象候補（例）:
- `tools/hako_check/cli.hako` 内の joinir/freeze になっている関数
- もしくは `tools/hako_check/rules/*` の文字走査系

やること:
- “continue-path” を持つ 1箇所だけを、この新subset形に寄せる（再帰化はしない）
- 既定挙動・出力JSONの仕様は変えない（必要なら fixture/expected を更新して SSOT化）

### Step 5: Fixture + gate smoke

- 新規 fixture: `apps/tests/phase29ca_generic_loop_continue_min.hako`（最小形で continue+early-step を含む）
- 新規 integration smoke:
  - strict/dev: FlowBox adopt tag を必須（raw output）
  - non-strict: tag が出ないことを必須
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に組み込み

## Acceptance criteria

- `cargo build --release` が通る
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` が緑
- strict/dev の gate smoke で `flowbox/adopt`（schema SSOTどおり）が観測できる
- non-strict でタグが出ない（releaseログ不変）

## Notes (avoid the CFG-language trap)

- `IfEffect` を “then-only + leaf-only + (optional Continue-exit at end)” のまま固定すること
- `ExitIf` を任意 goto に拡張しないこと
- join/payload は ports/payload の SSOT（post-phi final form）から外さないこと

