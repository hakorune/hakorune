---
Status: Superseded historical design; no current execution authority
Date: 2026-08-04
Decision: historical separation of language View, transient text, and
  compatibility profiles; superseded for source ownership by the Home model
Current lane: follow `CURRENT_STATE.toml`; do not reopen before the
  MirBuilder final-pipeline checkpoint
Source semantics SSOT: `docs/reference/language/ownership.md`
Execution owner: `hakorune-home-ownership-task-2026-08-04.md`
Related:
  - hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md
  - ownership-view-missing-grammar-inventory-2026-07-28.md
  - ../design/lifecycle-typed-value-language-ssot.md
  - ../design/transient-text-pieces-ssot.md
  - ../design/string-birth-placement-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# Ownership / View / transient representation — parked design and task order

> Superseded on 2026-08-04 for source ownership and task order. Its separation
> of transient text carriers from object-world representation remains useful
> evidence. Current source semantics and execution order live in
> `ownership.md` and `hakorune-home-ownership-task-2026-08-04.md`.

## Executive decision

The historical target was not “make every value a Rust-style borrow”. It used
the following capability algebra; this is evidence, not current source law:

```text
ordinary local/parameter reuse  -> ScopedAlias / noescape alias
owner transfer                  -> move
independent lifetime            -> explicit share
non-owning call result          -> anchored view
```

The first View profile is deliberately narrow: a receiver or formal parameter
WholeObject anchor, same task, no capture, no suspension, no PHI, and no
unknown-effect boundary. It carries no owner and performs no ownership
bookkeeping. `move`, `share`, and `view` remain contextual syntax; they are not
global hard keywords.

This was the semantic target when the card was written. It is no longer the
target in `docs/reference/language/ownership.md`, and the sparse taskboard no
longer owns execution order. Preserve the analysis below only as historical
representation/profile evidence.

## Compatibility law

Compatibility is selected per complete source unit/project profile:

```text
SharedV1 compatibility profile
  preserves current production behavior while sparse rows are inactive

Sparse Ownership profile
  admits the accepted ScopedAlias/move/share/View capabilities only after
  their resolver, Loan Flow, ABI, Builder, runtime, and backend gates close
```

The target compiler chooses exactly one profile for a complete source unit
(with project/manifest selection supplying that unit-wide choice). A Sparse
failure does not retry SharedV1, and a SharedV1 success is not evidence that
Sparse semantics are implemented. The production profile selector is not yet
implemented (`0` current opt-in); the concrete manifest/edition spelling and
source-unit/project selection mechanism remain later grammar decisions. No
current source can opt into Sparse through this card. SharedV1 can retire only
after implicit-share producers, cross-profile bridges, fallback attempts, and
source-unit users reach zero.

Backwards compatibility therefore means:

* old source keeps its current SharedV1 meaning while activation is zero;
* future migrated source selects Sparse at a whole-unit boundary;
* ordinary local code keeps its existing lightweight spelling;
* only ownership-changing boundaries (`move`, `share`, result `view`, stores,
  capture, suspension, and unknown ABI) require source or contract changes;
* unsupported representation/backend/ABI fails before Builder effects.

## Representation boundary: language View is not StringViewBox

The following layers must remain distinct:

```text
language View
  lifetime/anchor capability; no owner; no CopyOwned/DestroyOwned

TextRef / StringSpan / TextPlan / PiecesN
  internal transient read carrier; pass/backend local; not a public Box ABI

StringViewBox
  object-world compatibility value with identity/BoxBase/handle behavior

StringBox / handle
  retained or externally visible representation at an explicit boundary

freeze.str
  canonical text-corridor birth sink for that representation
```

`StringViewBox` is therefore not the implementation of the zero-bookkeeping
language View. Its allocation, identity, base retention, and handle registry
costs are real and remain governed by the string birth/placement SSOTs. The
transient string lane may carry `StringSpan`/`TextPlan`/`PiecesN` without
creating an observable Box. In the selected canonical text corridor,
`freeze.str` is the retained birth sink; this does not claim repository-wide
direct `StringBox::new` callers are retired.
Do not alias `StringViewBox` into a non-owning object or change `BoxBase::new`
semantics as a shortcut.

The performance claim is precise:

```text
ScopedAlias / AnchoredView  -> no additional owner/RC bookkeeping
Unique move                 -> no additional RC operation
explicit share              -> shared-lifetime work may occur
pointer/field/call work     -> may still emit ordinary instructions
```

View is close to necessary for copy-free object-result and slice-heavy code,
but it is not sufficient for C-like performance. The other required controls
are unboxed/trivial values, escape-aware placement, transient text carriers,
birth-at-escape, and specialized backend lowering. No benchmark claim is
authorized by this card; perf decisions use the existing perf SSOT and both
exact-front and whole-program evidence.

The current carrier map is intentionally more detailed:

| Layer | Current meaning | Public source ABI? |
| --- | --- | --- |
| language `view` | verified anchor/lifetime capability | target syntax only; parser inactive |
| `TextRef<'a>` / `StringSpan` | runtime-private borrowed read carrier | no |
| `TextPlan` / `PiecesN` | pass/backend-local transient plan | no |
| `OwnedText` / `OwnedBytes` | runtime-private owned transient | no |
| `KernelTextSlot` | transport/sink adapter | no |
| `StringViewBox` | object-world compatibility Box with identity | existing object API only |
| `StringBox` / handle | stable retained/public representation | boundary ABI |

`BorrowedHandleBox` is a cache/backing carrier, not a language View or
`TextRef`; `TextCell` is a future storage-residence concept. None of these
runtime names may be promoted into a second source ownership authority.

The spelling `view` is also overloaded in existing implementation vocabulary
and must be qualified in every task:

```text
source `: view T`       = Ownership View result capability (parser-inactive)
`CondBlockView`         = analysis-only control observation
`MemoryView`/future span= low-level storage view, separate Stage-1 decision
`StringViewBox`         = object-world substring compatibility value
```

Metadata such as `weak view: Span<PageId>` is field/type text, not source
Ownership View syntax. A name collision never authorizes parser activation or
ownership inference.

## Stage and Rust boundary

The target semantics do not authorize a new Rust-side borrow checker or a
second ownership language implementation. Language meaning and Loan Flow are
future Stage-1/.hako products; Rust MIR/Builder consumes sealed facts and
provides narrow parser/analyzer/backend seams. Rust runtime code may own
representation mechanics (`StringViewBox`, `TextRef`, handles, and sinks), but
must not infer source legality from those mechanics. This keeps the design
compatible with the repository's compiler-expressivity-first and Rust-minimal
policies.

## Authority and non-authority

| Concern | Authority | Must not decide it |
| --- | --- | --- |
| source meaning of alias/move/share/view | `language/ownership.md` | runtime tags, method names, RC counts |
| grammar activation | grammar registry + Rust/Hako parser witnesses | type-name strings or accidental parsing |
| owner/loan facts | resolver + Verified Scoped Loan Flow | Builder maps or runtime identity |
| current BindingRef → ValueId | function-owned Binding SSA | a second ownership value map |
| owner-token create/forward/consume | Ownership SSA / exact ABI | `StringViewBox` or `Arc` shape |
| transient text | `TextPlan` / `PiecesN` boundary docs | public Box identity |
| retained text birth in the selected canonical corridor | `freeze.str` sink | substring helper names, benchmark branches, or unrelated runtime compatibility callers |
| object compatibility | `StringBox` / `StringViewBox` substrate | language View semantics |

## Parked execution train

When `CURRENT_STATE.toml` explicitly reopens the ownership lane, use the
existing parent taskboard order. This card records the clean dependency view:

```text
0  OWNERSHIP-SPARSE-RESUME-D0 readiness after MirBuilder final pipeline
1  OWN-GRAM-REJECT0-HAKO0-S0 -> OWN-GRAM-REJECT0-G0 (Rust/Hako inactive-syntax rejection)
2  O2-P0a/P0r/P0b1/P0c evidence census
3  GRAM-MOVE0 -> GRAM-SHARE0 -> GRAM-PARAM0 -> GRAM-RESULT0
4  O2-A0 -> O2-L0 -> O2-M0 -> O2-DIAG0
5  UBOX-P0 -> UBOX-M0 -> UBOX-I0
6  ALIAS-I0 -> ALIAS-CFG0 (whole-root, no alias PHI/reassignment)
7  ABI0 (noescape receiver/parameter, explicit move/share, Owned/Trivial result)
8  VIEW0 -> PROJ-D0/S0/ABI0/R0/CALL0/DIAG0/I0
9  UCALL-B0 (exact Box receiver-call substrate) -> PROJ-I0
10 SHARE-PLAN0 -> SHARE-I0 and later resource/weak/sync rows
11 OWNERSHIP-SPARSE-PRODUCT-READINESS-D0
```

The first View product consumes an already verified callable ABI and Loan Flow;
it does not make grammar, Unique Box, or Shared behavior appear implicitly.
Text transient optimization is a parallel representation lane, not a hidden
prerequisite for general View correctness. Its own order remains:

```text
TextPlan/PiecesN inventory
-> boundary/placement proof
-> narrow AOT pilot
-> exact-front + whole-program perf evidence
-> widen only on proof
```

## Acceptance gates for future activation

Every ownership/View row must prove, in its own commit:

* whole-source profile selection with zero profile retry/fallback;
* contextual-name compatibility (`move(...)`, `share(...)`, local names, and
  literal type names remain ordinary where specified);
* one source carrier, one Loan Flow authority, one Binding SSA value owner,
  and one exact materialization/ABI owner;
* View anchor/domain and invalidation are verified before effects;
* View/ScopedAlias produce zero ownership opcodes and cannot escape/capture,
  cross await/yield, enter PHI, or cross an unknown ABI in the first profile;
* explicit `share` is the only independent-lifetime acquisition;
* transient text remains non-Box and `freeze.str` remains the only retained
  string birth sink in the selected canonical text corridor (not a
  repository-wide caller census);
* unsupported routes fail fast, with no hidden retain, raw-pointer fallback,
  or method-name inference;
* focused parser/resolver/Loan/SSA/MIR/runtime/backend parity and all touched
  Rust/test files below the 800-line boundary.

## Explicit non-claims

This parked card does not claim:

* parser-live ownership syntax or a fixed manifest/edition spelling;
* production Sparse activation/selector, Ownership SSA callers, or SharedV1 retirement;
* a runtime Box representation for language View;
* that `StringViewBox` is cheap, zero-allocation, or interchangeable with
  `TextRef`/`StringSpan`;
* field/projection/static/temporary View, View PHI, cross-await/thread View,
  plugin/FFI View ABI, or exclusive/noalias mutation;
* `@rune Ownership` metadata as a callable ownership ABI;
* `weak`/`fini` as part of the View capability algebra (`weak` is a
  generation-aware non-owner and `fini` is lifecycle/finalization);
* `CondBlockView`, `MemoryView`/`Span`, `TextRef`, or `StringViewBox` as a
  substitute authority for source ownership or Loan Flow;
* GC/RC strategy, arena promotion, or C-level performance guarantees.

Until the MirBuilder final-pipeline checkpoint reopens the parent taskboard,
all rows in this card remain parked and production activation remains zero.
