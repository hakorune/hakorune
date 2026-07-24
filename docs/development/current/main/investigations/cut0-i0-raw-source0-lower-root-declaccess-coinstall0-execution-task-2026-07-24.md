# DECLACCESS COINSTALL0 execution task

Status: **In progress — aggregate owner/preflight sub-slice**
Date: 2026-07-24  
Decision: **COINSTALL-prime-r1**  
Prerequisite: `bfde427dd2` (`MANIFEST0-RUNTIME` sub-slice)

## Decision

COINSTALL0 first adds one Builder-owned aggregate installer owner and its
mutation-free preflight. This sub-slice fixes the paired ownership and
rejection algebra; the actual manifest-derived Builder/shell mutation is the
following DECLACCESS-S0 sub-slice. It does not open a second session, expose
raw parts, or publish a root function.

```text
ManifestBoundRawRootPackageV1
  -> PHYSICAL0
       candidate session + RawRootPhysicalStateV1
       RawRootPhysicalManifestV1 (facts/runtime)
  -> private RawRootEnvironmentInstallOwnerV1
  -> RawRootEnvironmentInstallerV1::prepare(owner)
  -> PreparedRawRootEnvironmentInstallV1
  -> private handoff (installation deferred to DECLACCESS-S0)
  -> InstalledRawRootEnvironmentV1
```

The compiler does not pass a loose `(session, physical, projections)` tuple.
One private aggregate owns the paired session, physical carrier, and all
manifest-derived projections. A rejected product retains that aggregate and
exposes inspection plus discard only.

## Locked questions

### Q1 — owner and entrance

Use a Builder sibling module, for example
`src/mir/builder/raw_root_environment_install.rs`. Its only production-facing
terminal is an internal consuming `prepare(owner)`; the caller-visible
`declare_environment(self)` wrapper belongs to the later DECLACCESS S0 row.

Do not use `ModuleBuilderInvocationSessionV1::prepare_module_session()` here:
that closes external-commit readiness before BODY0. Do not call compiler
`builder_mut()`, `into_parts()`, or expose a shell/collector/ledger tuple.

### Q2 — one manifest, one projection split

The source manifest is the sole authority. Add one consuming split from the
manifest/source-facts product into named projections:

```text
Builder projection:
  route/root-mode fact
  exact plain static-Main declaration fact
  complete non-Clone callable catalog

Shell projection:
  source-file metadata
  sealed declaration facts
  ProvedAbsent static-data / closure capability lanes

handoff payload:
  captured runtime inputs and exact ScalarControl0 body payload
```

The split must move the catalog/body rather than clone or retain a second
source authority in `RawRootPhysicalManifestV1`. The Builder-only projection
constructor is the sole producer of sealed declaration facts; do not rescan
AST, call `current_module`, or invoke the declaration indexer again.

### Q3 — mutation-free prepare

Borrow the aggregate and validate all lanes before consuming it:

```text
brand/family/session/physical agreement
callable catalog destination vacant
Raw user-declaration/root-mode lanes vacant
current function/block, slot registry, context, recursion closed
prelude baseline unchanged
shell function/declaration/static-data/closure lanes vacant
root tracker completed count = 0
Main/condition reservations = 0
ledger has no open reservation or poison state
```

Only Raw-owned lanes must be vacant; immutable prelude metadata is an allowed
baseline. Add read-only ledger clean/open evidence and a named shell
replacement primitive as Builder-private seams where existing APIs are too
narrow. Collector/ledger history from CHILDREN0/CALLMAIN0 is retained and is
not treated as an empty destination.

### Q4 — private infallible handoff

After every fallible identity/vacancy check succeeds, the current sub-slice
performs only a named, consuming handoff. The following DECLACCESS-S0 must
commit only preflighted assignments:

```text
candidate callable catalog install
candidate route/declaration facts install
shell source-file/declaration-fact install
```

The future commit returns no `Result`, uses no `expect`, and performs no lookup
or allocation. Existing `Result`-returning catalog installation must receive
a preflighted Builder-private variant rather than being wrapped in `expect`.

### Q5 — failure and handoff

Every failure returns a named rejected owner retaining the exact session,
physical carrier, manifest/projections, runtime/body payload, and typed cause.
No retry, parts extraction, partial commit, fallback, AST re-resolution, or
BODY0 entry is exposed.

Success returns `InstalledRawRootEnvironmentV1` without claiming that source
facts have already been installed. The next executable row is DECLACCESS S0,
which alone may expose `declare_environment(self)` and produce
`DeclaredRawRootInvocationV1`. BODY0 is its only continuation.

## Required implementation files

```text
ADD
  src/mir/builder/raw_root_environment_install.rs
  src/mir/builder/raw_root_physical/environment_terminal.rs
  tools/checks/lib/cut0_i0_root0_raw_declaccess_coinstall0_guard.py

EDIT narrowly
  src/mir/compiler/raw_root_environment_manifest.rs
  src/mir/compiler/raw_root_source_facts.rs
  src/mir/compiler/raw_root_eligibility.rs
  src/mir/compiler/raw_root_package.rs
  src/mir/builder/raw_root_physical.rs
  src/mir/builder/module_invocation_session.rs
  CURRENT_STATE.toml
```

Keep each source/check file below 800 lines. Do not reconnect the legacy
`RawDraftInvocationV1` S0 path; its runtime snapshot discard remains an
explicit disconnected non-claim until a separate retirement/handoff row.

## Acceptance

```text
one aggregate installer producer = 1
one consuming manifest projection split = 1
compiler builder_mut/raw tuple = 0
prepare_module_session in COINSTALL0 = 0
current_module/AST rescan/indexer = 0
catalog clone/second sealing = 0
shell/collector/ledger re-acquisition = 0
partial commit/expect/retry/fallback = 0
BODY0/root batch/production consumer = 0
all modified/new source/check files < 800 lines
```

Required tests for this sub-slice:

```text
Script and App co-install success
catalog and declaration-fact destination dirty -> rejection
shell metadata/function lane dirty -> rejection
foreign brand/family/session -> rejection
open/poisoned ledger or nonzero root tracker -> rejection
all rejection snapshots: session/physical/manifest unchanged
success: paired owner handoff retains session/physical/manifest once
```

The following installation tests belong to DECLACCESS-S0, not this owner
sub-slice:

```text
catalog and declaration facts installed once
shell source-file/declaration metadata installed once
runtime/body payload retained once after projection split
```

Known follow-up: runtime snapshot tests currently use module-local environment
mutexes. A shared test lock/helper is a separate guard row; it is not a reason
to widen COINSTALL0 or alter production ownership.

## Non-claims

```text
BODY0/root body lowering
Main/condition root batch
drain/finalizer/postprocess/external commit
public ingress or JSON bridge behavior
legacy RawDraftInvocation retirement
production consumer
CUT0 activation
```
