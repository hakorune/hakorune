# MIRBUILDER-CLOSED-APP-STATIC-OMISSION-CLOSURE-S0

Status: implementation verified; original CLI first-stop pending
Date: 2026-10-11
Scope: conservative closed-App static physical-selection closure
Related: `mirbuilder-root-exit-static-two-exit-s0-2026-10-11.md`, `src/mir/source_call_target/whole_source_inventory.rs`

## Evidence and problem

Root-exit two-exit S0 remains unaccepted. Its pending transaction and shared
nonempty Home-order checks pass; JSON/EXE physical acceptance and final
caller-zero/deletion closeout remain owed. This selection dependency must
not close that parent or waive its physical requirements.

At source revision `98ee7a4be9`, the existing S0 CLI binary reproduces EXE
package issue `static-scalar/incoming-veto: NoIncoming(slot7)`. GDB traces
`checked_completed_static_scalar_cohort_v1` to
`select_static_scalar_local_routes_v1` ->
`issue_signature_and_finish_inputs_v1` -> package issuance. The actual source
input7 is `SizeClassBox.good_size`; resolver AppMain preparation reserves
owners in that input order. Its sole source incoming caller is
`SizeClassBox.good_size_usize`, which has zero incoming and is omitted.
The source inventory's one-pass omission retains good_size because it counts
that wrapper edge, but the later incoming scan omits the wrapper.

Diagnostic command (not an acceptance pass):
`NYASH_DISABLE_PLUGINS=1 HAKO_BACKEND_COMPILE_RECIPE=pure-first
HAKO_BACKEND_COMPAT_REPLAY=none target/quick/hakorune --emit-exe
/tmp/s0_98ee7a4be9_probe.exe --emit-exe-nyrt target/lifecycle-kernel/release
/tmp/s0_size_class_box_main.hako`, rc1; no artifact.
Source is unchanged size_class_box.hako plus trivial Main returning0.
Binary is the existing recorded S0 build, not rebuilt at this revision;
SHA256 `01488067e3a8ea9580595995b74c8c1bf3eefbdf8486b58b0bf99080074b2593`.
Combined source SHA256
`bef465541a085d10586601560d5e5ff764811f0e3af744357ffbcf797c12c06c`.
The JSON document profile reaches its separate Invoke transport rejection.

An initial worker hypothesis that omitted owners themselves retain drafts
was disproved: source_drafts skips absent selected keys, and incoming
inventory requires every draft owner to have a selected key. The issue is
the remaining callee, not an omitted-owner query. No workaround was applied.

## Decision

Source authority + canonical issuer: existing verified whole-source target
inventory, AppMain catalog co-seal and declaration attributes. Extend only
`closed_app_zero_incoming_static_keys` to the least fixed point of its same
zero-incoming omission rule: once a caller is omitted, its outgoing calls
cannot alone keep another candidate physically selected.

Non-authority: runtime reachability, MIR names, scalar draft spelling,
failed incoming query, changing Main to add evidence, or a new root list.

Fail-fast boundary: keep Main-owner methods and Public/Internal/FFI/ABI rune
methods; keep a candidate on unresolved same-selector calls from any retained
caller. Observation gaps and foreign AppMain brand still yield no omissions.
Instance/top-level retained callers remain incoming evidence. Cycles remain
selected under this conservative zero-incoming rule. Retain all original
source declarations, target claims and inventory; only physical membership
changes. AllSource selection is unchanged. NoIncoming and whole-cohort veto
for still-selected owners remain intact.

## Tasks and acceptance

1. Replace the existing one-pass projection with monotonic finite closure
   in that same owner. Do not add an incoming-error suppression or authority.
2. Extend existing omission tests with a multi-level unrooted chain, Main and
   exported roots keeping their dependencies, uncertain instance incoming,
   and a disconnected cycle. Preserve existing foreign-brand/gap tests.
3. Run focused positive/negative omission tests, related source-inventory
   regressions and the existing incoming-veto tests using one Cargo build.
4. Rebuild CLI as required and record original-source EXE first-stop. Passing
   a trivial Main after omitting its unused helpers does not prove execution
   of the selected two-exit function. Parent S0 still needs its exact physical
   requirements and relevant execution evidence.
5. Commit/push verified changes, then return pointer to the open root-exit
   S0 with exact remaining physical boundaries; no parent acceptance claim.

Non-claims: no global dead-code elimination, closed-world reachability sweep,
cycle deletion, source rewrite, broader scalar entry, general JSON Invoke
admission, whole SizeClassBox/mimalloc-lite/MirBuilder completion, or physical
old-owner retirement. This replaces one-pass physical selection responsibility;
source readers and shared incoming validators are retained.


## Implementation evidence

Existing omission issuer now computes `next` against the prior immutable
omission set, adds those candidates monotonically, and returns at the finite
fixed point. No source inventory/target/declared-function removal, new
receipt, incoming suppression, or scalar route change. Owner README records
the same selection boundary. Read-only worker verified the retained roots,
uncertain incoming, nonstatic callers, observation-gap/brand fences and cycles.

Focused `closed_app_static_omission`2/2 PASS; quick build/link8m12s,
test0.00s. Reuse of that executable: whole source inventory9/9 PASS0.01s,
existing good-row-plus-unresolved-caller veto1/1 PASS0.01s, existing
NoIncoming-cannot-authorize-ABI-change1/1 PASS0.01s. The original selected
Static two-exit transaction test1/1 PASS on the same executable, retaining
AllSource selected behavior. Test inventory remains9100; no baseline update.
Touched-file format/diff and pointer guard PASS. The prior unrelated scope
guard count debt is unchanged, as recorded in the parent S0 card.

Remaining: rebuild CLI and record original EXE first-stop. Until then this
slice is not fully closed and the parent remains open. In particular, omission
of unused helpers from a trivial Main is not selected two-exit execution proof.
