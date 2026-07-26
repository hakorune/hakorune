---
Status: design stop
Date: 2026-07-26
Decision: STATIC-BOX-DERIVE-COMPAT-D0
Classification: macro-policy / Main-entry contract boundary
Blocked row: OWN-GRAM-REJECT0 Hako transport half
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

## Decision required

### A — skip receiver-based default derives for static boxes (recommended)

```text
static box
  -> default Equals / ToString derive = absent

ordinary box
  -> current default derive behavior = unchanged
```

This is the smallest coherent policy: a static box has no instance receiver,
so receiver-based generated methods are inapplicable. A future static-safe
derive needs its own explicit macro contract and is not implied by this row.

### B — define a static-safe derive family now

This requires new semantics for generated static methods, including receiver
substitution, callable shape, and compatibility behavior. It is a separate
language/macro capability, not a baseline repair.

### C — weaken Main expansion (rejected)

Ignoring or special-casing `equals` in Main expansion would make source-entry
validity depend on generated method names. It duplicates macro policy in the
entry owner and is forbidden.

## Minimal S0 after A

```text
macro derive policy checks BoxDeclaration::is_static once
-> static box emits no receiver-based default derives
-> ordinary box derive behavior unchanged

fixtures:
  static Main has no generated equals / toString
  ordinary box retains generated equals / toString
  Stage-B return-type guard reaches its candidate
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
