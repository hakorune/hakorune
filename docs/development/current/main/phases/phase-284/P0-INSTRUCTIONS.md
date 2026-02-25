# Phase 284 P0（docs-only）: Return as ExitKind SSOT

目的: `return` を pattern 個別実装へ散らさず、`ExitKind::Return` と `compose::*` / `emit_frag()` に収束させるための SSOT を固定する。

## このP0でやること（コード変更なし）

1. SSOT を 1 枚にまとめる
   - `docs/development/current/main/phases/phase-284/README.md` を SSOT として整える（用語・責務・境界）。
2. 既存SSOTとの整合を取る
   - Phase 282 の “SSOT=extract / pattern_kind=safety valve / lower re-extract” と矛盾しないこと。
3. “移行期間の穴” を塞ぐ言い方にする
   - close-but-unsupported は `Ok(None)` ではなく `Err`（Fail-Fast）であることを明記。

## 文書に必ず入れる事項（チェックリスト）

- [ ] `return expr` は `ExitKind::Return` で表現する（pattern の特例は禁止）
- [ ] Return edge の返り値は `EdgeArgs`（または Return 用 args）で運ぶ
- [ ] terminator 生成は `emit_frag()` が SSOT（Return も例外なし）
- [ ] extractor の返り値境界: `Ok(None)` と `Err` の意味を固定（黙殺禁止）
- [ ] Phase 284 の P1+（実装）で “どこを触る” かの導線を箇条書きで残す（ただしコードは書かない）

## SSOTリンク

- `docs/development/current/main/phases/phase-284/README.md`
- `docs/development/current/main/design/edgecfg-fragments.md`
- `src/mir/builder/control_flow/edgecfg/api/compose/mod.rs`
- `src/mir/builder/control_flow/edgecfg/api/emit.rs`
- `docs/development/current/main/phases/phase-282/README.md`
