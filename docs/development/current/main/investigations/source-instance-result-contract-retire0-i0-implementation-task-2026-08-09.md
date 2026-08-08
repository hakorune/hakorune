---
Status: closed — bounded caller-zero retirement landed
Date: 2026-08-09
Decision: delete the old body-inferred instance-result/target family before declaration-first target I0
Parent: `source-instance-result-contract-retire0-r0-task-2026-08-09.md`
---

# SOURCE-INSTANCE-RESULT-CONTRACT-RETIRE0-I0

## Scope

Retire the audited caller-zero source-instance-result family in one
behavior-neutral BoxShape slice. The family is not a production route and must
not remain beside the declaration-first resolver target.

Delete:

```text
src/mir/source_instance_result_contract/**
src/mir/mod.rs source_instance_result_contract declaration
src/mir/callable_result_representation/body_proof_issue.rs
solver.rs issue_unannotated_body_proof() and its old-only imports
```

Remove the old-only test fixture helpers and rebind/pre-loop tests. Preserve
the general `source_call_target` source-site primitives and the unrelated
`callable_result_representation` result owners; only the unannotated
body-proof issuer is retired. The four raw source-view
cursor tests remain, but use a neutral fixture helper that supplies only the
catalog, caller, and exact nested-method sites; they must not construct or
pass a target/result contract.

## Allowed changes

1. Delete the caller-zero module and its exports/tests.
2. Remove only `instance_result_contract_source`, `stageb_source_for_lowering`,
   `with_instance_result_contract_inputs`, and the stage-B carrier fixture
   helpers if their only callers are the retired tests.
3. Rename/reduce the raw cursor fixture to a neutral source-view name and
   remove unused target/result callback arguments.
4. Update the module README, `src/mir/README.md` or relevant module index,
   this task receipt, and the implementation/task-map pointer in the same
   commit.

## Acceptance

```text
production imports/calls to source_instance_result_contract = 0
src/mir/mod.rs declaration removed
old target/result/rebind/preloop files deleted
general source_call_target users remain green
raw source-view cursor tests remain green through neutral fixture
no `instance_result_contract` helper/name remains in production or active test code
no new resolver target, Recipe/CallSlot, Builder/MIR route, or fallback
all changed Rust files < 760 lines
same-slice docs/reference/task receipt and pointer update
```

Required focused evidence:

```bash
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust source_call_target
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust callable_result_representation
RUSTFLAGS=-Awarnings cargo check -q -p nyash-rust
git grep -n 'source_instance_result_contract\|VerifiedCurrentOwnerInstanceResultTargetV1\|SealedNestedInstanceResultContractV1' -- src crates lang
```

The final grep must show no active code occurrence; historical investigation
documents may retain the name as a retirement record.

## Nonclaims

```text
no declaration-first resolver target
no CallableContract semantic issuer
no Home ABI / body conformance / physical ABI
no Recipe/CallSlot or source-bound call relation
no Builder/MIR/CFG/PHI/provider/runtime change
no production selection, retry, fallback, or provider behavior
```

## Closeout

After the implementation and focused gates are green, update the R0 audit
receipt, `source_call_target`/fixture README as needed, the language/callable
task map, `CURRENT_STATE.toml`, and `10-Now.md` in the same commit. Then push
the commit. The next design stop may open declaration-first resolver contract
issuance; it must not infer a replacement target from this retired family.

## Landed implementation receipt — 2026-08-09

The caller-zero family was removed in this slice:

```text
deleted src/mir/source_instance_result_contract/**
deleted src/mir/callable_result_representation/body_proof_issue.rs
deleted solver.rs::issue_unannotated_body_proof()
removed the mir module edge and old-only rebind/pre-loop fixtures
renamed/reduced the raw source-view fixture so it carries no target/result
contract
```

The general `source_call_target` primitives and static result-representation
owners remain. No declaration-first target, Recipe/CallSlot, Builder/MIR
route, or fallback was introduced.

Focused evidence:

```text
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust source_call_target        # 55 passed
RUSTFLAGS=-Awarnings cargo test -q -p nyash-rust callable_result_representation # 72 passed
RUSTFLAGS=-Awarnings cargo check -q -p nyash-rust                        # passed
git grep active old-family names in src/crates/lang/tools/checks           # empty
git diff --check                                                          # passed
bash tools/checks/current_state_pointer_guard.sh                          # passed
python3 tools/rust_lifecycle/mirbuilder_native_owner_candidate_inventory.py --check # passed
```

All changed Rust files remain below the 760-line review threshold. The
generated native-owner candidate inventory was regenerated to remove retired
paths and its check is green. The existing
`callable_result_i64_catalog_s0.py` guard still reports its unrelated baseline
`SourcePath projector owner count drift`; this retirement slice does not alter
that projector owner.

The next stop is design-only:

```text
LOOP-RESOLVER-CANONICAL-CALLABLE-CONTRACT-D0
```

It must first close the declaration-first source authority, typed Query
profile, same-declaration Home ABI relation, and separate body-conformance
boundary before any target or Recipe work opens.
