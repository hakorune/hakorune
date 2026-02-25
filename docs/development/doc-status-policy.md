# Development Docs Status & Scope Policy

このファイルは、開発ドキュメント（特に `docs/development/` 配下）が増えたときに  
「どれが現役の設計か」「どれが歴史メモか」「どれを入口に読むべきか」を迷わないようにするための運用ルールだよ。

## 1. ステータス分類

開発ドキュメントの先頭付近に、次のいずれか（または組み合わせ）のステータスを明示することを推奨するよ。

- `Status: SSOT`  
  - 単一情報源（Single Source of Truth）。同じ内容を別ファイルに複製しない前提の設計ドキュメント。
  - 変更があったらまずここを更新し、Phase 文書などは補足とする。
- `Status: Active`  
  - いまの開発ラインで直接参照される現役ドキュメント。
  - 設計・実装の前提として読むことを想定。
- `Status: Historical`  
  - 過去フェーズのメモや調査ログ。読み物としては有用だが、設計の SSOT ではない。
  - 先頭付近に「現行仕様は <path> を参照」と追記できるとさらに親切。
- `Status: VerificationReport`  
  - スモーク/ゴールデン/自動テストの実行レポート。
  - 仕様そのものではなく、「いつ・何を確認したか」の記録。

完全移行は必須ではないけれど、新しく書くドキュメントや編集するタイミングで  
少しずつステータスを追記していく運用を想定しているよ。

## 2. スコープと入口の書き方

ステータスに加えて、「このドキュメントは何の入口なのか」を 1–3 行で明示しておくと迷いにくくなるよ。

推奨フォーマットの例：

```markdown
Status: SSOT, Active  
Scope: JoinIR ライン全体（Loop/If/Boundary）の箱と契約の設計を横串で定義する。
See also: docs/development/current/main/01-JoinIR-Selfhost-INDEX.md
```

- `Scope:` では「このファイルを読んだら何が分かるか」を 1 文で書く。
- `See also:` では関連する INDEX や補助ドキュメントにリンクする（必要な場合だけで OK）。

## 3. JoinIR / Selfhost まわりの現時点の整理

特に問い合わせが多い JoinIR / Selfhost については、次を入口として扱うよ。

- トピック別 INDEX  
  - `docs/development/current/main/01-JoinIR-Selfhost-INDEX.md`
- JoinIR SSOT  
  - `docs/development/current/main/joinir-architecture-overview.md`（Status: SSOT, Active 想定）
- Selfhost Stage‑B/Stage‑1/Stage‑3 フロー  
  - `docs/development/current/main/selfhost_stage3_expected_flow.md`（Status: Active 想定）

これらの詳細な読み順や関連ドキュメントは、INDEX 側に集約していく方針だよ。

## 4. 重複ドキュメントの扱い方

「同じようなことが複数のファイルに書かれている」場合は、次の順番で整理するのがおすすめだよ。

1. **SSOT を決める**  
   - 設計として参照されるべき 1 ファイルを選び、`Status: SSOT` を付ける。
2. **他ファイルは役割を限定する**  
   - 例: `Status: Historical` または `Status: VerificationReport` を付ける。
   - 冒頭に「現行仕様は <SSOT パス> を参照」と追記する。
3. **新しい情報は SSOT に寄せる**  
   - フェーズメモから仕様が昇格したら、SSOT 側を先に更新し、メモ側にはリンクだけを残す。

いきなり全ファイルを一括変換するのではなく、  
「触るタイミングでステータスとスコープを少しずつ整える」方針で進めると安全だよ。

