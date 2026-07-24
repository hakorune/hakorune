# DECLACCESS MANIFEST0 execution task

Status: **In progress — source-facts sub-slice**
Date: 2026-07-24  
Decision: **DECLACCESS-IMPLEMENTATION-prime-r1**  
Prerequisite: `7cfde485f8` (`COVERAGE0`)

## Goal

Create one exact, non-Clone source manifest before PHYSICAL0 opens. The
manifest is the only source authority later consumed by DECLACCESS and BODY0.

The first implementation commit is intentionally limited to the source-facts
authority. It must not be called a MANIFEST0 closeout until runtime/config
ownership and the Builder/shell co-install terminal are also present. Those
remain explicit follow-up slices; no physical or production consumer is
claimed by this card yet.

## Owner transition

```text
EligibleSourceBoundRawRootPackageV1
  -> ManifestBoundRawRootPackageV1
       RawRootEnvironmentManifestV1
       RawRootEligibilityV1 coverage witness
       source/plan/config/module name
  -> open_physical(current)
       manifest moves into RawRootPhysicalCoreV1
```

Manifest failure retains the eligible owner. `RawRootPhysicalStateV1::open`
and `ModuleBuilderInvocationSessionV1::open_for_token` occur only after the
manifest-bound owner exists.

## Source-facts authority

Add a sibling `raw_root_source_facts.rs` below 800 lines. It performs the one
accepted-route source analysis and owns:

```text
Script/App route and fixed physical root identity
lexical static-helper schedule
located ScalarControl0 payload (literal/name/operator/path/span)
exact Main locator and callable rows
complete existing callable declaration catalog, sealed once and moved
runtime/config snapshots
ProvedAbsent static-data / closure / process-slot seals
```

Do not re-scan the AST, call `current_module`, or re-seal the callable catalog
after this product is created. Keep the summary PLAN0 view derived/temporary;
do not introduce a second environment authority.

## Required edits

```text
ADD
  src/mir/compiler/raw_root_source_facts.rs
  src/mir/compiler/raw_root_environment_manifest.rs

EDIT narrowly
  src/mir/compiler/raw_root_eligibility.rs
  src/mir/compiler/raw_root_children.rs
  src/mir/compiler/raw_root_callable_main.rs
  src/mir/compiler/mod.rs
```

No Builder/shell installation, BODY0 lowering, root batch, drain, finalizer,
postprocess, external commit, public ingress, or production consumer belongs
to this row.

## Acceptance

```text
manifest producer = 1
manifest-bound package producer = 1
physical/session open after manifest = 1
source classifier authority = 1
callable catalog sealing = 1
literal/path/span retention = exact
Script/App coverage witness = consumed, not re-decided
manifest/physical/source products = non-Clone
AST/current_module re-read after manifest = 0
compiler builder_mut / physical tuple = 0
all touched files < 800 lines
```

Verification:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
cargo check -q --lib
cargo test -q raw_root --lib -- --test-threads=1
```

## Interim source-facts acceptance

The source-facts sub-slice may close only these items:

```text
located ScalarControl0 payload = owned exactly once
existing callable catalog seal = one producer
source-facts module = <800 lines
production consumer = 0
runtime/config snapshot handoff = sealed and single-move
Builder/shell co-install = not yet claimed
```

Current evidence for this interim slice:

```text
cargo check -q --lib = green
cargo test -q raw_root --lib -- --test-threads=1 = green
```

The runtime/config handoff is now a separate sealed transition:

```text
RawSourceContinuationV1 -> RawRootContinuationV1
SourceBoundRawRootPackageV1
  -> ManifestBoundRawRootPackageV1
       manifest owns runtime inputs + Builder config
  -> PHYSICAL0 consumes config once into the session
       physical manifest retains runtime inputs
```

The legacy `RawDraftInvocationV1::open` consumer remains disconnected S0
evidence. It is not part of the root MANIFEST0 lane and does not claim runtime
input propagation; its package destructure intentionally discards the runtime
snapshot until that legacy owner is retired or receives its own handoff row.
The root lane has no such discard terminal.

The remaining MANIFEST0 claim is the Builder/shell co-install and its
mutation-free rejection matrix. No `declare_environment(self)` or production
consumer is claimed yet.
