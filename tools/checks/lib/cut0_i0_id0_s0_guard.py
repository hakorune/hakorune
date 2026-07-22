#!/usr/bin/env python3
"""CUT0-I0-ID0-S0 disconnected identity/token guard."""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-t-prime-r1-execution-task-2026-07-22.md"
)
IDENTITY = ROOT / "src/mir/builder/module_invocation_identity.rs"
FIXTURES = ROOT / "src/mir/builder/module_invocation_identity_p0.rs"
BRAND_CHAIN = ROOT / "src/mir/builder/module_invocation_owner_chain.rs"
COLLECTION = ROOT / "src/mir/builder/module_invocation_collection.rs"
COLLECT_FIXTURE = ROOT / "src/mir/builder/module_invocation_collect0_s0_p0.rs"
CALLABLE_BATCH = ROOT / "src/mir/builder/module_invocation_callable_batch.rs"
CALLABLE_BATCH_FIXTURE = ROOT / "src/mir/builder/resolved_lowering/callable_batch_collection_p0.rs"
BUILDER = ROOT / "src/mir/builder.rs"
SRC = ROOT / "src"
ALLOWED = {
    IDENTITY.relative_to(ROOT),
    FIXTURES.relative_to(ROOT),
    BRAND_CHAIN.relative_to(ROOT),
    COLLECTION.relative_to(ROOT),
    COLLECT_FIXTURE.relative_to(ROOT),
    CALLABLE_BATCH.relative_to(ROOT),
    CALLABLE_BATCH_FIXTURE.relative_to(ROOT),
    BUILDER.relative_to(ROOT),
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    identity = IDENTITY.read_text()
    fixtures = FIXTURES.read_text()
    builder = BUILDER.read_text()

    for path in (IDENTITY, FIXTURES, pathlib.Path(__file__)):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"ID0-S0 file must remain below 800 lines: {path}")

    require(state, "CUT0-I0-ID0-S0 is closed as a disconnected identity/token proof", "row closeout")
    require(state, "CUT0-I0-ID0-P0 is closed as a disconnected branded owner-chain proof", "row successor closeout")
    require(state, "CUT0-I0-COLLECT0-S0 is closed as a disconnected raw/canonical co-seal proof", "successor closeout")
    require(state, "CUT0-I0-COLLECT0-BATCH0 is closed as a disconnected atomic callable-batch proof", "batch closeout")
    require(state, "CUT0-I0-SESSION0 is closed as a disconnected Builder transaction", "session closeout")
    require(state, "CUT0-I0-ROOT0 is next", "next pointer")
    require(task, "CUT0-I0-ID0-S0 — closed", "task row")
    require(task, "CUT0-I0-ID0-P0", "next task row")
    require(task, "foreign family/source construction", "foreign-source acceptance")
    require(builder, "mod module_invocation_identity;", "identity registration")
    require(builder, "mod module_invocation_identity_p0;", "fixture registration")

    for variant in (
        "Raw {",
        "CanonicalAPlus {",
        "BindingSsaTrivial {",
        "BindingSsaAcyclic {",
        "BindingSsaRecursive {",
    ):
        require(identity, variant, f"sealed family variant {variant}")
    require(identity, "pub(in crate::mir::builder) struct ModuleInvocationIdV1", "opaque ID")
    require(identity, "enum ModuleInvocationTokenKindV1", "private token variants")
    require(identity, "pub(in crate::mir::builder) struct ModuleInvocationTokenV1", "opaque token")
    require(identity, "pub(in crate::mir::builder) fn mint(", "single test mint")
    require(identity, "#[cfg(test)]", "test-only factory")
    require(fixtures, "foreign_source_family_is_rejected_before_token_creation", "foreign fixture")
    require(fixtures, "one_factory_mints_each_existing_family_once", "five-family fixture")

    if "struct ModuleInvocationIdV1" in identity and "Clone" in identity.split("struct ModuleInvocationIdV1", 1)[0].split("derive", 1)[-1]:
        raise AssertionError("invocation ID must not derive Clone")
    if "enum ModuleInvocationTokenV1" in identity and "Clone" in identity.split("enum ModuleInvocationTokenV1", 1)[0].split("derive", 1)[-1]:
        raise AssertionError("invocation token must not derive Clone")

    consumers = []
    for path in SRC.rglob("*.rs"):
        if path.relative_to(ROOT) in ALLOWED:
            continue
        text = path.read_text()
        if "ModuleInvocationIdV1" in text or "ModuleInvocationTokenV1" in text:
            consumers.append(str(path.relative_to(ROOT)))
    if consumers:
        raise AssertionError("ID0-S0 production consumers: " + ", ".join(consumers))

    print("[cut0-i0-id0-s0-guard] ok families=5 producer=test-only production_consumers=0")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
