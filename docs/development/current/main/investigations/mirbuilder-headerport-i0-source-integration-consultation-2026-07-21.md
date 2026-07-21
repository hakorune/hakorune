# HEADERPORT0-REENTRANT-TERM0-I0: source integration consultation

Status: **Candidate A-prime selected; production I0 remains disconnected**
Date: 2026-07-21
Parent: `mirbuilder-headerport-reentrant-terminal-task-2026-07-21.md`
Decision: one invocation-owned shell plus one collector; shell vocabulary is
the next disconnected code-facing slice

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

## Selected Candidate A-prime

Candidate A is narrowed to a shell/collector split that preserves one function
authority without making the collector a mutable `MirModule` view.

```text
ModuleLoweringInvocationV1
  owns ModuleLoweringShellV1
  owns ModuleDraftCollectorV1
  lends LoweringHeaderPortV1 from the collector only

ModuleLoweringShellV1
  owns module name, globals, and module metadata accumulation
  owns no published function map during lowering

ModuleDraftCollectorV1
  owns every completed function draft and header identity
  owns the only completed-function header view during lowering
```

The shell may use the existing `MirModule` metadata/global vocabulary, but its
`functions` map is structurally empty until the final drain. A shell function
map is never used as a header fallback. The collector is drained exactly once
after all drafts are sealed and all module-level preflight has passed.

The root `main` is represented as the live, unpublished
`FunctionLoweringState` until its terminal capture. It is not inserted into a
shell function map while children are still lowering. The terminal sequence is:

```text
prepare shell metadata
-> lower root with one explicit raw port
-> capture main pending session
-> prepare/seal/collect Main
-> synthesize and collect condition_fn when absent
-> preflight collector-to-shell drain
-> drain collector exactly once into shell
-> final module verification and external return
```

Lowering-time metadata reads/writes are split from function-header reads:

```text
module name/globals/static plans/declared metadata
  -> ModuleLoweringShellPortV1

completed function signature/presence/inventory
  -> LoweringHeaderPortV1 (collector only)
```

No consumer may read `current_module.functions` during an active invocation.
The existing eight-reader census must therefore be rechecked after the shell
port is introduced; a reader that needs a completed body or metadata is a
typed stop, not a shell fallback.

### Why this is the smallest coherent owner

```text
one invocation = one shell + one collector
one header authority = collector
one function publication point = collector drain
one main admission = Main key / symbol main / arity 0
one synthetic admission = SyntheticConditionFn / symbol condition_fn / arity 1
```

This keeps legacy replacement and canonical duplicate policy inside prepared
collector admission. It does not redesign TypePipeline, PHI repair, JoinIR,
FACTSESSION, or finalization; those remain after the shell/collector bridge.

## I0-SHELL-S0 closeout

The disconnected shell vocabulary is now present in
`src/mir/builder/module_lowering_shell.rs`:

```text
ModuleLoweringShellV1
  accepts only a function-empty MirModule

PreparedModuleLoweringShellDrainV1
  is non-Clone and single-use
  preflights duplicate function symbols before shell mutation
```

Its three focused tests prove rejection of an already-published function map,
one successful batch drain, and duplicate-symbol rejection. The module is
registered behind the reusable HeaderPort guard, and no production lowering
consumer calls `prepare_drain` yet. The shell is therefore a vocabulary and
transaction boundary, not a second live module store.

The next row is `HEADERPORT0-REENTRANT-TERM0-I0-SHELL-P0`: inventory the
module metadata/global writes and prove the collector-to-shell drain preflight
without connecting a production root or child.

## Questions that must be answered before implementation

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

Do not wire a child consumer yet. The next approved code-facing slice is:

```text
I0-SHELL-S0
  immutable module shell + collector drain vocabulary, consumers = 0
```

The slice must include a fixture proving that a collected child header is
visible to a later child while the shell function map remains empty for the
same identity. It must not add a fallback or retry route.

## Non-claims

```text
production invocation cutover
current_module/functions readers retired
collector -> module aggregation exists
FACTSESSION0 activation
PHI or finalization repair changes
JoinIR or Loop bridge widening
```
