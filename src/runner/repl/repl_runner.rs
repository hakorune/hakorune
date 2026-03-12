//! ReplRunnerBox - REPL execution engine
//!
//! Box-First Design: Complete isolation of REPL execution logic
//! Phase 288 P1-P3
//!
//! Responsibilities:
//! - REPL loop management (.exit, .help, .reset commands)
//! - Line evaluation (parse → compile → execute)
//! - Session state management via ReplSessionBox

use super::repl_session::ReplSessionBox;
use crate::backend::VMValue; // Phase 288.1: For auto-display
use crate::cli::CliConfig;
use std::cell::RefCell;
use std::rc::Rc;

/// Phase 288: REPL実行器（箱理論モジュール化）
/// Phase 288.1: session を Rc<RefCell<>> で保持（永続化のため）
pub(super) struct ReplRunnerBox {
    #[allow(dead_code)]
    config: CliConfig,
    /// Phase 288.1: Rc<RefCell<>> で保持（clone は Rc のみ、中身は永続化）
    session: Rc<RefCell<ReplSessionBox>>,
    /// REPL mode での内部ログ抑制フラグ
    /// verbose が false の場合に true（REPL 専用）
    quiet_internal_logs: bool,
}

impl ReplRunnerBox {
    pub(super) fn new(config: CliConfig) -> Self {
        // REPL mode では verbose が false なら内部ログを抑制
        let quiet_internal_logs = !crate::config::env::cli_verbose();

        Self {
            config,
            // Phase 288.1: Rc<RefCell<>> で初期化（即座に生成）
            session: Rc::new(RefCell::new(ReplSessionBox::new())),
            quiet_internal_logs,
        }
    }

    /// REPL ループ（メインエントリーポイント）
    pub(super) fn run(&self) -> ! {
        use std::io::{self, Write};

        println!("Nyash REPL v1.0 - Phase 288 MVP");
        println!("Type .help for commands, .exit to quit");

        let stdin = io::stdin();
        let mut line_buf = String::new();

        loop {
            print!(">>> ");
            io::stdout().flush().unwrap();

            line_buf.clear();
            match stdin.read_line(&mut line_buf) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    let line = line_buf.trim();

                    // REPL commands
                    match line {
                        ".exit" | ".quit" => break,
                        ".help" => {
                            println!("Commands:");
                            println!("  .exit / .quit - Exit REPL");
                            println!("  .reset - Clear session");
                            println!("  .help - Show this help");
                            continue;
                        }
                        ".reset" => {
                            // Phase 288.1: Reset session
                            self.session.borrow_mut().reset();
                            println!("Session reset");
                            continue;
                        }
                        "" => continue,
                        _ => {}
                    }

                    // Evaluate line
                    match self.eval_line(line) {
                        Ok(result) => {
                            if !result.is_empty() {
                                println!("{}", result);
                            }
                        }
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Err(e) => {
                    eprintln!("Input error: {}", e);
                    break;
                }
            }
        }

        println!("Goodbye!");
        std::process::exit(0);
    }

    /// 1行評価（内部メソッド）
    fn eval_line(&self, line: &str) -> Result<String, String> {
        use crate::backend::mir_interpreter::MirInterpreter;
        use crate::mir::MirCompiler;
        use crate::parser::NyashParser;

        // REPL mode では内部デバッグログを自動抑制
        // （quiet_internal_logs フラグで制御、環境変数操作不要）

        // Phase 288.1: No lazy initialization needed (session already created in new())

        // Parse (minimal wrapper for REPL context - use Main for VM entry point)
        let code = format!("static box Main {{ main() {{ {} }} }}", line);
        let ast =
            NyashParser::parse_from_string(&code).map_err(|e| format!("Parse error: {}", e))?;

        // Phase 288.1: Check if wrapper AST is a pure expression (for auto-display)
        use super::ast_rewriter::ReplAstRewriter;
        let is_expression = ReplAstRewriter::is_pure_expression(&ast);

        // Phase 288.1: REPL AST rewrite (session variable bridge)
        let rewritten_ast = ReplAstRewriter::rewrite(ast);

        // Compile with REPL mode flag (暗黙 local 許可)
        let mut compiler = MirCompiler::new();
        compiler.set_repl_mode(true);

        // MirCompiler に quiet フラグを渡す
        compiler.set_quiet_internal_logs(self.quiet_internal_logs);

        // Phase 288.1: Use rewritten AST for compilation
        let mir_result = compiler
            .compile_with_source(rewritten_ast, Some("<repl>"))
            .map_err(|e| format!("Compile error: {}", e))?;

        // Phase 288.1: Set REPL session in VM (Rc clone, not inner session clone)
        let mut vm = MirInterpreter::new();
        vm.set_repl_session(self.session.clone());

        // Execute
        let result_box = vm
            .execute_module(&mir_result.module)
            .map_err(|e| format!("Runtime error: {}", e))?;

        // Phase 288.1: Convert to VMValue and store in session
        let vm_value = VMValue::from_nyash_box(result_box);

        // Phase 288.1: Auto-display logic
        let display_output = if is_expression {
            match &vm_value {
                VMValue::Void => String::new(), // Void は表示しない
                value => {
                    // `_` 変数に保存
                    self.session
                        .borrow_mut()
                        .set("_".to_string(), value.clone());
                    Self::format_vm_value(value)
                }
            }
        } else {
            String::new()
        };

        // Phase 288.1: Update session metadata
        {
            let mut session = self.session.borrow_mut();
            session.set_last_value(vm_value);
            session.eval_count += 1;
        }

        Ok(display_output)
    }

    /// Phase 288.1: Format VMValue for auto-display
    fn format_vm_value(value: &VMValue) -> String {
        match value {
            VMValue::Integer(i) => i.to_string(),
            VMValue::Float(f) => f.to_string(),
            VMValue::Bool(b) => b.to_string(),
            VMValue::String(s) => s.clone(),
            VMValue::Void => String::new(),
            _ => format!("{:?}", value),
        }
    }
}
