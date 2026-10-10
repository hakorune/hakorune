# Static CurrentOwner two-caller actual S1

Status: selected implementation
Date: 2026-10-10
Scope: MIRBUILDER-STATIC-CURRENTOWNER-TWO-CALLER-ACTUAL-S1
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-static-currentowner-condition-cohort-d0-2026-10-10.md
  - src/mir/normal_callable_semantic_package/README.md

## Selected replacement

In unchanged imported `size_class_box.hako`, `SizeClassBox.size_to_bin(size)`
has exactly two original CurrentOwner callers: `good_size` Body(0)
initializer and `accepts` Body(0) Eq Lhs. Replace their `SourceStatic`-only
outgoing actual responsibility together with one checked executable cohort.
Do not promote the initializer alone. The preceding one-caller
`size_to_bin -> normalize_size(size)` remains on its existing path.

The original `QualifiedStaticCallClaimIndexV1` and whole incoming inventory
own source identity. Reuse `issue_original_static_forwarded_actual_v1`, the
two original Home observations, selected physical signature, I64 result and
the existing `size_to_bin` Completion. The sole Static packet owner remains
`CallPacketSourceV1::static_i64`; this slice may grant that owner exact
completed actuals for both sites, but cannot issue a second packet route.
The existing V2 loop product already covers `size_to_bin` header/body/tail
and has a real-source standalone collector test. This does not prove final
module link or the integer Eq condition.

`seed_static_transport_owners_v1` currently admits one CurrentOwner
initializer and `borrowed_formal_incoming.rs` marks the Eq caller as an
unsupported context. The one-input finisher requires a one-row cohort.
Change these gates only for the exact two-row checked shape, using original
sites and claims rather than method-name inference. A nonmatching
noninitializer context retains the old fail-closed boundary. The original
actual `Rc`, target, ordinal, formal, source site, borrowed I64 class and
Home argument must agree at both sites before either gains executable phase.

## Acceptance

1. The unchanged real source has exactly the original `good_size` initializer
   and `accepts` Eq-Lhs incoming rows for `size_to_bin`; both completed
   actuals refer to the same callee signature/result/Completion and their
   own original source/Home argument. Both must become available together.
2. Missing or duplicate caller, changed source site/target/ordinal/formal,
   wrong actual class, Home argument drift, or absent signature/result/
   Completion rejects the whole two-row executable cohort. A foreign
   noninitializer caller remains source-only. No partial promotion or retry.
3. Existing one-caller `normalize_size` executable actual/packet and
   real-source V2 loop collector positives/negatives remain green. Run the
   focused Static actual/entry/packet tests and pointer guard. Classify any
   broader red against the recorded baseline.

No physical integer Eq envelope, final module link, mimalloc-lite EXE
advance, or old-edge deletion is claimed by source/actual completion alone.
After this S1, the condition cohort D0 owns ordered Lhs/Rhs physical
Invoke/Normal/Fault/Compare work.
