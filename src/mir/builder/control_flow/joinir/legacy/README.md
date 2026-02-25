# Legacy JoinIR Routing

## Policy (SSOT)

legacy routing は **互換/退避専用**。新規の呼び出し箇所を増やさない。

- Allowed callsite（コード側）:
  - `src/mir/builder/control_flow/joinir/routing.rs` の `cf_loop_joinir_impl(...)` で、
    `route_loop_pattern(...)` が `None` を返した後に **最後の退避**として呼ぶ。
- Disallowed:
  - 新しい pattern の受理やバグ修正のために legacy 側へ分岐を足す（“回避”の温床になる）。
  - by-name / 文字列一致などの例外分岐追加。
- SSOT:
  - `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`（Cleanliness Wave 3）
  - `docs/development/current/main/design/joinir-design-map.md`（入口/責務）

## 残す理由

- **既存コードパスとの互換性維持**: Pattern 1-4 の段階的移行中は legacy routing が必要
- **段階的移行のための過渡期対応**: Normalized shadow が全ケースをカバーするまでの橋渡し
- **回帰テスト基盤**: 既存の動作を保持しながら新しい routing を並行開発

## 撤去条件

以下の条件がすべて満たされたときに legacy routing を削除する：

1. **Router が閉じている**: `route_loop_pattern(...)` がカバーできない loop が残っていない。
2. **Normalized shadow の範囲が十分**: “loop(true) 正規化” 経路が SSOT の範囲で安定している。
3. **Gates が緑**: 少なくとも fast gate と planner_required/dev gate が緑（運用SSOTに従う）。
4. **依存箇所が 0**:
   - `rg -n "cf_loop_joinir_legacy_binding\\b" src/mir/builder/control_flow/joinir` が 0 件
   - `legacy/` 以下が未参照（export 停止 + drift check で固定）

## 依存箇所

現在 legacy routing を使用している箇所：

- `routing.rs` の fallback path: pattern router が処理できない場合の退避経路

Drift check（増殖防止）:
- `rg -n "cf_loop_joinir_legacy_binding\\b" src/`（callsite が増えていないこと）

## 移行ステップ（将来）

1. **Phase 132+**: Pattern 1-4 の新 routing 完成
2. **Phase 135+**: 回帰テスト全通過確認
3. **Phase 140+**: routing.rs の fallback path 削除
4. **Phase 145+**: legacy/ ディレクトリ削除

## ファイル構成

- `routing_legacy_binding.rs`: Legacy binding system（既存コード）
- `mod.rs`: Module export（このファイル）
- `README.md`: このファイル（削除条件・移行計画）
