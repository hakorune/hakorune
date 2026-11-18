#!/usr/bin/env python3
"""Compare two already-built Hakorune CLIs using existing Verify and EXE paths."""
import argparse
from collections import Counter
import json
from pathlib import Path
import re
import sys
import time

from comparison_policy import (MAX_JOBS, auto_jobs, can_reuse_exe, downstream_unchanged,
                               eligible_rows, env_fingerprint, exact_mir, summarize_skip)
from loop_measurement import (artifact_hashes, collect, measurement_configuration,
                              read_manifest, stream_text)
from loop_mir_text import extract, locations, normalize, parse, self_test
from resources import activate, available_memory_kib
from support import CORPUS_MANIFEST, sha, write


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


def diagnostic_fragments(status, roots):
    text = cleaned(status, roots).replace('❌ MIR compilation error: Control flow error',
                                         'Control flow error')
    # The scalar verifier can print the first unmatched block from a map.
    # Retain the refusal, function, counts and operation; the witness IDs can
    # vary on repeated invocations of the same binary with the same input.
    text = re.sub(
        r'(\[published-lifecycle-program/scalar-source-coverage\][^\n]*'
        r'unexpected=Some\(\(BasicBlockId\()\d+(\), BinOp \{ dst: ValueId\()\d+'
        r'(\), op: \w+, lhs: ValueId\()\d+(\), rhs: ValueId\()\d+',
        r'\g<1>_\g<2>_\g<3>_\g<4>_', text,
    )
    return sorted(re.split(r'\n|; (?=Control flow error in block )', text))


def compare(before, after, before_root=None, after_root=None):
    phases = ("dump", "compile")
    a = {phase: before[phase]["kind"] for phase in phases}
    b = {phase: after[phase]["kind"] for phase in phases}
    details = dict(before_kinds=a, after_kinds=b)
    lrun, rrun = runtime_observation(before), runtime_observation(after)
    if any(before[phase].get(field) != after[phase].get(field)
           for phase in phases for field in ("exit", "timeout", "verified", "emitted")):
        return details | {"category": "different", "reason": "exit, timeout or emitted artifact"}
    if a != b or any(kind in {"timeout", "internal-verification-error", "environment-error"}
                     for kind in [*a.values(), *b.values()]):
        unchanged = a == b and lrun == rrun and all(
            before[phase]["exit"] == after[phase]["exit"]
            and diagnostic_fragments(before[phase], [before_root]) == diagnostic_fragments(after[phase], [after_root])
            for phase in phases if a[phase] != "success"
        )
        compared_mir = next(
            (phase for phase in ("dump", "executable_mir")
             if all(row.get(phase, {}).get("kind") == "success" for row in (before, after))),
            None,
        )
        changes = []
        if compared_mir:
            old, new = (extract(stream_text(row[compared_mir])) for row in (before, after))
            if old != new:
                unchanged = False
                if old is not None and new is not None:
                    try:
                        changes = locations(parse(old), parse(new))
                    except ValueError:
                        pass
        return details | {
            "category": "same-baseline-failure" if unchanged else "different", "reason": "diagnostic kind or verification",
            "baseline_failure_unchanged": unchanged, "runtime_result_changed": lrun != rrun,
            "locations": changes,
        }
    for phase in phases:
        if a[phase] == "unknown-refusal" and (
            diagnostic_fragments(before[phase], [before_root]) != diagnostic_fragments(after[phase], [after_root])
            or before[phase]["exit"] != after[phase]["exit"]
        ):
            return details | {"category": "different", "reason": "changed unclassified diagnostic"}

    if lrun != rrun:
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
        return details | {"category": "same-baseline-failure" if lrun and lrun[1] != "executed" else "same"}
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


def test_comparison():
    import subprocess
    from tempfile import TemporaryDirectory

    with TemporaryDirectory() as folder:
        root = Path(folder)
        def git(*args):
            return subprocess.check_output(['git', '-C', folder, *args], text=True).strip()
        def commit(path):
            p = root / path
            p.parent.mkdir(parents=True, exist_ok=True)
            p.write_text('edited')
            git('add', path)
            git('-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-qm', path)
            return git('rev-parse', 'HEAD')
        git('init', '-q')
        before = commit('docs/example.md')
        after = commit('apps/tests/corpus/manifest.json')
        assert downstream_unchanged(before, after, root)
        assert not downstream_unchanged(before, 'unresolvable-ref', root)
        untracked = root / 'untracked.hako'
        untracked.write_text('untracked')
        assert not downstream_unchanged(before, after, root)
        untracked.unlink()
        dependency = commit('apps/tests/corpus/dependency.hako')
        assert not downstream_unchanged(after, dependency, root)
        runtime = commit('crates/runtime/example.rs')
        assert not downstream_unchanged(dependency, runtime, root)
        assert not downstream_unchanged(before, after, root)
    return self_test(exact_mir) | {"downstream_edit_blocks_skip": True, "unknown_revision_blocks_skip": True,
                                   "wrong_checkout_blocks_skip": True, "untracked_input_blocks_skip": True,
                                   "possible_dependency_edit_blocks_skip": True}


def main():
    if sys.argv[1:] == ["--self-test"]:
        print(json.dumps(test_comparison(), sort_keys=True))
        return
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, default=CORPUS_MANIFEST)
    parser.add_argument("--all-inputs", action="store_true", help="include the future coverage group")
    parser.add_argument("--template", type=Path, required=True,
                        help="JSON runtime settings used for both binaries")
    parser.add_argument("--work", type=Path, required=True)
    parser.add_argument("--before", type=Path, required=True)
    parser.add_argument("--after", type=Path)
    parser.add_argument("--before-baseline", type=Path,
                        help="reuse a completed collection directory with matching binary/input hashes")
    parser.add_argument("--jobs", type=int, choices=range(1, MAX_JOBS + 1))
    parser.add_argument("--rss-log", type=Path, help="command peak RSS measurements from a prior full run")
    parser.add_argument("--before-commit", help="git base revision for omission safety")
    parser.add_argument("--after-commit", help="git comparison revision for omission safety")
    parser.add_argument("--no-exe-skip", action="store_true", help="force full EXE generation and execution")
    parser.add_argument("--ast", action="store_true", help="retain parser JSON during collection")
    args = parser.parse_args()
    manifest = [row for row in read_manifest(args.manifest)
                if args.all_inputs or row.get('cadence') != 'future']
    old_work = args.before_baseline or args.work / "before"
    watch = activate(args.work, "loop-compare", max_children=args.jobs or MAX_JOBS)
    rss_log = args.rss_log
    peak = None
    if args.jobs is None:
        if rss_log is None and args.before_baseline is not None:
            previous = args.before_baseline.parent / "commands.jsonl"
            if previous.is_file():
                rss_log = previous
        if rss_log is None:
            # Measure both routes for a few large inputs plus a deterministic
            # spread of the selected manifest before setting the concurrency.
            ordered = sorted(manifest, key=lambda row: Path(row['path']).stat().st_size, reverse=True)
            sample = {row['path']: row for row in ordered[:6] + manifest[::max(1, len(manifest)//6)]}
            collect(args.before, list(sample.values()), args.template, args.work / "rss-pilot", 1, watch=watch)
            rss_log = args.work / "commands.jsonl"
        jobs, peak = auto_jobs(rss_log, available_memory_kib())
    else:
        jobs = args.jobs
    print(json.dumps({"jobs": jobs, "sample_peak_rss_kib": peak}), flush=True)

    start = time.monotonic()
    old = collect(args.before, manifest, args.template, old_work, jobs, args.ast, watch)
    before_seconds = time.monotonic() - start
    if args.after is None:
        write(args.work / "collection-summary.json", dict(jobs=jobs, seconds=before_seconds, rows=len(old)))
        return
    new_work = args.work / "after"
    exe_baseline = None
    skip_policy = "disabled by request"
    if not args.no_exe_skip:
        skip_policy = "git downstream change or unverified revision"
        if downstream_unchanged(args.before_commit, args.after_commit):
            env, _, artifact_paths = measurement_configuration(args.after.resolve(), args.template, new_work)
            hashes = artifact_hashes(artifact_paths)
            roles = {name: hashes[str(path)] for name, path in artifact_paths.items()}
            after_identity = dict(cli_sha256=roles['compiler'], artifact_roles=roles,
                                  execution_env=env_fingerprint(env), artifacts=hashes)
            before_identity = json.loads((old_work / "identity.json").read_text())
            skip_policy = "closed EXE route identity not established"
            if can_reuse_exe(before_identity, after_identity):
                exe_baseline = eligible_rows(old), sha(old_work / "results.json")
                skip_policy = "reuse existing execution only on identical verified MIR"
    start = time.monotonic()
    new = collect(args.after, manifest, args.template, new_work, jobs, args.ast, watch,
                  exe_baseline=exe_baseline)
    after_seconds = time.monotonic() - start
    results = []
    for lrow, rrow in zip(old, new, strict=True):
        assert lrow["path"] == rrow["path"] and lrow["sha256"] == rrow["sha256"]
        left_logs = Path(lrow["dump"]["stdout"]).parents[2]
        right_logs = Path(rrow["dump"]["stdout"]).parents[2]
        result = compare(lrow, rrow, left_logs, right_logs)
        if rrow.get("exe_skipped"):
            result["exe_skipped"] = rrow["exe_skipped"]
        results.append({"path": lrow["path"], "groups": lrow["groups"]} | result)
    counts = dict(Counter(row["category"] for row in results))
    skips = summarize_skip(new)
    summary = dict(counts=counts, jobs=jobs, sample_peak_rss_kib=peak, skipped_exe=sum(skips.values()),
                   skip_policy=skip_policy, before_seconds=before_seconds, after_seconds=after_seconds)
    write(args.work / "comparison.json", summary | dict(rows=results))
    print(json.dumps(summary, sort_keys=True))
    raise SystemExit(any(category.startswith("different") for category in counts))


if __name__ == "__main__":
    main()
