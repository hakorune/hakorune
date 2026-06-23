#!/usr/bin/env python3
"""Thin entrypoint for the RegionObserver SlotMetadata Rust-derived artifact."""

from __future__ import annotations

import argparse

from mirbuilder_region_observer_artifacts import run_region_observer_artifact_generator


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    run_region_observer_artifact_generator(check=args.check)


if __name__ == "__main__":
    main()
