# HEADERPORT0 re-entrant raw child terminal consultation

Status: DESIGN-STOP
Date: 2026-07-21
Scope: repair the terminal shape before `FINALIZE0-MODULEDRAFT0-HEADERPORT0-I0`
Related:

- `docs/development/current/main/investigations/mirbuilder-module-draft-headerport-i0-task-2026-07-21.md`
- `src/mir/builder/module_lowering_invocation.rs`
- `src/mir/builder/recursive_child_lowering.rs`
- `src/mir/builder/calls/function_session/terminal.rs`

## Trigger

The pre-I0 source audit invalidated one assumption of the selected terminal
API. `ModuleLoweringPortV1::complete_legacy_child` holds its mutable port
borrow while it executes the child-lowering closure. A raw child body can
contain another executable `BoxDeclaration`; that nested declaration needs the
same invocation port before the outer child is sealed and restored. Rust cannot
reborrow that port recursively while the outer completion method owns it.

The same audit found one concrete closure hole:

```text
raw_expression_dispatch ASTNode::BoxDeclaration
  constructor loop
    -> self.lower_method_as_function(...)
    -> legacy restore-then-bare-publication
```

That route bypasses the collector. Activating the current I0 plan would violate
both `one collector per invocation` and `collect before parent restore`.

## Preserved authority

```text
ModuleLoweringInvocationV1
  owns exactly one ModuleDraftCollectorV1

ModuleLoweringPortV1
  is the sole mutable collector capability

RawInvocationChildPortV1
  is only a stack reborrow of that same port

PendingFunctionSessionCloseV1
  owns one successful child draft plus the captured parent state

LoweringHeaderPortV1
  is a short signature-only borrow; it owns no draft, cache, or metadata
```

Non-authorities remain unchanged:

```text
MirBuilder / CompilationContext / TLS
current_module header cache
second collector
cloned draft or header store
JoinIR / Loop planner
fact-session lanes
```

## Candidate R-prime — split capture from collector commit

Recommended shape:

```text
RawInvocationChildPortV1::lower_*_box_method
  -> capture one PendingFunctionSessionCloseV1
       while the raw invocation port remains reborrowable by nested lowering
  -> child draft uses a scoped LoweringHeaderPortV1 only where it finalizes
  -> after child lowering returns, end every header borrow
  -> ModuleLoweringPortV1 prepares admission, seals, collects, then restores
```

The key change is not a second terminal. It is a two-phase use of the existing
pending terminal:

```text
capture child with Builder
  -> lower nested raw descendants through the same RawInvocationChildPortV1
  -> validate_before_restore
  -> hand the pending product back to ModuleLoweringPortV1
  -> prepare -> seal -> infallible collect -> restore
```

`ModuleLoweringPortV1` remains the only collector commit owner. The raw port
may create no admission and may not publish a draft. This restores recursive
use without putting a collector field in Builder or keeping a mutable collector
borrow across a header read.

## Rejected alternatives

```text
Pass &mut ModuleLoweringPortV1 into the existing complete_* closure
  Reentrant nested child completion aliases the same mutable port.

Store the port or collector in MirBuilder / CompilationContext / TLS
  Ambient authority and re-entrancy escape.

Let nested children use the legacy session facade
  Restores then bare-publishes; breaks the collector law.

Create a child-local collector and merge later
  Second draft truth and foreign-pairing risk.

Treat constructor lowering as an exception
  It is a physical function completion and must obey the same law.
```

## Required associated decision: header read injection

`finalize_function_draft` currently queries `current_module` for Call/Await
compatibility annotation. The re-entrant terminal must therefore pass a
signature-only lookup explicitly into the draft finalizer. The lookup supports
only:

```text
symbol -> FunctionSignature
symbol presence
```

It must not expose a function body, metadata, `MirModule`, or collector
mutation. Legacy test facades may retain a module-backed adapter until their
separate retirement, but no production HEADERPORT0 route may fall back from a
port query to `current_module.functions`.

## Decision requested

Select one of these before code resumes:

```text
R-prime (recommended)
  pending capture is separate from collector commit; raw recursion keeps one
  reborrowable invocation port and commit remains sole-owned by ModulePort.

R-alt
  replace the raw recursive lowering interface with a pure function plan
  before child capture. This expands scope into a dedicated pure-plan bridge.
```

## Acceptance after R-prime

```text
outer raw child containing nested static/instance BoxDeclaration
  -> both drafts collect before their respective parent restore

raw Box constructor plus ordinary instance method
  -> both use the same invocation collector

child finalizer
  -> sees only collector-owned prior signatures through an explicit header loan

primary / cleanup / admission / panic
  -> collector unchanged where required; parent restored exactly once

raw invocation port / legacy no-child path
  -> no new Builder, TLS, current_module cache, or JoinIR port consumer
```

## Non-claims

```text
No FACTSESSION0 activation.
No type-pipeline, Call/Await semantic, PHI, JoinIR, or MODULETX redesign.
No condition_fn retirement.
No canonical callable catalog replacement.
```
