# Phase 99: Trim/escape 実コード寄り強化（VM+LLVM EXE）

## ゴール
Phase96/97 の Trim/escape を "実コード寄り" に1段上げ、実アプリ側（MiniJsonLoader 相当）の失敗形を先に捕まえる状態にする。

## 実施内容

### P0-1: next_non_ws 3ケース固定（VM+LLVM）
- **拡張**: apps/tests/phase96_json_loader_next_non_ws_min.hako に3つ目のケース追加（`\n\r\tX` → 期待値 `3`）
- **smoke更新**: phase96_json_loader_next_non_ws_vm.sh と phase97_next_non_ws_llvm_exe.sh を3行比較に対応

### P0-2: escape 末尾バックスラッシュ固定（VM+LLVM）
- **新規fixture**: apps/tests/phase99_json_loader_escape_trailing_backslash_min.hako（`"hello\\` → 期待値 `hello\`）
- **現行仕様**: 末尾バックスラッシュは **best-effort**（そのまま出力）として固定
- **新規smoke**: phase99_escape_trailing_backslash_vm.sh と phase99_escape_trailing_backslash_llvm_exe.sh で検証

## 検証
- cargo test --lib
- bash tools/smokes/v2/profiles/integration/apps/archive/phase96_json_loader_next_non_ws_vm.sh
- bash tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh
- bash tools/smokes/v2/profiles/integration/apps/archive/phase99_escape_trailing_backslash_vm.sh
- bash tools/smokes/v2/profiles/integration/apps/archive/phase99_escape_trailing_backslash_llvm_exe.sh

## 原則
- **pattern増殖なし**: 既存の Policy/Recipe/Emitter で表現できる範囲だけを追加
- **integration smokeのみ**: LLVM不足/plugins不足はSKIP
