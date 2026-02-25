# Nyash Specs (Legacy / Experimental)

このディレクトリは、古い仕様案や実験的なスペックを一時的に置いておくための場所だよ。

## ステータス

- `specs/` 全体のステータス: **Legacy / Cleanup対象**
- 現行仕様の「正本」は `docs/reference/` と `docs/architecture/` 側に集約していく方針だよ。

## 現在の主なファイル

- `language/index-operator.md`  
  - 配列 / マップ向けの `[]` インデックス演算子の設計メモ。  
  - Phase‑20.31 スコープの設計案として残しているよ。

## 運用ルール（提案）

- 新しい仕様はここではなく、まず `docs/private/`（案）か `docs/reference/`（正本）に置く。  
- `specs/` にある文書は、以下のどちらかに徐々に移していく:
  - 内容が現行仕様として生きている → `docs/reference/` / `docs/architecture/` に昇格して整理する。
  - 歴史的な価値のみ → `docs/archive/specs/` 以下に移動し、先頭に「Archived」の一行メモを書く。

この README 自体は、「ここはAIや過去フェーズのメモ置き場だった」という目印兼、今後の整理方針のメモとして使うよ。  
大掃除したくなったタイミングで、ここから順番に `reference/` か `archive/` へ移していけば大丈夫だよ。

