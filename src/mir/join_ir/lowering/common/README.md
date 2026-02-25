# JoinIR Lowering Common Boxes

このディレクトリは「JoinIR lowerer の縫い目」を担う小箱の入口だよ。責務を混ぜないためのメモ。

- `conditional_step_emitter.rs` — ConditionalStep 専用の更新生成（P5b の i+=1/2 など）
- `condition_only_emitter.rs` — ConditionOnly  derived slot の再計算（Phase 93）
- `body_local_slot.rs` — 読み取り専用 body-local を条件式で使うためのガード付き抽出（Phase 92）
- `body_local_derived_emitter.rs` — 再代入される body-local（P5b `ch`）を Select で統合し、loop-var の +1/+2 も同時に出す（Phase 94）
- `body_local_derived_slot_emitter.rs` — 条件付き代入で再計算される body-local（seg）を Select で統合する（Phase 29ab P4）
- `dual_value_rewriter.rs` — name ベースの dual-value 書き換え（BodyLocal vs Carrier）を一箇所に閉じ込める

Fail-Fast 原則:
- 未対応 shape は error_tags::freeze などで理由付き停止（サイレント回避禁止）。
- フォールバック臭を出さず、ポリシーで「使う／使わない／拒否」を明示する。
