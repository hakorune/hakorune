import unittest

from src.llvm_py.instructions.plugin_invoke_args import build_plugin_invoke_args


class TestPluginInvokeArgs(unittest.TestCase):
    def test_build_plugin_invoke_args_clamps_and_boxes_values(self):
        boxed = []

        def resolve_arg(vid):
            return f"raw{vid}"

        def ensure_handle(value):
            boxed.append(value)
            return f"boxed:{value}"

        argc, a1, a2 = build_plugin_invoke_args(
            args=[1, 2, 3],
            resolve_arg=resolve_arg,
            ensure_handle=ensure_handle,
            argc_cap=2,
        )

        self.assertEqual(argc.constant, 2)
        self.assertEqual(a1, "boxed:raw1")
        self.assertEqual(a2, "boxed:raw2")
        self.assertEqual(boxed, ["raw1", "raw2"])

    def test_build_plugin_invoke_args_defaults_missing_args_to_zero(self):
        calls = []

        def resolve_arg(vid):
            calls.append(vid)
            return None

        argc, a1, a2 = build_plugin_invoke_args(
            args=[],
            resolve_arg=resolve_arg,
            ensure_handle=lambda value: value,
        )

        self.assertEqual(argc.constant, 0)
        self.assertEqual(a1.constant, 0)
        self.assertEqual(a2.constant, 0)
        self.assertEqual(calls, [])


if __name__ == "__main__":
    unittest.main()
