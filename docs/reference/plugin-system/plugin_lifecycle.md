# Plugin Lifecycle and Box RAII

最終更新: 2026-08-04

Status: C′ target boundary plus current plugin implementation inventory;
plugin/FFI terminal-Home activation 0.

## 概要
NyashのBoxには「ユーザー定義Box」「ビルトインBox」「プラグインBox」があります。いずれもRAII（取得した資源は所有者の寿命で解放）に従いますが、プラグインBoxは共有やシングルトン運用があるため、追加ルールがあります。

## 共通ライフサイクル（ユーザー/ビルトイン/プラグイン）
以下のHome/Shared/handle用語はaccepted Home directionを説明します。正確な
HomeV1文法とShared表現はまだD0で、現在のplugin source/runtimeはSharedV1
移行状態です。parser/resolver/Home Flow/source Lowerはproduction-activeでは
ありません。

- C′のBox-member `fini {}` はdirect-callできないterminal Home hookです。
  外部資源を早期・fallibleに閉じる操作はordinary `close()`/`shutdown()`
  methodとしてResultを処理します。
- `local` のスコープを抜けると、その binding は終了します。Home/Shared
  slotならtokenを消費し、ordinary handleならowner countは変わりません。
  最後のownerなら物理的な解放が起こり得ますが、タイミングは実装依存です。
- Shared cycleはterminal Homeを妨げるため`weak`でback-edgeを切ります。
  exact shutdownが必要な資源はordinary domain methodまたは現在のhost
  `shutdown_plugins_v2()`境界で閉じ、source `obj.fini()`は使いません。

補足:
- source Home/handle/share の SSOT は
  `docs/reference/language/ownership.md`、object lifecycle/weak/`fini`/GC は
  `docs/reference/language/lifecycle.md` です。
- verified owning/Shared fieldはHome tokenを保持します。親hookの後に
  field Homeを宣言逆順でreleaseし、そのreleaseがchildのterminal Homeなら
  child hookが自動実行されます。親から`child.fini()`は呼びません。

## プラグインBoxの特則（シングルトン）
- シングルトン（`nyash.toml`）
  - プラグインのBox型は `singleton = true` を宣言可能
  - ローダが起動時に `birth()` し、以後は同一ハンドルを共有して返却
  - 現行host実装ではシャットダウン時（`shutdown_plugins_v2()` など）に
    plugin `fini` ABIを呼びます。C′のsource hookへの写像は未実装です。

補足:
- 現行互換実装には Box 値を広く共有参照として扱う経路があります。これは
  SharedV1の移行状態であり、最終source defaultではありません。現行
  eager-fini/UseAfterFini挙動はC′ authorityではありません。
- プラグインBoxのterminal Home hook/weak/affinityはdedicated ABI rowまで
  fail-fastまたはcurrent host routeへ隔離します。
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
  - 高価な資源の早期解放はordinary `close()`/`shutdown()`でResultを処理
- プラグインBox
  - シングルトン化が望ましい長寿命資源（サーバ、デバイス）に `singleton = true`
  - 複数スコープで共有される可能性がある値は、スコープ終了時に自動 `fini` されないことを前提に設計
  - 終了前に `shutdown_plugins_v2()` を呼ぶと単一箇所で確実に `fini` を実行可能

## 実装参照
- 現行plugin routeはC′ terminal Home ownerをまだ持ちません。
- プラグインローダ: `src/runtime/plugin_loader_v2.rs`（シングルトン生成・保持・シャットダウン、`PluginHandleInner::drop` / `finalize_now()` の `fini`）
- 現行 `PluginHandleInner::drop` / `GenericPluginBox::drop` がuser `fini`
  routeを呼ぶ挙動はC′最終契約と不一致です。plugin familyはterminal Home
  hookとstructural instance-destroy ABIを分離するまでactivation対象外です。
