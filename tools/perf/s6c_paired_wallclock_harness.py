#!/usr/bin/env python3
"""Create, run, or terminally close one S6C MeasurementBatch V2."""

from __future__ import annotations

import argparse
import csv
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys

from s6c_paired_wallclock_batch import (
    SESSION_SLOTS, issue_manifest,
)
from s6c_paired_wallclock_batch_store import (
    BINARY_NAME, StoreError, close, close_abandoned, create_batch,
    exclusive_batch, load_manifest, publish_complete_session,
    publish_ineligible_session, read_json, self_test as store_self_test,
    session_path,
)
from s6c_paired_wallclock_plan import CANONICAL_CASES, seal_plan, validate_session


FIELDS = (
    "case", "slot", "block", "block_slot", "order", "attempt", "oracle_equal",
    "family", "size", "position", "sample", "iterations", "hako_ns", "c_ns",
    "sink", "scalars", "width1", "width2", "width3", "width4",
)
SYMBOLS = {"hako_s6c_meso", "hako_s6c_c_meso"}
ABANDON_REASONS = ("controller_interrupted", "host_shutdown", "tool_transport_lost")


class HarnessError(RuntimeError):
    pass


class AcquisitionIncomplete(HarnessError):
    pass


def sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def orders_text(plan: dict[str, object], case: str) -> str:
    return "".join("A" if order == "AB" else "B" for order in plan["schedules"][case])


def parse_case_output(text: str, expected_case: str) -> list[dict[str, object]]:
    reader = csv.DictReader(text.splitlines())
    if tuple(reader.fieldnames or ()) != FIELDS:
        raise HarnessError("robust_case_csv_header_drift")
    rows, expected_shape, expected_iterations = [], None, None
    for row in reader:
        if row["case"] != expected_case or \
                f'{row["family"]}/{row["size"]}/{row["position"]}' != expected_case:
            raise HarnessError("case_identity_drift")
        slot, size = int(row["slot"]), int(row["size"])
        shape = tuple(int(row[name]) for name in (
            "scalars", "width1", "width2", "width3", "width4"))
        iterations, sink = int(row["iterations"]), int(row["sink"])
        if int(row["sample"]) != slot or iterations <= 0 or sink == 0 or \
                sum((width + 1) * shape[width + 1] for width in range(4)) != size or \
                sum(shape[1:]) != shape[0]:
            raise HarnessError("sample_iteration_sink_or_utf8_shape_drift")
        if expected_shape is not None and (shape != expected_shape or
                                           iterations != expected_iterations):
            raise HarnessError("case_shape_or_calibration_drift")
        expected_shape, expected_iterations = shape, iterations
        rows.append({"case": row["case"], "slot": slot,
                     "block": int(row["block"]), "block_slot": int(row["block_slot"]),
                     "order": row["order"], "attempt": int(row["attempt"]),
                     "oracle_equal": row["oracle_equal"] == "true",
                     "hako_ns": int(row["hako_ns"]), "c_ns": int(row["c_ns"])})
    if len(rows) != 51:
        raise HarnessError("robust_case_pair_census_drift")
    return rows


def run_session(binary: Path, plan: dict[str, object], cpu: int) -> tuple[list[dict], str]:
    rows, raw_parts = [], []
    for case in plan["cases"]:
        family, size, position = case.split("/")
        process = subprocess.run(
            ["taskset", "-c", str(cpu), str(binary), "--robust-case",
             family, size, position, orders_text(plan, case)],
            text=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, timeout=900)
        if process.returncode:
            raise AcquisitionIncomplete("case_process_rejected")
        rows.extend(parse_case_output(process.stdout, case))
        raw_parts.append(process.stdout)
    return rows, "".join(raw_parts)


def validate_source(commit: str) -> None:
    root = Path(__file__).resolve().parents[2]
    head = subprocess.check_output(
        ["git", "-C", str(root), "rev-parse", "HEAD"], text=True).strip()
    dirty = any(subprocess.run(
        ["git", "-C", str(root), *arguments]).returncode for arguments in (
            ("diff", "--quiet"), ("diff", "--cached", "--quiet")))
    if head != commit or dirty:
        raise HarnessError("source_commit_or_worktree_drift")


def validate_wsl() -> None:
    if "microsoft" not in os.uname().release.lower():
        raise HarnessError("environment_role_drift")


def load_alignment(
        path: Path, binary_sha256: str, commit: str) -> tuple[str, str, str]:
    try:
        payload = path.read_text()
        manifest = json.loads(payload)
    except (OSError, json.JSONDecodeError) as error:
        raise HarnessError("alignment_manifest_unreadable") from error
    symbols = manifest.get("symbols", {})
    if manifest.get("schema") != "s6c-pinned-corridor-meso-alignment-evidence-v1" or \
            manifest.get("source_commit") != commit or \
            manifest.get("binary_sha256") != binary_sha256 or \
            set(symbols) != SYMBOLS or any(
                row.get("address_mod_64") != 0 or len(row.get("body_sha256", "")) != 64
                for row in symbols.values()) or \
            len(manifest.get("build_id", "")) != 40:
        raise HarnessError("alignment_manifest_identity_drift")
    return manifest["build_id"], hashlib.sha256(payload.encode()).hexdigest(), payload


def create_command(args: argparse.Namespace) -> int:
    if not args.binary.is_file():
        raise HarnessError("candidate_binary_missing")
    validate_source(args.commit)
    validate_wsl()
    binary_payload = args.binary.read_bytes()
    binary_sha256 = hashlib.sha256(binary_payload).hexdigest()
    build_id, alignment_sha256, alignment_payload = load_alignment(
        args.alignment_manifest, binary_sha256, args.commit)
    predecessor = read_json(args.predecessor) if args.predecessor else None
    manifest = issue_manifest(
        commit=args.commit, binary_sha256=binary_sha256, build_id=build_id,
        alignment_sha256=alignment_sha256, cpu=args.cpu, predecessor=predecessor,
        repeat_reason=args.repeat_reason)
    directory = create_batch(
        args.root, manifest, frozen_binary=binary_payload,
        alignment_payload=alignment_payload)
    print(directory)
    return 0


def plan_for(manifest: dict[str, object], slot: int) -> dict[str, object]:
    candidate = manifest["candidate"]
    plan = seal_plan(
        commit=candidate["commit"], binary_sha256=candidate["binary_sha256"],
        cases=CANONICAL_CASES, environment_class="wsl_development", session_index=slot)
    if plan["plan_sha256"] != manifest["session_plan_sha256"][slot]:
        raise HarnessError("session_plan_projection_drift")
    return plan


def run_command(args: argparse.Namespace) -> int:
    with exclusive_batch(args.batch_dir):
        manifest = load_manifest(args.batch_dir)
        validate_source(manifest["candidate"]["commit"])
        validate_wsl()
        binary = args.batch_dir / BINARY_NAME
        if sha256(binary) != manifest["candidate"]["binary_sha256"]:
            raise HarnessError("frozen_binary_drift")
        if any(session_path(args.batch_dir, slot).exists() for slot in SESSION_SLOTS):
            raise HarnessError("same_batch_resume_forbidden")
        integrity_failed = False
        for slot in SESSION_SLOTS:
            if integrity_failed:
                publish_ineligible_session(
                    args.batch_dir, manifest, slot=slot,
                    terminal_state="IntegrityInvalid",
                    reason="batch_aborted_after_integrity_invalid")
                continue
            try:
                plan = plan_for(manifest, slot)
                rows, raw_csv = run_session(binary, plan, manifest["environment"]["cpu"])
                outcome = validate_session(plan, rows)["outcome"]
                if sha256(binary) != manifest["candidate"]["binary_sha256"]:
                    raise HarnessError("frozen_binary_drift")
                publish_complete_session(
                    args.batch_dir, manifest, slot=slot, outcome=outcome, raw_csv=raw_csv)
            except (AcquisitionIncomplete, subprocess.TimeoutExpired) as error:
                reason = str(error) if isinstance(error, AcquisitionIncomplete) else "session_timeout"
                publish_ineligible_session(
                    args.batch_dir, manifest, slot=slot,
                    terminal_state="Incomplete", reason=reason)
            except (HarnessError, ValueError, KeyError) as error:
                if isinstance(error, HarnessError):
                    reason = str(error)
                elif isinstance(error, KeyError):
                    reason = "batch_projection_missing"
                else:
                    reason = "session_validation_failed"
                publish_ineligible_session(
                    args.batch_dir, manifest, slot=slot,
                    terminal_state="IntegrityInvalid", reason=reason)
                integrity_failed = True
        terminal = close(args.batch_dir)
        print(json.dumps({"terminal_state": terminal["terminal_state"],
                          "classification": terminal["classification"]}, sort_keys=True))
        return 0 if terminal["terminal_state"] == "Complete" else 1


def self_test() -> None:
    plan = seal_plan(commit="a" * 40, binary_sha256="b" * 64,
                     cases=["mixed/4096/first"],
                     environment_class="wsl_development")
    header = ",".join(FIELDS) + "\n"
    body = [f"mixed/4096/first,{slot},{slot // 17},{slot % 17},{order},1,true,"
            f"mixed,4096,first,{slot},1,40000000,40000000,1,1642,415,409,409,409\n"
            for slot, order in enumerate(plan["schedules"]["mixed/4096/first"])]
    assert validate_session(plan, parse_case_output(
        header + "".join(body), "mixed/4096/first"))["outcome"] == "development_green"
    try:
        parse_case_output(header + "".join(body[:-1]), "mixed/4096/first")
    except HarnessError:
        pass
    else:
        raise AssertionError("short robust output accepted")
    store_self_test()


def abandon_command(args: argparse.Namespace) -> int:
    terminal = close_abandoned(args.batch_dir, args.reason)
    print(json.dumps({"terminal_state": terminal["terminal_state"]}, sort_keys=True))
    return 0


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser()
    commands = result.add_subparsers(dest="command", required=True)
    create = commands.add_parser("create")
    create.add_argument("--root", required=True, type=Path)
    create.add_argument("--binary", required=True, type=Path)
    create.add_argument("--alignment-manifest", required=True, type=Path)
    create.add_argument("--commit", required=True)
    create.add_argument("--cpu", required=True, type=int)
    create.add_argument("--predecessor", type=Path)
    create.add_argument("--repeat-reason", choices=(
        "incomplete_predecessor", "confirmatory_development"))
    create.set_defaults(action=create_command)
    run = commands.add_parser("run")
    run.add_argument("--batch-dir", required=True, type=Path)
    run.set_defaults(action=run_command)
    abandoned = commands.add_parser("close-abandoned")
    abandoned.add_argument("--batch-dir", required=True, type=Path)
    abandoned.add_argument("--reason", required=True, choices=ABANDON_REASONS)
    abandoned.set_defaults(action=abandon_command)
    test = commands.add_parser("self-test")
    test.set_defaults(action=lambda _args: (self_test(), print(
        "[s6c-paired-wallclock-harness] self-test ok"), 0)[2])
    return result


def main() -> int:
    args = parser().parse_args()
    try:
        return args.action(args)
    except (OSError, ValueError, HarnessError, StoreError,
            subprocess.SubprocessError) as error:
        print(f"[s6c-paired-wallclock-harness] rejected: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
