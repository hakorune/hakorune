#!/usr/bin/env python3
"""Guard the disconnected HMI-S0-T0 authority boundary."""

from __future__ import annotations

import pathlib
import re
import sys


ROOT = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
HMI = ROOT / "tools/hako_shared/hmi"


def fail(message: str) -> None:
    raise SystemExit(f"[hmi/t0-authority] ERROR: {message}")


def read(path: pathlib.Path) -> str:
    return path.read_text(encoding="utf-8")


if not HMI.is_dir():
    fail("missing tools/hako_shared/hmi")

hako_files = sorted(HMI.rglob("*.hako"))
if not hako_files:
    fail("no HMI .hako sources")

for path in hako_files:
    if len(read(path).splitlines()) >= 800:
        fail(f"source file reaches 800 lines: {path.relative_to(ROOT)}")

all_hmi = "\n".join(read(path) for path in hako_files)
ingress = read(HMI / "strict_ingress.hako")
if ingress.count(".parse_with_policy(") != 1:
    fail("strict parser selector count must be exactly one")
if re.search(r"\.parse\s*\(", ingress):
    fail("compatibility JsonParser.parse is forbidden")

for forbidden in (
    "raw_root(",
    "raw_node(",
    "raw_cfg_node(",
    "field(name)",
    "metadata(name)",
):
    if forbidden in all_hmi:
        fail(f"forbidden generic view accessor remains: {forbidden}")

for forbidden in (
    "to_v0",
    "payload_normalize",
    "fallback_to",
    "retry_with",
):
    if forbidden in all_hmi:
        fail(f"forbidden conversion/retry token remains: {forbidden}")

source_without_tests = "\n".join(
    read(path) for path in hako_files if "/tests/" not in path.as_posix()
)
if re.search(r"\bprint\s*\(", source_without_tests):
    fail("unconditional HMI source print is forbidden")

publication_path = HMI / "view/publication.hako"
publication_source = read(publication_path) if publication_path.exists() else ""
constructor_sources = []
for path in hako_files:
    if path == publication_path:
        continue
    if "new VerifiedHmi" in read(path):
        constructor_sources.append(path.relative_to(ROOT).as_posix())
if constructor_sources:
    fail(f"verified view constructors escaped publication owner: {constructor_sources}")
if publication_source:
    for name in (
        "VerifiedHmiDocumentView",
        "VerifiedHmiFunctionView",
        "VerifiedHmiBlockView",
        "VerifiedHmiInstructionView",
    ):
        if publication_source.count(f"new {name}") != 1:
            fail(f"{name} constructor site count must be exactly one")
else:
    fail("missing sole Verified view publication owner")

publisher_calls = []
publisher_selector = "HmiVerifiedViewPublisherV1.publish("
for path in hako_files:
    count = read(path).count(publisher_selector)
    if count:
        publisher_calls.extend(
            [path.relative_to(ROOT).as_posix()] * count
        )
if publisher_calls != ["tools/hako_shared/hmi/document_seal.hako"]:
    fail(
        "whole-document publisher selector must occur exactly once in "
        f"document_seal: {publisher_calls}"
    )

root_handoff_users = []
for path in hako_files:
    if ".root_for_seal()" in read(path) and path.name != "document_seal.hako":
        root_handoff_users.append(path.relative_to(ROOT).as_posix())
if root_handoff_users:
    fail(f"raw parse root escaped whole-document seal: {root_handoff_users}")

external_callers = []
for path in ROOT.rglob("*.hako"):
    if HMI in path.parents:
        continue
    if "tools.hako_shared.hmi" in read(path):
        external_callers.append(path.relative_to(ROOT).as_posix())
if external_callers:
    fail(f"production/external callers must remain zero: {external_callers}")

error_source = read(HMI / "seal/error.hako")
renderer = '"[freeze:contract][hmi/mir_json_v1/" + me.family + "]"'
if renderer not in error_source:
    fail("stable HMI MIR JSON V1 error renderer missing")
for family in ("document", "cfg", "value_type", "ownership"):
    if f'"{family}"' not in error_source:
        fail(f"stable error family missing: {family}")

print(
    "[hmi/t0-authority] ok "
    f"hako_files={len(hako_files)} external_callers=0"
)
