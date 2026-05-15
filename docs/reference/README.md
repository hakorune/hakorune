# Nyash Reference Documentation 📖

このディレクトリには、Nyashプログラミング言語の正式な技術仕様が含まれています。

## 📚 サブディレクトリ

### language/
- 言語仕様（構文、型システム、Box仕様、デリゲーション）
- 正式な言語リファレンス
- Stage0 / Stage1 で使える `.hako` surface profile:
  `docs/reference/language/stage-profiles.md`

### architecture/
- システムアーキテクチャ（MIR、VM、インタープリター）
- 実行バックエンド仕様
- 内部設計ドキュメント

### concurrency/
- `co` / `task_scope` / `nowait` / `await` / `Channel` / `sync box` / `context` / `worker_local` の reference
- current structured-concurrency CONC status and semantics manual

### runtime/
- runtime/kernel/substrate reference manuals
- current substrate capability manual: `docs/reference/runtime/substrate-capabilities.md`

### api/
- ビルトインBoxのAPI仕様
- 標準ライブラリリファレンス
- 各Boxのメソッド詳細

### plugin-system/
- プラグインシステム仕様
- BID-FFI（Box Interface Definition - Foreign Function Interface）
- プラグイン開発ガイド

## 📝 注意事項
このディレクトリのドキュメントは安定版です。開発中の仕様は`development/`を参照してください。

Concurrency note:
- new concurrency Boundary model owner は `docs/reference/concurrency/boundary-model.md`
- implementation taskboard / compat archive rule owner は
  `docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md`
- current user-facing concurrency manual owner は `docs/reference/concurrency/semantics.md`
- `lock` / `scoped` / `worker_local` の historical/provisional state-model SSOT は `docs/reference/concurrency/lock_scoped_worker_local.md`

Runtime substrate note:
- current capability manual owner は `docs/reference/runtime/substrate-capabilities.md`
- language-facing low-level capability entry は
  `docs/reference/language/low-level-capabilities.md`

Language stage profile note:
- canonical grammar owner は `docs/reference/language/EBNF.md`
- Stage0 / Stage1 support manual owner は
  `docs/reference/language/stage-profiles.md`
