# Runtime / Ring0 / CLI INDEX（読み始めガイド）

Status: Active  
Scope: Runtime / Ring0 / Stage1 CLI に関する現役設計・提案ドキュメントの入口。

このファイルは、ランタイムと CLI まわりの設計ドキュメントが増えてきたときに  
「まずどれを読めばいまの前提が分かるか」を示すための簡単なインデックスだよ。

---

## 1. Runtime / File I/O / ABI ライン

- File I/O Provider（FileBox / コア + プラグイン）
  - `docs/development/runtime/FILEBOX_PROVIDER.md`
- C Core ABI / Numeric ABI（設計段階の仕様）
  - `docs/development/runtime/C_CORE_ABI.md`
  - `docs/development/runtime/NUMERIC_ABI.md`
- System Hakorune Subset（Runtime / Numeric Core のサブセット定義）
  - `docs/development/runtime/system-hakorune-subset.md`

これらは主に設計・提案レベルのドキュメントとして扱うよ（Status: design-stage 等）。

---

## 2. 環境変数と運用ガイド

- Nyash 環境変数の整理と最小セット
  - `docs/development/runtime/ENV_VARS.md`

環境変数が増えすぎないようにするポリシーや、`nyash.toml` での上書き方の現役ガイドとして参照してね。

---

## 3. Stage1 CLI / selfhost 実行ライン

- Stage1 Hakorune CLI Design（設計 + stub 実装）
  - `docs/development/runtime/cli-hakorune-stage1.md`

Selfhost / Stage1 CLI の詳細なフローについては、JoinIR / Selfhost INDEX から  
`docs/development/current/main/selfhost_stage3_expected_flow.md` もあわせて読むと全体像が掴みやすいよ。

