# Nyash Reference Documentation 📖

このディレクトリには、Nyashプログラミング言語の正式な技術仕様が含まれています。

## 📚 サブディレクトリ

### language/
- 言語仕様（構文、型システム、Box仕様、デリゲーション）
- 正式な言語リファレンス

### architecture/
- システムアーキテクチャ（MIR、VM、インタープリター）
- 実行バックエンド仕様
- 内部設計ドキュメント

### concurrency/
- `task_scope` / `nowait` / `await` / `lock` / `scoped` / `worker_local` の reference
- current structured-concurrency Phase-0 manual

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
- current user-facing concurrency manual owner は `docs/reference/concurrency/semantics.md`
- `lock` / `scoped` / `worker_local` の state-model SSOT は `docs/reference/concurrency/lock_scoped_worker_local.md`
