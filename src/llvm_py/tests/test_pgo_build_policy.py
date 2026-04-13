import os
import json
import tempfile
import unittest

from src.llvm_py.builders.pgo_build_policy import (
    PgoBuildPolicy,
    pgo_sidecar_path,
    resolve_pgo_build_policy,
)
from src.llvm_py.builders.ipo_build_policy import resolve_ipo_build_policy
from src.llvm_py.llvm_builder import NyashLLVMBuilder


class TestPgoBuildPolicy(unittest.TestCase):
    def tearDown(self):
        os.environ.pop("NYASH_LLVM_PGO_PHASE", None)
        os.environ.pop("HAKO_LLVM_PGO_PHASE", None)
        os.environ.pop("NYASH_LLVM_PGO_PROFILE", None)
        os.environ.pop("HAKO_LLVM_PGO_PROFILE", None)

    def test_resolve_pgo_build_policy_defaults_off(self):
        self.assertEqual(
            resolve_pgo_build_policy(),
            PgoBuildPolicy(
                phase="off",
                producer="none",
                artifact="none",
                exclusion="allow",
                hotness_feed="none",
            ),
        )

    def test_resolve_pgo_build_policy_generate_derives_profraw_artifact(self):
        os.environ["NYASH_LLVM_PGO_PHASE"] = "generate"

        policy = resolve_pgo_build_policy(output_path="/tmp/demo.o")

        self.assertEqual(policy.phase, "generate")
        self.assertEqual(policy.producer, "instr_ir")
        self.assertEqual(policy.artifact, "/tmp/demo.profraw")
        self.assertEqual(policy.hotness_feed, "none")

    def test_resolve_pgo_build_policy_use_requires_existing_profile(self):
        os.environ["NYASH_LLVM_PGO_PHASE"] = "use"
        with tempfile.TemporaryDirectory() as tmp:
            profile_path = os.path.join(tmp, "demo.profdata")
            with open(profile_path, "wb") as f:
                f.write(b"profile")
            os.environ["NYASH_LLVM_PGO_PROFILE"] = profile_path

            policy = resolve_pgo_build_policy(output_path="/tmp/demo.o")

        self.assertEqual(policy.phase, "use")
        self.assertEqual(policy.producer, "indexed_profdata")
        self.assertEqual(policy.artifact, profile_path)
        self.assertEqual(policy.hotness_feed, "pgo")

    def test_ipo_build_policy_reads_generate_phase(self):
        os.environ["NYASH_LLVM_PGO_PHASE"] = "generate"

        policy = resolve_ipo_build_policy(output_path="/tmp/demo.o")

        self.assertEqual(policy.pgo_mode, "generate")

    def test_pgo_sidecar_path_adds_json_suffix(self):
        policy = PgoBuildPolicy(
            phase="generate",
            producer="instr_ir",
            artifact="/tmp/demo.profraw",
            exclusion="allow",
            hotness_feed="none",
        )
        self.assertEqual(pgo_sidecar_path("/tmp/demo.o", policy), "/tmp/demo.pgo.json")

    def test_compile_to_object_emits_pgo_manifest_when_generate_enabled(self):
        os.environ["NYASH_LLVM_PGO_PHASE"] = "generate"
        builder = NyashLLVMBuilder()
        builder.build_from_mir(
            {
                "functions": [
                    {
                        "name": "main",
                        "params": [],
                        "blocks": [
                            {
                                "id": 0,
                                "instructions": [
                                    {"op": "const", "dst": 0, "value": {"type": "i64", "value": 7}},
                                    {"op": "ret", "value": 0},
                                ],
                            }
                        ],
                    }
                ]
            }
        )

        with tempfile.TemporaryDirectory() as tmp:
            output_path = os.path.join(tmp, "demo.o")
            builder.compile_to_object(output_path)
            manifest_path = os.path.join(tmp, "demo.pgo.json")
            self.assertTrue(os.path.exists(manifest_path))
            with open(manifest_path, "r", encoding="utf-8") as f:
                payload = json.load(f)
            self.assertEqual(payload["phase"], "generate")
            self.assertEqual(payload["artifact"], os.path.join(tmp, "demo.profraw"))


if __name__ == "__main__":
    unittest.main()
