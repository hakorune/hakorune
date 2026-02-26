import unittest

try:
    from cfg.utils import collect_integerish_value_ids
except ImportError:
    from src.llvm_py.cfg.utils import collect_integerish_value_ids


class TestCfgIntegerish(unittest.TestCase):
    def test_mir_call_length_is_integerish(self):
        blocks = [
            {
                "id": 0,
                "instructions": [
                    {
                        "op": "mir_call",
                        "dst": 10,
                        "mir_call": {
                            "callee": {"type": "Method", "name": "length"},
                            "args": [1],
                        },
                    }
                ],
            }
        ]
        integerish = collect_integerish_value_ids(blocks)
        self.assertIn(10, integerish)


if __name__ == "__main__":
    unittest.main()
