---
Status: Provisional SSOT
Scope: optimization metadata の最小導入仕様（canonical `@rune` surface + legacy compat aliases）
Related:
- docs/development/current/main/design/optimization-ssot-string-helper-density.md
- docs/development/current/main/design/auto-specialize-box-ssot.md
- docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
- docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
- docs/development/current/main/design/rune-v0-contract-rollout-ssot.md
- docs/development/current/main/design/rune-v1-metadata-unification-ssot.md
- docs/reference/language/EBNF.md
---

# Optimization Hints / Contracts / Intrinsic Candidate SSOT

## Goal

helper 境界コストが支配するワークロードに対して、特別処理を散らさずに最小の注釈面を導入する。

## Current Syntax Truth (2026-03-30)

- canonical surface is `@rune Hint(...)`, `@rune Contract(...)`, `@rune IntrinsicCandidate("...")`
- legacy `@hint(...)`, `@contract(...)`, `@intrinsic_candidate("...")` stay accepted as compat aliases during the migration window
- declaration-leading legacy aliases normalize to declaration-local `attrs.runes`
- statement-position legacy aliases remain parse/noop compat
- optimization metadata is still `parse/noop only`; verifier / registry / backend use is not active yet

## Implementation Status Snapshot (2026-02-28)

| Item | Syntax Parse | Program(JSON) parity | Runtime Verifier | Registry Consistency Gate | Backend Optimization Use | Status |
|---|---|---|---|---|---|---|
| `Hint` (`@rune Hint` / `@hint`) | done | done (noop) | n/a | n/a | todo | provisional |
| `Contract` (`@rune Contract` / `@contract`) | done | done (noop) | todo | n/a | todo | provisional |
| `IntrinsicCandidate` (`@rune IntrinsicCandidate` / `@intrinsic_candidate`) | done | done (noop) | n/a | todo | todo | provisional |

補足:
1. 現在の parser 実装は `noop` 契約（意味不変）で固定。
2. verifier / registry gate / backend 利用が揃うまでは `not active`（研究バックログ）として扱う。
3. active 化は docs-first で lane/task へ昇格したときのみ許可する。

## Non-goals

- ユーザーコード全体への大量注釈導入
- 注釈を意味論（正しさ）に使うこと
- parser ごとの独自仕様分岐

## Boundary With Rune

- canonical surface is now part of the shared Rune metadata lane:
  - `@rune Hint(...)`
  - `@rune Contract(...)`
  - `@rune IntrinsicCandidate("...")`
- legacy `@hint` / `@contract` / `@intrinsic_candidate` remain compat aliases, not the canonical docs surface
- visibility / ownership / ABI-facing metadata still follow Rune v0 SSOT
- gate/carrier/front-door unification follows Rune v1 SSOT
  - base families / backend scope: `rune-v0-contract-rollout-ssot.md`
  - metadata unification: `rune-v1-metadata-unification-ssot.md`

## Minimal Set (v0)

### 1) Hints（助言、意味は変えない）

- canonical:
  - `@rune Hint(inline)`
  - `@rune Hint(noinline)`
  - `@rune Hint(hot)`
  - `@rune Hint(cold)`
- compat aliases:
  - `@hint(inline|noinline|hot|cold)`

規約:

1. hint は advisory（最適化ヒント）であり、無視されても意味は不変。
2. hint の解釈は backend ごとに異なってよいが、意味論は変えてはいけない。

### 2) Contracts（最適化の前提、破れは fail-fast）

- canonical:
  - `@rune Contract(pure)`
  - `@rune Contract(readonly)`
  - `@rune Contract(no_alloc)`
  - `@rune Contract(no_safepoint)`
- compat alias:
  - `@contract(pure|readonly|no_alloc|no_safepoint)`

規約:

1. contract は「最適化の前提」にのみ使う（言語意味の変更は禁止）。
2. contract を使った最適化を有効にする前に verifier で検査可能であること。
3. strict/dev で contract 破れが検出された場合は fail-fast する。
4. 検査不能な backend は contract を使った最適化を有効化しない（契約破りの黙殺禁止）。

### 3) IntrinsicCandidate（標準ライブラリ限定）

- canonical:
  - `@rune IntrinsicCandidate("symbol")`
- compat alias:
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

- **optimization metadata を language grammar として active にする最終状態では、Rust parser と `.hako` parser の両方が必要。**
- canonical docs surface は `@rune` に統一する。
- legacy aliases は compat window 中だけ許可する。
- 初手は parser 拡張なしで進められる（registry 側 metadata のみで導入可能）。

固定順序:

1. docs-first: 本書 + EBNF/Decision を provisional で固定。
2. Phase-A（no grammar change）: std 用 registry metadata だけで最適化を開始。
3. Phase-B（grammar on）: Rust parser が canonical rune families と legacy aliases を受理し、declaration-local `attrs.runes` へ正規化する。
4. Phase-C（parity）: `.hako` parser も同じ surface を受理し、同一 declaration-local 形へ合わせる。
5. Phase-D: parity gate 緑を確認してから optimizer/lowering で metadata を本利用する。Program(JSON v0) はここでも widened carrier にしない。

禁止:

- 片方の parser だけが metadata grammar を受理する状態で本番既定ONにすること。
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
3. `bash tools/smokes/v2/profiles/integration/parser/parser_rune_decl_local_attrs_selected_entry_trace.sh`

## Activation Rule (Not Active -> Active)

この仕様は現時点で `Provisional / Not Active`。次を満たした時だけ active 実装へ昇格する。

1. `CURRENT_TASK.md` の `Current blocker` に対象 lane/task が追加されている。
2. `docs/development/current/main/30-Backlog.md` から対象項目を昇格済み（not active 表記を外す）。
3. 次の順序を崩さない:
   - verifier 導入（contract 破れ fail-fast）
   - registry consistency gate 導入（intrinsic 整合 fail-fast）
   - backend 最適化利用（hint/contract/intrinsic の本利用）
