# Phase 266 — EdgeCFG wires → MIR terminator（test-only PoC）

Status: Completed ✅  
SSOT (design): `docs/development/current/main/design/edgecfg-fragments.md`

## ゴール
- `Frag.wires`（解決済み配線）を **MIR terminator** に落とす最小 PoC を作る
- JoinIR/NormalizedShadow には触らず、**MIR BasicBlockId 層だけ**で証明する

## 完了内容
- `emit_wires(function, wires)` を SSOT として追加（Jump/Return のみ）
  - `from` ごとにグループ化し **1 block = 1 terminator** を Fail-Fast で強制
  - `Return` は `target=None` を許可（target が意味を持たない）
  - `Jump` は `set_jump_with_edge_args()`（Phase 260 の SSOT ルール）
  - `Return` は `set_terminator(Return) + set_return_env()`（Return 専用メタ）
- `verify_frag_invariants_strict()` を追加（段階導入）
  - 既存 `verify_frag_invariants()` は警告のまま維持
  - strict は `wires/exits` 分離契約を Err 化（PoC/emit 側のみ）

## 主要ファイル
- `src/mir/builder/control_flow/edgecfg/api/emit.rs`
- `src/mir/builder/control_flow/edgecfg/api/verify.rs`
- `src/mir/builder/control_flow/edgecfg/api/mod.rs`

## テスト
- `emit` のユニットテスト（Jump/Return/unwired/multiple-from）を追加し PASS

## 次フェーズ
- Phase 267:
  - Branch の生成（wires → MIR）を追加
  - JoinIR/NormalizedShadow/scan_with_init / split_scan / bool_predicate_scan route への実適用（層境界を守って段階導入）
