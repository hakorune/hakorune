# Phase 125: Reads-only inputs → Normalized env（planned）

## 目的

- Phase 124 で `Return(Variable)` を “writes由来の env” だけで解決できるようになった。
- Phase 125 では reads-only（外側スコープの読み取り専用入力）も Normalized env に載せ、`return x` が “writesではないが読み取り可能” な場合に自然に解決できるようにする。
- 既定挙動は不変：`joinir_dev_enabled()` のときだけ生成・検証し、本番経路の出力/動作は変えない。

## Scope

- 対象: if-only（loopなし）の Normalized（dev-only）
- 目的の追加機能:
  - reads-only inputs を env に追加（return/compare で参照可能）
  - “unknown-read” を capability（または structured error tags）として扱い、strict では Fail-Fast

## SSOT 方針

- StepTreeFacts/Contract の `reads` は “何を読むか” の SSOT
- “どこから読むか（host ValueId の供給）” は builder 側で SSOT 化し、ScopeManager / CapturedEnv / function params を入力として受け取る
- AST から “勝手に capture する” のは禁止（Phase 100 の pinned/captured と混同しない）

## 受け入れ基準

- `cargo test --lib` が PASS
- Phase 121–124 の smokes が退行しない
- Phase 125 の新規 fixture/smoke で `return` が reads-only inputs から解決できる

## 関連

- Phase 121–124: StepTree→Normalized dev-only の段階投入
  - `docs/development/current/main/design/control-tree.md`
  - `docs/development/current/main/phases/phase-121/README.md`
  - `docs/development/current/main/phases/phase-124/README.md`
