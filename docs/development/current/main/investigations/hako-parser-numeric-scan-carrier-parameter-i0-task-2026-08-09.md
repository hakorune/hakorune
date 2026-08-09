---
Status: retired as an acceptance repair — executable probe is preempted by an earlier dependency Loop
Date: 2026-08-09
Row: `HAKO-PARSER-NUMERIC-SCAN-CARRIER-PARAMETER-I0`
Parent: `HAKO-PARSER-NUMERIC-SCAN-CARRIER-SOURCE-D0`
Mode: BoxCount / one exact source signature fact
---

# HAKO-PARSER-NUMERIC-SCAN-CARRIER-PARAMETER-I0

## Goal

Declare the existing numeric scanner cursor input as exact `i64` and prove
that the existing header, parameter, local-copy, and GenericLoop owners carry
that fact without inference or fallback.

## Exact change

```hako
scan_int(src, i)
  -> scan_int(src, i: i64)
```

`src` remains untyped because the compatibility scanner admits `null`. No
other scanner or callable signature changes in this row.

## Required owner chain

```text
source ParamDecl(i: i64)
  -> existing declared-signature projection
  -> existing setup_function_params identity/type commit
  -> Variable(i)
  -> existing local j Copy + metadata propagation
  -> variable_map[j]
  -> existing GenericLoop exact Integer carrier preparation
```

No new public type, receipt, parameter publisher, local publisher, or Loop
rule is required. If this chain does not execute as written, stop and return
to design; do not add a second publication path.

## Acceptance

```text
clean direct scan_int("42}", 0) fixture exits successfully
null source compatibility remains byte-for-byte unchanged
ordinary integer/float/suffix compatibility rows remain unchanged
the emitted scanner function has exact Integer for formal i and local j
GenericLoop MissingTransientType/UnknownTransientType fail-fast unchanged
no source rewrite, name special-case, inference, retry, or fallback
existing all-I64 FreeStatic authority unchanged
```

## Verification

Add one focused fixture and guard for the existing `scan_int` API, register
the guard in `docs/tools/check-scripts-index.md`, and run:

```bash
bash tools/checks/hako_parser_numeric_scan_carrier_parameter_i0_guard.sh
bash tools/checks/hako_parser_rich_body_h2_s2_r0_guard.sh
bash tools/checks/hako_parser_box_declaration_h1_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/naming_charter_guard.sh
```

## Nonclaims

```text
numeric lexical-parts product
typed expression/Return/body product
ordinary method connection or H3 seal
Take/share/release or Home semantics
general static-callable signature redesign
GenericLoop acceptance expansion
resolver target, Recipe, DraftSeal, publication, runtime
```

## Closeout

Implementation, focused fixture/guard, check index, owner README receipt,
current pointers, commit, and push close together. When this canary is green,
resume the existing stashed `HAKO-PARSER-RICH-BODY-RESULT-H2-S2-S0` work.

## Failed probe receipt

The exact one-line source change and focused direct-call canary were attempted
without adding any new type/Loop owner. The executable still froze at:

```text
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(3) }
```

Import bisection then proved that `sh_core` alone reproduces the same failure.
Therefore this probe never established whether `scan_int`'s `i: i64` reaches
its parameter publisher; it is preempted by an earlier dependency Loop. Do
not infer a parameter-carriage loss from this result, and do not add a local
override or GenericLoop fallback.

The failed slice is preserved as:

```text
stash@{0}: wip/numeric-scan-carrier-parameter-i0
           (source i64 not carried to GenericLoop)
```

Do not resume this source-annotation row merely because the dependency canary
later reaches `scan_int`. Under the compiler-expressivity-first policy, a
valid source shape that lacks an internal semantic/type edge is repaired in
the compiler. Reopen a source signature Decision only if the language
contract itself requires an explicit declaration, not as a backend or
GenericLoop acceptance workaround.
