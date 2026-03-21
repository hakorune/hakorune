import unittest

try:
    from cfg.utils import collect_stringish_value_ids
except ImportError:
    from src.llvm_py.cfg.utils import collect_stringish_value_ids


class TestCfgStringishArrayBox(unittest.TestCase):
    def test_arraybox_push_get_propagates_stringish(self):
        blocks = [
            {
                "id": 0,
                "instructions": [
                    {
                        "op": "newbox",
                        "dst": 1,
                        "type": "ArrayBox",
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": {"type": "string", "value": "hello"},
                    },
                    {
                        "op": "mir_call",
                        "dst": 3,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "ArrayBox",
                                "name": "push",
                                "receiver": 1,
                            },
                            "args": [2],
                        },
                    },
                    {
                        "op": "mir_call",
                        "dst": 4,
                        "mir_call": {
                            "callee": {
                                "type": "Method",
                                "box_name": "ArrayBox",
                                "name": "get",
                                "receiver": 1,
                            },
                            "args": [5],
                        },
                    },
                ],
            }
        ]

        stringish = collect_stringish_value_ids(blocks)

        self.assertIn(4, stringish)

    def test_arraybox_boxcall_push_get_propagates_stringish(self):
        blocks = [
            {
                "id": 0,
                "instructions": [
                    {
                        "op": "newbox",
                        "dst": 1,
                        "type": "ArrayBox",
                    },
                    {
                        "op": "const",
                        "dst": 2,
                        "value": {"type": "string", "value": "hello"},
                    },
                    {
                        "op": "boxcall",
                        "dst": 3,
                        "box": 1,
                        "method": "push",
                        "args": [2],
                    },
                    {
                        "op": "boxcall",
                        "dst": 4,
                        "box": 1,
                        "method": "get",
                        "args": [5],
                    },
                ],
            }
        ]

        stringish = collect_stringish_value_ids(blocks)

        self.assertIn(4, stringish)


if __name__ == "__main__":
    unittest.main()
