# loop → if → loop 構造の未対応調査

Date: 2026-01-25
Status: Known Gap（Backlog で BoxShape として対応予定）
Related:
- `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md#known-gaps-未対応構造`
- `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`
- `docs/development/current/main/30-Backlog.md`

## 概要

`generic_loop_v1` で `loop → if → loop` 構造（if でガードされた内側ループ）が正しく動作しない問題。

## 再現コード

```hako
// test_nested_pure.hako
static box Main {
  main() {
    local i = 0
    local sum = 0

    loop(i < 3) {
      if i == 1 {
        local j = 0
        loop(j < 2) {
          sum = sum + 5
          j = j + 1
        }
      }
      i = i + 1
    }

    print(sum)
    return 0
  }
}
```

**期待出力**: 10（i==1 の時だけ内側ループが実行、5×2=10）
**実際の出力**: 0（内側ループが実行されない）

## 構造別検証結果

| 構造 | 状態 | 確認 |
|------|------|------|
| `loop → loop` (直接ネスト) | ✅ 動作 | `phase29bq_generic_loop_v1_nested_min.hako` → 出力 1 |
| `loop → if → loop` | ❌ 出力 0 | 上記コードで再現 |
| `loop → loop → if` | ❌ freeze | `[joinir/freeze] Loop lowering failed` |

## 既存 fixture との関係

- `phase29bq_generic_loop_v1_nested_min.hako`: 直接ネスト（loop → loop）は Pattern1 で受理
- `phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako`: loop → if → loop だが continue 文付きで `LoopCondContinueOnly` で受理

## 結論

これは「受理の問題」ではなく **JoinIR lower 側の CFG/SSA 構築が未対応**である。

- 原因箇所: JoinIR lower 側のループ lowering（`src/mir/builder/control_flow/joinir/patterns/` 配下）
- 対応方針: BoxShape（wiring 契約: IfJoin / Loop-carried）として実装

## 作業時の入口

実装開始時は以下の手順で進める：

1. `src/mir/builder/control_flow/joinir/patterns/` 配下の loop pattern 実装を調査
2. `loop → if → loop` 構造の CFG/SSA 構築ロジックを追加
3. `phase29bq_generic_loop_v1_nested_loop_if_min.hako` で fast gate に pin

Step4 実装ポイント（BoxShape wiring）:
- `src/mir/builder/control_flow/plan/parts/loop_.rs`
- `src/mir/builder/control_flow/plan/parts/dispatch/if_join.rs`
- `src/mir/builder/control_flow/plan/steps/join_payload.rs`
