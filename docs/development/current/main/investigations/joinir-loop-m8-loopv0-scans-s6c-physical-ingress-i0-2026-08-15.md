---
Status: current bounded implementation row
Date: 2026-08-15
Work mode: fast
Classification: T2-approved BoxShape; caller-zero
---

# JOINIR-LOOP-M8-LOOPV0-SCANS-S6C-PHYSICAL-INGRESS-I0

## Six-line brief

```text
Decision: implement exactly one non-Clone VerifiedS6CPrephysicalIngressV2 which consumes the sealed logical output once and remains Builder-free.
Source authority + canonical issuer: the retained S6C Facts/calls/Completion, fixed Recipe role map, and V2 Join transfer; issue_s6c_prephysical_ingress_v2 is the sole co-seal issuer.
Non-authority: Consumed, V1 effect/demand/continuation, a new Pure taxonomy, raw Facts/Recipe/JoinSig, item order, AST/name/MIR, generic CompareEq, physical IDs, selector, fallback, retry.
Fail-fast boundary: context, 15 placements, 13 role-specific source/execution rows, If, Loop Exit, calls, inputs/carrier, V2 After, and exact-two exits reject before aggregate publication.
Smallest next slice: one owner module, one separate focused test module, narrow output borrow seam, module README/reference receipt, and existing family guards only.
Non-claims: no TextEq physical target, ReadyEntry, host/session, Builder/MIR/CFG/SSA/PHI/layout, Return emission, Artifact/ABI, selector, production caller, fallback, retry, or retirement.
```

## Exact change surface

```text
new:
  src/mir/loop_recipe_contract/s6c_prephysical_ingress.rs
  src/mir/loop_recipe_contract/s6c_prephysical_ingress_tests.rs

touched narrowly:
  src/mir/loop_recipe_contract/s6c_scan_with_init_joinir_output.rs
  src/mir/loop_recipe_contract/mod.rs
  src/mir/loop_recipe_contract/README.md
  docs/reference/mir/loop-recipe-contract.md
```

Do not touch `typed_schema_v2.rs`, the Builder, physicalizer, selectors, runtime,
backend, or fixtures unless a focused negative proves the accepted mapping is
not expressible. If that happens, return to the D0 stop line instead of widening
the row.

Targets:

```text
owner source < 650 lines; mandatory design split before 700
test source  < 300 lines target; split before 760
all touched Rust sources < 760; 800 hard stop
new top-level guard = 0
```

## Product and issuer

```rust
#[derive(Debug)]
pub(crate) struct VerifiedS6CPrephysicalIngressV2 {
    output: VerifiedS6CScanWithInitLogicalOutputV1,
    context: VerifiedLoopSemanticContextV1,
    seal: S6CPrephysicalIngressSealV2,
}

pub(crate) fn issue_s6c_prephysical_ingress_v2(
    output: VerifiedS6CScanWithInitLogicalOutputV1,
) -> Result<VerifiedS6CPrephysicalIngressV2, S6CPrephysicalIngressRejectV2>;
```

The issuer must:

```text
1. borrow the retained output through a new narrow private prephysical seam
2. issue context from that same Facts membership only
3. validate exact domains and 15 = 13 + If + Exit
4. co-seal each fixed role with its exact source relation and V2 execution class
5. preserve role-specific source-anchor multiplicity, including I3 = 2
6. co-check Length/Substring source contract, Home/ABI/PureRead, and Recipe row
7. co-check If/Loop Exit and V2 branch/Return-summary/Backedge/After
8. co-check Loop Return and callable Tail against the same Completion/empty cleanup
9. store only private parity/transport evidence, then move output into the product
```

No caller may supply context, source rows, effects, continuation, or Completion
as a second argument.

## Borrow façade

The product exposes one private HRTB callback. It may lend:

```text
exact semantic context
Subject/Needle/Index and initializer/carrier view
logical placement rows
13 role-specific operation-source/execution views
role-wise Length/Substring physical-call prerequisite views
If and Loop Exit control view
borrowed V2 logical transfer and After
distinct Loop Return and callable Tail Completion views
```

It must not lend raw Recipe, JoinSig, Facts, Completion, the owned logical
output, or any physical capability. `Clone`, `into_parts`, `take_*`, and raw
getters are absent.

## Authority split

```text
Facts BodyEffect:
  Call = 2, Write = 1

CoreMethod contracts:
  Length/Substring Home + ABI + PureRead

Recipe operation-family diagnostic census:
  Read 4, Write 1, Call 2, expression operation 6

V2 execution authority:
  NonFaulting 11, FaultBeforeNormalResult 0, ExternallyBoundOutcome 2
```

No new enum may relabel the diagnostic `4/1/2/6` split as semantic effects.
TextEq is carried as exact source + typed logical operation + NonFaulting
execution evidence only; this I0 does not choose its physical target.

## Focused acceptance

Positive tests:

```text
canonical fixture issues one aggregate
exact domain and 15/13/1/1 census
all fixed roles map to the expected logical item/value/block
I3 has two distinct source anchors bound to one Recipe read
Length/Substring source/Recipe/Home/ABI/PureRead parity
TextEq Text/Text/Bool source and Body placement parity
one branch, Return summary, Backedge, and L0/B0/I64 After
Loop Return(index) and callable Tail(-1) stay distinct
HRTB view cannot escape and product is non-Clone/non-splittable
```

Negative tests must directly mutate or inject private drafts for:

```text
operation missing/duplicate/swap/foreign placement
If or Exit included in the operation census
I3 one-anchor collapse or duplicate anchor
call receiver/argument/result/role drift
TextEq operand/result/source-placement drift
owner/frame/scope mismatch
branch/summary/backedge/After drift
Tail imported as Loop Exit
nonempty/foreign Completion
raw constituent or V1 coercion API attempt
```

## Gates

Run only the focused and reusable family evidence required by this row:

```text
cargo test --lib s6c_prephysical_ingress -- --nocapture
cargo test --lib s6c_scan_with_init -- --nocapture
cargo check --lib
cargo fmt --all -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/joinir_loop_common_physicalizer_pre_cutover_guard.sh
git diff --check
```

Do not add a top-level guard. If an existing Loop authority guard has a narrow
inventory section for product API shape, update that section only when needed.

## Done / stop

Done only when:

```text
one aggregate issuer and one caller-zero product exist
all positive/negative focused evidence is green
source anchors and all authority categories remain distinct
owner README/reference record the implementation receipt
no Builder/MIR/physical/production edge is introduced
worktree is committed and pushed
```

Stop and return to design when implementation needs:

```text
AST/MIR/name/order rewalk
V1 operation-effect/continuation/demand coercion
Facts/Recipe/Join/Completion re-pairing
an invented Pure/effect/Home/ABI meaning
generic CompareEq as TextEq authority
raw constituent escape
host/session, ReadyEntry, ValueId, BasicBlockId, or physical ID
Option fallback, retry, selector, or production caller
```

After I0, retarget to `S6C-TEXT-EQ-PHYSICAL-CONTRACT-D0`. The physical session
remains closed until that owner and the single session design are accepted.
