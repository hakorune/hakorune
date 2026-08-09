# LOOP-RECIPE-V2-JOINSIG-DYNAMIC-D0

Status: accepted design; implementation 0
Date: 2026-08-10
Depends on: `LOOP-V2-DYNAMIC-LOCAL-SCOPE-R0` closed

## Decision

The unchanged Dynamic Loop uses the existing JoinSig algebra, but it must not
copy the V1 elaborator or erase V2 classes through a V2-to-V1 conversion.

The final authority is:

```text
VerifiedLoopRecipeV1 -- private borrowed V1 view --\
                                                  +-> one common JoinSig flow engine
VerifiedLoopRecipeV2 -- private borrowed V2 view --/      |
                                                          +-> V1 seal (compat)
                                                          +-> V2 seal (canonical V2)
```

The borrowed views expose exact Loop/Block/Item traversal, operation def/use,
carrier visibility, exits, and value classes. They are not stored, serialized,
or published as a second Recipe or Plan. Every operation arm is exhaustive;
unsupported vocabulary rejects rather than defaulting, retrying, or falling
back.

V1 and V2 may retain versioned wire products while both Recipe schemas are
live. They do not run in parallel for one Recipe: exactly one verified Recipe
enters the common engine and receives exactly one matching JoinSig. The V1
adapter is compatibility debt and is removed when V1 production consumers
reach zero.

## Why the current V1 implementation is insufficient

The V1 meaning is reusable, but its concrete types are not the V2 boundary:

1. `LoopJoinPayloadV1` stores `LoopValueClassV1` and cannot preserve
   `LoopValueClassV2::Dynamic`.
2. the V1 one-sided branch helper accepts Break/Continue only and explicitly
   rejects `Return`;
3. `LoopJoinBranchExitV1` requires a Loop target, so it cannot honestly encode
   a function exit;
4. the V1 operation walker does not know `DynamicAdd`, `DynamicLess`,
   `CallSlot`, or `TextEq` def/use rules.

The correction is a common private engine plus a V2 typed seal. It is not a
Dynamic-profile JoinSig owner, a copied V2 flow implementation, or a lossy V1
adapter.

## Exact unchanged-Recipe golden

### Loop row

The deterministic edge order and carrier payloads are:

| # | From | To | Role | Payload |
|---:|---|---|---|---|
| 1 | Preheader | Header | Enter | `B0=V1:Dynamic` |
| 2 | Header | Body | PredicateTrue | `B0=V1:Dynamic` |
| 3 | Header | After | PredicateFalse | `B0=V1:Dynamic` |
| 4 | Body | FunctionExit | Return | `B0=V1:Dynamic` |
| 5 | Body | Header | Backedge | `B0=V17:Dynamic` |

The final Loop carrier snapshot is:

```text
L0:
  B0 = V17 : Dynamic
```

The only port bindings are:

```text
Header: B0 : Dynamic
After:  B0 : Dynamic
```

`FunctionExit` is terminal and does not issue a Loop port binding.

### Inner If row

```text
owner_loop = L0
if_item    = I10
condition  = V13

then:
  Exit
  exit_item = I12
  role      = Return
  target    = FunctionExit
  payload   = [B0=V1:Dynamic]

else:
  Fallthrough
  payload = [B0=V1:Dynamic]
```

The V2 branch-exit target is typed:

```text
LoopJoinBranchExitTargetV2
  = Loop(LoopNodeKeyV1)
  | FunctionExit
```

Break/Continue require `Loop(target)`. Return requires `FunctionExit`. A fake
`target_loop=L0` for Return is forbidden.

### Explicit exclusions

`V10/ch` is an iteration-local operation value, not a Recipe binding or
carrier. Therefore it is absent from every edge payload, port binding, Header,
After, backedge, and future PHI. The JoinSig visible-payload owner derives rows
only from Recipe carriers.

The inner Return operand remains owned by the Recipe relation
`I12 -> E0 -> Return(Some(V14))`. `V14` is not duplicated into the carrier
payload.

The outer source Return is not a Recipe Exit or JoinSig Return:

```text
PredicateFalse -> After(B0:Dynamic)
                -> Callable Tail reads B0
                -> retained Completion outer-return site
```

## Owner table

| Meaning | Sole owner | Non-owners |
|---|---|---|
| Loop/Block/Item/If/Exit structure | verified Recipe V2 | JoinSig, Layout, CFG |
| value availability, current binding, logical transfer | common JoinSig engine | profile, Layout, CFG |
| Dynamic value class | verified Recipe V2; JoinSig carries exact projection | runtime tag, profile |
| inner Return operand | Recipe Exit `E0` | JoinSig payload, Completion |
| inner Return transfer | JoinSig `FunctionExit` | Recipe layout, physical layout |
| outer Return | Callable Tail + existing Completion | Loop Recipe, JoinSig |
| visible carrier payload | JoinSig from Recipe carriers | local-value relation, Home |
| V10/ch lexical relation | existing source/Recipe envelope local view | JoinSig carrier, PHI |
| Dynamic Fault | Dynamic invocation outcome and future exit transaction | Recipe, JoinSig, Completion |
| After capability | JoinSig derived from the exact Recipe | caller, profile |
| continuation transport | atomic semantic-program co-seal | caller-supplied wrapper |
| segment placement | physical Layout | Recipe/JoinSig meaning |
| physical edge/terminator | canonical CFG session | JoinSig/Layout |

## Fault and cleanup boundary

JoinSig describes normal lexical transfers only. A faultable Dynamic operation
has no implicit false, Void, Result, Recipe Exit, Return, or JoinSig edge.

```text
I6 Fault:
  no V10 publication
  no ch install/cleanup

I6 Normal, later Fault:
  V10 exists
  future general Dynamic capability/Home authority may install a Home
  only if the carrier is proved owner-bearing
  future exit transaction preserves the primary Fault and conditionally cleans

I12 inner Return:
  JoinSig authorizes FunctionExit
  future exit transaction conditionally cleans installed locals
  Completion consumes the exact inner source Return

normal backedge:
  future exit transaction leaves the iteration scope and conditionally cleans
  then the JoinSig-authorized Backedge proceeds
```

`Dynamic` and runtime tags never prove that a Home exists. `ch` is not assumed
to be the only owner-bearing Dynamic temporary. Home classification and exit
transactions remain separate named Decisions after semantic-program co-seal.

## Atomic semantic-program boundary

The V2 caller never supplies JoinSig, owner, root Loop key, After, or
Continuation. The later issuer consumes the exact existing
`VerifiedDynamicFullLoopSourceRecipeEnvelopeV2` and internally performs:

```text
exact verified Recipe V2
  -> common JoinSig elaboration
  -> require After(L0, B0, Dynamic) from that JoinSig
  -> co-seal source/Recipe/JoinSig/After
  -> VerifiedLoopSemanticProgramV2
```

The product exposes borrow-scoped views and is consumed whole. The two-site
Completion remains a sibling with the exact partition:

```text
inner source Return -> Recipe Return transfer
outer source Return -> Callable Tail after Loop After
```

Existing external `Continuation::from_after(owner, after)` and split physical
demand issuance are compatibility debt. They are not copied into the V2 path.

## Typed failure matrix

The bounded implementation rejects:

```text
view/engine:
  unsupported or unmapped operation
  missing result definition / use before definition
  unavailable binding / write without carrier closure
  unreachable item / branch state mismatch

target:
  Return with Loop target
  Break/Continue with FunctionExit target
  missing or wrong Return item
  unavailable Return operand

payload:
  missing / duplicate / extra binding
  wrong value or class
  Header/After binding-set mismatch
  wrong backedge value (not V17)
  any V10/ch payload or port binding
  V14 duplicated as carrier payload

boundary:
  outer Return represented as Recipe Exit or JoinSig edge
  Dynamic Fault represented as value, Exit, Return, false, or JoinSig edge
  independently supplied JoinSig, owner, After, or Continuation
  JoinSig derived from a foreign Recipe
```

## Ordered implementation ladder

1. `LOOP-JOINSIG-NEUTRAL-ENGINE-R0`
   - BoxShape-only behavior-preserving extraction;
   - one private borrowed view/engine;
   - V1 output and all V1 normalized tests remain identical;
   - no V2 connection and no newly accepted branch family.
2. `LOOP-RECIPE-V2-JOINSIG-DYNAMIC-I0`
   - V2 borrowed view and typed V2 seal;
   - exhaustive V2 operation def/use projection;
   - one-arm Return-to-FunctionExit support;
   - exact five-edge/one-branch golden and negative matrix;
   - no source co-seal, Continuation, Fault, Home, or physical effect.
3. `LOOP-V2-SEMANTIC-PROGRAM-COSEAL-I0`
   - consume the exact existing Dynamic source/Recipe/envelope product;
   - derive JoinSig and After internally;
   - preserve the inner/outer Completion partition;
   - expose no split/re-pair constructor.
4. `DYNAMIC-FAULT-EXIT-TRANSACTION-D0/I0`
   - enumerate every faultable cut point and normal/Return/backedge exit;
   - protect the primary outcome and define cleanup-failure precedence;
   - do not infer Home from Dynamic.
5. general Dynamic capability/Home classification and lexical cleanup;
6. JoinSig-authorized physical transfer binding and common physicalization.

## Removal conditions

The V1 adapter and compatibility constructors are removed when all are true:

```text
V2 is the canonical production Recipe wire
V1 production JoinSig consumers = 0
all JoinSig callers use the common borrowed engine
external continuation from_after callers = 0
three-argument split physical-demand issuers = 0
physical Layout direct transfer inference = 0
```

The private common engine remains durable. Borrowed version adapters remain
only while their corresponding verified Recipe wire has live consumers.

## Hard stops

```text
no copied V2 flow implementation
no Dynamic-profile JoinSig owner
no V2-to-V1 class erasure
no Builder / MIR / CFG / PHI / physical Layout
no Home install/cleanup or runtime-tag lifetime inference
no Fault-as-Recipe-value/Exit or JoinSig edge
no Tail/Completion/return ABI absorption
no caller-supplied JoinSig/After/Continuation
no retry/fallback/provider execution
no source rewrite or fixture narrowing
```

## D0 acceptance

Decision: **accepted**. One common engine owns every normal logical transfer;
the V2 seal preserves Dynamic and FunctionExit exactly; Fault and outer Tail
remain external siblings; and the next executable row is the behavior-neutral
engine extraction, not V2 activation.
