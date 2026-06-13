# JoinIR Lowering Common Boxes

このディレクトリは「JoinIR lowerer の縫い目」を担う小箱の入口だよ。責務を混ぜないためのメモ。

- `cfg_shape.rs` — CFG / instruction shape probes used by target-specific MIR lowerers.
- `dispatch.rs` — MIR-based vs handwritten lowering dispatch and fallback logging.
- `string_whitespace.rs` — shared string whitespace predicate instruction
  sequence for trim-style lowerers. It does not own route acceptance policy.
- `target_adapter.rs` — generic Case-A probe orchestration for target-local
  lowerers. It does not own route policy or Exec/LowerOnly behavior.
  Do not add target-specific guard options here; keep those in route modules.
- `type_hint.rs` — IfSelect / IfMerge type hint extraction from MIR constants.
- `case_a/` — Generic Case-A lowering helpers. route vocabulary and guards live with the active lowerer.
- `../common.rs` — stable re-export facade only; do not grow mixed helper logic there.
- retired: name-based dual-value rewrite helpers were removed in 291x-747.
  Do not reintroduce AST/name rewrite shelves; add analysis-only observations
  to the active route facts instead.

Fail-Fast 原則:
- 未対応 shape は error_tags::freeze などで理由付き停止（サイレント回避禁止）。
- フォールバック臭を出さず、ポリシーで「使う／使わない／拒否」を明示する。
