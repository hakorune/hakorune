---
Status: accepted parked task order
Date: 2026-07-27
Decision: MIRBUILDER-CLEANLINESS-FEEDBACK-prime-r1
Source:
  - GLM-5.2 / Claude review feedback
Current-lane relation:
  - CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0 remains active
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

The active TYPE-I0 row is not interrupted. These tasks are parked until its
G0 and exact Stage-B probe identify a safe cleanup window.

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
still read or pinned by:

```text
builder call routing
MIR JSON emitters
Stage1 / selfhost bridges
host-provider compatibility
integration smokes
test fixtures
tool wrappers
environment-variable policy artifacts
```

Some live compatibility paths explicitly pin `0`; others pin `1`. Deleting
the flag in `call_unified.rs` alone would silently reinterpret those callers.

### M3 has an existing physical owner

`MirInstruction::NewClosure` supports a thin descriptor with an optional
`ClosureBodyId`. The canonicalization pass externalizes inline closure bodies
into the function-owned closure-body table. The compatibility
`CallTarget::Closure` branch does not own an inline body, so `body: vec![]` is
intentional; only the comment is vague.

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
this CallTarget branch owns descriptor creation only
inline closure bodies are externalized by the canonical closure-body owner
body_id / closure body table remain the execution authority
```

Verify the existing NewClosure canonicalization tests. This row changes no
instruction, body storage, backend behavior, or closure capability.

## Task family C — AI collaboration history and source comments

### `AI-COLLABORATION-CREDITS0-S0`

Create a public root `CREDITS.md` and link it once from `README.md`.

The first version records project-level collaboration among the human owner
and AI tools/models, links representative archived design history, and states
that git history is the fine-grained provenance authority. It does not claim
per-line authorship or ranking.

### `AI-COMMENT-SEMANTIC0-S0`

After the credits artifact lands, inventory only production `src/**/*.rs`
comments that name ChatGPT, Claude, GLM, or another model.

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
  non-test ambient semantic readers = 0
  + every pin=0 caller retired or moved under one named compatibility owner
  + Stage1 / MIR-JSON parity green
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
```

Detect duplicate names, missing names, and names whose classification differs
by route.

### D0 recommendation

Prefer one compile-time, source-neutral policy owner under `src/mir/policies/`
unless the census proves that an existing sealed catalog can represent every
compiler and runtime built-in at the relevant boundary.

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
```

## Priority

```text
P0 current:
  TYPE-I0 S0/I0/P0/G0
  -> exact Stage-B probe

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
current TYPE-I0 task order changed             = 0
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
