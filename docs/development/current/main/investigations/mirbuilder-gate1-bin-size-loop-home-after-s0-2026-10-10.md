# Gate1 bin_size Loop Home After S0

Status: CLOSED; source-bound Home After passed, next first-stop Body10
Date: 2026-10-10
Scope: MIRBUILDER-GATE1-BIN-SIZE-LOOP-HOME-AFTER-S0
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/investigations/mirbuilder-gate1-bin-size-loop-recipe-s0-2026-10-10.md
  - src/mir/resolved_semantics/README.md
  - src/mir/loop_recipe_contract/README.md

## Decision

The unchanged `SizeClassBox.bin_size` Body9 Loop already has one retained
source-bound Recipe product, complete two-carrier JoinSig root After, and
current Home prestate. The package issuer may lend an owned, non-Clone
verified After loan from that product. The Home scanner checks exact owner,
site and complete I64 carrier set, then installs both post-loop scalar classes
atomically and crosses Body9. No loan or a typed refusal preserves the current
`PrefixNotCovered(Body9)` boundary without partial class mutation.

The issuer reuses the product's source/frame/scope relation and JoinSig
closure. It must not infer After from Loop syntax, names, or raw Recipe key
numbers. `shift_count` remains a read-only I64 input, not an After carrier.
Unselected Home probes have no authority and remain fail closed.

## Bounded acceptance

- Unchanged source first-stop moves beyond Body9 with `scale` and `i` as I64.
- Existing Body5 malformed-initializer negatives stay fail closed; missing,
  duplicate, foreign or partial After cannot grant Home passage.
- Focused `real_bin_size_` and owner tests, pointer guard, source cap and
  selected formatting pass. Record revision, binary/source provenance and
  first-stop accurately.

Non-claims: Body10 scalar expression, physical Loop/Mul, EXE, old-edge
retirement, CALLEE-RETURN-OUTCOME-S0, and the overall goal remain open.

## Handoff

The Recipe S0 row closed at `1f92ffc172`; its card retains its own evidence.
Read-only review found scanner callback unit return at
`home_new_prefix_scan.rs` and package product retention at
`ordinary_new_coseal_issue.rs`. The narrow loan seam is selected here.

## S0 evidence and closeout

The retained product now lends a move-only Home After only after its complete
root JoinSig After joins uniquely to the Core's two I64 carrier source
relations. The package retains the product or its typed refusal before the
scanner receives the owned loan. The scanner checks the current owner/site
and both I64 prestates, then installs both scalar classes together. No-loan
probe callbacks return `None`; there is no fallback path or partial install.

The unchanged `size_class_box.hako` SHA-256 is
`ac6513ccd595a664bf7bd4a47402baff1377152a93ef2a059566826b13d642b0`.
With quick profile and `CARGO_BUILD_JOBS=4`, the focused
`cargo test --profile quick -p nyash-rust --lib real_bin_size_ --quiet`
passed 6/6. The same binary
`nyash_rust-6da50fb535f1fa27` (SHA-256
`914b52fee9efde99cb6845c95280e0711be1e880f79aacc7112b89197704d1fd`)
passed `mir::loop_recipe_contract::` 227/227 and `home_new_prefix` 14/14.
The original source's first uncovered exit prefix moved from Body9 to Body10;
four malformed initializer variants still stop at Body5. The product family
also checks wrong bound class, operation, literal/order, and extra effect;
existing JoinSig tests reject missing/duplicate After. Compilation had two
intermediate callback-type errors from unconverted forwarding/default sites;
both were corrected before the passing run. Existing compiler warnings remain
informational, with no unclassified red.

Pointer guard, diff check, selected changed-file formatting check and the
800-line source cap pass. `mod.rs` and `ordinary_new_coseal_issue.rs` retain
their pre-existing whole-file formatting; unrelated rustfmt movement was
removed. The next bounded source frontier is Body10 `local words = (5 + top)
* scale`, requiring its own scalar-expression and physical Loop decisions.
This row neither authorizes physical execution nor closes the open
CALLEE-RETURN-OUTCOME-S0 obligations.
