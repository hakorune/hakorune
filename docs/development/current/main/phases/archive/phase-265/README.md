# Phase 265 — EdgeCFG Fragments（compose/verify の合成則）

Status: Completed ✅  
SSOT (design): `docs/development/current/main/design/edgecfg-fragments.md`

## ゴール
- Structured→CFG lowering を **ExitKind + Frag 合成**として表現する入口を育てる（legacy numbered route label 列挙を将来縮退させる）
- 既存の JoinIR/merge/scan_with_init / split_scan / bool_predicate_scan route には触らず、**API と合成則だけ**を段階投入する

## 完了内容
- P0: `compose`/`verify` の形を固定（入口SSOT迷子防止）
- P1: `compose::loop_` の配線（Break/Continue → wires）を実装（test-only PoC）
- P2: **wires/exits 分離**を導入し、`loop_`/`seq`/`if_` を合成則で実装
  - `exits`: `target=None` のみ（上位へ伝搬する未配線 exit）
  - `wires`: `target=Some(...)` のみ（内部で解決済み配線）
  - `if_` は `join_frag: Frag` を受け取り、Normal が「join 以降」を表すようにする

## 主要ファイル
- `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
- `src/mir/builder/control_flow/edgecfg/api/frag.rs`
- `src/mir/builder/control_flow/edgecfg/api/edge_stub.rs`
- `src/mir/builder/control_flow/edgecfg/api/verify.rs`

## コミット（参考）
- `ab1510920` Phase 265 P0
- `cda034fe8` Phase 265 P1
- `21387f381` Phase 265 P2

## 次フェーズ
- Phase 266: `wires → MIR terminator` の SSOT（test-only PoC）
- Phase 267: Branch 生成 + JoinIR/NormalizedShadow への実適用（段階導入）
