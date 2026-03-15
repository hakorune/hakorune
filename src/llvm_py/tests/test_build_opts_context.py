import os
import unittest

from src.llvm_py.build_opts import resolve_build_options


class TestBuildOptsContext(unittest.TestCase):
    def tearDown(self):
        for key in (
            "HAKO_LLVM_OPT_LEVEL",
            "NYASH_LLVM_OPT_LEVEL",
            "NYASH_LLVM_SANITIZE_EMPTY_PHI",
            "NYASH_LLVM_USE_HARNESS",
            "NYASH_LLVM_SKIP_VERIFY",
            "NYASH_LLVM_FAST",
            "NYASH_LLVM_FAST_IR_PASSES",
        ):
            os.environ.pop(key, None)

    def test_resolve_build_options_reads_opt_and_verify_flags(self):
        os.environ["HAKO_LLVM_OPT_LEVEL"] = "O3"
        os.environ["NYASH_LLVM_SKIP_VERIFY"] = "1"

        opts = resolve_build_options()
        self.assertEqual(opts.opt_level, 3)
        self.assertFalse(opts.verify_ir)
        self.assertFalse(opts.sanitize_empty_phi)
        self.assertFalse(opts.fast_ir_passes)

    def test_resolve_build_options_enables_sanitize_and_fast_paths(self):
        os.environ["NYASH_LLVM_USE_HARNESS"] = "1"
        os.environ["NYASH_LLVM_FAST"] = "1"
        os.environ["NYASH_LLVM_FAST_IR_PASSES"] = "1"

        opts = resolve_build_options()
        self.assertTrue(opts.sanitize_empty_phi)
        self.assertTrue(opts.fast_ir_passes)
        self.assertTrue(opts.verify_ir)


if __name__ == "__main__":
    unittest.main()
