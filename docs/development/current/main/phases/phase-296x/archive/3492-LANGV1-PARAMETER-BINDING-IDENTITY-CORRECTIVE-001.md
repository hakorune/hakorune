# 3492 - LANGV1-PARAMETER-BINDING-IDENTITY-CORRECTIVE-001

## Status

Complete. The stop-the-line BindingId corrective is green and 3493 may start.
This is a behavior-preserving identity correction, not a language decision.

Decision: accepted corrective scope.

## Trigger Evidence

```text
LANGV1_GRAMMAR_FULL=1 substrate guard:
  FAIL full Rust/Hako grammar conformance

differential batch:
  12/12 Hako normalized forms missing
  first failure = parser/hako_adapter_process_error

direct adapter stderr:
  [type/local_contract_binding_missing] name=flag boundary=assignment
```

The shared 106-fixture corpus remains structurally sound. The red is the Hako
adapter compilation path: an instance-method parameter is reassigned without a
BindingId.

## Root Cause

```text
static parameters:
  setup_function_params publishes ValueId + function_param_names
  AssignmentResolver accepts function_param_names without BindingId

instance parameters:
  setup_method_params publishes ValueId
  does not publish function_param_names or BindingId
  reassignment fails

synthetic pin names:
  AssignmentResolver accepts every __pin$ prefix without BindingId
  this is a second string-based identity exception
```

`function_param_names` is useful observation inventory for normalized-shadow
inputs, but it is not lexical identity authority. `__pin$` temporaries are
direct SSA values and must not enter named-assignment resolution.

## Structural Owner

Add one function-entry parameter declaration API owned by the MIR builder's
existing BindingId allocator and `binding_ctx`.

```text
declare_function_parameter(name, value_id, parameter_kind)
  -> publish variable_map
  -> allocate one BindingId through existing CoreContext owner
  -> publish binding_ctx
  -> record observational parameter inventory
  -> register value kind / slot metadata through existing owners
```

Do not use `declare_local_in_current_scope`: parameters exist at function entry
and local shadowing must capture/restore the parameter identity through the
normal lexical-scope frame.

## Ordered Tasks

1. Add the common parameter declaration API. Do not add another BindingId
   counter, name hash, fixed ID, or parameter-specific identity namespace.
2. Route static parameters, instance receiver `me`, and explicit instance
   parameters through the same API.
3. Keep receiver offset and `MirValueKind::Parameter` indexing explicit. Prove
   parameter carrier indexes and BindingIds are independent identities.
4. Remove `function_param_names` as an `AssignmentResolverBox` acceptance
   authority. Named assignment requires both current ValueId and BindingId.
5. Remove the `__pin$` prefix exemption from assignment resolution. Pin
   temporaries remain direct SSA values; reaching named assignment is a
   compiler-contract failure, not a compatibility path.
6. Preserve normalized-shadow input observation through
   `function_param_names`, or replace it only with a typed parameter inventory.
   Do not make normalized shadow infer parameters from names.
7. Add focused fixtures:
   - static parameter reassignment;
   - instance parameter reassignment;
   - receiver identity presence;
   - local shadowing of a parameter and outer identity restoration;
   - synthetic pin never enters named assignment.
8. Add one Hako adapter regression fixture that exercises an instance-method
   parameter reassignment in the actual grammar adapter dependency graph.
9. Run the normal and `LANGV1_GRAMMAR_FULL=1` grammar guards. The differential
   report must return 12/12 with one Hako adapter process.
10. Add a changed-surface guard: changes under MIR builder vars/parameter setup,
    MIR type-contract owners, Hako compiler/parser sources, or Language v1
    adapter/projection tools must invoke the FULL grammar gate in the owning
    milestone/CI entry. Use a centralized path manifest; do not duplicate path
    lists across shell scripts.

## Stable Fail-Fast Tags

```text
type/parameter_binding_identity_missing
type/parameter_binding_identity_duplicate
type/parameter_binding_identity_drift
type/local_contract_binding_missing
type/pin_named_assignment_forbidden
type/language_v1_full_gate_required
```

Existing site-specific tags remain valid. Define new strings once in their
selected owner.

## Fixture Matrix

| Fixture | Expected |
| --- | --- |
| static `method f(flag) { flag = false }` | same BindingId before/after assignment |
| instance `method f(flag) { flag = false }` | same BindingId before/after assignment |
| instance receiver `me` | function-entry BindingId exists |
| `local flag` shadows parameter | new BindingId; parameter restored on scope exit |
| duplicate parameter publication | fail duplicate tag |
| ValueId exists but BindingId absent | fail missing identity |
| `__pin$...` reaches AssignmentResolver | fail pin named-assignment tag |
| differential batch | 12/12 recursive witness parity |

## Gate Coupling

Create one checked sensitive-path manifest and one wrapper used by milestone/CI.
The wrapper accepts an explicit comparison base or changed-file list and runs
the FULL gate when any sensitive path matches. It must not depend on developer
shell history, a stale success stamp, or a hidden environment toggle.

Initial sensitive owners:

```text
src/mir/builder/vars/**
src/mir/builder/calls/parameter_setup.rs
src/mir/type_contracts/**
lang/src/compiler/**
tools/language_v1/**
grammar/language-v1-*
```

## Acceptance

```text
parameter_binding_identity_owner_count = 1
second_parameter_binding_allocator = 0
static_parameter_binding_registered = 1
instance_parameter_binding_registered = 1
receiver_binding_registered = 1
function_param_names_assignment_authority = 0
pin_prefix_assignment_authority = 0
instance_parameter_reassign_fixture = green
language_v1_full_gate = green
differential_case_count = 12
differential_hako_adapter_process_count = 1
sensitive_change_full_gate_coupling = 1
changed_production_source_over_800_lines = 0
```

## Closeout Evidence

```text
implementation commit:
  0b70fc96a0 fix: unify function parameter binding identity

identity owner:
  declare_function_parameter
  existing CoreContext::next_binding allocator only

focused tests:
  parameter_setup unit tests = green
  assignment_resolver unit tests = green
  static/instance reassignment source fixture = green
  parameter shadow/restore source fixture = green

FULL gate:
  LANGV1_GRAMMAR_FULL=1 language_v1_grammar_contract_substrate_guard.sh = green
  differential case count = 12
  Hako adapter process count = 1

changed-path coupling:
  one sensitive-path manifest
  one local/CI wrapper
  min-gate PR base comparison wired

source line ceiling:
  parameter_setup.rs = 224
  lowering.rs = 544
  assignment_resolver.rs = 110
```

## Explicit Non-Claims

```text
grammar_contract_changed = 0
parser_acceptance_changed = 0
next_type_family_selected = 0
record_contract_activation = 0
typed_array_contract_activation = 0
new_backend_lowering = 0
selfhost_claim = 0
```

## Next

Proceed to accepted 3493 record-value contract implementation. The 3491
record-versus-Array decision is already closed in favor of record
construction/update.
