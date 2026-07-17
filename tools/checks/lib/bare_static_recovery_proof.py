#!/usr/bin/env python3
"""Validate the disconnected canonical bare-static recovery P0 matrix."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
import re
import subprocess
import sys
from typing import Any


APP_DIR = Path("apps/bare-static-recovery-proof")
ARTIFACT_DIR = Path("target/checks/bare-static-recovery-proof")
CATALOG_DIR = Path("src/mir/builder/callable_declaration_catalog")
REQUIRED = {
    "provider_first_script.hako",
    "caller_first_script.hako",
    "provider_first_app.hako",
    "caller_first_app.hako",
    "cross_provider_first.hako",
    "cross_caller_first.hako",
    "ambiguous.hako",
    "instance_control.hako",
    "arity_overload.hako",
    "zero_arg.hako",
    "wrong_arity.hako",
    "no_candidate.hako",
    "text_merged_helper.hako",
    "text_merged_main.hako",
}


class ProofFailure(RuntimeError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ProofFailure(message)


def read_sources(root: Path) -> dict[str, str]:
    app = root / APP_DIR
    actual = {path.name for path in app.glob("*.hako")}
    require(actual == REQUIRED, f"fixture set drift: expected={sorted(REQUIRED)} actual={sorted(actual)}")
    sources = {
        name: (app / name).read_text(encoding="utf-8") for name in sorted(REQUIRED)
    }
    for name, source in sources.items():
        require(len(source.splitlines()) < 800, f"{name} reached 800 lines")
        require(not re.search(r"\bHMI\b|\bHmi", source), f"{name} must remain HMI-independent")
    return sources


def verify_source_matrix(sources: dict[str, str]) -> dict[str, Any]:
    for mode in ("script", "app"):
        provider = sources[f"provider_first_{mode}.hako"]
        caller = sources[f"caller_first_{mode}.hako"]
        require(provider.index("m_seed(x)") < provider.index("z_use(x)"), f"{mode} provider-first drift")
        require(caller.index("a_use(x)") < caller.index("m_seed(x)"), f"{mode} caller-first drift")
        require("return m_seed(x)" in provider, f"{mode} provider-first lost bare call")
        require("return m_seed(x)" in caller, f"{mode} caller-first lost bare call")
        if mode == "app":
            require("static box Main" in provider and "static box Main" in caller, "app mode requires Main.main")
        else:
            require("static box Main" not in provider and "static box Main" not in caller, "script mode must stay top-level")

    cross_provider = sources["cross_provider_first.hako"]
    cross_caller = sources["cross_caller_first.hako"]
    require(cross_provider.index("static box Provider") < cross_provider.index("static box Consumer"), "cross provider-first drift")
    require(cross_caller.index("static box Consumer") < cross_caller.index("static box Provider"), "cross caller-first drift")

    ambiguous = sources["ambiguous.hako"]
    require(len(re.findall(r"(?m)^\s{4}m_seed\(x\)", ambiguous)) == 2, "ambiguous fixture must own two static candidates")
    instance = sources["instance_control.hako"]
    require("box InstanceProvider" in instance, "instance control lost ordinary box")
    require("static box StaticProvider" in instance, "instance control lost static candidate")
    require("return m_seed(x)" in instance, "instance control lost bare call")

    overload = sources["arity_overload.hako"]
    require("m_seed(x)" in overload and "m_seed(x, y)" in overload, "arity overload matrix drift")
    require("m_seed()" in sources["zero_arg.hako"], "zero-arity fixture drift")
    require("m_seed(x, y)" in sources["wrong_arity.hako"], "wrong-arity provider drift")
    require("return m_seed(x)" in sources["wrong_arity.hako"], "wrong-arity call drift")
    require("return missing_seed(x)" in sources["no_candidate.hako"], "no-candidate call drift")

    using = sources["text_merged_main.hako"].splitlines()[0]
    require(using == 'using "apps/bare-static-recovery-proof/text_merged_helper.hako" as TextMergedHelpers', "text-merge using row drift")
    require("return m_seed(x)" in sources["text_merged_helper.hako"], "text-merged helper lost bare call")
    return {
        "fixture_count": len(sources),
        "same_box_order_modes": 4,
        "cross_box_orders": 2,
        "ambiguous_static_candidates": 2,
        "instance_contamination": 0,
        "text_merged_surfaces": 1,
        "hmi_mentions": 0,
    }


def verify_disconnected_owner(root: Path) -> dict[str, int]:
    catalog_root = root / CATALOG_DIR
    owner = (catalog_root / "recovery.rs").read_text(encoding="utf-8")
    require(owner.count("pub(crate) enum BareStaticRecoveryDecisionV1") == 1, "decision owner count drift")
    external = 0
    for path in (root / "src").rglob("*.rs"):
        if catalog_root in path.parents:
            continue
        external += path.read_text(encoding="utf-8").count("BareStaticRecoveryDecisionV1")
    require(external == 0, f"P0 production consumer count must remain zero, got {external}")
    return {"decision_owners": 1, "production_consumers": external}


def run_focused_tests(root: Path) -> None:
    completed = subprocess.run(
        [
            "cargo",
            "test",
            "-q",
            "--lib",
            "mir::builder::callable_declaration_catalog::recovery_tests",
        ],
        cwd=root,
        text=True,
        capture_output=True,
        check=False,
    )
    if completed.returncode != 0:
        raise ProofFailure(
            f"focused recovery tests failed rc={completed.returncode}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("root", nargs="?", default=".")
    args = parser.parse_args()
    root = Path(args.root).resolve()

    sources = read_sources(root)
    source_report = verify_source_matrix(sources)
    owner_report = verify_disconnected_owner(root)
    run_focused_tests(root)

    report = {
        "schema_version": 1,
        "row": "R0-BARE-STATIC-RECOVERY0-P0",
        "selection": "CANONICAL-UNIQUE-BARE-STATIC-RECOVERY-SEALED",
        "production_behavior_delta": 0,
        "source": source_report,
        "owner": owner_report,
    }
    artifact = root / ARTIFACT_DIR
    artifact.mkdir(parents=True, exist_ok=True)
    (artifact / "p0_observation.json").write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print("selection=CANONICAL-UNIQUE-BARE-STATIC-RECOVERY-SEALED")
    print("decision_owner_count=1")
    print("production_consumer_count=0")
    print(f"fixture_count={source_report['fixture_count']}")
    print("production_behavior_delta=0")
    print("summary=observed")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProofFailure as exc:
        print(f"[bare-static-recovery-proof] ERROR: {exc}", file=sys.stderr)
        raise SystemExit(1)
