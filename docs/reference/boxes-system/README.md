# 📦 Nyash Boxシステム設計ドキュメント

> Historical note: this folder preserves older Box-system reference material.
> Current Hakorune does not use “Everything is Box” as the whole language
> model. The current user-facing split is `record` for identity-free value
> aggregates and `box` for identity / behavior / lifecycle boundaries. See
> `docs/reference/language/types.md` and
> `docs/development/current/main/design/record-box-two-surface-one-substrate-ssot.md`.

## 🎯 概要

Nyash時代の「**Everything is Box**」に関する歴史的な設計ドキュメント集。
現在の Hakorune では Box は重要な境界の一つだが、唯一の境界ではありません。

注（`init { ... }` について）:
- `init { a, b, c }` は legacy のフィールド宣言（slot）です（互換のために残っています）。
- 新規コードでは、簡単な通常フィールドは `field`、初期値つきなら `field = expr`、型metadataを明示したいフィールドは `field: Type` / `field: Type = expr`、弱フィールドは `weak field` を推奨します。`init { ... }` は新規推奨ではなく互換用です（SSOT: `docs/reference/language/EBNF.md` / ライフサイクルSSOT: `docs/reference/language/lifecycle.md`）。

## 📚 ドキュメント構成

### 🌟 コア哲学

#### [everything-is-box.md](everything-is-box.md)
Nyash時代の「Everything is Box」の解説。現在読む場合は historical note を優先し、record/box 二面モデルと矛盾する箇所は historical として扱います。

### 📖 完全リファレンス

#### [box-reference.md](box-reference.md)  
**統合版Box型完全リファレンス**。全ビルトインBox型のAPI仕様、基本型からプラグインBoxまで。

### 🔄 システム設計

#### [delegation-system.md](delegation-system.md)
歴史的な delegation proposal。`from` 構文と `override` は canonical ではなく、現在の方針は explicit `delegate field exposes` へ移行予定。

#### [memory-finalization.md](memory-finalization.md)
**統合版メモリ管理&finiシステム**。Arc<Mutex>一元管理、fini()論理的解放、weak参照、プラグインメモリ安全性。

## 🔗 関連ドキュメント

- **[プラグインシステム](../plugin-system/)**: BID-FFIプラグインシステム完全仕様
- **[言語仕様](../core-language/)**: デリゲーション構文、言語リファレンス
- **[実行バックエンド](../execution-backend/)**: MIR、P2P通信仕様

## 🎨 設計原則

### Box Boundary
- `box` は identity / behavior / lifecycle boundary
- `record` は identity-free value aggregate
- 内部最適化は AggregateStoragePlan / ObjectStoragePlan で共有できるが、source surface は統合しない

### メモリ安全性
- Arc<Mutex>による統一管理
- fini()による決定論的リソース解放
- weak参照による循環参照回避

### プラグイン拡張性
- BID-FFIによる外部ライブラリ統合
- 型情報管理による安全な変換
- HostVtableによるメモリ管理

---

**最終更新**: 2025年8月19日 - boxes-system統合整理完了  
**Phase 9.75g-0成果**: プラグインシステムとの完全統合
