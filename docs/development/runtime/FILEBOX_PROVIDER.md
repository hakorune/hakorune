# File I/O Provider Architecture (Core‑RO + Plug‑in)

目的
- File I/O の意味論を単一起源（SSOT）に集約し、コアとプラグインの二重実装・分岐をなくす。
- Analyzer/テスト/CI は FileBox に依存せず動作（--source-file で直接テキストを渡す）。必要時はコアの read‑only 実装で安全に実行。
- 開発/本番はプラグイン（拡張版）を優先し、無い場合はコアにフォールバック。

設計（3層）
- リング0（SSOT / 共有抽象）
  - `src/boxes/file/provider.rs`
    - `trait FileIo { open(&str), read(), close(), caps(), … }`
    - `struct FileCaps { read: bool, write: bool }`
    - 共通エラー型・正規化ヘルパ（改行など）
  - 役割: File I/O の“意味論”の唯一の真実源。
- リング1（コア最小実装 + 薄いラッパ）
  - `src/boxes/file/core_ro.rs` … `CoreRoFileIo`（read‑only; write は Fail‑Fast）
  - `src/boxes/file/box_shim.rs` … `FileBox { provider: Arc<dyn FileIo> }`（委譲のみ; 分岐ロジックなし）
- リング1（選択ポリシー）
  - `src/runner/modes/common_util/provider_registry.rs`
    - `select_file_provider(mode: auto|core-ro|plugin-only) -> Arc<dyn FileIo>`
    - 選択はここに閉じ込める（FileBox から分岐を排除）
- リング2（プラグイン実装）
  - `plugins/nyash-filebox-plugin/src/provider.rs` … `PluginFileIo`（write/拡張を実装）
  - SSOT の `FileIo` を実装し、共通意味論に従う

モードと運用
- `NYASH_FILEBOX_MODE=auto|core-ro|plugin-only`
  - auto（既定）: プラグインがあれば PluginFileIo、無ければ CoreRoFileIo
  - core-ro: 常に CoreRoFileIo（Analyzer/CI 向け）
  - plugin-only: プラグイン必須（無い場合は Fail‑Fast）
- `NYASH_DISABLE_PLUGINS=1` のときは自動で core‑ro を選択
- Analyzer/テスト/CI
  - `--source-file <path> <text>` を第一経路（FileBox 非依存）
  - json‑lsp の stdout は純 JSON（ログは stderr）

利点
- File I/O の重複・分岐が provider_registry に一元化され、コードの責務が明確。
- Analyzer/テストはプラグイン非依存で安定（ノイズやロード失敗の影響を受けない）。
- 本番は拡張可能なプラグインを優先しつつ、不在時はコアにフォールバック。

テスト方針
- CoreRo: open/read/close 正常、存在しないパスで Fail‑Fast
- Plugin‑only: write を含む拡張 API 正常（プラグインが無い場合は Fail‑Fast）
- Auto: プラグイン不在時に CoreRo へフォールバック
- Analyzer: 全 HC テストが json‑lsp で緑、stdout は純 JSON を維持
