/*!
 * MIR Printer - Debug output and visualization
 *
 * Implements pretty-printing for MIR modules and functions
 */

use super::printer_helpers;
use super::{BasicBlock, MirFunction, MirInstruction, MirModule, MirType, ValueId};
use crate::debug::log as dlog;
use crate::runtime::get_global_ring0;
use std::collections::BTreeMap;
use std::fmt::Write;

/// MIR printer for debug output and visualization
pub struct MirPrinter {
    /// Indentation level
    #[allow(dead_code)]
    indent_level: usize,

    /// Whether to show detailed information
    verbose: bool,

    /// Whether to show line numbers
    show_line_numbers: bool,

    /// Whether to show per-instruction effect category
    show_effects_inline: bool,
}

impl MirPrinter {
    /// Create a new MIR printer with default settings
    pub fn new() -> Self {
        Self {
            indent_level: 0,
            verbose: false,
            show_line_numbers: true,
            show_effects_inline: false,
        }
    }

    /// Create a verbose MIR printer
    pub fn verbose() -> Self {
        Self {
            indent_level: 0,
            verbose: true,
            show_line_numbers: true,
            show_effects_inline: false,
        }
    }

    /// Set verbose mode
    pub fn set_verbose(&mut self, verbose: bool) -> &mut Self {
        self.verbose = verbose;
        self
    }

    /// Set line number display
    pub fn set_show_line_numbers(&mut self, show: bool) -> &mut Self {
        self.show_line_numbers = show;
        self
    }

    /// Show per-instruction effect category (pure/readonly/side)
    pub fn set_show_effects_inline(&mut self, show: bool) -> &mut Self {
        self.show_effects_inline = show;
        self
    }

    /// Print a complete MIR module
    pub fn print_module(&self, module: &MirModule) -> String {
        let mut output = String::new();

        // Module header
        writeln!(output, "; MIR Module: {}", module.name).unwrap();
        if let Some(ref source) = module.metadata.source_file {
            writeln!(output, "; Source: {}", source).unwrap();
        }
        writeln!(output).unwrap();

        // Module statistics
        if self.verbose {
            let stats = module.stats();
            writeln!(output, "; Module Statistics:").unwrap();
            writeln!(output, ";   Functions: {}", stats.function_count).unwrap();
            writeln!(output, ";   Globals: {}", stats.global_count).unwrap();
            writeln!(output, ";   Total Blocks: {}", stats.total_blocks).unwrap();
            writeln!(
                output,
                ";   Total Instructions: {}",
                stats.total_instructions
            )
            .unwrap();
            writeln!(output, ";   Pure Functions: {}", stats.pure_functions).unwrap();
            writeln!(output).unwrap();
        }

        // Global constants
        if !module.globals.is_empty() {
            writeln!(output, "; Global Constants:").unwrap();
            for (name, value) in &module.globals {
                writeln!(output, "global @{} = {}", name, value).unwrap();
            }
            writeln!(output).unwrap();
        }

        // Functions
        for (_name, function) in &module.functions {
            output.push_str(&self.print_function(function));
            output.push('\n');
        }

        output
    }

    /// Print a single MIR function
    pub fn print_function(&self, function: &MirFunction) -> String {
        let mut output = String::new();

        // Function signature
        write!(
            output,
            "define {} @{}(",
            self.format_type(&function.signature.return_type),
            function.signature.name
        )
        .unwrap();

        for (i, param_type) in function.signature.params.iter().enumerate() {
            if i > 0 {
                write!(output, ", ").unwrap();
            }
            write!(output, "{} %{}", self.format_type(param_type), i).unwrap();
        }
        write!(output, ")").unwrap();

        // Effects
        if !function.signature.effects.is_pure() {
            write!(output, " effects({})", function.signature.effects).unwrap();
        }

        writeln!(output, " {{").unwrap();

        // Function statistics
        if self.verbose {
            let stats = function.stats();
            writeln!(output, "  ; Function Statistics:").unwrap();
            writeln!(output, "  ;   Blocks: {}", stats.block_count).unwrap();
            writeln!(output, "  ;   Instructions: {}", stats.instruction_count).unwrap();
            writeln!(output, "  ;   Values: {}", stats.value_count).unwrap();
            writeln!(output, "  ;   Phi Functions: {}", stats.phi_count).unwrap();
            if stats.is_pure {
                writeln!(output, "  ;   Pure: yes").unwrap();
            }
            // Verbose: highlight MIR26-unified ops presence for snapshotting (TypeOp/WeakRef/Barrier)
            let mut type_check = 0usize;
            let mut type_cast = 0usize;
            let mut weak_new = 0usize;
            let mut weak_load = 0usize;
            let mut barrier_read = 0usize;
            let mut barrier_write = 0usize;
            for block in function.blocks.values() {
                for sp in block.iter_spanned() {
                    match sp.inst {
                        MirInstruction::Throw { .. } => {
                            if dlog::on("NYASH_DEBUG_MIR_PRINTER") {
                                get_global_ring0().log.debug(&format!(
                                    "[PRINTER] found throw in {}",
                                    function.signature.name
                                ));
                            }
                        }
                        MirInstruction::Catch { .. } => {
                            if dlog::on("NYASH_DEBUG_MIR_PRINTER") {
                                get_global_ring0().log.debug(&format!(
                                    "[PRINTER] found catch in {}",
                                    function.signature.name
                                ));
                            }
                        }
                        MirInstruction::TypeOp { op, .. } => match op {
                            super::TypeOpKind::Check => type_check += 1,
                            super::TypeOpKind::Cast => type_cast += 1,
                        },
                        MirInstruction::WeakRef { op, .. } => match op {
                            super::WeakRefOp::New => weak_new += 1,
                            super::WeakRefOp::Load => weak_load += 1,
                        },
                        MirInstruction::Barrier { op, .. } => match op {
                            super::BarrierOp::Read => barrier_read += 1,
                            super::BarrierOp::Write => barrier_write += 1,
                        },
                        _ => {}
                    }
                }
                if let Some(sp) = block.terminator_spanned() {
                    match sp.inst {
                        MirInstruction::Throw { .. } => {
                            if dlog::on("NYASH_DEBUG_MIR_PRINTER") {
                                get_global_ring0().log.debug(&format!(
                                    "[PRINTER] found throw(term) in {}",
                                    function.signature.name
                                ));
                            }
                        }
                        MirInstruction::Catch { .. } => {
                            if dlog::on("NYASH_DEBUG_MIR_PRINTER") {
                                get_global_ring0().log.debug(&format!(
                                    "[PRINTER] found catch(term) in {}",
                                    function.signature.name
                                ));
                            }
                        }
                        MirInstruction::TypeOp { op, .. } => match op {
                            super::TypeOpKind::Check => type_check += 1,
                            super::TypeOpKind::Cast => type_cast += 1,
                        },
                        MirInstruction::WeakRef { op, .. } => match op {
                            super::WeakRefOp::New => weak_new += 1,
                            super::WeakRefOp::Load => weak_load += 1,
                        },
                        MirInstruction::Barrier { op, .. } => match op {
                            super::BarrierOp::Read => barrier_read += 1,
                            super::BarrierOp::Write => barrier_write += 1,
                        },
                        _ => {}
                    }
                }
            }
            if type_check + type_cast > 0 {
                writeln!(
                    output,
                    "  ;   TypeOp: {} (check: {}, cast: {})",
                    type_check + type_cast,
                    type_check,
                    type_cast
                )
                .unwrap();
            }
            if weak_new + weak_load > 0 {
                writeln!(
                    output,
                    "  ;   WeakRef: {} (new: {}, load: {})",
                    weak_new + weak_load,
                    weak_new,
                    weak_load
                )
                .unwrap();
            }
            if barrier_read + barrier_write > 0 {
                writeln!(
                    output,
                    "  ;   Barrier: {} (read: {}, write: {})",
                    barrier_read + barrier_write,
                    barrier_read,
                    barrier_write
                )
                .unwrap();
            }
            if !function.metadata.string_corridor_facts.is_empty() {
                writeln!(output, "  ;   String Corridor Facts:").unwrap();
                for (value, fact) in &function.metadata.string_corridor_facts {
                    writeln!(output, "  ;     %{}: {}", value.0, fact.summary()).unwrap();
                }
            }
            writeln!(output).unwrap();
        }

        // Print blocks in order
        let mut block_ids: Vec<_> = function.blocks.keys().copied().collect();
        block_ids.sort();

        for (i, block_id) in block_ids.iter().enumerate() {
            if let Some(block) = function.blocks.get(block_id) {
                if i > 0 {
                    writeln!(output).unwrap();
                }
                output.push_str(&self.print_basic_block(block, &function.metadata.value_types));
            }
        }

        writeln!(output, "}}").unwrap();

        output
    }

    /// Print a basic block
    pub fn print_basic_block(
        &self,
        block: &BasicBlock,
        types: &BTreeMap<ValueId, MirType>,
    ) -> String {
        // DEBUG: Check span mismatch
        if block.instructions.len() != block.instruction_spans.len() {
            get_global_ring0().log.warn(&format!(
                "[printer/DEBUG] Block {:?} SPAN MISMATCH: instructions={}, spans={}",
                block.id,
                block.instructions.len(),
                block.instruction_spans.len()
            ));
        }

        let mut output = String::new();

        // Block header
        write!(output, "{}:", block.id).unwrap();

        // Predecessors
        if !block.predecessors.is_empty() && self.verbose {
            let preds: Vec<String> = block
                .predecessors
                .iter()
                .map(|p| format!("{}", p))
                .collect();
            write!(output, "  ; preds({})", preds.join(", ")).unwrap();
        }

        writeln!(output).unwrap();

        // Instructions
        for sp in block.all_spanned_instructions() {
            if self.show_line_numbers {
                write!(output, "  {:3}: ", sp.span.line).unwrap();
            } else {
                write!(output, "    ").unwrap();
            }

            let mut line = self.format_instruction(sp.inst, types);
            if self.show_effects_inline {
                let eff = sp.inst.effects();
                let cat = if eff.is_pure() {
                    "pure"
                } else if eff.is_read_only() {
                    "readonly"
                } else {
                    "side"
                };
                line.push_str(&format!("    ; eff: {}", cat));
            }
            writeln!(output, "{}", line).unwrap();
        }

        // Block effects (if verbose and not pure)
        if self.verbose && !block.effects.is_pure() {
            writeln!(output, "    ; effects: {}", block.effects).unwrap();
        }

        output
    }

    /// Format a single instruction
    fn format_instruction(
        &self,
        instruction: &MirInstruction,
        types: &BTreeMap<ValueId, MirType>,
    ) -> String {
        // Delegate to helpers to keep this file lean
        printer_helpers::format_instruction(instruction, types)
    }
    fn format_type(&self, mir_type: &super::MirType) -> String {
        printer_helpers::format_type(mir_type)
    }
}

impl Default for MirPrinter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{
        BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType,
        StringCorridorCarrier, StringCorridorFact, ValueId,
    };

    #[test]
    fn test_empty_module_printing() {
        let module = MirModule::new("test".to_string());
        let printer = MirPrinter::new();

        let output = printer.print_module(&module);

        assert!(output.contains("MIR Module: test"));
        assert!(!output.is_empty());
    }

    #[test]
    fn test_function_printing() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };

        let function = MirFunction::new(signature, BasicBlockId::new(0));
        let printer = MirPrinter::new();

        let output = printer.print_function(&function);

        assert!(output.contains("define void @test_func(i64 %0)"));
        assert!(output.contains("bb0:"));
    }

    #[test]
    fn test_verbose_printing() {
        let module = MirModule::new("test".to_string());
        let printer = MirPrinter::verbose();

        let output = printer.print_module(&module);

        assert!(output.contains("Module Statistics"));
    }

    #[test]
    fn test_verbose_printing_shows_string_corridor_facts() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![MirType::Integer],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.metadata.string_corridor_facts.insert(
            ValueId::new(1),
            StringCorridorFact::str_len(StringCorridorCarrier::CanonicalIntrinsic),
        );
        let printer = MirPrinter::verbose();

        let output = printer.print_function(&function);

        assert!(output.contains("String Corridor Facts"));
        assert!(output.contains("%1: str.len"));
    }
}
