# Plugin Lifecycle and Box RAII

最終更新: 2026-07-15

## 概要
NyashのBoxには「ユーザー定義Box」「ビルトインBox」「プラグインBox」があります。いずれもRAII（取得した資源は所有者の寿命で解放）に従いますが、プラグインBoxは共有やシングルトン運用があるため、追加ルールがあります。

## 共通ライフサイクル（ユーザー/ビルトイン/プラグイン）
- `fini()` は論理的な終了（use-after-fini禁止）であり、外部資源（fd/socket/native handle など）を決定的に解放するための SSOT です。
- `local` のスコープを抜けると、その binding は終了します。owning/Shared
  bindingならtokenを消費し、ScopedAlias/Viewならowner countは変わりません。
  最後のownerなら物理的な解放が起こり得ますが、タイミングは実装依存です。
- 共有・循環参照がありうるため、スコープ終了“だけ”に `fini()` を期待しないでください。必要な資源は `fini()` / `cleanup` / `shutdown_plugins_v2()` で明示的に閉じます。

補足:
- source ownership/alias/share の SSOT は
  `docs/reference/language/ownership.md`、object lifecycle/weak/`fini`/GC は
  `docs/reference/language/lifecycle.md` です。
- verified owning/Shared fieldはtokenを保持します。子resourceをfinalizeする
  必要がある場合は親のuser hookから明示的に`child.fini()`を呼びます。
  stored field tokenの自動dropは子のuser finiを意味しません。

## プラグインBoxの特則（シングルトン）
- シングルトン（`nyash.toml`）
  - プラグインのBox型は `singleton = true` を宣言可能
  - ローダが起動時に `birth()` し、以後は同一ハンドルを共有して返却
  - シャットダウン時（`shutdown_plugins_v2()` など）に一括 `fini()` されます

補足:
- 現行互換実装には Box 値を広く共有参照として扱う経路があります。これは
  SharedV1の移行状態であり、最終source defaultではありません。`fini()` が
  論理的な終了（use-after-fini禁止）である点は両profileで共通です。
- プラグインBoxも同じルールです。`fini` 後の利用はエラー（Use after fini）。
- 長寿命が必要なケースは「シングルトン」で運用してください（個別のBoxに特例は設けない）。

### 例: `nyash.toml` 抜粋
```toml
[libraries."libnyash_counter_plugin.so".CounterBox]
type_id = 7
singleton = true
```

## Net Plugin（HTTP/TCP）運用メモ
- ログ
  - `NYASH_NET_LOG=1` で有効化、`NYASH_NET_LOG_FILE=net_plugin.log` 出力先
- 並列実行とポート
  - E2Eや並列CIではポート競合を避けるため、テスト毎にポートを明示（例: 8080, 8081, ...）
  - サーバ終了タイミング（`stop()`/スコープ終了）とクライアント接続の順序に注意

## ベストプラクティス
- ユーザー/ビルトインBox
  - フィールドの weak 指定（循環参照の解消）を活用
  - 必要に応じて明示 `fini()` を呼び、高価な資源（ファイル/ソケット等）を早期解放
- プラグインBox
  - シングルトン化が望ましい長寿命資源（サーバ、デバイス）に `singleton = true`
  - 複数スコープで共有される可能性がある値は、スコープ終了時に自動 `fini` されないことを前提に設計
  - 終了前に `shutdown_plugins_v2()` を呼ぶと単一箇所で確実に `fini` を実行可能

## 実装参照
- スコープ終了専用 owner としての自動 `fini` は持たない設計です。
- プラグインローダ: `src/runtime/plugin_loader_v2.rs`（シングルトン生成・保持・シャットダウン、`PluginHandleInner::drop` / `finalize_now()` の `fini`）
- 現行 `PluginHandleInner::drop` / `GenericPluginBox::drop` がuser `fini`
  routeを呼ぶ挙動はB′最終契約と不一致です。plugin familyは論理finiと
  structural instance-destroy ABIを分離するまでObjectCell cutover対象外です。
