//! JoinIR Frontend Debug Test

use std::fs;

fn main() {
    // フィクスチャ読み込み
    let fixture_path = "docs/private/roadmap2/phases/phase-34-joinir-frontend/fixtures/joinir_if_select_simple.program.json";
    let fixture_json = fs::read_to_string(fixture_path)
        .expect("Failed to read fixture JSON");
    let program_json: serde_json::Value = serde_json::from_str(&fixture_json)
        .expect("Failed to parse JSON");

    println!("=== Program JSON ===");
    println!("{}", serde_json::to_string_pretty(&program_json).unwrap());

    // Lowerer 実行
    use nyash_rust::mir::join_ir::frontend::AstToJoinIrLowerer;

    let mut lowerer = AstToJoinIrLowerer::new();
    let join_module = lowerer.lower_program_json(&program_json);

    println!("\n=== JoinIR Module ===");
    println!("Entry: {:?}", join_module.entry);

    for (func_id, func) in &join_module.functions {
        println!("\nFunction {:?}: {}", func_id, func.name);
        println!("  Params: {:?}", func.params);
        println!("  Instructions:");
        for (i, inst) in func.body.iter().enumerate() {
            println!("    {}: {:?}", i, inst);
        }
    }
}
