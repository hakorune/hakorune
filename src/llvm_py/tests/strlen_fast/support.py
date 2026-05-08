import os
import unittest

from src.llvm_py.llvm_builder import NyashLLVMBuilder


class StrlenFastTestCase(unittest.TestCase):
    def setUp(self):
        os.environ['NYASH_LLVM_FAST'] = '1'

    def tearDown(self):
        os.environ.pop('NYASH_LLVM_FAST', None)
