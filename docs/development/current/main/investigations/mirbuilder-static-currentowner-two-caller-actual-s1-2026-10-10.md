# Static CurrentOwner two-caller actual S1

Status: design stop; target-scoped executable ingress decision pending
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
An attempted exact two-row global seed compiled, but the unchanged imported
source failed at `HakoAllocHeap.allocateResult -> me.allocate(size)` with
`ordinary-new/borrowed-actual/entry-source`. It put `size_to_bin` into the
global `transport_owners` before graph closure; that activated unrelated
Heap/Page actual construction. The rejected actual was `SelfRooted` formal
`size` (binding 1), while the entry receiver was binding 0. Treating it as
the receiver would weaken the ownership contract. The source code experiment
was removed after the failure; its patch is retained outside the checkout at
`/tmp/hako-two-caller-global-seed-attempt-20261010.patch` for diagnosis.

Decision brief:

- Source authority + canonical issuer: the immutable original
  `source_incoming` inventory, existing Eq source selector and Home pair,
  selected signature/result/Completion, and sole
  `CallPacketSourceV1::static_i64` physical packet issuer.
- Non-authority: adding `size_to_bin` to global `transport_owners`, a
  source-only actual, a name/AST shortcut, or independent packet construction.
- Fail-fast boundary: both original callers must agree or neither gets an
  executable actual; unchanged Heap/Page source-only behavior must remain.
- Smallest next slice: design one target-scoped checked cohort from
  `source_incoming.project({size_to_bin})`, consumed by the actual finisher,
  Static packet selection, and callee borrowed entry. Merely completing
  actuals is insufficient because those consumers currently read global
  `source.incoming` / `source.definitions`.
- Non-claims: no executable two-caller entry, integer Eq, final module link,
  EXE advance, or Heap/Page actual support is established yet.

Before resuming implementation, fix one shared target-scoped accessor and
its entry/packet consumers in this card. Preserve the exact pre-Home Eq
eligibility query: resolved If-region, Equal binary, one-input CurrentOwner
I64 Lhs with local actual, zero-input CurrentOwner I64 RHS, both original
Static claim sites, exactly one initializer plus this Lhs and no other caller.
The later Home pair remains mandatory. The original actual `Rc`, target,
ordinal, formal, source site, borrowed I64 class and Home argument must agree
at both sites before either gains executable phase.

## Acceptance

1. The unchanged real source has exactly the original `good_size` initializer
   and `accepts` Eq-Lhs incoming rows for `size_to_bin`; both completed
   actuals refer to the same callee signature/result/Completion and their
   own original source/Home argument. Both must become available together.
2. Missing or duplicate caller, changed source site/target/ordinal/formal,
   wrong actual class, Home argument drift, or absent signature/result/
   Completion rejects the whole two-row executable cohort. An initializer
   paired with `if me.pick(p) > 0` or `&&` does not qualify at source selection
   and remains source-only; a foreign noninitializer caller does likewise.
   The unchanged Heap/Page route must also retain its pre-S1 source-only
   boundary instead of failing package construction at `entry-source`.
   No partial promotion or retry.
3. Existing one-caller `normalize_size` executable actual/packet and
   real-source V2 loop collector positives/negatives remain green. Run the
   focused Static actual/entry/packet tests and pointer guard. Classify any
   broader red against the recorded baseline.

No physical integer Eq envelope, final module link, mimalloc-lite EXE
advance, or old-edge deletion is claimed by source/actual completion alone.
After this S1, the condition cohort D0 owns ordered Lhs/Rhs physical
Invoke/Normal/Fault/Compare work.
