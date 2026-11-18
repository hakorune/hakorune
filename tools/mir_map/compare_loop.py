#!/usr/bin/env python3
"""Compare two already-built Hakorune CLIs using existing Verify and EXE paths."""
import argparse
from collections import Counter
import json
from pathlib import Path
import re
import sys

from loop_measurement import collect, read_manifest, stream_text
from loop_mir_text import extract, locations, normalize, parse, self_test
from resources import activate
from support import write


def cleaned(status, roots=()):
    text = stream_text(status)
    for root in roots:
        if root is not None:
            text = text.replace(str(root), "$WORK")
    return re.sub(r"nyash_cli_emit_\d+\.json", "nyash_cli_emit_$ID.json", text)


def runtime_observation(row):
    run = row.get("run")
    if not run:
        return None
    return (run["exit"], run["kind"], *(
        Path(run[stream]).read_bytes() for stream in ("stdout", "stderr")))


def compare(before, after, before_root=None, after_root=None):
    phases = ("dump", "compile")
    a = {phase: before[phase]["kind"] for phase in phases}
    b = {phase: after[phase]["kind"] for phase in phases}
    details = dict(before_kinds=a, after_kinds=b)
    if a != b or any(kind in {"timeout", "internal-verification-error", "environment-error"}
                     for kind in [*a.values(), *b.values()]):
        unchanged = a == b and all(
            before[phase]["exit"] == after[phase]["exit"]
            and cleaned(before[phase], [before_root]) == cleaned(after[phase], [after_root])
            for phase in phases if a[phase] != "success"
        )
        return details | {
            "category": "different", "reason": "diagnostic kind or verification",
            "baseline_failure_unchanged": unchanged,
        }
    for phase in phases:
        if a[phase] == "unknown-refusal" and (
            cleaned(before[phase], [before_root]) != cleaned(after[phase], [after_root])
            or before[phase]["exit"] != after[phase]["exit"]
        ):
            return details | {"category": "different", "reason": "changed unclassified diagnostic"}

    lrun, rrun = runtime_observation(before), runtime_observation(after)
    if (lrun is None) != (rrun is None) or (
        lrun is not None and (lrun != rrun or lrun[1] != "executed")
    ):
        return details | {"category": "different", "reason": "execution result or availability"}
    mir_phase = "dump"
    if a["dump"] != "success":
        if lrun is None:
            return details | {"category": "same", "reason": "same refusal kind; no MIR"}
        mir_phase = "executable_mir"
        if not all(row.get(mir_phase, {}).get("kind") == "success" for row in (before, after)):
            return details | {"category": "different", "reason": "could not verify executable MIR"}
    old = extract(stream_text(before[mir_phase]))
    new = extract(stream_text(after[mir_phase]))
    if old is None or new is None:
        return details | {"category": "different", "reason": "verified command omitted MIR text"}
    if old == new:
        return details | {"category": "same"}
    ln, rn = normalize(old), normalize(new)
    same_module = old.splitlines()[0] == new.splitlines()[0]
    if same_module and ln is not None and rn is not None and ln == rn:
        return details | {"category": "same-after-renumbering"}
    try:
        difference = locations(ln, rn) if ln is not None and rn is not None else locations(parse(old), parse(new))
    except ValueError:
        difference = []
    if not same_module:
        difference.insert(0, {"change": "module name", "before": old.splitlines()[0],
                              "after": new.splitlines()[0]})
    return details | {
        "category": "different-execution-confirmed" if lrun is not None else "different-meaning-unconfirmed",
        "locations": difference,
        "location_basis": "normalized" if ln is not None and rn is not None else "printed",
        "location_limit": None if difference else "diff remains in raw MIR logs",
    }


def main():
    if sys.argv[1:] == ["--self-test"]:
        print(json.dumps(self_test(), sort_keys=True))
        return
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--template", type=Path, required=True,
                        help="JSON runtime settings used for both binaries")
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path)
    parser.add_argument("--before-baseline", type=Path,
                        help="reuse a completed collection directory with matching binary/input hashes")
    parser.add_argument("--jobs", type=int, choices=range(1, 5), default=4)
    parser.add_argument("--ast", action="store_true", help="retain parser JSON during collection")
    args = parser.parse_args()
    manifest = read_manifest(args.manifest)
    old_work = args.before_baseline or args.work / "before"
    watch = activate(args.work, "loop-compare")
    old = collect(args.before, manifest, args.template, old_work, args.jobs, args.ast, watch)
    if args.after is None:
        return
    new_work = args.work / "after"
    new = collect(args.after, manifest, args.template, new_work, args.jobs, args.ast, watch)
    results = []
    for lrow, rrow in zip(old, new, strict=True):
        assert lrow["path"] == rrow["path"] and lrow["sha256"] == rrow["sha256"]
        result = compare(lrow, rrow, old_work.resolve(), new_work.resolve())
        results.append({"path": lrow["path"], "groups": lrow["groups"]} | result)
    counts = dict(Counter(row["category"] for row in results))
    write(args.work / "comparison.json", dict(counts=counts, rows=results))
    print(json.dumps(counts, sort_keys=True))
    raise SystemExit(bool(counts.get("different")))


if __name__ == "__main__":
    main()
