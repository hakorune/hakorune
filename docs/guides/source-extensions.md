Source Extensions Policy — .hako と .nyash の等価性（Phase 20.47）

Intent
- 拡張子が異なるだけで、言語は 1 つ（.hako / .nyash は等価）。
- using は言語中立な“テキスト・プレリュード統合”に一本化してからパーサへ渡す。

Execution Mapping (unified)
- .hako / .nyash → using のテキスト統合（merge_prelude_text）→ Nyash Parser → MIR → VM 実行。
- verify (MIR v1) → hv1 早期経路（NYASH_VERIFY_JSON + HAKO_VERIFY_V1_FORCE_HAKOVM=1）。

Resolver/Include/Normalize
- Using: 常にテキスト統合。AST プレリュード統合は任意（プロファイルによる）。
- 拡張子の扱い: .hako を優先、.nyash を次点（両方探索）。
- using.paths 既定: apps, lib, ., lang/src（nyash.toml/hakorune.toml）。
- Include: 言語としては非推奨（quick は ERROR）。必要時のみ preinclude スクリプトを使用。
- Normalize（inline/dev）: CRLF→LF、冗長 `; }` の最小削除、先頭 `local` の互換補助 等。

Fail‑Fast Guards / Profiles
- Fail‑Fast: 既定OFF。`HAKO_FAIL_FAST_ON_HAKO_IN_NYASH_VM=1` のときのみ Hako 風ソースを Nyash VM 経路で拒否（診断用）。
- Profiles:
  - dev/ci: using 有効。AST マージ 既定ON。相対パス using は許容。
  - prod: using は名前解決のみ（alias/modules）。相対パス using は ERROR。`NYASH_USING_PROFILE=prod`。
- Extern（Hako provider）: `HAKO_V1_EXTERN_PROVIDER=1`（開発時ガード）、`HAKO_V1_EXTERN_PROVIDER_C_ABI=1`（任意タグ）。

Why two extensions?
- 言語は 1 つ。拡張子が異なるだけ（歴史的理由）。解決は等価で、.hako を優先・.nyash を次点で探索する。

Migration Plan
- 本ドキュメントの時点で統一完了（Phase 20.47）。以降は代表拡張/ハードニングを Phase 21.x で進める。

Best Practices (now)
- パス using ではなく、nyash.toml/hakorune.toml の [modules]/[using.paths]/[aliases]/workspace を活用。
- ソース内 include は避ける（必要時は preinclude を使ってテキスト展開）。
- verify は JSON を env（`NYASH_VERIFY_JSON`）経由で渡し、末尾数値を rc として評価。

Quick Example (alias using)
```
using selfhost.vm.entry as MiniVmEntryBox
static box Main { method main(args) {
  // alias 解決の確認（静的メソッド呼び出し）
  local _s = MiniVmEntryBox.int_to_str(0)
  return 0
} }
```
実行（quick プロファイル、dev 想定）:
`bash tools/smokes/v2/run.sh --profile quick --filter 'phase2047/using_alias_selfhost_vm_entry_canary_vm.sh'`

Nested Prelude Example
```
using selfhost.vm.hakorune-vm.json_v1_reader as JsonV1ReaderBox
static box Main { method main(args) {
  local seg = JsonV1ReaderBox.get_block0_instructions('{"schema_version":"1.0","functions":[{"name":"main","blocks":[{"id":0,"instructions":[]}]}]}')
  return seg == "" ? 0 : 1
} }
```
実行:
`bash tools/smokes/v2/run.sh --profile quick --filter 'phase2047/using_alias_nested_prelude_json_v1_reader_canary_vm.sh'`

Notes (this pass)
- Stage‑B emit の安定化: inline コンパイラ無効化と失敗時 tail 出力で原因を可視化。
- hv1 inline の拡張: const/compare/branch/jump/phi/extern（最小）を実装。
