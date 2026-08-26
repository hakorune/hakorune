#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="mir-call-ingress-schema-lifecycle-guard"
CARD="$ROOT_DIR/docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml"
STATE="$ROOT_DIR/docs/development/current/main/CURRENT_STATE.toml"
WORKSTREAM="$ROOT_DIR/docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md"
MANIFEST="$ROOT_DIR/tools/checks/guard_rows.toml"
INDEX="$ROOT_DIR/docs/tools/check-scripts-index.md"

fail() {
  echo "[$TAG] result_class=current-change failure status=fail: $*" >&2
  exit 1
}

[[ $# -le 1 ]] || fail "usage: $0 [reference_child_i0|canonical_v1_value_s0|core_direct_substring_product_aot_s0|core_direct_retire_r0|wpre_readiness]"
PHASE="${1:-wpre_readiness}"
case "$PHASE" in
  wpre_readiness|reference_child_i0|canonical_v1_value_s0|core_direct_substring_product_aot_s0|core_direct_retire_r0) ;;
  wpre_i0|typed_global_b1|r7_closeout)
    fail "recognized future phase is not landed: $PHASE"
    ;;
  *)
    fail "unknown phase: $PHASE"
    ;;
esac

for file in "$CARD" "$STATE" "$WORKSTREAM" "$MANIFEST" "$INDEX"; do
  [[ -f "$file" ]] || fail "required owner missing: ${file#$ROOT_DIR/}"
done

python3 - "$ROOT_DIR" "$CARD" "$STATE" "$WORKSTREAM" "$MANIFEST" "$INDEX" "$PHASE" <<'PY'
from pathlib import Path
import sys
import tomllib

root, card_path, state_path, workstream_path, manifest_path, index_path = map(
    Path, sys.argv[1:7]
)
phase = sys.argv[7]


def fail(message: str) -> None:
    raise SystemExit(message)


with card_path.open("rb") as stream:
    card = tomllib.load(stream)
with state_path.open("rb") as stream:
    state = tomllib.load(stream)
with manifest_path.open("rb") as stream:
    manifest = tomllib.load(stream)
workstream = workstream_path.read_text(encoding="utf-8")
index = index_path.read_text(encoding="utf-8")

guard_id = "mir-call-ingress-schema-lifecycle-guard"
guard_script = "tools/checks/mir_call_ingress_schema_lifecycle_guard.sh"
execution_row = "MIR-CALL-INGRESS-SCHEMA-LIFECYCLE-GUARD-S0"

rows = manifest.get("rows")
if not isinstance(rows, list):
    fail("guard_rows.toml rows table is missing")
matches = [row for row in rows if isinstance(row, dict) and row.get("id") == guard_id]
if len(matches) != 1:
    fail(f"expected one registry row for {guard_id}, found {len(matches)}")
row = matches[0]
if row.get("profiles") != ["pilot", "quick-static"]:
    fail("ingress lifecycle guard profiles drifted")
if row.get("cmd") != ["bash", guard_script]:
    fail("ingress lifecycle guard command drifted")
if sum(1 for item in rows if isinstance(item, dict) and item.get("id") == guard_id) != 1:
    fail("ingress lifecycle guard id is duplicated")

if state.get("work_mode") not in {"fast", "design_stop", "closeout"}:
    fail("ingress lifecycle row requires CURRENT_STATE work_mode=fast, design_stop, or closeout")
if state.get("latest_card_path") != str(card_path.relative_to(root)):
    fail("CURRENT_STATE latest_card_path no longer names the active card")

if phase == "core_direct_substring_product_aot_s0":
    expected_row = "CORE-DIRECT-SUBSTRING-PRODUCT-AOT-S0"
    current_row = state.get("current_execution_row")
    if state.get("work_mode") not in {"fast", "design_stop", "closeout"}:
        fail("core_direct_substring_product_aot_s0 requires fast, design_stop, or closeout work mode")
    if current_row not in {expected_row, "CORE-DIRECT-RETIRE-R0"}:
        fail("core_direct_substring_product_aot_s0 pointer drifted")
    if current_row == expected_row and state.get("next_execution_card") != expected_row:
        fail("core_direct_substring_product_aot_s0 open next execution card drifted")
    if current_row == "CORE-DIRECT-RETIRE-R0" and not str(state.get("next_execution_card", "")).startswith("none"):
        fail("landed ProductAot S0 requires a design-stop next execution card")
    if card.get("implementation_permission") is not False:
        fail("active card top-level permission must remain false; use scoped S0 permission")
    smoke_rows = card.get("core_direct_smoke_disposition")
    if not isinstance(smoke_rows, dict):
        fail("core_direct_smoke_disposition is missing")
    s0 = smoke_rows.get("product_aot_s0")
    if not isinstance(s0, dict):
        fail("core_direct product_aot_s0 section is missing")
    status = s0.get("status")
    if status not in {"fast_open", "landed"}:
        fail("core_direct product_aot_s0 status is not fast_open or landed")
    if status == "fast_open" and current_row != expected_row:
        fail("fast-open ProductAot S0 must remain the current execution row")
    if status == "landed" and current_row not in {expected_row, "CORE-DIRECT-RETIRE-R0"}:
        fail("landed ProductAot S0 is outside its closure boundary")
    if status == "fast_open" and s0.get("implementation_permission") is not True:
        fail("core_direct product_aot_s0 fast-open permission is not set")
    if status == "landed" and s0.get("implementation_permission") is not False:
        fail("landed core_direct product_aot_s0 must close scoped permission")
    expected_files = {
        "apps/tests/string_substring_in_range_min.hako",
        "tools/smokes/v2/profiles/integration/apps/string_substring_in_range_exe.sh",
        guard_script,
    }
    if set(s0.get("allowed_files") or []) != expected_files:
        fail("core_direct product_aot_s0 allowed file boundary drifted")
    if status == "landed":
        source_path = root / "apps/tests/string_substring_in_range_min.hako"
        smoke_path = root / "tools/smokes/v2/profiles/integration/apps/string_substring_in_range_exe.sh"
        if not source_path.is_file() or not smoke_path.is_file():
            fail("landed core_direct product_aot_s0 owner is missing")
        source = source_path.read_text(encoding="utf-8")
        smoke = smoke_path.read_text(encoding="utf-8")
        if 's.substring(2, 5)' not in source:
            fail("ProductAot source no longer pins substring(2,5)")
        required_markers = [
            "emit_mir_route.sh --route direct",
            "pure_first_route_preflight.py",
            "compat_replay=none",
            "NYASH_NYRT_SILENT_RESULT=1",
            "cmp",
            "cde",
        ]
        missing = [marker for marker in required_markers if marker not in smoke]
        if missing:
            fail("ProductAot smoke is missing markers: " + ", ".join(missing))
        for forbidden in ("2>&1", "filter_noise", "tail -n1", "|| true"):
            if forbidden in smoke:
                fail(f"ProductAot smoke uses forbidden evidence shortcut: {forbidden}")
    print(
        f"[{guard_id}] result_class=current-change failure status=pass "
        f"phase={phase} status={status} exact_aot_successor={'landed' if status == 'landed' else 'open'} "
        "vm_successor=none deletion=forbidden"
    )
    raise SystemExit(0)

if phase == "core_direct_retire_r0":
    expected_row = "CORE-DIRECT-RETIRE-R0"
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout", "design_stop"}:
        fail("core_direct_retire_r0 requires fast, closeout, or design_stop work mode")
    current_row = state.get("current_execution_row")
    successor_row = "MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-FORCE-HV1-FATE"
    landed_closeout = current_row == successor_row and card.get("status") == "core_direct_r0_landed"
    if current_row != expected_row and not landed_closeout:
        fail("core_direct_retire_r0 pointer drifted")
    if mode == "design_stop" and not landed_closeout:
        if not str(state.get("next_execution_card", "")).startswith("none"):
            fail("core_direct_retire_r0 next execution card must remain none during design stop")
        if not state.get("current_design_stop"):
            fail("core_direct_retire_r0 must retain an explicit design stop")
    elif not landed_closeout:
        if state.get("next_execution_card") != expected_row:
            fail("core_direct_retire_r0 fast/closeout pointer drifted")
        if mode == "fast" and state.get("current_design_stop"):
            fail("core_direct_retire_r0 fast row must not retain a design stop")
    smoke_rows = card.get("core_direct_smoke_disposition")
    if not isinstance(smoke_rows, dict):
        fail("core_direct_smoke_disposition is missing")
    s0 = smoke_rows.get("product_aot_s0")
    if not isinstance(s0, dict) or s0.get("status") != "landed":
        fail("ProductAot S0 must be landed before CoreDirect R0")
    if s0.get("implementation_permission") is not False:
        fail("landed ProductAot S0 must have closed scoped permission")
    if mode == "design_stop" and not landed_closeout:
        if card.get("implementation_permission") is not False:
            fail("active card top-level permission must remain false during R0 design stop")
    else:
        expected_card_status = "core_direct_r0_fast_open" if mode == "fast" and not landed_closeout else "core_direct_r0_landed"
        if card.get("status") != expected_card_status:
            fail(f"active card status must be {expected_card_status} for R0")
        expected_permission = mode == "fast" and not landed_closeout
        if card.get("implementation_permission") is not expected_permission:
            fail("active card implementation permission does not match R0 mode")
        allowed_files = set(card.get("core_direct_tag_rc_contract", {}).get("implementation_allowed_files") or [])
        expected_files = {
            "src/runner/core_executor.rs",
            "tools/smokes/v2/profiles/integration/core_direct/core_direct_retire_r0.sh",
            "tools/smokes/v2/profiles/integration/core_direct/core_direct_string_substring_ok_vm.sh",
            "tools/smokes/v2/profiles/integration/core_direct/core_direct_string_bounds_rc_vm.sh",
            "tools/smokes/v2/profiles/integration/core_direct/core_direct_string_charat_bounds_rc_vm.sh",
            "tools/smokes/v2/profiles/integration/core_direct/core_direct_map_bad_key_rc_vm.sh",
            "tools/smokes/v2/profiles/integration/core_direct/core_direct_string_replace_ok_vm.sh",
            "tools/smokes/v2/profiles/integration/core_direct/core_direct_array_oob_set_rc_vm.sh",
            guard_script,
            "tools/selfhost/README.md",
            "docs/development/current/main/CURRENT_STATE.toml",
            "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
            "docs/development/current/main/design/vm-active-lane-retirement-ssot.md",
            "docs/development/current/main/design/mir-canonical-callsite-lane-ssot.md",
            "docs/development/current/main/design/mirbuilder-final-pipeline-ssot.md",
            "docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml",
        }
        if allowed_files != expected_files:
            fail("CoreDirect R0 implementation file boundary drifted")
    terminal = card.get("core_direct_tag_rc_contract")
    if not isinstance(terminal, dict):
        fail("core_direct_tag_rc_contract is missing")
    expected_terminal_status = (
        "design_accepted_one_state_pre_wpre"
        if mode == "design_stop" and not landed_closeout
        else "fast_open_one_state_pre_wpre"
        if mode == "fast" and not landed_closeout
        else "landed_one_state_pre_wpre"
    )
    if terminal.get("status") != expected_terminal_status:
        fail("CoreDirect R0 terminal contract is not the accepted one-state design")
    issuer = str(terminal.get("canonical_issuer", ""))
    if "one terminal" not in issuer or "[core-direct/retired]" not in issuer:
        fail("CoreDirect R0 one-state retired terminal is not recorded")
    if "unavailable remains ParkedSealed" not in issuer:
        fail("CoreDirect R0 unavailable parking rule is missing")
    if mode != "design_stop" or landed_closeout:
        def read(relative: str) -> str:
            path = root / relative
            if not path.is_file():
                fail(f"CoreDirect R0 implementation owner missing: {relative}")
            return path.read_text(encoding="utf-8")

        core = read("src/runner/core_executor.rs")
        smoke = read("tools/smokes/v2/profiles/integration/core_direct/core_direct_retire_r0.sh")
        required_core = (
            "fn core_direct_requested()",
            "fn core_direct_retired()",
            "core_direct_is_one_state_post_decode_terminal",
            "core_direct_does_not_relabel_wrong_entrance",
        )
        for marker in required_core:
            if marker not in core:
                fail(f"CoreDirect R0 core owner is missing marker: {marker}")
        forbidden_core = (
            "maybe_try_core_direct_for_mir_json",
            "looks_like_mir_json_text",
            "try_run_core_direct",
            "HAKO_CORE_DIRECT_INPROC",
            "NYASH_CORE_DIRECT_INPROC",
            "falling back to VM interpreter",
            "core_exec_direct.hako",
            "apply_core_wrapper_env",
        )
        for marker in forbidden_core:
            if marker in core:
                fail(f"CoreDirect R0 old route marker remains: {marker}")
        for marker in ("core_direct_retire_r0", "core-direct/retired", "wrong-entrance", "core_exec_direct.hako", "cmp"):
            if marker not in smoke:
                fail(f"CoreDirect R0 smoke is missing marker: {marker}")
        for old_script in (
            "core_direct_string_substring_ok_vm.sh",
            "core_direct_string_bounds_rc_vm.sh",
            "core_direct_string_charat_bounds_rc_vm.sh",
            "core_direct_map_bad_key_rc_vm.sh",
            "core_direct_string_replace_ok_vm.sh",
            "core_direct_array_oob_set_rc_vm.sh",
        ):
            if (root / "tools/smokes/v2/profiles/integration/core_direct" / old_script).exists():
                fail(f"retired CoreDirect smoke remains: {old_script}")
        if "HAKO_CORE_DIRECT_INPROC" in read("tools/selfhost/README.md"):
            fail("selfhost README still advertises the retired in-process route")
    if mode == "design_stop" and not landed_closeout:
        print(
            f"[{guard_id}] result_class=current-change failure status=pass "
            "phase=core_direct_retire_r0 s0=landed terminal=one_state_pre_wpre "
            "unavailable=parked deletion=forbidden"
        )
    else:
        print(
            f"[{guard_id}] result_class=current-change failure status=pass "
            "phase=core_direct_retire_r0 s0=landed terminal=one_state_pre_wpre "
            f"unavailable=parked implementation={'landed' if landed_closeout else 'fast_open'}"
        )
    raise SystemExit(0)

if phase in {"reference_child_i0", "canonical_v1_value_s0"}:
    if state.get("work_mode") not in {"fast", "closeout", "design_stop"}:
        fail(f"{phase} requires CURRENT_STATE work_mode=fast, closeout, or design_stop")
else:
    if state.get("implementation_permission") is True:
        fail("CURRENT_STATE must not expose semantic implementation permission during guard-only phase")

guard_row_key = "canonical_v1_value_guard_row" if phase == "canonical_v1_value_s0" else "ingress_schema_guard_row"
guard_row = card.get(guard_row_key)
if not isinstance(guard_row, dict):
    fail(f"active card {guard_row_key} is missing")
if guard_row.get("execution_row") != execution_row:
    fail("active card guard execution row drifted")
if guard_row.get("phase") != phase:
    fail(f"active card phase is not {phase}")
if guard_row.get("status") not in {"selected_fast_guard_only", "landed_guard_only"}:
    fail("active card guard-only status drifted")
if card.get("implementation_permission") is not False:
    fail("active card top-level implementation permission must remain false")
allowed_files = set(guard_row.get("allowed_files") or [])
expected_files = {
    guard_script,
    "tools/checks/guard_rows.toml",
    "docs/development/current/main/investigations/mir-call-d1b-root-lineage-exact-target-loan-d0-2026-08-26.toml",
    "docs/development/current/main/CURRENT_STATE.toml",
    "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    "docs/tools/check-scripts-index.md",
}
if allowed_files != expected_files:
    fail("guard-only allowed file boundary drifted")
if guard_row.get("change", "").find("reusable fail-closed guard") < 0:
    fail("active card no longer describes the reusable fail-closed guard")
if "No parser" not in guard_row.get("contract", ""):
    fail("active card guard contract permits parser changes")

proof_plan = card.get("proof_plan")
if not isinstance(proof_plan, dict) or guard_id not in str(proof_plan.get("future_guard_rows", "")):
    fail("active card proof plan does not name the ingress guard")
if guard_script not in index:
    fail("check-script index does not list the ingress lifecycle guard")
if guard_id not in workstream:
    fail("active workstream does not name the ingress guard row")

if phase == "reference_child_i0":
    card_status = card.get("status")
    if card_status not in {"reference_child_i0_fast_open", "reference_child_i0_landed"}:
        fail("active card is not marked reference-child I0 open or landed")
    scope = str(card.get("permission_scope", ""))
    if "reference-child private transport I0" not in scope or "all Wpre" not in scope:
        fail("reference-child permission scope is not narrow and explicit")

    reference = card.get("reference_child_reentry")
    if not isinstance(reference, dict):
        fail("active card reference_child_reentry section is missing")
    if card_status == "reference_child_i0_fast_open":
        if reference.get("implementation_permission") is not True:
            fail("reference_child_i0 fast-open permission is not set")
        if state.get("work_mode") != "fast" or state.get("current_execution_row") != "MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-REFERENCE-CHILD-PRIVATE-TRANSPORT-I0":
            fail("reference_child_i0 fast-open pointer drifted")
    else:
        if reference.get("implementation_permission") is not False:
            fail("reference_child_i0 landed row must close scoped implementation permission")
    required_env_remove = {
        "NYASH_VERIFY_JSON",
        "HAKO_VERIFY_PRIMARY",
        "HAKO_ROUTE_HAKOVM",
        "HAKO_VERIFY_V1_FORCE_HAKOVM",
        "NYASH_USE_STAGE1_CLI",
        "HAKO_STAGE1_ENABLE",
        "HAKO_EMIT_PROGRAM_JSON",
        "HAKO_EMIT_MIR_JSON",
        "NYASH_STAGE1_CLI_CHILD",
        "HAKO_PROGRAM_JSON",
        "HAKO_PROGRAM_JSON_FILE",
        "HAKO_STAGE1_PROGRAM_JSON",
        "NYASH_STAGE1_PROGRAM_JSON",
        "NYASH_STAGE1_MODE",
        "HAKO_STAGE1_MODE",
        "NYASH_STAGE1_INPUT",
        "HAKO_STAGE1_INPUT",
        "STAGE1_INPUT",
        "NYASH_STAGE1_BACKEND",
        "HAKO_STAGE1_BACKEND",
        "STAGE1_BACKEND",
        "NYASH_EMIT_MIR_TRACE",
        "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON",
        "HAKO_VM_HAKO_DRIVER_PAYLOAD_FILE",
    }
    card_env_remove = set(reference.get("child_env_remove") or [])
    if not required_env_remove.issubset(card_env_remove):
        fail("active card child_env_remove is missing a known non-authority alias")
    required_pins = {
        "NYASH_SKIP_TOML_ENV=1",
        "NYASH_VM_USE_FALLBACK=0",
        "NYASH_VM_HAKO_PREFER_STRICT_DEV=0",
        "NYASH_USE_NY_COMPILER=0",
        "--backend vm",
    }
    if not required_pins.issubset(set(reference.get("child_env_pin") or [])):
        fail("active card child_env_pin is incomplete")
    if reference.get("transport_cardinality", "").find("Exactly one private key") < 0:
        fail("private transport cardinality is not fixed")

    allowed_implementation_files = {
        "src/runner/reference/vm_hako/driver_spawn.rs",
        "src/runner/reference/vm_hako/driver_main.hako",
        "tools/smokes/v2/profiles/integration/apps/lib/vm_hako_json_parity_common.sh",
        "tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_non_future_reject_vm.sh",
        "tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_newclosure_probe_vm.sh",
        "tools/smokes/v2/profiles/integration/vm_hako_caps/compare/compare_ported_vm.sh",
        guard_script,
    }
    if set(reference.get("implementation_allowed_files") or []) != allowed_implementation_files:
        fail("reference-child implementation file boundary drifted")

    def read(relative: str) -> str:
        path = root / relative
        if not path.is_file():
            fail(f"reference-child implementation owner missing: {relative}")
        return path.read_text(encoding="utf-8")

    spawn = read("src/runner/reference/vm_hako/driver_spawn.rs")
    driver = read("src/runner/reference/vm_hako/driver_main.hako")
    parity = read("tools/smokes/v2/profiles/integration/apps/lib/vm_hako_json_parity_common.sh")
    await_smoke = read("tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_non_future_reject_vm.sh")
    closure_smoke = read("tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_newclosure_probe_vm.sh")
    compare = read("tools/smokes/v2/profiles/integration/vm_hako_caps/compare/compare_ported_vm.sh")

    if '.env("NYASH_VERIFY_JSON"' in spawn:
        fail("driver_spawn still writes the public payload carrier")
    if "@file:" in spawn or "NYASH_VERIFY_JSON" in driver or "@file:" in driver:
        fail("driver/child still contains the public payload or @file sentinel")
    if spawn.count("cmd.status()") != 1:
        fail("driver spawn count is not exactly one")
    if "for key in CHILD_ENV_REMOVE" not in spawn or "cmd.env_remove(key)" not in spawn:
        fail("driver_spawn does not apply the finite child env scrub")
    scrub_markers = {
        "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON": "    INLINE_PAYLOAD_ENV,",
        "HAKO_VM_HAKO_DRIVER_PAYLOAD_FILE": "    FILE_PAYLOAD_ENV,",
    }
    for key in sorted(required_env_remove):
        marker = scrub_markers.get(key, f'    "{key}",')
        if marker not in spawn:
            fail(f"driver_spawn does not scrub {key}")
    for pin, marker in {
        "NYASH_SKIP_TOML_ENV": '.env("NYASH_SKIP_TOML_ENV", "1")',
        "NYASH_VM_USE_FALLBACK": '.env("NYASH_VM_USE_FALLBACK", "0")',
        "NYASH_VM_HAKO_PREFER_STRICT_DEV": '.env("NYASH_VM_HAKO_PREFER_STRICT_DEV", "0")',
        "NYASH_USE_NY_COMPILER": '.env("NYASH_USE_NY_COMPILER", "0")',
    }.items():
        if marker not in spawn:
            fail(f"driver_spawn pin missing: {pin}")
    if '.arg("--backend")' not in spawn or '"vm"' not in spawn:
        fail("driver_spawn backend vm pin missing")
    if "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON" not in spawn or "HAKO_VM_HAKO_DRIVER_PAYLOAD_FILE" not in spawn:
        fail("driver_spawn private one-of keys missing")
    if "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON" not in driver or "HAKO_VM_HAKO_DRIVER_PAYLOAD_FILE" not in driver:
        fail("driver_main private payload reader missing")

    def hako_runner_section(text: str) -> str:
        start = text.find("run_hako_vm_runner()")
        if start < 0:
            start = text.find("HAKO_OUTPUT=$(")
        end = text.find("HAKO_RC=$?", start)
        if end < 0:
            end = text.find("rc=$?", start)
        if start < 0 or end < 0:
            fail("live monitor Hako runner section is not recognizable")
        return text[start:end]

    for name, text in {
        "parity": parity,
        "await": await_smoke,
        "newclosure": closure_smoke,
    }.items():
        section = hako_runner_section(text)
        if "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON=\"$JSON_PAYLOAD\"" not in section:
            fail(f"{name} monitor does not use the private payload key")
        if "NYASH_VERIFY_JSON=\"$JSON_PAYLOAD\"" in section:
            fail(f"{name} monitor still writes the public payload key")
    if "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON=\"$NE_ALIAS_JSON_PAYLOAD\"" not in compare:
        fail("compare alias probe does not use the private payload key")
    if 'env.get("NYASH_VERIFY_JSON")' in compare or 'NYASH_VERIFY_JSON="$' in compare:
        fail("compare alias probe still contains the public payload key")
    if "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON" not in parity or "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON" not in await_smoke or "HAKO_VM_HAKO_DRIVER_PAYLOAD_JSON" not in closure_smoke:
        fail("one of the live monitors lacks the private transport declaration")

    print(
        f"[{guard_id}] result_class=current-change failure status=pass "
        "phase=reference_child_i0 private_transport=sealed env_scrub=pinned "
        "spawn=one public_reentry=zero"
    )
    raise SystemExit(0)

if phase == "canonical_v1_value_s0":
    if state.get("work_mode") not in {"fast", "closeout"}:
        fail("canonical_v1_value_s0 requires fast or closeout work mode")
    if state.get("current_execution_row") != "MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-REFERENCE-CANONICAL-V1-VALUE-S0":
        fail("canonical_v1_value_s0 pointer drifted")
    if state.get("next_execution_card") != state.get("current_execution_row"):
        fail("canonical_v1_value_s0 next execution card drifted")
    value_row = card.get("reference_canonical_v1_value_s0")
    if not isinstance(value_row, dict):
        fail("active card reference_canonical_v1_value_s0 section is missing")
    if card.get("status") not in {"canonical_v1_value_s0_fast_open", "canonical_v1_value_s0_landed"}:
        fail("active card is not marked CanonicalV1 Value S0 open or landed")
    if card.get("status") == "canonical_v1_value_s0_fast_open":
        if value_row.get("implementation_permission") is not True:
            fail("CanonicalV1 Value S0 fast-open permission is not set")
    else:
        if value_row.get("implementation_permission") is not False:
            fail("landed CanonicalV1 Value S0 must close scoped implementation permission")
    expected_files = set(value_row.get("allowed_files") or [])
    required_files = {
        "src/runner/mir_json_emit/root.rs",
        "src/runner/mir_json_emit/io.rs",
        "src/runner/reference/vm_hako.rs",
        "src/runner/reference/vm_hako/compile_bridge.rs",
        "src/runner/reference/vm_hako/payload_normalize.rs",
        "src/runner/reference/vm_hako/subset_check/mod.rs",
        "src/runner/reference/vm_hako/driver_spawn.rs",
        "src/runner/reference/vm_hako/driver_main.hako",
        guard_script,
    }
    if not required_files.issubset(expected_files):
        fail("CanonicalV1 Value S0 allowed file boundary is incomplete")

    def read(relative: str) -> str:
        path = root / relative
        if not path.is_file():
            fail(f"CanonicalV1 Value owner missing: {relative}")
        return path.read_text(encoding="utf-8")

    root_rs = read("src/runner/mir_json_emit/root.rs")
    io_rs = read("src/runner/mir_json_emit/io.rs")
    vm_rs = read("src/runner/reference/vm_hako.rs")
    compile_rs = read("src/runner/reference/vm_hako/compile_bridge.rs")
    normalize_rs = read("src/runner/reference/vm_hako/payload_normalize.rs")
    subset_rs = read("src/runner/reference/vm_hako/subset_check/mod.rs")
    if "build_mir_json_root_with_profile" not in root_rs:
        fail("explicit profile root issuer is missing")
    if "emit_canonical_v1_value_for_reference" not in io_rs:
        fail("reference CanonicalV1 Value issuer is missing")
    if "JsonEgressProfile::CanonicalV1" not in io_rs:
        fail("reference issuer is not pinned to CanonicalV1")
    vm_prod_marker = "\n#[cfg(test)]\nfn compile_source_to_mir_json_v0"
    vm_prod = vm_rs.split(vm_prod_marker, 1)[0]
    if vm_prod == vm_rs:
        fail("vm-hako test-only legacy wrapper marker is missing")
    if "compile_source_to_canonical_v1" not in compile_rs or "compile_source_to_mir_json_v0" in vm_prod:
        fail("production vm-hako path still exposes the v0 String compile entry")
    if "ScopedEnvVar" in compile_rs or "NYASH_MIR_UNIFIED_CALL" in compile_rs or "NYASH_JSON_SCHEMA_V1" in compile_rs:
        fail("reference compile bridge still selects profile through ambient env")
    if "compile_source_to_canonical_v1" not in vm_rs:
        fail("vm-hako runner does not consume the explicit CanonicalV1 issuer")
    if "check_vm_hako_subset_value" not in vm_rs or "project_main_payload" not in vm_rs:
        fail("vm-hako runner does not use Value subset/projection seams")
    if "normalize_canonical_v1_value" not in vm_rs:
        fail("vm-hako runner does not normalize the owned Value explicitly")
    if "check_vm_hako_subset_value" not in subset_rs:
        fail("subset checker lacks the Value entry")
    if "project_main_payload" not in normalize_rs:
        fail("payload projector lacks the Value entry")
    if "serde_json::from_str" in vm_prod:
        fail("vm-hako production runner reparses raw JSON")
    if "serde_json::from_str" in compile_rs:
        fail("reference compile bridge reparses emitted JSON")
    if normalize_rs.count("normalize_aliases_in_root") > 1:
        fail("payload normalization has more than one alias normalization call")
    subset_prod = subset_rs.split("#[cfg(test)]", 1)[0]
    if "normalize_aliases_in_root" in subset_prod:
        fail("subset checker production path reparses or normalizes raw JSON")
    if card.get("status") == "canonical_v1_value_s0_fast_open":
        print(
            f"[{guard_id}] result_class=current-change failure status=pass "
            "phase=canonical_v1_value_s0 permission=fast_open value=owned "
            "normalize=one subset=one projection=one raw_reparse=zero"
        )
    else:
        print(
            f"[{guard_id}] result_class=current-change failure status=pass "
            "phase=canonical_v1_value_s0 permission=landed value=owned "
            "normalize=one subset=one projection=one raw_reparse=zero"
        )
    raise SystemExit(0)

# Wpre is a boundary census, not a parser implementation.  The inventory is
# deliberately explicit: removing one edge during this guard-only row is a
# current-change failure, while the later Wpre row will change this contract.
edge_markers = {
    "src/main.rs": ("core_executor::execute_json_artifact",),
    "src/runner/pipe_io.rs": ("try_run_json_v0_pipe", "execute_json_artifact"),
    "src/runner/mod.rs": ("execute_mir_json_text", "try_run_json_v0_pipe"),
    "src/runner/dispatch.rs": ("try_parse_v1_to_module", "parse_mir_v0_to_module", "text.contains"),
    "src/runner/core_executor.rs": (
        "core_direct_requested",
        "core_direct_retired",
        "parse_direct_mir_json_text_with_v0_fallback",
    ),
    "src/runner/json_artifact/mir_loader.rs": (
        "try_parse_v1_to_module",
        "parse_direct_mir_json_text_with_v0_fallback",
        "parse_mir_v0_to_module",
    ),
    "src/runner/json_artifact/mod.rs": (
        "canonicalize_module_json",
        "load_mir_json_to_module",
        "load_program_json_v0_to_module",
    ),
    "src/runner/modes/common_util/core_bridge.rs": (
        "canonicalize_module_json",
        "methodize_calls",
    ),
    "src/runner/json_v1_bridge/parse/mod.rs": ("try_parse_v1_to_module",),
    "src/runner/mir_json_v0.rs": ("parse_mir_v0_to_module",),
    "src/runner/json_artifact/program_json_v0_loader.rs": (
        "load_program_json_v0_to_module",
        "maybe_merge_program_json_v0_imports",
    ),
    "src/runner/modes/common_util/selfhost/json.rs": (
        "parse_mir_json_v0_line",
        "parse_json_v0_line",
    ),
    "src/runner/modes/common_util/selfhost/stage_a_route.rs": (
        "run_captured_json_v0_command",
        "parse_mir_json_v0_line",
    ),
    "src/runner/modes/common_util/selfhost/stage_a_compat_bridge.rs": (
        "parse_mir_json_v0_line",
        "parse_json_v0_line",
        "enforce_stage_a_rust_json_bridge_guard_or_exit",
    ),
    "src/runner/reference/vm_hako/payload_normalize.rs": (
        "normalize_instruction_aliases_in_root",
        "normalize_global_mir_calls",
    ),
    "src/host_providers/mir_builder/backend_shape.rs": (
        "normalize_console_print_externcall",
        "nyash.console.log",
        '"type": "Global"',
    ),
}
for relative, markers in edge_markers.items():
    path = root / relative
    if not path.is_file():
        fail(f"Wpre entrance owner missing: {relative}")
    text = path.read_text(encoding="utf-8")
    missing = [marker for marker in markers if marker not in text]
    if missing:
        fail(f"raw selector/retry edge drifted in {relative}: {', '.join(missing)}")

# A guard-only readiness phase must not accidentally become a partial schema
# implementation.  Scan compiled Rust/inc surfaces, but not docs or this
# guard, so the check is about executable authority rather than prose.
source_files = [
    path
    for tree in (root / "src", root / "crates")
    if tree.is_dir()
    for path in tree.rglob("*")
    if path.suffix in {".rs", ".inc"} and path.is_file()
]
source_text = "\n".join(path.read_text(encoding="utf-8") for path in source_files)
for marker in (
    "CanonicalGlobalTargetV1",
    "CanonicalSameModuleGlobalTargetV1",
    "CanonicalBuiltinGlobalV1",
    "GlobalTargetV2",
    'schema_version: "2.0"',
    'schema_version = "2.0"',
    '"schema_version":"2.0"',
    '"schema_version": "2.0"',
):
    if marker in source_text:
        fail(f"partial v2/typed-Global surface appeared before Wpre: {marker}")

print(
    f"[{guard_id}] result_class=current-change failure status=pass "
    f"phase={phase} raw_selector_retry_edges=frozen typed_v2_surface=absent"
)
PY
