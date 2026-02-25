---
Status: Provisional SSOT
Scope: `.hako` 移植向け parser 拡張の最小セット（param type / implements / interface generic）
Related:
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/reference/language/EBNF.md
---

# Parser Extensions (Minimal Set) SSOT

## Goal

`.hako` への移植で詰まりやすい構文差分を、意味論を増やさずに最小で受理する。

## Non-goals

- 型検査の導入
- ジェネリクス制約（`where` / trait bound）
- optimizer / lowering の意味変更

## Minimal Set (ordered)

1. Parameter type annotation
   - 受理: `name: Type`（メソッド/関数/コンストラクタ/interface method）
   - v0 契約: AST は従来どおり `params: Vec<String>` を維持（型情報は保持しない）
   - 目的: Rust 由来コードの構文を fail-fast ではなく受理する

2. `implements` clause in `box` header
   - 受理: `box A interface I1, I2 { ... }`（既存）+ `box A implements I1, I2 { ... }`（追加）
   - v0 契約: `BoxDeclaration.implements` に格納
   - 注意: `implements` は parser sugar（内部語彙は増やさない）

3. Interface generics
   - 受理: `interface box Mapper<T, U> { ... }`
   - v0 契約: `BoxDeclaration.type_parameters` に格納
   - 注意: 型パラメータは parser/AST 層のみ（意味論は現行維持）

## Parser Contract

- Rust parser と `.hako` parser の parity を崩さない。
- 受理できない場合は fail-fast（silent fallback 禁止）。
- 既存 gate が緑の状態で、1拡張ずつ fixture + gate で固定する。
- `.hako` selfhost lane（FuncScanner）では `name: Type` を param 名へ正規化し、
  defs 契約（`params` は bare name 配列）を維持する。

## Acceptance (Rust parser lane)

最小受け入れ（各拡張で追加）:

1. `cargo test parser_header_param_extensions -- --nocapture`
2. `cargo check --bin hakorune`
3. `bash tools/smokes/v2/profiles/integration/selfhost/phase29cc_selfhost_stageb_funcscanner_typed_params_implements_min_vm.sh`
4. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`

## Rollout Order

1. docs update（本書 + `EBNF.md`）
2. Rust parser 受理 + unit test pin
3. `.hako` parser parity（必要形のみ）+ parity gate pin
4. selfhost subset へ failure-driven で PROMOTE
