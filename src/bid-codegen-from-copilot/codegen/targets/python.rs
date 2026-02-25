/*!
 * Python Target Generator - Generate Python FFI wrappers
 *
 * ⚠️ **DEPRECATION STATUS**: This is a placeholder stub.
 *
 * **Status**: Not implemented (NOT on Phase 15 critical path)
 *
 * **Replacement Path**:
 * - For production Python FFI: Consider using LLVM harness + ctypes/pyo3 bindings
 * - For development: Use VM interpreter with Python wrappers (manual implementation)
 *
 * **Future**: If Python support needed, implement via WASM target or language-specific bindings.
 *
 * **Decision**: This stub remains in place to preserve CodeGenTarget API surface.
 * Remove if Python code generation is officially deprecated (Phase 150+).
 */

use crate::bid::{BidDefinition, BidResult};
use crate::bid::codegen::{CodeGenOptions, GeneratedFile};

pub struct PythonGenerator;

impl PythonGenerator {
    /// Generate Python wrappers
    ///
    /// ⚠️ Not implemented - returns empty vec
    pub fn generate(bid: &BidDefinition, _options: &CodeGenOptions) -> BidResult<Vec<GeneratedFile>> {
        // TODO: Implement Python code generation
        eprintln!("⚠️  Python code generation not yet implemented for {}", bid.name());
        eprintln!("   See: docs/development/current/main/REFACTORING_OPPORTUNITIES.md#bid-codegen-stubs-decision");
        Ok(vec![])
    }
}