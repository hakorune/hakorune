Status: SSOT
Scope: JoinIR plan/normalize/lower freeze tag taxonomy (Phase-agnostic)
Related:
- AI handoff + debug contract: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
- Reject taxonomy: `docs/development/current/main/design/plan-reject-handoff-gap-taxonomy-ssot.md`

# PlanFrag Freeze Tag Taxonomy (SSOT)

This document fixes the freeze tag contract across layers so tests do not drift.
Tags must be stable, single-line, and layer-specific.

## Tag classes (by layer)

### Plan / Facts / Planner
Used when the loop is rejected or frozen before JoinIR lowering.

- Freeze: `[plan/freeze:<reason>] ...`
- Reject (non-freeze): `[plan/reject:<reason>] ...`

### Normalizer / Recipe / Contract
Used when a plan was chosen but internal recipe/contract invariants fail.

- Contract freeze: `[freeze:contract][<area>] ...`

### JoinIR Lowering
Used when JoinIR lowering fails after plan/normalizer succeeded.

- Generic JoinIR freeze: `[joinir/freeze] ...`
- Route-specific contracts: `[joinir/<phase>/<pattern>/contract] ...` (tag path keeps `pattern` for traceability; current semantics are route-first)

## Test expectations (SSOT)

Tests must match the layer they exercise:

- **Plan/Facts/Planner tests**: accept `[plan/freeze:*]` (and optionally `[plan/reject:*]` if the test expects rejection).
- **Normalizer/Recipe contract tests**: accept `[freeze:contract][...]`.
- **JoinIR Lowering tests**: require `[joinir/freeze]` or the specific contract tag.

## Return Contract (SSOT)

### `Ok(None)` = NotApplicable

「plan 化の対象ではない」ため、既存経路に委譲して良い。

- 例:
  - region が単純直列で、Plan/Frag を挟む必然がない
  - 必須の入口条件が揃っていない（loop/header/exit が確定できない等）※ただし “対象っぽい” 場合は Freeze

### `Ok(Some(plan))` = Unique plan

候補が一意に確定し、emit に必要な情報が揃っている。

### `Err(Freeze)` = Fail-Fast（silent fallback 禁止）

“対象っぽい” のに plan が一意化できない、または契約違反/禁止形が観測された。

## Tags (recommended)

- `plan/freeze:contract`
  - 形が契約を破っている（例: 必須 step が欠落、join 入力の整合が崩れている）
- `plan/freeze:ambiguous`
  - 複数候補が成立し、一意化できない（将来のルール追加で解消される可能性がある）
- `plan/freeze:unstructured`
  - Skeleton が確定できない（irreducible CFG / multi-entry loop など、構造化CFGの定義域外）
- `plan/freeze:unsupported`
  - 一意に判定できるが、未実装で扱えない（“未対応” を誤って None にしない）
- `plan/freeze:bug`
  - 不変条件が壊れている/到達してはいけない状態（実装バグ・内部矛盾）

## Message format (SSOT)

コード側の `Display` は安定化する（例）:

- `"[plan/freeze:{tag}] {message}"`

必要なら hint を別枠で付ける（ログで検知しやすくする）。

## References

- Entry: `docs/development/current/main/phases/phase-29ai/README.md`
- Plan/Frag overview: `docs/development/current/main/design/edgecfg-fragments.md`
- scan_with_init/split_scan route contracts (legacy numbered labels are traceability-only): `docs/development/current/main/design/pattern6-7-contracts.md`
- CorePlan Skeleton/Feature model: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`

## Non-goals

- Do not treat program exit codes as a substitute for freeze tags.
- Do not mix tags across layers in the same test unless the test explicitly spans layers.
