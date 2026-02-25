---
Status: SSOT
Scope: Selfhost bringup / hako_check
---

# Selfhost bringup policy: prefer strengthening CorePlan over patching `.hako`

## Goal

セルフホスト導線（`tools/hako_check/*` を含む）が JoinIR/CorePlan の未対応で停止したときに、場当たり的な `.hako` 書き換えではなく、**コンパイラ側（CorePlan/Facts/Composer/Lowerer）を強くして恒久解決**する判断基準を固定する。

## Default policy (SSOT)

### Prefer: compiler-side fix

次の条件に当てはまる場合、原則として **CorePlan（または周辺のFacts/Composer/Lowerer）を拡張して吸収**する。

- `.hako` は “正しい” のに、JoinIR/CorePlan が loop shape を受理できず `[joinir/freeze]` で停止する
- 停止箇所が `lang/src/**` や `apps/lib/**` の **共通ヘルパー/stdlib** で、他の自己ホスト導線も依存している
- 解析導線の制約（例: `NYASH_DISABLE_PLUGINS=1`）を維持したい

狙い:
- 表現力を CorePlan に寄せることで、以後の `.hako` 開発（自己ホスト/ツール）を楽にする
- “未対応ループ” を SSOT 化し、次の穴埋めが機械的に進むようにする

### Allow: patch `.hako` (only when it is actually wrong / local)

次の条件に当てはまる場合のみ `.hako` 側の修正を許容する。

- `.hako` の挙動が仕様として誤り（バグ）であり、修正が自然で影響範囲が局所
- ツール固有の一時コードであり、将来的に削除予定が明確（撤去条件が docs にある）

禁止:
- JoinIR freeze 回避のためだけの “形を変える” 書き換え（結果として同じ意味でも、コンパイラ側の穴が残るため）

## Hard rules

- `NYASH_DISABLE_PLUGINS=1` の解析/検証導線は維持する（決定性と依存縮小のため）
- by-name/文字列一致での暫定ディスパッチは禁止（AGENTS.md 5.1）
- strict/dev では `flowbox/freeze` に収束させて可視化し、release は既定挙動/ログを変えない

## Acceptance (selfhost gate)

最終的に以下を満たすこと。

- `cargo build --release`
- `./tools/hako_check_deadcode_smoke.sh`
- `./tools/hako_check_deadblocks_smoke.sh`
- `./tools/hako_check/run_tests.sh`
- `./tools/hako_check.sh apps/selfhost-runtime/boxes_std.hako`

## Example

- `tools/hako_check/cli.hako` が `using selfhost.shared.common.string_helpers` を読み込む
- `StringHelpers.*` のループ形状が未対応で `[joinir/freeze]` になった
  - → `.hako` ではなく CorePlan を拡張して吸収し、gate を緑に戻す

