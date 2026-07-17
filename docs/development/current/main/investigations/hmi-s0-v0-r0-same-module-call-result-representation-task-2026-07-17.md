---
Status: closed through M0; B-prime selected in child card
Date: 2026-07-17
Decision: inventory-first; production representation owner remains a design stop
Baseline: 446e8782e6
Parent: hmi-s0-v0-r0-generic-loop-carrier-type-task-2026-07-17.md
Scope: identify the lowering-time representation owner for a forward same-module call result
---

# HMI R0 same-module call-result representation task

## Current progress

The design stop is now resolved by:

```text
docs/development/current/main/investigations/
  callable-result-i64-catalog0-task-2026-07-17.md
```

Three worker audits selected a narrow conditional exact-i64 result catalog.
The next code-facing row is `R0-CALLABLE-RESULT-I64-CATALOG0-L0a`; this parent
card remains the evidence source for the original timing failure.

`R0-SAME-MODULE-CALL-RESULT-REP0-M0` is closed with the exclusive
classification:

```text
CALLEE-REPRESENTATION-AUTHORITY-ABSENT
```

The checked-in machine inventory and source-derived drift checker are:

```text
tools/checks/fixtures/
  same_module_call_result_representation_m0_inventory_v1.json

tools/checks/lib/
  same_module_call_result_representation_inventory.py
```

The declaration index publishes the complete static method name/arity and a
body-bearing `LoweredMethodAst`, but that product contains only `params`,
`param_decls`, and `body`; it owns no generic return-representation fact. The
caller allocates a call-result `ValueId`, emits the call, and consults only an
already-published current-module signature or narrow known-name heuristics.
Function-local finalization repeats the same current-module lookup and records
`Unknown` when the forward callee is still absent.

The exact Integer fact later observed in final MIR is published only after the
complete module exists, through:

```text
refresh_module_semantic_metadata
  -> refresh_module_route_fixpoint
  -> refresh_module_global_call_routes
  -> publish_global_call_route_result_value_types
  -> MirFunction.metadata.value_types
```

That is a finalized-module metadata publication surface, not a lowering-time
authority. Consequently M0 does not authorize a local producer I0. The next
work is one design consultation selecting a complete, declaration-order-
independent callable result-representation authority.

`R0-SAME-MODULE-CALL-RESULT-REP0-S0` is closed with production behavior and
production type publishers both zero. One HMI-independent proof app owns five
small source cases and one direct checker:

```text
forward untyped direct call result:
  Missing

forward untyped call result through one local Copy:
  Missing

forward explicitly i64-annotated callee:
  Missing

the same untyped provider/caller with provider declared first:
  Exact; generic_loop_v1 returns 6

independent literal-i64 control:
  Exact; generic_loop_v1 returns 6
```

Debug and release produce the same normalized observation. The reverse-order
MIR diagnostic has exact i64 on the call result, its Copy, and the carrier PHI,
but the checker labels finalized metadata diagnostic-only. The typed-forward
case proves that a source return annotation does not by itself make a callee
fact available before that callee is lowered. No source rewrite, declaration
reordering, name heuristic, GenericLoop default, final-metadata fallback, or
new type publication is present.

Validation:

```text
bash apps/same-module-call-result-representation-proof/test.sh
python3 -m py_compile \
  tools/checks/lib/same_module_call_result_representation_proof.py
bash tools/checks/generic_loop_progression_role_v0_guard.sh
cargo check -q
tools/checks/dev_gate.sh quick
```

The quick gate exposed one older Add-result unit-test boundary leak: two test
values were written by direct `variable_map.insert`. The test now uses the
existing `publish_emission_cache` owner instead; the no-growth inventory is 47
and quick is green at 66/66. This changes no production behavior.

## Selected next slice

Three read-only worker audits agree that the current GenericLoop failure must
not be repaired inside GenericLoop. The locally authorized work is one generic,
behavior-neutral source fixture followed by one machine timing inventory:

```text
R0-SAME-MODULE-CALL-RESULT-REP0-S0
  -> R0-SAME-MODULE-CALL-RESULT-REP0-M0
  -> DESIGN-STOP
  -> selected producer S0/P0/I0/G0
  -> R0-GENERICLOOP-CARRIER-TYPE0-G0
```

The completed code-facing row was:

```text
R0-SAME-MODULE-CALL-RESULT-REP0-M0
```

M0 found no existing generic pre-consumer producer seam. It therefore does not
authorize a new interprocedural result-representation authority. Production
I0 remains forbidden until a new design decision selects that authority.

## Current evidence

The exact failure is stable in debug and release:

```text
function:
  ParserBox.static_const_parse_add/2

selected GenericLoop init:
  ValueId(28)

failure:
  MissingTransientType { init: ValueId(28) }
```

The source carrier is `pos`:

```text
lang/src/compiler/parser/parser_box.hako:617
  pos = ParserStringUtilsBox.skip_ws(
      text,
      me.static_const_eval_pos(ret),
  )

lang/src/compiler/parser/parser_box.hako:618
  loop(...)
```

The saved final MIR normalizes this as:

```text
%28 = call_global ParserStringUtilsBox.skip_ws/2(...)
%31 = phi [%28, ...]
```

`%28` is exact Integer in final MIR, but its lowering-time
`builder.type_ctx.value_types` entry is missing when the GenericLoop skeleton
asks for the selected init representation.

The current call annotator publishes a result fact when the target function is
already present in the current module, or when a narrow builtin rule owns an
exact result. `ParserStringUtilsBox.skip_ws/2` is lowered after the caller, has
no source return annotation, and is not a builtin result rule. Its final
Integer fact appears only after later whole-module route/signature work. That
late fact is diagnostic evidence, not a lowering-time authority.

## Existing authorities that remain unchanged

```text
GenericLoop carrier role:
  GenericLoopCarrierRoleV1

selected init identity:
  variable_map[loop_var]

current lowering-time representation truth:
  builder.type_ctx.value_types[init]

representation verifier:
  prepare_generic_loop_carrier_representation_v1

production verifier consumer:
  GenericLoop skeleton exactly one

skeleton callers:
  normalizer one + composers two
```

`MissingTransientType` and `UnknownTransientType` remain intentional
fail-fast boundaries. GenericLoop facts continue to own role only and contain
zero `MirType` policy.

## Non-authorities

The following may not supply the missing representation:

```text
NumericProgression or BodyManagedState role
loop condition or update syntax
loop variable, function, callee, field, method, or HMI name
runtime tags
final MirFunction metadata during lowering
whole-module route fixpoint run during lowering
compile/declaration order
value_origin_newbox backfill
a second persistent ValueId type map
source return annotation added only as a workaround
fallback, retry, or legacy probing
```

## R0-SAME-MODULE-CALL-RESULT-REP0-S0

`production behavior delta = 0`.

Add one HMI-independent proof app and a direct checker. The fixture must keep
the callee after the caller so that declaration-order dependence is visible.
It must include at least:

```text
forward untyped static/global callee returning one stable scalar shape
same source with declaration order reversed
already-published current-module signature control
ordinary local Copy of the call result
consumer that requires the result before final module refresh
rejected compile followed by an independent valid compile
```

Suggested physical shape:

```text
apps/same-module-call-result-representation-proof/
  README.md
  main.hako
  helper.hako
  test.sh

tools/checks/lib/same_module_call_result_representation_proof.py
```

The checker records facts; it must not make either declaration order pass by
rewriting source, changing compile order, or adding a result annotation.

S0 owns only:

```text
generic reproduction source
stable debug/release observation vocabulary
current-module hit/miss observation
lowering-time Missing | Unknown | Exact classification
```

S0 owns no call-result publication, GenericLoop change, HMI source change,
runtime change, backend change, or result-representation policy.

## R0-SAME-MODULE-CALL-RESULT-REP0-M0

`production behavior delta = 0`.

Create one checked-in machine inventory that fixes:

```text
caller and callee canonical identities
source declaration order
call-result ValueId and definition kind
callee presence in current module at call lowering
callee declaration/body availability
source return annotation presence
lowering-time call annotation decision
type_ctx publication site and time
LocalSSA Copy/Phi publication chain
final route/signature result as diagnostic-only evidence
final MirFunction metadata difference
debug/release parity
```

M0 must classify the first missing publication with exactly one token:

```text
CANONICAL-PRODUCER-PUBLICATION-REQUIRED
CALLEE-REPRESENTATION-AUTHORITY-ABSENT
LOCAL-COPY-PUBLICATION-MISSING
GENERICLOOP-INIT-SELECTION-MISMATCH
MULTIPLE-PRODUCER-SEAMS
```

Only `CANONICAL-PRODUCER-PUBLICATION-REQUIRED` authorizes a locally selected
producer implementation row. Every other result returns to `DESIGN-STOP`.

## Candidate slices after M0

### A. Canonical producer publication

Conditionally recommended. The call-result producer may publish an exact
transient representation only if M0 proves that one existing, generic,
order-independent authority already owns that fact before its consumer.

This candidate may not be named or connected before M0 identifies the exact
producer and fact source.

### B. Complete callable result-representation authority

Possible but not pre-authorized. If the callee result has no existing exact
pre-lowering authority, sealing forward same-module results requires a new
interprocedural design decision. This returns to consultation rather than
being hidden inside a call annotator patch.

### C. GenericLoop role/default or use-site inference

Rejected. Supplying Integer from a loop role breaks the closed TYPE0 law.
Re-deriving the call result from the loop condition, update, callee body, or
callee name creates a second type inference engine.

## I0 authorization law

After M0, a producer I0 may be taskized locally only when all are true:

```text
classification = CANONICAL-PRODUCER-PUBLICATION-REQUIRED
generic producer seams = exactly one
exact representation authority already exists before the consumer
source/function/callee/HMI name conditions = 0
declaration-order dependence = 0
GenericLoop fallback/default changes = 0
final metadata reads during lowering = 0
new persistent type/owner maps = 0
```

The future producer row must be named from the identified owner, not from
GenericLoop or HMI.

## Active DESIGN-STOP consultation packet

The consultation must select one source/body result-representation authority
that is complete before any caller body consumes a forward same-module call.
It must answer, in one decision:

```text
1. Which product owns callable result representation before caller lowering?
2. Is the fact source-only, body-derived, or a co-seal of both?
3. How are unannotated functions such as skip_ws/2 handled without declaration
   order, name heuristics, or finalized metadata?
4. What exact first grammar is admitted, and what remains Unknown/rejected?
5. How is conflict between declared and body-derived representation rejected?
6. Which pre-Builder phase seals the complete catalog/result correspondence?
7. Which single lowering-time consumer publishes the call-result ValueId type?
8. Which existing late metadata publishers remain diagnostic-only?
```

The selected architecture must preserve:

```text
complete immutable same-module callable catalog before body lowering
declaration reorder parity
one result-representation truth per callable key
no GenericLoop/type-role inference
no callee/function/HMI name rules
no finalized metadata reads during lowering
no source annotation added solely as a workaround
no compile retry, fallback, or callee-first publication dependency
```

The first post-consultation task order must be named from the selected owner:

```text
<CALLABLE-RESULT-OWNER>-S0
  disconnected product and conflict vocabulary; production consumers 0

<CALLABLE-RESULT-OWNER>-P0
  forward/backward, typed/untyped, conflict, and declaration-reorder proof

<CALLABLE-RESULT-OWNER>-I0
  exactly one pre-body producer and one call-result publication consumer

<CALLABLE-RESULT-OWNER>-G0
  producer/consumer counts, old order-dependent path classification, guards

then:
  R0-GENERICLOOP-CARRIER-TYPE0-G0
```

Until that consultation closes, code changes to declaration indexing, call
annotation, final semantic refresh, or GenericLoop are forbidden.

## Required counters

```text
GenericLoop role owners = 1
GenericLoop representation decisions = 1
GenericLoop skeleton consumers = 1
GenericLoop skeleton callers = 3

GenericLoop missing/Unknown defaults = 0
GenericLoop definition-based type inference = 0
final metadata lowering-time reads = 0
mid-lowering route-fixpoint runs = 0
compile-order fixes = 0
name/runtime heuristics = 0
new persistent ValueId type maps = 0

S0/M0 production type publishers = 0
S0/M0 production behavior delta = 0
HMI source delta = 0
stash apply/pop/restore = 0
fallback/retry = 0
source/check files >= 800 lines = 0
```

## Stop conditions

Stop before producer implementation if any applies:

1. More than one producer or timing seam is required.
2. The exact result exists only in finalized metadata.
3. A new source ABI or complete-module return fixed point is required.
4. GenericLoop role must supply Integer or weaken missing/Unknown rejection.
5. GenericLoop must inspect call definitions, conditions, or updates.
6. Declaration order or callee-first compilation becomes semantic authority.
7. A function/callee/HMI-name exception is required.
8. A source annotation is required only to make this fixture pass.
9. A second type map, runtime tag, fallback, or retry is required.
10. PHI, runtime, backend, ownership, or HMI handler work must change in the
    same row.
11. A source/check file reaches 800 lines.

## Claims and non-claims

S0/M0 may claim only:

```text
one generic forward same-module call-result timing reproduction
one machine classification of the first missing lowering-time fact
debug/release observation parity
an exact next producer token or an explicit design stop
```

They must not claim:

```text
forward-call result typing support
complete callable result representation
general return inference
GenericLoop widening
declaration-order independence
HMI completion
runtime/backend/ownership widening
fallback or recovery
```

## Decision lock

Three worker audits select an inventory-first boundary. The current failure is
not a GenericLoop role or PHI defect: its selected init is a forward,
same-module, untyped static/global call result whose exact Integer
representation is published only after the lowering-time consumer has already
failed. `R0-SAME-MODULE-CALL-RESULT-REP0-S0` has now closed the generic
reproduction and typed observations with zero production delta. M0 is also
closed and returns `CALLEE-REPRESENTATION-AUTHORITY-ABSENT`: declaration
indexing owns no generic result representation, caller-local publication is
current-module/order dependent, and the existing exact result publisher runs
only on finalized whole-module metadata. GenericLoop keeps its exact current-
type authority and Missing/Unknown stop law. No production repair is locally
authorized. The lane is now stopped at one consultation for a complete,
declaration-order-independent callable result-representation authority.
