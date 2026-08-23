# Normal root execution source P0

Status: selected Fast slice
Date: 2026-08-23
Decision: NORMAL-ROOT-EXECUTION-SOURCE-P0
Owner: parser callable source -> exact final callable transform

## Six-line brief

Decision:
  Add only the parser-owned total App/ProgramRuntime execution relation and
  move it through the exact final transform. This is one BoxCount.
Source authority + canonical issuer:
  Exact initial callable Program + normal Program authority + complete
  parameter catalog; ParserNormalRootExecutionIssuerV1::issue_once, caller 1.
Non-authority:
  Narrow App/Script-A products, raw expansion, AST pointer, name/ordinal alone,
  Builder bool/state, work plan, Recipe, MIR, and compatibility AST.
Fail-fast boundary:
  Issue in ParsedProgramWithCallableParameterSourceV1::new; reject transform
  drift before VerifiedFinalCallableProgramSourceV1 is created.
Smallest next slice:
  Parser model/issuer, paired private loan, required move chain, preservation,
  focused tests, parser README receipt, and one reusable lane guard.
Non-claims:
  No Builder consumer, lifecycle cutover, catalog/work-plan rewrite, fixture,
  fallback, compatibility change, raw retirement, Recipe, MIR, or publication.

## Change

Add:

~~~text
callable_parameter_source/normal_root_execution/
  model.rs
  issuer.rs
  tests.rs

normal_callable_program_source/
  normal_root_execution_preservation.rs
~~~

Touch existing parser owners only to attach required fields, expose one private
paired issuer loan, and move the same disposition through the exact transform.
Keep each production source below 760 lines; split before 760 and hard-stop at
800.

## Contract

- `Ready` is exactly `App` or `ProgramRuntime`; there is no Outside/default.
- Program statements own nested callable relations; independent arrays cannot
  be zipped by a caller.
- App has exactly one RootMain, all Main helpers, all ordinary siblings, and
  compatibility-name child execution order.
- ProgramRuntime has source-backed no-static-Main evidence and all ordinary
  statement/callable rows.
- Opaque parser identity is primary. Final slot/site and execution ordinal are
  coverage/schedule evidence, never standalone pairing keys.
- The product is AST-free, non-Clone, constructor-private, and a required field.
- Compatibility receives no empty or fabricated execution product.
- Final preservation moves the same product after exact Program/callable checks;
  transformed AST cannot reissue it.
- Existing narrow Main.main/0 and Script-A products remain independent and do
  not select total execution.

## Done

Focused positive evidence:

~~~text
ProgramRuntime: no Main, non-Main provider, non-static Main
App: main/0, main/N, multiple helpers, mixed top-level siblings
helper source order != compatibility-name execution order
one static Main + one non-static Main
exact unchanged final transform
~~~

Focused negative/terminal evidence:

~~~text
duplicate static Main
missing main
source-backed reachable non-static main/helper
foreign parser invocation
missing/duplicate/contradictory callable identity/slot/site
statement/callable coverage drift
final transform statement/declaration/slot drift
compatibility-only malformed AST receives no Ready product
~~~

Structural guard:

~~~text
total issuer definition/caller                         = 1 / 1
product Clone / public constructor                    = 0 / 0
ASTNode, AST ref, Builder/Recipe/MIR identifiers       = 0
required execution disposition represented as Option = 0
narrow App/Script-A used as execution selector        = 0
transformed-AST reissuer                               = 0
Builder production consumer                           = 0
compatibility Ready fabrication                       = 0
~~~

Parser focused tests, `cargo check`, the reusable guard, current-state pointer
guard, and `git diff --check` must be green.

## Stop

- Do not add a Builder getter or consumer in P0.
- Do not edit `VerifiedRawRootExpansionV1`, work-plan APIs, or `root_is_app_mode`.
- Do not change helper order or App/ProgramRuntime language behavior.
- If the initial callable owner cannot lend statement + identity + final slot
  as one paired row, return to NoSafeSlice; do not zip raw slices.
- If a Ready product cannot move through the exact transform without re-scan or
  reissue, return to NoSafeSlice.
- P0 may remain caller-zero only for this one commit. Its mandatory immediate
  successor is NORMAL-ROOT-EXECUTION-CONSUMER-S0; otherwise revert P0.
