# HEADERPORT0-REENTRANT-TERM0-I0: source integration consultation

Status: **design stop; no production I0 implementation is authorized**
Date: 2026-07-21
Parent: `mirbuilder-headerport-reentrant-terminal-task-2026-07-21.md`
Decision prerequisite: choose one module-shell/header authority before code

## Evidence

`HEADERPORT0-REENTRANT-TERM0-S0/P0` is closed. The disconnected terminal
proof is green: capture-before-commit, short header loans, nested static and
instance children, constructors, TaskScope/FastMem body descent, Main
pre-effect rejection, primary failure, admission failure, parent restore, and
existing panic parity.

The production I0 boundary is not yet a mechanical call-site replacement:

```text
MirBuilder::build_module
  -> prepare_module
  -> lower_root
  -> finalize_module
```

`prepare_module` installs an empty `current_module` and a live `main` function.
`lower_root` directly lowers root instance/static functions and directly calls
`build_static_main_box` for the App entry. Those paths publish child functions
through `current_module` while lowering continues.

At the same time, `ModuleLoweringInvocationV1` owns a separate
`ModuleDraftCollectorV1`. The collector currently owns unpublished drafts and
header views, but it has no production module-shell aggregation operation. A
partial I0 would therefore create one of these forbidden states:

```text
collector owns child header A
current_module owns child header A'

collector owns a child draft
current_module publishes the same child before parent restore

collector header view is used for one child
current_module.functions remains authority for another child
```

The I0 task explicitly forbids all three. The eight audited lowering-time
header readers also cannot be switched one by one if their source authority
alternates between the two stores.

## Why a partial cutover is rejected

The following changes are not admissible:

```text
replace only RawInvocationChildPortV1::complete_legacy_child
wrap only nested BoxDeclaration lowering
leave root main in current_module and children in collector
add collector -> current_module fallback lookup
clone collector headers into a temporary module cache
```

Each one either leaves a direct post-restore publication path, introduces a
second header truth, or makes source/evaluation order depend on which route
happens to be active.

## Candidates for the missing owner boundary

### Candidate A — invocation owns the complete module shell

```text
ModuleLoweringInvocationV1
  owns module shell + collector
  main and every child become unpublished drafts
  collector drains into one shell only at terminal commit
```

Requirements:

```text
main draft capture/commit is port-aware
lower_root receives one explicit raw port
all root child terminals use capture -> prepare -> seal -> collect
collector provides the only lowering-time header view
final module assembly consumes the collector exactly once
```

This preserves one authority, but requires an explicit decision about which
module metadata remains live during lowering and how the existing `main`
FunctionState is represented before collection.

### Candidate B — current_module becomes the collector backing store

```text
ModuleDraftCollectorV1
  borrows/owns the same module function map
```

Rejected for now: it exposes mutable `MirModule` through the collector, makes
collect-before-restore impossible to prove at the type boundary, and turns the
temporary header port into a view over a mutable publication object.

### Candidate C — collector prefix plus current_module read view

```text
header lookup = collector prefix, then current_module fallback
```

Rejected: this is a dual authority and violates the selected HeaderPort law.
It also makes duplicate and replacement semantics dependent on lookup order.

## Questions that must be answered before I0

1. Does `ModuleLoweringInvocationV1` own the module shell, or does a separate
   `ModuleDraftCollector` terminal consume an explicit shell product?
2. Which module metadata is immutable lowering input, and which fields are
   accumulated only after all function drafts are collected?
3. How is the root `main` represented while `lower_root` recursively opens
   children without exposing a second live module function map?
4. What exact operation drains collected drafts into the module shell, and is
   it infallible after preflight?
5. Which current-module readers are allowed to observe only immutable
   declaration/catalog authority rather than completed function headers?
6. Does the canonical A+/BindingSSA module transaction reuse this collector
   physically, or does it adapt into the same prepared admission product?

## Minimum implementation slice after consultation

Do not wire a child consumer yet. The next approved code-facing slice should
be one of:

```text
I0-SHELL-S0
  immutable module shell + collector drain vocabulary, consumers = 0

I0-MAIN-S0
  port-aware main draft product and explicit main admission, consumers = 0
```

The slice must include a fixture proving that a collected child header is
visible to a later child while `current_module.functions` remains empty for
the same identity. It must not add a fallback or retry route.

## Non-claims

```text
production invocation cutover
current_module/functions readers retired
collector -> module aggregation exists
FACTSESSION0 activation
PHI or finalization repair changes
JoinIR or Loop bridge widening
```

