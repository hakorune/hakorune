# MIRBUILDER-CLOSED-APP-STATIC-PUBLICATION-SELECTION-S0

Status: verified; ready for commit and parent handoff
Date: 2026-10-11
Scope: physical publication demand for explicitly omitted Closed-App Static callers
Related: `mirbuilder-closed-app-static-omission-closure-s0-2026-10-11.md`, `mirbuilder-root-exit-static-two-exit-s0-2026-10-11.md`

## Decision

The original EXE at `0ac73da948` gets past NoIncoming, but publication drain
rejects omitted `SizeClassBox.accepts/1` calling `size_to_bin/1` at
`[Body(0), IfCondition, Lhs]`. The package has already co-sealed omission;
the production publication issuer still projects demand from all source rows.
Read-only worker independently confirmed both sides of this mismatch.

Source authority + canonical issuer: borrow the package's existing
`VerifiedSourceBackedSameModuleCallableCatalogV1`. Publication issuance derives
its declaration catalog from that loan and checks the original target/result
brands. Keep all exact source target rows, but do not mint physical Selected
or TargetOnly demand for a caller explicitly in that catalog's omitted cohort.
Retained callers keep the existing projection and one-shot residual checks.

Non-authority: target membership, a fresh reachability sweep, names, arbitrary
predicates, post-issuance pruning, or suppressing `finish_empty`.

Fail-fast boundary: wrong catalog brands and accidental takes for omitted
caller sites still reject. The existing AllSource issuer is unchanged. No
source declarations, target/result catalogs, or shared consumer are removed.

## Tasks / acceptance

1. Lend the existing source-backed catalog from the package; add the selected
   source issuance entry to the existing publication owner and connect the
   production Script lookup issuer. Reuse one common issuance algorithm.
2. Exercise that production issuer: omitted source target retained without
   physical demand; retained caller to the same target still residual; both
   Selected and TargetOnly obligations respect caller omission; AllSource
   unchanged; wrong brands and omitted-site take reject.
3. Run focused positive/negative and related publication/lookup tests with one
   Cargo build, format/diff/pointer checks. Rebuild CLI and record the same
   original-source EXE first-stop without claiming trivial Main executes the
   selected two-exit function. Commit/push only verified changes.
4. Return to parent S0 only with accurate remaining acceptance obligations.

Non-claims: parent two-exit S0 acceptance, final caller-zero/deletion review,
JSON Invoke transport, whole SizeClassBox/mimalloc-lite/MirBuilder completion.
The previous omission row also retains its changed real-mimalloc red
classification obligation; pre-existing red is not equivalent first-stop.
No parked lane is reopened and no failure baseline is updated by this slice.

## Implementation / verification in progress

The package now lends its existing source-backed catalog; the publication
owner derives declarations from that loan and uses one shared issuance loop.
It retains exact target rows before skipping physical demand for an explicit
omitted caller. Production Script issuance uses this entry. Read-only worker
found no substantive issue in that change. Owner README records the contract.

The focused production-issuer test covers AllSource versus ClosedApp,
retained Main calling the same target, residual rejection, omitted-site take
refusal and foreign target brand. Initial compilation exposed a test reference
to a non-reexported error type; that build was stopped, Cargo/rustc termination
checked, and the test fixed without widening the production export surface.
The first focused build finished in8m16s:1/1 PASS0.01s. That same executable
passed the existing lookup batch9/9 and publication-owner batch11/11, each
0.01s. This validates Selected caller projection, not the whole slice.
The same test now also covers Float-result TargetOnly caller projection and
foreign result-catalog rejection. Its final rebuild finished in8m12s:
1/1 PASS0.01s. Reused that executable for lookup9/9 PASS0.02s,
publication-owner11/11 PASS0.01s, and original Static two-exit transaction
1/1 PASS0.04s. Inventory9101; no full-suite or baseline update is claimed.
Touched-file format/diff checks PASS. Fresh CLI evidence is recorded below.

Read-only GDB on the recorded `0ac73da948` CLI and its saved test executable
maps the prior row's entry-source red to `HakoAllocHeap.reallocResult`
(CLI slot34, test slot28), while test slot14 is `LayoutBox.class_size`.
This identifies the later real callable reached after omission closure, but
does not yet establish why its exact binding/root fails. That classification
obligation stays open; no baseline-equivalence claim is made.

## Next physical evidence boundary (read-only)

Before the new CLI build, the recorded `0ac73da948` binary was also used on
the unchanged SizeClassBox body with three Main invocations: size_to_bin(0),
size_to_bin(9), size_to_bin(4194305). All rc1, no artifact, at
`mir/main-import-view/selected-header-missing` (0.032s each). These are
additional execution probes, not replacement authority or accepted fixtures.

The existing Main qualified-method relation requires an explicit scalar
result header and all ExactI64 formals. The target deliberately has neither:
its unannotated result and Opaque formal use the existing Static V2
signature/packet owner. Read-only worker confirmed that lending this signature
as an annotation header would mix authorities, and would still fail the
parameter check. Future parent S0 work must determine the terminal qualified
call ingress to that same Static packet/incoming/publication owner; adding
source annotations or bypassing the scalar-header checks is not authorized.
New CLI results must separately confirm whether these boundaries remain.

The replaced AllSource publication wrapper now has only cfg(test) callers;
read-only caller census confirms the sole production issuer uses the typed
source selection entry. After Cargo termination was checked, that old
wrapper was limited to cfg(test). This retires only the replaced production entry, keeps
its test behavior and the common issuance algorithm, and avoids a new
production dead-code warning. Tests above used the identical test-cfg body;
the subsequent non-test CLI build verifies caller removal as recorded below.

## Final CLI receipt / closeout

Non-test CLI build PASS3m49s,1729 warnings (same count as the preceding CLI).
It compiles with the old declaration-only issuer unavailable outside test cfg,
proving no remaining production caller. Binary SHA256
`071eaaf9c20e16dafa9d25cdfff25558af1ff10d903a65f32b84787107d8399b`;
build source is `0ac73da948` plus Rust patch SHA256
`c6af4fdd131b0471acd13ff48cc8e0744f3fd8fd0ff9c27a48c94964be3db022`.
No Rust source changed after this build.

With the same env/command/source as the preceding omission receipt
(combined source SHA256 `bef465541a085d10586601560d5e5ff764811f0e3af744357ffbcf797c12c06c`),
trivial Main gets past publication drain and MIR generation. The unchanged
lifecycle-only runtime directory lacks the ordinary libnyash_kernel.a needed
by this scalar artifact; that rc1 is an invocation setup limit, not a compiler
regression. The existing archive at `target/release/libnyash_kernel.a`
(SHA256 `e000c0e45ceb1a5a03acb96629b5f8ebf4ce31f5095aa9ada1011ae750426572`)
was then selected explicitly:

`NYASH_DISABLE_PLUGINS=1 HAKO_BACKEND_COMPILE_RECIPE=pure-first
HAKO_BACKEND_COMPAT_REPLAY=none target/quick/hakorune --emit-exe
/tmp/publication-selection-trivial-runtime.exe --emit-exe-nyrt target/release
/tmp/s0_size_class_box_main.hako`

Compile/link rc0,0.914s; emitted native ELF execution rc0,0.001s, Result:0.
This is a trivial Main artifact pass, not execution of omitted SizeClass
helpers. Actual size_to_bin calls remain selected-header-missing (0.024,
0.022,0.022s); no artifacts. JSON Invoke transport and parent S0 final
caller-zero/deletion acceptance remain outstanding.

This slice closes only the publication-demand mismatch and replacement of
its production issuer. Focused positive/negative and related tests, non-test
build, format/diff and pointer checks PASS. Parent S0 and omission red
classification obligations are carried forward rather than redefined.
