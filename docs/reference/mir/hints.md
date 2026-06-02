# MIR Hints — Zero‑Cost Structural Guidance

目的
- 構造を変えずに最適な IR を導くための“軽量ヒント”集合。Release では完全に剥離（ゼロコスト）。

原則
- ヒントは意味論を持たない（最適化・検証の補助のみ）。
- 生成器はヒントなしでも正しい MIR/IR を出す。ヒントは安定化・検証・最適化誘導のために用いる。

ヒント一覧（MVP 案）
- hint.scope_enter(id), hint.scope_leave(id)
  - スコープ境界を指示（cleanup 合流の挿入点検討に使用）。
- hint.defer(call-list)
  - defer 呼出し列の静的展開に用いる（例外未導入の間は分岐/return/loop-exit 経路へ複製）。
- hint.join_result(var)
  - If/Match 式の合流結果（join 変数）を明示。空 PHI 抑止とブロック先頭 PHI を誘導。
- hint.loop_carrier(vars…)
  - ループヘッダで同一グループ PHI へ揃える対象変数集合（LoopForm と整合）。
- hint.loop_header, hint.loop_latch
  - 自然ループの境界指示（コードレイアウト/最適化の補助）。
- hint.no_empty_phi（検証）
  - 空 PHI 禁止の検証を有効化（開発/CI向け）。

パイプラインでの扱い
1) Macro: If/Match 正規化・Scope 属性付与・LoopForm（while/for/foreach）整形後に、
2) Lowering: 上記ヒントを埋める（構造は不変）。
3) Verify: 空 PHI 不在・PHI は合流先頭・ループヘッダの PHI 整列などを確認。
4) Strip: Release ではヒントを完全剥離（IRには一切痕跡なし）。

注意
- 既存の機能（マクロ・正規化）で構造を整えた上で使う。ヒントのみでは誤構造は正せない。
- CI の軽量ゲートでは `hint.no_empty_phi` 相当のスモークで IR 健全性を監視する。

## Rune Optimization Metadata

Canonical declaration metadata uses `@rune`:

```hako
@rune Inline(prefer)
@rune Inline(required)
@rune IntrinsicCandidate("StringBox.length/0")
```

Current live verifier row:

- `Contract(no_alloc)` is checked by the MIR verifier and rejects instructions
  whose effect mask contains `Alloc`.
- `Contract(no_safepoint)` is checked by the MIR verifier and rejects explicit
  `Safepoint` instructions.
- `Contract(no_alloc)` / `Contract(no_safepoint)` populate MIR-owned
  `metadata.effect_plans`; the verifier consumes those plans as the obligation
  source.
- Distinct `Contract(...)` runes may appear on the same declaration. Exact
  duplicate contract values are rejected.
- `Contract(pure)` and `Contract(readonly)` remain metadata-only until their
  verifier rows land.
- No contract is currently exported for backend optimization use.
- `metadata.capability_plans` exists for future capability rows; capability
  verifier/backend use remains future.
- `Profile(allocator.fast|allocator.slow|substrate.leaf|intrinsic.leaf|raw.layout)`
  names are reserved compatibility/profile-registry entries. New source should
  use primitive runes such as `Inline(required)` instead of profile bundles.
- `Hint(inline/noinline/hot/cold)` is preserved into MIR-owned
  `metadata.inline_plans` as compat advisory inline/profile spelling.
- `Inline(prefer/avoid/required)` is the canonical source surface for inline
  requests:
  - `Inline(prefer)` maps to `request=prefer`, `fallback=keep_call`.
  - `Inline(avoid)` maps to `request=avoid`, `fallback=keep_call`.
  - `Inline(required)` maps to `request=required`, `fallback=fail_fast`.
    The verifier accepts only supported required leaf shapes and may infer
    `no_alloc` / `no_safepoint` from that shape. Explicit contracts remain
    available but are not required for the small receiver-fieldset leaf row.
- Mixed-base publication helpers are not a supported required leaf shape. A
  helper that reads a foreign object field and publishes either the scalar
  snapshot or the foreign handle must first be summarized as an effect shape.
  Reopen this surface through an `EffectSummary` and a narrow publication plan,
  not by letting the generic required-inline verifier accept arbitrary
  multi-base bodies.
- `Hint(inline)` remains a compat alias for advisory inline preference and may
  trigger the narrow M11c-soft-leaf MIR optimizer row:
  best-effort same-module pure leaf inline. Unsupported shapes keep the call.
- `Lowering(inline_required)` remains accepted as a compat spelling for
  `Inline(required)`. It is preserved as `request=required` and
  `fallback=fail_fast`; M11c-required-verify sets `verified=true` only when
  the supported narrow leaf-inline shape passes. It is still not backend-active.

Fail-fast diagnostics use the stable tags:

```text
[freeze:contract][rune/no_alloc]
[freeze:contract][rune/no_safepoint]
[inline-plan/body-too-large]
[inline-plan/recursive-cycle]
[inline-plan/dynamic-dispatch]
[inline-plan/unsupported-call]
[inline-plan/required-not-verified]
```

## InlinePlan Boundary

Inline is not a backend-local keyword.

Accepted flow:

```text
@rune Inline(prefer)
-> MIR InlinePlan request=prefer
-> M11c-soft-leaf may inline same-module pure leaf calls
-> unsupported shapes keep the call
-> backend emits the resulting MIR
```

Substrate-only required inline flow:

```text
@rune Inline(required)
-> MIR InlinePlan request=required
-> verifier accepts or fail-fast rejects
-> accepted plans carry verified=true
```

Backends and `.inc` readers must not discover inline policy from function names,
box names, or allocator-specific symbols.

## Mixed-Base Helper Reopen Path

When a same-module helper uses both the receiver base and one foreign object
base, classify it before considering inline:

```text
EffectSummary:
  receiver_reads
  receiver_writes
  foreign_reads
  foreign_writes
  handle_publications
  nested_call_count
  allocation_count
  safepoint_count
  branch_count
  foreign_base_count
```

The v0 rule is:

```text
receiver-local fieldset leaf:
  may be verified by Inline(required)

mixed-base publication helper:
  not a generic Inline(required) leaf
  first gate is same-module no-coredump / compile-time diagnostic
  second gate is EffectSummary metadata
  optional later gate is ReceiverSnapshotPublicationPlanV0
```

Decision: accepted metadata-only. `EffectSummary` is emitted in MIR function
metadata and exposed by `hako_check fastpath-explain`. It classifies helper
effect shape only; it does not authorize inline, direct call lowering, handle
publication, or backend route changes.

Rejected until a narrow plan says otherwise:

```text
multiple foreign bases
foreign writes
nested calls
branch or loop bodies
allocation
dynamic field access
handle publication that needs a runtime barrier
```

Compatibility:

```text
Hint(inline)              -> Inline(prefer)
Hint(noinline)            -> Inline(avoid)
Lowering(inline_required) -> Inline(required)
```
