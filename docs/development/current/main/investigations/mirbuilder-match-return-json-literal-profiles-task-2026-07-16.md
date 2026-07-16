---
Status: accepted; MR-SD0-I1 closed; next code-facing row is MR-IS0-I1
Date: 2026-07-16
Decision: exact whole-match literal profiles, not an open literal allow-list
Parent blocker: JSON-NATIVE-ITER0-CUT0
Resume target: JSON-NATIVE-ITER0-CUT0
Scope: two compiler BoxCount rows; CUT0 BoxShape stays stashed and untouched
---

# Match-return JSON literal profiles task

## Outcome

`JSON-NATIVE-ITER0-CUT0` exposed an existing compiler expressivity boundary.
The iterative parser itself is not the owner of the failure.

```text
[plan/freeze:unsupported]
match return else must be literal
```

The relevant `.hako` sources use two exact return-only `match` families:

```text
Family SD:
  variable scrutinee
  String labels
  String arm results
  exact Null else

Family IS:
  variable scrutinee
  Integer labels
  String arm results
  String else
```

The existing `MatchReturnFacts` accepts only Integer/Bool labels and
Integer/Bool results. The downstream composer already materializes String and
Null constants, registers their existing MIR types, emits existing Compare and
Return operations, and requires no PHI because every branch terminates.

The selected order is:

```text
MR-SD0-I1
  -> MR-IS0-I1
  -> restore CUT0 stash by immutable hash
  -> JSON-NATIVE-ITER0-CUT0
  -> HMI-S0-T0
```

These are two BoxCount commits followed by the already-selected CUT0 BoxShape
commit. They must never coexist as uncommitted edits in one worktree.

## Current source inventory

Exactly four `return match` sites exist under `apps/lib/json_native`.

| Source | Labels | Arm results | Else | Reachability |
| --- | --- | --- | --- | --- |
| `lexer/tokenizer.hako::keyword_to_token_type` | String | String | Null | production tokenizer |
| `lexer/token.hako::TokenFactory.char_to_token_type` | String | String | Null | imported compile surface; repository runtime callers 0 |
| `lexer/token.hako::TokenFactory.keyword_to_token_type` | String | String | Null | imported compile surface; repository runtime callers 0 |
| `utils/escape.hako::int_to_hex_digit` | Integer | String | String | stringify/escape path |

No json_native site requires Float, Void, TypedInteger, Null labels, arbitrary
expressions, or an unconstrained cross-product of label/result kinds.

## CUT0 parking law

The CUT0 WIP is parked as an immutable stash commit:

```text
stash commit:
  c0cfc7bddda8b4ca3b7bc4bd68a096440fbb9df4

base / taskization HEAD:
  a4901f3cc783d4a8172ec1862a3ebdd44e1621a1

tracked CUT0 paths:
  16

untracked CUT0 paths:
  2
```

Do not refer to this WIP by `stash@{n}` because the ordinal is mutable. Do not
drop or pop it before the pushed CUT0 commit is verified.

The compiler rows must not edit:

```text
apps/lib/json_native/parser/**
apps/lib/json_native/core/node.hako
JsonNode.parse callers
json_native parser authority guard
CUT0 parser docs
```

## Selected semantic product

Do not widen two independent predicates into:

```text
labels = Int | Bool | String
results = Int | Bool | String | Null
```

That would silently admit unused combinations such as String-to-Integer,
Bool-to-String, Integer-to-Null, and arbitrary mixed String/Null arms.

Instead, `MatchReturnFacts` seals one whole-match profile witness:

```rust
pub enum MatchReturnLiteralProfileV1 {
    LegacyIntegerBool,

    StringDispatch {
        // all labels: String
        // all arm results: String
        // else: Null
    },

    IntegerToString {
        // all labels: Integer
        // all arm results: String
        // else: String
    },
}
```

`LegacyIntegerBool` preserves the current accepted subset. `MR-SD0-I1` adds
only `StringDispatch`; `MR-IS0-I1` adds only `IntegerToString`.

```text
Facts:
  observes the complete match
  selects exactly one profile
  stores the profile witness

Composer:
  consumes sealed LiteralValue rows
  does not select or re-check a profile

Lower:
  consumes the CorePlan
  does not read source literal kinds
```

No method-name, file-name, HMI-field, runtime-value, or backend probing is
allowed.

## Authority split

| Concern | Authority | Non-authority |
| --- | --- | --- |
| backend-safe token snapshot decision | `lang/src/compiler/lib/match_return_facts.hako` | parser/CUT0 |
| Rust full-AST production admission | `match_return_facts.rs` kept in parity with the adopted snapshot | composer/Lower |
| exact source literal identity | AST `LiteralValue` | method names/runtime tags |
| whole-match profile | `MatchReturnLiteralProfileV1` in sealed facts | independent label/result allow-lists |
| String/Null constant materialization | existing `branchn_return.rs::alloc_const_effect` | Facts |
| String equality | existing MIR Compare/backend paths | MatchReturn profile |
| control topology | existing BranchN CorePlan | json_native parser |
| oracle/parity rows | existing match-return Rust-oracle fixture and Hako parity gate | CUT0 tests |

The adopted Hako owner is scoped to a backend-safe token snapshot. This task
does not claim full AST traversal, Freeze construction, BranchN composition,
return lowering, backend lowering, route selection, or Source Selfhost.

## MR-SD0-I1 — exact String dispatch

Status: closed.

### Accepted grammar

```text
scrutinee:
  Variable only for this profile

arm count:
  2 or more

every label:
  String literal

every explicit arm result:
  String literal

else:
  exact Null literal
```

Representative source:

```hako
classify(s) {
    return match s {
        "true" => "TRUE",
        "false" => "FALSE",
        _ => null
    }
}
```

### Required implementation

```text
Hako snapshot owner:
  classify the whole two-arm token snapshot as StringDispatch

Rust Facts:
  classify every actual arm plus else as StringDispatch
  store MatchReturnLiteralProfileV1::StringDispatch

Composer/Lower:
  production changes = 0

MIR vocabulary:
  new instructions/types = 0

Ownership operations:
  CopyOwned/DestroyOwned/ReleaseStrong = 0
```

### Pass fixtures

```text
first String arm
later String arm
Null else
String Const labels
String Eq Compare
String Const terminal return
Null Const terminal return
strict/dev source execution
release source execution
Hako snapshot parity
```

### Reject fixtures after SD0

```text
String label + Integer arm
String label + Bool arm
String label + explicit Null arm
String label + String else
String literal scrutinee
Null/Float/Void label
Float/Void/TypedInteger result
non-literal arm
non-literal else
one arm
Integer label + String result remains rejected until IS0
```

### MR-SD0-I1 closeout

```text
whole-match witness:
  LegacyIntegerBool
  StringDispatch

Hako/Rust parity:
  12/12

StringDispatch:
  String labels
  String arms
  exact Null else

IntegerToString before IS0:
  strict reject
  match_return_literal_profile_mismatch

focused source:
  phase29at_match_return_string_dispatch_min.hako

explicit MIR reference:
  legacy strict/release green
  StringDispatch strict/release green

quick:
  66/66

source/check files at or above 800 lines:
  0
```

## MR-IS0-I1 — exact Integer-to-String lookup

This row begins only after SD0 is committed, pushed, and green.

### Accepted grammar

```text
scrutinee:
  Variable only for this profile

arm count:
  2 or more

every label:
  Integer literal

every explicit arm result:
  String literal

else:
  String literal
```

Representative source:

```hako
digit(n) {
    return match n {
        0 => "0",
        10 => "a",
        _ => "0"
    }
}
```

### Pass fixtures

```text
first Integer arm
later Integer arm
String else
16-arm EscapeUtils.int_to_hex_digit shape
int_to_hex4 representative digits
strict/dev and release execution
Hako snapshot parity
```

### Reject fixtures after IS0

```text
Integer label + Null else
Integer label + mixed String/Bool arms
Bool label + String result
Float/Void/TypedInteger label or result
arbitrary expression result
```

## Hako/Rust parity update law

Rust-only widening is forbidden. Each code row updates the adopted Hako token
snapshot and its pinned parity artifacts in the same commit.

Expected surfaces:

```text
lang/src/compiler/lib/match_return_facts.hako
src/mir/builder/control_flow/plan/facts/match_return_facts.rs
src/mir/builder/control_flow/plan/facts/reject_reason.rs when taxonomy changes
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-match-return-facts-rust-oracle-v0.json
  mirbuilder-match-return-facts-token-snapshot-hako-adoption-decision-v0.json
tools/checks/rust_lifecycle_mirbuilder_match_return_facts_parity_gate.sh
tools/checks/rust_lifecycle_mirbuilder_match_return_facts_token_snapshot_hako_adoption_decision_guard.sh
focused source fixture/gate
```

Pinned hashes, row counts, required row IDs, and normalized summaries must be
updated from the new exact rows. Historical semantic types are not retained as
test-only fallbacks.

## Diagnostic law

Shape and unsupported literal kinds should remain distinguishable.

```text
non-literal else:
  MatchReturnElseNotLiteral

literal else outside every selected profile:
  MatchReturnElseLiteralTypeUnsupported

non-literal arm:
  MatchReturnArmNotLiteral

literal arm/label outside every selected profile:
  typed profile mismatch reason
```

If the new reason vocabulary is added, update `reject_reason.rs`, its handoff
table, `planner-entry-guards-ssot.md`, Rust tests, and Hako parity tokens in the
same row. Do not add raw Freeze strings in Facts.

## Gates per compiler row

Minimum focused gates:

```text
cargo test -q match_return_facts
bash tools/checks/rust_lifecycle_mirbuilder_match_return_facts_parity_gate.sh
bash tools/checks/rust_lifecycle_mirbuilder_match_return_facts_token_snapshot_hako_adoption_decision_guard.sh
cargo build --release --features vm-reference --bin hakorune
strict/release `target/release/hakorune --backend mir` execution:
  existing legacy fixture
  focused StringDispatch or IntegerToString fixture
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

The historical `match_return_*_vm.sh` scripts currently select the explicit
VM-Hako lane. That lane independently freezes while compiling
`RecipeFactsV0Box.analyze_root` and is not an MR-SD0/IS0 acceptance authority.
Do not hide that failure or reinterpret it as a MatchReturn regression. The
exact compiler-row execution authority is the feature-gated Rust MIR reference
backend named above. CUT0's later actual-consumer validation must inventory the
VM-Hako blocker separately if it remains after IS0.

Before CUT0 resumes, additionally prove the actual consumers compile without
the old Freeze:

```text
JsonTokenizer null/true/false/unknown keyword path
EscapeUtils int_to_hex4 representative path
json_pp VM/LLVM parity where LLVM is available
```

A smoke script that prints `[FAIL]` but exits zero is not green evidence. The
task must inspect the semantic result or repair the smoke harness in a separate
test-infrastructure row if required.

## CUT0 resume law

CUT0 may resume only when both compiler commits are pushed and the worktree is
clean.

Apply the parked WIP by immutable hash without dropping it:

```bash
git stash apply c0cfc7bddda8b4ca3b7bc4bd68a096440fbb9df4
```

After apply, verify the changed-path allow-list is exactly the parked CUT0
surface. In particular:

```text
src/mir/** changes from stash: 0
lang/src/** changes from stash: 0
compiler fixture/guard changes from stash: 0
```

If semantic conflicts occur, stop. Do not resolve them by mixing compiler and
parser edits. Rebase or reconstruct the CUT0 WIP deliberately.

Drop the stash only after the CUT0 commit is pushed and its gates are green.

## Required counters

```text
whole-match literal profile owners = 1
independent open literal allow-lists = 0
Lower source literal reclassification = 0
method/file-name profile inference = 0

new MIR opcodes = 0
new MIR types = 0
new backend routes = 0
new ABI rows = 0
new PHI authority = 0

Hako snapshot parity drift = 0
Rust-only semantic widening = 0
source if-chain workarounds = 0
fallback/retry = 0

CUT0 files touched during compiler rows = 0
compiler files touched by CUT0 stash = 0
VM MAX_CALL_DEPTH delta = 0
HMI production caller delta = 0

CopyOwned = 0
DestroyOwned = 0
selected-route ReleaseStrong = 0

touched source/check files at or above 800 lines = 0
```

## Implementation may claim after both rows

```text
the existing return-only MatchReturn path accepts two additional exact
whole-match profiles required by json_native

StringDispatch accepts only String labels, String arms, and exact Null else

IntegerToString accepts only Integer labels and String arms/else

existing BranchN, Const, Compare, Return, VM, and LLVM representations are
reused without a new opcode, type, ABI, PHI owner, or fallback

the adopted Hako token snapshot and Rust production Facts remain in parity

JSON-NATIVE-ITER0-CUT0 may be restored as an isolated BoxShape commit
```

## Implementation must not claim

```text
general literal MatchReturn support
arbitrary String/Null/Integer/Bool cross-products
String literal scrutinee support
Float/Void/TypedInteger match support
non-literal arm/else support
general match expressions outside return-only BranchN
full-AST Hako adoption
Source Selfhost
ownership semantics
HMI semantic seal or opcode execution
CUT0 completion before the parked WIP is restored and verified
```

## Stop conditions

Stop the compiler row if it requires:

1. a new MIR opcode, MIR type, ABI, backend route, or runtime target probe;
2. a PHI/join or hidden String-to-Null conversion;
3. independent label/result allow-lists that admit unobserved cross-products;
4. Lower re-reading source literal classes;
5. Float, Void, TypedInteger, Null labels, or arbitrary expression widening;
6. full-AST Hako adoption instead of the scoped token snapshot;
7. strict-to-release, compiler-route, or backend fallback;
8. rewriting json_native `match` expressions into `if` chains;
9. touching CUT0 parser/JsonNode files in the compiler commit;
10. a source/check file at or above 800 lines.

Stop CUT0 resume if:

1. the stash changed-path allow-list drifts;
2. applying the stash touches `src/mir/**` or `lang/src/**`;
3. semantic conflicts require compiler edits inside CUT0;
4. the parser authority guard needs method names or payload-shape inference;
5. a retained parser/json smoke regresses outside the selected cutover;
6. any retry, fallback, parser toggle, or VM call-depth change is needed.

## Final decision lock

> The json_native CUT0 blocker is a compiler BoxCount prerequisite, not a
> parser BoxShape defect. MatchReturn admission will be widened through two
> exact whole-match witnesses only: `StringDispatch` first, then
> `IntegerToString`. The adopted Hako token-snapshot owner and Rust production
> Facts change in the same commit for each row; the existing composer, CorePlan,
> MIR, VM, and LLVM paths remain consumers and do not reclassify the source.
> CUT0 stays parked at stash commit
> `c0cfc7bddda8b4ca3b7bc4bd68a096440fbb9df4` until both compiler rows are
> committed, pushed, and green. No `.hako` rewrite, literal cross-product,
> fallback, or mixed compiler/parser commit is permitted.
