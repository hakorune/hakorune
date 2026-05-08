#!/usr/bin/env python3
import unittest

from src.llvm_py.tests.strlen_fast.array_routes import TestStrlenFastArrayRoutes
from src.llvm_py.tests.strlen_fast.concat_routes import TestStrlenFastConcatRoutes
from src.llvm_py.tests.strlen_fast.core_routes import TestStrlenFastCore
from src.llvm_py.tests.strlen_fast.string_flow import TestStrlenFastStringFlow


if __name__ == '__main__':
    unittest.main()
