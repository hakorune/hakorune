# JoinIR Lowering Common Boxes

このディレクトリは「JoinIR lowerer の縫い目」を担う小箱の入口だよ。責務を混ぜないためのメモ。

- `dual_value_rewriter.rs` — name ベースの dual-value 書き換え（BodyLocal vs Carrier）を一箇所に閉じ込める

Fail-Fast 原則:
- 未対応 shape は error_tags::freeze などで理由付き停止（サイレント回避禁止）。
- フォールバック臭を出さず、ポリシーで「使う／使わない／拒否」を明示する。
