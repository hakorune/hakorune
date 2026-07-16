---
Status: Ready for implementation
Date: 2026-07-16
Decision: accepted
Baseline: 2bf7f7c8af30
Parent: hmi-s0-v0-disconnected-scalar-state-task-2026-07-16.md
Supersedes: hmi-s0-v0-r0-register-storage-boxshape-consultation-question-2026-07-16.md
Scope: proof-manifest hygiene, generic MapBox field-owner proof, and one compiler BoxShape selection
---

# HMI-S0-V0-R0 MapFieldOwner proof task

## Outcome

The register-storage stop is taskized as:

```text
PROOF-MANIFEST-HYGIENE0
  -> R0-STOP0-S0
  -> R0-STOP0-M0
  -> R0-STOP0-V0
  -> R0-STOP0-G0
  -> exactly one selected compiler task:
       R0-COMPILER-KEY0
       or
       R0-COMPILER-RECV0
  -> clean HMI-S0-V0-R0 reimplementation
```

The next code-facing row is `PROOF-MANIFEST-HYGIENE0`.

This prerequisite is behavior-neutral and must land separately from the
generic compiler proof. The proof-manifest runner is currently unusable:

```text
run_proof_app.sh --list:
  duplicate MIMAP-085A

proof_app_manifest_test_entry_guard.sh:
  M216 and M191 use direct legacy guards
  M217 and M218 test entries are not executable
  MIMAP-019A has no test entry
```

Do not register a new proof in a broken manifest and do not mix allocator
manifest repair with the later compiler semantic fix.

## Decision lock

`R0-STOP0` is a generic compiler diagnostic proof.

```text
HMI imports:
  0

HMI names:
  0

HMI source changes:
  0

compiler/runtime behavior changes:
  0

MapBox runtime changes:
  0

grammar/backend/ownership widening:
  0
```

Every source case has the same language-level expectation:

```text
set value
observe the same value through the same logical MapBox owner
result = pass
```

The proof succeeds when green controls remain green and the first structural
failure is classified by one exclusive token. It does not require all cases
to be green before the compiler prerequisite is implemented.

## Worker-audited source shape

Use one import-free fixture:

```text
apps/map-field-owner-boxshape-proof/
  README.md
  main.hako
  test.sh
```

The source contains only:

```hako
box MapFieldOwnerProbeV1 {
    storage

    birth() {
        me.storage = new MapBox()
    }
}

static box Main {
    main(args) {
        // matrix
    }
}
```

Use repository-global owner-specific wrapper names. Avoid generic method names
such as `initialize`, `read`, `get`, `set`, `has`, `contains`, `snapshot`,
`size`, `set_success`, and `set_failure`.

Required wrapper vocabulary:

```text
map_field_probe_v1_put_literal
map_field_probe_v1_has_literal
map_field_probe_v1_load_literal_present

map_field_probe_v1_put_id
map_field_probe_v1_has_id
map_field_probe_v1_load_id_present

map_field_probe_v1_put_key
map_field_probe_v1_has_key
map_field_probe_v1_load_key_present

map_field_probe_v1_put_then_has_direct
map_field_probe_v1_contains_id_internal
map_field_probe_v1_put_then_has_self
map_field_probe_v1_put_after_fallthrough
map_field_probe_v1_alias_put
```

`birth` is the only generic spelling permitted because it is the language
constructor hook.

The fixture must not expose or return the raw storage MapBox. Each independent
case uses a fresh owner unless the case explicitly proves aliasing or instance
isolation. Case results remain scalar locals; the proof must not create a
second MapBox or ArrayBox result ledger.

## Exact matrix

### C1 — local_map

```text
local MapBox
local integer id
key = "" + id
set / has / get
expected = pass
```

This is the general dynamic-key control.

### C2 — field_literal

```text
owner field MapBox
literal key "2"
put method
separate has/get methods
expected = pass
```

This is the field identity control.

### C3 — field_formal_concat

```text
method formal id
key = "" + id inside put/has/load methods
expected = pass
```

This isolates formal-derived String provenance.

### C4 — field_formal_key

```text
caller local id
caller local key = "" + id
key passed as method formal
expected = pass
```

This separates caller-side construction from callee-side concatenation.

### C5 — same_method_direct

```text
same method:
  field MapBox set
  direct field MapBox has/get
expected = pass
```

### C6 — same_method_self

```text
same method:
  field MapBox set
  me.map_field_probe_v1_contains_id_internal(id)
expected = pass
```

C5/C6 asymmetry is a method-boundary stop, not automatically RECV0.

### C7a/C7b — control_merge

```text
C7a:
  one fallthrough If
  selected key after merge

C7b:
  two fallthrough Ifs
  selected key after both merges

expected:
  selected key present with exact value
  unselected key absent
```

Do not use early Return inside the If arms if that would leave the currently
accepted grammar. The purpose is receiver/key continuity across the existing
fallthrough control shape.

### C8 — receiver_alias

```text
local alias = owner
mutate through alias
observe through original owner
expected = pass
```

### C9 — instance_isolation

```text
owner A and owner B
same literal key
A mutation visible only in A
B remains absent or independently valued
expected = pass
```

Additional negative assertions:

```text
fresh owner has=false
validation rejection publishes nothing
fallthrough unselected key is absent
alias mutation never reaches a second instance
missing key is checked with has before get
```

## Stable observation output

The fixture prints only stable machine-readable lines:

```text
map-field-owner-boxshape-proof
case.local_map=0|1
case.field_literal=0|1
case.field_formal_concat=0|1
case.field_formal_key=0|1
case.same_method_direct=0|1
case.same_method_self=0|1
case.control_merge_one=0|1
case.control_merge_two=0|1
case.receiver_alias=0|1
case.instance_isolation=0|1
selection=<token>
summary=observed
```

Do not use English diagnostic prose, MapBox receipt strings, `stringify()`, or
allocation identity as classification authority.

## Exclusive selection law

Evaluate in this exact order:

```text
base =
  C1 && C2 && C9

if !base:
  STOP-REAUDIT-FIELD0

else if !C3 && C4:
  KEY0

else if !C3 && !C4:
  STOP-KEY-BOUNDARY0

else if C3 && !C4:
  STOP-CALLER-KEY0

else if C5 != C6:
  STOP-METHOD-BOUNDARY0

else if C3 && C4 && C5 && C6 && (!C7a || !C7b || !C8):
  RECV0

else if C1 && C2 && C3 && C4 && C5 && C6 && C7a && C7b && C8 && C9:
  NONE-HMI-DELTA0

else:
  STOP-UNCLASSIFIED0
```

The classifier must produce exactly one token.

Interpretation:

```text
KEY0:
  callee-side untyped formal -> String concatenation loses exact key provenance

RECV0:
  formal key and straight-line method boundaries work, but receiver/field
  identity fails across accepted fallthrough merge or receiver aliasing

NONE-HMI-DELTA0:
  generic compiler matrix is green; inspect exactly one minimized structural
  difference in the HMI-derived shape before selecting another compiler row

any STOP-*:
  return to design consultation
```

## MIR evidence owner

Use one reusable Python checker:

```text
tools/checks/lib/map_field_owner_boxshape_proof.py
```

It runs debug and release observations, emits/reads MIR JSON, normalizes
function-relative evidence, and applies the exclusive classifier.

Record per relevant function:

```text
key producer:
  literal
  local String+Integer
  formal String+Unknown
  passed formal

key MirType:
  String
  Unknown
  other

receiver route:
  MapBox
  RuntimeDataBox

certainty:
  Known
  Union

set/has/get receiver ValueIds and normalized roots
receiver PHI count and input roots
storage field_get / field_set count
callee argument order
CopyOwned / DestroyOwned / ReleaseStrong count
```

The report must preserve `Unknown` and `Union` as observations. It must not
reject them before the runtime matrix is classified.

Hard assertions:

```text
birth publishes exactly one new MapBox into storage
storage field replacement after birth = 0
wrapper inner calls use MapBox.set/has/get
RuntimeDataBox.set/has/get Union routes = 0 for accepted proof
raw MapBox return = 0
static storage facade = 0
CopyOwned = 0
DestroyOwned = 0
new ReleaseStrong = 0
HMI names/imports/special cases = 0
```

Do not assert literal ValueId numbers. Normalize producer/root relations.
`Copy` and PHI instructions are allowed when they preserve the same root.

## Proof runner law

After `PROOF-MANIFEST-HYGIENE0` is green, register one manifest-backed proof:

```toml
[[proof_apps]]
id = "MAPFIELD-R0-STOP0"
app = "apps/map-field-owner-boxshape-proof"
label = "generic MapFieldOwner BoxShape selection proof"
profiles = ["pilot"]
row_kind = "diagnostic"
validation_profile = "scalar-mir"
first_pattern = false
exe = "auto"
cmd = [
  "python3",
  "tools/checks/lib/map_field_owner_boxshape_proof.py",
  ".",
]
```

The app `test.sh` contains only the standard wrapper call:

```bash
exec bash tools/checks/lib/proof_app_test_entry.sh MAPFIELD-R0-STOP0
```

Prefer a small dedicated manifest include rather than growing the root
manifest with a large row block. Do not add a one-off shell guard.

Public focused entry:

```bash
bash tools/checks/run_proof_app.sh --only MAPFIELD-R0-STOP0
```

Add that stable entry to `docs/tools/check-scripts-index.md` only after the
manifest row is executable.

## Implementation order

### PROOF-MANIFEST-HYGIENE0

Behavior delta:

```text
0
```

Required repairs:

```text
remove duplicate MIMAP-085A registration
M216 and M191 test.sh -> proof_app_test_entry.sh wrapper
make M217 and M218 test.sh executable
add MIMAP-019A test.sh wrapper
```

Acceptance:

```text
bash tools/checks/run_proof_app.sh --list
bash tools/checks/proof_app_manifest_test_entry_guard.sh
relevant manifest pilot guard
```

Do not add the new MapFieldOwner proof in this commit.

### R0-STOP0-S0

Add only:

```text
generic source fixture
README boundary
standard test.sh wrapper
semantic expected matrix
```

No compiler, MapBox runtime, HMI, or manifest selection change.

### R0-STOP0-M0

Add:

```text
debug/release VM observation
normalized MIR report
hard no-workaround assertions
```

Do not select KEY0 or RECV0 yet.

### R0-STOP0-V0

Apply the exact exclusive classifier and record the observed token in this
card.

The first result is immutable evidence for the selected compiler task. A later
compiler fix does not silently rewrite the historical diagnostic to
`NONE-HMI-DELTA0`; it transitions or retires the diagnostic expectation in the
fix closeout.

### R0-STOP0-G0

Register the proof, update the public check index, run focused/current gates,
and move `CURRENT_STATE.toml` to exactly one selected compiler task or a new
design stop.

Do not implement the compiler fix in the STOP0 commit.

## Selected compiler task templates

### R0-COMPILER-KEY0

May be selected only by `selection=KEY0`.

Scope:

```text
preserve exact String representation/provenance for:
  untyped method formal
  -> "" + formal
  -> MapBox key argument
```

Non-scope:

```text
receiver identity
MapBox runtime behavior
HMI storage
ownership/backend/grammar widening
```

### R0-COMPILER-RECV0

May be selected only by `selection=RECV0`.

Scope:

```text
preserve one user-box receiver/field-held MapBox root through:
  accepted fallthrough control merge
  receiver alias publication/observation
```

Non-scope:

```text
key typing repair
MapBox runtime behavior
HMI storage
general receiver ABI
ownership/backend/grammar widening
```

## Focused validation order

```text
1. git status -sb
2. current_state_pointer_guard
3. proof-manifest hygiene gates
4. focused debug VM observation
5. focused release VM observation
6. normalized MIR evidence checker
7. run_proof_app --only MAPFIELD-R0-STOP0
8. neighboring exact-numeric helper field-mutation guard
9. HMI T0 authority/inventory guards unchanged
10. source/check file-size and diff checks
11. quick gate 66/66 at closeout
```

## Required counters

```text
selected JSON/HMI parser delta = 0
HMI V0-R0 stash restored = 0
HMI source/check changes = 0
MapBox runtime changes = 0
compiler semantic changes during STOP0 = 0

proof manifest duplicate ids = 0
invalid proof app test entries = 0
new dedicated shell guards = 0

proof owner boxes = 1
proof result collection boxes = 0
raw MapBox return paths = 0
static storage facades = 0
by-name/runtime special cases = 0
fallback/retry/env toggles = 0

CopyOwned = 0
DestroyOwned = 0
new ReleaseStrong = 0

source/check files >= 800 lines = 0
exclusive selection tokens emitted = 1
```

## Stop conditions

Stop immediately if any of these is required:

1. HMI fixture or register source is restored/imported for the proof.
2. Cases share mutable state except where alias/isolation is the subject.
3. Runtime bits alone are used to name KEY0 or RECV0.
4. C5/C6 asymmetry is classified as RECV0.
5. C2 failure is classified as KEY0.
6. MapBox message/receipt text is used as semantic authority.
7. The proof creates an ArrayBox/MapBox result ledger.
8. A raw MapBox carrier, static facade, or caller-side HMI workaround is added.
9. MapBox runtime, ownership, grammar, backend, or HMI production code changes.
10. A by-name branch, fallback, retry, environment toggle, or route probe.
11. The proof manifest hygiene repair and compiler semantic fix are mixed.
12. Any touched source/check file reaches 800 lines.

## Implementation may claim

After `R0-STOP0-G0` only:

```text
one generic HMI-independent MapFieldOwner matrix is executable
debug/release observation and normalized MIR evidence agree
green controls are preserved
exactly one structural selection or design-stop token is emitted
the next compiler task is mechanically selected
HMI production behavior remains unchanged
```

## Implementation must not claim

```text
HMI register storage is fixed
all MapBox mutation shapes work
general receiver ABI support
general untyped formal inference
ownership/view/shared support
backend widening
parser/MIRBuilder migration completion
```

