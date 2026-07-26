---
Status: accepted; implementation active
Date: 2026-07-26
Decision: STATIC-BOX-DERIVE-COMPAT-D0
Classification: macro-policy / Main-entry contract boundary
Blocked row: OWN-GRAM-REJECT0 Hako transport half
Accepted option: A
First executable row: STATIC-BOX-DERIVE-COMPAT0-S0
---

# Static-box default derive compatibility

## Observed baseline failure

The Stage-B return-type guard fails before its ownership-result candidate is
observed:

```text
[mir/main-expansion/preflight]
StaticChildMustBeStatic { method: "equals" }
```

The failure reproduces in a clean worktree without the parked Hako WIP.

## Established ownership

```text
MacroEngine default derive
  -> injects `equals` / `toString` into every BoxDeclaration
  -> generated methods have `is_static = false`

VerifiedMainExpansionV1
  -> accepts only static children in `static box Main`
```

The second rule is the correct source-entry contract. `equals` requires a
receiver, so it is not a static `Main` child. Main expansion must not ignore
the name, reclassify the method, or add an entry-specific exception.

## Accepted decision

### A — skip receiver-based default derives for static boxes

```text
static box
  -> default Equals / ToString derive = absent

ordinary box
  -> current default derive behavior = unchanged
```

This is the smallest coherent policy: a static box has no instance receiver,
so receiver-based generated methods are inapplicable. A future static-safe
derive needs its own explicit macro contract and is not implied by this row.

The decision is final for this row:

```text
static box:
  receiver-based default Equals / ToString derive = absent

ordinary box:
  existing default derive behavior = unchanged

explicit user methods:
  preserved

Main expansion exception        = 0
static-safe derive semantics     = 0
ownership grammar activation    = 0
fallback / environment selector = 0
```

### Rejected B — define a static-safe derive family now

This requires new semantics for generated static methods, including receiver
substitution, callable shape, and compatibility behavior. It is a separate
language/macro capability, not a baseline repair.

### Rejected C — weaken Main expansion

Ignoring or special-casing `equals` in Main expansion would make source-entry
validity depend on generated method names. It duplicates macro policy in the
entry owner and is forbidden.

## First executable row

```text
STATIC-BOX-DERIVE-COMPAT0-S0
```

This is one bounded macro-policy correction. It is not a new grammar or
source-entry capability.

```text
macro derive policy checks BoxDeclaration::is_static once
-> static box emits no receiver-based default derives
-> ordinary box derive behavior unchanged
-> explicit user-defined equals / toString remain untouched

fixtures:
  static utility box has no generated equals / toString
  static Main has no generated equals / toString
  ordinary box retains generated equals / toString
```

Exact source boundary:

```text
src/macro/engine.rs
  sole BoxDeclaration expansion arm

src/tests/macro_derive_test.rs
  existing derive regression module
```

The implementation must not change `build_equals_method()`,
`build_tostring_method()`, `src/mir/builder/main_expansion.rs`, parser,
resolver, Builder, MIR, runtime, backend, or the parked Hako transport.

## Acceptance

```text
receiver-based derive policy reads is_static       = 1
static box generated equals                        = 0
static box generated toString                      = 0
ordinary box generated equals                      = existing 1
ordinary box generated toString                    = existing 1
explicit static-box methods removed                = 0

Main expansion exception                           = 0
new static-safe generated method family            = 0
new environment toggle                             = 0
fallback / retry                                   = 0

new shell guard                                    = 0
all modified source/test files                     < 800 lines
```

Focused verification:

```bash
cargo test -q --lib macro_derive
cargo build --release --bin hakorune
bash tools/checks/k2_wide_stageb_return_type_annotation_alignment_guard.sh
```

The Stage-B guard is an end-to-end blocker discovery surface, not a promise
that this S0 closes every later Stage-B issue. S0 closes when the
`StaticChildMustBeStatic { method: "equals" }` failure is absent and the
focused macro fixtures are green.

Read-only worker evidence already found a later independent failure with macro
expansion disabled:

```text
[plan/freeze:contract]
generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(28) }
```

This observation must be rerun after S0. It must not be repaired in the macro
policy commit.

## Task order

```text
STATIC-BOX-DERIVE-COMPAT-D0-CLOSEOUT
  accepted here

-> STATIC-BOX-DERIVE-COMPAT0-S0
  one macro policy fact
  + ordinary/static/static-Main focused fixtures

-> post-S0 Stage-B blocker rerun
   if green:
     OWN-GRAM-REJECT0-HAKO0-S0

   if MissingTransientType reproduces:
     STAGEB-GENERIC-LOOP-TRANSIENT-TYPE-D0
     -> selected bounded repair
     -> OWN-GRAM-REJECT0-HAKO0-S0

   if another failure appears:
     stop at its exact typed owner

-> OWN-GRAM-REJECT0-G0
  Rust/Hako inactive ownership-result reject parity
```

The Hako row remains parked until the Stage-B baseline reaches its candidate.
Its WIP must stay outside the S0 commit.

## Proof budget

```text
ceremony_tier             = T1 bounded implementation after T2 policy decision
proof_inventory_before    = existing ordinary derive test + existing Stage-B guard
new_proofs                = focused static/static-Main cases in existing Rust module
new_shell_guards          = 0
retired_or_merged_proofs  = 0
temporary_scaffolding     = 0
sunset                    = none; fixtures are durable behavior regressions
```

No ownership grammar, AST syntax, resolver, MIR, runtime, backend, default
route, or fallback change is authorized by this decision.

## Evidence

```text
src/macro/engine.rs
  default derive injects `equals` / `toString` for every box
  generated `equals` is an instance method

src/mir/builder/main_expansion.rs
  static Main child contract rejects non-static `equals`

tools/checks/k2_wide_stageb_return_type_annotation_alignment_guard.sh
  fails before the OWN-GRAM-REJECT0 Hako candidate
```

## Non-claims

```text
Stage-B full guard green
generic-loop MissingTransientType repair
ownership result syntax activation
Hako ownership reject transport
Main expansion weakening
static-safe Equals / ToString generation
parser / AST / resolver / MIR / runtime / backend change
default route change
fallback
```

## S0 closeout evidence

```text
source change:
  MacroEngine checks is_static once before selecting either receiver-based
  default derive

focused fixtures:
  ordinary box              = equals / toString present
  static utility + deriveAll= no receiver methods
  static Main               = main retained; no receiver methods

verification:
  cargo test -q --lib macro_derive                  = green
  cargo build --release --bin hakorune              = green
  Stage-B guard after the fresh release build:
    StaticChildMustBeStatic { equals }              = absent
    next exact blocker = MissingTransientType { init: ValueId(28) }
```

The Stage-B rerun therefore proves the macro-policy correction and moves the
active stop to `STAGEB-GENERIC-LOOP-TRANSIENT-TYPE-D0`. The Hako ownership
transport remains parked; it was not staged or changed by S0.
