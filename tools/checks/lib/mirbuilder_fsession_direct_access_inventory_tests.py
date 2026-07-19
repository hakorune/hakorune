#!/usr/bin/env python3
"""P0 proof for the bounded FSESSION direct-access source scanner."""

from __future__ import annotations

import re
import unittest
from pathlib import Path

import mirbuilder_fsession_direct_access_inventory as inventory


def owner(source: str) -> inventory.ReceiverOwnerIndex:
    clean = inventory.strip_rust_literals_and_comments(source)
    return inventory.ReceiverOwnerIndex(clean, inventory.struct_wrapper_kinds(clean), frozenset({"factory"}))


class DirectAccessInventoryP0Tests(unittest.TestCase):
    def test_lexer_ignores_literals_comments_and_preserves_lifetime(self) -> None:
        source = """
// builder.type_ctx
/* nested /* builder.variable_ctx */ builder.binding_ctx */
let regular = "builder.current_block";
let raw = r#"builder.scope_ctx"#;
let byte = b"builder.comp_ctx";
let c_text = c"builder.metadata_ctx";
let character = 'x';
let borrowed: &'a str = value;
"""
        clean = inventory.strip_rust_literals_and_comments(source)
        self.assertNotIn("builder.type_ctx", clean)
        self.assertNotIn("builder.variable_ctx", clean)
        self.assertNotIn("builder.current_block", clean)
        self.assertIn("&'a str", clean)

    def test_cfg_domains_are_bounded_and_preserve_source_offsets(self) -> None:
        source = """
fn production() { builder.type_ctx; }
#[cfg(test)] fn unit() { builder.variable_ctx; }
#[cfg(all(test, feature = "x"))] fn gated_unit() { builder.binding_ctx; }
#[cfg(not(test))] fn release() { builder.scope_ctx; }
#[cfg(any(test, feature = "x"))] fn shared() { builder.comp_ctx; }
#[test] fn direct_test() { builder.metadata_ctx; }
"""
        partitions = inventory.partition_cfg_items(source)
        self.assertEqual(set(partitions), {"production", "test", "shared"})
        self.assertTrue(all(len(partition) == len(source) for partition in partitions.values()))
        self.assertIn("production", partitions["production"])
        self.assertIn("unit", partitions["test"])
        self.assertIn("gated_unit", partitions["test"])
        self.assertIn("release", partitions["production"])
        self.assertIn("shared", partitions["shared"])
        self.assertIn("direct_test", partitions["test"])

    def test_receiver_owner_grammar_accepts_only_structural_builder_carriers(self) -> None:
        source = """
struct MirBuilder;
impl MirBuilder { fn direct(&mut self) { self.type_ctx; } }
struct Wrapper { builder: MirBuilder }
impl Wrapper { fn wrapped(&mut self) { self.builder.variable_ctx; } }
struct Tuple(MirBuilder);
impl Tuple { fn tupled(&mut self) { self.0.binding_ctx; } }
struct Other;
impl Other { fn foreign(&mut self) { self.type_ctx; } }
fn parameter(builder: &mut MirBuilder) { builder.current_block; }
fn local() { let mut builder = MirBuilder::new(); builder.pending_phis; }
fn factory() -> MirBuilder { MirBuilder::new() }
fn factory_local() { let builder = factory(); builder.local_ssa_map; }
fn wrapper_local() { let fixture = Wrapper::new(); fixture.builder.schedule_mat_map; }
fn unknown(other: &mut Other) { other.builder.pin_slot_names; }
"""
        clean = inventory.strip_rust_literals_and_comments(source)
        index = inventory.ReceiverOwnerIndex(clean, inventory.struct_wrapper_kinds(clean), frozenset({"factory"}))

        def disposition(fragment: str) -> str:
            position = clean.index(fragment)
            receiver = re.match(inventory.RECEIVER, clean[position:]).group("receiver")
            return index.classify(receiver, position)

        self.assertEqual(disposition("self.type_ctx"), "accept")
        self.assertEqual(disposition("self.builder.variable_ctx"), "accept")
        self.assertEqual(disposition("self.0.binding_ctx"), "accept")
        self.assertEqual(disposition("builder.current_block"), "accept")
        self.assertEqual(disposition("builder.pending_phis"), "accept")
        self.assertEqual(disposition("builder.local_ssa_map"), "accept")
        self.assertEqual(disposition("fixture.builder.schedule_mat_map"), "accept")
        self.assertEqual(disposition("other.builder.pin_slot_names"), "unknown")

    def test_route_patterns_assign_mixed_api_families_once(self) -> None:
        source = """
impl MirBuilder {
    fn mixed(&mut self) {
        self.scope_ctx.push_lexical_scope();
        self.scope_ctx.clear_for_function_entry();
        self.comp_ctx.reserve_value_id(value);
        self.comp_ctx.propagate_record_local_value_from_phi(dst, src);
        self.metadata_ctx.record_value_span(value, span);
        self.metadata_ctx.value_origin_callers(value);
    }
}
"""
        clean = inventory.strip_rust_literals_and_comments(source)
        index = owner(source)
        hits: set[str] = set()
        for selector, patterns in inventory.ROUTE_PATTERNS.items():
            for pattern in map(re.compile, patterns):
                for match in pattern.finditer(clean):
                    if index.classify(match.group("receiver"), match.start("receiver")) == "accept":
                        hits.add(selector)
        self.assertEqual(
            hits,
            {
                "scope.lexical_scope_stack",
                "scope.entry_clear",
                "compilation.reserved_value_ids",
                "compilation.record_local_values",
                "value_origins.spans",
                "value_origins.callers",
            },
        )

    def test_snapshot_contract_is_complete_and_census_owned(self) -> None:
        routes = inventory.census_routes()
        observed = inventory.observe(routes)
        self.assertEqual(len(observed), len(routes) * 3)
        self.assertEqual([(row["selector"], row["domain"]) for row in observed], sorted((row["selector"], row["domain"]) for row in observed))
        self.assertTrue(all(row["files"] == sorted(set(row["files"])) for row in observed))
        self.assertTrue(all(row["destination"] == routes[row["selector"]]["destination"] for row in observed))


if __name__ == "__main__":
    unittest.main()
