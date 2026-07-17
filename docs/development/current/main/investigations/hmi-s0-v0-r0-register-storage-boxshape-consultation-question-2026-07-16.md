---
Status: Resolved; superseded by MapFieldOwner proof task
Date: 2026-07-16
Decision needed: select the generic compiler proof that precedes HMI-S0-V0-R0
Baseline: 188edc601c
Parent: hmi-s0-v0-disconnected-scalar-state-task-2026-07-16.md
Scope: read-only evidence and generic compiler prerequisite selection
---

# HMI-S0-V0-R0 register storage BoxShape consultation

## Question

Which exact generic compiler BoxShape must be proved and, if necessary,
repaired before the typed HMI scalar register file resumes?

Recommended answer:

```text
select R0-STOP0 first

one generic seven-case proof matrix
no HMI imports
no production behavior change

then select exactly one:
  R0-COMPILER-KEY0
  or
  R0-COMPILER-RECV0

only the first failing structural delta may become the compiler slice
```

Do not select an HMI-side storage workaround.

## Closed prerequisite

`188edc601c` closes HMI-S0-V0-L0:

```text
typed state errors
tagged i64/i1 values
explicit outcomes
explicit predecessors
harness-only step budget
production callers / handlers / fallback = 0
```

The R0 prototype is not in the worktree.

```text
stash:
  wip/hmi-s0-v0-r0 register storage fails field mutation
```

It is evidence only and must not be restored as authority.

## Confirmed evidence

### Green controls

```text
local MapBox:
  set(dynamic local key) -> has/get
  green

minimal user box:
  birth owns one MapBox field
  put(literal String key)
  caller has/get
  green

direct scalar value in local MapBox:
  set/get tagged HmiScalarValueV1
  green
```

Therefore these broad claims are rejected:

```text
MapBox mutation is generally broken
user-box field MapBox is generally copied deeply
receiver copy always destroys field identity
tagged scalar boxes cannot be stored in MapBox
```

### Failing HMI-derived shape

The typed register prototype:

```text
method formal value_id
  -> "" + value_id
  -> field MapBox set
  -> same register owner has/read
```

reported:

```text
set emitted
immediate observation false
```

Additional independent compiler seams were found and removed from the
diagnosis:

```text
generic set_success/set_failure names:
  resolved to another result box

optional scalar accessor:
  null branch forced a Void return representation
```

Those are not sufficient to explain the remaining storage failure.

## Remaining root candidates

### Candidate KEY0 — formal-derived dynamic String key

Evidence:

```text
green literal key:
  "2"

green local typed key:
  local id = 2
  "" + id

failing candidate:
  untyped method formal id
  "" + id
```

Possible missing compiler fact:

```text
method formal
  -> String concatenation
  -> exact String key representation
```

### Candidate RECV0 — receiver alias after control merge

Evidence:

```text
minimal straight-line owner:
  green

register define:
  validation If branches
  receiver and arguments cross several PHIs
  mutation and later observation use different receiver SSA aliases
```

Possible missing compiler fact:

```text
same user-box receiver identity
  preserved across fallthrough control merge
  for field-held mutable Box observation
```

### Candidate WORKAROUND — reject

Rejected alternatives:

```text
public/raw MapBox state carrier
caller-side ValueId stringification
dense or parallel ArrayBox register storage
HMI by-name dispatch
RuntimeDataBox special case
fallback/retry
MapBox runtime semantic change
```

They hide the compiler expressivity gap and weaken the selected typed state
proof.

## R0-STOP0 exact proof matrix

Create one generic fixture with no HMI names or imports.

```hako
box MapFieldOwner {
    storage

    birth() {
        me.storage = new MapBox()
    }
}
```

Cases:

```text
1. local_map
   local MapBox
   local integer id -> dynamic String key
   set / has / get

2. field_literal
   owner field MapBox
   literal "2"
   put method -> caller has/get

3. field_formal_concat
   put_id(id) {
       me.storage.set("" + id, 10)
   }
   caller has_id(id)

4. field_formal_key
   caller constructs local key = "" + id
   put_key(key)
   caller has_key(key)

5. same_method_direct
   set("" + id)
   direct field.has("" + id)

6. same_method_self
   set("" + id)
   me.contains_id(id)

7. control_merge
   one and two fallthrough/early-return validation Ifs
   then cases 5 and 6

8. receiver_alias
   mutate through a local receiver alias
   observe through original receiver

9. instance_isolation
   two owners
   mutation never crosses instances
```

Cases 8 and 9 are invariants around the first seven cases, not separate
accepted language features.

## Required MIR evidence

For every case record:

```text
key producer MirType:
  String | Unknown

receiver route:
  MapBox | RuntimeDataBox

certainty:
  Known | Union

set receiver ValueId root
has/get receiver ValueId root
receiver PHI inputs
field.get / field.set count
callee argument order

CopyOwned:
  0

DestroyOwned:
  0

ReleaseStrong selected-route additions:
  0
```

Do not use runtime success alone to name the compiler root.

## Selection law

```text
case 3 fails
case 4 passes
  -> KEY0

cases 3 and 4 fail
literal cases pass
  -> method-formal key provenance boundary
     refine before implementation

cases 3-6 pass
case 7 fails
  -> RECV0

direct field.has fails
self contains passes
  -> method-name/receiver resolution boundary

case 2 fails
  -> evidence contradicts the current minimal control
     stop and re-audit field identity

all cases pass
  -> HMI RegisterFile has another first structural delta
     add exactly one minimized delta; do not widen generically
```

## Recommended compiler implementation law

After the matrix selects exactly one failing shape:

```text
one blocker
one accepted BoxShape
one fixture
one reusable proof/gate
one commit
```

The compiler slice must:

```text
live outside HMI
use generic MapFieldOwner naming
change no HMI schema/state/opcode
change no MapBox runtime semantics
add no by-name logic
add no fallback
activate no ownership operation
keep every source/check file < 800 lines
```

If the selected fix needs any of the following, stop again:

```text
general receiver ABI / P0r
Ownership V2
SSA-I1-O1 BoxRef owner
MapBox runtime behavior change
new backend capability
source grammar change
```

## Proof entry

Prefer:

```text
one generic proof app
one manifest-backed existing proof runner entry
```

Reuse neighboring numeric-field mutation evidence only as a regression. It
does not prove Box-valued field identity.

Required checks:

```text
release VM
debug VM
normalized MIR assertions
existing nearby field-mutation guard
current pointer
diff check
quick 66/66
source/check < 800
```

Do not extend the HMI authority guard to prove compiler field semantics.

## HMI resume law

HMI-S0-V0-R0 resumes only after:

```text
the matrix selects exactly one root
the generic compiler proof is green
the fix is landed and pushed
the original typed RegisterFile design is rebuilt cleanly
raw payload/static-facade WIP pieces are not restored
production callers remain zero
```

The selected HMI result remains:

```text
private MapBox behind typed RegisterFile API
tagged i64/i1 values
single publication
undefined-read failure
immutable block-entry snapshot
no raw storage exposure
```

## Explicit non-claims

```text
general MapBox typing
general receiver identity theorem
general borrow/ownership semantics
BoxRef storage
thread safety
LLVM/AOT parity
HMI opcode execution
product VM replacement
```

## Final question

> Adopt `R0-STOP0` as the sole next code-facing slice: a generic,
> HMI-independent MapFieldOwner proof matrix that distinguishes
> formal-derived dynamic String key provenance from receiver/field mutation
> visibility after control-flow merge. Only the first failing structural delta
> may select `R0-COMPILER-KEY0` or `R0-COMPILER-RECV0`. HMI-side raw MapBox,
> caller stringification, ArrayBox storage, runtime special cases, fallback,
> and ownership/backend widening remain forbidden. Is this selection accepted?
