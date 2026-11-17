use super::*;
use crate::mir::{BinaryOp, Callee, ConstValue, MirInstruction, MirModule, MirType, MirVerifier};
use crate::parser::NyashParser;

fn prepared(code: &str) -> (ResolvedModule, MirModule, Vec<lower::Correspondence>) {
    let source = resolve(NyashParser::parse_from_string(code).unwrap()).unwrap();
    let (candidate, records) = lower_and_seal(&source).unwrap();
    MirVerifier::new_strict()
        .verify_document_module(&candidate)
        .unwrap();
    correspondence::verify(&source, &candidate, &records).unwrap();
    (source, candidate, records)
}
#[test]
fn annotated_scalar_source_resolves_before_any_mir_exists() {
    let (source, _, _) = prepared("static box Main { main(): i64 { return 1 + 2 } }");
    assert_eq!(source.declarations.entries.len(), 1);
    assert_eq!(source.functions.len(), 1);
    let invalid = [
        (
            "static box Main { main() { return 1 } }",
            "MissingTypeAnnotation",
        ),
        (
            "static box Main { main(): i64 { return true } }",
            "TypeMismatch",
        ),
        (
            "static box Main { main(): i64 { return missing } }",
            "UnknownName",
        ),
        (
            "function f(x: i64): i64 { return x }\nstatic box Main { main(): i64 { return f() } }",
            "ArityMismatch",
        ),
        (
            "static box Main { main(): i64 { return \"outside\" } }",
            "UnsupportedExpression",
        ),
        (
            "static box Main { main(): i64 { local x = 3\nreturn x / 0 } }",
            "UnsupportedDivisionContract",
        ),
        (
            "static box Main { main(): i64 { loop i in 0..2 { i = 1 }\nreturn 0 } }",
            "ReadonlyBinding",
        ),
    ];
    for (code, expected) in invalid {
        // Resolve returns only an error. No MIR allocator or Lower is invoked.
        let ast = NyashParser::parse_from_string(code).unwrap();
        assert_eq!(resolve(ast).unwrap_err().code, expected, "{code}");
    }
}
const MUTATION_SOURCE: &str = r#"
static box Main {
 a(x: i64, y: i64): i64 { print(x)
 return x - y }
 b(x: i64, y: i64): i64 { print(x)
 return y - x }
 tick(): i64 { print(7)
 return 7 }
 main(): i64 {
 local sum = 4 + 5
 local out = me.a(sum, 2)
 if out > 0 { me.tick() }
 local n = 0
 loop(n < 2) { n += 1 }
 return out + n
 }
}
"#;
fn main_symbol(source: &ResolvedModule) -> String {
    source.declarations.entries[source.entry.0]
        .key
        .mir_symbol_projection()
}
fn instruction(
    function: &crate::mir::MirFunction,
    predicate: impl Fn(&MirInstruction) -> bool,
) -> (crate::mir::BasicBlockId, usize) {
    function
        .blocks
        .iter()
        .flat_map(|(block, body)| {
            body.instructions
                .iter()
                .enumerate()
                .map(move |(index, row)| (*block, index, row))
        })
        .find(|(_, _, row)| predicate(row))
        .map(|(block, index, _)| (block, index))
        .unwrap()
}
#[test]
fn correspondence_rejects_eight_semantic_mutations_from_finished_mir() {
    for mutation in 0..8 {
        let (source, mut candidate, records) = prepared(MUTATION_SOURCE);
        let symbol = main_symbol(&source);
        let function = candidate.functions.get_mut(&symbol).unwrap();
        match mutation {
            0 => {
                let (b, i) = instruction(function, |op| {
                    matches!(
                        op,
                        MirInstruction::BinOp {
                            op: BinaryOp::Add,
                            ..
                        }
                    )
                });
                let MirInstruction::BinOp { op, .. } =
                    &mut function.blocks.get_mut(&b).unwrap().instructions[i]
                else {
                    unreachable!()
                };
                *op = BinaryOp::Sub;
            }
            1 => {
                let (b, i) = instruction(
                    function,
                    |op| matches!(op,MirInstruction::Call(call)if call.args.len()==2),
                );
                let MirInstruction::Call(call) =
                    &mut function.blocks.get_mut(&b).unwrap().instructions[i]
                else {
                    unreachable!()
                };
                call.args.swap(0, 1);
            }
            2 => {
                let (b, i) = instruction(
                    function,
                    |op| matches!(op,MirInstruction::Call(call)if call.args.is_empty()),
                );
                let body = function.blocks.get_mut(&b).unwrap();
                body.instructions.remove(i);
                body.instruction_spans.remove(i);
            }
            3 => {
                let (b, i) = instruction(
                    function,
                    |op| matches!(op,MirInstruction::Call(call)if call.args.is_empty()),
                );
                let mut duplicate = function.blocks[&b].instructions[i].clone();
                let fresh = function.next_value_id();
                let MirInstruction::Call(call) = &mut duplicate else {
                    unreachable!()
                };
                call.dst = Some(fresh);
                function
                    .metadata
                    .value_types
                    .insert(fresh, MirType::Integer);
                function
                    .blocks
                    .get_mut(&b)
                    .unwrap()
                    .add_instruction(duplicate);
            }
            4 => {
                let key = source
                    .declarations
                    .entries
                    .iter()
                    .find(|d| d.key.name() == "b")
                    .unwrap()
                    .key
                    .clone();
                let (b, i) = instruction(
                    function,
                    |op| matches!(op,MirInstruction::Call(call)if call.args.len()==2),
                );
                let MirInstruction::Call(call) =
                    &mut function.blocks.get_mut(&b).unwrap().instructions[i]
                else {
                    unreachable!()
                };
                call.callee = Callee::Global(key.canonical_global_target_v1().unwrap());
            }
            5 => {
                let (b, i) = instruction(
                    function,
                    |op| matches!(op,MirInstruction::Call(call)if call.args.is_empty()),
                );
                let body = function.blocks.get_mut(&b).unwrap();
                let moved = body.instructions.remove(i);
                body.instruction_spans.remove(i);
                function
                    .blocks
                    .get_mut(&function.entry_block)
                    .unwrap()
                    .add_instruction(moved);
            }
            6 => {
                let body = function
                    .blocks
                    .values_mut()
                    .find(|body| matches!(body.terminator, Some(MirInstruction::Branch { .. })))
                    .unwrap();
                let Some(MirInstruction::Branch {
                    then_bb, else_bb, ..
                }) = &mut body.terminator
                else {
                    unreachable!()
                };
                std::mem::swap(then_bb, else_bb);
            }
            7 => {
                let image = records[source.entry.0].loops.values().next().unwrap();
                let phi = function.blocks[&image.header]
                    .instructions
                    .iter()
                    .find_map(|row| match row {
                        MirInstruction::Phi { inputs, .. } => Some(inputs),
                        _ => None,
                    })
                    .unwrap();
                let predecessor = phi[1].0;
                function
                    .blocks
                    .get_mut(&predecessor)
                    .unwrap()
                    .set_terminator(MirInstruction::Jump {
                        target: image.exit,
                        edge_args: None,
                    });
            }
            _ => unreachable!(),
        }
        function.update_cfg();
        if mutation != 7 {
            MirVerifier::new_strict()
                .verify_document_module(&candidate)
                .unwrap();
        }
        let rejected = correspondence::verify(&source, &candidate, &records).unwrap_err();
        assert!(
            rejected.starts_with("blueprint/internal/correspondence/"),
            "mutation {mutation}: {rejected}"
        );
    }
}

#[test]
fn nested_control_and_short_circuit_preserve_actual_phi_predecessors() {
    let code = r#"
function yes(x: i64): Bool { return x > 0 }
static box Main {
 main(): i64 {
 local sum = 0
 local i = 0
 loop(i < 3 && yes(1)) {
  i += 1
  loop j in 0..3 {
   if j == 1 { continue }
   if i == 2 && yes(j) { break }
   sum += i + j
  }
 }
 return sum
 }
}
"#;
    prepared("static box Main { main(): i64 { local x = 1\nif true { local x = 2\nif true { print(x) } }\nreturn x } }");
    let (_, candidate, _) = prepared(code);
    assert!(candidate
        .functions
        .values()
        .flat_map(|f| f.blocks.values())
        .any(|b| b.instructions.iter().any(|i| matches!(
            i,
            MirInstruction::Phi {
                type_hint: Some(MirType::Bool),
                ..
            }
        ))));
}

#[test]
fn loop_headers_only_carry_writes_and_reject_a_missing_write_contract() {
    let code = "function count(n: i64): i64 {\nlocal i = 0\nloop(i < n) { i += 1 }\nreturn i + n\n}\nstatic box Main { main(): i64 { return count(3) } }";
    let (mut source, candidate, _) = prepared(code);
    let count = candidate.functions.get("count/1").unwrap();
    assert_eq!(
        count
            .blocks
            .values()
            .flat_map(|b| &b.instructions)
            .filter(|op| matches!(op, MirInstruction::Phi { .. }))
            .count(),
        1
    );
    let body = source
        .functions
        .iter_mut()
        .find(|f| {
            f.body
                .iter()
                .any(|s| matches!(s.kind, declaration_table::StatementKind::Loop(_)))
        })
        .unwrap();
    let contract = body
        .body
        .iter_mut()
        .find_map(|s| match &mut s.kind {
            declaration_table::StatementKind::Loop(contract) => Some(contract),
            _ => None,
        })
        .unwrap();
    // Simulate an incomplete write contract. Its lowered CFG remains valid,
    // but c must detect a changed invariant on the backedge independently.
    contract.assigned.clear();
    let (candidate, records) = lower_and_seal(&source).unwrap();
    MirVerifier::new_strict()
        .verify_document_module(&candidate)
        .unwrap();
    assert!(correspondence::verify(&source, &candidate, &records).is_err());
}
