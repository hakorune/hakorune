---
Status: design freeze accepted; bounded BoxShape R0 selected; implementation not started
Date: 2026-08-23
Decision: MIRBUILDER-SCRIPT-DIRECT-STATIC-SEMANTIC-SHELF-R0
Parent: mirbuilder-structure-refactor-queue-d0-2026-08-23.md
Classification: behavior-neutral physical shelf; no semantic or production-route change
---

# MirBuilder Script direct-static semantic shelf R0

## Six-line brief

```text
Decision: Accept narrowed B-prime: move only the existing Recipe producer and operational claim ledger, together with their test siblings, into one Builder-owned semantic-protocol shelf.
Source authority + canonical issuer: parser-backed Script lookup remains in source_call_target; VerifiedScriptDirectStaticRecipeV1::issue remains the sole Recipe/key issuer; Bundle and Join remain the claim ledger's semantic authorities.
Non-authority: directory names, re-exports, lookup, claim transport, Join children, physical publication/bridge/exit, compiler capability, tests, names, and line counts cannot issue or replace source/Recipe/physical meaning.
Fail-fast boundary: stop before the first move if logical module identity, visibility, file bytes, caller set, test ownership, or route/fallback edges cannot be preserved by path-only edits.
Smallest next slice: four byte-identical file moves plus two parent #[path] changes; no module facade, re-export, body edit, use-path edit, fixture, or production switch.
Non-claims: no Join subtree move, lookup/admission move, source or backend owner migration, route retirement, physical activation, semantic widening, performance claim, or broad normal_script shelf.
```

## Decision and vocabulary correction

The broad `normal_script/direct_static/` move is rejected because the named
files span four owner flows and six physical roots. A re-export facade is also
rejected: it would add an alias surface without changing physical ownership
and could preserve an old import bypass.

The accepted candidate is a physical shelf only:

```text
src/mir/builder/normal_script/direct_static/semantic/
```

It contains the smallest independently movable Builder semantic-protocol
pair. The shelf is not a Rust module and introduces no new authority.

External review vocabulary is normalized to the current code. There is no
production type named `NormalScriptDirectStaticRecipeIssuerV1` and no
`RecipeV1::claim(self)` API. The current SSOT names are:

```text
Recipe/key issuer:
  VerifiedScriptDirectStaticRecipeV1::issue

linear claim owner:
  ScriptDirectStaticClaimLedgerV1
    ::issue_direct / ::complete_no_direct
    ::take / ::complete / ::finish
```

The ledger is not a second semantic issuer. Its module contract says Bundle
and Join remain semantic authorities; the ledger only co-seals their rows for
one lowering scope and makes consumption linear.

## Authority map

| Layer | Existing owner / issuer | R0 disposition |
| --- | --- | --- |
| Parser-backed Script call observation and lookup | `source_call_target/script_direct_static.rs`; `VerifiedScriptDirectStaticCallLookupV1::issue_from_program_loan` | Excluded; stays source/lookup-owned |
| Script direct-static Recipe and producer-local key | `normal_script_direct_static_recipe.rs`; `VerifiedScriptDirectStaticRecipeV1::issue` | Move byte-identically |
| One-scope claim state | `normal_script_direct_static_claim_ledger.rs`; `ScriptDirectStaticClaimLedgerV1` | Move byte-identically |
| Bundle, result-publication demand, Join, required-argument proof | Existing sibling owners and mixed-owner Join subtree | Excluded |
| Claim transport and semantic-lowering input | Existing transport owners | Excluded |
| Generic Call, ExactI64 publication, Script physical exit | Existing physical bridge/publication/session owners | Excluded |
| Function/backend capability | `canonical_direct_static_call_capability.rs` and backend capability | Excluded |

## Current production chain

The move must preserve this existing chain exactly:

```text
NormalScriptPreEffectSourceObservationIssuerV1
  -> issue_into_c_transport
  -> CanonicalScriptCBoundSourceV1::consume_into_lowering_source
  -> VerifiedScriptDirectStaticRecipeV1::issue
  -> ScriptDirectStaticClaimInputV1::DirectStaticClaims
  -> ScriptDirectStaticClaimLedgerV1::issue_direct
  -> claim transport peek/take
  -> calls::member_route
  -> lower_claimed_script_direct_static_v1
  -> claim complete
  -> finish_direct_static_claims
```

Current census frozen for R0:

```text
VerifiedScriptDirectStaticRecipeV1::issue production caller = 1
  src/mir/builder/normal_script_a/consumer.rs

ScriptDirectStaticClaimLedgerV1::complete_no_direct production caller = 1
ScriptDirectStaticClaimLedgerV1::issue_direct production caller = 1
  src/mir/builder/normal_script_semantic_lowering_state.rs

claim take / complete production owner =
  src/mir/builder/normal_script_direct_static_claim_transport.rs

claim finish production owner =
  src/mir/builder/raw_invocation_source_transport.rs

lower_claimed_script_direct_static_v1 production caller = 1
  src/mir/builder/calls/member_route.rs

detached lower_direct_static_physical_input_v1 production caller = 0
```

The detached physical-input kernel remains outside this R0. Its caller-zero
state is not used to claim a production physical cutover.

## Exact move manifest

All four moved files retain their basenames so the production files' existing
test `#[path]` declarations continue to resolve without body edits.

| Old path under `src/mir/builder/` | New path under `src/mir/builder/normal_script/direct_static/semantic/` | Logical module |
| --- | --- | --- |
| `normal_script_direct_static_recipe.rs` | `normal_script_direct_static_recipe.rs` | `crate::mir::builder::normal_script_direct_static_recipe` |
| `normal_script_direct_static_recipe_tests.rs` | `normal_script_direct_static_recipe_tests.rs` | existing cfg(test) child |
| `normal_script_direct_static_claim_ledger.rs` | `normal_script_direct_static_claim_ledger.rs` | `normal_script_semantic_lowering_state::direct_static_claim_ledger` |
| `normal_script_direct_static_claim_ledger_tests.rs` | `normal_script_direct_static_claim_ledger_tests.rs` | existing cfg(test) child |

Only these parent declarations may change:

```text
src/mir/builder.rs
  add #[path = "builder/normal_script/direct_static/semantic/normal_script_direct_static_recipe.rs"]
  retain mod normal_script_direct_static_recipe;

src/mir/builder/normal_script_semantic_lowering_state.rs
  replace the existing claim-ledger #[path] literal with
  #[path = "normal_script/direct_static/semantic/normal_script_direct_static_claim_ledger.rs"]
  retain mod direct_static_claim_ledger;
```

No `mod.rs` or re-export facade is created. Logical module names, all `use`
paths, visibility, cfg scope, type names, constructors, and callers remain
unchanged.

## Frozen source evidence

Pre-move line counts:

```text
builder.rs                                              738
normal_script_semantic_lowering_state.rs                270
normal_script_direct_static_recipe.rs                   333
normal_script_direct_static_claim_ledger.rs             424
normal_script_direct_static_recipe_tests.rs             199
normal_script_direct_static_claim_ledger_tests.rs        235
production move pair total                              757
```

Every production source remains below the 760-line split trigger and the
800-line hard stop. Adding the Recipe `#[path]` leaves `builder.rs` at 739.

Pre-move SHA-256:

```text
recipe              2e0f4cf84425bfeb7366c90ee0a7fe2d1eb3930d24f1325fcd9fa054fd08e23e
claim ledger         65b1f698bf12dc7a68440885d08f2d488ebac55242257cf374f922b67ffac614
recipe tests         13f8fa2b70cf5388eef664d709266109a928a4114f5f30b3a49ba90bd7006928
claim-ledger tests   b69d227ad7699c598945aae23f831603960fa8b214aff15bc43ff5a3b020efe5
```

The hashes must be identical at the new paths after each move.

## Design task closure

The consultation tasks close as follows:

| Task | Result |
| --- | --- |
| D0 Authority freeze | Four flows/six roots classified; source, semantic protocol, physical, and backend owners remain separate |
| D1 Canonical chain census | Exact current production chain and caller cardinalities frozen above |
| D2 Bypass/retry census | Move set contains no fallback/retry/legacy route; R0 permits no edge delta and keeps the detached kernel caller-zero |
| D3 Move manifest | Four byte-identical moves and two parent path edits fixed above |
| D4 Preservation contract | Module identity, visibility, bytes, callers, tests, route edges, and fallback set must remain unchanged |
| D5 Guard packet | Structure guard and existing focused guards/commands fixed below |
| D6 Design freeze | Conditional SafeSlice accepted; any contract failure returns to NoSafeSlice |

## R0 implementation sequence

### Commit 1 — Recipe atom

```text
move recipe production file byte-identically
move its test sibling byte-identically
add one #[path] to builder.rs
run Recipe-focused and structural checks
```

This commit must build independently.

### Commit 2 — Claim-ledger atom

```text
move claim-ledger production file byte-identically
move its test sibling byte-identically
update one existing #[path] literal
run ledger-focused and structural checks
```

This commit must build independently.

### Commit 3 — Closeout evidence

```text
record before/after hashes and caller census
record guards/tests/check results
update CURRENT_STATE and this card
commit/push on main
```

Do not add a Join, lookup, transport, physical, capability, fixture, route, or
fallback change to any of these commits.

## Guard packet

Add one reusable structure guard for this R0. It must prove:

```text
new four paths exist exactly once
old four paths do not exist
logical mod declarations remain exactly once
parent #[path] literals match the frozen manifest
four moved-file SHA-256 values match the frozen values
new shelf has no mod.rs, pub use, alias, shim, issuer, or route function
Recipe issuer definition = 1; production issue caller = 1
ledger production constructors = 1 each at the existing lowering state
physical bridge production caller = 1 at member_route
detached physical-input kernel production caller = 0
fallback/retry/legacy delta in move set = 0
all touched production files < 760; hard stop < 800
```

Required verification after each atom:

```bash
cargo test --profile quick mir::builder::normal_script_direct_static_recipe --lib
cargo test --profile quick mir::builder::normal_script_semantic_lowering_state::direct_static_claim_ledger::tests --lib
cargo check --profile quick
bash tools/checks/script_direct_static_a_c_consumer_i0_guard.sh
bash tools/checks/script_direct_static_target_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Run only the focused test belonging to Commit 1 before the ledger move; run
the full packet after Commit 2.

## Hard NoSafeSlice conditions

Return to design stop before committing code if any of these is required:

```text
re-export, alias, compatibility stub, or old-path forwarding module
logical module name, cfg scope, or visibility change
production/test file body edit or hash drift
caller use-path or caller-set change
Recipe/key issuer or claim state-machine API change
Join parent or any mixed-owner Join child move
lookup, claim transport, semantic-input, physical, source, or backend move
new fallback, retry, relookup, direct physical edge, or production switch
fixture addition or semantic acceptance change
source/compiler capability movement in the same series
any touched production source reaching 760 lines
failure to make either atom independently buildable
unexpected HEAD, dirty overlapping work, or pointer mismatch at start
```

The goal of R0 is not to complete a feature-name shelf. It gives one proven
Builder semantic-protocol atom a truthful physical home while preserving the
existing authority and execution graph exactly.
