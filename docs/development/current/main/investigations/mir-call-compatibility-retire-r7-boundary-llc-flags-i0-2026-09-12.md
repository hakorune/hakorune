---
Status: closed__Fast__R7BoundaryLlcFlags__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-BOUNDARY-LLC-FLAGS-I0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-boundary-llc-flags-d0-2026-09-12.md
---

# R7 Boundary llc-flags implementation

## Contract

The existing Boundary profile1 physical-options owner is the only issuer. At
admission, NULL `llc_flags` captures the existing C default once; a non-NULL
empty string remains explicit no flags; a nonempty string is copied unchanged.
The Rust Boundary callers must preserve empty versus unset for this field. The
existing `llc_flags_for` path consumes only the invocation-owned copy and never
re-reads ambient state. Generic, Static, Harness, AOT/public/link symbols and
MIR routes are outside the delete-set.

## Focused acceptance

Use the existing fake-tool options smoke and add only its Boundary profile1
coverage: unset -> `-O3 -mcpu=native`, empty -> no flags, explicit -> exact
value, and environment mutation after admission -> captured value. Assert the
child receives no ambient flag and that invalid contract/tool inputs reject
before JSON/child/artifact effects. Keep existing Generic/Static/Harness tests.

Required gates: C FFI build, options smoke, static/route/pointer guards,
`git diff --check`, and source-size limits. No Cargo or MIR test is required
for this C/Rust transport-only change unless the existing focused owner gate
requires it.

## I0 closeout evidence

The existing C options smoke passed after the change, including the private
ownership test: Boundary NULL captures the effective value before a later
environment change, explicit empty remains empty, and the copy is owned by the
invocation. The main `nyash-rust` plugin check passed in 33.19s and the
standalone `nyash-llvm-compiler` check passed in 0.40s, both with jobs=1.
Static/route/pointer guards and `git diff --check` passed; source files remain
below the 800-line hard boundary. Generic, Static and Harness profiles were not
changed by the Boundary-only materialization.
