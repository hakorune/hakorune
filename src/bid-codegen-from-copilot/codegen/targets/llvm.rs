/*!
 * LLVM Target Generator - Generate LLVM IR declarations
 *
 * ⚠️ **DEPRECATION STATUS**: This is a placeholder stub.
 *
 * **Status**: Not implemented (NOT on Phase 15 critical path)
 *
 * **Current Approach**: Use llvmlite harness (src/llvm_py/) for LLVM backend
 * - MIR14 → LLVM IR: Python llvmlite builder (2000 lines, fully functional)
 * - No need for separate Rust-side LLVM codegen at this time
 *
 * **Replacement Path**:
 * - For MIR → LLVM: Continue using llvmlite harness
 * - For BID → LLVM: Not on roadmap (use llvmlite harness instead)
 *
 * **Decision**: This stub remains in place to preserve CodeGenTarget API surface.
 * Remove if official roadmap excludes LLVM codegen from BID system (Phase 150+).
 */

use crate::bid::{BidDefinition, BidResult};
use crate::bid::codegen::{CodeGenOptions, GeneratedFile};

pub struct LlvmGenerator;

impl LlvmGenerator {
    /// Generate LLVM declarations
    ///
    /// ⚠️ Not implemented - returns empty vec
    pub fn generate(bid: &BidDefinition, _options: &CodeGenOptions) -> BidResult<Vec<GeneratedFile>> {
        // TODO: Implement LLVM code generation (or remove if llvmlite harness is preferred)
        eprintln!("⚠️  LLVM code generation not yet implemented for {}", bid.name());
        eprintln!("   Current approach: Use llvmlite harness (src/llvm_py/)");
        eprintln!("   See: docs/development/current/main/REFACTORING_OPPORTUNITIES.md#bid-codegen-stubs-decision");
        Ok(vec![])
    }
}