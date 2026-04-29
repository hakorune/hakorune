# JoinIR Lowering Common Boxes

このディレクトリは「JoinIR lowerer の縫い目」を担う小箱の入口だよ。責務を混ぜないためのメモ。

- `case_a/` — Generic Case-A lowering helpers. route vocabulary and guards live with the active lowerer.
- retired: name-based dual-value rewrite helpers were removed in 291x-747. Do not reintroduce AST/name rewrite shelves; add analysis-only observations to the active route facts instead.

Fail-Fast 原則:
- 未対応 shape は error_tags::freeze などで理由付き停止（サイレント回避禁止）。
- フォールバック臭を出さず、ポリシーで「使う／使わない／拒否」を明示する。
