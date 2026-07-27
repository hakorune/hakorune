---
Status: accepted parked task order
Date: 2026-07-27
Decision: MIRBUILDER-CLEANLINESS-FEEDBACK-prime-r1
Source:
  - GLM-5.2 / Claude review feedback
Current-lane relation:
  - CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0 is closed
  - CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0 remains active
  - this card does not change CURRENT_STATE.toml
---

# MirBuilder Cleanliness Feedback Task Order

## Decision

The feedback is useful, but its items do not share one authority. They are
split into five independent cleanup families so that behavior-neutral cleanup,
compatibility retirement, policy centralization, and historical preservation
do not enter one commit series.

```text
M1 stale dead-code suppression:
  accepted after factual correction

M2 unified-call environment gate:
  accepted as a future census + sunset decision

M3 closure "for now" comment:
  accepted as a descriptor-contract clarification

L1 AI attribution comments:
  preserve the history outside production source
  rewrite source comments to semantic rationale

L2 box-name classification:
  accepted as a policy-SSOT census
  runtime-configurable name table is not preselected
```

The active Stage-B probe is not interrupted. These tasks are parked until that
probe identifies a safe cleanup window.

## Evidence corrections

### M1 is not permission to delete the input fields

`RawLegacyMethodCallInputV1::{receiver, method, arguments}` are read by the
blanket `MethodCallDescentPortV1` implementation, and its constructor is used
by the production Raw member-call facade. The two `#[allow(dead_code)]`
attributes are suspicious migration residue, but the fields are not proven
dead.

Therefore:

```text
field deletion                    = forbidden without a new caller census
RawLegacy -> located cutover      = not claimed
first cleanup                     = remove stale allow attributes only
warning-free cargo check required = yes
```

### M2 is a compatibility boundary, not a local flag deletion

`NYASH_MIR_UNIFIED_CALL` is default-on in the current Rust builder, but it is
not owned by one reader. The census must distinguish at least:

```text
Rust MirBuilder/config accessor
Rust MIR-JSON emitter and root selection
llvm_py legacy call instruction selection
```

These authorities are pinned or inherited by Stage1/selfhost bridges,
host-provider compatibility, integration smokes, tests, tool wrappers, and
environment-variable policy artifacts. `Phase0MirJsonEnvGuard` is an exact
live compatibility owner that pins `0`; several Stage1, VM-Hako, selfhost, and
tool paths pin or preserve `1`. Deleting the flag in `call_unified.rs` alone
would silently reinterpret those callers.

The census must also reconcile the environment inventory that says default
false with the live readers whose effective defaults are true.

### M3 has an existing physical owner

`CallTarget::Closure` carries descriptor fields only. It has neither an inline
body nor a body identity. This compatibility branch therefore emits
`NewClosure { body_id: None, body: vec![] }`. Source Lambda lowering is a
separate path that interns an executable body and supplies its identity.
Canonicalization can externalize a non-empty inline body, but it does not
synthesize a missing body. The empty vector is intentional; only the comment
is vague.

### L1 already has durable historical evidence

AI collaboration is already recorded in:

```text
git history
archived phase/design documents
README project statement
historical proposal authorship
```

The history has value. Production comments, however, should explain
invariants, authority, or failure behavior rather than celebrate a tool or
model. Preserve both by moving curated attribution to a public credits
artifact and retaining technical rationale in source.

The bounded inventory is 18 AI-name occurrences across 16 production files
under `src/mir/**/*.rs`. Attribution-only labels may be removed; comments that
also carry an invariant must be rewritten rather than deleted.

### L2 currently has two classification layers

`call_unified::classify_box_kind()` owns the large static/runtime list, while
`CalleeResolverBox::classify_box_kind()` adds resolver-local compiler boxes.
This is a real SSOT split. A configurable table or declaration catalog is not
automatically correct because compiler boxes, runtime built-ins, and user
declarations do not currently share one registration lifecycle.

## Task family A — stale suppression

### `RAW-LEGACY-METHOD-INPUT-ALLOW0-S0`

Scope:

```text
remove the two #[allow(dead_code)] attributes
retain receiver / method / arguments
retain constructor and blanket port implementation
behavior/API/caller delta = 0
```

Acceptance:

```bash
cargo check --lib
RUSTFLAGS=-Awarnings cargo test -q --lib method_call_descent
python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_arg0.py
```

If removing the attributes reveals a real warning under a supported feature
set, stop and inventory that feature/caller set. Do not delete the fields to
silence the warning.

## Task family B — closure descriptor documentation

### `CLOSURE-DESCRIPTOR-COMMENT0-S0`

Replace:

```text
body: vec![], // Empty body for now
```

with a stable contract explaining:

```text
CallTarget::Closure carries descriptor fields only
source Lambda lowering interns executable bodies separately
canonicalization does not synthesize a missing body
```

Verify the existing NewClosure canonicalization tests. This row changes no
instruction, body storage, backend behavior, or closure capability.

Whether the descriptor-only `CallTarget::Closure` branch is still needed is a
separate caller census. Do not mix its retirement into this comment row.

## Task family C — AI collaboration history and source comments

### `AI-COLLABORATION-CREDITS0-S0`

Create a public root `CREDITS.md` and link it once from `README.md`.

The first version records project-level collaboration among the human owner
and AI tools/models, links representative archived design history, and states
that git history is the fine-grained provenance authority. It does not claim
per-line authorship or ranking.

Use three explicit evidence layers:

```text
root CREDITS.md                    = public project-level memorial/index
archived philosophy/design docs   = detailed historical evidence
git history                       = fine-grained provenance authority
```

Link `CREDITS.md` once from the existing README creator section. Do not
duplicate the link throughout the README or place permanent attribution under
`docs/development/current/**`.

### `AI-COMMENT-SEMANTIC0-S0`

After the credits artifact lands, inventory only production `src/mir/**/*.rs`
comments that name ChatGPT, Claude, GLM, or another model. The current bounded
baseline is 18 occurrences across 16 files.

For every row:

```text
preserve technical invariant/rationale
remove celebratory or attribution-only wording from production source
leave archived historical documents unchanged
behavior delta = 0
```

Do not mechanically delete a whole comment when it also carries an invariant.
Do not add a source comment linking back to credits at every site.

## Task family D — unified-call gate sunset

```text
UNIFIED-CALL-MODE-SUNSET-CENSUS0-P0
-> UNIFIED-CALL-MODE-SUNSET-D0
-> UNIFIED-CALL-MODE-CANONICAL0-S0
-> UNIFIED-CALL-MODE-COMPAT-FENCE0-S0
-> UNIFIED-CALL-MODE-ENV-RETIRE0-S0
-> UNIFIED-CALL-MODE-G0
```

### Census

Classify every non-archive occurrence as:

```text
canonical pin=1
legacy compatibility pin=0
test-only
tool wrapper
environment-policy artifact
stale
```

The census must name exact `pin=0` production/compatibility consumers,
including host-provider and MIR-JSON/Stage1 lanes.

Record each row by:

```text
reader authority
caller / pin or inherited value
emitted call dialect
backend / consumer
```

Reuse the existing Stage0-keep versus Stage1-mainline dialect contract rather
than inventing a second compatibility taxonomy. The census must also name the
existing unified-call projection-policy guard and immutable fixture that a
later sunset must explicitly supersede, update, or retire.

### D0

Choose the canonical route and exact compatibility owner. Required law:

```text
normal canonical route cannot be changed by ambient environment
compatibility route must be explicitly typed/named
implicit unified -> legacy fallback = 0
```

### Sunset

```text
sunset_id =
  MIR-UNIFIED-CALL-ENV-SUNSET-001

retire_when =
  direct ambient semantic readers in Rust Builder = 0
  + direct ambient semantic readers in Rust MIR-JSON = 0
  + direct ambient semantic readers in llvm_py = 0
  + every pin=0 caller retired or moved under one named compatibility owner
  + Stage1 / MIR-JSON parity green
  + old projection-policy guard/fixture superseded, updated, or retired
  + environment docs and tool wrappers updated
```

Do not remove the environment variable before the D0 and parity evidence.

## Task family E — box-kind policy SSOT

```text
CALL-BOX-KIND-POLICY-CENSUS0-P0
-> CALL-BOX-KIND-POLICY-D0
-> CALL-BOX-KIND-POLICY0-S0
-> CALL-BOX-KIND-POLICY-MIGRATE0-I0
-> CALL-BOX-KIND-POLICY-G0
```

### Census

Inventory:

```text
call_unified static compiler names
call_unified runtime built-in names
resolver-local static compiler names
all classify_box_kind consumers
catalog/registry facts currently available at each consumer
UnknownBox and RuntimeDataBox facade/sentinel behavior
```

Detect duplicate names, missing names, and names whose classification differs
by route.

The initial audit already found an exact drift that the census must preserve
as evidence:

```text
BreakFinderBox / PhiInjectorBox / LoopSSA:
  resolver-local classifier = StaticCompiler
  base call_unified classifier = UserDefined
```

It also found that `UnknownBox` is deliberately runtime data while the
`RuntimeDataBox` facade spelling is not classified by the base list. These
rows require an explicit D0 disposition rather than a mechanical list merge.

### D0 recommendation

Prefer one compile-time, source-neutral policy owner under `src/mir/policies/`
unless the census proves that an existing sealed catalog can represent every
compiler, runtime built-in, and runtime facade at the relevant boundary.

The policy vocabulary should distinguish, or explicitly project:

```text
StaticCompiler
RuntimeBuiltin
RuntimeFacade
UserDefined conservative default
```

Do not choose:

```text
ambient config file
environment variable
caller-local string match
fallback classification inferred from method name
```

User-defined remains the conservative default unless the D0 selects a stronger
sealed authority.

### Migration law

```text
box-kind decision owner = 1
resolver-local exception list = 0
call_unified list = thin facade or retired
classification behavior delta = 0
unknown/user-defined behavior delta = 0
cross-route matrix covers every named row = 1
```

## Priority

```text
P0 current:
  exact Stage-B probe only

P1 next safe cleanup window:
  RAW-LEGACY-METHOD-INPUT-ALLOW0-S0
  -> CLOSURE-DESCRIPTOR-COMMENT0-S0

P2 history cleanup:
  AI-COLLABORATION-CREDITS0-S0
  -> AI-COMMENT-SEMANTIC0-S0

P3 policy migrations:
  UNIFIED-CALL-MODE-SUNSET-CENSUS0-P0
  -> its D0/retirement series

  CALL-BOX-KIND-POLICY-CENSUS0-P0
  -> its D0/migration series
```

M2 and L2 may be audited in parallel, but their implementations must not share
a commit or one decision owner.

## Structural gate

```text
current Stage-B task order changed             = 0
RawLegacy input field deletion                 = 0 before census
unified-call environment deletion              = 0 before sunset D0
box-name policy behavior change                = 0 before policy D0

production AI attribution comments             = inventory then semantic rewrite
archived AI collaboration history deletion     = 0
public AI collaboration credits artifact       = 1 before source cleanup

new environment variable                       = 0
silent fallback                                = 0
per-row shell guard                            = 0
all modified/new source/check files            < 800 lines
```

## Non-claims

```text
TYPE-I0 implementation or schedule preemption
RawLegacy located-input cutover
NYASH_MIR_UNIFIED_CALL immediate removal
Stage1 / MIR-JSON compatibility retirement
dynamic box-kind registry activation
parser / grammar / VM / backend change
Alias / View activation
default backend cutover
```
