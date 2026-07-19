#!/usr/bin/env python3
"""Structural guard for LOOP0-S0a/S0b provenance and S0c atomic claims."""

from __future__ import annotations

import re
from pathlib import Path


CONSTRUCTOR_PATHS = (
    "src/mir/builder/control_flow/plan/normalizer/common.rs",
    "src/mir/builder/control_flow/plan/normalizer/helpers_value.rs",
    "src/mir/builder/control_flow/plan/normalizer/cond_lowering_prelude.rs",
    "src/mir/builder/control_flow/plan/normalizer/loop_body_lowering.rs",
    "src/mir/builder/control_flow/plan/parts/stmt.rs",
    "src/mir/builder/control_flow/plan/features/generic_loop_body/v0.rs",
    "src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs",
    "src/mir/builder/control_flow/plan/features/loop_cond_bc_util.rs",
    "src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs",
    "src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_body_helpers.rs",
    "src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_pipeline.rs",
    "src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs",
)


def _read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        raise RuntimeError(f"LOOP0-S0a missing {relative}")
    return path.read_text(encoding="utf-8")


def _skip_rust_space_and_comments(text: str, index: int) -> int:
    while index < len(text):
        if text[index].isspace():
            index += 1
            continue
        if text.startswith("//", index):
            newline = text.find("\n", index + 2)
            return len(text) if newline < 0 else _skip_rust_space_and_comments(text, newline + 1)
        if text.startswith("/*", index):
            depth = 1
            index += 2
            while index < len(text) and depth:
                if text.startswith("/*", index):
                    depth += 1
                    index += 2
                elif text.startswith("*/", index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
            continue
        return index
    return index


def _skip_rust_literal_or_comment(text: str, index: int) -> int | None:
    if text.startswith("//", index):
        newline = text.find("\n", index + 2)
        return len(text) if newline < 0 else newline + 1
    if text.startswith("/*", index):
        depth = 1
        cursor = index + 2
        while cursor < len(text) and depth:
            if text.startswith("/*", cursor):
                depth += 1
                cursor += 2
            elif text.startswith("*/", cursor):
                depth -= 1
                cursor += 2
            else:
                cursor += 1
        return cursor

    raw = re.match(r"(?:br|r)(?P<hashes>#{0,16})\"", text[index:])
    if raw is not None:
        delimiter = '"' + raw.group("hashes")
        body = index + raw.end()
        close = text.find(delimiter, body)
        return len(text) if close < 0 else close + len(delimiter)

    quote_index = index + 1 if text.startswith(('b"', 'c"'), index) else index
    if quote_index < len(text) and text[quote_index] == '"':
        cursor = quote_index + 1
        while cursor < len(text):
            if text[cursor] == "\\":
                cursor += 2
            elif text[cursor] == '"':
                return cursor + 1
            else:
                cursor += 1
        return len(text)

    # Only treat a quote as a character literal when it has a nearby closing
    # quote. Lifetimes such as `'plan` must remain ordinary tokens.
    if text[index] == "'":
        cursor = index + 2 if text.startswith("'\\", index) else index + 1
        close = text.find("'", cursor)
        if 0 <= close - index <= 8:
            return close + 1
    return None


def _matching_rust_brace(text: str, opening: int) -> int:
    depth = 1
    cursor = opening + 1
    while cursor < len(text):
        skipped = _skip_rust_literal_or_comment(text, cursor)
        if skipped is not None:
            cursor = skipped
            continue
        if text[cursor] == "{":
            depth += 1
        elif text[cursor] == "}":
            depth -= 1
            if depth == 0:
                return cursor + 1
        cursor += 1
    raise RuntimeError("LOOP0-S0b unterminated #[cfg(test)] item")


def _cfg_test_item_end(text: str, attribute_start: int) -> int:
    cursor = attribute_start + len("#[cfg(test)]")
    cursor = _skip_rust_space_and_comments(text, cursor)
    while text.startswith("#[", cursor):
        close = text.find("]", cursor + 2)
        if close < 0:
            raise RuntimeError("LOOP0-S0b unterminated test attribute")
        cursor = _skip_rust_space_and_comments(text, close + 1)

    item_start = cursor
    use_item = re.match(r"(?:pub(?:\([^)]*\))?\s+)?use\b", text[item_start:])
    paren_depth = 0
    bracket_depth = 0
    while cursor < len(text):
        skipped = _skip_rust_literal_or_comment(text, cursor)
        if skipped is not None:
            cursor = skipped
            continue
        token = text[cursor]
        if token == "(":
            paren_depth += 1
        elif token == ")":
            paren_depth -= 1
        elif token == "[":
            bracket_depth += 1
        elif token == "]":
            bracket_depth -= 1
        elif token == ";" and paren_depth == 0 and bracket_depth == 0:
            return cursor + 1
        elif token == "{" and paren_depth == 0 and bracket_depth == 0 and not use_item:
            end = _matching_rust_brace(text, cursor)
            end = _skip_rust_space_and_comments(text, end)
            return end + 1 if end < len(text) and text[end] == ";" else end
        cursor += 1
    raise RuntimeError("LOOP0-S0b unterminated #[cfg(test)] item")


def _production(text: str) -> str:
    """Remove each cfg(test) item without discarding later production items."""

    output = []
    cursor = 0
    marker_line = re.compile(r"(?m)^[ \t]*(?P<marker>#\[cfg\(test\)\])")
    while True:
        match = marker_line.search(text, cursor)
        if match is None:
            output.append(text[cursor:])
            return "".join(output)
        start = match.start("marker")
        output.append(text[cursor:start])
        end = _cfg_test_item_end(text, start)
        # Preserve line structure so diagnostics and simple structural regexes
        # cannot accidentally join tokens across the removed test item.
        output.append("\n" * text[start:end].count("\n"))
        cursor = end


def _is_test_source(path: Path) -> bool:
    return (
        path.name == "tests.rs"
        or path.name.endswith("_tests.rs")
        or "tests" in path.parts
    )


def _struct_body(text: str, owner: str) -> str:
    match = re.search(rf"\bstruct\s+{owner}(?:<[^>]*>)?\s*\{{(?P<body>.*?)\n\}}", text, re.S)
    if match is None:
        raise RuntimeError(f"LOOP0-S0b missing struct body: {owner}")
    return match.group("body")


def _enum_variant_body(text: str, variant: str) -> str:
    match = re.search(rf"\b{variant}\s*\{{(?P<body>.*?)\n\s*\}},", text, re.S)
    if match is None:
        raise RuntimeError(f"LOOP0-S0a missing call variant body: {variant}")
    return match.group("body")


def _reject_clone_owner(text: str, owner: str) -> None:
    derived_clone = re.search(
        rf"#\[derive\([^\]]*\bClone\b[^\]]*\)\]\s*"
        rf"(?:pub(?:\([^)]*\))?\s+)?struct\s+{owner}\b",
        text,
    )
    if derived_clone or re.search(rf"\bimpl(?:<[^>]*>)?\s+Clone\s+for\s+{owner}\b", text):
        raise RuntimeError(f"LOOP0-S0b non-Clone owner became Clone: {owner}")


def check_loop0_s0a(root: Path) -> str:
    source_path = "src/mir/builder/control_flow/plan/call_source.rs"
    effect_path = "src/mir/builder/control_flow/plan/effect.rs"
    remapper_path = (
        "src/mir/builder/control_flow/plan/normalizer/"
        "cond_lowering_freshen/remapper.rs"
    )
    located_path = "src/mir/builder/control_flow/plan/located_loop.rs"
    located_error_path = "src/mir/builder/control_flow/plan/located_loop_error.rs"
    schedule_path = "src/mir/callable_result_representation/loop_claim_schedule.rs"
    batch_path = "src/mir/callable_result_representation/loop_claim_batch.rs"
    ledger_path = "src/mir/callable_result_representation/caller_ledger.rs"
    source = _read(root, source_path)
    effect = _read(root, effect_path)
    remapper = _read(root, remapper_path)
    located = _read(root, located_path)
    located_error = _read(root, located_error_path)
    schedule = _read(root, schedule_path)
    batch = _read(root, batch_path)
    ledger = _read(root, ledger_path)

    if source.count("enum CoreCallSourceV1") != 1:
        raise RuntimeError("LOOP0-S0a requires one call-source vocabulary owner")
    if source.count("fn visit_core_call_sources_v1") != 1:
        raise RuntimeError("LOOP0-S0a requires one exhaustive call-source visitor")
    effect_production = _production(effect)
    for variant in ("MethodCall", "GlobalCall", "ValueCall", "ExternCall"):
        variant_body = _enum_variant_body(effect_production, variant)
        if variant_body.count("source: CoreCallSourceV1,") != 1:
            raise RuntimeError(
                f"LOOP0-S0a call-source field drift: variant={variant}"
            )
    if effect_production.count("source: CoreCallSourceV1,") != 4:
        raise RuntimeError("LOOP0-S0a requires exactly four call-source fields")

    plan_root = root / "src/mir/builder/control_flow/plan"
    production_plan_sources = []
    production_plan_by_path = {}
    for path in sorted(plan_root.rglob("*.rs")):
        if _is_test_source(path):
            continue
        text = _production(path.read_text(encoding="utf-8"))
        production_plan_sources.append(text)
        production_plan_by_path[path.relative_to(root).as_posix()] = text
    production_plan_text = "\n".join(production_plan_sources)
    # P0a routes the 13 source-derived normalizer constructors through one
    # stack-scoped call-source port. B0 moves six statement constructors onto
    # that same associated-input authority, leaving 15 raw/synthetic
    # constructors explicitly Unlocated. The P0a guard below owns the dynamic
    # producer and production-located-zero invariants; the B0 guard fixes the
    # six associated statement owners.
    unlocated = production_plan_text.count("source: CoreCallSourceV1::Unlocated")
    if unlocated != 15:
        raise RuntimeError(
            f"LOOP0-P0a explicit Unlocated constructor drift: expected=15 actual={unlocated}"
        )
    located_production = _production(located)
    located_variant_reads = located_production.count(
        "if let CoreCallSourceV1::LocatedMethodCall(site) = source"
    )
    if located_variant_reads != 1:
        raise RuntimeError("LOOP0-S0b wrapper must inspect located call sources exactly once")
    located_source_allowlist = {
        located_path: 1,
        "src/mir/builder/control_flow/plan/expression_port.rs": 1,
    }
    for path, text in production_plan_by_path.items():
        actual = text.count("CoreCallSourceV1::LocatedMethodCall(")
        expected = located_source_allowlist.get(path, 0)
        if actual != expected:
            raise RuntimeError(
                "LOOP0-O0-R0 production located call-source owner drift: "
                f"path={path} expected={expected} actual={actual}"
            )

    source_production = _production(source)
    if source_production.count("LocatedMethodCall") != 1:
        raise RuntimeError(
            "LOOP0-S0a production call-source module must only define the located variant"
        )
    remapper_production = _production(remapper)
    if "LocatedMethodCall" in remapper_production:
        raise RuntimeError("LOOP0-S0a remapper must preserve provenance opaquely")
    if remapper_production.count("source: _,") != 4:
        raise RuntimeError("LOOP0-S0a remapper must cover all four sources without mutation")

    all_mir_production = []
    all_mir_production_by_path = {}
    for path in sorted((root / "src/mir").rglob("*.rs")):
        if _is_test_source(path):
            continue
        relative = path.relative_to(root).as_posix()
        text = _production(path.read_text(encoding="utf-8"))
        all_mir_production.append(text)
        all_mir_production_by_path[relative] = text
    all_mir_production_text = "\n".join(all_mir_production)
    for owner in (
        "struct VerifiedLocatedCoreLoopPlanV1",
        "struct VerifiedCallableResultLoopClaimScheduleV1",
        "enum LocatedCoreLoopPlanErrorV1",
        "enum CallableResultLoopClaimScheduleErrorV1",
        "struct ClaimedCallableResultLoopBatchV1",
        "enum CallableResultLoopClaimBatchErrorV1",
    ):
        count = all_mir_production_text.count(owner)
        if count != 1:
            raise RuntimeError(f"LOOP0-S0b owner drift: owner={owner!r} count={count}")
    # A consumer must mention the original type even when imported through an
    # alias. Freeze every allowed production occurrence instead of looking for
    # one spelling of `Type::verify(`.
    occurrence_allowlist = {
        "VerifiedLocatedCoreLoopPlanV1": {
            located_path: 2,
            "src/mir/builder/control_flow/plan/mod.rs": 1,
            "src/mir/builder/control_flow/plan/features/generic_loop_located_composer.rs": 3,
        },
        "VerifiedCallableResultLoopClaimScheduleV1": {
            located_path: 4,
            schedule_path: 2,
            batch_path: 2,
            "src/mir/callable_result_representation/mod.rs": 1,
        },
    }
    for owner, allowed_by_path in occurrence_allowlist.items():
        for path, text in all_mir_production_by_path.items():
            expected = allowed_by_path.get(path, 0)
            actual = text.count(owner)
            if actual != expected:
                raise RuntimeError(
                    "LOOP0-S0b production occurrence drift: "
                    f"owner={owner} path={path} expected={expected} actual={actual}"
                )

    schedule_production = _production(schedule)
    located_error_production = _production(located_error)
    passive_callable_result_consumers = {
        located_path,
        located_error_path,
        "src/mir/builder/control_flow/plan/expression_port.rs",
        "src/mir/builder/control_flow/plan/generic_loop/located_representation/mod.rs",
        "src/mir/builder/control_flow/plan/generic_loop/located_representation/lowering_view.rs",
        "src/mir/builder/control_flow/plan/generic_loop/located_representation/product.rs",
        "src/mir/builder/control_flow/plan/generic_loop/located_representation/recipe_seal.rs",
        "src/mir/builder/control_flow/plan/features/generic_loop_located_composer.rs",
    }
    for path, text in production_plan_by_path.items():
        if (
            path not in passive_callable_result_consumers
            and "callable_result_representation" in text
        ):
            raise RuntimeError(
                f"LOOP0-S0b callable-result authority escaped located seal: {path}"
            )
    for forbidden in (
        "VerifiedCallableResultCallerLedgerV1",
        "ClaimedCallableResultActivationSiteV1",
        "ClaimedCallableResultLoopBatchV1",
        "claim_loop_batch",
        "&mut VerifiedCallableResultCallerLedgerV1",
        ".claim(",
        ".finish(",
    ):
        if forbidden in located_production + located_error_production + schedule_production:
            raise RuntimeError(f"LOOP0-S0b seal must remain ledger-read-only: {forbidden}")

    _reject_clone_owner(located_production, "VerifiedLocatedCoreLoopPlanV1")
    _reject_clone_owner(schedule_production, "VerifiedCallableResultLoopClaimScheduleV1")
    batch_production = _production(batch)
    ledger_production = _production(ledger)
    _reject_clone_owner(batch_production, "ClaimedCallableResultLoopBatchV1")
    if re.search(r"\b(?:Arc|Rc)\s*<", located_production + schedule_production):
        raise RuntimeError("LOOP0-S0b seal/schedule must not use Arc or Rc")
    if re.search(r"\b(?:Arc|Rc)\s*<", batch_production):
        raise RuntimeError("LOOP0-S0c claim batch must not use Arc or Rc")

    located_fields = _struct_body(located_production, "VerifiedLocatedCoreLoopPlanV1")
    schedule_fields = _struct_body(
        schedule_production, "VerifiedCallableResultLoopClaimScheduleV1"
    )
    for fields, required, count in (
        (located_fields, "plan: CorePlan,", 1),
        (located_fields, "schedule: VerifiedCallableResultLoopClaimScheduleV1<'plan>,", 1),
        (schedule_fields, "activation_plan: &'plan VerifiedCallableResultActivationPlanV1,", 1),
        (schedule_fields, "caller: &'plan CanonicalSameModuleCallableKeyV1,", 1),
        (schedule_fields, "loop_root: SourceStmtSiteV1,", 1),
        (schedule_fields, "rows: Box<[&'plan VerifiedCallableResultActivationSiteV1]>,", 1),
    ):
        actual = fields.count(required)
        if actual != count:
            raise RuntimeError(
                f"LOOP0-S0b sealed-field drift: field={required!r} expected={count} actual={actual}"
            )
    visible_sealed_field = re.search(
        r"(?m)^\s*pub(?:\([^)]*\))?\s+"
        r"(?:plan|schedule|activation_plan|caller|rows|loop_root|_seal)\s*:",
        located_fields + schedule_fields,
    )
    if visible_sealed_field is not None:
        raise RuntimeError(
            "LOOP0-S0b sealed fields must remain private: "
            f"{visible_sealed_field.group(0).strip()}"
        )

    if schedule_production.count(".rows_for(caller)") != 1:
        raise RuntimeError("LOOP0-S0b schedule must borrow one canonical rows_for(caller) order")
    if schedule_production.count("rows.push(row)") != 1:
        raise RuntimeError("LOOP0-S0b schedule must retain activation rows without reconstruction")
    for forbidden_order in (
        "BTreeMap",
        ".sort(",
        ".sort_by(",
        ".sort_by_key(",
        ".sort_unstable(",
        ".sort_unstable_by(",
    ):
        if forbidden_order in schedule_production:
            raise RuntimeError(
                f"LOOP0-S0b source-order schedule must not reorder rows: {forbidden_order}"
            )
    for required_brand in (
        "parts.plan_identity != activation_plan as *const _ as usize",
        "std::ptr::eq(parts.caller, caller)",
        "std::ptr::eq(self.activation_plan, activation_plan)",
    ):
        if required_brand not in schedule_production:
            raise RuntimeError(f"LOOP0-S0b schedule brand check missing: {required_brand}")
    if "PhantomData" in schedule_production:
        raise RuntimeError("LOOP0-S0b schedule must retain real branded references")

    if located_production.count("visit_core_call_sources_v1(&plan") != 1:
        raise RuntimeError("LOOP0-S0b wrapper must reuse one exhaustive CorePlan visitor")
    verifier_call = "PlanVerifier::verify(&plan)"
    schedule_call = "VerifiedCallableResultLoopClaimScheduleV1::verify("
    if located_production.count(verifier_call) != 1:
        raise RuntimeError("LOOP0-S0b wrapper must run PlanVerifier exactly once")
    if located_production.count(schedule_call) != 1:
        raise RuntimeError("LOOP0-S0b wrapper must construct the claim schedule exactly once")
    if located_production.index(verifier_call) > located_production.index(schedule_call):
        raise RuntimeError("LOOP0-S0b PlanVerifier must run before claim-schedule construction")
    for required_error in (
        "PlanVerification(String)",
        "ClaimSchedule(CallableResultLoopClaimScheduleErrorV1)",
        "ExpectedLoopPlan",
        "MissingLocatedOccurrence",
        "DuplicateLocatedOccurrence",
        "UnexpectedLocatedOccurrence",
    ):
        if required_error not in located_error_production:
            raise RuntimeError(f"LOOP0-S0b located-plan error vocabulary drift: {required_error}")
    for required_error in (
        "ForeignPlan",
        "ForeignCaller",
        "ExpectedLocatedLoop",
        "DuplicateActivationSite",
        "NoActivationRowsUnderLoop",
    ):
        if required_error not in schedule_production:
            raise RuntimeError(f"LOOP0-S0b schedule error vocabulary drift: {required_error}")

    if "VerifiedLocatedCoreLoopPlanV1" in remapper_production:
        raise RuntimeError("LOOP0-S0b remap after located-plan seal must remain unavailable")
    for remap_api in ("fn remap", ".remap(", "remap_core", "freshen"):
        if remap_api in located_production:
            raise RuntimeError(f"LOOP0-S0b remap API after seal detected: {remap_api}")
    for escape in (
        "fn plan(&",
        "fn plan_mut(",
        "fn into_plan(",
        "fn into_parts(",
        "Deref for VerifiedLocatedCoreLoopPlanV1",
        "DerefMut for VerifiedLocatedCoreLoopPlanV1",
        "AsRef<CorePlan> for VerifiedLocatedCoreLoopPlanV1",
        "AsMut<CorePlan> for VerifiedLocatedCoreLoopPlanV1",
    ):
        if escape in located_production:
            raise RuntimeError(f"LOOP0-S0b sealed CorePlan escape detected: {escape}")
    if re.search(r"->\s*(?:&\s*(?:mut\s+)?)?CorePlan\b", located_production):
        raise RuntimeError("LOOP0-S0b sealed CorePlan return-type escape detected")

    batch_fields = _struct_body(batch_production, "ClaimedCallableResultLoopBatchV1")
    for required, count in (
        ("activation_plan: &'plan VerifiedCallableResultActivationPlanV1,", 1),
        ("caller: &'plan CanonicalSameModuleCallableKeyV1,", 1),
        ("loop_root: SourceStmtSiteV1,", 1),
        ("source_order: Box<[&'plan SourceExprSiteV1]>,", 1),
        ("claims_by_site: BTreeMap<SourceExprSiteV1, LoopClaimSlotV1<'plan>>,", 1),
    ):
        if batch_fields.count(required) != count:
            raise RuntimeError(f"LOOP0-S0c batch field drift: {required}")
    for forbidden_field in ("ValueId", "target:", "abi:", "effect:", "span:"):
        if forbidden_field in batch_fields:
            raise RuntimeError(f"LOOP0-S0c batch copied foreign authority: {forbidden_field}")
    if batch_production.count("fn claim_loop_batch(") != 1:
        raise RuntimeError("LOOP0-S0c requires one ledger batch-claim entry")
    if all_mir_production_text.count(".claim_loop_batch(") != 0:
        raise RuntimeError("LOOP0-S0c production claim callers must remain zero")
    for required_error in ("UnexpectedSite", "AlreadyConsumed", "Unconsumed"):
        if required_error not in batch_production:
            raise RuntimeError(f"LOOP0-S0c batch error vocabulary drift: {required_error}")
    if batch_production.count("std::mem::replace(slot, LoopClaimSlotV1::Consumed)") != 1:
        raise RuntimeError("LOOP0-S0c claim removal must leave one consumed tombstone")
    if ledger_production.count("fn prevalidate_and_commit_loop_schedule(") != 1:
        raise RuntimeError("LOOP0-S0c requires one ledger-owned batch commit")
    if ledger_production.count("self.claimed.extend(staged)") != 1:
        raise RuntimeError("LOOP0-S0c requires one non-fallible staged commit")
    commit = ledger_production.index("self.claimed.extend(staged)")
    prevalidate = ledger_production.index("fn prevalidate_and_commit_loop_schedule(")
    if commit < prevalidate:
        raise RuntimeError("LOOP0-S0c ledger committed before batch prevalidation")
    for forbidden_api in (
        "pub(crate) fn claim_site(",
        "pub(crate) fn claim_sites(",
        "pub(crate) fn claim_source_site(",
    ):
        if forbidden_api in batch_production + ledger_production:
            raise RuntimeError(f"LOOP0-S0c unbranded site claim API detected: {forbidden_api}")

    builder_root = _production(_read(root, "src/mir/builder.rs"))
    for forbidden_field in (
        "VerifiedCallableResultCallerLedgerV1",
        "ClaimedCallableResultActivationSiteV1",
        "VerifiedLocatedCoreLoopPlanV1",
        "VerifiedCallableResultLoopClaimScheduleV1",
        "ClaimedCallableResultLoopBatchV1",
    ):
        if forbidden_field in builder_root:
            raise RuntimeError(f"LOOP0-S0a MirBuilder authority leak: {forbidden_field}")

    touched = (
        source_path,
        effect_path,
        remapper_path,
        located_path,
        located_error_path,
        schedule_path,
        batch_path,
        ledger_path,
        "src/mir/callable_result_representation/tests/loop_claim_batch.rs",
        "src/mir/builder/control_flow/plan/located_loop_tests.rs",
        "src/mir/builder/control_flow/plan/mod.rs",
        "src/mir/callable_result_representation/mod.rs",
        *CONSTRUCTOR_PATHS,
        __file__,
    )
    oversized = []
    for path in touched:
        relative = str(path) if isinstance(path, str) else str(Path(path).relative_to(root))
        if len(_read(root, relative).splitlines()) >= 800:
            oversized.append(relative)
    if oversized:
        raise RuntimeError(f"LOOP0-S0b source/check files reached 800 lines: {oversized}")

    return (
        "loop0_s0a_sources=4 explicit_unlocated_constructors=15 located_producers=0 "
        "loop0_s0b_wrapper=1 schedule=1 loop0_s0c_batch=1 "
        "production_claim_callers=0 atomic_ledger_commits=1"
    )
