//! Phase 34-7.5: JoinIR Frontend テストヘルパー箱
//!
//! 目的: フィクスチャベースの AST→JoinIR テストを簡潔に書けるようにする

use crate::config::env::joinir_test_debug_enabled;
use crate::mir::join_ir::frontend::AstToJoinIrLowerer;
use crate::mir::join_ir::JoinModule;
use crate::mir::join_ir_ops::JoinValue;
use crate::mir::join_ir_vm_bridge::run_joinir_via_vm;

/// JoinIR Frontend テストランナー箱
///
/// フィクスチャ読み込み → lowering → 実行 → 検証の共通処理を提供
pub struct JoinIrFrontendTestRunner {
    fixture_path: String,
    join_module: Option<JoinModule>,
    debug_enabled: bool,
}

impl JoinIrFrontendTestRunner {
    /// フィクスチャからテストランナーを作成
    pub fn from_fixture(fixture_path: &str) -> Self {
        Self {
            fixture_path: fixture_path.to_string(),
            join_module: None,
            debug_enabled: joinir_test_debug_enabled(),
        }
    }

    /// デバッグモードを有効化（JoinIR Module をダンプ）
    #[allow(dead_code)]
    pub fn with_debug(mut self) -> Self {
        self.debug_enabled = true;
        self
    }

    /// フィクスチャを lowering
    pub fn lower(mut self) -> Result<Self, String> {
        let fixture_json = std::fs::read_to_string(&self.fixture_path)
            .map_err(|e| format!("Failed to read fixture {}: {}", self.fixture_path, e))?;

        let program_json: serde_json::Value = serde_json::from_str(&fixture_json)
            .map_err(|e| format!("Failed to parse JSON {}: {}", self.fixture_path, e))?;

        let mut lowerer = AstToJoinIrLowerer::new();
        let join_module = lowerer.lower_program_json(&program_json);

        if self.debug_enabled {
            self.dump_joinir_module(&join_module);
        }

        self.join_module = Some(join_module);
        Ok(self)
    }

    /// JoinIR Module をダンプ（デバッグ用）
    fn dump_joinir_module(&self, module: &JoinModule) {
        eprintln!("=== JoinIR Module ===");
        eprintln!("Entry: {:?}", module.entry);
        for (func_id, func) in &module.functions {
            eprintln!("\nFunction {:?}: {}", func_id, func.name);
            eprintln!("  Params: {:?}", func.params);
            eprintln!("  Instructions:");
            for (i, inst) in func.body.iter().enumerate() {
                eprintln!("    {}: {:?}", i, inst);
            }
        }
    }

    /// テストケースを実行（単一入力・単一出力）
    pub fn run_case(&self, inputs: &[JoinValue], expected: JoinValue) -> Result<(), String> {
        let module = self
            .join_module
            .as_ref()
            .ok_or("Module not lowered. Call .lower() first")?;

        let result = run_joinir_via_vm(module, module.entry.unwrap(), inputs).map_err(|e| {
            format!(
                "JoinIR execution failed\n\
                 Inputs: {:?}\n\
                 Error: {:?}\n\
                 \n\
                 === JoinIR Module Dump ===\n\
                 {:?}",
                inputs, e, module
            )
        })?;

        if result != expected {
            return Err(format!(
                "Assertion failed\n\
                 Inputs: {:?}\n\
                 Expected: {:?}\n\
                 Actual: {:?}\n\
                 \n\
                 === JoinIR Module Dump ===\n\
                 {:?}",
                inputs, expected, result, module
            ));
        }

        Ok(())
    }

    /// 複数テストケースを一括実行
    pub fn run_cases(&self, cases: &[(Vec<JoinValue>, JoinValue)]) -> Result<(), String> {
        for (inputs, expected) in cases {
            self.run_case(inputs, expected.clone())?;
        }
        Ok(())
    }
}
