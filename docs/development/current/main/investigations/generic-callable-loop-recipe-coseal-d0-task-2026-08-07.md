# Callable single-loop Recipe co-seal D0

Status: `Decision: accepted design; bounded caller-zero implementation is the next row; production selection is not authorized`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-MAP-S1`

## Purpose

MAP-S1 is now a closed caller-zero source map. It co-seals the resolver-owned
callable ledger with the neutral `VerifiedSourceSyntaxFactsV1` product, but it
does not issue a portable Recipe, JoinSig, ValueId, CFG, PHI, After, or
physical input. This one shallow design stop fixes the common owner boundary
before any Recipe implementation begins.

The selected callable profile remains separate from the nested Generic G0
profile:

```text
StringHelpers.int_to_str/1
  prefix: local value = helper.to_i64(n)
  loop:   i < 1 { i = i + 1 }
  tail:   return value
```

No shape similarity may relabel this profile as Generic G0.

## Decision candidates to close

The design must choose one common, not callable-specific, product for:

```text
Recipe / JoinSig
LoopOperationSourceRelation
BindingSSA effect relation
After / Tail / completion envelope
resolver Loop source + frame + Scope/Region
```

The source map remains the only source-to-role input. It owns neutral source
roles and exact resolver evidence; the Recipe mapper owns numeric/operator
policy already admitted by MAP; the common Recipe owner issues logical keys;
the canonical SSA/CFG session remains the sole ValueId/PHI/physical owner;
completion/DraftSeal remains the sole terminal owner. No layer may mint a
second identity or re-pair rows by AST, name, path suffix, ordinal, or
`variable_map`.

The prefix and tail are whole-callable envelope obligations, not additional
Loop operations. The selected MethodCall prefix has no canonical callable
target in the resolver product, so its call boundary must remain an outer
callable-plan obligation until a separate callable-target authority is
accepted. `After` and tail must not be inferred from the Loop-only rows.

## Accepted common design

The callable profile uses the existing common chain. It does not introduce a
callable-specific Recipe, JoinSig, SSA, PHI, or physicalizer owner.

```text
resolver ledger + MAP-S1
  -> common Recipe producer (logical keys and operations)
  -> LoopRecipeVerifierV1
  -> LoopJoinSigElaboratorV1
  -> source-bound Core/effect verifier
  -> common input + After/Tail envelope
  -> common physical demand (later)
  -> CanonicalSsaFunctionSessionV2 (sole ValueId/CFG/PHI owner)
  -> VerifiedFunctionCompletionV1 / DraftSeal (sole terminal owner)
```

The common co-seal is named `VerifiedLoopRecipeCoSealV1` for design purposes.
It is move-only and contains the already verified common Core plus the
profile-neutral operation-source, input-source, semantic-context, and
After/Tail capabilities. It may be implemented as a thin extension of
`VerifiedLoopCoreProductV1`; it must not create a second Core authority.

### Source role to common product

| MAP-S1 role | Common logical product | Common effect/source relation | Owner and boundary |
| --- | --- | --- | --- |
| `InitialCarrier` | `LoopRecipeCarrier(entry_value)` and one `InputSourceRelation` | initializer site, carrier binding, literal/source value | Recipe producer issues keys; input relation preserves the preheader source. The literal is not silently moved into the loop body. |
| `ConditionRead` | `ReadBinding -> value` | `SourceRead` plus exact operation/item/value site row | Recipe producer maps; BindingSSA later consumes the verified relation. |
| `ConditionBound` | `ConstI64(1) -> value` | literal site and value key | Recipe producer maps the admitted literal; no AST/name re-discovery. |
| `ConditionOperator` | `CompareI64(Less)` | compare item/result and operator site | Recipe/JoinSig own logical semantics; no physical identity. |
| `StepRead` | `ReadBinding -> value` | `SourceRead` plus exact operation/item/value site row | Same binding as the carrier is verified before publication. |
| `StepDelta` | `ConstI64(1) -> value` | literal site and value key | Exact profile policy only. |
| `StepOperator` | `BinaryI64(Add) -> value` | arithmetic item/result and operator site | Recipe producer maps; no inline or Builder route. |
| `StepWrite` | `WriteBinding(binding, value)` | `SourceWrite` plus exact assignment target | One rebind of the carrier; target and lhs must agree. |
| `PrefixBoundary` | outside `LoopRecipeV1`/`JoinSig` | outer callable-prelude boundary receipt | MAP preserves optional direct target; absence is explicit and is not repaired by name. |
| `TailReturnRead` | outside `LoopRecipeV1`/`JoinSig` | common `AfterTailEnvelope` with terminal return site and lexical `BindingRef` | Tail returns the prefix `value` binding in the selected profile; it is not the loop carrier After binding. |
| loop source/frame | `VerifiedLoopSemanticContextV1` | owner/origin/source-kind, loop source, frame, Scope/Region | Resolver/MAP issue and seal it; physical session only consumes the brand. |

The selected logical loop shape is deliberately small:

```text
L0 root; B0 condition; B1 body
K0 = i:I64
V0 = preheader initializer input
V1 = Read(K0); V2 = ConstI64(1); V3 = Less(V1,V2)
V4 = Read(K0); V5 = ConstI64(1); V6 = Add(V4,V5)
I0..I6 = read, const, compare, read, const, add, write
C0 = (L0, K0, I64, V0)
```

`LoopRecipeV1.inputs` alone is insufficient to prove that `i = 0` came from
the prefix. The co-seal therefore requires a profile-neutral
`VerifiedLoopInputRelationV1` carrying the initializer statement/expression
sites, source `BindingRef`, recipe binding/value key, class, and declaration
origin. A later physical owner may materialize that preheader input; the
Recipe producer must not synthesize a loop-body constant to hide the gap.

The operation-source relation is likewise profile-neutral and must retain the
exact Recipe item key, operation kind, input/output value keys, optional source
binding, role, and source site. Ordinal-only anchors are insufficient.

The common After/Tail capability is profile-neutral. It carries owner, frame,
loop key, the opaque logical After binding when one exists, the exact terminal
return statement/value sites, and the already verified function completion
contract. If the return ABI or completion owner cannot be sealed, the result
is `NoSafeSlice`; no Generic or callable-specific After product is invented.

### Authority and exact consumption

| Concern | Sole authority |
| --- | --- |
| source membership, BindingRef, owner/origin/source-kind, frame, Scope/Region | resolver ledger and MAP-S1 |
| Loop/Block/Item/Value/Binding/Carrier keys and logical operations | common Recipe producer |
| logical ports and edges | `LoopJoinSigElaboratorV1` |
| operation/input source relations and binding effects | common source-bound co-seal/verifier; BindingSSA owns their later physical interpretation |
| ValueId, CFG, PHI, ownership SSA | `CanonicalSsaFunctionSessionV2` only |
| terminal return, cleanup, publication | `VerifiedFunctionCompletionV1` / DraftSeal only |

Every MAP row is consumed exactly once by one typed relation keyed by
`(source site, role, target kind)`. The mapper consumes the move-only map; no
consumer re-resolves by name, AST path suffix, ordinal, or `variable_map`.
Missing, duplicate, foreign, unconsumed, or cross-owner rows reject before
any physical effect. A second Recipe/SSA/PHI/After owner is `NoSafeSlice`.

## Required design output (satisfied by this decision)

One compact table must close each source role to its common logical product,
effect/BindingSSA relation, and completion owner:

```text
InitialCarrier       -> carrier/entry relation
ConditionRead        -> condition read
ConditionBound       -> constant operand
ConditionOperator    -> compare operator
StepRead             -> recurrence read
StepDelta            -> recurrence constant
StepOperator         -> recurrence arithmetic
StepWrite            -> one exact rebind
PrefixBoundary       -> outer callable prelude
TailReturnRead       -> outer callable After/Tail
Loop source/frame    -> Scope/Region + physical session brand
```

The table above defines one sealed `JoinSig`/completion shape, one source
coverage key, and one exact-consumption rule for the later mapper. If the
common products cannot represent a future profile without a second owner, the
outcome remains `NoSafeSlice`; the implementation row may not widen this
mapper opportunistically.

## Explicit non-claims after design acceptance

```text
Recipe producer             = 0
ValueId / PHI / CFG         = 0
canonical physicalizer      = 0
production caller/selection = 0
retry/fallback/reselection  = 0
legacy retirement/deletion  = 0
```

## Acceptance

- worker-reviewed source-role → Recipe/JoinSig/effect/After/Tail table is
  stored in this task and the companion Recipe SSOT;
- one authority is named for every logical key, BindingSSA effect, scope/frame,
  completion, and physical session;
- prefix target absence and tail's separate prefix binding are explicit, not
  silently inferred;
- negative matrix covers missing/duplicate/foreign source or frame, binding
  mismatch, unsupported policy, absent After/Tail, cross-owner pairing, and
  any second Recipe/SSA/PHI owner;
- implementation entry is one bounded caller-zero Recipe co-seal row,
  with focused tests and `docs/reference/**` updated in the same implementation
  commit;
- physicalization and production selection remain closed until the design's
  fresh-session and exact-consumption gates are separately accepted.

## Stop rule

Do not touch Builder/MIR, open a physicalizer, or delete legacy callers in the
design closeout. Do not create deeper D4-style suffixes. After this decision,
open only the bounded implementation task named in the companion card, then
stop again before physicalization. The implementation commit must update the
exact `docs/reference/**` receipts in the same commit; this is mandatory, not
postponed documentation work.
