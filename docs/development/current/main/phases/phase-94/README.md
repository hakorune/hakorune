# Phase 94: escape route P5b “完全E2E” のための `ch` 再代入対応

- 目的: `tools/selfhost/test_pattern5b_escape_minimal.hako`（legacy selfhost test stem）を JoinIR（loop_break route; legacy label `Pattern2Break`）で VM E2E PASS に固定する。
- 新箱: `BodyLocalDerivedEmitter`（`src/mir/join_ir/lowering/common/body_local_derived_emitter.rs`）で `ch` を Select ベースの derived 値として表現する。
- 契約: `escape_cond` は base 値で評価し、override は副作用なし・評価順を SSOT 化。`HAKO_JOINIR_STRICT=1` では未対応形を理由付き Fail-Fast。
