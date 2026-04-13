import os
import unittest

from src.llvm_py.builders.pgo_build_policy import PgoBuildPolicy, resolve_pgo_build_policy
from src.llvm_py.builders.ipo_build_policy import resolve_ipo_build_policy


class TestPgoBuildPolicy(unittest.TestCase):
    def tearDown(self):
        os.environ.pop("NYASH_LLVM_PGO_PHASE", None)
        os.environ.pop("HAKO_LLVM_PGO_PHASE", None)

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

    def test_resolve_pgo_build_policy_keeps_generate_disabled_in_scaffold(self):
        os.environ["NYASH_LLVM_PGO_PHASE"] = "generate"

        self.assertEqual(resolve_pgo_build_policy().phase, "off")

    def test_ipo_build_policy_reads_pgo_scaffold_phase(self):
        os.environ["NYASH_LLVM_PGO_PHASE"] = "use"

        policy = resolve_ipo_build_policy()

        self.assertEqual(policy.pgo_mode, "off")


if __name__ == "__main__":
    unittest.main()
