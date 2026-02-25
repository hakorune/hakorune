"""
Phase 132-P1: Function-local context module

Box-First principle: Isolate function-level state to prevent cross-function collisions.
"""

from .function_lower_context import FunctionLowerContext

__all__ = ['FunctionLowerContext']
