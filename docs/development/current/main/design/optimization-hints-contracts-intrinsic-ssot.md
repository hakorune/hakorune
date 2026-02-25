---
Status: Provisional SSOT
Scope: `@hint` / `@contract` / `@intrinsic_candidate` の最小導入仕様（docs-first）
Related:
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/auto-specialize-box-ssot.md
- docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
- docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
- docs/reference/language/EBNF.md
---

# Optimization Hints / Contracts / Intrinsic Candidate SSOT

## Goal

helper 境界コストが支配するワークロードに対して、特別処理を散らさずに最小の注釈面を導入する。

## Non-goals

- ユーザーコード全体への大量注釈導入
- 注釈を意味論（正しさ）に使うこと
- parser ごとの独自仕様分岐

## Minimal Set (v0)

### 1) Hints（助言、意味は変えない）

- `@hint(inline)`
- `@hint(noinline)`
- `@hint(hot)`
- `@hint(cold)`

規約:

1. hint は advisory（最適化ヒント）であり、無視されても意味は不変。
2. hint の解釈は backend ごとに異なってよいが、意味論は変えてはいけない。

### 2) Contracts（最適化の前提、破れは fail-fast）

- `@contract(pure)`
- `@contract(readonly)`
- `@contract(no_alloc)`
- `@contract(no_safepoint)`

規約:

1. contract は「最適化の前提」にのみ使う（言語意味の変更は禁止）。
2. contract を使った最適化を有効にする前に verifier で検査可能であること。
3. strict/dev で contract 破れが検出された場合は fail-fast する。
4. 検査不能な backend は contract を使った最適化を有効化しない（契約破りの黙殺禁止）。

### 3) IntrinsicCandidate（標準ライブラリ限定）

- `@intrinsic_candidate("symbol")`

規約:

1. 対象は std（ランタイム同梱）メソッドのみ。
2. 注釈は「候補」を示すだけで、置換保証はしない。
3. 実際の置換可否は `IntrinsicRegistryBox` の登録情報で最終決定する。

## Registry Consistency Gate（CheckIntrinsics 相当）

特別扱いの散在を防ぐため、注釈と registry の整合を gate で監査する。

必須チェック:

1. `@intrinsic_candidate` が付いたメソッドは registry に存在し、symbol/arity が一致する。
2. registry 側で「注釈必須」と宣言したエントリに注釈欠落がない。
3. 重複登録（同一 symbol の不一致定義）がない。

運用:

- 不整合は strict/dev で fail-fast。
- 置換不能なケースは generic route に戻す（意味不変）。

## Parser Rollout Contract（Rust / .hako 二重実装の要否）

結論:

- **言語文法として `@hint/@contract/@intrinsic_candidate` を受理するなら、最終的に Rust parser と .hako parser の両方が必要。**
- ただし初手は parser 拡張なしで進められる（registry 側 metadata のみで導入可能）。

固定順序:

1. docs-first: 本書 + EBNF/Decision を provisional で固定。
2. Phase-A（no grammar change）: std 用 registry metadata だけで最適化を開始。
3. Phase-B（grammar on）: Rust parser が注釈を受理し Program(JSON v0) に属性を出力。
4. Phase-C（parity）: .hako parser も同じ注釈を受理し、同一 Program(JSON v0) 形へ合わせる。
5. Phase-D: parity gate 緑を確認してから optimizer/lowering で注釈を本利用する。

禁止:

- 片方の parser だけが注釈を受理する状態で本番既定ONにすること。
- parser 差分を workaround で吸収すること。

## Acceptance (docs phase)

docs-first フェーズでは、次の 2 条件を満たすまで実装を進めない。

1. `selfhost-language-v1-freeze-ssot.md` に「v1 freeze 範囲外（provisional）」として明記されている。
2. `selfhost-parser-mirbuilder-migration-order-ssot.md` に parser parity 順序が固定されている。

## Acceptance (Phase-A: no grammar change)

registry metadata だけで進める段階の最小受け入れ:

1. `PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_mir_call_intrinsic_registry.py src/llvm_py/tests/test_mir_call_auto_specialize.py`
2. `PERF_GATE_INTRINSIC_REGISTRY_CHECK=1 bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_gate_vm.sh`

## Acceptance (Phase-B/C: parser extension + parity)

grammar ON の最小受け入れ:

1. `cargo test parser_opt_annotations -- --nocapture`
2. `bash tools/smokes/v2/profiles/integration/parser/parser_opt_annotations_dual_route_noop.sh`
