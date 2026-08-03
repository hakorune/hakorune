# JOINIR Nested Loop Source Forest D0

Date: 2026-08-03
Status: design stop; implementation not authorized until this contract is accepted.
Task: `JOINIR-LOOP-NESTED-SOURCE-FOREST0-D0`
Parent: `JOINIR-LOOP-RECURSIVE-RECIPE-CLOSURE0-S5`

## Decision boundary

The DirectAccum Loop family envelope is landed, but Nested cannot use it yet.
The existing `VerifiedResolvedLoopSourceV1` is a non-`Clone` witness for one
sealed Loop site only. A Nested portable recipe has a root Loop and one or more
semantic child Loops; binding only the root would leave the child source claim
unowned. Nested therefore remains caller-zero until a single source-owned,
non-`Clone` root+child forest exists.

This card fixes the source-authority boundary only. It does not add a Nested
recipe producer, a canonical plan variant, a production caller, a route policy,
a retry edge, a PHI/SSA writer, or a physicalizer.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| sealed resolved function/loop-region index | exact Loop sites and their ancestry | portable recipe meaning, route choice, MIR |
| proposed `VerifiedResolvedLoopSourceForestV1` | one consumed root+child source witness in semantic preorder | AST, raw child indices, route/family dispatch |
| `LoopSourcePathV1` projection | stable portable source coordinates after the witness is consumed | source lookup or child discovery |
| Nested facts/recipe producer | Nested semantic recipe and JoinSig input | source re-observation, PHI/SSA, Builder |
| `CanonicalCfgSessionV1` + `BindingSsaBuilderV1` + `PhiTxn` | physical CFG, reaching Binding SSA, PHI lifecycle | source forest, Nested route selection |
| `LoopPhiMaterializerV1` | caller-zero mechanical evidence only | production PHI authority |

The existing semantic owner forest is not silently reused as a Loop source
forest. It owns Function/Lambda lexical owners and capture relationships. The
new capability must consume the sealed Loop-region owner/index that already
backs `resolved_loop_source`, or fail closed if that index cannot expose exact
Nested ancestry.

## Existing evidence

- `VerifiedResolvedLoopSourceV1` is non-`Clone` and is issued only by the
  sealed `ResolvedLoopRegionIndexV1` lookup.
- `bind_resolved_loop_root_v1` projects one resolved source into the portable
  `BodyItem`/`ScopeBodyItem`/`LoopBodyItem` path grammar.
- `LoopRecipeSourceBindingV1` already requires one-to-one loop-key/path rows and
  rejects duplicate, skipped, orphan, and malformed paths.
- Nested portable goldens can describe an `Always` child, but no production
  source issuer currently owns the corresponding child path.
- The existing Nested route, composer, and route-local PHI writers remain
  legacy execution authority; their caller counts are unchanged by this card.

## Proposed product

The recommended product is a consuming, non-`Clone` witness with a sealed
preorder forest. Exact Rust names may be adjusted during implementation, but
the shape and ownership are fixed here:

```text
VerifiedResolvedLoopSourceForestV1
  owner: one declared-function source owner
  members: non-empty ordered forest members

ForestMemberV1
  source: one sealed VerifiedResolvedLoopSourceV1
  portable_path: one projected LoopSourcePathV1
  semantic_parent: None for the root, Some(parent-key) for a child
```

Required properties:

1. The root member is issued from the selected root capability, not from a
   route id, AST walk, or raw body index.
2. Child members are enumerated by the sealed resolved Loop-region owner in
   canonical preorder. Their parent relation is supplied by that owner; the
   producer never reconstructs it from syntax.
3. A child path must be the semantic parent's complete prefix followed by one
   `LoopBodyItem`, then only the permitted nested `ScopeBodyItem` steps. A path
   that skips an intermediate Loop, crosses a sibling, or points at an orphan
   body rejects before any recipe/Builder effect.
4. The forest is consumed exactly once by the source-binding adapter. It is not
   `Clone`, and no member or path may be minted after consumption.
5. `Program`/Script owner roots, unsupported scope roots, missing region rows,
   duplicate sites, duplicate parent keys, and incomplete child coverage are
   typed rejection reasons, not empty forests.
6. The portable artifact remains source-claim data only. It does not retain
   `ASTNode`, `StmtRef`, `CondBlockView`, `MirBuilder`, physical IDs, or a
   borrowed forest reference.

## Candidate issuer boundary

The issuer should be attached to the sealed resolved-function owner, near the
existing `resolved_loop_source` capability. A minimal API may accept the
selected root site and return either:

```text
VerifiedResolvedLoopSourceForestV1
  or
ResolvedLoopSourceForestRejectV1
```

The issuer may use the resolver's canonical source-site and loop-region tables.
It may not scan raw AST, call a route composer, inspect `CanonicalLoopFacts`,
or infer child membership from portable paths after the fact. If the sealed
index cannot prove exact preorder/parent coverage, the correct result is a
typed `Unavailable`/`Incomplete` rejection and the Nested row remains parked.

## Negative fixtures required before acceptance

The first implementation slice must be Builder-free and test-only. It must
pin at least:

- root plus one `Always` child in canonical preorder;
- root plus child under a lexical scope, retaining the full prefix;
- missing child region;
- duplicate child site;
- skipped intermediate Loop in a forged/incorrect lookup result;
- orphan `LoopBodyRoot`/scope-root segment;
- sibling child attached to the wrong parent;
- unsupported `Program` owner;
- deeper nesting as an explicit boundary, either accepted by the same product
  or rejected with a named bounded reason.

The tests must compare only source-site/path/parent evidence. They must not
assert MIR IDs, PHI rows, ValueIds, physical block topology, or legacy route
selection.

## Acceptance gates

```text
source authority = one sealed resolved Loop-region owner
forest product = non-Clone, consuming, root+child, canonical preorder
child path = full parent prefix + LoopBodyItem + permitted scope steps
missing/duplicate/skipped/orphan/unsupported = typed fail-fast
AST/raw-index child reconstruction = 0
Nested production caller = 0
new Recipe/JoinSig/PHI/SSA/physicalizer owner = 0
touched Rust/check files < 800 lines = true
```

Only after these gates are green may M7-S0 add a caller-zero Nested-`Always`
producer that consumes the same portable verifier/JoinSig terminal as
DirectAccum. Production `route_loop` wiring and legacy Nested PHI retirement
remain M10/M10b work.

## Explicit non-claims

This design does not prove Nested semantic parity, carrier final-value
correctness, branch/merge closure, Generic winner equivalence, physical
candidate abort, or selfhost execution. It only closes the missing source
authority needed before those claims can be tested honestly.
