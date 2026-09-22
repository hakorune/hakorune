---
Status: selected__fast__2026-09-14
Task: MIR-CALL-R7-STRINGBOX-LOWER-STRUCTURAL-MEMBERSHIP-I0
Date: 2026-09-14
Priority: implement the accepted Hako structural membership contract
Parent: mir-call-r7-stringbox-lower-structural-membership-d0-2026-09-14.md
NextCard: none__select_after_i0_closeout
Implementation permission: true for one Hako lowerer responsibility and its focused evidence
---

# StringBox lower structural membership I0

## Bounded implementation

Implement the accepted D0 in
`lang/src/mir/builder/internal/lower_return_method_string_length_box.hako`.
The lowerer must recognize only the exact `Return -> Method` object and its
owned receiver/argument objects. Every search is delimited by
`JsonCursorBox.seek_obj_end` or `seek_array_end` for the current node.

Selected rows are direct `Str`/`String` literal `length|size` with zero method
arguments, `New(StringBox)` with exactly one direct string constructor argument
and zero-argument `length|size`, and the existing phase17 New-box `indexOf` with
exactly one direct string argument. Empty literals remain valid. All malformed,
wrong-class, extra-argument, embedded, opaque, and unrelated shapes return
`null` before emission.

## Ordered tasks

1. Add bounded readers for the owning Return/Method object, receiver object,
   New constructor array, selector, and method argument array.
2. Replace `_read_first_string_literal_between`,
   `_read_method_first_string_arg`, and the unbounded `k_class`/receiver scans.
3. Keep `_emit_length_mir`, `_emit_size_mir`,
   `_emit_new_stringbox_boxcall0`, and `_emit_new_stringbox_boxcall1_string`
   unchanged as the canonical recipes.
4. Add or update Hako-owned positive/negative tests for direct, New-box,
   empty, indexOf, wrong-class, extra-field, embedded-string, and malformed
   inputs. Include a structural guard that forbids first-string recognition.
5. Update the owning MirBuilder README and current MIR call reference, then
   run the focused Hako path and classify any red before closeout.

## Exact deletion and retention

Delete only the old recognition responsibility: the two first-string helper
methods, the unbounded class/receiver extraction, and equivalent substring
membership branches. Retain the three emitters, registry/fallback callers,
body-only Program(JSON) entry, `parse(&str)`, unrelated lowerers, and general
`indexOf` compatibility.

## Acceptance

```text
direct length/size and New-box length/size preserve their existing MIR output
phase17 exact New-box indexOf remains boxcall-compatible
empty literals are accepted; every listed malformed/unsupported shape returns null
all reads are bounded to their owning JSON object or array
no first-string or unbounded class scan remains
registry/fallback/body-only callers are unchanged
focused Hako positive/negative/structural guards pass
README/reference and CURRENT_STATE are synchronized in closeout
```

No Rust source-artifact transport, caller migration, shared-schema retirement,
backend parity, or whole-R7 completion is claimed by this I0.

## Implementation checkpoint (2026-09-14)

The bounded reader implementation is present in the selected Hako owner. The
old first-string helpers and unbounded class/receiver scans are deleted. The
three existing MIR emitters, registry/fallback callers, and body-only
compatibility entry remain unchanged. The source is 295 lines, below the
800-line hard stop.

Evidence collected without a second Cargo process:

- `CARGO_BUILD_JOBS=1 cargo build --profile quick -j1` passed in 6m03s; the
  crate emitted existing warnings only.
- `bash tools/checks/hako_mirbuilder_stringbox_structural_membership_guard.sh`
  passed (`lines=295`).
- `target/quick/hakorune --dump-ast
  lang/src/mir/builder/internal/lower_return_method_string_length_box.hako`
  returned 0; stderr contained only the environment's missing optional-plugin
  diagnostics.
- The phase14, phase16, and phase17 owner-to-terminal probes were attempted
  with the rebuilt binary. Each stopped before the selected Hako entry at the
  existing `[freeze:contract][raw-loop-child-entry/callable-ledger-missing]`
  planner failure. This is classified as known baseline debt, not a
  current-change failure, and therefore does not prove the dynamic positive
  or negative membership rows.

Closeout remains open until the selected owner can be reached by a focused
runtime probe. The next action is to recover or isolate that pre-owner
planner baseline, then rerun phase14/16/17 plus malformed, wrong-class,
extra-argument, embedded, and empty-literal cases. No source-artifact route,
fallback, retry, or backend gate is opened while this evidence is pending.

The recovery dependency is now design-scoped as
`MIR-CALL-RAW-LOOP-COMPATIBILITY-SCOPE-SELECTION-D0` in
`mir-call-raw-loop-compatibility-scope-selection-d0-2026-09-14.md`. Its
boundary is limited to the compatibility root's scope/ledger choice; it does
not reopen Generic G0 or change this Hako owner. After that D0's I0 lands, the
phase14/16/17 and malformed-shape probes return here for closeout.

## Type-tag boundary correction (2026-09-22)

The bounded `_array_single_direct_string` reader now validates the element
`type` tag as `Str` or `String` before accepting its `value`. Constructor and
`indexOf` argument objects with a present but foreign tag are therefore
rejected before any emitter is called. The structural guard was extended to
pin this check.

An attempted direct positive/foreign-tag probe could not reach this lowerer:
the existing whole-module route stops first at
`[freeze:contract][static-call/legacy-fallback-retired]` for
`ParserStringUtilsBox.starts_with/3`. This reproduces the card's existing
known baseline and is not evidence for or against the new type-tag branch.

## Direct lowerer probe recheck (2026-09-22)

A direct `LowerReturnMethodStringLengthBox.try_lower` probe was attempted with
positive direct/New/empty shapes and negative foreign-tag, wrong-class,
extra-argument, and malformed inputs. The probe did not reach the selected
lowerer: the Hako VM import closure stopped first at the existing
`[freeze:contract][static-call/legacy-fallback-retired]`
`StringScanBox.find_unescaped/3` terminal. This is the same retained static
call boundary, observed one import layer earlier than the phase14/16/17
`ParserStringUtilsBox.starts_with/3` smoke. It is classified as known baseline
debt, not a current-change failure. No runtime membership claim is made from
this probe; the structural guard and source inspection remain the evidence for
the type-tag branch until the selected static-call boundary has an accepted
owner or terminal.

The reported direct-canary environment does not establish a source-backed
admission. The Compatibility import route has no publication owner even when
the Rust parser is used. This differs from the canonical merged-parser
`TargetOnly/RecursiveDependency` boundary tracked in
[recursive String result D0](mir-call-parser-recursive-string-result-authority-d0-2026-09-22.md).
The previous claim that this ruled out all compiler-selection drift was too
strong: a future acceptance must pin the actual binary and source-backed entry.
No dynamic membership result or same-command parent replay is claimed here.
This row retains its pending dynamic evidence; it does not authorize VM repair.

## Recovery result and next baseline (2026-09-14)

The compatibility-root recovery I0 is closed in
`mir-call-raw-loop-compatibility-scope-selection-i0-2026-09-14.md`. Its
constructor branch and three route-level exact tests passed. The rebuilt
`target/quick/hakorune` then crossed the former
`callable-ledger-missing` terminal on all three owner-to-terminal smokes.

The phase14, phase16, and phase17 smokes still stop at the same downstream
baseline before the StringBox owner:

```text
[freeze:contract][static-call/legacy-fallback-retired]
owner=ParserStringUtilsBox method=starts_with arity=3
```

This is classified as a known baseline, not a current-change failure. No
fallback or retry is added to make the StringBox rows appear green. The next
design boundary must identify the existing static-call owner and its exact
caller-local retirement or retained compatibility scope before these smokes
can serve as StringBox owner acceptance. Until that decision is selected, the
Hako membership implementation remains implemented but its dynamic positive
and negative rows are not claimed as closed.
